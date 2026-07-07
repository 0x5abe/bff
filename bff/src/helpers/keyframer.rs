use binrw::{BinRead, BinWrite};

use super::{DynArray, NumeratorFloat, Vec as BffVec, Vec2f, Vec3f, Vec4f};
use crate::BffResult;
use crate::names::Name;
use crate::source::keyframer::{
    Follow,
    FollowTrack,
    SourceInterpolation,
    SourceKey,
    SourceLinearTrack,
    SourceTangentKey,
    SourceTangentTrack,
    SourceTrack,
    StartStop as SourceStartStop,
    StartStopAction,
    StartStopTrack,
    ToSourceTrack,
};
use crate::source::message::{SourceMessage, SourceMessageTrack};

type Key = f32;

#[derive(..BffStruct)]
pub struct KeyTgtTplValue<T> {
    value: T,
    tangent_in: T,
    tangent_out: T,
}

#[derive(..BffStruct)]
pub struct KeyTgtTpl<T> {
    time: Key,
    #[brw(align_size_to = 4)]
    #[bw(fill_value = 0xFF)]
    #[serde(flatten)]
    value: KeyTgtTplValue<T>,
}

#[derive(..BffStruct)]
pub struct KeyLinearTpl<T> {
    time: Key,
    #[brw(align_size_to = 4)]
    #[bw(fill_value = 0xFF)]
    value: T,
}

#[derive(..BffStruct)]
#[brw(repr = u16)]
pub enum KeyframerInterpolationType {
    Smooth = 1,
    Linear = 2,
    Square = 3,
    Unknown4 = 4,   // scroll_keyframer in MaterialAnim uses this
    Unknown8 = 8,   // scroll_keyframer in MaterialAnim uses this
    Unknown17 = 17, // unknown1 in Rtc's RtcAnimationNode uses this
}

#[derive(..BffStruct)]
#[br(bound(for<'a> TKey: BinRead<Args<'a>: Clone + Default> + 'a))]
#[bw(bound(for<'a> TKey: BinWrite<Args<'a>: Clone + Default> + 'a))]
pub struct KeyframerTpl<TKey> {
    interpolation_type: KeyframerInterpolationType,
    keyframes: DynArray<TKey>,
}

#[derive(..BffStruct)]
#[br(bound(for<'a> TKey: BinRead<Args<'a>: Clone + Default> + 'a))]
#[bw(bound(for<'a> TKey: BinWrite<Args<'a>: Clone + Default> + 'a))]
pub struct KeyframerNoFlagsTpl<TKey> {
    keyframes: DynArray<TKey>,
}

#[derive(..BffStruct)]
pub struct Message {
    pub message_id: u32,
    pub u32_param: Name,
    pub flag_param: u32,
    pub float_param: f32,
    pub name_param: Name,
}

#[derive(..BffStruct)]
pub struct KeyFollow {
    node_name: Name,
    axis: Vec3f,
    progress: f32,
    orientation: u32,
}

#[derive(..BffStruct)]
pub struct StartStop {
    anim_frame_name: Name,
    value: u32,
}

#[derive(..BffStruct)]
pub struct KeyStartStop {
    start_stop_count: u32,
    time: f32,
    #[br(count = start_stop_count)]
    start_stops: Vec<StartStop>,
}

pub type Vec4Comp = BffVec<4, NumeratorFloat<i16, 4096>>;
pub type Vec3Comp = BffVec<3, NumeratorFloat<i16, 4096>>;
pub type Vec2Comp = BffVec<2, NumeratorFloat<i16, 4096>>;
pub type QuatComp = BffVec<4, NumeratorFloat<i16, 2000>>;

pub type KeyFlag = KeyLinearTpl<u32>;
pub type KeyHdl = KeyLinearTpl<Name>;
pub type KeyMessage = KeyLinearTpl<DynArray<Message>>;
pub type KeyFloat = KeyTgtTpl<f32>;
pub type KeyFloatComp = KeyTgtTpl<NumeratorFloat<i16, 4096>>;
pub type KeyFloatLinear = KeyLinearTpl<f32>;
pub type KeyFloatLinearComp = KeyLinearTpl<NumeratorFloat<i16, 4096>>;
pub type KeyU32Linear = KeyLinearTpl<u32>;
pub type KeyVec2f = KeyTgtTpl<Vec2f>;
pub type KeyVec2fComp = KeyTgtTpl<Vec2Comp>;
pub type KeyVec2fLinear = KeyLinearTpl<Vec2f>;
pub type KeyVec2fLinearComp = KeyLinearTpl<Vec2Comp>;
pub type KeyVec3f = KeyTgtTpl<Vec3f>;
pub type KeyVec3fComp = KeyTgtTpl<Vec3Comp>;
pub type KeyVec3fLinear = KeyLinearTpl<Vec3f>;
pub type KeyVec3fLinearComp = KeyLinearTpl<Vec3Comp>;
pub type KeyVec4f = KeyTgtTpl<Vec4f>;
pub type KeyVec4fComp = KeyTgtTpl<Vec4Comp>;
pub type KeyVec4fLinear = KeyLinearTpl<Vec4f>;
pub type KeyVec4fLinearComp = KeyLinearTpl<Vec4Comp>;
pub type KeyRot = KeyLinearTpl<QuatComp>;
pub type KeyBezierRot = KeyTgtTpl<Vec3f>;

