# Acme Example API v3alpha1

Alpha feature flags, experiments, and pipeline orchestration.

## Operations

- [/experiments/{experiment_id}/assign](operations/POST%20-experiments-%7Bexperiment_id%7D-assign.md)
- [/experiments/{experiment_id}/assignments/stream](operations/GET%20-experiments-%7Bexperiment_id%7D-assignments-stream.md)
- [/feature-flags](operations/GET%20-feature-flags.md)
- [/feature-flags](operations/PUT%20-feature-flags.md)
- [/pipelines](operations/POST%20-pipelines.md)
- [/pipelines/{run_id}](operations/GET%20-pipelines-%7Brun_id%7D.md)
- [/pipelines/{run_id}/watch](operations/GET%20-pipelines-%7Brun_id%7D-watch.md)

## Schemas

- [AssignExperimentRequest](schemas/AssignExperimentRequest.md)
- [ExperimentAssignment](schemas/ExperimentAssignment.md)
- [FeatureFlag](schemas/FeatureFlag.md)
- [FlagOverride](schemas/FlagOverride.md)
- [ListFeatureFlagsResponse](schemas/ListFeatureFlagsResponse.md)
- [PipelineRun](schemas/PipelineRun.md)
- [PipelineStatus](schemas/PipelineStatus.md)
- [PipelineStepResult](schemas/PipelineStepResult.md)
- [Problem](schemas/Problem.md)
- [ReleaseChannel](schemas/ReleaseChannel.md)
- [StartPipelineRequest](schemas/StartPipelineRequest.md)

## Parameters

- [ExperimentId](parameters/ExperimentId.md)
- [RunId](parameters/RunId.md)

## Responses

- [BadRequest](responses/BadRequest.md)
- [NotFound](responses/NotFound.md)

