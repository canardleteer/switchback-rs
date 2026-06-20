# FilterExpression

FilterExpression is a intentionally verbose filter AST placeholder.

```protobuf
message [FilterExpression](FilterExpression.md) {
  string field = 1;
  string op = 2;
  string value = 3;
  repeated [FilterExpression](FilterExpression.md) children = 4;
}
```

