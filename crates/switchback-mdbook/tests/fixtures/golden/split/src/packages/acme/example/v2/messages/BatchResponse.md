# BatchResponse

BatchResponse collects results for batch RPC documentation.

```protobuf
message [BatchResponse](BatchResponse.md) {
  repeated [BatchItemResult](BatchItemResult.md) results = 1;
  [PageResult](PageResult.md) page = 2;
}
```

