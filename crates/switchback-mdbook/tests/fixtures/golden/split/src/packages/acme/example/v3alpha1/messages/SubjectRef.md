# SubjectRef

SubjectRef identifies a user or device for experiment assignment.

```protobuf
message [SubjectRef](SubjectRef.md) {
  oneof identity {
      string user_id = 1 [(buf.validate.field).string.uuid = true];
      string device_id = 2 [(buf.validate.field).string.min_len = 8];
    }
  acme.example.v2.[TenantRef](../../v2/messages/TenantRef.md) tenant = 3;
}
```

