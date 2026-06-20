# GatewayService

*`acme/example/v1/gateway.proto`*

GatewayService focuses on streaming patterns and catalog adjacency.

**RelayConnect** ( [RelayConnectRequest](../messages/RelayConnectRequest.md) ) returns ( [RelayConnectResponse](../messages/RelayConnectResponse.md) )

RelayConnect opens a bidirectional stream for frame exchange.

**PublishEvents** ( [PublishEventsRequest](../messages/PublishEventsRequest.md) ) returns ( [PublishEventsResponse](../messages/PublishEventsResponse.md) )

PublishEvents accepts a client stream of PublishEventsRequest messages.

**ListCatalogProducts** ( [ListCatalogProductsRequest](../messages/ListCatalogProductsRequest.md) ) returns ( [ListCatalogProductsResponse](../messages/ListCatalogProductsResponse.md) )

ListCatalogProducts bridges to v2 catalog types for cross-package links.

