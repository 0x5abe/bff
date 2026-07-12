use crate::class::mesh::shared::{AABBNode, Strip};
use crate::helpers::{
    BffBox,
    Cylindre,
    DynArray,
    NumeratorFloat,
    ObjectLinkHeaderV1_06_63_02PC,
    Sphere,
    Vec2f,
    Vec3f,
};
use crate::names::Name;

type OptimizedVertex = [NumeratorFloat<i16, 1024>; 3];
type OptimizedSkinnedVertex = [NumeratorFloat<i16, 4096>; 3];

#[derive(..BffStruct)]
pub struct TBVtx {
    tangent: Vec3f,
    handedness: f32,
}

#[derive(..BffStruct)]
pub struct MorphValue {
    displacement: Vec3f,
    self_idx: u16,
    pad: u16,
}

#[derive(..BffStruct)]
pub struct MorphTargetDesc {
    name: Name,
    morph_values: DynArray<MorphValue>,
}

#[derive(..BffStruct)]
pub struct Morpher {
    morph_values: DynArray<MorphValue>,
    morph_target_descs: DynArray<MorphTargetDesc>,
}

#[derive(..BffStruct)]
pub struct Points {
    positions: DynArray<Vec3f>,
    tangent_binormal_vertices: DynArray<TBVtx>,
    morpher: Morpher, // source: we'll export this (need to make a agnostic struct for it cause some versions have stuff that this one doesnt, like a AABB node tree similar to collision, but for triggering morphing dynamically for say car crash geometry displacements, it may be detailed in v1_381)
}

#[derive(..BffStruct)]
pub struct Vtx {
    uv_idx: u16,
    normal_idx: u16,
    color_idx: u16,
    vertices_idx: u16,
}

#[derive(..BffStruct)]
pub struct StripExt {
    vertices: DynArray<Vtx>,
}

#[derive(..BffStruct)]
pub struct SphereCol {
    col_sph: Sphere,
    flag: u32,
    name: Name,
} // source: we'll need an agnostic struct for this (to account for variations in different versions)

#[derive(..BffStruct)]
pub struct BoxCol {
    col_box: BffBox,
    flag: u32,
    name: Name,
} // source: we'll need an agnostic struct for this (to account for variations in different versions)

#[derive(..BffStruct)]
pub struct CylindreCol {
    col_cylindre: Cylindre,
    flag: u32,
    name: Name,
} // source: we'll need an agnostic struct for this (to account for variations in different versions)

#[derive(..BffStruct)]
pub struct FaceCol {
    first_vertex_id: i16,
    second_vertex_id: i16,
    third_vertex_id: i16,
    material_index: i16,
}

#[derive(..BffStruct)]
pub struct AABBCol {
    collision_faces: DynArray<FaceCol>,
    collision_aabb_nodes: DynArray<AABBNode>,
} // source: we will need to determine if we wanna export this, or if we would wanna lose the data for source export, and then rebuild (bake) on import

#[derive(..BffStruct)]
#[br(import(is_skinned: bool))]
pub enum OptimizedVertices {
    #[br(pre_assert(!is_skinned))]
    Unskinned(DynArray<OptimizedVertex>),
    #[br(pre_assert(is_skinned))]
    Skinned(DynArray<OptimizedSkinnedVertex>),
} // source: We'll export these when they are the main mesh vertices (for example GameCube) but on PC they are superseeded by vertex buffers for rendering (these ones are only used for collision, or left over)

#[derive(..BffStruct)]
#[br(import(link_header: &ObjectLinkHeaderV1_06_63_02PC))]
pub struct MeshBodyV1_06_63_0X {
    points: Points,
    uvs: DynArray<Vec2f>, // source: generally they will be empty when reading existing bigfiles, cause the data gets baked and they emptied it after that. For getting source uvs we'll have to extract them from the baked data. Idk if it's worth checking if these are not empty using them.
    normals: DynArray<Vec3f>, // source: same as uvs comment
    strips: DynArray<Strip>, // source: same as uvs comment
    #[br(if(link_header.flags & 2 != 0))]
    #[br(count = strips.len())]
    strip_override_material_indices: Option<Vec<u32>>, // source: same as uvs comment
    strip_exts: DynArray<StripExt>, // source: same as uvs comment
    material_names: DynArray<Name>, // source: we need to decide what to do. Materials are normally mesh dependent, so really we should pull the materials referenced in this list into mesh (like how we pull animframes to node). There are special edge cases, for example in the case of skinned meshes these materials are superseeded by a material list in Skel. But for an initial impl we wanna do only the NON-SKINNED mesh path. So source export for, Mesh, Material, Bitmap.
    drawing_start_distance: f32,
    drawing_cutoff_distance: f32,
    original_face_count: u32, // source: this is kinda repetitive, I'm pretty sure it's unused in the final baked data, so we can probably ignore
    original_vertex_count: u32, // source: same as above
    related_to_counts: [u32; 2], // source: same as above
    sphere_cols: DynArray<SphereCol>, // source: as I said, agnostic structs for these.
    box_cols: DynArray<BoxCol>, // source: as I said, agnostic structs for these.
    cylindre_cols: DynArray<CylindreCol>, // source: as I said, agnostic structs for these.
    aabb_col: AABBCol, // source: as I said, decide if we wanna have it in source, or if it will be baked for the final data
    #[br(args(link_header.flags & 2 != 0))]
    optimized_vertices: OptimizedVertices, // source: See what to do with these. Again the goal is to have separate lists of faces, vertices, vertex normals, tex coords, (colors but not for this version cause we dont have them), and whatever extra is needed.
}
