//! AsyncAPI-specific mdBook rendering helpers.

use switchback_protocols::{
    AmqpPayloadKind, DecodedAttachment, KafkaPayloadKind, MqttPayloadKind, ProtocolRegistry,
};
use switchback_traits::{
    ChannelBody, EntityBody, LinkContext, LinkFormatter, MessageBody, OperationBody, Options,
    Property, Reference, SchemaBody, StoredEntity, apply_intra_links, entity_category_dir,
    entity_rel_path,
};

use crate::render::fence::{
    entity_module_group, link_ref, link_structural_refs_in_prose, proto_file_name, push_fence_body,
    render_schema_fence,
};
use crate::render::markdown_doc::format_markdown_doc;
use crate::render::{md_heading, push_paragraph_break};

const SECTION_LEVEL: usize = 2;
const ENTITY_LEVEL: usize = 3;
const SUBSECTION_LEVEL: usize = 4;

pub fn is_asyncapi_family(family: &str) -> bool {
    family == "asyncapi"
}

pub fn category_section_title(category: &str) -> &'static str {
    match category {
        "channel" => "Channels",
        "operation" => "Operations",
        "message" => "Messages",
        "schema" => "Schemas",
        "parameter" => "Parameters",
        "security-scheme" => "Security schemes",
        _ => "Entities",
    }
}

pub fn asyncapi_summary_link_text(entity: &StoredEntity) -> String {
    match entity.category.as_str() {
        "operation" | "channel" => entity.title.clone(),
        category => format!("{} {}", summary_prefix(category), entity.name),
    }
}

pub fn asyncapi_summary_sort_key(entity: &StoredEntity) -> String {
    match entity.category.as_str() {
        "operation" => entity.title.clone(),
        "channel" => entity.name.clone(),
        _ => entity.name.clone(),
    }
}

pub fn summary_prefix(category: &str) -> &'static str {
    match category {
        "channel" => "Channel",
        "operation" => "Operation",
        "message" => "Message",
        "schema" => "Schema",
        "parameter" => "Parameter",
        "security-scheme" => "Security scheme",
        _ => "Entity",
    }
}

pub fn renderable_asyncapi_entities(entities: &[StoredEntity]) -> Vec<&StoredEntity> {
    entities
        .iter()
        .filter(|e| {
            matches!(
                e.body,
                EntityBody::Channel(_)
                    | EntityBody::Operation(_)
                    | EntityBody::Message(_)
                    | EntityBody::Schema(_)
            )
        })
        .collect()
}

pub fn asyncapi_index_categories() -> &'static [&'static str] {
    &["channel", "operation", "message", "schema"]
}

pub(crate) fn asyncapi_category_rank(category: &str) -> usize {
    asyncapi_index_categories()
        .iter()
        .position(|c| *c == category)
        .unwrap_or(asyncapi_index_categories().len())
}

pub fn render_asyncapi_index_sections(
    out: &mut String,
    entities: &[StoredEntity],
    group_dir: &str,
    markdown_root: &str,
    index_from: &std::path::Path,
    links: &LinkContext,
) {
    for category in asyncapi_index_categories() {
        let mut section: Vec<_> = renderable_asyncapi_entities(entities)
            .into_iter()
            .filter(|e| e.category == *category)
            .collect();
        if section.is_empty() {
            continue;
        }
        section.sort_by_key(|a| asyncapi_summary_sort_key(a));
        out.push_str(&md_heading(SECTION_LEVEL, category_section_title(category)));
        for entity in section {
            let rel = asyncapi_entity_rel_path(markdown_root, group_dir, entity);
            let p = std::path::PathBuf::from(rel);
            let title = asyncapi_summary_link_text(entity);
            out.push_str("- ");
            out.push_str(&links.summary_link(index_from, &p, &title));
            out.push('\n');
        }
        out.push('\n');
    }
}

