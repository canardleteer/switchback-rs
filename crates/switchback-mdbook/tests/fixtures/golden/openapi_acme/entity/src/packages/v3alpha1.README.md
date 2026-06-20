# Acme Example API v3alpha1 (alpha)

`acme.example.v3alpha1` — feature flags, experiment assignments, and pipeline
orchestration for alpha-channel documentation tests.

## Feature flags

List and upsert tenant overrides; pagination reuses v2 `PageResult` via `$ref`.

## Experiments

Assignment unary RPC plus `GET /experiments/{experiment_id}/assignments/stream`
for SSE assignment events.

## Pipelines

`POST /pipelines` starts a long-running run; poll `GET /pipelines/{run_id}` or
subscribe with `GET /pipelines/{run_id}/watch` (SSE step events).

See `DEV-NOTES.md` for alpha caveats.
