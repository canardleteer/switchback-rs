# 2. Async-first traits with synchronous secondary APIs in switchback-traits

Date: 2026-06-19

## Status

Accepted

## Context

The switchback-rs toolchain must support async I/O: URL $ref fetching, remote
schema resolution, service-side rendering pipelines, and IDE-plugin-style
streaming.

Every core trait that performs I/O needs both a sync and an async variety:

- `Contract` (sync `groups()`/`entities()` vs async loading from remote sources)
- `Renderer` (sync `render()` vs async streaming render)
- `SwitchbackCodec` (sync `serialize`/`deserialize` vs async I/O-backed codec)
- `LinkExtractor` (sync `extract()` vs async resolution of cross-manual links)
- `CompanionStrategy` (sync discovery vs async fetch from remote sources)

`ContractFamily` is pure metadata (no I/O) and does not need an async variety.
switchback-traits is the seam every parser and renderer depends on; the concrete
Rust pattern (paired traits, runtime-generic trait, or async-block-based
approach) must be pinned here before parser or renderer crates ship.

## Decision

**Async boundary safety (all seam types).** Regardless of whether a trait
exposes sync methods, async methods, or both, every type at the
switchback-traits seam must be able to traverse async task boundaries:

- Trait definitions are `Send` (and `Sync` where the trait is shared via `&dyn`
  across tasks).
- Async trait methods return `Send` futures unless a specific API documents
  otherwise.
- Model and option types (`ReferenceManual`, `ResolvedManual`, `OutputFile`,
  `LinkContext`, `Options`, entity bodies, link targets, and the rest) are
  `Send` (and `Sync` when shared immutably across tasks).

Sync-only entry points do not relax this requirement. Callers using
`SyncRenderer` and callers using `Renderer::render` must both be able to move
the same trait objects and model values through async pipelines when needed.

**I/O traits: async-first with sync secondary.** Define I/O-bearing traits
async-first using native async fn in trait (Rust 1.85+). Each async trait is the
primary API for service-side tooling and any caller that already runs inside an
async runtime.

Provide synchronous secondary APIs alongside the async traits, not instead of
them:

- Paired sync traits (e.g. `Renderer` and `SyncRenderer`) or sync methods on
  the same type with a `blocking_` prefix, chosen per trait during
  implementation.
- Sync APIs exist for **compatibility**: external integrations that cannot host
  an async runtime or wrap the async traits themselves. First-party tooling
  should prefer the async APIs.
- Sync APIs may wrap or block on the async implementation where the work is
  local and cheap (in-memory render, local file codec, local companion
  discovery).
- Callers that cannot or should not block (remote fetch, streaming render,
  long-running service handlers) use the async API only.

**Non-I/O traits.** `ContractFamily` does not need an async variety. Other
non-I/O traits (e.g. `EntityCategory`, `LinkFormatter`) expose sync methods
only and remain subject to the async boundary safety rules above.

## Consequences

Positive:

- Pins the async/sync trait pattern for switchback-traits before downstream
  crates ship.
- One coherent async story for service-side tooling without bolting async on
  later.
- External callers blocked on sync-only integration can implement sync
  secondary traits without forking the seam.
- All seam types — sync API or async — can move across async task boundaries in
  service deployments and MCP-adjacent tooling.

Negative:

- More surface area: each I/O trait ships twice (async primary + sync
  secondary).
- Sync wrappers must document when blocking is acceptable; misuse can stall
  async runtimes.
- Trait and model authors must satisfy `Send`/`Sync` bounds in addition to
  choosing async vs sync entry points.

Neutral:

- switchback-traits gains a minimal async runtime dependency only if needed for
  shared helpers; prefer std/async fn in trait without pulling in tokio at the
  seam unless tests require it.
- An ADR link from AGENTS.md and the switchback-traits implementation plan
  replaces the earlier sync-only scaffold approach.
