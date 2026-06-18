use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::class::trivial_class::TrivialClass;
use crate::helpers::{Mat3x4f, Vec3f, RGB, RGBA, ResourceObjectLinkHeaderV1_06_63_02PC};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(BinRead, Debug, Serialize, BinWrite, Deserialize, JsonSchema, ReferencedNames)]
#[br(import(_link_header: &ResourceObjectLinkHeaderV1_06_63_02PC))]
pub struct MaterialBodyV1_291_03_06PC {
    pub diffuse: RGBA,
    pub emission: RGB,
    cdcdcdcd: u32,
    uv_transform_matrix: Mat3x4f,
    specular: Vec3f,
    specular_pow: f32,
    params: [u32; 4],
    diffuse_translation: [f32; 2],
    diffuse_scale: [f32; 2],
    diffuse_rotation: f32,
    flags: [u32; 3],
    texture_flag: u8,
    textures: [Name; 8],
}

pub type MaterialV1_291_03_06PC =
    TrivialClass<ResourceObjectLinkHeaderV1_06_63_02PC, MaterialBodyV1_291_03_06PC>;

impl Export for MaterialV1_291_03_06PC {}
impl Import for MaterialV1_291_03_06PC {}
