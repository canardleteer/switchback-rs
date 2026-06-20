//! Entity bodies and stored entity mapping.

use switchback_traits::{
    ChannelBody, EntityBody, ExtensionBody, MessageBody, OperationBody, ParameterBody,
    RequestBodyBody, ResponseBody, SchemaBody, SecuritySchemeBody, ServiceBody, StoredEntity,
};

use crate::convert::{manual, missing_entity_body, opt_string, string_opt};
use crate::pb;
use crate::pb::__buffa::oneof::entity::Body as PbEntityBody;

use super::link;

pub fn entity_to_proto(entity: &StoredEntity) -> switchback_traits::Result<pb::Entity> {
    Ok(pb::Entity {
        name: entity.name.clone(),
        category: entity.category.clone(),
        title: entity.title.clone(),
        doc: opt_string(&entity.doc),
        source: entity
            .source
            .as_ref()
            .map(manual::source_to_proto)
            .map(buffa::MessageField::some)
            .unwrap_or_default(),
        refs: entity
            .refs
            .iter()
            .map(link::reference_to_proto)
            .collect::<switchback_traits::Result<_>>()?,
        intra_links: entity
            .intra_links
            .iter()
            .map(link::intra_link_to_proto)
            .collect::<switchback_traits::Result<_>>()?,
        body: Some(entity_body_to_proto(&entity.body)?),
        ..Default::default()
    })
}

pub fn entity_from_proto(entity: pb::Entity) -> switchback_traits::Result<StoredEntity> {
    Ok(StoredEntity {
        name: entity.name,
        category: entity.category,
        title: entity.title,
        doc: string_opt(entity.doc),
        source: if entity.source.is_set() {
            Some(manual::source_from_proto(&entity.source))
        } else {
            None
        },
        refs: entity
            .refs
            .into_iter()
            .map(link::reference_from_proto)
            .collect::<switchback_traits::Result<_>>()?,
        intra_links: entity
            .intra_links
            .into_iter()
            .map(link::intra_link_from_proto)
            .collect::<switchback_traits::Result<_>>()?,
        body: entity
            .body
            .map(entity_body_from_proto)
            .transpose()?
            .ok_or_else(missing_entity_body)?,
    })
}

fn entity_body_to_proto(body: &EntityBody) -> switchback_traits::Result<PbEntityBody> {
    Ok(match body {
        EntityBody::Operation(value) => {
            PbEntityBody::Operation(Box::new(operation_body_to_proto(value)?))
        }
        EntityBody::Schema(value) => PbEntityBody::Schema(Box::new(schema_body_to_proto(value)?)),
        EntityBody::Channel(value) => PbEntityBody::Channel(Box::new(channel_body_to_proto(value))),
        EntityBody::Message(value) => PbEntityBody::Message(Box::new(message_body_to_proto(value))),
        EntityBody::Parameter(value) => {
            PbEntityBody::Parameter(Box::new(parameter_body_to_proto(value)))
        }
        EntityBody::Response(value) => {
            PbEntityBody::Response(Box::new(response_body_to_proto(value)))
        }
        EntityBody::RequestBody(value) => {
            PbEntityBody::RequestBody(Box::new(request_body_body_to_proto(value)))
        }
        EntityBody::SecurityScheme(value) => {
            PbEntityBody::SecurityScheme(Box::new(security_scheme_body_to_proto(value)))
        }
        EntityBody::Service(value) => PbEntityBody::Service(Box::new(service_body_to_proto(value))),
        EntityBody::Extension(value) => {
            PbEntityBody::Extension(Box::new(extension_body_to_proto(value)))
        }
    })
}

