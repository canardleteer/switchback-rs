//! Export BSR deps from a Buf module for protoc `-I` (never protoc inputs).

#[cfg(feature = "buf")]
use crate::input::resolve_buf_path;
#[cfg(not(feature = "buf"))]
use crate::input::tool_exists;
use anyhow::{bail, Context, Result};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const VALIDATE_PROTO: &str = "buf/validate/validate.proto";
const EXPORT_LOCK: &str = ".proto-deps.export.lock";
const EXPORT_STAMP: &str = ".export.stamp";
const MIN_VALIDATE_BYTES: u64 = 100_000;
const LOCK_TIMEOUT: Duration = Duration::from_secs(120);
const LOCK_RETRY: Duration = Duration::from_millis(50);

pub fn validate_proto_path(export_dir: &Path) -> PathBuf {
    export_dir.join(VALIDATE_PROTO)
}

fn export_lock_path(export_dir: &Path) -> PathBuf {
    export_dir
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(EXPORT_LOCK)
}

fn export_stamp_path(export_dir: &Path) -> PathBuf {
    export_dir.join(EXPORT_STAMP)
}

fn staging_export_dir(export_dir: &Path) -> PathBuf {
    let parent = export_dir.parent().unwrap_or_else(|| Path::new("."));
    let name = export_dir
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "proto-deps".to_owned());
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    parent.join(format!("{name}.staging-{nonce}"))
}

fn trash_export_dir(export_dir: &Path) -> PathBuf {
    export_dir
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(format!(
            "{}.export-trash",
            export_dir
                .file_name()
                .map(|value| value.to_string_lossy().into_owned())
                .unwrap_or_else(|| "proto-deps".to_owned())
        ))
}

struct ExportLock {
    lock_path: PathBuf,
    _file: File,
}

impl Drop for ExportLock {
    fn drop(&mut self) {
        let _ = self._file.sync_all();
        let _ = fs::remove_file(&self.lock_path);
    }
}

fn acquire_export_lock(export_dir: &Path) -> Result<ExportLock> {
    if let Some(parent) = export_dir.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }

    let lock_path = export_lock_path(export_dir);
    let deadline = Instant::now() + LOCK_TIMEOUT;
    loop {
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&lock_path)
        {
            Ok(file) => {
                return Ok(ExportLock {
                    lock_path,
                    _file: file,
                });
            }
            Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                if Instant::now() >= deadline {
                    bail!(
                        "timed out waiting for proto-deps export lock at {}",
                        lock_path.display()
                    );
                }
                thread::sleep(LOCK_RETRY);
            }
            Err(err) => {
                return Err(err)
                    .with_context(|| format!("open export lock {}", lock_path.display()));
            }
        }
    }
}

fn read_export_stamp(export_dir: &Path) -> Option<u64> {
    fs::read_to_string(export_stamp_path(export_dir))
        .ok()
        .and_then(|stamp| stamp.trim().parse().ok())
}

fn write_export_stamp(export_dir: &Path, size: u64) -> Result<()> {
    fs::write(export_stamp_path(export_dir), format!("{size}\n"))
        .with_context(|| format!("write {}", export_stamp_path(export_dir).display()))
}

fn validate_proto_tail(path: &Path) -> Result<bool> {
    let mut file = File::open(path).with_context(|| format!("open {}", path.display()))?;
    let len = file.metadata()?.len();
    if len == 0 {
        return Ok(false);
    }
    let tail_len = len.min(64);
    file.seek(SeekFrom::End(-(tail_len as i64)))?;
    let mut tail = vec![0; tail_len as usize];
    file.read_exact(&mut tail)?;
    Ok(tail.ends_with(b"}\n") || tail.ends_with(b"}\r\n"))
}

fn export_is_current(export_dir: &Path) -> Result<bool> {
    let validate = validate_proto_path(export_dir);
    let metadata = match fs::metadata(&validate) {
        Ok(metadata) => metadata,
        Err(_) => return Ok(false),
    };
    let size = metadata.len();
    if size < MIN_VALIDATE_BYTES {
        return Ok(false);
    }
    if read_export_stamp(export_dir) != Some(size) {
        return Ok(false);
    }
    validate_proto_tail(&validate)
}

fn validate_export_dir(export_dir: &Path, proto_root: &Path) -> Result<u64> {
    let validate = validate_proto_path(export_dir);
    if !validate.is_file() {
        bail!(
            "buf export missing {}; run `buf dep update` in {}",
            validate.display(),
            proto_root.display()
        );
    }
    let size = fs::metadata(&validate)
        .with_context(|| format!("stat {}", validate.display()))?
        .len();
    if size < MIN_VALIDATE_BYTES {
        bail!(
            "buf export produced unexpectedly small {}; expected at least {MIN_VALIDATE_BYTES} bytes",
            validate.display()
        );
    }
    if !validate_proto_tail(&validate)? {
        bail!(
            "buf export produced incomplete {}; file does not end with a closing brace",
            validate.display()
        );
    }
    Ok(size)
}

fn resolve_buf_for_export(explicit: Option<&Path>) -> Result<PathBuf> {
    #[cfg(feature = "buf")]
    {
        resolve_buf_path(explicit)
    }
    #[cfg(not(feature = "buf"))]
    {
        let _ = explicit;
        if tool_exists("buf") {
            return Ok(PathBuf::from("buf"));
        }
        bail!(
            "buf not found on PATH; install with: cargo install buf-toolchain --locked --version 1.70.0-hotfix.1"
        )
    }
}

fn run_buf_export(proto_root: &Path, output_dir: &Path, buf_path: Option<&Path>) -> Result<()> {
    if output_dir.exists() {
        fs::remove_dir_all(output_dir).with_context(|| {
            format!(
                "clear proto-deps staging export at {}",
                output_dir.display()
            )
        })?;
    }
    if let Some(parent) = output_dir.parent() {
        fs::create_dir_all(parent).with_context(|| format!("create {}", parent.display()))?;
    }

    let buf = resolve_buf_for_export(buf_path)?;
    let status = Command::new(&buf)
        .current_dir(proto_root)
        .args(["export", ".", "--output"])
        .arg(output_dir)
        .status()
        .context("buf export")?;
    if !status.success() {
        bail!("buf export failed");
    }
    Ok(())
}

fn publish_export(staging: &Path, export_dir: &Path) -> Result<()> {
    let trash = trash_export_dir(export_dir);
    let _ = fs::remove_dir_all(&trash);
    if export_dir.exists() {
        fs::rename(export_dir, &trash).with_context(|| {
            format!(
                "rotate proto-deps export {} -> {}",
                export_dir.display(),
                trash.display()
            )
        })?;
    }
    fs::rename(staging, export_dir).with_context(|| {
        format!(
            "publish proto-deps export {} -> {}",
            staging.display(),
            export_dir.display()
        )
    })?;
    let _ = fs::remove_dir_all(&trash);
    Ok(())
}

pub fn ensure_proto_deps_export(
    proto_root: &Path,
    export_dir: &Path,
    refresh: bool,
    buf_path: Option<&Path>,
) -> Result<PathBuf> {
    let _lock = acquire_export_lock(export_dir)?;

    if !refresh && export_is_current(export_dir)? {
        return Ok(export_dir.to_path_buf());
    }

    let staging = staging_export_dir(export_dir);
    run_buf_export(proto_root, &staging, buf_path)?;
    let size = validate_export_dir(&staging, proto_root)?;
    write_export_stamp(&staging, size)?;
    publish_export(&staging, export_dir)?;
    Ok(export_dir.to_path_buf())
}
