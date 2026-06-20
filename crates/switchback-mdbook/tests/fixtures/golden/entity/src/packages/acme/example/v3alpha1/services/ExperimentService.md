# ExperimentService

ExperimentService documents assignment and streaming experiment RPCs.

**AssignExperiment** ( [AssignExperimentRequest](../messages/AssignExperimentRequest.md) ) returns ( [AssignExperimentResponse](../messages/AssignExperimentResponse.md) )

AssignExperiment picks an arm for a subject.

```protobuf
rpc AssignExperiment (acme.example.v3alpha1.[AssignExperimentRequest](../messages/AssignExperimentRequest.md)) returns (acme.example.v3alpha1.[AssignExperimentResponse](../messages/AssignExperimentResponse.md));
```

**StreamAssignments** ( [StreamAssignmentsRequest](../messages/StreamAssignmentsRequest.md) ) returns ( [StreamAssignmentsResponse](../messages/StreamAssignmentsResponse.md) )

StreamAssignments pushes assignment events for an experiment.

```protobuf
rpc StreamAssignments (acme.example.v3alpha1.[StreamAssignmentsRequest](../messages/StreamAssignmentsRequest.md)) returns (stream acme.example.v3alpha1.[StreamAssignmentsResponse](../messages/StreamAssignmentsResponse.md));
```

