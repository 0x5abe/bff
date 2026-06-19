use binrw::{BinRead, BinWrite};

use super::{DynArray, NumeratorFloat, Vec as BffVec, Vec2f, Vec3f, Vec4f};
use crate::names::Name;

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
    message_class: u32,
    reciever_name: Name,
    c: u32,
    parameter: f32,
    message_name: Name,
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
