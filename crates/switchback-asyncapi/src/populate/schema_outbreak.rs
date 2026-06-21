//! Promote inline JSON Schema and nested Avro types to schema entities.

use std::collections::BTreeSet;

use serde_json::Value;
use switchback_avro::collect_named_avro_schemas;
use switchback_jsonschema::schema::Schema;
use switchback_avro::populate_avro_schema_body;
use switchback_traits::{EntityBody, EntityCategory, RefKind, Reference};

use crate::category::AsyncApiCategory;
use crate::populate::components::push_schema_entity;
use crate::populate::schema_dispatch::{
    is_avro_schema_format, payload_value, schema_format,
};
use crate::populate::{PopulateCtx, PopulatedEntity};

pub fn outbreak_entities(entities: &mut Vec<PopulatedEntity>, ctx: &PopulateCtx<'_>) {
    outbreak_inline_payloads(entities, ctx);
    attach_message_payload_refs(entities, ctx);
    refresh_avro_schema_properties(entities, ctx);
}

fn schema_names(entities: &[PopulatedEntity]) -> BTreeSet<String> {
    entities
        .iter()
        .filter(|pe| pe.entity.category == AsyncApiCategory::Schema)
        .map(|pe| pe.entity.id.name.clone())
        .collect()
}

fn outbreak_inline_payloads(entities: &mut Vec<PopulatedEntity>, ctx: &PopulateCtx<'_>) {
    let mut names = schema_names(entities);
    let mut new_schemas = Vec::new();

    for pe in entities.iter() {
        if pe.entity.category != AsyncApiCategory::Message {
            continue;
        }
        let msg_val = parse_message_fence(pe, ctx.doc.fence_language());
        outbreak_message_payload(
            &msg_val,
            &pe.entity.id.name,
            ctx,
            &mut names,
            &mut new_schemas,
        );
    }

    for pe in entities.iter() {
        if pe.entity.category != AsyncApiCategory::Schema {
            continue;
        }
        if let EntityBody::Schema(body) = &pe.entity.body
            && body.payload_format.contains("avro")
            && let Ok(value) = serde_json::from_str::<Value>(&body.fence_body)
        {
            outbreak_avro_named_types(&value, ctx, &mut names, &mut new_schemas, &[]);
        }
    }

    entities.extend(new_schemas);
}

fn outbreak_message_payload(
    msg_val: &Value,
    msg_name: &str,
    ctx: &PopulateCtx<'_>,
    names: &mut BTreeSet<String>,
    out: &mut Vec<PopulatedEntity>,
) {
    let payload = payload_value(msg_val);
    if payload.as_object().is_some_and(|o| o.contains_key("$ref")) {
        return;
    }

    if let Some(format) = schema_format(msg_val).or_else(|| schema_format(payload))
        && is_avro_schema_format(format)
    {
        let schema_val = payload.get("schema").unwrap_or(payload);
        outbreak_avro_named_types(schema_val, ctx, names, out, &[msg_name]);
        return;
    }

    let schema_val = inline_json_schema_value(payload);
    if schema_val.is_none() {
        return;
    }
    let schema_val = schema_val.expect("inline json schema checked above");
    if schema_val.as_object().is_some_and(|o| o.contains_key("$ref")) {
        return;
    }

    let entity_name = inline_json_schema_name(msg_name, schema_val);
    if names.contains(&entity_name) {
        return;
    }
    names.insert(entity_name.clone());
    push_schema_entity(schema_val, &entity_name, ctx, out);
}

fn outbreak_avro_named_types(
    root: &Value,
    ctx: &PopulateCtx<'_>,
    names: &mut BTreeSet<String>,
    out: &mut Vec<PopulatedEntity>,
    skip_names: &[&str],
) {
    for (name, schema_val) in collect_named_avro_schemas(root) {
        if skip_names.iter().any(|skip| *skip == name) || names.contains(&name) {
            continue;
        }
        names.insert(name.clone());
        push_avro_schema_entity(&schema_val, &name, ctx, out);
    }
}

