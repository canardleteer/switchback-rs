//! Outcome severity classes for operation and RPC responses.
//!
//! Parsers map family-specific status codes into these classes at populate time
//! via protocol implementations in `switchback-protocols`. Renderers read the
//! stored entity field rather than re-deriving transport semantics.

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
