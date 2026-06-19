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

Use [`ryl`](https://crates.io/crates/ryl) to lint YAML.

- Check YAML before finishing config edits: `ryl .`
- Directory scans respect `.gitignore`, so generated or local-only files are
  skipped.
- Run `ryl --fix` to apply safe auto-fixes where a rule supports them.

## Workspace checks (`xtask`)

The workspace task runner lives in [`xtask/`](xtask/).

[`rust-toolchain.toml`](rust-toolchain.toml) pins the Rust channel and rustup
components (`rustfmt`, `clippy`). Run `rustup toolchain install` in the repo
root if `cargo xtask check-toolchain` reports missing components.

`cargo xtask ci` also needs these tools on `PATH` (install once per machine):

- [`cargo-audit`](https://github.com/rustsec/rustsec/tree/main/cargo-audit) —
  `cargo install cargo-audit --locked`
- [`rumdl`](https://github.com/rvben/rumdl) — `cargo install rumdl --locked`
- [`ryl`](https://crates.io/crates/ryl) — `cargo install ryl --locked`

If any are missing, `xtask` prints an install hint before failing.

### `cargo xtask ci` — run this before finishing

**`ci` is the always-on gate.** It runs every check that must stay green in
local work and in CI. Individual subcommands (`fmt-check`, `clippy`, `test`,
etc.) exist so you can run one step while iterating; they are not a substitute
for `ci`.

`cargo xtask ci` runs, in order:

1. `check-toolchain` — pin matches [`rust-toolchain.toml`](rust-toolchain.toml)
2. `fmt-check` — `cargo fmt --all --check`
3. `check` — `cargo check --workspace --all-targets`
4. `clippy` — `cargo clippy --workspace --all-targets -- -D warnings`
5. `test` — `cargo test --workspace`
6. `audit` — `cargo audit` (requires
   [`cargo-audit`](https://github.com/rustsec/rustsec/tree/main/cargo-audit))
7. `rumdl check --respect-gitignore .`
8. `ryl .`

Later phases add parser/renderer gates (`parse`, `render`, `link-check`,
golden checks) to `ci`; those are not part of the core seam gate yet.

### Fix commands (not CI)

- **`cargo xtask fmt`** — apply fixes: `cargo fmt --all`, `rumdl fmt` on touched
  markdown (or the whole tree), `ryl --fix` where applicable.
- **`cargo xtask fmt-check`**, **`clippy`**, **`test`** — single-step shortcuts;
  same flags as the matching step inside `ci`.

Parser/renderer gates (`parse`, `render`, `link-check`, golden checks) are not
part of the core seam gate yet.