pub type KeyframerFlag = KeyframerNoFlagsTpl<KeyFlag>;
pub type KeyframerHdl = KeyframerNoFlagsTpl<KeyHdl>;
pub type KeyframerMessage = KeyframerNoFlagsTpl<KeyMessage>;
pub type KeyframerFloat = KeyframerTpl<KeyFloat>;
pub type KeyframerFloatComp = KeyframerTpl<KeyFloatComp>;
pub type KeyframerFloatLinear = KeyframerTpl<KeyFloatLinear>;
pub type KeyframerFloatLinearComp = KeyframerTpl<KeyFloatLinearComp>;
pub type KeyframerU32Linear = KeyframerTpl<KeyU32Linear>;
pub type KeyframerVec2f = KeyframerTpl<KeyVec2f>;
pub type KeyframerVec2fComp = KeyframerTpl<KeyVec2fComp>;
pub type KeyframerVec2fLinear = KeyframerTpl<KeyVec2fLinear>;
pub type KeyframerVec2fLinearComp = KeyframerTpl<KeyVec2fLinearComp>;
pub type KeyframerVec3f = KeyframerTpl<KeyVec3f>;
pub type KeyframerVec3fComp = KeyframerTpl<KeyVec3fComp>;
pub type KeyframerVec3fLinear = KeyframerTpl<KeyVec3fLinear>;
pub type KeyframerVec3fLinearComp = KeyframerTpl<KeyVec3fLinearComp>;
pub type KeyframerVec4f = KeyframerTpl<KeyVec4f>;
pub type KeyframerVec4fComp = KeyframerTpl<KeyVec4fComp>;
pub type KeyframerVec4fLinear = KeyframerTpl<KeyVec4fLinear>;
pub type KeyframerVec4fLinearComp = KeyframerTpl<KeyVec4fLinearComp>;
pub type KeyframerRot = KeyframerNoFlagsTpl<KeyRot>;
pub type KeyframerBezierRot = KeyframerNoFlagsTpl<KeyBezierRot>;
pub type KeyframerFollow = KeyframerNoFlagsTpl<KeyFollow>;
pub type KeyframerStartStop = KeyframerNoFlagsTpl<KeyStartStop>;

const fn source_interpolation(
    interpolation_type: &KeyframerInterpolationType,
) -> SourceInterpolation {
    match interpolation_type {
        KeyframerInterpolationType::Smooth => SourceInterpolation::Smooth,
        KeyframerInterpolationType::Linear => SourceInterpolation::Linear,
        KeyframerInterpolationType::Square => SourceInterpolation::Square,
        KeyframerInterpolationType::Unknown4 => SourceInterpolation::Unknown4,
        KeyframerInterpolationType::Unknown8 => SourceInterpolation::Unknown8,
        KeyframerInterpolationType::Unknown17 => SourceInterpolation::Unknown17,
    }
}

fn source_tangent_track<T, U>(
    keyframer: &KeyframerTpl<KeyTgtTpl<T>>,
    convert: fn(&T) -> U,
) -> SourceTangentTrack<U> {
    SourceTrack {
        interpolation: source_interpolation(&keyframer.interpolation_type),
        keyframes: keyframer
            .keyframes
            .iter()
            .map(|key| SourceTangentKey {
                time: key.time,
                value: convert(&key.value.value),
                tangent_in: convert(&key.value.tangent_in),
                tangent_out: convert(&key.value.tangent_out),
            })
            .collect(),
    }
}

fn source_linear_track<T, U>(
    keyframer: &KeyframerTpl<KeyLinearTpl<T>>,
    convert: fn(&T) -> U,
) -> SourceLinearTrack<U> {
    SourceTrack {
        interpolation: source_interpolation(&keyframer.interpolation_type),
        keyframes: keyframer
            .keyframes
            .iter()
            .map(|key| SourceKey {
                time: key.time,
                value: convert(&key.value),
            })
            .collect(),
    }
}

