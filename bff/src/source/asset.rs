use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::ClassType;
use crate::names::Name;
use crate::source::classes::node::NodeSource;
use crate::source::context::UncookContext;
use crate::source::preservation::PreservedFragment;
use crate::source::project::CookedProject;
use crate::{BffError, BffResult};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SourceAsset {
    pub name: Name,
    pub class_type: ClassType,
    #[serde(flatten)]
    pub data: SourceAssetData,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub preserved: Vec<PreservedFragment>,
}

impl SourceAsset {
    pub const fn class_type(&self) -> ClassType {
        self.class_type
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SourceAssetData {
    Node(NodeSource),
}

impl SourceAssetData {
    pub const fn class_type(&self) -> ClassType {
        match self {
            Self::Node(_) => ClassType::Node,
        }
    }
}

#[derive(Debug)]
pub struct SourceAssetBuildFailure {
    pub name: Name,
    pub class_type: ClassType,
    pub error: BffError,
}

pub struct SourceAssetBuild<T> {
    pub name: Name,
    pub asset: T,
    pub preserved: Vec<PreservedFragment>,
}

impl<T> SourceAssetBuild<T> {
    pub const fn new(name: Name, asset: T) -> Self {
        Self {
            name,
            asset,
            preserved: Vec::new(),
        }
    }
}

pub trait SourceAssetClass: Sized {
    const SOURCE_CLASS_TYPE: ClassType;

    fn candidate_class_type() -> ClassType {
        Self::SOURCE_CLASS_TYPE
    }

    fn build(name: Name, ctx: &mut UncookContext) -> BffResult<SourceAssetBuild<Self>>;

    fn into_asset_data(self) -> SourceAssetData;

    fn build_all(
        project: &CookedProject,
        ctx: &mut UncookContext,
        assets: &mut Vec<SourceAsset>,
        failures: &mut Vec<SourceAssetBuildFailure>,
    ) {
        let Some(names) = project.names_of_type(Self::candidate_class_type()) else {
            return;
        };

        for &name in names {
            if ctx.is_source_asset_built(Self::SOURCE_CLASS_TYPE, &name) {
                continue;
            }

            match Self::build(name, ctx) {
                Ok(build) => {
                    ctx.mark_source_asset_built(Self::SOURCE_CLASS_TYPE, build.name);
                    assets.push(SourceAsset {
                        name: build.name,
                        class_type: Self::SOURCE_CLASS_TYPE,
                        data: build.asset.into_asset_data(),
                        preserved: build.preserved,
                    });
                }
                Err(error) => failures.push(SourceAssetBuildFailure {
                    name,
                    class_type: Self::SOURCE_CLASS_TYPE,
                    error,
                }),
            }
        }
    }
}

type SourceAssetBuildFn = fn(
    &CookedProject,
    &mut UncookContext,
    &mut Vec<SourceAsset>,
    &mut Vec<SourceAssetBuildFailure>,
);

const SOURCE_ASSET_BUILDERS: &[SourceAssetBuildFn] = &[NodeSource::build_all];

pub fn build_source_assets(
    project: &CookedProject,
    ctx: &mut UncookContext,
) -> (Vec<SourceAsset>, Vec<SourceAssetBuildFailure>) {
    let mut assets = Vec::new();
    let mut failures = Vec::new();

    for build_assets in SOURCE_ASSET_BUILDERS {
        build_assets(project, ctx, &mut assets, &mut failures);
    }

    (assets, failures)
}
