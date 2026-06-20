# RelayClose

RelayClose ends a relay session gracefully.

```protobuf
message [RelayClose](RelayClose.md) {
  string session_id = 1;
  string reason = 2;
}
```

