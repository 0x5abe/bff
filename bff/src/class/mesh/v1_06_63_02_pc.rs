use std::io::Cursor;

use bff_derive::ReferencedNames;
use binrw::{BinRead, BinWrite, Endian, binrw};
use schemars::schema::Schema;
use schemars::{JsonSchema, SchemaGenerator};
use serde::de::Error as DeError;
use serde::ser::Error as SerError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::shared::{AABBNode, Strip};
use crate::class::trivial_class::TrivialClass;
use crate::helpers::{
    BffBox,
    Cylindre,
    DynArray,
    ObjectLinkHeaderV1_06_63_02PC,
    Sphere,
    Vec2f,
    Vec3f,
    Vec3i16,
};
use crate::names::Name;
use crate::traits::{Export, Import};

#[derive(..BffStruct)]
struct TBVtx {
    tangent: Vec3f,
    handedness: f32, // 1.0f or -1.0f
}

#[derive(..BffStruct)]
struct MorphValue {
    displacement: Vec3f,
    self_idx: u16,
    pad: u16,
}

#[derive(..BffStruct)]
struct MorphTargetDesc {
    name: Name,
    morph_values: DynArray<MorphValue>,
}

#[derive(..BffStruct)]
struct Morpher {
    morph_values: DynArray<MorphValue>,
    morph_target_descs: DynArray<MorphTargetDesc>,
}

#[derive(..BffStruct)]
struct SphereCol {
    col_sph: Sphere,
    flag: u32,
    name: Name,
}

#[derive(..BffStruct)]
struct BoxCol {
    col_box: BffBox,
    flag: u32,
    name: Name,
}

#[derive(..BffStruct)]
struct CylindreCol {
    col_cylindre: Cylindre,
    flag: u32,
    name: Name,
}

#[derive(..BffStruct)]
struct FaceCol {
    first_vertex_id: i16,
    second_vertex_id: i16,
    third_vertex_id: i16,
    material_index: i16,
}

#[derive(..BffStruct)]
struct AABBCol {
    collision_faces: DynArray<FaceCol>,
    collision_aabb_nodes: DynArray<AABBNode>,
}

#[derive(..BffStruct, Clone)]
struct PrimitiveInfo {
    vertex_buffer_index: u32,
    index_buffer_index: u32,
    obj_constant_placeholder_ptr: u32,
    primitive_type: u16,
    vertex_layout: u16,
    cdcd: u16,
    vertex_buffer_range_begin: u16,
    vertex_count: u16,
    index_buffer_offset_in_shorts: u16,
    face_count: u32,
    start_vertex: u16,
    vertex_stride: u16,
    cdcdcdcd: u32,
}

#[derive(..BffStruct)]
struct Points {
    positions: DynArray<Vec3f>,
    tangent_binormal_vertices: DynArray<TBVtx>,
    morpher: Morpher,
}

#[derive(..BffStruct)]
struct Vtx {
    uv_idx: u16,
    normal_idx: u16,
    color_idx: u16,
    vertices_idx: u16,
}

#[derive(..BffStruct)]
struct StripExt {
    vertices: DynArray<Vtx>,
}

#[derive(BinRead, BinWrite, Debug)]
struct MeshBuffers {
    vertex_buffers: DynArray<VertexBufferExt>,
    index_buffers: DynArray<IndexBufferExt>,
    prim_infos: DynArray<PrimitiveInfo>,
}

impl crate::traits::ReferencedNames for MeshBuffers {
    fn extend_referenced_names(&self, names: &mut std::collections::HashSet<Name>) {
        crate::traits::ReferencedNames::extend_referenced_names(&self.vertex_buffers, names);
        crate::traits::ReferencedNames::extend_referenced_names(&self.index_buffers, names);
        crate::traits::ReferencedNames::extend_referenced_names(&self.prim_infos, names);
    }
}

const PRIMITIVE_TYPE_TRIANGLE_LIST: u16 = 4;

const VERTEX_LAYOUT_NO_BLEND: u16 = 0;
const VERTEX_LAYOUT_VOLUME: u16 = 1;
const VERTEX_LAYOUT_SKIN_1_BLEND: u16 = 2;
const VERTEX_LAYOUT_SKIN_VOLUME: u16 = 3;
const VERTEX_LAYOUT_SKIN_4_BLEND: u16 = 4;
const VERTEX_LAYOUT_MORPH_1_BLEND: u16 = 5;
const VERTEX_LAYOUT_MORPH_4_BLEND: u16 = 6;