fn push_avro_schema_entity(
    value: &Value,
    name: &str,
    ctx: &PopulateCtx<'_>,
    out: &mut Vec<PopulatedEntity>,
) {
    let group = entity_group_id(ctx);
    let body = populate_avro_schema_body(
        value,
        ctx.module_id,
        &group,
        Some("application/vnd.apache.avro+json"),
    );
    let refs = crate::populate::refs::structural_refs(
        value,
        ctx.doc_uri,
        ctx.module_id,
        ctx.uri_to_group,
        ctx.index,
    );
    out.push(PopulatedEntity {
        entity: switchback_traits::Entity {
            id: switchback_traits::EntityId::new(
                group.as_str(),
                AsyncApiCategory::Schema.as_str(),
                name,
            ),
            category: AsyncApiCategory::Schema,
            title: name.to_string(),
            doc: None,
            source_span: None,
            body: EntityBody::Schema(body),
        },
        refs,
    });
}

fn inline_json_schema_value(payload: &Value) -> Option<&Value> {
    if payload.get("schema").is_some() {
        let schema = payload.get("schema").unwrap();
        if Schema::is_schema_value(schema) {
            return Some(schema);
        }
        return None;
    }
    if Schema::is_schema_value(payload) {
        return Some(payload);
    }
    None
}

fn inline_json_schema_name(msg_name: &str, schema_val: &Value) -> String {
    if let Some(title) = schema_val
        .get("title")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
    {
        return title.to_string();
    }
    format!("{msg_name}Payload")
}

fn attach_message_payload_refs(entities: &mut [PopulatedEntity], ctx: &PopulateCtx<'_>) {
    let group = entity_group_id(ctx);
    let schema_names = schema_names(entities);

    for pe in entities.iter_mut() {
        if pe.entity.category != AsyncApiCategory::Message {
            continue;
        }
        let msg_val = parse_message_fence(pe, ctx.doc.fence_language());
        let payload = payload_value(&msg_val);

        if payload.as_object().is_some_and(|o| o.contains_key("$ref")) {
            continue;
        }

        if let Some(format) = schema_format(&msg_val).or_else(|| schema_format(payload))
            && is_avro_schema_format(format)
        {
            let schema_val = payload.get("schema").unwrap_or(payload);
            for (name, _) in collect_named_avro_schemas(schema_val) {
                if name == pe.entity.id.name {
                    continue;
                }
                push_schema_ref_if_present(pe, &name, &group, &schema_names, ctx);
            }
            continue;
        }

        let payload_ref_name = inline_json_schema_value(payload).map(|schema_val| {
            inline_json_schema_name(&pe.entity.id.name, schema_val)
        });

        if let Some(name) = payload_ref_name {
            push_schema_ref_if_present(pe, &name, &group, &schema_names, ctx);
        }
    }
}

fn push_schema_ref_if_present(
    pe: &mut PopulatedEntity,
    name: &str,
    group: &str,
    schema_names: &BTreeSet<String>,
    ctx: &PopulateCtx<'_>,
) {
    if !schema_names.contains(name) {
        return;
    }
    if pe
        .refs
        .iter()
        .any(|r| r.target.category == "schema" && r.target.name == name)
    {
        return;
    }
    pe.refs.push(Reference {
        target: switchback_traits::EntityRef {
            module: ctx.module_id.to_string(),
            group: group.to_string(),
            category: "schema".to_string(),
            name: name.to_string(),
        },
        kind: RefKind::Internal,
    });
}

fn refresh_avro_schema_properties(entities: &mut [PopulatedEntity], ctx: &PopulateCtx<'_>) {
    let group = entity_group_id(ctx);
    for pe in entities.iter_mut() {
        if pe.entity.category != AsyncApiCategory::Schema {
            continue;
        }
        let EntityBody::Schema(body) = &mut pe.entity.body else {
            continue;
        };
        if !body.payload_format.contains("avro") {
            continue;
        }
        let Ok(value) = serde_json::from_str::<Value>(&body.fence_body) else {
            continue;
        };
        let refreshed = populate_avro_schema_body(
            &value,
            ctx.module_id,
            &group,
            Some(body.payload_format.as_str()),
        );
        body.properties = refreshed.properties;
    }
}

fn entity_group_id(ctx: &PopulateCtx<'_>) -> String {
    ctx.entry_group
        .unwrap_or(crate::paths::COMPONENTS_GROUP)
        .to_string()
}

fn parse_message_fence(pe: &PopulatedEntity, fence_language: &str) -> Value {
    let EntityBody::Message(body) = &pe.entity.body else {
        return Value::Null;
    };
    if fence_language == "yaml" {
        serde_saphyr::from_str(&body.fence_body).unwrap_or(Value::Null)
    } else {
        serde_json::from_str(&body.fence_body).unwrap_or(Value::Null)
    }
}
