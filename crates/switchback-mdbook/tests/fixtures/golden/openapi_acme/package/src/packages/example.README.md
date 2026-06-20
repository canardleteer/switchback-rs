# Example services

Documentation for versioned HTTP APIs under `acme/example/`, mirroring the
protobuf Acme packages (`acme.example.v1`, `.v2`, `.v3alpha1`).

| Entry | OpenAPI | Group id |
| --- | --- | --- |
| `v1/openapi.yaml` | 3.1.0 | `acme.example.v1` |
| `v2/openapi.yaml` | 3.0.3 | `acme.example.v2` |
| `v3alpha1/openapi.yaml` | 3.1.0 | `acme.example.v3alpha1` |

Each entry declares a live `servers[]` URL (`https://api.acme.example/v1`,
`/v2`, `/v3alpha1`). Cross-entry `$ref` links exercise multifile resolution
during populate (for example v1 catalog proxy → v2 schemas, v3alpha1 pagination
→ v2).