type VertexVectorComponent = u8;
type VertexVector3u8 = [VertexVectorComponent; 3];
type VertexBlendIndex = f32;
type VertexUVShort2N = [i16; 2];
type VertexBlendIndexU8 = u8;
type VertexBlendWeightU8 = u8;

#[derive(..BffStruct)]
struct VertexLayoutNoBlend {
    position: Vec3f,
    tangent: VertexVector3u8,
    tangent_w: VertexVectorComponent,
    normal: VertexVector3u8,
    normal_w: VertexVectorComponent,
    uv: Vec2f,
    lightmap_uv: Vec2f,
}

impl VertexLayoutNoBlend {
    const SIZE: u16 = 36;
}

#[derive(..BffStruct)]
struct VertexLayoutVolume {
    position: Vec3f,
    normal: Vec3f,
}

impl VertexLayoutVolume {
    const SIZE: u16 = 24;
}

#[derive(..BffStruct)]
struct VertexLayoutSkin1Blend {
    position: Vec3f,
    tangent: VertexVector3u8,
    tangent_w: VertexVectorComponent,
    normal: VertexVector3u8,
    normal_w: VertexVectorComponent,
    uv: Vec2f,
    blend_index: VertexBlendIndex,
    pad2: [i32; 3],
    blend_weight: f32,
}

impl VertexLayoutSkin1Blend {
    const SIZE: u16 = 48;
}

#[derive(..BffStruct)]
struct VertexLayoutSkinVolume {
    position0: Vec3f,
    position1: Vec3f,
    position2: Vec3f,
    blend_indices: [VertexBlendIndex; 3],
}

impl VertexLayoutSkinVolume {
    const SIZE: u16 = 48;
}

#[derive(..BffStruct)]
struct VertexLayoutSkin4Blend {
    position: Vec3f,
    tangent: VertexVector3u8,
    tangent_w: VertexVectorComponent,
    normal: VertexVector3u8,
    normal_w: VertexVectorComponent,
    uv: Vec2f,
    blend_indices: [VertexBlendIndex; 4],
    blend_weights: [f32; 4],
}

impl VertexLayoutSkin4Blend {
    const SIZE: u16 = 60;
}

#[derive(..BffStruct)]
struct VertexLayoutMorph1Blend {
    position: Vec3f,
    tangent: VertexVector3u8,
    tangent_w: VertexVectorComponent,
    normal: VertexVector3u8,
    normal_w: VertexVectorComponent,
    uv: VertexUVShort2N,
    blend_index: VertexBlendIndexU8,
    pad: [u8; 3],
    blend_weight: f32,
}

impl VertexLayoutMorph1Blend {
    const SIZE: u16 = 32;
}

#[derive(..BffStruct)]
struct VertexLayoutMorph4Blend {
    position: Vec3f,
    tangent: VertexVector3u8,
    tangent_w: VertexVectorComponent,
    normal: VertexVector3u8,
    normal_w: VertexVectorComponent,
    uv: VertexUVShort2N,
    blend_indices: [VertexBlendIndexU8; 4],
    blend_weights: [VertexBlendWeightU8; 4],
}

impl VertexLayoutMorph4Blend {
    const SIZE: u16 = 32;
}

#[derive(Debug, Serialize, Deserialize, ReferencedNames, JsonSchema)]
enum MeshVertexBuffer {
    NoBlend(Vec<VertexLayoutNoBlend>),
    Volume(Vec<VertexLayoutVolume>),
    Skin1Blend(Vec<VertexLayoutSkin1Blend>),
    SkinVolume(Vec<VertexLayoutSkinVolume>),
    Skin4Blend(Vec<VertexLayoutSkin4Blend>),
    Morph1Blend(Vec<VertexLayoutMorph1Blend>),
    Morph4Blend(Vec<VertexLayoutMorph4Blend>),
}

