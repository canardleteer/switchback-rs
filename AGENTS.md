# AGENTS.md

Instructions for AI agents (and humans) working in this repository.

## Architectural Decision Records

We record architectural decisions as **ADRs**, using the
[`adrs`](https://joshrotenberg.com/adrs/) tool. It also ships
an [MCP server](https://joshrotenberg.com/adrs/mcp.html) for listing, searching,
creating, and updating ADRs from agents.

- ADRs live in [`docs/adr/`](docs/adr/) (the location is pinned by `.adr-dir`).
  Record one for any decision that is architectural in scope: a new crate, a
  trait shape, a serialization format, a versioning scheme, a deferred gap,
  or a change to the seam between parsers, the core, and renderers.
- Create and edit ADRs with `adrs` (or the MCP server), not by hand-copying
  section headings — `adrs new` applies the Nygard template, sequences files,
  and titles records for you.
- Prefer an ADR over a buried code comment or a planning note when the
  decision shapes how other agents should approach the codebase.
- Prefer the `adrs` [MCP server](https://joshrotenberg.com/adrs/mcp.html) for
  listing, searching, creating, and updating ADRs when it is available. Start
  it from the repository root (`adrs mcp serve --ng`) or pass `-C` with the
  path to this checkout.

## Documentation linking

When you change structural references (`Reference`, `StoredEntity.refs`),
intra-links (`IntraLink`, `LinkExtractor`, populate/extract wiring), or
render-time link formatting (`LinkFormatter`, `LinkContext`, mdBook link
application), update
[`docs/documentation-linking.md`](docs/documentation-linking.md) so family
examples and naming stay accurate. Prefer adjusting that doc over burying
linking conventions in code comments.

## Parser test fixtures

Family parser crates may vend **upstream** example corpora and maintain
**micro** hand-written regressions. Do not conflate the two.

| Tier | Path (`switchback-openapi`) | Maintained how | Agent rule |
| --- | --- | --- | --- |
| **Upstream** | `crates/switchback-openapi/tests/fixtures/upstream/` | `cargo xtask spec-vendor fetch-fixtures --family openapi`; locked in `example-fixtures.lock.toml` | **Do not hand-edit.** Refresh from upstream, update lock SHA-256, run `validate-fixtures`. See `tests/fixtures/upstream/FIXTURES.md`. |
| **Micro** | `crates/switchback-openapi/tests/fixtures/micro/` | Hand-maintained in-repo | **Safe to edit.** One isolated behavior each (`x-tagGroups`, `nullable` 3.0, beside companion, minimal multifile `$ref`). Keep tiny. |

**Related locks (`switchback-openapi`):**

- `meta-schemas/` + `meta-schemas.lock.toml` — JSON Schema meta-schemas only
  (ADR 0005); `spec-vendor fetch --family openapi`.
- `example-fixtures.lock.toml` — example API descriptions only;
  `spec-vendor fetch-fixtures --family openapi`.

Other family crates may adopt the same split later; protobuf/jsonschema fixtures
keep their existing layouts until then.

| Tier | Path (`switchback-asyncapi`) | Maintained how | Agent rule |
| --- | --- | --- | --- |
| **Upstream** | `crates/switchback-asyncapi/tests/fixtures/upstream/` | `cargo xtask spec-vendor fetch-fixtures --family asyncapi`; locked in `example-fixtures.lock.toml` | **Do not hand-edit.** Refresh from upstream, update lock SHA-256, run `validate-fixtures`. See `tests/fixtures/upstream/FIXTURES.md`. |
| **Micro** | `crates/switchback-asyncapi/tests/fixtures/micro/` | Hand-maintained in-repo | **Safe to edit.** Acme three-version corpus, minimal smoke fixture. Keep tiny. |

| Tier | Path (`switchback-openrpc`) | Maintained how | Agent rule |
| --- | --- | --- | --- |
| **Upstream** | `crates/switchback-openrpc/tests/fixtures/upstream/` | `cargo xtask spec-vendor fetch-fixtures --family openrpc`; locked in `example-fixtures.lock.toml` | **Do not hand-edit.** Refresh from upstream, update lock SHA-256, run `validate-fixtures`. See `tests/fixtures/upstream/FIXTURES.md`. |
| **Micro** | `crates/switchback-openrpc/tests/fixtures/micro/` | Hand-maintained in-repo | **Safe to edit.** Acme three-version corpus, tag-groups, companion, multifile `$ref`. Keep tiny. |

**Related locks (`switchback-openrpc`):**

- `meta-schemas/` + `meta-schemas.lock.toml` — OpenRPC document meta-schemas;
  `spec-vendor fetch --family openrpc`.
- `example-fixtures.lock.toml` — example API descriptions only;
  `spec-vendor fetch-fixtures --family openrpc`.

**Related locks (`switchback-asyncapi`):**

- `meta-schemas/` + `meta-schemas.lock.toml` — AsyncAPI document meta-schemas;
  `spec-vendor fetch --family asyncapi`.
- `example-fixtures.lock.toml` — example API descriptions only;
  `spec-vendor fetch-fixtures --family asyncapi`.

## Markdown

Use [`rumdl`](https://github.com/rvben/rumdl) to lint Markdown. Configuration
lives in [`.rumdl.toml`](.rumdl.toml).

- Check Markdown before finishing doc edits:
  `rumdl check --respect-gitignore .`
- Respect `.gitignore` when scanning so generated or local-only files are
  skipped.
- Run `rumdl fmt` to apply auto-fixes where a rule supports them.

### Crate READMEs and published rustdoc

Each `crates/*/README.md` is published on [crates.io](https://crates.io/) with
the crate. Crate-level rustdoc (`src/lib.rs`) is published on
[docs.rs](https://docs.rs/). crates.io only requires a README in the published
crate tarball; it does not require particular link styles or `docs.rs` URLs in
that README (crates.io links to docs.rs separately when a build exists).

**Relative links are allowed but unreliable.** A published crate is a tarball,
not a browsable repo tree. crates.io may rewrite relative URLs when
`package.repository` is set (GitHub-specific heuristics), but behavior varies
with monorepos, `readme = "../..."` in `Cargo.toml`, symlinks, and release
metadata. Paths with `../` that leave the crate directory — for example
`../../docs/...` in a workspace member — usually resolve incorrectly on
crates.io.

Practical guidance:

- **Do not** pepper crate READMEs with `https://docs.rs/...` links to this
  crate's API; use plain `` `TypeName` `` names. Link to docs.rs from rustdoc
  (`` [`TypeName`] ``) or let crates.io's documentation link handle it.
- **Do** use absolute GitHub URLs for workspace docs outside the crate
  (glossary, ADRs) when you want the link to work on crates.io:
  `https://github.com/canardleteer/switchback-rs/blob/main/...`.
- Relative links to files **inside the published crate** (for example
  `./LICENSE`) may work after crates.io rewriting but are not guaranteed; prefer
  absolute URLs when the link must work on crates.io.
- Keep each crate's README at `crates/<name>/README.md` (not
  `readme = "../README.md"`) unless you accept broken or mis-resolved relative
  links on crates.io.

Same rules apply to workspace links in crate-level rustdoc on docs.rs.

## YAML

Use [`ryl`](https://crates.io/crates/ryl) to lint in-repo YAML (not
[`.github/workflows/`](.github/workflows/); those are validated by GitHub).
Configuration lives in [`.yamllint`](.yamllint).

- Check YAML before finishing config edits: `ryl .`
- Directory scans respect `.gitignore`, so generated or local-only files are
  skipped.
- Run `ryl --fix` to apply safe auto-fixes where a rule supports them.

## Workspace checks (`xtask`)

The workspace task runner lives in [`xtask/`](xtask/).

[`rust-toolchain.toml`](rust-toolchain.toml) pins the Rust channel and rustup
components (`rustfmt`, `clippy`). Run `rustup toolchain install` in the repo
root if `cargo xtask check-toolchain` reports missing components.

Before opening a PR, run the full pre-merge block below (or the subset matching
your changed paths).

### Matching GitHub Actions locally

| Workflow | When (GHA) | Local equivalent |
| --- | --- | --- |
| [`rust-tests.yml`](.github/workflows/rust-tests.yml) gate | Push/PR; `linux-gate` on `ubuntu-latest` | `cargo xtask align-workspace-versions --check`, `fmt-check`, `check`, `clippy`, `publish-check`, `audit` |
| `rust-tests.yml` matrix (linux) | After gate; `ci-post` only | `cargo xtask ci-post` (or full `cargo xtask ci`) |
| `rust-tests.yml` matrix (linux / macOS) | After gate; `check`/`clippy` then `ci-post` on macOS only | `cargo xtask check`, `clippy`, `ci-post` (or full `cargo xtask ci`) |
| [`rumdl.yml`](.github/workflows/rumdl.yml) | Every PR to `main`; push to `main` when `**/*.md` or `.rumdl.toml` change | `cargo xtask rumdl-check` |
| [`yaml-lint.yml`](.github/workflows/yaml-lint.yml) | Every PR to `main`; push to `main` when in-repo YAML under `crates/`, `examples/`, `proto/` changes | `cargo xtask ryl` |
| [`release-plz.yml`](.github/workflows/release-plz.yml) | Push to `main`, `workflow_dispatch` | N/A |
| [`publish-crate.yml`](.github/workflows/publish-crate.yml) | Manual `workflow_dispatch` on `main` only | N/A — emergency republish one crate at a time |

GHA uses
[`rustsec/audit-check`](https://github.com/rustsec/audit-check/commit/858dc40f52ca2b8570b7a997c1c4e35c6fc9a432)
(Node 24) once in **`linux-gate`**; locally that is the same check as
`cargo xtask audit` → `cargo audit`.

**Full pre-merge gate (linux-equivalent)** — mirrors all three workflows on a
typical PR:

```bash
cargo xtask align-workspace-versions --check   # rust-tests.yml linux-gate
cargo xtask ci                                 # full local gate (ci + ci-post)
cargo xtask audit                              # rust-tests.yml linux-gate
cargo xtask publish-check                      # rust-tests.yml linux-gate
cargo xtask rumdl-check                        # rumdl.yml
cargo xtask ryl                                # yaml-lint.yml
```

GHA splits compile gates across **`linux-gate`** (align, fmt-check, check,
clippy, publish-check, audit) and the **matrix** (`ci-post` on linux; check,
clippy, then `ci-post` on macOS). Local pre-merge still uses the
undivided `ci` block above.

Hygiene subcommands require these tools on `PATH` (install once per machine):

- [`cargo-audit`](https://github.com/rustsec/rustsec/tree/main/cargo-audit) —
  `cargo install cargo-audit --locked`
- [`rumdl`](https://github.com/rvben/rumdl) — `cargo install rumdl --locked`
- [`ryl`](https://crates.io/crates/ryl) — `cargo install ryl --locked`

If any are missing, `xtask` prints an install hint before failing.

### `cargo xtask ci` — Rust/parser gate

**`ci` is the full local Rust/parser gate.** GHA runs it in two parts: the
**`linux-gate`** job (through clippy + audit) and the **matrix** (`ci-post`;
check/clippy before `ci-post` on macOS only). Individual subcommands
(`fmt-check`, `clippy`, `test`, etc.) exist so you can run one step while
iterating; they are not a substitute for `ci`.

`cargo xtask ci` runs, in order:

1. `check-toolchain` — pin matches [`rust-toolchain.toml`](rust-toolchain.toml)
2. `fmt-check` — `cargo fmt --all --check` plus wire-schema `buf lint` /
   `buf format --diff`
3. `check` — `cargo check --workspace --all-targets`
4. `clippy` — `cargo clippy --workspace --all-targets -- -D warnings`
5. `ci-post` — steps 6–11 below

`cargo xtask ci-post` runs the integration half only (GHA matrix after compile
gates):

6. `test` — `cargo test --workspace`
7. `render mdbook` — golden renderer regression
8. `link-check` — intra-link validation
9. `check-highlight-rust` — protobuf / CEL highlighter golden HTML
10. `spec-vendor validate` — vendored meta-schema SHA-256 locks
11. `example-fixtures validate` — OpenAPI and AsyncAPI upstream fixture locks

Audit, Markdown, and YAML hygiene run via separate workflows (see table above).

## Releases

Configuration lives in [`release-plz.toml`](release-plz.toml); the workspace
changelog is [`CHANGELOG.md`](CHANGELOG.md).

### crates.io bootstrap (completed)

All ten `switchback-*` crate names are registered on crates.io at
`0.0.1-0.dev.0.<suffix>` (lockstep suffix via `version_suffix` on
[`publish-crate.yml`](.github/workflows/publish-crate.yml); **`main` stays at
`0.0.1-0.dev.1` in git**). Keep `publish-crate.yml` for emergency manual
republish of one crate at a time.

Bootstrap used [`publish-crate.yml`](.github/workflows/publish-crate.yml)
(`workflow_dispatch` on **`main` only**, `release` environment). It temporarily
**stripped** workspace `switchback-*` entries from `[dev-dependencies]` and used
`--no-verify` so `cargo publish` could register new crate names before every
dependency existed on crates.io. That strip is **bootstrap-only**; committed
manifests must be `release-plz`-clean before Phase 3.

**Publish order** (runtime dependencies, then dev-dependencies; enforced by
`cargo xtask publish-check`):

1. `switchback-traits`
2. `switchback-codec-pb`
3. `switchback-protocols`
4. `switchback-jsonschema`
5. `switchback-asyncapi`
6. `switchback-openrpc`
7. `switchback-openapi` — fifth new crate; rate-limit boundary during bootstrap
8. `switchback-protobuf`
9. `switchback-assemble`
10. `switchback-mdbook` — dev-depends on `openapi`, `protobuf`, and `assemble`

After every **fifth** new crate, wait **eleven minutes** before the next run
([crates.io rate limits](https://crates.io/docs/rate-limits)).

```bash
gh workflow run publish-crate.yml -f crate=switchback-traits -f version_suffix=ffcda32 --ref main
```

### Publishable dev-dependencies (completed)

`cargo publish` resolves `[dev-dependencies]` against crates.io even though they
are not shipped. Workspace `switchback-*` dev-deps must not form cycles and must
only point at crates **earlier** in the publish order above. Fixed on `main` in
[#11](https://github.com/canardleteer/switchback-rs/pull/11):

- Removed the `switchback-codec-pb` ↔ `switchback-protocols` dev-dep cycle
  (protocol round-trip test moved to `switchback-protocols`).
- Removed cross-crate dev-deps from `switchback-jsonschema` (meta-schema loader
  smoke tests moved to `switchback-openapi` / `switchback-openrpc`).
- Removed duplicate `switchback-codec-pb` dev-deps where it is already a runtime
  dependency.
- Kept `switchback-mdbook` integration-test dev-deps on `openapi`, `asyncapi`,
  `openrpc`, `protobuf`, and `assemble`; **`assemble` must publish before
  `mdbook`**.

`cargo xtask publish-check` fails if any publishable crate dev-depends on a
later crate in the order.

### Steady-state release-plz

[release-plz](https://release-plz.dev/) is enabled in
[`.github/workflows/release-plz.yml`](.github/workflows/release-plz.yml).
Keep `publish-crate.yml` for emergency manual republish of one crate at a time.

Flow:

1. A conventional commit lands on `main`.
2. **`release-plz-pr`** opens or updates a Release PR (GitHub App
   `switchback-rs-release-plz-app`, so `rust-tests` and `rumdl` run on that PR).
3. The workflow runs `cargo xtask align-workspace-versions` on the Release PR so
   `[workspace.dependencies]` `switchback-*` pins match
   `[workspace.package].version` (required for **`linux-gate`**).
4. Merge the Release PR.
5. **`release-plz-release`** publishes all workspace crates to crates.io (using
   the `release` environment `CARGO_REGISTRY_TOKEN`), tags `v{{ version }}`, and
   creates one GitHub Release.

The first release-plz publish after bootstrap should upload `0.0.1-0.dev.1` from
committed manifests as a **version update** on crate names already registered
during bootstrap (no `publish_allow_dirty`).

All ten publishable crates share `version_group = "switchback"` in
`release-plz.toml`. Do not bump versions with `cargo set-version`; release-plz
updates `[workspace.package].version` and the align step syncs dependency pins.

### `cargo xtask publish-check`

Runs before merge in **`linux-gate`**. Validates that publishable crate
`[dev-dependencies]` only reference `switchback-*` crates earlier in the
publish order (no cycles or forward dev-deps). For each publishable crate:
`cargo package --list -p <crate>`. Then
`cargo publish -p switchback-traits --dry-run --allow-dirty` for the leaf crate.

### Semver check (`cargo-semver-checks`)

[`release-plz.toml`](release-plz.toml) sets `semver_check = false` while the
workspace version carries a **pre-release label** (for example
`0.0.1-0.dev.1`). API stability is not promised during that phase.

**After the first stable release without pre-release metadata** (for example
`1.0.0`, or `0.1.0` with no `-alpha`/`-beta` suffix), enable semver checking:

1. Set `semver_check = true` under `[workspace]` in
   [`release-plz.toml`](release-plz.toml).
2. Treat Release PR **semver-check failures** as blocking — they indicate an
   API-breaking change that needs a major bump (or an explicit, reviewed
   exception before merge).

release-plz runs
[cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) on
library crates when this flag is on. Do not re-disable it without an ADR.

### Future binary releases (not implemented)

When publishable crates ship `[[bin]]` targets, add a separate workflow on
`release: types: [published]` (triggered by the GitHub Release release-plz
creates). Build a matrix (`ubuntu-latest`, `macos-14`, `windows-2025`), attach
artifacts with `gh release upload` or `softprops/action-gh-release`. Library
crate releases must not depend on binary artifacts being present.

### `cargo xtask align-workspace-versions`

Syncs `[workspace.package].version` with every `switchback-*` `version` in
`[workspace.dependencies]` (root [`Cargo.toml`](Cargo.toml) only). All member
crates use `version.workspace = true`; do **not** run `cargo set-version` here.
Release-plz bumps the workspace version; CI and the release-plz workflow run
this command to keep dependency pins aligned.

```bash
# Bump workspace semver
cargo xtask align-workspace-versions --version 0.0.1-0.dev.2
cargo generate-lockfile

# CI invariant (linux-gate job)
cargo xtask align-workspace-versions --check
```

### Fix commands (not CI)

- **`cargo xtask fmt`** — apply fixes: `cargo fmt --all`, `rumdl fmt` on touched
  markdown (or the whole tree), `ryl --fix` where applicable.
- **`cargo xtask fmt-check`**, **`clippy`**, **`test`**, **`audit`**,
  **`rumdl-check`**, **`ryl`** — single-step shortcuts; same flags as the
  matching hygiene or gate step.
