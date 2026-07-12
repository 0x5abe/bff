use bff_derive::ReferencedNames;
use binrw::binrw;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::v1_06_63_0x::MeshBodyV1_06_63_0X;
use crate::class::trivial_class::TrivialClass;
use crate::helpers::{DynArray, NumeratorFloat, ObjectLinkHeaderV1_06_63_02PC};
use crate::names::Name;
use crate::traits::{Export, Import};

const GX_VTXFMT1: u8 = 1;
const GX_VTXFMT2: u8 = 2;

#[derive(..BffStruct)]
struct MeshGCVertexRef {
    position_idx: u16,
    normal_idx: u16,
    texcoord0_idx: u16,
    texcoord1_idx: u16,
}

#[derive(..BffStruct)]
struct SkinGCVertexRef {
    position_idx: u16,
    normal_idx: u16,
    texcoord0_idx: u16,
}

#[binrw]
#[br(import(vertex_format: u8, vertex_ref_count: usize))]
#[derive(Debug, Serialize, Deserialize, ReferencedNames, JsonSchema)]
enum DisplayListVertexRefs {
    #[br(pre_assert(vertex_format == GX_VTXFMT1))]
    Mesh(#[br(count = vertex_ref_count)] Vec<MeshGCVertexRef>),
    #[br(pre_assert(vertex_format == GX_VTXFMT2))]
    Skin(#[br(count = vertex_ref_count)] Vec<SkinGCVertexRef>),
}

impl DisplayListVertexRefs {
    const fn len(&self) -> usize {
        match self {
            Self::Mesh(vertex_refs) => vertex_refs.len(),
            Self::Skin(vertex_refs) => vertex_refs.len(),
        }
    }

    const fn vertex_size(&self) -> usize {
        match self {
            Self::Mesh(_) => 8,
            Self::Skin(_) => 6,
        }
    }

    const fn byte_count(&self) -> usize {
        3 + self.len() * self.vertex_size()
    }
}

const fn align_display_list_size(byte_count: usize) -> usize {
    (byte_count + 31) & !31
}

#[binrw]
#[derive(Debug, Serialize, Deserialize, ReferencedNames, JsonSchema)]
struct DisplayList {
    #[br(temp)]
    #[bw(calc = align_display_list_size(vertex_refs.byte_count() + trailing_data.len()) as u32)]
    padded_byte_count: u32,
    #[br(temp)]
    #[bw(calc = (vertex_refs.byte_count() + trailing_data.len()) as u32)]
    byte_count: u32,
    primitive_and_vtx_fmt: u8,
    #[br(temp)]
    #[bw(calc = vertex_refs.len() as u16)]
    vertex_ref_count: u16,
    #[br(args(
        primitive_and_vtx_fmt & 0x7,
        vertex_ref_count as usize
    ))]
    vertex_refs: DisplayListVertexRefs,
    #[br(count = byte_count as usize - vertex_refs.byte_count())]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    trailing_data: Vec<u8>,
    #[br(temp, count = (padded_byte_count - byte_count) as usize)]
    #[bw(calc = vec![
        0;
        align_display_list_size(vertex_refs.byte_count() + trailing_data.len())
            - vertex_refs.byte_count()
            - trailing_data.len()
    ])]
    _padding: Vec<u8>,
}

#[derive(..BffStruct)]
#[br(import(link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct MeshBodyV1_06_63_01GC {
    #[br(args(link_header))]
    #[serde(flatten)]
    shared: MeshBodyV1_06_63_0X,
    optimized_texcoords: DynArray<NumeratorFloat<i16, 1024>>, // source: these are the baked tex coords so we'll have to extract them to source from here for this format (GameCube)
    optimized_normals: DynArray<NumeratorFloat<i8, 64>>, // source: these are the baked tex coords so we'll have to extract them to source from here for this format (GameCube)
    display_lists: DynArray<DisplayList>, // source: Here are the baked display lists for the mesh. There's one per material, and each one has the faces (indices), so we'll use this to get the face list (there wont be separated per material, each face will have a material index)
    material_link_names: DynArray<Name>,
}

pub type MeshV1_06_63_01GC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, MeshBodyV1_06_63_01GC>;

impl Export for MeshV1_06_63_01GC {}
impl Import for MeshV1_06_63_01GC {}
