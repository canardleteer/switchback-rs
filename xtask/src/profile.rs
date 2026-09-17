//! Samply profiling for the reference-manual example workload.

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, ensure};

use crate::policy;
use crate::process::{CommandSpec, build_binary, require, run as run_command};

const SAMPLY_INSTALL: &str = "Install it with `cargo install --locked samply`. See current platform advice at https://github.com/mstange/samply/blob/main/README.md.";

pub fn run(root: &Path, open: bool) -> Result<()> {
    require(
        root,
        "Samply",
        &CommandSpec::new("samply").arg("--version"),
        SAMPLY_INSTALL,
    )?;
    explain_linux_setting();
    let binary = build_binary(
        root,
        policy::PROFILE_PACKAGE,
        policy::PROFILE_BINARY,
        "profiling",
    )?;
    let report = root.join(policy::PROFILE_REPORT);
    fs::create_dir_all(report.parent().expect("fixed profile path has a parent"))
        .with_context(|| format!("creating profile directory for {}", report.display()))?;

    let mut record =
        CommandSpec::new("samply").args(["record", "--save-only", "--no-open", "--output"]);
    record.args.push(report.as_os_str().to_owned());
    record.args.push("--".into());
    record.args.push(binary.into_os_string());
    record
        .args
        .extend(policy::PROFILE_ARGS.iter().map(Into::into));
    run_command(root, &record)?;
    ensure!(
        report.is_file(),
        "Samply succeeded but no profile exists at {}",
        report.display()
    );

    if open {
        run_command(
            root,
            &CommandSpec::new("samply")
                .arg("load")
                .arg(report.as_os_str()),
        )?;
    }
    Ok(())
}

fn explain_linux_setting() {
    #[cfg(target_os = "linux")]
    if let Ok(value) = fs::read_to_string("/proc/sys/kernel/perf_event_paranoid") {
        eprintln!(
            "Linux profiling setting kernel.perf_event_paranoid={}; if Samply reports a permission error, follow its current Linux guidance. The xtask will not change this privileged setting.",
            value.trim()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_workload_is_local_and_typed() {
        assert_eq!(policy::PROFILE_PACKAGE, "reference-manual-example");
        assert_eq!(policy::PROFILE_BINARY, "reference-manual");
        assert!(policy::PROFILE_ARGS.contains(&"--markdown-only"));
        assert!(!policy::PROFILE_ARGS.iter().any(|arg| arg.contains("http")));
    }
}
