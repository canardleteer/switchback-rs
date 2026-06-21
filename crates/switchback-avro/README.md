# switchback-avro

> [!WARNING]
> Early prototype while exploring design and aiming for equivalence
> with [protobuf-mdbook](https://github.com/canardleteer/protobuf-mdbook), while
> expanding scope through traits and intermediary on-disk representation.
>
> This is not ready for adoption, nor even stable at a `v1alpha1` yet. You'll
> want to keep eyes on the repository for development.

Shared Avro schema document layer for the switchback-rs toolchain.

`switchback-avro` parses Avro JSON schema payloads (AsyncAPI `avroSchema_v1`,
Avro `.avsc` JSON encoding) into IR and produces **schema entity bodies** for
downstream family parsers such as `switchback-asyncapi`. It is a sibling shared
layer to `switchback-jsonschema` for JSON Schema documents.

## Usage

```rust
use serde_json::json;
use switchback_avro::{populate_avro_schema_body, AvroSchema};

let value = json!({
    "type": "record",
    "name": "User",
    "fields": [
        { "name": "id", "type": "long" },
        { "name": "name", "type": "string" }
    ]
});
let schema = AvroSchema::from_value(&value);
let body = populate_avro_schema_body(
    &value,
    "acme.events",
    "components",
    Some("application/vnd.apache.avro+json"),
);
assert_eq!(body.payload_format, "application/vnd.apache.avro+json");
```

Vendored Avro format meta-schemas live under `meta-schemas/` with SHA-256 locks
in `meta-schemas.lock.toml` (see ADR 0018).
