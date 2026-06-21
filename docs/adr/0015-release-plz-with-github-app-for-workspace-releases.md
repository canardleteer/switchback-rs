# 15. release-plz with GitHub App for workspace releases

Date: 2026-06-21

## Status

Proposed

## Context

The workspace publishes ten `switchback-*` crates in lockstep
(`version.workspace = true` plus `[workspace.dependencies]` path pins). Manual
version bumps and crates.io publishes do not scale; Release PRs must trigger
existing CI (`rust-tests`, `rumdl`). release-plz does not sync
`[workspace.dependencies]` `switchback-*` version fields when it bumps
`[workspace.package].version`.

## Decision

Automate releases with release-plz and a dedicated GitHub App
(`switchback-rs-release-plz-app`) so Release PRs and tags trigger downstream
workflows. Configure [`release-plz.toml`](../../release-plz.toml) with
`release_always = false`, a single `version_group = "switchback"` across all
publishable crates, one workspace `CHANGELOG.md` owned by `switchback-traits`,
and a single git tag/GitHub Release (`v{{ version }}`) from that crate. Exclude
`xtask` and example packages with `release = false`. After
`release-plz release-pr`, the GitHub Actions workflow runs
`cargo xtask align-workspace-versions` and `cargo generate-lockfile` on the
Release PR. Publishing uses a `CARGO_REGISTRY_TOKEN` in the `release`
environment (not OIDC trusted publishing for now).

## Consequences

Positive: conventional-commit-driven Release PRs, crates.io publish and GitHub
Releases on merge, Release PRs run full CI via the App token. Negative: two
post-merge pushes to Release PRs when alignment changes (release-plz then align
commit); release-plz and align logic must stay in sync with the workspace
versioning invariant enforced in `linux-gate`. Binary artifacts are deferred;
document the follow-up workflow in AGENTS.md when bins ship.

First-time crates.io publish: ten new crate names cannot be uploaded in one
`release-plz release` run because of
[crates.io new-crate rate limits](https://crates.io/docs/rate-limits) (burst of
five, then one every ten minutes). Until bootstrap completes, `release-plz.yml`
stays disabled. Bootstrap uses `publish-crate.yml` (`workflow_dispatch` on
`main`) to register each name at `0.0.1-0.dev.0.<short_sha>`; `main` keeps
`0.0.1-0.dev.1` in git. After all names exist, re-enable release-plz so the
first automated release publishes `0.0.1-0.dev.1` as version updates (within the
existing-crate burst).
