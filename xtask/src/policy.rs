//! Repository-owned seams for check order, coverage reports, and profiling.

use crate::CheckStep;

pub const CHECK_ORDER: [CheckStep; 10] = [
    CheckStep::Toolchain,
    CheckStep::Fmt,
    CheckStep::Check,
    CheckStep::Clippy,
    CheckStep::Test,
    CheckStep::Render,
    CheckStep::LinkCheck,
    CheckStep::Highlight,
    CheckStep::SpecVendor,
    CheckStep::ExampleFixtures,
];

pub const LLVM_COV_REPORT: &str = "target/coverage/llvm-cov/html/index.html";
pub const TARPAULIN_REPORT: &str = "target/coverage/tarpaulin/tarpaulin-report.html";

pub const PROFILE_REPORT: &str = "target/profile/profile.json";
pub const PROFILE_PACKAGE: &str = "reference-manual-example";
pub const PROFILE_BINARY: &str = "reference-manual";
pub const PROFILE_ARGS: &[&str] = &["--markdown-only", "-o", "target/profile/ref-book"];