impl MeshVertexBuffer {
    fn layout(&self) -> u16 {
        match self {
            Self::NoBlend(_) => VERTEX_LAYOUT_NO_BLEND,
            Self::Volume(_) => VERTEX_LAYOUT_VOLUME,
            Self::Skin1Blend(_) => VERTEX_LAYOUT_SKIN_1_BLEND,
            Self::SkinVolume(_) => VERTEX_LAYOUT_SKIN_VOLUME,
            Self::Skin4Blend(_) => VERTEX_LAYOUT_SKIN_4_BLEND,
            Self::Morph1Blend(_) => VERTEX_LAYOUT_MORPH_1_BLEND,
            Self::Morph4Blend(_) => VERTEX_LAYOUT_MORPH_4_BLEND,
        }
    }

    fn stride(&self) -> u16 {
        match self {
            Self::NoBlend(_) => VertexLayoutNoBlend::SIZE,
            Self::Volume(_) => VertexLayoutVolume::SIZE,
            Self::Skin1Blend(_) => VertexLayoutSkin1Blend::SIZE,
            Self::SkinVolume(_) => VertexLayoutSkinVolume::SIZE,
            Self::Skin4Blend(_) => VertexLayoutSkin4Blend::SIZE,
            Self::Morph1Blend(_) => VertexLayoutMorph1Blend::SIZE,
            Self::Morph4Blend(_) => VertexLayoutMorph4Blend::SIZE,
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::NoBlend(vertices) => vertices.len(),
            Self::Volume(vertices) => vertices.len(),
            Self::Skin1Blend(vertices) => vertices.len(),
            Self::SkinVolume(vertices) => vertices.len(),
            Self::Skin4Blend(vertices) => vertices.len(),
            Self::Morph1Blend(vertices) => vertices.len(),
            Self::Morph4Blend(vertices) => vertices.len(),
        }
    }

    fn from_raw(layout: u16, stride: u16, raw_vertices: &[Vec<u8>]) -> Result<Self, String> {
        match layout {
            VERTEX_LAYOUT_NO_BLEND => read_vertex_buffer(
                raw_vertices,
                stride,
                VertexLayoutNoBlend::SIZE,
                Self::NoBlend,
            ),
            VERTEX_LAYOUT_VOLUME => {
                read_vertex_buffer(raw_vertices, stride, VertexLayoutVolume::SIZE, Self::Volume)
            }
            VERTEX_LAYOUT_SKIN_1_BLEND => read_vertex_buffer(
                raw_vertices,
                stride,
                VertexLayoutSkin1Blend::SIZE,
                Self::Skin1Blend,
            ),
            VERTEX_LAYOUT_SKIN_VOLUME => read_vertex_buffer(
                raw_vertices,
                stride,
                VertexLayoutSkinVolume::SIZE,
                Self::SkinVolume,
            ),
            VERTEX_LAYOUT_SKIN_4_BLEND => read_vertex_buffer(
                raw_vertices,
                stride,
                VertexLayoutSkin4Blend::SIZE,
                Self::Skin4Blend,
            ),
            VERTEX_LAYOUT_MORPH_1_BLEND => read_vertex_buffer(
                raw_vertices,
                stride,
                VertexLayoutMorph1Blend::SIZE,
                Self::Morph1Blend,
            ),
            VERTEX_LAYOUT_MORPH_4_BLEND => read_vertex_buffer(
                raw_vertices,
                stride,
                VertexLayoutMorph4Blend::SIZE,
                Self::Morph4Blend,
            ),
            _ => Err(format!("unsupported vertex layout {}", layout)),
        }
    }

    fn to_raw(&self) -> Result<Vec<Vec<u8>>, String> {
        match self {
            Self::NoBlend(vertices) => write_vertex_buffer(vertices, self.stride()),
            Self::Volume(vertices) => write_vertex_buffer(vertices, self.stride()),
            Self::Skin1Blend(vertices) => write_vertex_buffer(vertices, self.stride()),
            Self::SkinVolume(vertices) => write_vertex_buffer(vertices, self.stride()),
            Self::Skin4Blend(vertices) => write_vertex_buffer(vertices, self.stride()),
            Self::Morph1Blend(vertices) => write_vertex_buffer(vertices, self.stride()),
            Self::Morph4Blend(vertices) => write_vertex_buffer(vertices, self.stride()),
        }
    }
}

