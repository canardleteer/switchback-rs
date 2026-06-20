//! Compile-time and smoke tests for the public API.

use static_assertions::assert_impl_all;

use crate::{
    AsyncContractLoader, AsyncLinkExtractor, CompanionStrategy, Contract, ContractFamily,
    EntityCategory, GenericCategory, Layout, LinkContext, LinkExtractor, LinkFormatter,
    ReferenceManual, Renderer, SwitchbackCodec, SyncRenderer, SyncSwitchbackCodec,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum StubCategory {
    Schema,
}

impl EntityCategory for StubCategory {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Schema => "schema",
        }
    }

    fn dir(&self) -> &str {
        "schemas"
    }

    fn summary_prefix(&self) -> &str {
        "Schema"
    }

    fn to_generic(&self) -> Option<GenericCategory> {
        Some(GenericCategory::Schema)
    }
}

struct StubCompanion;

impl CompanionStrategy for StubCompanion {
    fn discovery(&self) -> crate::CompanionDiscovery {
        crate::CompanionDiscovery::Beside
    }

    fn output_name(&self, _source_dir: &[&str], stem: &str) -> String {
        format!("{stem}.md")
    }

    fn companion_media_types(&self) -> &'static [&'static str] {
        &["text/markdown"]
    }
}

struct StubLinkExtractor;

impl LinkExtractor for StubLinkExtractor {
    type Family = StubFamily;

    fn name(&self) -> &'static str {
        "stub"
    }

    fn extract<C: EntityCategory>(
        &self,
        _entity: &crate::Entity<C>,
        _manual: &crate::ResolvedManual,
    ) -> Vec<crate::IntraLink> {
        Vec::new()
    }
}

struct StubFamily;

impl ContractFamily for StubFamily {
    type Category = StubCategory;
    type LinkExtractor = StubLinkExtractor;
    type CompanionStrategy = StubCompanion;

    fn name(&self) -> &'static str {
        "Stub"
    }

    fn fence_language(&self) -> &'static str {
        "yaml"
    }

    fn default_title(&self) -> &'static str {
        "Stub documentation"
    }

    fn default_markdown_root(&self) -> &'static str {
        "contracts"
    }

    fn extensions(&self) -> &'static [&'static str] {
        &[".yaml"]
    }

    fn companion_strategy(&self) -> &Self::CompanionStrategy {
        &StubCompanion
    }

    fn category_names(&self) -> &'static [&'static str] {
        &["schema"]
    }

    fn detect_version(&self, _raw: &crate::RawDoc) -> Option<crate::SpecVersion> {
        None
    }

    fn supported_versions(&self) -> &'static [crate::SupportedVersion] {
        &[]
    }

    fn link_extractor(&self) -> &Self::LinkExtractor {
        &StubLinkExtractor
    }
}

struct StubLoaded {
    version: crate::SpecVersion,
}

impl Contract for StubLoaded {
    type Family = StubFamily;
    type Category = StubCategory;

    fn family(&self) -> &Self::Family {
        &StubFamily
    }

    fn version(&self) -> &crate::SpecVersion {
        &self.version
    }

    fn groups(&self) -> &[crate::Group] {
        &[]
    }

    fn entities(&self, _group: &crate::Group) -> &[crate::Entity<Self::Category>] {
        &[]
    }

    fn link_context(&self, opts: &crate::Options) -> LinkContext {
        LinkContext::empty(opts.layout, &opts.book_root, &opts.markdown_root)
    }
}

struct StubLoader;

impl AsyncContractLoader for StubLoader {
    type Family = StubFamily;
    type Loaded = StubLoaded;

    async fn load(&self) -> crate::Result<Self::Loaded> {
        Ok(StubLoaded {
            version: "1.0".into(),
        })
    }
}

struct StubCodec;

impl SwitchbackCodec for StubCodec {
    async fn serialize(&self, manual: &ReferenceManual) -> crate::Result<Vec<u8>> {
        Ok(manual.title.as_bytes().to_vec())
    }

    async fn deserialize(&self, bytes: &[u8]) -> crate::Result<ReferenceManual> {
        Ok(ReferenceManual {
            title: String::from_utf8_lossy(bytes).into_owned(),
            ..ReferenceManual::default()
        })
    }
}

impl SyncSwitchbackCodec for StubCodec {
    fn serialize(&self, manual: &ReferenceManual) -> crate::Result<Vec<u8>> {
        Ok(manual.title.as_bytes().to_vec())
    }

    fn deserialize(&self, bytes: &[u8]) -> crate::Result<ReferenceManual> {
        Ok(ReferenceManual {
            title: String::from_utf8_lossy(bytes).into_owned(),
            ..ReferenceManual::default()
        })
    }
}

struct StubRenderer;

impl Renderer for StubRenderer {
    type Opts = ();

    async fn render(
        &self,
        _manual: &ReferenceManual,
        _opts: &Self::Opts,
    ) -> crate::Result<Vec<crate::OutputFile>> {
        Ok(Vec::new())
    }
}

impl SyncRenderer for StubRenderer {
    type Opts = ();