pub fn render_asyncapi_package_sections(
    out: &mut String,
    entities: &[StoredEntity],
    group: &str,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    for category in asyncapi_index_categories() {
        let mut section: Vec<_> = entities
            .iter()
            .filter(|e| e.category.as_str() == *category)
            .collect();
        if section.is_empty() {
            continue;
        }
        section.sort_by(|a, b| {
            if *category == "operation" {
                asyncapi_summary_sort_key(a).cmp(&asyncapi_summary_sort_key(b))
            } else {
                a.title.cmp(&b.title)
            }
        });
        out.push_str(&md_heading(SECTION_LEVEL, category_section_title(category)));
        for entity in section {
            render_asyncapi_entity_section(out, entity, group, ctx, opts, formatter);
        }
    }
}

fn entity_link_group(entity: &StoredEntity) -> &str {
    entity
        .refs
        .iter()
        .find(|r| !r.target.group.is_empty())
        .map(|r| r.target.group.as_str())
        .unwrap_or("")
}

fn render_asyncapi_entity_section(
    out: &mut String,
    entity: &StoredEntity,
    group: &str,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    out.push_str(&md_heading(ENTITY_LEVEL, &asyncapi_entity_heading(entity)));
    out.push_str(&render_asyncapi_entity_body(
        entity, group, ctx, opts, formatter,
    ));
}

pub fn render_asyncapi_entity_page(
    entity: &StoredEntity,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut page = md_heading(1, &asyncapi_entity_heading(entity));
    page.push_str(&render_asyncapi_entity_body(
        entity,
        entity_link_group(entity),
        ctx,
        opts,
        formatter,
    ));
    page
}

fn asyncapi_entity_heading(entity: &StoredEntity) -> String {
    match entity.category.as_str() {
        "operation" | "channel" => entity.title.clone(),
        _ => entity.title.clone(),
    }
}

fn render_asyncapi_entity_body(
    entity: &StoredEntity,
    group: &str,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    match &entity.body {
        EntityBody::Channel(body) => {
            render_asyncapi_channel_markdown(entity, body, group, ctx, opts, formatter)
        }
        EntityBody::Operation(body) => {
            render_asyncapi_operation_markdown(entity, body, group, ctx, opts, formatter)
        }
        EntityBody::Message(body) => {
            render_asyncapi_message_fence(entity, body, ctx, opts, formatter)
        }
        EntityBody::Schema(body) => {
            render_asyncapi_schema_fence(entity, body, ctx, opts, formatter)
        }
        _ => String::new(),
    }
}

pub fn asyncapi_entity_rel_path(
    markdown_root: &str,
    group_dir: &str,
    entity: &StoredEntity,
) -> String {
    format!(
        "{markdown_root}/{}",
        entity_rel_path(
            group_dir,
            entity_category_dir(&entity.category),
            &entity.name,
        )
    )
}

fn render_asyncapi_channel_markdown(
    entity: &StoredEntity,
    body: &ChannelBody,
    group: &str,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut out = format_channel_signature_line(&body.signature);
    push_entity_doc(&mut out, entity, group, ctx, opts, formatter);
    out.push_str(&render_message_ref_section(entity, ctx));
    push_fence_body(&mut out, &body.fence_language, &body.fence_body);
    out
}

fn render_asyncapi_operation_markdown(
    entity: &StoredEntity,
    body: &OperationBody,
    group: &str,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut out = format_asyncapi_operation_line(&body.signature, &body.protocols);
    push_entity_doc(&mut out, entity, group, ctx, opts, formatter);
    out.push_str(&render_message_ref_section(entity, ctx));
    push_fence_body(&mut out, &body.fence_language, &body.fence_body);
    out
}

fn render_message_ref_section(entity: &StoredEntity, ctx: &LinkContext) -> String {
    let refs: Vec<&Reference> = entity
        .refs
        .iter()
        .filter(|r| r.target.category == "message")
        .collect();
    render_ref_list_section("Messages", &refs, ctx)
}

