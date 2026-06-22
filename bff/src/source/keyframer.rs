use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::BffResult;
use crate::helpers::Vec3f;
use crate::names::Name;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum SourceInterpolation {
    Smooth,
    Linear,
    Square,
    Unknown4,
    Unknown8,
    Unknown17,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SourceTrack<K> {
    pub interpolation: SourceInterpolation,
    pub keyframes: Vec<K>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SourceKey<T> {
    pub time: f32,
    pub value: T,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct SourceTangentKey<T> {
    pub time: f32,
    pub value: T,
    pub tangent_in: T,
    pub tangent_out: T,
}

pub type SourceLinearTrack<T> = SourceTrack<SourceKey<T>>;
pub type SourceTangentTrack<T> = SourceTrack<SourceTangentKey<T>>;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Follow {
    pub spline_node_name: Name,
    pub axis: Vec3f,
    pub orient_to_spline: bool,
    pub advance: f32,
}

pub type FollowTrack = SourceLinearTrack<Follow>;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub enum StartStopAction {
    Stop,
    Start,
    Pause,
    Unknown(u32),
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct StartStop {
    pub anim_frame_name: Name,
    pub action: StartStopAction,
}

pub type StartStopTrack = SourceLinearTrack<Vec<StartStop>>;

pub trait ToSourceTrack<T> {
    fn to_source_track(&self) -> BffResult<T>;
}
