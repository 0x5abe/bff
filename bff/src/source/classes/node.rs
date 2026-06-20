use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::BffResult;
use crate::class::{Class, ClassType};
use crate::error::{MissingSourceResourceError, WrongSourceClassError};
use crate::helpers::{Quat, RGBA, Vec3f};
use crate::names::Name;
use crate::source::keyframer::{
    FollowTrack,
    SourceLinearTrack,
    SourceTangentTrack,
    StartStopTrack,
};
use crate::source::message::SourceMessageTrack;
use crate::source::part::{SourcePart, ToSourcePart};
use crate::source::project::CookedProject;

pub struct NodeSource {
    pub name: Name,
    pub node: NodeSourcePart,
    pub user_define: Option<UserDefineSourcePart>,
    pub anim_frames: Vec<AnimFrameSourcePart>,
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

pub struct NodeSourcePart {
    pub name: Name,
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

impl SourcePart for NodeSourcePart {
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

pub struct UserDefineSourcePart {
    pub name: Name,
    pub data: String,
}

impl SourcePart for UserDefineSourcePart {
    fn from_project(name: &Name, project: &CookedProject) -> BffResult<Self> {
        let bff_class = project
            .class(name)
            .ok_or_else(|| MissingSourceResourceError::new(*name))?;

        let Class::UserDefine(user_define) = &bff_class.class else {
            return Err(WrongSourceClassError::new(
                *name,
                ClassType::UserDefine,
                bff_class.class.class_type(),
            )
            .into());
        };

        user_define.to_source_part(*name)
    }
}

pub struct AnimFrameSourcePart {
    pub name: Name,
    pub animated_node_name: Name,
    pub duration: f32,
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