fn render_asyncapi_message_fence(
    entity: &StoredEntity,
    body: &MessageBody,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut out = String::new();
    push_entity_doc(
        &mut out,
        entity,
        entity_link_group(entity),
        ctx,
        opts,
        formatter,
    );
    let payload_refs: Vec<&Reference> = entity
        .refs
        .iter()
        .filter(|r| {
            r.target.category == "schema" && r.target.name != entity.name
        })
        .collect();
    out.push_str(&render_ref_list_section("Payload", &payload_refs, ctx));
    out.push_str(&render_message_payload_properties(entity, body, ctx));
    push_fence_body(&mut out, &body.fence_language, &body.fence_body);
    out
}

fn render_message_payload_properties(
    entity: &StoredEntity,
    body: &MessageBody,
    ctx: &LinkContext,
) -> String {
    let msg_val = parse_message_fence_value(body, &body.fence_language);
    let payload = message_payload_value(&msg_val);
    let format = message_schema_format(&msg_val, payload);
    if let Some(format) = format.as_deref().filter(|f| is_avro_schema_format(f)) {
        let schema_val = payload.get("schema").unwrap_or(payload);
        let (module, group) = entity_module_group(entity);
        if module.is_empty() || group.is_empty() {
            return String::new();
        }
        let schema_body = switchback_avro::populate_avro_schema_body(
            schema_val,
            module,
            group,
            Some(format),
        );
        return render_schema_properties_table(&schema_body.properties, ctx);
    }
    String::new()
}

fn parse_message_fence_value(body: &MessageBody, fence_language: &str) -> serde_json::Value {
    if fence_language == "yaml" {
        serde_saphyr::from_str(&body.fence_body).unwrap_or(serde_json::Value::Null)
    } else {
        serde_json::from_str(&body.fence_body).unwrap_or(serde_json::Value::Null)
    }
}

fn message_payload_value(msg_val: &serde_json::Value) -> &serde_json::Value {
    if msg_val.get("payload").is_some() {
        msg_val.get("payload").unwrap_or(msg_val)
    } else {
        msg_val
    }
}

