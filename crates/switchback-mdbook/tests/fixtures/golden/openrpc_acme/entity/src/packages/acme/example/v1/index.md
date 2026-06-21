# Acme APIs

Echo, streaming, and gateway entry for the Acme documentation fixture.

## Operations

- [Echo a message (unary)](operations/echoUnary.md)
- [Open relay session (server SSE + client POST frames)](operations/relayConnect.md)
- [Proxy list products (v2 catalog types)](operations/listCatalogProducts.md)
- [Send one inbound relay frame](operations/relaySendFrame.md)
- [Stream echo chunks (SSE)](operations/echoServerStream.md)
- [Upload echo parts (octet-stream)](operations/echoClientStream.md)
- [Watch echo events (NDJSON)](operations/watchEcho.md)

## Schemas

- [Schema EchoStreamChunk](schemas/EchoStreamChunk.md)
- [Schema EchoUnaryRequest](schemas/EchoUnaryRequest.md)
- [Schema EchoUnaryResponse](schemas/EchoUnaryResponse.md)
- [Schema EchoUploadResponse](schemas/EchoUploadResponse.md)
- [Schema Problem](schemas/Problem.md)
- [Schema RelayFrame](schemas/RelayFrame.md)
- [Schema RelayOpen](schemas/RelayOpen.md)
- [Schema WatchEchoEvent](schemas/WatchEchoEvent.md)

## Parameters

- [Parameter Authorization](parameters/Authorization.md)
- [Parameter XRequestId](parameters/XRequestId.md)