fn entity_body_from_proto(body: PbEntityBody) -> switchback_traits::Result<EntityBody> {
    Ok(match body {
        PbEntityBody::Operation(value) => EntityBody::Operation(operation_body_from_proto(*value)?),
        PbEntityBody::Schema(value) => EntityBody::Schema(schema_body_from_proto(*value)?),
        PbEntityBody::Channel(value) => EntityBody::Channel(channel_body_from_proto(*value)),
        PbEntityBody::Message(value) => EntityBody::Message(message_body_from_proto(*value)),
        PbEntityBody::Parameter(value) => EntityBody::Parameter(parameter_body_from_proto(*value)),
        PbEntityBody::Response(value) => EntityBody::Response(response_body_from_proto(*value)),
        PbEntityBody::RequestBody(value) => {
            EntityBody::RequestBody(request_body_body_from_proto(*value))
        }
        PbEntityBody::SecurityScheme(value) => {
            EntityBody::SecurityScheme(security_scheme_body_from_proto(*value))
        }
        PbEntityBody::Service(value) => EntityBody::Service(service_body_from_proto(*value)),
        PbEntityBody::Extension(value) => EntityBody::Extension(extension_body_from_proto(*value)),
    })
}

fn operation_body_to_proto(body: &OperationBody) -> switchback_traits::Result<pb::OperationBody> {
    Ok(pb::OperationBody {
        signature: body.signature.clone(),
        fence_language: body.fence_language.clone(),
        fence_body: body.fence_body.clone(),
        parameters: body
            .parameters
            .iter()
            .map(link::parameter_ref_to_proto)
            .collect::<switchback_traits::Result<_>>()?,
        responses: body
            .responses
            .iter()
            .map(link::response_ref_to_proto)
            .collect::<switchback_traits::Result<_>>()?,
        request_body: body
            .request_body
            .as_ref()
            .map(link::operation_request_body_ref_to_proto)
            .transpose()?
            .map(buffa::MessageField::some)
            .unwrap_or_default(),
        ..Default::default()
    })
}

fn operation_body_from_proto(body: pb::OperationBody) -> switchback_traits::Result<OperationBody> {
    Ok(OperationBody {
        signature: body.signature,
        fence_language: body.fence_language,
        fence_body: body.fence_body,
        parameters: body
            .parameters
            .into_iter()
            .map(link::parameter_ref_from_proto)
            .collect::<switchback_traits::Result<_>>()?,
        responses: body
            .responses
            .into_iter()
            .map(link::response_ref_from_proto)
            .collect::<switchback_traits::Result<_>>()?,
        request_body: body
            .request_body
            .into_option()
            .map(link::operation_request_body_ref_from_proto)
            .transpose()?,
    })
}

fn schema_body_to_proto(body: &SchemaBody) -> switchback_traits::Result<pb::SchemaBody> {
    Ok(pb::SchemaBody {
        fence_language: body.fence_language.clone(),
        fence_body: body.fence_body.clone(),
        payload_format: body.payload_format.clone(),
        properties: body
            .properties
            .iter()
            .map(link::property_to_proto)
            .collect::<switchback_traits::Result<_>>()?,
        ..Default::default()
    })
}

fn schema_body_from_proto(body: pb::SchemaBody) -> switchback_traits::Result<SchemaBody> {
    Ok(SchemaBody {
        fence_language: body.fence_language,
        fence_body: body.fence_body,
        payload_format: body.payload_format,
        properties: body
            .properties
            .into_iter()
            .map(link::property_from_proto)
            .collect::<switchback_traits::Result<_>>()?,
    })
}

fn channel_body_to_proto(body: &ChannelBody) -> pb::ChannelBody {
    pb::ChannelBody {
        signature: body.signature.clone(),
        fence_language: body.fence_language.clone(),
        fence_body: body.fence_body.clone(),
        ..Default::default()
    }
}

fn channel_body_from_proto(body: pb::ChannelBody) -> ChannelBody {
    ChannelBody {
        signature: body.signature,
        fence_language: body.fence_language,
        fence_body: body.fence_body,
    }
}

