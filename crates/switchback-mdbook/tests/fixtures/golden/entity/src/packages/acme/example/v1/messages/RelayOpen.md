# RelayOpen

RelayOpen carries handshake metadata for RelayConnect.

```protobuf
message [RelayOpen](RelayOpen.md) {
  string session_name = 1;
  acme.example.v2.[ResourceIdentity](../../v2/messages/ResourceIdentity.md) identity = 2;
  acme.example.v2.[QuotaLimits](../../v2/messages/QuotaLimits.md) requested_limits = 3;
  repeated acme.example.v2.[Label](../../v2/messages/Label.md) labels = 4;
}
```