    fn render(
        &self,
        _manual: &ReferenceManual,
        _opts: &Self::Opts,
    ) -> crate::Result<Vec<crate::OutputFile>> {
        Ok(Vec::new())
    }
}

struct StubFormatter;

impl LinkFormatter for StubFormatter {
    fn name(&self) -> &'static str {
        "passthrough"
    }

    fn format(&self, _target: &crate::LinkTarget, _ctx: &LinkContext) -> String {
        String::new()
    }
}

struct StubAsyncExtractor;

impl AsyncLinkExtractor for StubAsyncExtractor {
    type Family = StubFamily;

    async fn extract<C: EntityCategory>(
        &self,
        _entity: &crate::Entity<C>,
        _manual: &crate::ResolvedManual,
    ) -> crate::Result<Vec<crate::IntraLink>> {
        Ok(Vec::new())
    }
}

#[test]
fn entity_rel_path_smoke() {
    assert_eq!(
        crate::entity_rel_path("pets", "schemas", "Pet"),
        "pets/schemas/Pet.md"
    );
}

#[test]
fn resolved_manual_from_reference_manual() {
    use crate::{
        EntityBody, Group, GroupId, ManualContract, Module, ModuleId, ReferenceManual, SchemaBody,
        SpecVersion, StoredEntity,
    };

    let manual = ReferenceManual {
        switchback_version: "v1alpha1".into(),
        title: "Test".into(),
        sources: Vec::new(),
        modules: vec![Module {
            id: ModuleId::from("mod"),
            title: "Mod".into(),
            overview: String::new(),
            contracts: vec![ManualContract {
                family: "jsonschema".into(),
                version: SpecVersion::from("2020-12"),
                groups: vec![Group {
                    id: GroupId::from("g1"),
                    dir: "g1".into(),
                    title: "G1".into(),
                    overview: None,
                    source: None,
                    entities: vec![StoredEntity {
                        name: "Pet".into(),
                        category: "schema".into(),
                        title: "Pet".into(),
                        doc: None,
                        source: None,
                        refs: Vec::new(),
                        intra_links: Vec::new(),
                        body: EntityBody::Schema(SchemaBody::default()),
                    }],
                    source_path: Default::default(),
                }],
                companions: Vec::new(),
                protocols: Vec::new(),
            }],
        }],
    };

    let resolved = crate::ResolvedManual::from_reference_manual(&manual);
    assert_eq!(resolved.entities.len(), 1);
    assert_eq!(resolved.entities[0].module_id.as_str(), "mod");
    assert_eq!(resolved.entities[0].contract_family, "jsonschema");
    assert_eq!(resolved.entities[0].group_id.as_str(), "g1");
    assert_eq!(resolved.by_ref.len(), 1);
    assert!(resolved.by_ref.contains_key(&crate::EntityRef {
        module: "mod".into(),
        group: "g1".into(),
        category: "schema".into(),
        name: "Pet".into(),
    }));
}

#[test]
fn model_types_build() {
    let _manual = ReferenceManual {
        switchback_version: "v1alpha1".into(),
        title: "Acme API".into(),
        sources: Vec::new(),
        modules: Vec::new(),
    };
    let _ctx = LinkContext::empty(Layout::Package, ".", "src/packages");
    let _resolved = crate::ResolvedManual::default();
}

#[test]
fn seam_traits_are_send_sync() {
    assert_impl_all!(StubFamily: Send, Sync);
    assert_impl_all!(StubLoaded: Send, Sync);
    assert_impl_all!(StubCodec: Send, Sync);
    assert_impl_all!(StubRenderer: Send, Sync);
    assert_impl_all!(StubFormatter: Send, Sync);
    assert_impl_all!(StubLinkExtractor: Send, Sync);
    assert_impl_all!(ReferenceManual: Send, Sync);
    assert_impl_all!(LinkContext: Send, Sync);
}

#[test]
fn async_traits_are_object_safe_enough_to_name() {
    fn assert_loader<L: AsyncContractLoader>() {}
    fn assert_async_extractor<E: AsyncLinkExtractor>() {}
    fn assert_renderer<R: Renderer<Opts = ()>>() {}
    fn assert_codec<C: SwitchbackCodec>() {}

    assert_loader::<StubLoader>();
    assert_async_extractor::<StubAsyncExtractor>();
    assert_renderer::<StubRenderer>();
    assert_codec::<StubCodec>();
}

#[test]
fn stub_family_wires_associated_types() {
    let family = StubFamily;
    assert_eq!(family.name(), "Stub");
    assert_eq!(family.category_names(), &["schema"]);
    assert_eq!(
        family
            .link_extractor()
            .extract::<StubCategory>(
                &crate::Entity {
                    id: crate::EntityId::new("g", "schema", "Pet"),
                    category: StubCategory::Schema,
                    title: "Pet".into(),
                    doc: None,
                    source_span: None,
                    body: crate::EntityBody::Schema(crate::SchemaBody::default()),
                },
                &crate::ResolvedManual::default(),
            )
            .len(),
        0
    );
}
