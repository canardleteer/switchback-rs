# ErrorDetail

ErrorDetail mirrors a simplified rich error shape for docs.

```protobuf
message [ErrorDetail](ErrorDetail.md) {
  string code = 1;
  string message = 2;
  map<string, string> metadata = 3;
  repeated string help_links = 4;
}
```

