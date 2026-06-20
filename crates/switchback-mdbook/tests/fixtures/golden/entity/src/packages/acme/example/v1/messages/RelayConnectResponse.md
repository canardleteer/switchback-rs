# RelayConnectResponse

RelayConnectResponse is one server-to-client frame in RelayConnect.

```protobuf
message [RelayConnectResponse](RelayConnectResponse.md) {
  oneof payload {
      [RelayAck](RelayAck.md) ack = 1;
      [RelayFrame](RelayFrame.md) frame = 2;
      [RelayClose](RelayClose.md) close = 3;
    }
}
```

