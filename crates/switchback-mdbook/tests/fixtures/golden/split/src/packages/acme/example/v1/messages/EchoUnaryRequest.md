# EchoUnaryRequest

EchoUnaryRequest carries the unary payload.

 Fields reference `[SharedMetadata](../../v2/messages/SharedMetadata.md)` for trace identifiers.
 See also `[BatchRequest](../../v2/messages/BatchRequest.md)` for multi-item batch uploads.

```protobuf
message [EchoUnaryRequest](EchoUnaryRequest.md) {
// User-visible text to echo back.
// 
//  Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
//  commodo consequat.
  string message = 1;
  acme.example.v2.[SharedMetadata](../../v2/messages/SharedMetadata.md) metadata = 2;
  oneof _locale {
    optional string locale = 3;
  }
  repeated string tags = 4;
  acme.example.v2.[EchoExtension](../../v2/messages/EchoExtension.md) extension = 5;
}
```