fn source_linear_no_flags_track<T, U>(
    keyframer: &KeyframerNoFlagsTpl<KeyLinearTpl<T>>,
    convert: fn(&T) -> U,
) -> SourceLinearTrack<U> {
    SourceTrack {
        interpolation: SourceInterpolation::Linear,
        keyframes: keyframer
            .keyframes
            .iter()
            .map(|key| SourceKey {
                time: key.time,
                value: convert(&key.value),
            })
            .collect(),
    }
}

const fn copy_value<T: Copy>(value: &T) -> T {
    *value
}

fn numerator_float<const DENOMINATOR: usize>(value: &NumeratorFloat<i16, DENOMINATOR>) -> f32 {
    **value
}

fn vec3_comp(value: &Vec3Comp) -> Vec3f {
    [
        numerator_float(&value[0]),
        numerator_float(&value[1]),
        numerator_float(&value[2]),
    ]
}

fn quat_comp(value: &QuatComp) -> Vec4f {
    [
        numerator_float(&value[0]),
        numerator_float(&value[1]),
        numerator_float(&value[2]),
        numerator_float(&value[3]),
    ]
}

pub fn source_message_track(
    keyframer: &KeyframerMessage,
    convert_message: fn(&Message) -> SourceMessage,
) -> SourceMessageTrack {
    SourceTrack {
        interpolation: SourceInterpolation::Linear,
        keyframes: keyframer
            .keyframes
            .iter()
            .map(|key| SourceKey {
                time: key.time,
                value: key.value.iter().map(convert_message).collect(),
            })
            .collect(),
    }
}

const fn source_message(message: &Message) -> SourceMessage {
    SourceMessage::Raw {
        message_id: message.message_id,
        u32_param: message.u32_param,
        flag_param: message.flag_param,
        float_param: message.float_param,
        name_param: message.name_param,
    }
}

const fn start_stop_action(value: u32) -> StartStopAction {
    match value {
        0 => StartStopAction::Stop,
        1 => StartStopAction::Start,
        2 => StartStopAction::Pause,
        value => StartStopAction::Unknown(value),
    }
}

impl ToSourceTrack<SourceTangentTrack<Vec3f>> for KeyframerVec3f {
    fn to_source_track(&self) -> BffResult<SourceTangentTrack<Vec3f>> {
        Ok(source_tangent_track(self, copy_value))
    }
}

impl ToSourceTrack<SourceTangentTrack<Vec3f>> for KeyframerVec3fComp {
    fn to_source_track(&self) -> BffResult<SourceTangentTrack<Vec3f>> {
        Ok(source_tangent_track(self, vec3_comp))
    }
}

impl ToSourceTrack<SourceTangentTrack<f32>> for KeyframerFloatComp {
    fn to_source_track(&self) -> BffResult<SourceTangentTrack<f32>> {
        Ok(source_tangent_track(self, numerator_float))
    }
}

impl ToSourceTrack<SourceLinearTrack<Vec3f>> for KeyframerVec3fLinear {
    fn to_source_track(&self) -> BffResult<SourceLinearTrack<Vec3f>> {
        Ok(source_linear_track(self, copy_value))
    }
}

impl ToSourceTrack<SourceLinearTrack<Vec4f>> for KeyframerRot {
    fn to_source_track(&self) -> BffResult<SourceLinearTrack<Vec4f>> {
        Ok(source_linear_no_flags_track(self, quat_comp))
    }
}

impl ToSourceTrack<SourceMessageTrack> for KeyframerMessage {
    fn to_source_track(&self) -> BffResult<SourceMessageTrack> {
        Ok(source_message_track(self, source_message))
    }
}

impl ToSourceTrack<FollowTrack> for KeyframerFollow {
    fn to_source_track(&self) -> BffResult<FollowTrack> {
        Ok(SourceTrack {
            interpolation: SourceInterpolation::Linear,
            keyframes: self
                .keyframes
                .iter()
                .map(|key| SourceKey {
                    // V1_06 cooked follow keys do not store a key time. Source keeps
                    // one for formats that do.
                    time: 0.0,
                    value: Follow {
                        spline_node_name: key.node_name,
                        axis: key.axis,
                        orient_to_spline: key.orientation != 0,
                        advance: key.progress,
                    },
                })
                .collect(),
        })
    }
}

impl ToSourceTrack<StartStopTrack> for KeyframerStartStop {
    fn to_source_track(&self) -> BffResult<StartStopTrack> {
        Ok(SourceTrack {
            interpolation: SourceInterpolation::Linear,
            keyframes: self
                .keyframes
                .iter()
                .map(|key| SourceKey {
                    time: key.time,
                    value: key
                        .start_stops
                        .iter()
                        .map(|start_stop| SourceStartStop {
                            anim_frame_name: start_stop.anim_frame_name,
                            action: start_stop_action(start_stop.value),
                        })
                        .collect(),
                })
                .collect(),
        })
    }
}
