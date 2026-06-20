//! Outcome severity mappers moved from `switchback-traits`.

use switchback_traits::ResponseSeverity;

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

/// Classify a gRPC status code (numeric `google.rpc.Code`).
pub fn grpc_status_severity(code: i32) -> ResponseSeverity {
    match code {
        0 => ResponseSeverity::Success,
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
        assert_eq!(http_status_severity("500"), ResponseSeverity::ServerError);
    }

    #[test]
    fn grpc_status_classes() {
        assert_eq!(grpc_status_severity(0), ResponseSeverity::Success);
        assert_eq!(
            grpc_status_name_severity("INTERNAL"),
            ResponseSeverity::ServerError
        );
    }
}
