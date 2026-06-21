# Acme Example API v3alpha1 (alpha)

`acme.example.v3alpha1` — feature flags, experiment assignments, and pipeline
orchestration for alpha-channel documentation tests.

## Feature flags

List and upsert tenant overrides.

## Experiments

Assignment unary RPC plus `streamAssignments` for assignment events.

## Pipelines

`startPipeline` starts a long-running run; poll with `getPipelineRun` or
subscribe with `watchPipeline`.

See `DEV-NOTES.md` for alpha caveats.
