# ExperimentSpec

ExperimentSpec defines arms and eligibility for assignment RPCs.

```protobuf
message [ExperimentSpec](ExperimentSpec.md) {
  
  string experiment_id = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.uuid = true
    ];
  string hypothesis = 2;
  repeated [ExperimentArm](ExperimentArm.md) arms = 3 [(buf.validate.field).repeated.min_items = 2];
  acme.example.v2.[TimeWindow](../../v2/messages/TimeWindow.md) enrollment_window = 4;
  acme.example.v2.[LabelSet](../../v2/messages/LabelSet.md) audience = 5;
}
```

**Protovalidate (CEL)**

```cel
id: "experiment.arms_weights_positive"
      message: "each arm must have positive weight in basis points"
      expression: "this.arms.all(arm, arm.weight_basis_points > 0u)"
```

