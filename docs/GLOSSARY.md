# Glossary

Terms used across the switchback-rs toolchain. Nesting levels run from the
whole manual down to individual renderable units; cross-cutting kinds describe
shapes and behavior inside a contract.

## Hierarchy

Each [contract family](#contract-family) groups its ④ [documents](#document)
and parser crate. A **parser** fills the [switchback](#switchback); a
**renderer** turns it into the ① [reference manual](#reference-manual) (mdBook).
Numbers match the [nesting levels](#nesting-levels) table below.

```mermaid
flowchart TB
  classDef outcome fill:#14532d,stroke:#052e16,stroke-width:4px,color:#fff,font-weight:bold
  classDef role fill:#dbeafe,stroke:#1d4ed8,stroke-width:2px
  classDef document fill:#fef3c7,stroke:#b45309,stroke-width:1px

  subgraph families["Contract families"]
    direction LR

    subgraph fam_pb["protobuf"]
      direction TB
      D1["④ .proto files"]
      L1["switchback-protobuf"]
      D1 --> L1
    end

    subgraph fam_o["OpenAPI"]
      direction TB
      D2["④ openapi.yaml + $refs"]
      L2["switchback-openapi"]
      D2 --> L2
    end

    subgraph fam_a["AsyncAPI"]
      direction TB
      D3["④ asyncapi.yaml + $refs"]
      L3["switchback-asyncapi"]
      D3 --> L3
    end

    subgraph fam_r["OpenRPC"]
      direction TB
      D4["④ openrpc.json + $refs"]
      L4["switchback-openrpc"]
      D4 --> L4
    end
  end

  parse["Parser<br/>switchback-{family}"]

  subgraph SW["switchback (switchback binary file)"]
    direction LR
    subgraph source["Source layer"]
      src["④ documents<br/>verbatim input files"]
    end
    subgraph derived["Derived layer — glossary nesting"]
      direction TB
      g2["② module<br/>may span families"]
      g3["③ contract<br/>one per family"]
      g5["⑤ group<br/>package · tag · app · x-tagGroup"]
      g6["⑥ entity<br/>schema · operation · …"]
      g2 --> g3 --> g5 --> g6
    end
    src -.->|"feeds"| g3
  end

  render["Renderer<br/>switchback-mdbook → mdBook"]
  g1["① reference manual<br/>(mdBook)"]

  families --> parse --> SW --> render --> g1

  class g1 outcome
  class parse,render role
  class D1,D2,D3,D4,src document
```

## Nesting levels

| # | Term | Meaning | Example |
|---|---|---|---|
| 1 | [reference manual](#reference-manual) | The rendered artifact. Contains one or more modules. | the `api-book/` for "Acme Platform" |
| 2 | [module](#module) | A cohesive documentation unit that may span contract families. Becomes a top-level part. | "UserService" with a gRPC contract and an OpenAPI contract |
| 3 | [contract](#contract) | One family's description of a module. The merge of one or more documents. Belongs to one [contract family](#contract-family). | the protobuf IDL of UserService; the OpenAPI Description of UserService |
| 4 | [document](#document) | A single input file. | `user.proto`, `openapi.yaml`, `asyncapi.json` |
| 5 | [group](#group) | Intra-contract grouping: a protobuf package, an OpenAPI tag/`x-tagGroup`, an AsyncAPI application/tag. | `acme.user.v1`, the `admin` tag group |
| 6 | [entity](#entity) | An addressable renderable unit. | a schema, an operation, a channel, a message, a parameter, a response, a security scheme |

## Cross-cutting kinds

| Term | Meaning |
|---|---|
| [schema](#schema) | A data-shape definition inside a contract (JSON Schema object, protobuf message/enum shape). Never used for the whole document. |
| [operation](#operation) | The unit of behavior: a gRPC method, an HTTP operation, an AsyncAPI operation, a JSON-RPC method. |
| [component](#component) | A named, reusable entity declared once in a contract and referenced by name wherever it is reused. OpenAPI, AsyncAPI, and OpenRPC store components under a `components` object and reference them with `$ref`; protobuf has no `components` object and instead references top-level messages, enums, and services by fully-qualified name. A component is always an entity; an entity is a component only if it is named and reused by reference. |

## Terms

### anchor

A byte span into a specific prose field (`doc`, `fence_body`, or a named body
field) of an [entity](#entity). Part of an [intra-link](#intra-link); lets a
[renderer](#renderer) splice a resolved link without re-parsing the prose.

### companion

A markdown file discovered beside contract inputs and copied verbatim into the
reference manual. Companion discovery and placement rules are owned by each
[contract family](#contract-family) via its companion strategy. On the wire,
each companion stores `title`, `source_dir`, and `stem` nav metadata (see
[ADR 0009](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0009-companion-nav-metadata-on-wire-in-switchback-traits.md))
so [renderers](#renderer) can build navigation without re-parsing companion
bytes or re-walking source trees. How each target format uses those fields (for
example mdBook `SUMMARY.md` nesting) is renderer-specific.

### component

See the [cross-cutting kinds](#cross-cutting-kinds) table.

### contract

One [contract family](#contract-family)'s description of a service or
application. A contract may span several input [documents](#document) (for
example an `openapi.yaml` and the files it `$ref`s). The term is deliberately
not *schema*, because JSON Schema definitions inside OpenAPI and AsyncAPI are
already called schemas, and the documents themselves are more than schemas.

### contract family

A specification family whose documents parsers understand: Protobuf, OpenAPI,
AsyncAPI, OpenRPC, or a JSON Schema catalog. Each family has one
`ContractFamily` trait impl (identity, defaults, categories, link syntax) and
one `Contract` trait impl per loaded instance.

### derived layer

The resolved `ReferenceManual` / `Module` / `Contract` / `Group` / `Entity`
tree inside a [switchback](#switchback), with `$ref`s already resolved into
structural cross-references. [Renderers](#renderer) consume the derived layer.

### document

A single input file: `user.proto`, `openapi.yaml`, `asyncapi.json`, and so on.
One or more documents merge into a [contract](#contract).

### entity

An addressable renderable unit inside a [group](#group): a schema, an
operation, a channel, a message, a parameter, a response, a security scheme, and
so on. Each [contract family](#contract-family) defines its own typed
[entity category](#entity-category) enum.

### entity category

A family-owned label for an [entity](#entity)'s kind (`schema`, `operation`,
`channel`, …). Implemented as the `EntityCategory` trait in
`switchback-traits`; the core never holds a closed enum of categories.

### envelope

The shared API-description object model lifted into `switchback-jsonschema`:
`info`, `servers`, `components`, `security`, `tags`, `externalDocs`, and
related fields common to OpenAPI, AsyncAPI, and OpenRPC documents.

### generic category

Renderer-known entity categories (`Schema`, `Operation`, `Service`, `Generic`)
that `EntityCategory::to_generic` maps family-specific categories onto.
Categories with no mapping render through the generic fallback.

### group

The intra-[contract](#contract) grouping unit: a protobuf package, an OpenAPI
tag or `x-tagGroup`, an AsyncAPI application or tag.

### intra-link

A cross-reference an author writes in prose (inside a `description`, fenced
snippet, or comment). Distinct from structural `Reference`s in `Entity.refs`,
which come from the contract's schema shape (`$ref`, protobuf field types).
Intra-links are pre-resolved at ingest time; [renderers](#renderer) format them
for the output target rather than re-deriving the target.

### link extractor

The `LinkExtractor` trait: one impl per [contract family](#contract-family).
Walks an entity's prose fields, finds author-syntax links, and emits resolved
[intra-links](#intra-link) against the whole manual's address space.

### link formatter

The `LinkFormatter` trait: turns a resolved intra-link target into a concrete
string for a [renderer](#renderer)'s output format (markdown relative link, HTML
URL, JSON for IDE plugins, and so on).

### module

A cohesive documentation unit that may span
[contract families](#contract-family). A module becomes a top-level part in a
[reference manual](#reference-manual). Described explicitly by a
[module manifest](#module-manifest) or synthesized implicitly when a single
contract is presented alone.

### module manifest

A `module.yaml` file beside the contracts that names the module, its overview,
and the contracts that belong to it. Parsed on the parser side before the
[switchback](#switchback) is frozen; recorded in the switchback [source
layer](#source-layer) so the manual is reproducible.

### operation

See the [cross-cutting kinds](#cross-cutting-kinds) table.

### parser

A `switchback-{family}` crate that turns a [contract](#contract) into a
[switchback](#switchback). Implements `ContractFamily`, `Contract`, and
`LinkExtractor`. Always emits a
[switchback binary file](#switchback-binary-file) unless `--no-switchback` is
set.

### reference manual

The rendered artifact the toolchain produces (for example, an mdBook). Contains
one or more [modules](#module). The term is output-format neutral.

### renderer

A `switchback-{target}` crate that turns a [switchback](#switchback) into a
target format. Implements the `Renderer` trait. May run standalone on a
switchback binary file or be invoked in-process by a parser (`--render mdbook`).

### schema

See the [cross-cutting kinds](#cross-cutting-kinds) table.

### seam

`switchback-traits`: the core crate between parsers and renderers. Owns the
trait spine, the in-memory model, format-agnostic helpers, and the
`SwitchbackCodec` trait. Knows nothing about any contract family, output
format, or serialization format.

### source layer

The `repeated Document sources` inside a [switchback](#switchback). Each
`Document` carries raw as-authored bytes, a media type, and a
[SourceRef](#sourceref). Keeps the switchback lossless: anything not modeled in
the [derived layer](#derived-layer) remains recoverable from the source bytes.

### SourceRef

Provenance for a [document](#document) in the [source layer](#source-layer):
`{ uri, commit, content_hash }`. The content hash (SHA-256, hex) makes
"stable" verifiable so consumers can detect drift after the switchback was
built.

### structural reference

A cross-reference encoded in an [entity](#entity)'s schema shape (a `$ref` JSON
Pointer, a protobuf fully-qualified type name in a field). Lands in
`Entity.refs` and is distinct from an [intra-link](#intra-link).

### switchback

The versioned, lossless intermediate representation between [parsers](#parser)
and [renderers](#renderer). Every parser emits it; every renderer reads it. The
name is the metaphor of a hairpin turn on a mountain trail: climb from source
[contracts](#contract) through parsing, pivot at the switchback, then either
return to verbatim source [documents](#document) or continue into any rendered
form.

### switchback binary file

The canonical serialized form of a [switchback](#switchback), produced by a
`SwitchbackCodec` implementation (for example `switchback-codec-pb`).
Deterministic, cacheable, and the only artifact both parser and renderer sides
must agree on.

### SwitchbackCodec

The serialize/deserialize trait for a [switchback](#switchback).
`switchback-codec-pb` implements it as the reference binary codec using types
compiled from
`crates/switchback-codec-pb/proto/canardleteer/switchback/v1alpha1/switchback.proto`
(`canardleteer.switchback.v1alpha1`; repo-root `proto/` symlinks to this tree).

## Vocabulary by contract family

How the nesting and cross-cutting terms map onto each supported family:

| Concept | protobuf | OpenAPI | AsyncAPI | OpenRPC |
|---|---|---|---|---|
| reference manual | the rendered manual | the rendered manual | the rendered manual | the rendered manual |
| module | the service the `.proto` set describes | the service the OpenAPI Description describes | the application | the service the OpenRPC document describes |
| contract | the `.proto` file set (compiled together) | OpenAPI Description (entry doc + `$ref`d files) | AsyncAPI document (entry doc + `$ref`d files) | OpenRPC document (entry doc + `$ref`d files) |
| document | one `.proto` file | one `openapi.yaml`/`.json` or referenced file | one `asyncapi.yaml`/`.json` or referenced file | one `openrpc.json`/`.yaml` or referenced file |
| group | protobuf package | tag / `x-tagGroup` | application `id` / tag | `x-tagGroup` |
| operation | service method | HTTP operation | operation (send/receive) | method |
| behavior surface | service | paths + webhooks | channels + operations | methods |
| data shape | message / enum | Schema Object | Schema Object / Multi-Format Schema | Content Descriptor |
| component (reusable) | top-level message/enum/service by FQN | `components.*` by `$ref` | `components.*` by `$ref` | `components.*` by `$ref` |
| reference | fully-qualified name | `$ref` JSON Pointer | `$ref` JSON Pointer | `$ref` JSON Pointer |
| transport | gRPC over HTTP/2 | HTTP | protocol bindings | JSON-RPC 2.0 |
| validation hook | Protovalidate (CEL) | JSON Schema + `--validate` | JSON Schema + `--validate` | JSON Schema + `--validate` |
