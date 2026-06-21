//! Basic Mermaid sequence diagrams for AsyncAPI operations.

/// Build a Mermaid sequence diagram block for one operation on a channel.
pub fn operation_sequence_diagram(
    channel: &str,
    action: &str,
    operation_id: &str,
) -> String {
    format!(
        "```mermaid\nsequenceDiagram\n  participant Client\n  participant Broker as {channel}\n  Client->>Broker: {action} ({operation_id})\n```"
    )
}

/// Merge optional Mermaid prose into entity documentation.
pub fn merge_doc_with_mermaid(doc: Option<String>, mermaid: &str) -> Option<String> {
    if mermaid.trim().is_empty() {
        return doc.filter(|s| !s.is_empty());
    }
    match doc.filter(|s| !s.is_empty()) {
        Some(existing) => Some(format!("{existing}\n\n{mermaid}")),
        None => Some(mermaid.to_string()),
    }
}
