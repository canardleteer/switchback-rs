# acme.example.v2

## Services

### CatalogService

CatalogService documents commerce and inventory RPCs in the v2 package.

**ListProducts** ( [ListProductsRequest](#listproductsrequest) ) returns ( [ListProductsResponse](#listproductsresponse) )

```protobuf
rpc ListProducts (acme.example.v2.[ListProductsRequest](#listproductsrequest)) returns (acme.example.v2.[ListProductsResponse](#listproductsresponse));
```

ListProducts returns a paginated product catalog page.

**GetProduct** ( [GetProductRequest](#getproductrequest) ) returns ( [GetProductResponse](#getproductresponse) )

```protobuf
rpc GetProduct (acme.example.v2.[GetProductRequest](#getproductrequest)) returns (acme.example.v2.[GetProductResponse](#getproductresponse));
```

GetProduct fetches a single product by identifier.

**ApplyInventoryAdjustments** ( [ApplyInventoryAdjustmentsRequest](#applyinventoryadjustmentsrequest) ) returns ( [ApplyInventoryAdjustmentsResponse](#applyinventoryadjustmentsresponse) )

```protobuf
rpc ApplyInventoryAdjustments (acme.example.v2.[ApplyInventoryAdjustmentsRequest](#applyinventoryadjustmentsrequest)) returns (acme.example.v2.[ApplyInventoryAdjustmentsResponse](#applyinventoryadjustmentsresponse));
```

ApplyInventoryAdjustments applies batched stock deltas.

**WatchInventory** ( [WatchInventoryRequest](#watchinventoryrequest) ) returns ( [WatchInventoryResponse](#watchinventoryresponse) )

```protobuf
rpc WatchInventory (acme.example.v2.[WatchInventoryRequest](#watchinventoryrequest)) returns (stream acme.example.v2.[WatchInventoryResponse](#watchinventoryresponse));
```

WatchInventory streams inventory adjustments for a warehouse.

**UploadDrafts** ( [UploadDraftsRequest](#uploaddraftsrequest) ) returns ( [UploadDraftsResponse](#uploaddraftsresponse) )

```protobuf
rpc UploadDrafts (stream acme.example.v2.[UploadDraftsRequest](#uploaddraftsrequest)) returns (acme.example.v2.[UploadDraftsResponse](#uploaddraftsresponse));
```

UploadDrafts accepts a client stream of draft SKU payloads.

**SyncCatalog** ( [SyncCatalogRequest](#synccatalogrequest) ) returns ( [SyncCatalogResponse](#synccatalogresponse) )

```protobuf
rpc SyncCatalog (stream acme.example.v2.[SyncCatalogRequest](#synccatalogrequest)) returns (stream acme.example.v2.[SyncCatalogResponse](#synccatalogresponse));
```

SyncCatalog synchronizes catalog revisions over a bidirectional stream.

### PlatformService

PlatformService documents cross-cutting telemetry and health RPCs.

**ExportAuditBatch** ( [ExportAuditBatchRequest](#exportauditbatchrequest) ) returns ( [ExportAuditBatchResponse](#exportauditbatchresponse) )

```protobuf
rpc ExportAuditBatch (acme.example.v2.[ExportAuditBatchRequest](#exportauditbatchrequest)) returns (acme.example.v2.[ExportAuditBatchResponse](#exportauditbatchresponse));
```

ExportAuditBatch returns a snapshot of audit records (unary).

**StreamAuditRecords** ( [StreamAuditRecordsRequest](#streamauditrecordsrequest) ) returns ( [StreamAuditRecordsResponse](#streamauditrecordsresponse) )

```protobuf
rpc StreamAuditRecords (acme.example.v2.[StreamAuditRecordsRequest](#streamauditrecordsrequest)) returns (stream acme.example.v2.[StreamAuditRecordsResponse](#streamauditrecordsresponse));
```

StreamAuditRecords pushes audit rows over a server stream.

**IngestTelemetry** ( [IngestTelemetryRequest](#ingesttelemetryrequest) ) returns ( [IngestTelemetryResponse](#ingesttelemetryresponse) )

```protobuf
rpc IngestTelemetry (stream acme.example.v2.[IngestTelemetryRequest](#ingesttelemetryrequest)) returns (acme.example.v2.[IngestTelemetryResponse](#ingesttelemetryresponse));
```

IngestTelemetry accepts a client stream of metric samples.

**GetAggregateHealth** ( [GetAggregateHealthRequest](#getaggregatehealthrequest) ) returns ( [GetAggregateHealthResponse](#getaggregatehealthresponse) )

```protobuf
rpc GetAggregateHealth (acme.example.v2.[GetAggregateHealthRequest](#getaggregatehealthrequest)) returns (acme.example.v2.[GetAggregateHealthResponse](#getaggregatehealthresponse));
```

GetAggregateHealth returns synthetic component health.

## Messages and enums

### ProductSku

ProductSku identifies a sellable item in documentation tables.

```protobuf
message [ProductSku](#productsku) {
  string sku = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.min_len = 1,
      (buf.validate.field).string.max_len = 64
    ];
  string title = 2;
  string description = 3;
  [Money](#money) price = 4;
  [ProductStatus](#productstatus) status = 5;
  repeated [Label](#label) labels = 6;
}
```

### Money

Money represents a decimal amount with currency code.

```protobuf
message [Money](#money) {
  int64 units = 1;
  int32 nanos = 2;
  string currency_code = 3 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.len = 3
    ];
}
```

### Product

Product bundles many SKUs for list RPC examples.

```protobuf
message [Product](#product) {
  string product_id = 1;
  string display_name = 2;
  repeated [ProductSku](#productsku) skus = 3;
  [TenantRef](#tenantref) owner = 4;
  google.protobuf.Timestamp created_at = 5;
  google.protobuf.Timestamp updated_at = 6;
  [Address](#address) warehouse = 7;
}
```

### ListProductsRequest

ListProductsRequest paginates catalog inventory.

```protobuf
message [ListProductsRequest](#listproductsrequest) {
  [ListOptions](#listoptions) options = 1;
  [ProductStatus](#productstatus) status_filter = 2;
  string search_query = 3 [(buf.validate.field).string.max_len = 256];
}
```

### ListProductsResponse

ListProductsResponse returns a page of products.

```protobuf
message [ListProductsResponse](#listproductsresponse) {
  repeated [Product](#product) products = 1;
  [PageResult](#pageresult) page = 2;
}
```

### GetProductRequest

GetProductRequest fetches a single product by id.

```protobuf
message [GetProductRequest](#getproductrequest) {
  string product_id = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.uuid = true
    ];
}
```

### GetProductResponse

GetProductResponse returns one product.

```protobuf
message [GetProductResponse](#getproductresponse) {
  [Product](#product) product = 1;
}
```

### InventoryAdjustment

InventoryAdjustment documents stock changes for admin RPCs.

```protobuf
message [InventoryAdjustment](#inventoryadjustment) {
  string sku = 1;
  int64 delta = 2;
  string reason = 3;
  google.protobuf.Timestamp adjusted_at = 4;
}
```

### ApplyInventoryAdjustmentsRequest

ApplyInventoryAdjustmentsRequest batches adjustments.

```protobuf
message [ApplyInventoryAdjustmentsRequest](#applyinventoryadjustmentsrequest) {
  repeated [InventoryAdjustment](#inventoryadjustment) adjustments = 1;
  [ResourceIdentity](#resourceidentity) actor = 2;
}
```

### ApplyInventoryAdjustmentsResponse

ApplyInventoryAdjustmentsResponse summarizes applied rows.

```protobuf
message [ApplyInventoryAdjustmentsResponse](#applyinventoryadjustmentsresponse) {
  uint32 applied = 1;
  repeated [ErrorDetail](#errordetail) failures = 2;
}
```

### ProductStatus

ProductStatus lifecycle for catalog stories.

```protobuf
enum [ProductStatus](#productstatus) {
  PRODUCT_STATUS_UNSPECIFIED = 0;
  PRODUCT_STATUS_DRAFT = 1;
  PRODUCT_STATUS_ACTIVE = 2;
  PRODUCT_STATUS_ARCHIVED = 3;
  PRODUCT_STATUS_DISCONTINUED = 4;
}
```

### WatchInventoryRequest

WatchInventoryRequest subscribes to inventory change events.

```protobuf
message [WatchInventoryRequest](#watchinventoryrequest) {
  string warehouse_id = 1 [
      (buf.validate.field).required = true,
      (buf.validate.field).string.min_len = 1,
      (buf.validate.field).string.max_len = 128
    ];
  [TimeWindow](#timewindow) window = 2;
  repeated [FilterExpression](#filterexpression) filters = 3;
}
```

### WatchInventoryResponse

WatchInventoryResponse is one event on the WatchInventory server stream.

```protobuf
message [WatchInventoryResponse](#watchinventoryresponse) {
  string event_id = 1;
  [InventoryAdjustment](#inventoryadjustment) adjustment = 2;
  google.protobuf.Timestamp observed_at = 3;
  [StreamCursor](#streamcursor) cursor = 4;
}
```

### UploadDraftsRequest

UploadDraftsRequest is one client-streaming product draft chunk.

```protobuf
message [UploadDraftsRequest](#uploaddraftsrequest) {
  string draft_id = 1;
  [ProductSku](#productsku) sku = 2;
  uint32 part_index = 3;
  bool last_part = 4;
}
```

### UploadDraftsResponse

UploadDraftsResponse aggregates uploaded draft parts.

```protobuf
message [UploadDraftsResponse](#uploaddraftsresponse) {
  string draft_id = 1;
  uint32 parts_received = 2;
  [Product](#product) product = 3;
}
```

### SyncCatalogRequest

SyncCatalogRequest is one frame in a bidirectional catalog sync session.

```protobuf
message [SyncCatalogRequest](#synccatalogrequest) {
  uint64 sequence = 1;
  string catalog_revision = 2;
  [PayloadEnvelope](#payloadenvelope) payload = 3;
  bool fin = 4;
}
```

### SyncCatalogResponse

SyncCatalogResponse mirrors a bidirectional catalog sync frame.

```protobuf
message [SyncCatalogResponse](#synccatalogresponse) {
  uint64 sequence = 1;
  [StreamCursor](#streamcursor) cursor = 2;
  [ErrorDetail](#errordetail) error = 3;
  bool fin = 4;
}
```

### ExportAuditBatchRequest

ExportAuditBatchRequest exports a batch of audit rows (unary).

```protobuf
message [ExportAuditBatchRequest](#exportauditbatchrequest) {
  [ResourceIdentity](#resourceidentity) identity = 1;
  [TimeWindow](#timewindow) window = 2;
  [ListOptions](#listoptions) options = 3;
}
```

### ExportAuditBatchResponse

ExportAuditBatchResponse returns exported audit data.

```protobuf
message [ExportAuditBatchResponse](#exportauditbatchresponse) {
  [AuditBatch](#auditbatch) batch = 1;
  [PageResult](#pageresult) page = 2;
}
```

### StreamAuditRecordsRequest

StreamAuditRecordsRequest opens a server stream of audit rows.

```protobuf
message [StreamAuditRecordsRequest](#streamauditrecordsrequest) {
  [TenantRef](#tenantref) tenant = 1;
  repeated [FilterExpression](#filterexpression) filters = 2;
}
```

### StreamAuditRecordsResponse

StreamAuditRecordsResponse is one audit row on the stream.

```protobuf
message [StreamAuditRecordsResponse](#streamauditrecordsresponse) {
  [AuditRecord](#auditrecord) record = 1;
  [StreamCursor](#streamcursor) cursor = 2;
}
```

### IngestTelemetryRequest

IngestTelemetryRequest is one client-streaming telemetry point.

```protobuf
message [IngestTelemetryRequest](#ingesttelemetryrequest) {
  string metric_name = 1;
  double value = 2;
  google.protobuf.Timestamp observed_at = 3;
  [LabelSet](#labelset) labels = 4;
}
```

### IngestTelemetryResponse

IngestTelemetryResponse acknowledges ingested points.

```protobuf
message [IngestTelemetryResponse](#ingesttelemetryresponse) {
  uint64 accepted = 1;
  uint64 rejected = 2;
}
```

### GetAggregateHealthRequest

GetAggregateHealthRequest is an empty platform health request.

```protobuf
message [GetAggregateHealthRequest](#getaggregatehealthrequest) {
}
```

### GetAggregateHealthResponse

GetAggregateHealthResponse returns rolled-up health from types.proto.

```protobuf
message [GetAggregateHealthResponse](#getaggregatehealthresponse) {
  [AggregateHealth](#aggregatehealth) health = 1;
}
```

### SharedMetadata

SharedMetadata is referenced from v1 echo and gateway messages.

 Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque
 laudantium. Keep `trace_id` opaque to callers; format is service-specific.

```protobuf
message [SharedMetadata](#sharedmetadata) {
// Distributed trace id (W3C `traceparent` or equivalent).
  string trace_id = 1 [(buf.validate.field).string.min_len = 1];
// Optional parent span for nested calls.
  string parent_span_id = 2;
// When the trace context was minted.
  google.protobuf.Timestamp created_at = 3;
// Arbitrary baggage for examples (not a production pattern).
  map<string, string> baggage = 4;
// Classification for routing in docs.
  [SharedKind](#sharedkind) kind = 5;
}
```

### TraceContext

TraceContext duplicates some metadata for nested RPC examples.

```protobuf
message [TraceContext](#tracecontext) {
  string trace_id = 1;
  string span_id = 2;
  google.protobuf.Duration sampling_delay = 3;
}
```

### PayloadHeader

PayloadHeader precedes user bytes in envelope examples.

```protobuf
message [PayloadHeader](#payloadheader) {
  string content_type = 1;
  uint64 content_length = 2;
  string checksum_sha256 = 3;
  [Priority](#priority) priority = 4;
}
```

### PayloadEnvelope

PayloadEnvelope wraps opaque bytes with metadata.

```protobuf
message [PayloadEnvelope](#payloadenvelope) {
  [PayloadHeader](#payloadheader) header = 1;
  bytes body = 2;
  [SharedMetadata](#sharedmetadata) metadata = 3;
}
```

### AuditRecord

AuditRecord captures a single synthetic audit line.

```protobuf
message [AuditRecord](#auditrecord) {
  string actor = 1;
  string action = 2;
  google.protobuf.Timestamp occurred_at = 3;
  google.protobuf.Struct details = 4;
}
```

### AuditBatch

AuditBatch groups records for batch RPC demonstrations.

```protobuf
message [AuditBatch](#auditbatch) {
  repeated [AuditRecord](#auditrecord) records = 1;
  string batch_id = 2;
}
```

### Location

Location describes a fictional region for catalog cross-links.

```protobuf
message [Location](#location) {
  string region_code = 1;
  string display_name = 2;
  double latitude = 3;
  double longitude = 4;
}
```

### Address

Address is a postal-style structure used in nested messages.

```protobuf
message [Address](#address) {
  string line1 = 1;
  string line2 = 2;
  string city = 3;
  string postal_code = 4;
  [Location](#location) region = 5;
}
```

### ContactInfo

ContactInfo supports oneof-based documentation rendering.

```protobuf
message [ContactInfo](#contactinfo) {
  oneof channel {
      string email = 1;
      string phone_e164 = 2;
      string slack_user_id = 3;
    }
  string display_name = 4;
}
```

### TenantRef

TenantRef identifies a fictional tenant for multi-tenant stories.

```protobuf
message [TenantRef](#tenantref) {
  string tenant_id = 1 [(buf.validate.field).string.uuid = true];
  string slug = 2 [
      (buf.validate.field).string.min_len = 1,
      (buf.validate.field).string.max_len = 64,
      (buf.validate.field).string.pattern = "^[a-z0-9-]+$"
    ];
  [SharedKind](#sharedkind) tier = 3;
}
```

### QuotaLimits

QuotaLimits documents soft limits referenced from gateway RPCs.

```protobuf
message [QuotaLimits](#quotalimits) {
  uint32 max_requests_per_minute = 1;
  uint32 max_stream_duration_seconds = 2;
  uint64 max_payload_bytes = 3;
}
```

### ErrorDetail

ErrorDetail mirrors a simplified rich error shape for docs.

```protobuf
message [ErrorDetail](#errordetail) {
  string code = 1;
  string message = 2;
  map<string, string> metadata = 3;
  repeated string help_links = 4;
}
```

### RetryPolicy

RetryPolicy is embedded in long-running operation messages.

```protobuf
message [RetryPolicy](#retrypolicy) {
  uint32 max_attempts = 1;
  google.protobuf.Duration initial_backoff = 2;
  google.protobuf.Duration max_backoff = 3;
  double backoff_multiplier = 4;
}
```

### LongRunningOperation

LongRunningOperation is returned by admin-style RPCs in v1.

```protobuf
message [LongRunningOperation](#longrunningoperation) {
  string name = 1;
  [OperationStatus](#operationstatus) status = 2;
  google.protobuf.Timestamp start_time = 3;
  google.protobuf.Timestamp end_time = 4;
  [RetryPolicy](#retrypolicy) retry_policy = 5;
  [ErrorDetail](#errordetail) error = 6;
  double percent_complete = 7;
}
```

### Label

Label is a key/value tag used across catalog and echo fixtures.

```protobuf
message [Label](#label) {
  string key = 1;
  string value = 2;
}
```

### LabelSet

LabelSet aggregates labels for resource descriptions.

```protobuf
message [LabelSet](#labelset) {
  repeated [Label](#label) labels = 1;
}
```

### ResourceIdentity

ResourceIdentity combines tenant, labels, and metadata.

```protobuf
message [ResourceIdentity](#resourceidentity) {
  [TenantRef](#tenantref) tenant = 1;
  [LabelSet](#labelset) labels = 2;
  [SharedMetadata](#sharedmetadata) metadata = 3;
  string resource_name = 4;
}
```

### NumericRange

NumericRange supports validation comment examples.

```protobuf
message [NumericRange](#numericrange) {
  
  int64 min_inclusive = 1;
  int64 max_inclusive = 2;
}
```

**Protovalidate (CEL)**

```cel
id: "numeric_range.min_lte_max"
      message: "min_inclusive must not exceed max_inclusive"
      expression: "this.min_inclusive <= this.max_inclusive"
```

### TimeWindow

TimeWindow defines inclusive bounds for queries.

```protobuf
message [TimeWindow](#timewindow) {
  google.protobuf.Timestamp start = 1;
  google.protobuf.Timestamp end = 2;
}
```

### FilterExpression

FilterExpression is a intentionally verbose filter AST placeholder.

```protobuf
message [FilterExpression](#filterexpression) {
  string field = 1;
  string op = 2;
  string value = 3;
  repeated [FilterExpression](#filterexpression) children = 4;
}
```

### PageToken

PageToken supports pagination narrative in list RPCs.

```protobuf
message [PageToken](#pagetoken) {
  string opaque = 1;
  uint32 page_size = 2;
}
```

### PageResult

PageResult completes a paginated list response.

```protobuf
message [PageResult](#pageresult) {
  [PageToken](#pagetoken) next_page_token = 1;
  uint32 total_size = 2;
}
```

### SortKey

SortKey pairs a field name with an order.

```protobuf
message [SortKey](#sortkey) {
  string field = 1;
  [SortOrder](#sortorder) order = 2;
}
```

### ListOptions

ListOptions bundles pagination and sorting for list RPCs.

```protobuf
message [ListOptions](#listoptions) {
  [PageToken](#pagetoken) page = 1;
  repeated [SortKey](#sortkey) sort = 2;
  repeated [FilterExpression](#filterexpression) filters = 3;
}
```

### ComponentHealth

ComponentHealth describes one sub-system in aggregate health.

```protobuf
message [ComponentHealth](#componenthealth) {
  string component = 1;
  [HealthStatus](#healthstatus) status = 2;
  string detail = 3;
}
```

### AggregateHealth

AggregateHealth rolls up component statuses.

```protobuf
message [AggregateHealth](#aggregatehealth) {
  [HealthStatus](#healthstatus) overall = 1;
  repeated [ComponentHealth](#componenthealth) components = 2;
  google.protobuf.Timestamp evaluated_at = 3;
}
```

### StreamCursor

StreamCursor supports resumable stream examples.

```protobuf
message [StreamCursor](#streamcursor) {
  string stream_id = 1;
  uint64 sequence = 2;
  google.protobuf.Timestamp emitted_at = 3;
}
```

### StreamChunk

StreamChunk is a unit payload in streaming RPC fixtures.

```protobuf
message [StreamChunk](#streamchunk) {
  [StreamCursor](#streamcursor) cursor = 1;
  [PayloadEnvelope](#payloadenvelope) payload = 2;
  bool terminal = 3;
}
```

### BatchKey

BatchKey identifies an item inside batch requests.

```protobuf
message [BatchKey](#batchkey) {
  string id = 1;
  string correlation_id = 2;
}
```

### BatchItem

BatchItem pairs a key with an envelope for batch APIs.

```protobuf
message [BatchItem](#batchitem) {
  [BatchKey](#batchkey) key = 1;
  [PayloadEnvelope](#payloadenvelope) envelope = 2;
}
```

### BatchRequest

BatchRequest aggregates many items for BatchEcho RPCs.

```protobuf
message [BatchRequest](#batchrequest) {
  repeated [BatchItem](#batchitem) items = 1 [(buf.validate.field).repeated.min_items = 1];
  [ResourceIdentity](#resourceidentity) identity = 2;
  bool partial_failure_allowed = 3;
}
```

### BatchItemResult

BatchItemResult reports per-item outcomes.

```protobuf
message [BatchItemResult](#batchitemresult) {
  [BatchKey](#batchkey) key = 1;
  bool ok = 2;
  [ErrorDetail](#errordetail) error = 3;
  [PayloadEnvelope](#payloadenvelope) response_envelope = 4;
}
```

### BatchResponse

BatchResponse collects results for batch RPC documentation.

```protobuf
message [BatchResponse](#batchresponse) {
  repeated [BatchItemResult](#batchitemresult) results = 1;
  [PageResult](#pageresult) page = 2;
}
```

### EchoExtension

EchoExtension carries optional hints from v2 into v1 echo flows.

```protobuf
message [EchoExtension](#echoextension) {
  string locale = 1;
  repeated string keywords = 2;
  map<string, string> annotations = 3;
  [NumericRange](#numericrange) length_bounds = 4;
}
```

### DocumentationAnchor

DocumentationAnchor is a meta-message for cross-link stress tests.

```protobuf
message [DocumentationAnchor](#documentationanchor) {
  string title = 1;
  string href = 2;
  string summary = 3;
}
```

### SeeAlsoBlock

SeeAlsoBlock groups anchors for See Also sections in comments.

```protobuf
message [SeeAlsoBlock](#seealsoblock) {
  repeated [DocumentationAnchor](#documentationanchor) anchors = 1;
}
```

### SharedKind

SharedKind classifies metadata rows in examples.

 | Value | Meaning |
 |-------|---------|
 | UNSPECIFIED | default |
 | ALPHA | first variant |
 | BETA | second variant |
 | GAMMA | third variant |

```protobuf
enum [SharedKind](#sharedkind) {
  SHARED_KIND_UNSPECIFIED = 0;
  SHARED_KIND_ALPHA = 1;
  SHARED_KIND_BETA = 2;
  SHARED_KIND_GAMMA = 3;
}
```

### Priority

Priority influences scheduler hints in narrative docs only.

```protobuf
enum [Priority](#priority) {
  PRIORITY_UNSPECIFIED = 0;
  PRIORITY_LOW = 1;
  PRIORITY_NORMAL = 2;
  PRIORITY_HIGH = 3;
  PRIORITY_CRITICAL = 4;
}
```

### OperationStatus

OperationStatus tracks synthetic long-running work.

```protobuf
enum [OperationStatus](#operationstatus) {
  OPERATION_STATUS_UNSPECIFIED = 0;
  OPERATION_STATUS_PENDING = 1;
  OPERATION_STATUS_RUNNING = 2;
  OPERATION_STATUS_SUCCEEDED = 3;
  OPERATION_STATUS_FAILED = 4;
  OPERATION_STATUS_CANCELLED = 5;
}
```

### SortOrder

SortOrder documents ascending/descending list semantics.

```protobuf
enum [SortOrder](#sortorder) {
  SORT_ORDER_UNSPECIFIED = 0;
  SORT_ORDER_ASC = 1;
  SORT_ORDER_DESC = 2;
}
```

### HealthStatus

HealthStatus is reported by gateway health RPCs.

```protobuf
enum [HealthStatus](#healthstatus) {
  HEALTH_STATUS_UNSPECIFIED = 0;
  HEALTH_STATUS_SERVING = 1;
  HEALTH_STATUS_NOT_SERVING = 2;
  HEALTH_STATUS_DEGRADED = 3;
}
```

