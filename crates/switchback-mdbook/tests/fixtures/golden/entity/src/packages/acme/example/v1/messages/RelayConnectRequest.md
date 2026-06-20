# RelayConnectRequest

RelayConnectRequest is one client-to-server frame in RelayConnect.

```protobuf
message [RelayConnectRequest](RelayConnectRequest.md) {
  oneof payload {
      [RelayOpen](RelayOpen.md) open = 1;
      [RelayFrame](RelayFrame.md) frame = 2;
      [RelayClose](RelayClose.md) close = 3;
    }
}
```

