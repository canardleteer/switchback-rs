# PayloadHeader

PayloadHeader precedes user bytes in envelope examples.

```protobuf
message [PayloadHeader](PayloadHeader.md) {
  string content_type = 1;
  uint64 content_length = 2;
  string checksum_sha256 = 3;
  [Priority](../enums/Priority.md) priority = 4;
}
```

