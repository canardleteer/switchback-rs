# ListOptions

ListOptions bundles pagination and sorting for list RPCs.

```protobuf
message [ListOptions](ListOptions.md) {
  [PageToken](PageToken.md) page = 1;
  repeated [SortKey](SortKey.md) sort = 2;
  repeated [FilterExpression](FilterExpression.md) filters = 3;
}
```

