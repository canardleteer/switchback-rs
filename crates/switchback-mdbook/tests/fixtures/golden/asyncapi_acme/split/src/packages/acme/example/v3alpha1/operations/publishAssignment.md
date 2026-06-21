# Publish experiment assignment

**SEND** `experiments~1assignments`

```mermaid
sequenceDiagram
  participant Client
  participant Broker as experiments~1assignments
  Client->>Broker: send (publishAssignment)
```

```yaml
action: send
channel:
  $ref: "#/channels/experiments~1assignments"
messages:
- $ref: "#/channels/experiments~1assignments/messages/assignmentCreated"
summary: Publish experiment assignment
```

