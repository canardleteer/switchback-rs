# EchoExtension

EchoExtension carries optional hints from v2 into v1 echo flows.

```protobuf
message [EchoExtension](EchoExtension.md) {
  string locale = 1;
  repeated string keywords = 2;
  map<string, string> annotations = 3;
  [NumericRange](NumericRange.md) length_bounds = 4;
}
```

