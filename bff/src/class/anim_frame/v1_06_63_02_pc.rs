use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};

use crate::class::message::source_message_v1_06_63_02_pc;
use crate::class::trivial_class::TrivialClass;
use crate::error::Error;
use crate::helpers::{
    KeyframerFloatComp,
    KeyframerFollow,
    KeyframerMessage,
    KeyframerRot,
    KeyframerStartStop,
    KeyframerVec3f,
    KeyframerVec3fComp,
    KeyframerVec3fLinear,
    ResourceObjectLinkHeaderV1_06_63_02PC,
    source_message_track,
};
use crate::names::Name;
use crate::source::classes::node::{AnimFrameSourcePart, AnimFrameSourceParts};
use crate::source::keyframer::ToSourceTrack;
use crate::source::part::Named;
use crate::traits::{Export, Import};

#[bitsize(16)]
#[derive(
    BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames, JsonSchemaBits,
)]
struct AnimFramePlayFlags {
    fl_anim_play: u1,
    fl_anim_started: u1,
    fl_anim_readmessage: u1,
    fl_anim_playonce: u1,
    fl_anim_neveragain: u1,
    fl_anim_played: u1,
    fl_anim_autostart: u1,
    fl_anim_message: u1,
    fl_anim_paused: u1,
    fl_anim_unk_0x200: u1,
    padding: u6,
}

#[derive(..BffStruct)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct AnimFrameBodyV1_06_63_02PC {
    animated_node_name: Name,
    duration: f32,
    translation_keyframer: KeyframerVec3f,
    rot_keyframer: KeyframerRot,
    scale_keyframer: KeyframerVec3fComp,
    time_keyframer: KeyframerFloatComp, // used to update follow_keyframer
    color_keyframer: KeyframerVec3fLinear, // for hfog/light/omni
    ambient_keyframer: KeyframerVec3fLinear, // for light
    msg_keyframer: KeyframerMessage,
    follow_keyframer: KeyframerFollow,
    start_stop_keyframer: KeyframerStartStop, // start and stop other anim frames
    cur_time: f32,
    start_time: f32,
    prev_time: f32,
    play_flags: AnimFramePlayFlags,
}

pub type AnimFrameV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, AnimFrameBodyV1_06_63_02PC>;

impl Export for AnimFrameV1_06_63_02PC {}
impl Import for AnimFrameV1_06_63_02PC {}

impl TryFrom<Named<'_, AnimFrameV1_06_63_02PC>> for AnimFrameSourceParts {
    type Error = Error;

    fn try_from(named: Named<'_, AnimFrameV1_06_63_02PC>) -> Result<Self, Self::Error> {
        let body = &named.value.body;

        Ok(Self {
            anim_frame: AnimFrameSourcePart {
                name: named.name,
                duration: body.duration,
                translation: body.translation_keyframer.to_source_track()?,
                rotation: body.rot_keyframer.to_source_track()?,
                scale: body.scale_keyframer.to_source_track()?,
                time: body.time_keyframer.to_source_track()?,
                color: body.color_keyframer.to_source_track()?,
                ambient: body.ambient_keyframer.to_source_track()?,
                messages: source_message_track(&body.msg_keyframer, source_message_v1_06_63_02_pc),
                follow: body.follow_keyframer.to_source_track()?,
                start_stop: body.start_stop_keyframer.to_source_track()?,
            },
            animated_node_name: body.animated_node_name,
            represented_resources: vec![named.name],
            preserved: Vec::new(),
        })
    }
}
