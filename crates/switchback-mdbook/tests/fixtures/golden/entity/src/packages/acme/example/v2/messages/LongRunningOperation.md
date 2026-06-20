# LongRunningOperation

LongRunningOperation is returned by admin-style RPCs in v1.

```protobuf
message [LongRunningOperation](LongRunningOperation.md) {
  string name = 1;
  [OperationStatus](../enums/OperationStatus.md) status = 2;
  google.protobuf.Timestamp start_time = 3;
  google.protobuf.Timestamp end_time = 4;
  [RetryPolicy](RetryPolicy.md) retry_policy = 5;
  [ErrorDetail](ErrorDetail.md) error = 6;
  double percent_complete = 7;
}
```

