use crate::helpers::Vec3f;
use crate::names::Name;

pub enum SourceInterpolation {
    Smooth,
    Linear,
    Square,
    Unknown4,
    Unknown8,
    Unknown17,
}

pub struct SourceTrack<K> {
    pub interpolation: SourceInterpolation,
    pub keyframes: Vec<K>,
}

pub struct SourceKey<T> {
    pub time: f32,
    pub value: T,
}

pub struct SourceTangentKey<T> {
    pub time: f32,
    pub value: T,
    pub tangent_in: T,
    pub tangent_out: T,
}

pub type SourceLinearTrack<T> = SourceTrack<SourceKey<T>>;
pub type SourceTangentTrack<T> = SourceTrack<SourceTangentKey<T>>;

pub struct Follow {
    pub spline_node_name: Name,
    pub axis: Vec3f,
    pub orient_to_spline: bool,
    pub advance: f32,
}

pub type FollowTrack = SourceLinearTrack<Follow>;

pub enum StartStopAction {
    Stop,
    Start,
    Pause,
    Unknown(u32),
}

pub struct StartStop {
    pub anim_frame_name: Name,
    pub action: StartStopAction,
}

pub type StartStopTrack = SourceLinearTrack<Vec<StartStop>>;