fn message_body_to_proto(body: &MessageBody) -> pb::MessageBody {
    pb::MessageBody {
        fence_language: body.fence_language.clone(),
        fence_body: body.fence_body.clone(),
        ..Default::default()
    }
}

fn message_body_from_proto(body: pb::MessageBody) -> MessageBody {
    MessageBody {
        fence_language: body.fence_language,
        fence_body: body.fence_body,
    }
}

fn parameter_body_to_proto(body: &ParameterBody) -> pb::ParameterBody {
    pb::ParameterBody {
        name: body.name.clone(),
        location: body.location.clone(),
        required: body.required,
        fence_language: body.fence_language.clone(),
        fence_body: body.fence_body.clone(),
        ..Default::default()
    }
}

fn parameter_body_from_proto(body: pb::ParameterBody) -> ParameterBody {
    ParameterBody {
        name: body.name,
        location: body.location,
        required: body.required,
        fence_language: body.fence_language,
        fence_body: body.fence_body,
    }
}

fn response_body_to_proto(body: &ResponseBody) -> pb::ResponseBody {
    pb::ResponseBody {
        status: body.status.clone(),
        media_type: body.media_type.clone(),
        fence_language: body.fence_language.clone(),
        fence_body: body.fence_body.clone(),
        severity: buffa::EnumValue::from(link::response_severity_to_proto(body.severity)),
        ..Default::default()
    }
}

fn response_body_from_proto(body: pb::ResponseBody) -> ResponseBody {
    ResponseBody {
        status: body.status,
        severity: link::response_severity_from_proto(&body.severity),
        media_type: body.media_type,
        fence_language: body.fence_language,
        fence_body: body.fence_body,
    }
}

fn request_body_body_to_proto(body: &RequestBodyBody) -> pb::RequestBodyBody {
    pb::RequestBodyBody {
        required: body.required,
        fence_language: body.fence_language.clone(),
        fence_body: body.fence_body.clone(),
        ..Default::default()
    }
}

fn request_body_body_from_proto(body: pb::RequestBodyBody) -> RequestBodyBody {
    RequestBodyBody {
        required: body.required,
        fence_language: body.fence_language,
        fence_body: body.fence_body,
    }
}

fn security_scheme_body_to_proto(body: &SecuritySchemeBody) -> pb::SecuritySchemeBody {
    pb::SecuritySchemeBody {
        scheme_type: body.scheme_type.clone(),
        fence_language: body.fence_language.clone(),
        fence_body: body.fence_body.clone(),
        ..Default::default()
    }
}

fn security_scheme_body_from_proto(body: pb::SecuritySchemeBody) -> SecuritySchemeBody {
    SecuritySchemeBody {
        scheme_type: body.scheme_type,
        fence_language: body.fence_language,
        fence_body: body.fence_body,
    }
}

fn service_body_to_proto(body: &ServiceBody) -> pb::ServiceBody {
    pb::ServiceBody {
        signature: body.signature.clone(),
        fence_language: body.fence_language.clone(),
        fence_body: body.fence_body.clone(),
        ..Default::default()
    }
}

fn service_body_from_proto(body: pb::ServiceBody) -> ServiceBody {
    ServiceBody {
        signature: body.signature,
        fence_language: body.fence_language,
        fence_body: body.fence_body,
    }
}

fn extension_body_to_proto(body: &ExtensionBody) -> pb::ExtensionBody {
    pb::ExtensionBody {
        extension_type: body.extension_type.clone(),
        payload: body.payload.clone(),
        fence_language: opt_string(&body.fence_language),
        fence_body: opt_string(&body.fence_body),
        ..Default::default()
    }
}

fn extension_body_from_proto(body: pb::ExtensionBody) -> ExtensionBody {
    ExtensionBody {
        extension_type: body.extension_type,
        payload: body.payload,
        fence_language: string_opt(body.fence_language),
        fence_body: string_opt(body.fence_body),
    }
}