fn message_schema_format(
    msg_val: &serde_json::Value,
    payload: &serde_json::Value,
) -> Option<String> {
    msg_val
        .get("schemaFormat")
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .or_else(|| {
            payload
                .get("schemaFormat")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
}

fn is_avro_schema_format(format: &str) -> bool {
    format == "application/vnd.apache.avro+json"
        || format == "application/vnd.apache.avro+yaml"
        || format.contains("avro")
}

fn render_ref_list_section(title: &str, refs: &[&Reference], ctx: &LinkContext) -> String {
    if refs.is_empty() {
        return String::new();
    }
    let from = ctx
        .render_from
        .as_deref()
        .unwrap_or_else(|| std::path::Path::new(&ctx.markdown_root));
    let mut out = md_heading(SUBSECTION_LEVEL, title);
    for reference in refs {
        out.push_str("- ");
        out.push_str(&link_ref(reference, ctx, from));
        out.push('\n');
    }
    out.push('\n');
    out
}

fn render_schema_properties_table(
    properties: &[Property],
    ctx: &LinkContext,
) -> String {
    if properties.is_empty() {
        return String::new();
    }
    let from = ctx
        .render_from
        .as_deref()
        .unwrap_or_else(|| std::path::Path::new(&ctx.markdown_root));
    let mut out = md_heading(SUBSECTION_LEVEL, "Properties");
    out.push_str("| Field | Type |\n");
    out.push_str("| --- | --- |\n");
    for property in properties {
        out.push_str("| `");
        out.push_str(&property.name);
        out.push_str("` | ");
        out.push_str(&link_ref(&property.schema_ref, ctx, from));
        out.push_str(" |\n");
    }
    out.push('\n');
    out
}

fn render_asyncapi_schema_fence(
    entity: &StoredEntity,
    body: &SchemaBody,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) -> String {
    let mut out = render_schema_properties_table(&body.properties, ctx);
    out.push_str(&render_schema_fence(
        &body.fence_language,
        &proto_file_name(entity),
        entity.doc.as_deref(),
        &body.fence_body,
        opts.escape_tags,
        &entity.intra_links,
        formatter,
        ctx,
    ));
    out
}

fn format_channel_signature_line(signature: &str) -> String {
    let Some((kind, address)) = signature.split_once(' ') else {
        return format!("**{signature}**\n\n");
    };
    format!("**{kind}** `{address}`\n\n")
}

pub fn format_asyncapi_operation_line(
    signature: &str,
    protocols: &[switchback_traits::ProtocolAttachment],
) -> String {
    let Some((action, channel)) = signature.split_once(' ') else {
        return format!("**{signature}**\n\n");
    };
    let badges = protocol_badges(protocols);
    let mut line = format!("**{action}** `{channel}`");
    if !badges.is_empty() {
        line.push_str(" — ");
        line.push_str(&badges.join(" · "));
    }
    format!("{line}\n\n")
}

fn protocol_badges(protocols: &[switchback_traits::ProtocolAttachment]) -> Vec<String> {
    let registry = ProtocolRegistry::with_builtins();
    let mut badges = Vec::new();
    for attachment in protocols {
        match registry.decode_attachment(attachment) {
            Ok(DecodedAttachment::Kafka(KafkaPayloadKind::Channel(meta)))
                if !meta.topic.is_empty() =>
            {
                badges.push(format!("`kafka` topic `{}`", meta.topic));
            }
            Ok(DecodedAttachment::Mqtt(MqttPayloadKind::Channel(meta)))
                if !meta.topic.is_empty() =>
            {
                badges.push(format!("`mqtt` topic `{}`", meta.topic));
            }
            Ok(DecodedAttachment::Amqp(AmqpPayloadKind::Channel(meta))) => {
                if !meta.exchange_name.is_empty() {
                    badges.push(format!("`amqp` exchange `{}`", meta.exchange_name));
                } else if !meta.queue_name.is_empty() {
                    badges.push(format!("`amqp` queue `{}`", meta.queue_name));
                } else if !meta.channel_kind.is_empty() {
                    badges.push(format!("`amqp` {}", meta.channel_kind));
                }
            }
            Ok(DecodedAttachment::Mqtt(MqttPayloadKind::Operation(meta))) if meta.qos > 0 => {
                badges.push(format!("QoS {}", meta.qos));
            }
            Ok(DecodedAttachment::Kafka(KafkaPayloadKind::Operation(_))) => {}
            Ok(DecodedAttachment::Opaque { protocol_id, .. }) if !protocol_id.is_empty() => {
                badges.push(format!("`{protocol_id}`"));
            }
            _ => {}
        }
    }
    badges.sort();
    badges.dedup();
    badges
}

fn push_entity_doc(
    out: &mut String,
    entity: &StoredEntity,
    group: &str,
    ctx: &LinkContext,
    opts: &Options,
    formatter: &dyn LinkFormatter,
) {
    if let Some(doc) = entity.doc.as_deref() {
        let from = ctx
            .render_from
            .as_deref()
            .unwrap_or_else(|| std::path::Path::new(&ctx.markdown_root));
        let module = entity
            .refs
            .iter()
            .find(|r| !r.target.module.is_empty())
            .map(|r| r.target.module.as_str())
            .unwrap_or("");
        let doc = apply_intra_links("doc", doc, &entity.intra_links, formatter, ctx);
        let doc = link_structural_refs_in_prose(&doc, &entity.refs, module, group, ctx, from);
        out.push_str(&format_markdown_doc(&doc, opts.escape_tags));
        push_paragraph_break(out);
    }
}
