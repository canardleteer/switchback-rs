# Address

Address is a postal-style structure used in nested messages.

```protobuf
message [Address](Address.md) {
  string line1 = 1;
  string line2 = 2;
  string city = 3;
  string postal_code = 4;
  [Location](Location.md) region = 5;
}
```

