use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::BffResult;
use crate::class::{Class, ClassType};
use crate::error::{MissingSourceResourceError, WrongSourceClassError};
use crate::helpers::{Quat, RGBA, Vec3f};
use crate::names::Name;
use crate::source::asset::{SourceAssetBuild, SourceAssetClass, SourceAssetData};
use crate::source::classes::user_define::UserDefineSourcePart;
use crate::source::context::UncookContext;
use crate::source::keyframer::{
    FollowTrack,
    SourceLinearTrack,
    SourceTangentTrack,
    StartStopTrack,
};
use crate::source::message::SourceMessageTrack;
use crate::source::part::{SourcePart, SourcePartBuild, ToSourcePart as _};
use crate::source::project::CookedProject;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeSource {
    pub node: NodeSourcePart,
    pub user_define: Option<UserDefineSourcePart>,
    pub anim_frames: Vec<AnimFrameSourcePart>,
}

impl NodeSource {
    pub fn build_from_project(
        name: Name,
        ctx: &mut UncookContext,
    ) -> BffResult<SourceAssetBuild<Self>> {
        let node_parts = ctx.require_part::<NodeSourcePartBuild>(&name)?;
        let mut represented_resources = node_parts.node.represented_resources;
        let mut preserved = node_parts.node.preserved;

        let user_define = node_parts
            .user_define_name
            .map(|name| ctx.require_part::<UserDefineSourcePart>(&name))
            .transpose()?;
        if let Some(user_define) = &user_define {
            represented_resources.push(user_define.name);
        }

        let mut anim_frames = Vec::new();
        for anim_frame_name in ctx.project().incoming_of_type(&name, ClassType::AnimFrame) {
            let anim_frame_parts =
                ctx.require_part::<AnimFrameSourcePartBuild>(&anim_frame_name)?;
            if anim_frame_parts.animated_node_name != name {
                continue;
            }

            represented_resources.extend(anim_frame_parts.anim_frame.represented_resources);
            preserved.extend(anim_frame_parts.anim_frame.preserved);
            anim_frames.push(anim_frame_parts.anim_frame.part);
        }

        for represented_resource in represented_resources {
            ctx.mark_represented_resource(represented_resource);
        }

        Ok(SourceAssetBuild {
            name,
            asset: Self {
                node: node_parts.node.part,
                user_define,
                anim_frames,
            },
            preserved,
        })
    }

    pub fn from_project(name: Name, ctx: &mut UncookContext) -> BffResult<Self> {
        Ok(Self::build_from_project(name, ctx)?.asset)
    }
}

impl SourceAssetClass for NodeSource {
    const SOURCE_CLASS_TYPE: ClassType = ClassType::Node;

    fn build(name: Name, ctx: &mut UncookContext) -> BffResult<SourceAssetBuild<Self>> {
        Self::build_from_project(name, ctx)
    }

    fn into_asset_data(self) -> SourceAssetData {
        SourceAssetData::Node(self)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum NodeFlag {
    Scan,
    Unknown0x2,
    Unknown0x4,
    Unknown0x8,
    Update,
    UpdateLighting,
    UpdateObject,
    InvalidMatrix,
    InvalidRotation,
    Animated,
    NoOmni,
    NoOccluder,
    NoAgent,
    Sequenced,
    Skinned,
    Uncollided,
    NoSeadCollide,
    NoSeadDisplay,
    Hide,
    UserLock,
    Vp0Hide,
    Vp1Hide,
    Vp2Hide,
    Vp3Hide,
    NoUnshared,
    Unknown0x2000000,
    Collide,
    Shadow,
    SequencedAbort,
    SpecialVision,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeFlags {
    pub flags: Vec<NodeFlag>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct NodeSourcePart {
    pub head_child_name: Name,
    pub next_node_name: Name,
    pub object_name: Name,
    pub radiosity_bitmap_name: Option<Name>,
    pub translation: Vec3f,
    pub rotation: Quat,
    pub scale: Vec3f,
    pub flags: NodeFlags,
    pub color: RGBA,
    pub start: f32,
    pub end: f32,
}

pub struct NodeSourcePartBuild {
    pub node: SourcePartBuild<NodeSourcePart>,
    pub user_define_name: Option<Name>,
}

impl SourcePart for NodeSourcePartBuild {
    fn from_project(name: &Name, project: &CookedProject) -> BffResult<Self> {
        let bff_class = project
            .class(name)
            .ok_or_else(|| MissingSourceResourceError::new(*name))?;

        let Class::Node(node) = &bff_class.class else {
            return Err(WrongSourceClassError::new(
                *name,
                ClassType::Node,
                bff_class.class.class_type(),
            )
            .into());
        };

        node.to_source_part(*name)
    }
}

impl SourcePart for NodeSourcePart {
    fn from_project(name: &Name, project: &CookedProject) -> BffResult<Self> {
        Ok(NodeSourcePartBuild::from_project(name, project)?.node.part)
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AnimFrameSourcePart {
    pub name: Name,
    pub duration: f32,
    pub play_flags: AnimFramePlayFlags,
    pub translation: SourceTangentTrack<Vec3f>,
    pub rotation: SourceLinearTrack<Quat>,
    pub scale: SourceTangentTrack<Vec3f>,
    pub time: SourceTangentTrack<f32>,
    pub color: SourceLinearTrack<Vec3f>,
    pub ambient: SourceLinearTrack<Vec3f>,
    pub messages: SourceMessageTrack,
    pub follow: FollowTrack,
    pub start_stop: StartStopTrack,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum AnimFramePlayFlag {
    Play,
    Started,
    ReadMessage,
    PlayOnce,
    NeverAgain,
    Played,
    AutoStart,
    Message,
    Paused,
    Unknown0x200,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct AnimFramePlayFlags {
    pub flags: Vec<AnimFramePlayFlag>,
}

pub struct AnimFrameSourcePartBuild {
    pub anim_frame: SourcePartBuild<AnimFrameSourcePart>,
    pub animated_node_name: Name,
}

impl SourcePart for AnimFrameSourcePartBuild {
    fn from_project(name: &Name, project: &CookedProject) -> BffResult<Self> {
        let bff_class = project
            .class(name)
            .ok_or_else(|| MissingSourceResourceError::new(*name))?;

        let Class::AnimFrame(anim_frame) = &bff_class.class else {
            return Err(WrongSourceClassError::new(
                *name,
                ClassType::AnimFrame,
                bff_class.class.class_type(),
            )
            .into());
        };

        anim_frame.to_source_part(*name)
    }
}

impl SourcePart for AnimFrameSourcePart {
    fn from_project(name: &Name, project: &CookedProject) -> BffResult<Self> {
        Ok(AnimFrameSourcePartBuild::from_project(name, project)?
            .anim_frame
            .part)
    }
}
