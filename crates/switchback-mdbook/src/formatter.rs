//! Format a resolved link target for mdBook relative output.

use std::path::Path;

use switchback_traits::{EntityRef, LinkContext, LinkFormatter, LinkTarget, ProtobufEntityKind};

/// Default mdBook relative link formatter (`mdbook-relative`).
#[derive(Clone, Copy, Debug, Default)]
pub struct MdBookRelativeFormatter;

impl LinkFormatter for MdBookRelativeFormatter {
    fn name(&self) -> &'static str {
        "mdbook-relative"
    }

    fn format(&self, target: &LinkTarget, ctx: &LinkContext) -> String {
        match target {
            LinkTarget::Entity(entity_ref) => format_entity(entity_ref, ctx),
            LinkTarget::Group(group) => {
                let from = ctx
                    .render_from
                    .as_deref()
                    .unwrap_or_else(|| Path::new(&ctx.markdown_root));
                let target = ctx.package_page_rel(&group.group);
                ctx.summary_link(from, &target, &group.group)
            }
            LinkTarget::Unresolved => String::new(),
            _ => String::new(),
        }
    }
}

fn format_entity(entity_ref: &EntityRef, ctx: &LinkContext) -> String {
    let kind = if entity_ref.category == "service" {
        ProtobufEntityKind::Service
    } else if ctx
        .layout_entity_path(
            &entity_ref.group,
            ProtobufEntityKind::Enum,
            &entity_ref.name,
        )
        .is_some()
    {
        ProtobufEntityKind::Enum
    } else {
        ProtobufEntityKind::Message
    };
    let from = ctx
        .render_from
        .as_deref()
        .or_else(|| ctx.entity_paths.get(entity_ref).map(|p| p.as_path()))
        .unwrap_or_else(|| Path::new(&ctx.markdown_root));
    ctx.link_from(from, &entity_ref.group, kind, &entity_ref.name)
}
