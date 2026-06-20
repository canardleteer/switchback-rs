# switchback-assemble

Merge OpenAPI and protobuf (and future family) loads into one
[`ReferenceManual`](https://github.com/canardleteer/switchback-rs) module with
multiple contracts.

See
[ADR 0014](https://github.com/canardleteer/switchback-rs/blob/main/docs/adr/0014-multi-contract-reference-manual-assembly.md)
for group-id prefix policy and the MVP `module.yaml` shape used by
`examples/reference-manual`.

```rust
use switchback_assemble::{assemble_module, AssembleArgs, GroupPrefixPolicy};
use switchback_openapi::load::LoadArgs as OpenApiLoadArgs;
use switchback_protobuf::load::LoadArgs as ProtobufLoadArgs;

let manual = assemble_module(&AssembleArgs {
    module_id: "acme".into(),
    title: "Acme APIs".into(),
    overview: "HTTP and gRPC reference for Acme v1.".into(),
    group_prefix: GroupPrefixPolicy::ContractFamily,
    openapi: Some(openapi_args),
    protobuf: Some(protobuf_args),
})?;
```
