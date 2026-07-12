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
use crate::source::classes::node::{
    AnimFramePlayFlag,
    AnimFramePlayFlags as SourceAnimFramePlayFlags,
    AnimFrameSourcePart,
    AnimFrameSourcePartBuild,
};
use crate::source::flags::{FlagMapping, decode_flags};
use crate::source::keyframer::ToSourceTrack as _;
use crate::source::part::{Named, SourcePartBuild};
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

impl TryFrom<Named<'_, AnimFrameV1_06_63_02PC>> for AnimFrameSourcePartBuild {
    type Error = Error;

    fn try_from(named: Named<'_, AnimFrameV1_06_63_02PC>) -> Result<Self, Self::Error> {
        let body = &named.value.body;

        Ok(Self {
            anim_frame: SourcePartBuild::new(named.name, AnimFrameSourcePart {
                name: named.name,
                duration: body.duration,
                play_flags: SourceAnimFramePlayFlags {
                    flags: decode_flags(
                        anim_frame_play_flags_raw(&body.play_flags),
                        ANIM_FRAME_PLAY_FLAGS_V1_06_63_02_PC,
                    )?,
                },
                translation: body.translation_keyframer.to_source_track()?,
                rotation: body.rot_keyframer.to_source_track()?,
                scale: body.scale_keyframer.to_source_track()?,
                time: body.time_keyframer.to_source_track()?,
                color: body.color_keyframer.to_source_track()?,
                ambient: body.ambient_keyframer.to_source_track()?,
                messages: source_message_track(&body.msg_keyframer, source_message_v1_06_63_02_pc),
                follow: body.follow_keyframer.to_source_track()?,
                start_stop: body.start_stop_keyframer.to_source_track()?,
            }),
            animated_node_name: body.animated_node_name,
        })
    }
}

fn anim_frame_play_flags_raw(flags: &AnimFramePlayFlags) -> u32 {
    let mut raw = 0;

    if u8::from(flags.fl_anim_play()) != 0 {
        raw |= 1 << 0;
    }
    if u8::from(flags.fl_anim_started()) != 0 {
        raw |= 1 << 1;
    }
    if u8::from(flags.fl_anim_readmessage()) != 0 {
        raw |= 1 << 2;
    }
    if u8::from(flags.fl_anim_playonce()) != 0 {
        raw |= 1 << 3;
    }
    if u8::from(flags.fl_anim_neveragain()) != 0 {
        raw |= 1 << 4;
    }
    if u8::from(flags.fl_anim_played()) != 0 {
        raw |= 1 << 5;
    }
    if u8::from(flags.fl_anim_autostart()) != 0 {
        raw |= 1 << 6;
    }
    if u8::from(flags.fl_anim_message()) != 0 {
        raw |= 1 << 7;
    }
    if u8::from(flags.fl_anim_paused()) != 0 {
        raw |= 1 << 8;
    }
    if u8::from(flags.fl_anim_unk_0x200()) != 0 {
        raw |= 1 << 9;
    }

    raw | (u32::from(u8::from(flags.padding_i())) << 10)
}

const ANIM_FRAME_PLAY_FLAGS_V1_06_63_02_PC: &[FlagMapping<AnimFramePlayFlag>] = &[
    FlagMapping {
        raw: 1 << 0,
        source: AnimFramePlayFlag::Play,
    },
    FlagMapping {
        raw: 1 << 1,
        source: AnimFramePlayFlag::Started,
    },
    FlagMapping {
        raw: 1 << 2,
        source: AnimFramePlayFlag::ReadMessage,
    },
    FlagMapping {
        raw: 1 << 3,
        source: AnimFramePlayFlag::PlayOnce,
    },
    FlagMapping {
        raw: 1 << 4,
        source: AnimFramePlayFlag::NeverAgain,
    },
    FlagMapping {
        raw: 1 << 5,
        source: AnimFramePlayFlag::Played,
    },
    FlagMapping {
        raw: 1 << 6,
        source: AnimFramePlayFlag::AutoStart,
    },
    FlagMapping {
        raw: 1 << 7,
        source: AnimFramePlayFlag::Message,
    },
    FlagMapping {
        raw: 1 << 8,
        source: AnimFramePlayFlag::Paused,
    },
    FlagMapping {
        raw: 1 << 9,
        source: AnimFramePlayFlag::Unknown0x200,
    },
];
