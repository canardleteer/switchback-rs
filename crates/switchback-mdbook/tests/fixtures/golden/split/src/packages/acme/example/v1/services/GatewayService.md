# GatewayService

GatewayService focuses on streaming patterns and catalog adjacency.

**RelayConnect** ( [RelayConnectRequest](../messages/RelayConnectRequest.md) ) returns ( [RelayConnectResponse](../messages/RelayConnectResponse.md) )

RelayConnect opens a bidirectional stream for frame exchange.

```protobuf
rpc RelayConnect (stream acme.example.v1.[RelayConnectRequest](../messages/RelayConnectRequest.md)) returns (stream acme.example.v1.[RelayConnectResponse](../messages/RelayConnectResponse.md));
```

**PublishEvents** ( [PublishEventsRequest](../messages/PublishEventsRequest.md) ) returns ( [PublishEventsResponse](../messages/PublishEventsResponse.md) )

PublishEvents accepts a client stream of PublishEventsRequest messages.

```protobuf
rpc PublishEvents (stream acme.example.v1.[PublishEventsRequest](../messages/PublishEventsRequest.md)) returns (acme.example.v1.[PublishEventsResponse](../messages/PublishEventsResponse.md));
```

**ListCatalogProducts** ( [ListCatalogProductsRequest](../messages/ListCatalogProductsRequest.md) ) returns ( [ListCatalogProductsResponse](../messages/ListCatalogProductsResponse.md) )

ListCatalogProducts bridges to v2 catalog types for cross-package links.

```protobuf
rpc ListCatalogProducts (acme.example.v1.[ListCatalogProductsRequest](../messages/ListCatalogProductsRequest.md)) returns (acme.example.v1.[ListCatalogProductsResponse](../messages/ListCatalogProductsResponse.md));
```

