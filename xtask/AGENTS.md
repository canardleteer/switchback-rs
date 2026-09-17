# xtask Instructions for Agents

These instructions govern the standard development commands and any additional
tasks added to this crate. The crate is a root workspace member and
[`.cargo/config.toml`](../.cargo/config.toml) maps `cargo xtask` to
`cargo run --package xtask --`.

## Command Policy

- `check` registers `toolchain`, `fmt`, `check`, `clippy`, `test`, `render`,
  `link-check`, `highlight`, `spec-vendor`, and `example-fixtures` in that
  order. It is fail-fast, runs all ten by default, supports mutually exclusive
  `--only` and `--exclude`, and defaults feature-aware Cargo commands to
  `--all-features`. `ci` is a visible alias over exactly the same
  implementation.
- Keep CI/workflow declarations and scripts aligned with `check` when behavior
  changes. The xtask must remain provider-agnostic and must not inspect those
  declarations itself.
- `image` is omitted: this repository does not own Dockerfiles or
  Containerfiles.
- `coverage` supports `llvm-cov` and `tarpaulin`. Reports are
  `target/coverage/llvm-cov/html/index.html` and
  `target/coverage/tarpaulin/tarpaulin-report.html`. Both `coverage --open` and
  `coverage-open` generate a fresh report first.
- `profile` builds the exact `reference-manual` binary from
  `reference-manual-example` with Cargo JSON messages, records
  `--markdown-only -o target/profile/ref-book` under the `profiling` profile,
  and writes `target/profile/profile.json`. Both opening forms record before
  loading.
- MCP evaluation is omitted: this repository does not expose a stdio MCP
  server.

Repository-specific shortcuts (`fmt`, `fmt-check`, `clippy`, `test`, `audit`,
`rumdl-check`, `ryl`, `align-workspace-versions`, `publish-check`,
`check-toolchain`, `spec-vendor`, `render`, `link-check`, highlight/golden
updaters) remain for iteration. They are not a substitute for the full
`check` / `ci` gate.

## Tool Guidance

Probe only tools selected by the command. A failed launch is an unusable tool,
not an absent one. Recommend these exact Cargo installs when applicable:

```text
cargo install --locked cargo-llvm-cov
cargo install --locked cargo-tarpaulin
cargo install --locked samply
cargo install --locked cargo-audit
cargo install --locked rumdl
cargo install --locked ryl
cargo install buf-toolchain --locked --version 1.73.0-rc.1
```

Use `rustup component add rustfmt` or `rustup component add clippy` for missing
Rust components. Inspect and explain profiling settings, but never modify a
privileged system setting automatically.

When `buf` is not on `PATH`, `buf-tools` 1.73.0-rc.1 supplies the vendored
binary used by `fmt` and `switchback-protobuf`.

## Implementation and Validation

Represent subprocesses as a program plus OS argument vector. Keep `main.rs`
thin, use Clap derive types, and retain parser, selection, feature, alias,
artifact-selection, and failure-path tests. When a command actually needs
async I/O or concurrency, `tokio` and `tracing` are appropriate. Do not add
them as ceremony when the work is synchronous. This crate is synchronous.

Prefer adding Cargo-aware cross-platform orchestration here over adding a
Python wrapper. This repository has no retained Python orchestration.

Run from the repository root:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test -p xtask
cargo xtask check --only=fmt,check
```

The coverage and profile commands depend on selected local tools and should be
exercised when their implementations change.
