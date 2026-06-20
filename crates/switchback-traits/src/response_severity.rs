//! Outcome severity classes for operation and RPC responses.
//!
//! Parsers map family-specific status codes into these classes at populate time.
//! HTTP APIs use [`http_status_severity`]; gRPC uses [`grpc_status_severity`].
//! AsyncAPI and other transports can supply their own mapping when populated.

/// Cross-family outcome severity for a response or RPC result.
///
/// Renderers may use this for grouping, color, or ordering without re-deriving
/// transport-specific semantics from raw status strings.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ResponseSeverity {
    /// Severity was not determined or does not apply.
    #[default]
    Unspecified,
    /// Informational outcome (HTTP 1xx; gRPC equivalents are rare).
    Informational,
    /// Successful outcome (HTTP 2xx; gRPC `OK`).
    Success,
    /// Redirection or alternate continuation (HTTP 3xx).
    Redirection,
    /// Client-side fault (HTTP 4xx; most gRPC non-OK codes except internal/unavailable).
    ClientError,
    /// Server-side fault (HTTP 5xx; gRPC `INTERNAL`, `UNAVAILABLE`, etc.).
    ServerError,
}

impl ResponseSeverity {
    /// Stable wire and log slug (`unspecified`, `informational`, …).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unspecified => "unspecified",
            Self::Informational => "informational",
            Self::Success => "success",
            Self::Redirection => "redirection",
            Self::ClientError => "client_error",
            Self::ServerError => "server_error",
        }
    }

    /// Parse a stored slug; unknown values map to [`Unspecified`].
    pub fn parse_str(s: &str) -> Self {
        match s {
            "informational" => Self::Informational,
            "success" => Self::Success,
            "redirection" => Self::Redirection,
            "client_error" => Self::ClientError,
            "server_error" => Self::ServerError,
            _ => Self::Unspecified,
        }
    }
}

/// Classify an OpenAPI / HTTP status key (`200`, `4XX`, `default`, …).
pub fn http_status_severity(status: &str) -> ResponseSeverity {
    let trimmed = status.trim();
    if trimmed.eq_ignore_ascii_case("default") {
        return ResponseSeverity::Unspecified;
    }

    if let Ok(code) = trimmed.parse::<u16>() {
        return http_status_code_severity(code);
    }

    let upper = trimmed.to_ascii_uppercase();
    match upper.as_str() {
        "1XX" => ResponseSeverity::Informational,
        "2XX" => ResponseSeverity::Success,
        "3XX" => ResponseSeverity::Redirection,
        "4XX" => ResponseSeverity::ClientError,
        "5XX" => ResponseSeverity::ServerError,
        _ => ResponseSeverity::Unspecified,
    }
}

/// Classify a numeric HTTP status code.
pub fn http_status_code_severity(code: u16) -> ResponseSeverity {
    match code {
        100..=199 => ResponseSeverity::Informational,
        200..=299 => ResponseSeverity::Success,
        300..=399 => ResponseSeverity::Redirection,
        400..=499 => ResponseSeverity::ClientError,
        500..=599 => ResponseSeverity::ServerError,
        _ => ResponseSeverity::Unspecified,
    }
}

/// Classify a gRPC status code (numeric [`google.rpc.Code`](https://github.com/googleapis/googleapis/blob/master/google/rpc/code.proto)).
pub fn grpc_status_severity(code: i32) -> ResponseSeverity {
    match code {
        0 => ResponseSeverity::Success, // OK
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 15 => ResponseSeverity::ClientError,
        12..=14 => ResponseSeverity::ServerError,
        _ => ResponseSeverity::Unspecified,
    }
}

/// Classify a gRPC status name (`OK`, `INVALID_ARGUMENT`, …).
pub fn grpc_status_name_severity(name: &str) -> ResponseSeverity {
    match name.trim().to_ascii_uppercase().as_str() {
        "OK" => ResponseSeverity::Success,
        "CANCELLED"
        | "UNKNOWN"
        | "INVALID_ARGUMENT"
        | "DEADLINE_EXCEEDED"
        | "NOT_FOUND"
        | "ALREADY_EXISTS"
        | "PERMISSION_DENIED"
        | "RESOURCE_EXHAUSTED"
        | "FAILED_PRECONDITION"
        | "ABORTED"
        | "OUT_OF_RANGE"
        | "UNAUTHENTICATED"
        | "UNIMPLEMENTED" => ResponseSeverity::ClientError,
        "INTERNAL" | "UNAVAILABLE" | "DATA_LOSS" => ResponseSeverity::ServerError,
        _ => ResponseSeverity::Unspecified,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn http_status_classes() {
        assert_eq!(http_status_severity("200"), ResponseSeverity::Success);
        assert_eq!(http_status_severity("202"), ResponseSeverity::Success);
        assert_eq!(http_status_severity("302"), ResponseSeverity::Redirection);
        assert_eq!(http_status_severity("402"), ResponseSeverity::ClientError);
        assert_eq!(http_status_severity("403"), ResponseSeverity::ClientError);
        assert_eq!(http_status_severity("500"), ResponseSeverity::ServerError);
        assert_eq!(http_status_severity("2XX"), ResponseSeverity::Success);
        assert_eq!(
            http_status_severity("default"),
            ResponseSeverity::Unspecified
        );
    }

    #[test]
    fn grpc_status_classes() {
        assert_eq!(grpc_status_severity(0), ResponseSeverity::Success);
        assert_eq!(
            grpc_status_name_severity("NOT_FOUND"),
            ResponseSeverity::ClientError
        );
        assert_eq!(
            grpc_status_name_severity("INTERNAL"),
            ResponseSeverity::ServerError
        );
    }
}
