//! Manual, document, module, and source mapping.

use switchback_traits::{Document, Module, ModuleId, ReferenceManual, Source, SourceRef, Span};

use crate::convert::WIRE_VERSION;
use crate::pb;

use super::contract;

pub fn reference_manual_to_proto(
    manual: &ReferenceManual,
) -> switchback_traits::Result<pb::ReferenceManual> {
    Ok(pb::ReferenceManual {
        switchback_version: WIRE_VERSION.to_string(),
        title: manual.title.clone(),
        sources: manual.sources.iter().map(document_to_proto).collect(),
        modules: manual
            .modules
            .iter()
            .map(module_to_proto)
            .collect::<switchback_traits::Result<_>>()?,
        ..Default::default()
    })
}

pub fn reference_manual_from_proto(
    manual: pb::ReferenceManual,
) -> switchback_traits::Result<ReferenceManual> {
    Ok(ReferenceManual {
        switchback_version: manual.switchback_version,
        title: manual.title,
        sources: manual
            .sources
            .into_iter()
            .map(document_from_proto)
            .collect(),
        modules: manual
            .modules
            .into_iter()
            .map(module_from_proto)
            .collect::<switchback_traits::Result<_>>()?,
    })
}

fn document_to_proto(doc: &Document) -> pb::Document {
    pb::Document {
        r#ref: buffa::MessageField::some(source_ref_to_proto(&doc.source_ref)),
        media_type: doc.media_type.clone(),
        content: doc.content.clone(),
        ..Default::default()
    }
}

fn document_from_proto(doc: pb::Document) -> Document {
    Document {
        source_ref: source_ref_from_proto(&doc.r#ref),
        media_type: doc.media_type,
        content: doc.content,
    }
}

fn source_ref_to_proto(source_ref: &SourceRef) -> pb::SourceRef {
    pb::SourceRef {
        uri: source_ref.uri.clone(),
        commit: source_ref.commit.clone(),
        content_hash: source_ref.content_hash.clone(),
        ..Default::default()
    }
}

fn source_ref_from_proto(source_ref: &pb::SourceRef) -> SourceRef {
    SourceRef {
        uri: source_ref.uri.clone(),
        commit: source_ref.commit.clone(),
        content_hash: source_ref.content_hash.clone(),
    }
}

fn module_to_proto(module: &Module) -> switchback_traits::Result<pb::Module> {
    Ok(pb::Module {
        id: module.id.as_str().to_string(),
        title: module.title.clone(),
        overview: module.overview.clone(),
        contracts: module
            .contracts
            .iter()
            .map(contract::contract_to_proto)
            .collect::<switchback_traits::Result<_>>()?,
        ..Default::default()
    })
}

fn module_from_proto(module: pb::Module) -> switchback_traits::Result<Module> {
    Ok(Module {
        id: ModuleId::from(module.id),
        title: module.title,
        overview: module.overview,
        contracts: module
            .contracts
            .into_iter()
            .map(contract::contract_from_proto)
            .collect::<switchback_traits::Result<_>>()?,
    })
}

pub(crate) fn source_to_proto(source: &Source) -> pb::Source {
    pb::Source {
        file: source.file.clone(),
        span: source
            .span
            .as_ref()
            .map(span_to_proto)
            .map(buffa::MessageField::some)
            .unwrap_or_default(),
        ..Default::default()
    }
}

pub(crate) fn source_from_proto(source: &pb::Source) -> Source {
    Source {
        file: source.file.clone(),
        span: if source.span.is_set() {
            Some(span_from_proto(&source.span))
        } else {
            None
        },
    }
}

fn span_to_proto(span: &Span) -> pb::Span {
    pb::Span {
        start_line: span.start_line as i32,
        start_col: span.start_col as i32,
        end_line: span.end_line as i32,
        end_col: span.end_col as i32,
        ..Default::default()
    }
}

fn span_from_proto(span: &pb::Span) -> Span {
    Span {
        start_line: span.start_line as u32,
        start_col: span.start_col as u32,
        end_line: span.end_line as u32,
        end_col: span.end_col as u32,
    }
}