fn read_vertex_buffer<T>(
    raw_vertices: &[Vec<u8>],
    stride: u16,
    layout_size: u16,
    wrap: impl FnOnce(Vec<T>) -> MeshVertexBuffer,
) -> Result<MeshVertexBuffer, String>
where
    for<'a> T: BinRead<Args<'a> = ()>,
{
    let expected_stride = layout_size as usize;
    if stride as usize != expected_stride {
        return Err(format!(
            "vertex stride {} does not match {} byte layout",
            stride, expected_stride
        ));
    }

    let mut vertices = Vec::with_capacity(raw_vertices.len());
    for raw_vertex in raw_vertices {
        if raw_vertex.len() != expected_stride {
            return Err(format!(
                "raw vertex has {} bytes, expected {}",
                raw_vertex.len(),
                expected_stride
            ));
        }

        let mut reader = Cursor::new(raw_vertex);
        vertices.push(
            T::read_options(&mut reader, Endian::Little, ())
                .map_err(|error| format!("failed to read vertex: {}", error))?,
        );
    }

    Ok(wrap(vertices))
}

fn write_vertex_buffer<T>(vertices: &[T], stride: u16) -> Result<Vec<Vec<u8>>, String>
where
    for<'a> T: BinWrite<Args<'a> = ()>,
{
    let expected_stride = stride as usize;
    vertices
        .iter()
        .map(|vertex| {
            let mut writer = Cursor::new(Vec::new());
            vertex
                .write_options(&mut writer, Endian::Little, ())
                .map_err(|error| format!("failed to write vertex: {}", error))?;
            let raw_vertex = writer.into_inner();
            if raw_vertex.len() != expected_stride {
                return Err(format!(
                    "wrote {} vertex bytes, expected {}",
                    raw_vertex.len(),
                    expected_stride
                ));
            }
            Ok(raw_vertex)
        })
        .collect()
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
struct MeshBuffersSerde {
    vertex_buffers: Vec<MeshVertexBuffer>,
    index_buffers: Vec<IndexBufferExt>,
    prim_infos: Vec<PrimitiveInfo>,
}

impl MeshBuffers {
    fn infer_vertex_buffer_layout(&self, vertex_buffer_index: usize) -> Result<(u16, u16), String> {
        let mut layout_and_stride = None;

        for prim_info in &self.prim_infos.inner {
            if prim_info.vertex_buffer_index as usize != vertex_buffer_index {
                continue;
            }

            if prim_info.primitive_type != PRIMITIVE_TYPE_TRIANGLE_LIST {
                return Err(format!(
                    "primitive uses unsupported primitive type {}",
                    prim_info.primitive_type
                ));
            }

            let current = (prim_info.vertex_layout, prim_info.vertex_stride);
            if let Some(previous) = layout_and_stride {
                if previous != current {
                    return Err(format!(
                        "vertex buffer {} is referenced with multiple layouts/strides: {:?} and {:?}",
                        vertex_buffer_index, previous, current
                    ));
                }
            } else {
                layout_and_stride = Some(current);
            }
        }

        layout_and_stride.ok_or_else(|| {
            format!(
                "vertex buffer {} is not referenced by any primitive info",
                vertex_buffer_index
            )
        })
    }

    fn to_serde(&self) -> Result<MeshBuffersSerde, String> {
        let vertex_buffers = self
            .vertex_buffers
            .iter()
            .enumerate()
            .map(|(index, vertex_buffer)| {
                let (layout, stride) = self.infer_vertex_buffer_layout(index)?;
                if vertex_buffer.vertex_stride != stride {
                    return Err(format!(
                        "vertex buffer {} stride {} does not match primitive stride {}",
                        index, vertex_buffer.vertex_stride, stride
                    ));
                }
                MeshVertexBuffer::from_raw(layout, stride, &vertex_buffer.raw_vertices)
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(MeshBuffersSerde {
            vertex_buffers,
            index_buffers: self.index_buffers.inner.clone(),
            prim_infos: self.prim_infos.inner.clone(),
        })
    }

    fn from_serde(value: MeshBuffersSerde) -> Result<Self, String> {
        let vertex_buffers = value
            .vertex_buffers
            .iter()
            .map(|vertex_buffer| {
                Ok(VertexBufferExt {
                    vertex_stride: vertex_buffer.stride(),
                    raw_vertices: vertex_buffer.to_raw()?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        let mut prim_infos = value.prim_infos;
        for prim_info in &mut prim_infos {
            let Some(vertex_buffer) = value
                .vertex_buffers
                .get(prim_info.vertex_buffer_index as usize)
            else {
                return Err(format!(
                    "primitive references missing vertex buffer {}",
                    prim_info.vertex_buffer_index
                ));
            };

            let Some(index_buffer) = value
                .index_buffers
                .get(prim_info.index_buffer_index as usize)
            else {
                return Err(format!(
                    "primitive references missing index buffer {}",
                    prim_info.index_buffer_index
                ));
            };

            prim_info.vertex_layout = vertex_buffer.layout();
            prim_info.vertex_stride = vertex_buffer.stride();

            let vertex_range_end =
                prim_info.vertex_buffer_range_begin as usize + prim_info.vertex_count as usize;
            if vertex_range_end > vertex_buffer.len() {
                return Err(format!(
                    "primitive vertex range {}..{} exceeds vertex buffer length {}",
                    prim_info.vertex_buffer_range_begin,
                    vertex_range_end,
                    vertex_buffer.len()
                ));
            }

            let index_range_end = prim_info.index_buffer_offset_in_shorts as usize
                + prim_info.face_count as usize * 3;
            if index_range_end > index_buffer.indices.len() {
                return Err(format!(
                    "primitive index range {}..{} exceeds index buffer length {}",
                    prim_info.index_buffer_offset_in_shorts,
                    index_range_end,
                    index_buffer.indices.len()
                ));
            }
        }

        Ok(Self {
            vertex_buffers: vertex_buffers.into(),
            index_buffers: value.index_buffers.into(),
            prim_infos: prim_infos.into(),
        })
    }
}

impl Serialize for MeshBuffers {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.to_serde()
            .map_err(S::Error::custom)?
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MeshBuffers {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = MeshBuffersSerde::deserialize(deserializer)?;
        Self::from_serde(value).map_err(D::Error::custom)
    }
}

impl JsonSchema for MeshBuffers {
    fn schema_name() -> String {
        "MeshBuffersV1_06_63_02PC".to_owned()
    }

    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::MeshBuffers").into()
    }

    fn json_schema(schema_generator: &mut SchemaGenerator) -> Schema {
        MeshBuffersSerde::json_schema(schema_generator)
    }
}

#[binrw]
#[derive(Debug, ReferencedNames)]
struct VertexBufferExt {
    #[br(temp)]
    #[bw(calc = raw_vertices.len() as u16)]
    vertex_count: u16,
    vertex_stride: u16,
    #[br(args {
        count: vertex_count as usize,
        inner: binrw::args! { count: vertex_stride as usize },
    })]
    raw_vertices: Vec<Vec<u8>>,
}

#[binrw]
#[derive(Debug, Clone, Serialize, Deserialize, ReferencedNames, JsonSchema)]
struct IndexBufferExt {
    #[br(temp)]
    #[bw(calc = indices.len() as u16)]
    index_count: u16,
    #[br(count = index_count)]
    indices: Vec<u16>,
}

#[derive(..BffStruct)]
#[br(import(link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct MeshBodyV1_06_63_02PC {
    points: Points,
    uvs: DynArray<u32>,
    normals: DynArray<Vec3f>,
    strips: DynArray<Strip>,
    #[br(if(link_header.flags & 2 >= 1))]
    #[br(count = strips.len())]
    strip_override_material_indices: Option<Vec<u32>>,
    strip_exts: DynArray<StripExt>,
    material_names: DynArray<Name>,
    drawing_start_distance: f32,
    drawing_cutoff_distance: f32,
    shadow_related: u32,
    related_to_counts: [u32; 3],
    sphere_cols: DynArray<SphereCol>,
    box_cols: DynArray<BoxCol>,
    cylindre_cols: DynArray<CylindreCol>,
    aabb_col: AABBCol,
    aabb_col_vertices: DynArray<Vec3i16>,
    unknown0: DynArray<u32>,
    primitive_info_indices: DynArray<u32>,
    mesh_buffers: MeshBuffers,
}

pub type MeshV1_06_63_02PC = TrivialClass<ObjectLinkHeaderV1_06_63_02PC, MeshBodyV1_06_63_02PC>;

impl Export for MeshV1_06_63_02PC {}
impl Import for MeshV1_06_63_02PC {}
