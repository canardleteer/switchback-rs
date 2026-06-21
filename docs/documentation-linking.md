# Documentation linking in switchback-rs

This document names the two cross-reference channels in the switchback model,
how they move through parse → wire → render, and what they look like in each
[contract family](GLOSSARY.md#contract-family). For hierarchy terms see
[GLOSSARY.md](GLOSSARY.md); this file is the linking-specific companion.

**Status:** protobuf and OpenAPI structural `refs` reflect current behavior.
OpenAPI, AsyncAPI, and
OpenRPC examples are **synthetic** — intended naming and shape targets until
those parsers land.

---

## Two channels (do not conflate the names)

Switchback uses **two different words** for two different mechanisms:

| Channel | Meaning | Stored as | Produced by | Rendered by |
| --- | --- | --- | --- | --- |
| **Structural reference** | Cross-ref encoded in contract **shape** (schema graph, RPC I/O, `$ref`) | `StoredEntity.refs` → `Reference` | **Populate** (parser, deterministic) | Renderer uses `refs` + layout index (`LinkContext`) |
| **Intra-link** | Cross-ref an **author writes in prose** (description, comment, doc field) | `StoredEntity.intra_links` → `IntraLink` | **Extract** (`LinkExtractor`, heuristic / configurable) | `apply_intra_links` + `LinkFormatter` |

**Rule of thumb:** if removing the prose would not change whether the link
exists, it is almost certainly a **structural reference**. If it only appears
because someone typed it in documentation, it is an **intra-link**.

Both channels resolve to the same address types (`EntityRef`, `GroupRef`, …) and
both are formatted for output by a `LinkFormatter` (today:
`MdBookRelativeFormatter`, name `"mdbook-relative"`). They are stored and
produced differently.

See also [GLOSSARY.md § intra-link](GLOSSARY.md#intra-link) and
[GLOSSARY.md § structural reference](GLOSSARY.md#structural-reference).

---

## Core types (Rust / wire)

| Name | Role |
| --- | --- |
| `Reference` | One structural cross-ref; `target: EntityRef`, `kind: RefKind` |
| `RefKind` | `Internal`, `External`, `Component`, `Inline`, … |
| `IntraLink` | One prose link; `anchor` (field + byte span), `target: LinkTarget`, `raw` |
| `Anchor` | Locates link text inside a field (`doc`, `fence_body`, …) |
| `LinkTarget` | Resolved destination (`Entity`, `Group`, `External`, `Unresolved`, …) |
| `LinkExtractor` | Family parser trait: prose → `Vec<IntraLink>` |
| `LinkFormatter` | Renderer trait: `LinkTarget` → output string (markdown link, URL, …) |
| `LinkContext` | Layout-aware path index built from manual + `Options` |

`LinkTarget::Unresolved` is in-memory only; codecs strip it before wire
serialize.

Implementation lives in `crates/switchback-traits/` (`model/link.rs`,
`traits/link.rs`, `link_context.rs`, `intra_links.rs`).

---

## Lifecycle

```text
Source contract
  │
  ├─ populate ──► Entity.refs (structural)
  │
  └─ assemble manual + ResolvedManual
         │
         └─ LinkExtractor::extract ──► Entity.intra_links (prose)
                │
                ▼
         ReferenceManual (switchback.binpb)
                │
                └─ Renderer + LinkContext + LinkFormatter
                       ├─ splice intra_links in prose fields
                       └─ link structural refs in fences / signatures / bodies
```

---

## protobuf

**Default extractor:** `ProtobufLinkExtractor` (alias
`ProtobufFqnLinkExtractor`), name `"protobuf-fqn"`. Implemented in
`crates/switchback-protobuf/src/link.rs`.

### Structural reference (`refs`)

Deterministic: derived from descriptor shape during populate.

**RPC I/O types** (operation entity):

```protobuf
service DocumentService {
  // Intra-link example in prose — see below.
  rpc DoSomething(Document) returns (Document);
}
```

- Populate records `Reference`s for input/output FQNs on the **operation**
  entity (`operation_refs`).
- Stored `fence_body` holds pre-synthesized syntax; the renderer links type
  tokens via `LinkContext` (and uses `refs` for signature lines).

**Message field types** (schema entity):

```protobuf
message Document {
  acme.example.v2.SharedMetadata metadata = 1;
}
```

- Populate records `Reference`s for each linkable `type_name` on the **message**
  entity (`message_field_refs`).

Wire shape (conceptual):

```text
StoredEntity {
  name: "DoSomething"
  category: "operation"
  refs: [
    { target: EntityRef { group: "acme.example.v1", name: "Document", ... },
      kind: Internal },
    { target: EntityRef { group: "acme.example.v1", name: "Document", ... },
      kind: Internal },
  ]
  intra_links: []
}
```

### Intra-link (`intra_links`)

Heuristic: extracted from **prose fields** (today: entity `doc`) by
`LinkExtractor`, not from RPC/fence syntax.

```protobuf
// EchoUnaryRequest carries the unary payload.
//
// Fields reference `acme.example.v2.SharedMetadata` for trace identifiers.
message EchoUnaryRequest { ... }
```

- Extractor finds bare FQN substrings matching `package.Type` pattern.
- Emits `IntraLink { anchor: doc[byte_start..byte_end], target: Entity(...),
  raw }`.
- Renderer splices formatted link at anchor via `apply_intra_links`.

**Not an intra-link today:** bare `` `Document` `` or short names in comments
without FQN — those would need a different named extractor (future).

---

## openapi

**Default extractor:** `OpenApiLinkExtractor` (name `"openapi"`). Prose
intra-links are **deferred** — extractor returns empty `intra_links` today.

### Structural reference (`refs`) — implemented

From OpenAPI component graph and operation/request/response wiring.

**Operation → schema ref:**

```yaml
paths:
  /pets/{id}:
    get:
      summary: Get a pet
      responses:
        "200":
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
```

- Populate: operation entity `refs` includes internal ref to
  `components/schemas/Pet` (`RefKind::Component` or `Internal`).

**Schema `$ref` chain:**

```yaml
components:
  schemas:
    Pet:
      properties:
        owner:
          $ref: "#/components/schemas/Person"
```

- Populate: `Pet` schema entity `refs` includes ref to `Person`.

### Intra-link (`intra_links`)

From author prose in `description`, `summary`, or extension doc fields.

```yaml
paths:
  /pets/{id}:
    get:
      description: |
        Returns a [Pet](#/components/schemas/Pet) owned by the caller.
        See also `#/components/schemas/Person`.
```

- Extractor (planned): recognize markdown links, bare JSON Pointers, or
  `components/schemas/Pet` strings in prose.
- Emit `IntraLink` with anchor in `doc`; resolve against `ResolvedManual`.

---

## asyncapi *(synthetic — parser not implemented)*

**Planned default extractor:** `AsyncApiLinkExtractor` (stub today, name
`"asyncapi-stub"`).

### Structural reference (`refs`)

From channel/message/payload graph.

**Message payload ref:**

```yaml
channels:
  user/signedup:
    publish:
      message:
        payload:
          $ref: "#/components/schemas/UserSignedUp"
```

- Populate: operation (or message) entity `refs` → `UserSignedUp` schema.

**Schema field ref:** same pattern as OpenAPI (`$ref` in payload properties).

### Intra-link (`intra_links`)

From prose in channel or message `description`.

```yaml
components:
  messages:
    UserSignedUp:
      description: |
        Emitted when a `User` completes signup. Payload matches
        `#/components/schemas/UserSignedUp`.
```

- Extractor (planned): AsyncAPI doc conventions + shared JSON Pointer / `$ref`
  prose rules with OpenAPI where possible.

---

## openrpc

**Default extractor:** `OpenRpcLinkExtractor` (name `"openrpc"`). Intra-link
extraction is deferred; populate still records structural `refs`.

### Structural reference (`refs`)

From method `params` / `result` JSON Schema refs and component schemas.

```json
{
  "methods": [
    {
      "name": "echoUnary",
      "params": [
        {
          "name": "request",
          "schema": { "$ref": "#/components/schemas/EchoUnaryRequest" }
        }
      ],
      "result": { "$ref": "#/components/schemas/EchoUnaryResponse" }
    }
  ]
}
```

- Populate: method entity `refs` → `EchoUnaryRequest`, `EchoUnaryResponse`
  schema entities in the same group (or resolved cross-file target group).

### Intra-link (`intra_links`)

Deferred in the first behavior parser. Method `summary` / `description` prose
does not yet populate `intra_links`.

### Entity naming

| OpenRPC source | Entity category | `StoredEntity.name` |
| --- | --- | --- |
| `methods[].name` | `operation` | method name (e.g. `echoUnary`) |
| `components.schemas.*` | `schema` | schema key |
| `components.contentDescriptors.*` | `parameter` | descriptor key |

### Grouping

- **Single-entry:** `x-tagGroup` sections plus `default` and `components`
  groups.
- **Multi-entry Acme:** one group per entry via `info.x-switchback-group` or
  `vN/openrpc.json` path convention → `acme.example.vN`.

---

## Multi-variant extractors and formatters

Each family may ship **more than one** `LinkExtractor` over time (e.g.
protobuf `"protobuf-fqn"` today; `"manual://"` later). The default is
`ContractFamily::link_extractor()`. Additional extractors are named statics;
selection is not a dynamic registry yet.

Renderers select `LinkFormatter` via `Options::link_format` (default
`"mdbook-relative"`). Other formatters (`html-absolute`, `json`, `passthrough`)
remain deferred.

---

## When to update this document

Update this file whenever you change:

- `Reference`, `IntraLink`, `LinkTarget`, `RefKind`, or related wire shapes
- `LinkExtractor` / `LinkFormatter` trait contracts or default family impls
- Populate rules for structural `refs` in any parser
- Render-time linking behavior in `switchback-mdbook` or `LinkContext`

See
[AGENTS.md](https://github.com/canardleteer/switchback-rs/blob/main/AGENTS.md).
