# 20. Unified cargo xtask check and ci contract

Date: 2026-09-17

## Status

Proposed

## Context

The workspace task runner previously treated `check` as `cargo check`, `ci`
as the full local gate, and `ci-post` as the GitHub Actions matrix
integration half.

rust-default-xtasks requires `check` and `ci` to share one parser and
execution path with `--only` / `--exclude`, plus optional coverage and
profile handles.

GitHub Actions called the old names, so a bare `cargo xtask check` after
alignment would have run the entire gate inside linux-gate.

## Decision

Expose `cargo xtask check` as the canonical quality command with `ci` as a
visible alias.

Register toolchain, fmt, check, clippy, test, render, link-check,
highlight, spec-vendor, and example-fixtures. Delete `ci-post`.

Feature-aware Cargo steps default to `--all-features` when no feature
option is passed. Adopt coverage and profile handles. Omit image and MCP
evaluation. Keep iteration shortcuts.

GitHub Actions must call `check --only=` selections so linux-gate and the
OS matrix keep their current job split. Humans keep workflow YAML aligned;
the xtask does not inspect a CI provider.

## Consequences

Local `cargo xtask check` now means the full gate. Use `--only=check` for
`cargo check`.

GHA rust-tests.yml uses selected steps instead of `fmt-check`, bare
`check`, `clippy`, or `ci-post`.

Coverage and Samply profiling are local-only. Agents must read
xtask/AGENTS.md for step order, report paths, and omitted handles.
