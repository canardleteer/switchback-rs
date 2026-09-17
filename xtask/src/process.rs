//! Typed subprocess construction and execution.

use std::ffi::{OsStr, OsString};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, ensure};
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandSpec {
    pub program: OsString,
    pub args: Vec<OsString>,
}

impl CommandSpec {
    pub fn new(program: impl Into<OsString>) -> Self {
        Self {
            program: program.into(),
            args: Vec::new(),
        }
    }

    pub fn arg(mut self, arg: impl Into<OsString>) -> Self {
        self.args.push(arg.into());
        self
    }

    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }
}

pub fn run(root: &Path, spec: &CommandSpec) -> Result<()> {
    eprintln!("+ {}", display(spec));
    let status = Command::new(&spec.program)
        .args(&spec.args)
        .current_dir(root)
        .status()
        .with_context(|| format!("starting {}", display(spec)))?;
    ensure!(status.success(), "{} failed with {status}", display(spec));
    Ok(())
}

pub fn probe(root: &Path, label: &str, spec: &CommandSpec, guidance: &str) -> Result<(), String> {
    match Command::new(&spec.program)
        .args(&spec.args)
        .current_dir(root)
        .output()
    {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Err(format!(
            "{label} is not installed or not on PATH. {guidance}"
        )),
        Err(error) => Err(format!(
            "{label} could not be launched: {error}. {guidance}"
        )),
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => Err(format!(
            "{label} is installed but unusable ({}): {}. {guidance}",
            output.status,
            concise_output(&output.stdout, &output.stderr)
        )),
    }
}

pub fn require(root: &Path, label: &str, spec: &CommandSpec, guidance: &str) -> Result<()> {
    probe(root, label, spec, guidance).map_err(anyhow::Error::msg)
}

pub fn build_binary(root: &Path, package: &str, binary: &str, profile: &str) -> Result<PathBuf> {
    let package_id_output = Command::new("cargo")
        .current_dir(root)
        .args(["pkgid", "--locked", "--package", package])
        .output()
        .context("resolving Cargo package identity")?;
    ensure!(
        package_id_output.status.success(),
        "Cargo could not resolve package {package}"
    );
    let package_id = String::from_utf8(package_id_output.stdout)
        .context("Cargo package identity was not UTF-8")?;
    let package_id = package_id.trim();

    let mut child = Command::new("cargo")
        .current_dir(root)
        .args([
            "build",
            "--locked",
            "--package",
            package,
            "--bin",
            binary,
            "--profile",
            profile,
            "--message-format=json-render-diagnostics",
        ])
        .stderr(Stdio::inherit())
        .stdout(Stdio::piped())
        .spawn()
        .context("starting Cargo artifact build")?;
    let stdout = child
        .stdout
        .take()
        .context("Cargo build stdout unavailable")?;
    let mut artifact = None;
    for line in BufReader::new(stdout).lines() {
        let line = line.context("reading Cargo JSON message")?;
        let message: Value = serde_json::from_str(&line).context("parsing Cargo JSON message")?;
        if artifact_matches(&message, package_id, binary) {
            artifact = message
                .get("executable")
                .and_then(Value::as_str)
                .map(PathBuf::from);
        }
    }
    let status = child.wait().context("waiting for Cargo artifact build")?;
    ensure!(
        status.success(),
        "Cargo artifact build failed with {status}"
    );
    artifact.context("Cargo did not report the requested binary artifact")
}

fn artifact_matches(message: &Value, package_id: &str, binary: &str) -> bool {
    let package_matches = message.get("package_id").and_then(Value::as_str) == Some(package_id);
    let target = &message["target"];
    message["reason"] == "compiler-artifact"
        && package_matches
        && target["name"] == binary
        && target["kind"]
            .as_array()
            .is_some_and(|kinds| kinds.iter().any(|kind| kind == "bin"))
        && message.get("executable").is_some_and(Value::is_string)
}

fn display(spec: &CommandSpec) -> String {
    std::iter::once(spec.program.as_os_str())
        .chain(spec.args.iter().map(OsString::as_os_str))
        .map(OsStr::to_string_lossy)
        .collect::<Vec<_>>()
        .join(" ")
}

fn concise_output(stdout: &[u8], stderr: &[u8]) -> String {
    let text = if stderr.is_empty() { stdout } else { stderr };
    let text = String::from_utf8_lossy(text);
    let one_line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if one_line.is_empty() {
        "no diagnostic output".to_string()
    } else {
        one_line.chars().take(400).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn selects_only_the_exact_binary_artifact() {
        let matching = json!({
            "reason": "compiler-artifact",
            "package_id": "path+file:///tmp/reference-manual-example#0.1.0",
            "target": {"name": "reference-manual", "kind": ["bin"]},
            "executable": "/tmp/reference-manual"
        });
        let package_id = "path+file:///tmp/reference-manual-example#0.1.0";
        assert!(artifact_matches(&matching, package_id, "reference-manual"));
        assert!(!artifact_matches(&matching, "other", "reference-manual"));
        assert!(!artifact_matches(&matching, package_id, "other"));
    }

    #[test]
    fn empty_probe_diagnostic_is_helpful() {
        assert_eq!(concise_output(b"", b""), "no diagnostic output");
    }

    #[test]
    fn probe_distinguishes_a_missing_program() {
        let error = probe(
            Path::new("."),
            "fixture",
            &CommandSpec::new("__switchback_xtask_missing_probe_fixture__"),
            "Install the fixture.",
        )
        .expect_err("fixture must be absent");
        assert!(error.contains("not installed or not on PATH"));
        assert!(error.contains("Install the fixture."));
    }

    #[test]
    fn probe_distinguishes_an_unusable_program() {
        let error = probe(
            Path::new("."),
            "fixture",
            &CommandSpec::new("cargo").arg("__switchback_xtask_bad_subcommand_fixture__"),
            "Repair the fixture.",
        )
        .expect_err("Cargo must reject the impossible subcommand");
        assert!(error.contains("installed but unusable"));
        assert!(error.contains("Repair the fixture."));
    }
}
