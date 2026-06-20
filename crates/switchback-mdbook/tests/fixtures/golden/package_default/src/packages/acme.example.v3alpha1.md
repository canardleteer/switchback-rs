# acme.example.v3alpha1

## Services

### FeatureFlagService

FeatureFlagService documents flag and override RPCs.

**ListFeatureFlags** ( [ListFeatureFlagsRequest](#listfeatureflagsrequest) ) returns ( [ListFeatureFlagsResponse](#listfeatureflagsresponse) )

```protobuf
rpc ListFeatureFlags (acme.example.v3alpha1.[ListFeatureFlagsRequest](#listfeatureflagsrequest)) returns (acme.example.v3alpha1.[ListFeatureFlagsResponse](#listfeatureflagsresponse));
```

ListFeatureFlags returns a paginated flag catalog page.

**UpsertFlagOverride** ( [UpsertFlagOverrideRequest](#upsertflagoverriderequest) ) returns ( [UpsertFlagOverrideResponse](#upsertflagoverrideresponse) )

```protobuf
rpc UpsertFlagOverride (acme.example.v3alpha1.[UpsertFlagOverrideRequest](#upsertflagoverriderequest)) returns (acme.example.v3alpha1.[UpsertFlagOverrideResponse](#upsertflagoverrideresponse));
```

UpsertFlagOverride stores a tenant override row.

**PublishHints** ( [PublishHintsRequest](#publishhintsrequest) ) returns ( [PublishHintsResponse](#publishhintsresponse) )

```protobuf
rpc PublishHints (stream acme.example.v3alpha1.[PublishHintsRequest](#publishhintsrequest)) returns (acme.example.v3alpha1.[PublishHintsResponse](#publishhintsresponse));
```

PublishHints accepts a client stream of platform hints.

**SyncFlags** ( [SyncFlagsRequest](#syncflagsrequest) ) returns ( [SyncFlagsResponse](#syncflagsresponse) )

```protobuf
rpc SyncFlags (stream acme.example.v3alpha1.[SyncFlagsRequest](#syncflagsrequest)) returns (stream acme.example.v3alpha1.[SyncFlagsResponse](#syncflagsresponse));
```

SyncFlags synchronizes flag revisions over a bidirectional stream.

### ExperimentService

ExperimentService documents assignment and streaming experiment RPCs.

**AssignExperiment** ( [AssignExperimentRequest](#assignexperimentrequest) ) returns ( [AssignExperimentResponse](#assignexperimentresponse) )

```protobuf
rpc AssignExperiment (acme.example.v3alpha1.[AssignExperimentRequest](#assignexperimentrequest)) returns (acme.example.v3alpha1.[AssignExperimentResponse](#assignexperimentresponse));
```

AssignExperiment picks an arm for a subject.

**StreamAssignments** ( [StreamAssignmentsRequest](#streamassignmentsrequest) ) returns ( [StreamAssignmentsResponse](#streamassignmentsresponse) )

```protobuf
rpc StreamAssignments (acme.example.v3alpha1.[StreamAssignmentsRequest](#streamassignmentsrequest)) returns (stream acme.example.v3alpha1.[StreamAssignmentsResponse](#streamassignmentsresponse));
```

StreamAssignments pushes assignment events for an experiment.

### PipelineService

PipelineService documents orchestration RPCs in the alpha package.

**StartPipeline** ( [StartPipelineRequest](#startpipelinerequest) ) returns ( [StartPipelineResponse](#startpipelineresponse) )

```protobuf
rpc StartPipeline (acme.example.v3alpha1.[StartPipelineRequest](#startpipelinerequest)) returns (acme.example.v3alpha1.[StartPipelineResponse](#startpipelineresponse));
```

StartPipeline creates a pipeline run from staged inputs.

**WatchPipeline** ( [WatchPipelineRequest](#watchpipelinerequest) ) returns ( [WatchPipelineResponse](#watchpipelineresponse) )

```protobuf
rpc WatchPipeline (acme.example.v3alpha1.[WatchPipelineRequest](#watchpipelinerequest)) returns (stream acme.example.v3alpha1.[WatchPipelineResponse](#watchpipelineresponse));
```

WatchPipeline streams step results for a run.

**CancelPipeline** ( [CancelPipelineRequest](#cancelpipelinerequest) ) returns ( [CancelPipelineResponse](#cancelpipelineresponse) )

```protobuf
rpc CancelPipeline (acme.example.v3alpha1.[CancelPipelineRequest](#cancelpipelinerequest)) returns (acme.example.v3alpha1.[CancelPipelineResponse](#cancelpipelineresponse));
```

CancelPipeline stops a run by identifier.

## Messages and enums

### PipelineStepInput

PipelineStepInput is one unit of work inside a pipeline run.

```protobuf
message [PipelineStepInput](#pipelinestepinput) {
  string step_name = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.min_len = 1,
      (buf.validate.field).string.max_len = 64
    ];
  acme.example.v2.[PayloadEnvelope](acme.example.v2.md#payloadenvelope) input = 2;
  [RolloutStage](#rolloutstage) rollout = 3;
}
```

### PipelineStepResult

PipelineStepResult reports completion for a single step.

```protobuf
message [PipelineStepResult](#pipelinestepresult) {
  string step_name = 1;
  [PipelineStatus](#pipelinestatus) status = 2;
  acme.example.v2.[ErrorDetail](acme.example.v2.md#errordetail) error = 3;
  acme.example.v2.[StreamCursor](acme.example.v2.md#streamcursor) cursor = 4;
  google.protobuf.Timestamp finished_at = 5;
}
```

### PipelineStage

PipelineStage is a oneof-heavy stage definition for doc rendering.

```protobuf
message [PipelineStage](#pipelinestage) {
  oneof stage {
      [PipelineStepInput](#pipelinestepinput) step = 1;
      [RolloutStage](#rolloutstage) rollout_only = 2;
      acme.example.v2.[AuditBatch](acme.example.v2.md#auditbatch) audit_snapshot = 3;
    }
}
```

### PipelineRun

PipelineRun identifies a long-lived orchestration instance.

```protobuf
message [PipelineRun](#pipelinerun) {
  
  string run_id = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.uuid = true
    ];
  string pipeline_name = 2 [(buf.validate.field).string.min_len = 1];
  [PipelineStatus](#pipelinestatus) status = 3;
  repeated [PipelineStepResult](#pipelinestepresult) results = 4;
  acme.example.v2.[ResourceIdentity](acme.example.v2.md#resourceidentity) owner = 5;
  google.protobuf.Timestamp started_at = 6;
  google.protobuf.Timestamp completed_at = 7;
}
```

**Protovalidate (CEL)**

```cel
id: "pipeline_run.completed_after_started"
      message: "completed_at must not precede started_at when both are set"
      expression: "!has(this.completed_at) || !has(this.started_at) || this.completed_at >= this.started_at"
```

### StartPipelineRequest

StartPipelineRequest kicks off a pipeline from staged inputs.

```protobuf
message [StartPipelineRequest](#startpipelinerequest) {
  string pipeline_name = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.pattern = "^[a-z][a-z0-9-]*$"
    ];
  repeated [PipelineStage](#pipelinestage) stages = 2 [(buf.validate.field).repeated.min_items = 1];
  acme.example.v2.[ResourceIdentity](acme.example.v2.md#resourceidentity) actor = 3;
}
```

### StartPipelineResponse

StartPipelineResponse returns the created run handle.

```protobuf
message [StartPipelineResponse](#startpipelineresponse) {
  [PipelineRun](#pipelinerun) run = 1;
}
```

### WatchPipelineRequest

WatchPipelineRequest subscribes to pipeline step events.

```protobuf
message [WatchPipelineRequest](#watchpipelinerequest) {
  string run_id = 1 [(buf.validate.field).string.uuid = true];
  acme.example.v2.[TimeWindow](acme.example.v2.md#timewindow) window = 2;
  repeated acme.example.v2.[FilterExpression](acme.example.v2.md#filterexpression) filters = 3;
}
```

### WatchPipelineResponse

WatchPipelineResponse is one event on the WatchPipeline server stream.

```protobuf
message [WatchPipelineResponse](#watchpipelineresponse) {
  string event_id = 1;
  [PipelineStepResult](#pipelinestepresult) step = 2;
  google.protobuf.Timestamp observed_at = 3;
  acme.example.v2.[StreamCursor](acme.example.v2.md#streamcursor) cursor = 4;
}
```

### CancelPipelineRequest

CancelPipelineRequest stops a run by id.

```protobuf
message [CancelPipelineRequest](#cancelpipelinerequest) {
  string run_id = 1 [(buf.validate.field).string.uuid = true];
  string reason = 2 [(buf.validate.field).string.max_len = 512];
}
```

### CancelPipelineResponse

CancelPipelineResponse acknowledges cancellation.

```protobuf
message [CancelPipelineResponse](#cancelpipelineresponse) {
  [PipelineRun](#pipelinerun) run = 1;
}
```

### PipelineStatus

PipelineStatus tracks synthetic workflow execution.

```protobuf
enum [PipelineStatus](#pipelinestatus) {
  PIPELINE_STATUS_UNSPECIFIED = 0;
  PIPELINE_STATUS_QUEUED = 1;
  PIPELINE_STATUS_RUNNING = 2;
  PIPELINE_STATUS_SUCCEEDED = 3;
  PIPELINE_STATUS_FAILED = 4;
  PIPELINE_STATUS_CANCELLED = 5;
}
```

### ListFeatureFlagsRequest

ListFeatureFlagsRequest paginates feature flags for a tenant.

```protobuf
message [ListFeatureFlagsRequest](#listfeatureflagsrequest) {
  acme.example.v2.[ListOptions](acme.example.v2.md#listoptions) options = 1;
  acme.example.v2.[TenantRef](acme.example.v2.md#tenantref) tenant = 2;
  [ReleaseChannel](#releasechannel) channel_filter = 3;
}
```

### ListFeatureFlagsResponse

ListFeatureFlagsResponse returns a page of flags.

```protobuf
message [ListFeatureFlagsResponse](#listfeatureflagsresponse) {
  repeated [FeatureFlag](#featureflag) flags = 1;
  acme.example.v2.[PageResult](acme.example.v2.md#pageresult) page = 2;
}
```

### UpsertFlagOverrideRequest

UpsertFlagOverrideRequest writes a tenant-specific override.

```protobuf
message [UpsertFlagOverrideRequest](#upsertflagoverriderequest) {
  [FlagOverride](#flagoverride) override = 1;
}
```

### UpsertFlagOverrideResponse

UpsertFlagOverrideResponse echoes the stored override.

```protobuf
message [UpsertFlagOverrideResponse](#upsertflagoverrideresponse) {
  [FlagOverride](#flagoverride) override = 1;
}
```

### AssignExperimentRequest

AssignExperimentRequest assigns a subject to an experiment arm.

```protobuf
message [AssignExperimentRequest](#assignexperimentrequest) {
  [ExperimentSpec](#experimentspec) spec = 1;
  [SubjectRef](#subjectref) subject = 2;
  repeated string exclusion_keys = 3 [(buf.validate.field).repeated.max_items = 32];
}
```

### AssignExperimentResponse

AssignExperimentResponse returns the assignment row.

```protobuf
message [AssignExperimentResponse](#assignexperimentresponse) {
  [ExperimentAssignment](#experimentassignment) assignment = 1;
}
```

### StreamAssignmentsRequest

StreamAssignmentsRequest opens a server stream of assignment events.

```protobuf
message [StreamAssignmentsRequest](#streamassignmentsrequest) {
  string experiment_id = 1 [(buf.validate.field).string.uuid = true];
  acme.example.v2.[TimeWindow](acme.example.v2.md#timewindow) window = 2;
}
```

### StreamAssignmentsResponse

StreamAssignmentsResponse is one assignment event on the stream.

```protobuf
message [StreamAssignmentsResponse](#streamassignmentsresponse) {
  [ExperimentAssignment](#experimentassignment) assignment = 1;
  acme.example.v2.[StreamCursor](acme.example.v2.md#streamcursor) cursor = 2;
  google.protobuf.Timestamp observed_at = 3;
}
```

### PublishHintsRequest

PublishHintsRequest is one client-streaming platform hint chunk.

```protobuf
message [PublishHintsRequest](#publishhintsrequest) {
  string batch_id = 1;
  [PlatformHint](#platformhint) hint = 2;
  uint32 part_index = 3;
  bool last_part = 4;
}
```

### PublishHintsResponse

PublishHintsResponse aggregates uploaded hint parts.

```protobuf
message [PublishHintsResponse](#publishhintsresponse) {
  string batch_id = 1;
  uint32 parts_received = 2;
  acme.example.v2.[PageResult](acme.example.v2.md#pageresult) page = 3;
}
```

### SyncFlagsRequest

SyncFlagsRequest is one frame in a bidirectional flag sync session.

```protobuf
message [SyncFlagsRequest](#syncflagsrequest) {
  uint64 sequence = 1;
  [FeatureFlag](#featureflag) flag = 2;
  bool fin = 3;
}
```

### SyncFlagsResponse

SyncFlagsResponse mirrors a bidirectional flag sync frame.

```protobuf
message [SyncFlagsResponse](#syncflagsresponse) {
  uint64 sequence = 1;
  acme.example.v2.[StreamCursor](acme.example.v2.md#streamcursor) cursor = 2;
  acme.example.v2.[ErrorDetail](acme.example.v2.md#errordetail) error = 3;
  bool fin = 4;
}
```

### FeatureFlag

FeatureFlag describes a toggle evaluated by fictional clients.

```protobuf
message [FeatureFlag](#featureflag) {
// Stable identifier (slug).
  string key = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.min_len = 1,
      (buf.validate.field).string.max_len = 128,
      (buf.validate.field).string.pattern = "^[a-z][a-z0-9_.-]*$"
    ];
  string display_name = 2 [(buf.validate.field).string.max_len = 256];
  string description = 3;
  bool default_enabled = 4;
  [ReleaseChannel](#releasechannel) channel = 5;
  acme.example.v2.[SharedMetadata](acme.example.v2.md#sharedmetadata) metadata = 6;
  google.protobuf.Timestamp updated_at = 7;
}
```

### FlagOverride

FlagOverride pins a flag value for one tenant.

```protobuf
message [FlagOverride](#flagoverride) {
  acme.example.v2.[TenantRef](acme.example.v2.md#tenantref) tenant = 1;
  string flag_key = 2 [(buf.validate.field).string.min_len = 1];
  bool enabled = 3;
  google.protobuf.Timestamp expires_at = 4;
}
```

### ExperimentArm

ExperimentArm is one variant in an A/B style experiment.

```protobuf
message [ExperimentArm](#experimentarm) {
  string arm_id = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.pattern = "^arm_[a-z0-9]+$"
    ];
  string label = 2 [(buf.validate.field).string.max_len = 64];
  uint32 weight_basis_points = 3 [
      (buf.validate.field).uint32.gte = 0,
      (buf.validate.field).uint32.lte = 10000
    ];
  map<string, string> parameters = 4;
}
```

### ExperimentSpec

ExperimentSpec defines arms and eligibility for assignment RPCs.

```protobuf
message [ExperimentSpec](#experimentspec) {
  
  string experiment_id = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.uuid = true
    ];
  string hypothesis = 2;
  repeated [ExperimentArm](#experimentarm) arms = 3 [(buf.validate.field).repeated.min_items = 2];
  acme.example.v2.[TimeWindow](acme.example.v2.md#timewindow) enrollment_window = 4;
  acme.example.v2.[LabelSet](acme.example.v2.md#labelset) audience = 5;
}
```

**Protovalidate (CEL)**

```cel
id: "experiment.arms_weights_positive"
      message: "each arm must have positive weight in basis points"
      expression: "this.arms.all(arm, arm.weight_basis_points > 0u)"
```

### ExperimentAssignment

ExperimentAssignment records which arm a subject received.

```protobuf
message [ExperimentAssignment](#experimentassignment) {
  string experiment_id = 1;
  string subject_id = 2 [(buf.validate.field).string.min_len = 1];
  string arm_id = 3;
  google.protobuf.Timestamp assigned_at = 4;
  acme.example.v2.[TraceContext](acme.example.v2.md#tracecontext) trace = 5;
}
```

### SubjectRef

SubjectRef identifies a user or device for experiment assignment.

```protobuf
message [SubjectRef](#subjectref) {
  oneof identity {
      string user_id = 1 [(buf.validate.field).string.uuid = true];
      string device_id = 2 [(buf.validate.field).string.min_len = 8];
    }
  acme.example.v2.[TenantRef](acme.example.v2.md#tenantref) tenant = 3;
}
```

### RolloutStage

RolloutStage documents progressive delivery for pipeline stories.

```protobuf
message [RolloutStage](#rolloutstage) {
  [ReleaseChannel](#releasechannel) channel = 1;
  google.protobuf.Duration bake_time = 2;
  uint32 max_parallel = 3 [(buf.validate.field).uint32.lte = 1000];
}
```

### PlatformHint

PlatformHint links alpha metadata back into v2 envelopes.

```protobuf
message [PlatformHint](#platformhint) {
  string hint_id = 1;
  acme.example.v2.[PayloadEnvelope](acme.example.v2.md#payloadenvelope) envelope = 2;
  repeated acme.example.v2.[Label](acme.example.v2.md#label) labels = 3;
}
```

### ReleaseChannel

ReleaseChannel classifies how aggressively a flag rolls out.

```protobuf
enum [ReleaseChannel](#releasechannel) {
  RELEASE_CHANNEL_UNSPECIFIED = 0;
  RELEASE_CHANNEL_INTERNAL = 1;
  RELEASE_CHANNEL_CANARY = 2;
  RELEASE_CHANNEL_STABLE = 3;
}
```

