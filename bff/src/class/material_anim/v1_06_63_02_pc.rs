use bff_derive::ReferencedNames;
use bilge::prelude::*;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    KeyframerFlag,
    KeyframerFloatLinearComp,
    KeyframerHdl,
    KeyframerVec2fLinear,
    KeyframerVec3fLinear,
    KeyframerVec4fLinear,
    ResourceObjectLinkHeaderV1_06_63_02PC,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[bitsize(8)]
#[derive(
    BinRead, DebugBits, SerializeBits, BinWrite, DeserializeBits, ReferencedNames, JsonSchema,
)]
struct MaterialAnimFlags {
    fl_mat_play: u1,
    fl_mat_played: u1,
    fl_mat_playonce: u1,
    fl_mat_neveragain: u1,
    fl_mat_autostart: u1,
    flag_5: u1,
    flag_6: u1,
    flag_7: u1,
}

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct MaterialAnimBodyV1_06_63_02PC {
    flags: MaterialAnimFlags,
    duration: f32,
    bitmap_name_keyframer: KeyframerHdl,
    translation_keyframer: KeyframerVec2fLinear,
    scale_keyframer: KeyframerVec2fLinear,
    rotation_keyframer: KeyframerFloatLinearComp,
    diffuse_color_keyframer: KeyframerVec3fLinear,
    emissive_color_keyframer: KeyframerVec3fLinear,
    alpha_keyframer: KeyframerFloatLinearComp,
    specular_keyframer: KeyframerVec4fLinear,
    params_keyframer: KeyframerVec4fLinear,
    collision_flag_keyframer: KeyframerFlag,
    render_flag_keyframer: KeyframerFlag,
    object_flag_keyframer: KeyframerFlag,
    material_name: Name,
}

pub type MaterialAnimV1_06_63_02PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, MaterialAnimBodyV1_06_63_02PC>;

impl Export for MaterialAnimV1_06_63_02PC {}
impl Import for MaterialAnimV1_06_63_02PC {}
