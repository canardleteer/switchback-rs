# switchback-assemble

> [!WARNING]
> Early prototype while exploring design and aiming for equivalence
> with [protobuf-mdbook](https://github.com/canardleteer/protobuf-mdbook), while
> expanding scope through traits and intermediary on-disk representation.
>
> This is not ready for adoption, nor even stable at a `v1alpha1` yet. You'll
> want to keep eyes on the repository for development.
>
> A lot of this is clanker driven, so vetting a good human read through pass
> hasn't been completed yet.

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
