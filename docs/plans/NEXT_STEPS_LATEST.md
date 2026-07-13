# Non-Skinned Mesh Source Implementation Plan

## Summary

Implement `MeshSource` before doing more source-layer cleanup or macro work. The first complete target is a non-skinned `Mesh` from Asobo `1.06.63.xx` on both PC and GameCube, uncooked into one platform-independent representation.

The source representation must describe authored geometry and dependencies, not PC vertex buffers, D3D primitive info, GameCube display lists, collision AABB trees, or other cooked acceleration data. PC and GameCube keep separate adapters that decode their cooked forms into the same source structures.

The first complete Mesh path includes:

- Object-level metadata and version-independent Object flags.
- Positions, normals, optional colors, optional tangents, multiple UV channels, and triangle faces.
- Independent per-corner indices for each attribute.
- The paired cooked `MeshData` resource as an owned input/output of the Mesh source asset, never as a standalone source asset or named source dependency.
- Ordered material slots embedded as `MaterialSourcePart` values.
- Material texture slots referencing platform-neutral source texture artifacts produced from cooked Bitmap resources.
- Sphere, box, and cylinder collision primitives in semantic source forms.
- Morph targets represented semantically; support can follow the first static geometry milestone, but non-empty unsupported morph data must never be silently discarded.

Skinning and shadow-volume meshes are explicitly later. Skinned PC vertex layouts, PC `Volume`/`SkinVolume`, GameCube `GX_VTXFMT2`, and any Object carrying `ShadowVolume` must fail Mesh source conversion and fall back to rich/raw export until those paths are implemented deliberately.

Manifestless bigfile construction is also deliberately later. First stabilize source cooking so a complete source/rich/binary project can produce a deterministic target-specific cooked resource collection. Then follow `MANIFESTLESS_BIGFILE_BUILD_PLAN.md`; do not mix block ordering or manifest policy into the source data model.

## Evidence and Design Constraints

### Source mesh format

The supplied engine source parser confirms that source faces do not use a single unified vertex index. Each face corner has separate indices for:

- Position.
- Normal.
- Color.
- UV channel 0.
- UV channel 1.

The old text format only consumes UV channel 0 in that engine version, but it still parses two UV indices. The new source model should therefore support any number of UV channels instead of hard-coding `uv` and `lightmap_uv` fields.

### Existing Blender adapter

`ZounaBlender/zouna/generic/mesh.py` already has the correct broad topology:

- Separate position, normal, UV, and lightmap-UV arrays.
- A face contains corners with independent attribute indices.
- Materials are indexed per face.

Its `v1_06_63_02_pc/mesh.py` adapter is useful as a cooking reference, but its current uncook path makes simplifying assumptions that the Rust source layer must not copy:

- It assumes one vertex buffer and one index buffer.
- It assigns the same cooked vertex index to position, normal, and UV arrays.
- It assumes primitive-info order is material order.
- It ignores `start_vertex`, vertex ranges, and nonzero vertex-buffer indices.
- It does not independently intern source attribute streams.

Coordinate-system conversion, UV V-axis flipping, and winding reversal are Blender concerns. `MeshSource` must remain in native engine coordinates and native engine winding.

### Cooked platform boundaries

PC geometry is reconstructed from:

- `MeshBuffers`.
- `PrimitiveInfo` ranges.
- Vertex-buffer layouts.
- Index-buffer ranges.

GameCube geometry is reconstructed from:

- `OptimizedVertices::Unskinned`.
- `optimized_normals`.
- `optimized_texcoords`.
- `DisplayList` commands using `GX_VTXFMT1`.

Neither representation belongs in source JSON. They are inputs to version/platform-specific adapters only.

## Phase 0: Binary Corpus and Format Analysis

Complete a focused data-analysis phase before freezing the Mesh source structs or implementing uncook adapters. The purpose is not to understand every unknown field. It is to resolve the questions that change source ownership, topology, indexing, material assignment, or the ability to regenerate cooked outputs.

Keep observations separate from conclusions:

- **Observed:** a value or relationship seen in one or more files.
- **Supported invariant:** true across the deliberately varied corpus so far, but still open to counterexamples.
- **Proven by code/format:** established by engine source, decompilation, or a generation algorithm.
- **Unresolved:** must remain guarded by fallback or preservation.

Do not turn a one-file observation into a source-schema rule.

### Build a representative binary corpus

Use binary extraction so ImHex sees the exact serialized resource, including its BFF resource header:

```powershell
bff-cli x <bigfile> <output-directory> -e binary
```

The resulting resource files can be opened directly with the matching ImHex pattern. Rich JSON is useful as a second decoded view, but it is not binary ground truth and must not be used to infer fields that the Rust parser already interpreted incorrectly.

Build a local corpus containing:

- Several normal, non-skinned PC Meshes.
- Several normal, non-skinned GameCube Meshes.
- Equivalent PC and GameCube assets when matching resource names/content exist.
- Single-material and multi-material Meshes.
- One and several primitive/display-list groups.
- Empty and non-empty primitive collision arrays.
- Empty and non-empty AABB collision arrays.
- Meshes with populated and empty legacy source-like arrays.
- Meshes with morph data, skinned flags/layouts, and shadow-volume flags/layouts for classification, even though the first implementation rejects them.
- Every observed vertex/index-buffer count, including uncommon multiple-buffer cases.
- Mesh/paired-MeshData/Material/Bitmap resource groups, not isolated Mesh files only.

Do not commit copyrighted game binaries. Record a local corpus manifest with stable identifiers so the same samples can be reopened:

- Bigfile path or content hash.
- Engine version and platform.
- Resource raw name and resolved name when available.
- Class type.
- Extracted file path or file hash.
- Why the sample was selected.

### ImHex pattern strategy

ImHex is the primary executable validator for this phase, not only an interactive viewer. `ImZouna/scripts/validate.py` already runs the selected patterns recursively and in parallel against every matching extracted resource. Use that path to test each structural assertion over the whole corpus after every pattern change.

Start from the existing Rat ImHex pattern:

```text
C:\Users\Sabe\Downloads\_ZounaReversingStuff\_General\ImZouna\patterns\rat\Mesh_Z.hexpat
```

Also use the existing 010 Editor templates under:

```text
C:\Users\Sabe\Downloads\_ZounaReversingStuff\_General\zouna-templates-docs\templates\Rat
```

The highest-priority templates to port or compare are:

- `PC/MeshPc_Z.bt`.
- `GC/MeshGC_Z.bt`.
- `PC/MeshDataPc_Z.bt`.
- `PC/Material_Z.bt`.
- `PC/Bitmap_Z.bt`.

The PS2/Xbox Mesh and Skin templates can be consulted later for cross-platform semantic clues, but they are not implementation targets for the initial PC/GC phase. The 010 templates contain useful field layouts, diagnostics, and years of observations, but they are evidence rather than authority. Port each field with its endianness and offset, compare it with the current Rust structs and engine/decompilation evidence, and do not preserve template bugs or guesses merely for compatibility.

The existing Rat `Mesh_Z.hexpat` already dispatches PC versus GameCube and parses the important Mesh structures. Keep the production pattern strict. For exploratory work, add sibling analysis patterns instead of repeatedly weakening production assertions:

- `patterns/rat/Mesh_Z_Analysis.hexpat`: extend the current ImHex pattern with diagnostics ported from `MeshPc_Z.bt` and `MeshGC_Z.bt`, plus derived counts, candidate index resolutions, material mapping, AABB/render comparison helpers, and GC command diagnostics.
- `patterns/rat/MeshData_Z.hexpat`: port the actual `1.06.63.xx` ObjectDatas/MeshVolume layout from `MeshDataPc_Z.bt`, then replace opaque names only when newer evidence supports the semantics.
- `patterns/rat/Material_Z.hexpat`: port `Material_Z.bt`, preserving ordered texture links, semantic values, flags, and unknown parameter observations needed by `MaterialSourcePart`.
- `patterns/rat/Bitmap_Z.hexpat`: port the common/PC fields from `Bitmap_Z.bt`, then add the GameCube payload interpretation from binaries and platform code because there is no corresponding Rat GC Bitmap 010 template in this directory.

The existing `patterns/fuel/MeshData_Z.hexpat`, `Material_Z.hexpat`, and `Bitmap_Z.hexpat` may provide naming or semantic clues, but they describe a different engine version. Do not apply their byte layouts to `1.06.63.xx` resources without independently validating every field.

Port the 010 templates in this order:

1. Reproduce the byte layout and full-input consumption without adding new semantics.
2. Port useful `Printf`/analysis loops as visible derived values, formatters, or assertions suitable for ImHex output.
3. Run the pattern over all matching extracted resources with `validate.py`.
4. Investigate every failure as either a bad pattern assumption, a real format variant, corrupt input, or wrong version/platform classification.
5. Only after the layout validates broadly, add candidate semantic relationships in the `_Analysis` pattern.
6. Promote confirmed structure back into the strict production pattern while leaving unresolved hypotheses in the analysis pattern.

For each analysis pattern:

1. Keep raw fields visible before adding transformed views.
2. Expose byte offsets, array counts, strides, ranges, and selected enum/flag values.
3. Add derived counters and booleans for candidate relationships instead of immediately asserting one interpretation.
4. Assert only structural safety first: array bounds, valid addresses, element sizes, alignment, and full input consumption.
5. Promote a candidate relationship to a strict assertion only after it holds across the corpus and agrees with engine code where available.
6. Preserve PC little-endian and GameCube big-endian dispatch explicitly.
7. Keep analysis-only fields out of Rust source structs and source JSON.

Run strict patterns across the corpus with the existing script:

```powershell
cd C:\Users\Sabe\Downloads\_ZounaReversingStuff\_General\ImZouna
python scripts/validate.py -C <EXTRACTED_DIRECTORY> -j <THREAD_COUNT> --game rat --tests Mesh_Z MeshData_Z Material_Z Bitmap_Z
```

Make one small required script improvement as part of Phase 0: add an optional analysis-pattern suffix or explicit pattern override to `validate.py`, allowing `*.Mesh_Z` files to run against `Mesh_Z_Analysis.hexpat` without replacing the production pattern.

`jsonify.py` is not needed for this phase. Most hypotheses should become analysis-pattern assertions and be tested by `validate.py` directly. When aggregate values or cross-resource joins are useful, have analysis patterns emit compact, tagged records with `std::print` and add an optional report-output mode to `validate.py` that retains those records from successful runs. A thin correlation step may join the emitted records by resource name for Mesh-to-MeshData-to-Material-to-Bitmap questions; it must not implement a second binary parser. Do not add analysis-only representations to the BFF public API.

### Analysis questions

#### 1. Mesh and MeshData ownership

- Verify that every Mesh `data_name` resolves to exactly one `ClassType::MeshData` in the same BigFile.
- Verify the reverse relation: determine whether any MeshData is shared, orphaned, or referenced by more than one Mesh.
- Record missing, null, duplicate, and cross-class `data_name` cases.
- Compare the pairing rule on both PC and GameCube.

Expected architectural result: if the observed relation remains 1:1, MeshData stays an unnamed owned cooked part of `MeshSource`, never a standalone source asset.

#### 2. ObjectDatas flag and color derivation

- Record the raw ObjectDatas flag value and color for every paired MeshData.
- Correlate them with Mesh Object flags, visibility, skin/morph/shadow classification, material/color values, and any matching original source file.
- Look for constants, direct copies, simple masks, and deterministic defaults.
- Use engine construction/cooking code when available to distinguish authored values from runtime defaults.

Expected architectural result: derived values are omitted from source JSON and rebuilt by the MeshData cooker. Add a source field only if a value is genuinely authored and cannot be recovered from another Mesh source field.

#### 3. PC primitive and index semantics

- Record vertex-buffer count, index-buffer count, primitive count, layout, stride, selected buffer indices, ranges, `start_vertex`, and index-buffer start.
- Test candidate final-index formulas explicitly: raw index alone, raw index plus `start_vertex`, raw index plus vertex-range begin, and combinations of those offsets.
- Reject a candidate when it escapes the selected primitive range or produces topology inconsistent with bounds/AABB/render evidence.
- Determine whether `primitive_info_indices` is identity, a permutation, a material-order map, or used only in certain files.
- Determine the exact primitive-to-material-slot rule.
- Verify packed tangent/normal component order and signed normalization with recognizable geometry.
- Search specifically for valid multi-buffer files instead of designing around the common one-buffer case.

Required decision before the PC adapter: one exact index-resolution formula and one exact material-assignment rule, with unsupported forms identified explicitly.

#### 4. GameCube display-list semantics

- Split `primitive_and_vtx_fmt` into the GX primitive opcode and low vertex-format bits, then record every observed combination.
- Verify the endianness and component grouping of optimized positions, normals, and UVs.
- Validate all position/normal/UV references against their arrays.
- Determine whether `byte_count` contains one command or a stream of commands.
- Separate meaningful trailing command bytes from zero/alignment padding; do not call all remaining bytes padding without evidence.
- Validate triangle-list, strip, and fan winding using known geometry.
- Determine the display-list-to-material-slot rule and the relationship between shared `material_names` and GC `material_link_names`.

Required decision before the GC adapter: supported command stream grammar, exact corner-index interpretation, winding rules, and authoritative material ordering.

#### 5. Render geometry versus AABB collision data

Run the detailed AABB experiment described in the Collision Model section across both platforms. Compare resolved triangle multisets, winding, material/surface values, quantization, and count differences. Include counterexample-seeking samples rather than only files whose counts already match.

Required decision: classify AABB faces as a derived view of render geometry, render geometry plus semantic per-face data, or distinct authored proxy geometry. Until classified, non-empty AABB data remains unsupported by source conversion.

#### 6. MeshVolume derivation

- Implement the Rat MeshData pattern without assigning semantics to the four opaque arrays prematurely.
- View each element simultaneously as raw bytes, aligned words, candidate indices/offsets, and floats where useful.
- Use known element sizes and engine pointer-remapping code to identify vertex, face, and edge arrays.
- Convert candidate face and edge references to indices and validate every range.
- Test whether volume faces match normal render triangles, AABB triangles, or a distinct closed proxy.
- Derive undirected edge adjacency from candidate faces and compare it with the stored edge graph.
- Check manifoldness and winding, and compare equivalent PC/GC assets where possible.

Required decision for the later volume cooker: reuse `MeshGeometrySource` or add one optional semantic `ShadowVolumeSourceMesh` inside `MeshSource`. Never expose raw pointer graphs or opaque runtime arrays as source data.

#### 7. Legacy arrays, morphs, and primitive collisions

- Measure how often shared positions/normals/texcoords/strips/strip extensions are populated.
- When both legacy arrays and platform render data exist, compare topology and attributes rather than assuming either is authoritative.
- Record every non-empty morph layout and map `self_idx`, displacements, normals, and target names back to source points.
- Validate sphere/box/cylinder transforms, dimensions, flags, and names against matching source files or engine constructors.

Required decision: define when legacy arrays are fallback input, validation data, or a distinct semantic source contribution. Keep morph and collision fields guarded until their mappings are established.

#### 8. Materials and textures

- Confirm Mesh material slot order against primitive/display-list assignment and both GC material-name arrays.
- Record Material flags, values, texture slots, null/sentinel links, and the incoming owners of every Material. The current hypothesis is that a Material resource belongs to one source asset rather than being shared by multiple Meshes; actively search for counterexamples.
- Record Bitmap/texture fan-in separately. Textures are expected to be shared by Materials belonging to different source assets.
- For Bitmap payloads, record platform, dimensions, mip count, format/palette/transparency metadata, flags, `precalculated_size`, payload offset/size, and magic bytes.
- Classify PC payloads into embedded DDS and every observed headerless/raw pixel format. Correlate raw payload size and layout with `format`, `format_copy`, palette/transparency fields, mip count, dimensions, and flags instead of treating every non-DDS payload as unknown binary.
- Confirm PC DDS container behavior, derive raw PC row/mip/palette layouts, and classify GameCube swizzle/palette formats.
- Decode matching PC/GC textures to a neutral pixel buffer and compare dimensions, alpha, and pixel hashes where lossy compression permits.

Required decision: confirm Material ownership cardinality, identify which Material/texture values are authored source semantics, and distinguish those from target cooking choices. This determines the reusable `MaterialSourcePart` shape, the shared `TextureSourceRef` shape, and which owning source asset represents each cooked Material.

#### 9. Cross-platform neutral-form check

For equivalent PC and GameCube assets, compare the candidate neutral results rather than the cooked arrays:

- Triangle count and topology.
- Position values within known platform quantization.
- Independent normal and UV streams.
- Material slot order and texture identity.
- Primitive collision shapes.
- Mesh/MeshData ownership.

Differences caused solely by packing, quantization, index expansion, display-list grouping, or vertex layout must disappear in `MeshSource`. A persistent semantic difference must be represented or explicitly rejected.

### Analysis outputs and phase gate

Produce these local analysis artifacts before Phase 1:

- A corpus manifest.
- Compact per-resource/per-primitive report records only for investigations that need aggregate values or cross-resource joins.
- A short hypothesis ledger with evidence and status.
- Updated strict patterns for confirmed layouts plus separate analysis patterns for unresolved fields.
- A decision record for each required decision above.
- A list of resource identifiers that exercise every supported and rejected case for later regression testing.

Phase 0 is complete enough to begin source structs when all of these are true:

1. Mesh-to-MeshData pairing is validated or exceptions have an explicit fallback rule.
2. PC index resolution and material assignment are unambiguous for the initial `NoBlend` path.
3. GC command parsing, corner references, winding, and material assignment are unambiguous for `GX_VTXFMT1`.
4. Non-empty AABB data has either a proven source mapping or remains a deliberate rejection gate.
5. ObjectDatas flag/color handling has a derivation rule or an explicit unresolved policy.
6. Material and Bitmap ownership/order are known, and embedded PC DDS, raw PC payload families, and GameCube swizzled payloads are classified well enough to define one neutral source-image boundary without losing required metadata.
7. Unsupported skinned, morph, shadow-volume, and unknown-command cases are detectable before any resource is marked represented.

### Reusable pattern-analysis prompt

Use this as a starting prompt for a later focused investigation:

```text
Analyze the Asobo 1.06.63.xx cooked binary question: <QUESTION>.

Use the binary corpus at <CORPUS_PATH> and inspect the existing patterns under
ImZouna/patterns/rat, especially Mesh_Z.hexpat. Also inspect the matching Rust
cooked structs in bff/src/class and the relevant Rat 010 templates under
zouna-templates-docs/templates/Rat. Port useful 010 layout and diagnostics into
ImHex, but verify them rather than treating either the 010 or Fuel templates as
authoritative for this binary.

Create or update a sibling *_Analysis.hexpat pattern. Keep raw fields visible,
preserve PC/GC endianness, expose offsets/counts/ranges, and add named derived
candidate values and booleans. Assert structural bounds immediately, but do not
turn the hypothesis into a strict assertion until it survives varied samples.
Do not change source structs or public APIs during this analysis.

Run the pattern recursively over the corpus through ImZouna/scripts/validate.py,
using parallel jobs. For relationships spanning resources, emit compact tagged
records from the ImHex pattern and correlate those ImHex-produced records;
do not write a second binary parser. Test positive examples and actively search
for counterexamples. Return: files changed, corpus samples used, batch pass/fail
counts, observations, failed candidates, the supported invariant, unresolved
cases, and the exact consequence for MeshSource/uncook/cook design.
```

## Source Data Model

Add the source-domain types before writing either platform adapter. Keep them ordinary Rust structs with `Serialize`, `Deserialize`, and `JsonSchema`; do not introduce source macros yet.

### Files

- `bff/src/source/object.rs`: reusable Object metadata and Object flags.
- `bff/src/source/collision.rs`: reusable semantic collision structures and collision flags.
- `bff/src/source/classes/mesh.rs`: `MeshSource`, geometry, faces, corners, morph targets, any source-semantic contribution recovered from cooked MeshData, and Mesh assembly.
- `bff/src/source/classes/material.rs`: `MaterialSourcePart` and material-specific flags.
- `bff/src/source/texture.rs`: platform-neutral source texture references and artifact metadata.

Material remains a source part because it may later be owned by source assets other than Mesh. Bitmap is a cooked resource family, but the corresponding source concept is a texture artifact, not necessarily a serialized `BitmapSourcePart` or standalone source asset.

### Mesh ownership shape

Use the same assembly pattern as Node:

```rust
pub struct MeshSource {
    pub mesh: MeshSourcePart,
    pub materials: Vec<MaterialSourcePart>,
}
```

`MeshSourcePart` contains the Mesh's authored semantic data. `MeshSource` assembles the ordered dependencies that belong in the source asset.

Do not add an empty serialized `mesh_data` object merely because the cooked format has a `MeshData` class. The source schema follows authored concepts, not cooked class boundaries. If inspection later proves that MeshData contains a source-authored value which does not naturally belong in `MeshSourcePart`, define an unnamed `MeshDataSourcePart` in this same module and embed it here. Until then, MeshData participates in conversion and ownership without forcing a public JSON field.

The cooked adapter should return build metadata separately:

```rust
pub struct MeshSourcePartBuild {
    pub mesh: SourcePartBuild<MeshSourcePart>,
    pub mesh_data_name: Name,
    pub material_names: Vec<Name>,
}
```

`mesh_data_name` is required build-only metadata read from the cooked Object header. It locates the exact 1:1 cooked MeshData resource during uncooking, but it is not serialized and is not a source-level reference. Do not put dependency names in serialized `MeshSourcePart` merely to help assembly.

### Object source metadata

Define a reusable `ObjectSourcePart` containing source-relevant data from the cooked Object link header:

- Friendly/link name.
- Local bounding sphere.
- Local bounding box in a semantic transform/extent form.
- Fade-out distance.
- `ObjectFlags`.

Do not put cooked `data_name` in `ObjectSourcePart`. The Mesh cooked adapter returns it as `MeshSourcePartBuild.mesh_data_name`, and Mesh assembly uses it exactly once to load the owned MeshData. `ObjectType::Mesh` and link-header dependency arrays are cooked metadata and are not source fields.

Bounds are initially retained because they are present in the source-relevant Object data. Cooking may later offer an explicit recompute-bounds policy so edited geometry cannot accidentally keep stale bounds.

### Object flags

Add source-level Object flags using the existing flag architecture:

```rust
pub enum ObjectFlag {
    MaxBoundingSphere,
    Skinned,
    NoSeadDisplay,
    NoSeadCollide,
    Unknown0x10,
    Unknown0x20,
    NoDisplay,
    ShadowVolume,
    Unknown0x100,
    Unknown0x200,
    Unknown0x400,
    Unknown0x800,
    Unknown0x1000,
    Unknown0x2000,
    Unknown0x4000,
    Unknown0x8000,
    Unknown0x10000,
    Unknown0x20000,
    Unknown0x40000,
    Unknown0x80000,
    Active,
    SplineTrackLoop,
    Unknown0x400000,
    Unknown0x800000,
    Unknown0x1000000,
    OmniUnknownSphereVsBox1,
    OmniUnknown0x4000000,
    OmniSpotlight,
    OmniUnknownSphereVsBox2,
    Unknown0x20000000,
    Unknown0x40000000,
    Unknown0x80000000,
}

pub struct ObjectFlags {
    pub flags: Vec<ObjectFlag>,
}
```

Use `FlagMapping<ObjectFlag>` to decode and eventually encode each cooked version. The source enum contains semantic names with no associated raw values. Unknown meanings still get stable source variants so their presence survives uncooking; the hexadecimal suffix identifies the known `1.06.63.xx` flag until a semantic name is discovered.

The complete `1.06.63.xx` mapping is:

| Source flag | Raw bit | Current meaning |
| --- | ---: | --- |
| `MaxBoundingSphere` | `0x00000001` | Bounding sphere is already at maximum radius and should not be recalculated. |
| `Skinned` | `0x00000002` | Skinned geometry. |
| `NoSeadDisplay` | `0x00000004` | Do not use SEADs for display. |
| `NoSeadCollide` | `0x00000008` | Do not use SEADs for collision. |
| `Unknown0x10` | `0x00000010` | Unknown; observed on Mesh. |
| `Unknown0x20` | `0x00000020` | Unknown; observed on Mesh. |
| `NoDisplay` | `0x00000040` | Do not display the Object. |
| `ShadowVolume` | `0x00000080` | Shadow-volume Mesh. |
| `Unknown0x100` | `0x00000100` | Used by `MeshGC_Z::DrawWithMaterials` for immediate drawing versus queued draw calls when drawing through Skin. |
| `Unknown0x200` | `0x00000200` | Unknown. |
| `Unknown0x400` | `0x00000400` | Unknown. |
| `Unknown0x800` | `0x00000800` | Unknown. |
| `Unknown0x1000` | `0x00001000` | Unknown. |
| `Unknown0x2000` | `0x00002000` | Unknown. |
| `Unknown0x4000` | `0x00004000` | Unknown. |
| `Unknown0x8000` | `0x00008000` | Unknown. |
| `Unknown0x10000` | `0x00010000` | Unknown. |
| `Unknown0x20000` | `0x00020000` | Unknown. |
| `Unknown0x40000` | `0x00040000` | Unknown. |
| `Unknown0x80000` | `0x00080000` | Unknown. |
| `Active` | `0x00100000` | Object is active; used by Light and Omni. |
| `SplineTrackLoop` | `0x00200000` | Spline track loops. The exact wider semantics are still uncertain. |
| `Unknown0x400000` | `0x00400000` | Unknown. |
| `Unknown0x800000` | `0x00800000` | Unknown. |
| `Unknown0x1000000` | `0x01000000` | Unknown. |
| `OmniUnknownSphereVsBox1` | `0x02000000` | Omni-specific, apparently involved in sphere-versus-box behavior. |
| `OmniUnknown0x4000000` | `0x04000000` | Omni-specific unknown behavior. |
| `OmniSpotlight` | `0x08000000` | Omni is a spotlight. |
| `OmniUnknownSphereVsBox2` | `0x10000000` | Omni-specific, apparently involved in sphere-versus-box behavior. |
| `Unknown0x20000000` | `0x20000000` | Unknown. |
| `Unknown0x40000000` | `0x40000000` | Unknown. |
| `Unknown0x80000000` | `0x80000000` | Unknown. |

`FL_OBJECT_NONE` is zero and does not need a source enum variant. An empty `ObjectFlags.flags` vector represents it.

Keep the `1.06.63.xx` table in its cooked adapter as a `const` slice of `FlagMapping<ObjectFlag>`. Later versions get separate mapping tables. A later version may map a different raw bit to the same semantic source variant; it must not inherit the `1.06.63.xx` bit positions implicitly. Likewise, an `Unknown0x...` source variant should only map into another version when its correspondence has been established.

This table covers all 32 bits for `1.06.63.xx`, so decoding that version preserves every flag. For any other version with an unmapped raw bit, fail source conversion and leave the Mesh eligible for rich/raw fallback rather than dropping it.

`Skinned` and `ShadowVolume` are initial candidate gates: the first Mesh builder rejects either flag. They lead to separate Skin and volume pipelines later.

### Geometry

Use a source topology close to the old engine source format:

```rust
pub struct MeshGeometrySource {
    pub positions: Vec<Vec3f>,
    pub normals: Vec<Vec3f>,
    pub colors: Vec<RGBA>,
    pub tangents: Vec<Vec4f>,
    pub uv_channels: Vec<Vec<Vec2f>>,
    pub faces: Vec<MeshFaceSource>,
}

pub struct MeshFaceSource {
    pub corners: [MeshCornerSource; 3],
    pub material_index: u32,
}

pub struct MeshCornerSource {
    pub position_index: u32,
    pub normal_index: Option<u32>,
    pub color_index: Option<u32>,
    pub tangent_index: Option<u32>,
    pub uv_indices: Vec<Option<u32>>,
}
```

Rules:

- Source faces are triangles for the first implementation.
- Attribute arrays are independent; do not force a unified vertex index.
- UV channel 0 is the regular UV set and channel 1 is the lightmap UV set for `LayoutNoBlend` and `GX_VTXFMT1`.
- Keep UV channels distinct even when their values happen to match.
- Tangents are optional source data. PC can preserve them; GC may emit none. A later cooker may either use supplied tangents or regenerate them.
- Colors are part of the source model even though this particular cooked Mesh path may emit an empty array.
- All indices use fixed-width integer types such as `u32`, not `usize`, so JSON schemas are stable across languages.
- Validate every index before producing a successful source asset.

### Deterministic attribute interning

Reverse cooked vertex expansion by interning each attribute stream independently:

- Position key: exact component bit patterns.
- Normal key: decoded semantic components, using exact decoded bit patterns.
- Color key: exact component bit patterns.
- Tangent key: exact decoded component bit patterns.
- UV key: exact component bit patterns per channel.

Use first-seen order so output is deterministic. Do not use epsilon-based welding in the uncooker.

Keep a cooked-vertex-to-source-position map as build-only data on `MeshSource` with serde/schema skipping. This is not a separate `MeshUncookInfo` type. `SkinSource` can later read the built `MeshSource` and use that map to reconcile weights across vertices duplicated by UV seams or material boundaries.

### Drawing distances and counts

Keep semantic drawing distances in `MeshSourcePart`.

Do not export fields annotated as redundant cooked counts:

- `original_face_count`.
- `original_vertex_count`.
- `related_to_counts`.

Cooking derives those values from source geometry.

### Morph targets

Define the semantic shape now, even if static geometry is implemented first:

```rust
pub struct MorphTargetSource {
    pub name: Name,
    pub channel: Option<u32>,
    pub affected_vertices: Vec<MorphVertexSource>,
}

pub struct MorphVertexSource {
    pub position_index: u32,
    pub displacement: Vec3f,
    pub normal: Option<Vec3f>,
}
```

Map only authored morph meaning:

- Target identity.
- Affected source point.
- Displacement.
- Optional replacement normal.
- Optional channel where a version contains it.

Do not export AABB morph triggers, maps, displacement-buffer indices, or other cooked lookup structures. Those are rebuilt when cooking.

For the first static milestone, a Mesh with non-empty morph data must return an unsupported-source error rather than exporting a Mesh with lost morphs. Add the `1.06.63.xx` morph conversion immediately after static PC/GC geometry.

## Collision Model

### Primitive collisions

Create shared semantic source structs rather than exposing `BffBox`, `Sphere`, `Cylindre`, or version-specific wrappers directly:

- `SphereCollisionSource`: center and radius.
- `BoxCollisionSource`: center, orientation, and half-extents.
- `CylinderCollisionSource`: origin, normalized direction, length, and radius.

Wrap each shape with common metadata:

- Collision name/category.
- Source-level collision flags.

Keep separate `spheres`, `boxes`, and `cylinders` vectors. A tagged shape enum is unnecessary and gives generated C++/Python bindings a more awkward representation.

Collision flags use the same `FlagMapping` approach as Node and Object flags. The semantic source flag enum is shared; raw values remain in version adapters.

### AABB collision data investigation

Do not add `CollisionMeshSource` in the initial design. There is evidence that `AABBCol.face_cols` is another cooked view of the same faces used for drawing: one inspected GameCube Mesh has 220 render faces and 220 collision faces. Equal counts are suggestive but are not enough to establish identity.

Use the ImHex pattern at:

```text
C:\Users\Sabe\Downloads\_ZounaReversingStuff\_General\ImZouna\patterns\rat\Mesh_Z.hexpat
```

to test the relationship before defining source data. Make a temporary analysis branch of the pattern and inspect several non-skinned, non-shadow-volume Meshes on both PC and GameCube:

1. Count render triangles from PC primitive/index ranges or GameCube display-list commands.
2. Compare that count with `original_face_count` and `aabb_col.face_cols.size`.
3. Resolve every `FaceCol_Z` index through `optimized_vertices` to obtain three semantic positions.
4. Resolve every render triangle through the PC vertex buffer or GameCube display-list position references.
5. Canonicalize each triangle by its three position values and compare the two triangle multisets without winding first.
6. Compare directed winding separately after membership matches.
7. Compare `FaceCol_Z.material_idx` with the render face's resolved material slot.
8. Check whether optimized positions are exactly the render positions, quantized copies, a deduplicated subset, or a distinct proxy.

The ImHex pattern can help by adding temporary derived face counters, assertions for count equality, and formatted/bookmarked position triples next to both representations. If full multiset comparison is awkward in the pattern language, export the resolved triples from ImHex and compare them with a small external script; do not add that analysis-only representation to BFF source JSON.

Test more than the known 220-face example. Include Meshes with multiple materials and primitive groups, and deliberately inspect files where the counts differ. Keep skinned and shadow-volume cases out of the initial conclusion because they may use different optimized-vertex rules.

Choose the source model only after the result is known:

- If the AABB faces are the same render faces, omit `FaceCol_Z`, optimized collision vertices, and AABB nodes from source. Rebuild all of them from `MeshGeometrySource` during cooking.
- If they are the same geometry but carry additional material/surface meaning, add that semantic information to existing source faces rather than duplicating the mesh.
- If they are genuinely distinct authored geometry, revisit a narrowly named optional proxy representation then. Do not preemptively add `CollisionMeshSource`.

Until this is proven, AABB data is outside the initial source-consumption claim. A Mesh with non-empty unexplained AABB data must fail source conversion; it cannot be marked represented while fields inside that same cooked Mesh are being dropped.

## MeshData ownership and shadow-volume generation

The cooked Object header's `data_name` identifies the exact 1:1 MeshData paired with the Mesh. Read that field directly in the cooked Mesh adapter. Never infer MeshData from the Mesh name, class-wide dependency searches, or graph ordering.

MeshData is an owned cooked part of `MeshSource`, but it is not a top-level source asset and does not produce its own source document. The original asset pipeline had no `.TMESHDATA` source file; cooking generated MeshData from the Mesh source asset. Preserve that relationship in both directions:

- Uncooking one Mesh requires its paired MeshData and marks both cooked resources represented only after the whole Mesh source build succeeds.
- Cooking one `MeshSource` emits both the target Mesh resource and its paired MeshData resource in the same cook transaction.
- `data_name` remains internal lookup/output metadata. It never appears as a reference in source JSON.
- Do not add `MeshData` to `SourceAssetData`, `SOURCE_ASSET_BUILDERS`, or the future source-asset registry.

Here, "part" describes conversion and ownership, not a mandatory one-to-one JSON object. A MeshData conversion may contribute validation and represented-resource ownership while contributing no serialized fields. Do not serialize an empty `{}` to prove that the cooked part was visited.

Use the same principle for other generated data families later: a cooked `FooData` resource belongs to `FooSource`, is read through the exact cooked 1:1 link, and is recreated when cooking `FooSource`. It only produces source fields when it contains authored semantics that are not derivable from the owning source asset.

The current `MeshDataV1_06_63_02PC` body contains `ObjectDatas { unknown, color }` and `MeshVolume`. The field currently named `unknown` is believed to be a flag and, like the color, is probably derived during cooking. This does not mean either value should be copied mechanically into a serialized `MeshDataSourcePart`. Confirm their derivation first:

1. Compare `ObjectDatas.color` with Mesh/Object/material source values and known original source files.
2. Identify the semantics and derivation rule for the currently unknown `ObjectDatas` flag.
3. If a value is a default, cache, duplicate, or derivation of another source field, regenerate it while cooking.
4. If a value is genuinely authored and has a clear domain meaning, place it directly in `MeshSourcePart` or in an embedded unnamed `MeshDataSourcePart` in `source/classes/mesh.rs`.
5. If an opaque value must be retained before it is understood, preserve only that scoped fragment in the owning Mesh source asset; do not create a standalone MeshData source file or mirror the entire cooked body.

It is valid for the initial `MeshSource` schema to have no serialized MeshData-specific field. MeshData still participates in assembly, validation, represented-resource accounting, and future cooking.

`MeshDataGC_Z` / `MeshData_Z` contains `MeshVolume_Z`, which is a cooked stencil shadow-volume proxy and acceleration structure, not normal render geometry. Its runtime representation contains:

- Position data.
- Faces whose stored vertex pointers must be interpreted as indices into the position array.
- Edges whose face and vertex pointers must be interpreted as adjacency indices.
- Per-face plane/facing data used at runtime.
- A secondary unknown `Vec4f` array.

Raw pointers, face adjacency pointers, edge adjacency, runtime facing values, and the secondary cooked array are not source data.

The minimum neutral input needed to rebuild a volume is a closed triangle mesh:

```rust
pub struct ShadowVolumeSourceMesh {
    pub positions: Vec<Vec3f>,
    pub indices: Vec<u16>,
}
```

Even this type should not be added yet. First determine whether the normal render positions/faces already provide that closed mesh. Edges are always derived from triangle adjacency using the undirected key `(min(v0, v1), max(v0, v1))`; source-authored edge arrays are unnecessary.

If the render mesh is the shadow source mesh, cooking can derive both targets from `MeshGeometrySource`:

- GameCube builds `FaceVolume_Z` and `EdgeVolume_Z`, then performs CPU silhouette extrusion.
- PC builds its special Volume vertex/index buffers for shader-driven extrusion.

If later evidence shows a distinct authored shadow proxy, add the optional `ShadowVolumeSourceMesh` at that point. It is a shadow source, not generic collision geometry.

Initial scope therefore excludes all shadow-volume paths:

- Reject Object flags containing `ShadowVolume`.
- Reject PC `Volume` and `SkinVolume` vertex layouts.
- Do not assume GameCube optimized vertices alone are sufficient for a volume.
- Treat the existing `MeshVolume_Z` arrays as generated cooked output and do not expose them in source JSON.

This shadow-volume exclusion does not make MeshData independently eligible for fallback after a successful normal Mesh source build. The owning `MeshSource` represents the paired MeshData resource as a generated output. If later evidence proves that `MeshVolume_Z` encodes a distinct authored proxy unavailable from normal Mesh geometry, add that semantic proxy to `MeshSource`; it still does not become a standalone MeshData source asset.

## Material and Texture Ownership

### Ownership rule

For the initial architecture, each cooked Material is an owned part embedded directly in one source asset, like AnimFrame and UserDefine parts embedded in `NodeSource`. A Mesh therefore contains its ordered `Vec<MaterialSourcePart>` and represents those cooked Material resources atomically when the Mesh source build succeeds.

`MaterialSourcePart` remains a reusable source type because future source assets other than Mesh may own Materials too. Reusing the Rust type does not make Material a standalone source asset and does not imply that the same cooked Material instance is embedded in several source documents. If Phase 0 finds a Material referenced by multiple candidate owners, do not duplicate it silently; record the exception and define an explicit ownership/reference rule before source conversion accepts it.

Textures have different ownership. A Material embeds texture slots, but each slot references a Bitmap-backed source texture by stable resource identity. Multiple Materials, including Materials owned by different source assets, may reference the same texture and source image artifact.

### Ordered material slots

Read material names directly from the cooked Mesh variant in their original order. Do not use `DependencyIndex` to reconstruct material order because referenced-name collection and graph edges do not define slot order.

Each face's `material_index` refers to this ordered source vector.

GameCube has both shared Mesh material fields and `material_link_names`. Before finalizing its adapter, inspect real rich exports and establish which list is authoritative. Validate both when both are populated; do not silently choose one when they disagree.

### Material source part

`MaterialSourcePart` should preserve semantic material values:

- Resource and friendly names.
- Diffuse color and opacity.
- Emission.
- Specular color and power.
- UV translation, scale, and rotation.
- Semantic material collision/render/object flags.
- Named parameter values that are understood, with explicitly named unknown parameters only when necessary.
- Explicit optional texture slots: diffuse, environment, normal, and specular.

Do not export `uv_transform_matrix` separately when it is derived from translation, scale, and rotation. Do not export sentinel values such as `cdcdcdcd`.

Each texture slot references a potentially shared source texture artifact and retains the cooked Bitmap resource identity needed to rebuild dependencies. It is not merely an unordered dependency name.

### Cooked Bitmap versus source texture

Do not assume that a cooked Bitmap should become a standalone Bitmap JSON object. The desired source concept is normally the texture image itself, matching the engine pipeline's use of TGA source files.

A Material texture slot can use a compact reference such as:

```rust
pub struct TextureSourceRef {
    pub resource_name: Name,
    pub friendly_name: Option<Name>,
    pub artifact: String,
}
```

The exact fields can be reduced after testing. The important separation is:

- Resource identity remains in source JSON so cooking can recreate links and deduplicate shared textures.
- Pixel payload lives in a normal image artifact, not JSON.
- Cooked GPU format, swizzle, palette layout, mip layout, and platform flags do not leak into the source representation.

The cooked class layout may be shared across PC and GameCube, but its payload interpretation is platform-specific:

- PC may store either an embedded DDS container or headerless/raw pixel and mip data. Raw PC layout must be selected from the Bitmap metadata and validated payload size, not inferred from the absence of DDS magic alone.
- GameCube stores a platform-swizzled texture payload that must be unswizzled and decoded.

Therefore Bitmap-to-texture conversion needs platform-specific adapters even when both platforms use the same Rust cooked class variant. Dispatch must use the `BffClass` header platform/version from `CookedProject`; matching only the `Bitmap` enum variant is insufficient.

Use a two-stage texture pipeline:

1. A cooked adapter classifies the payload using platform plus Bitmap metadata, then decodes PC DDS, the applicable raw PC pixel format, or GameCube swizzled data into a neutral pixel/image representation.
2. A source artifact encoder writes the selected source image format.

The reverse cooking pipeline reads the neutral source image and uses a target adapter to generate the Bitmap payload expected by the target version/profile: embedded PC DDS, a raw PC pixel/mip layout, or GameCube swizzled data.

Do not make DDS the universal source artifact. It is only one possible cooked PC representation and does not cover raw PC or GameCube payloads. Evaluate TGA first because it matches the known original engine source pipeline. Compare it with PNG or another lossless format using these requirements:

- Exact color and alpha preservation.
- Normal-map channel preservation.
- Broad Blender/C++/Python tooling support.
- Deterministic encoding.
- No dependence on platform swizzle or GPU compression.

Mipmaps, palette choice, hardware compression, swizzling, and similar values should normally be cooking-profile decisions. If investigation finds authored Bitmap semantics that cannot be inferred from the image or Material slot, add only those semantic overrides to `TextureSourceRef`; do not reproduce the rich Bitmap body in source JSON.

Extend source asset artifact handling so `export_source_asset` writes texture artifacts after `source.json`, mirroring the existing rich `Export` timing. Use deterministic artifact paths based on Bitmap resource names so repeated references resolve to the same file.

On source import, resolve the explicit artifact reference before cooking. If the relevant platform decoder is unavailable or loses required data, do not mark the Bitmap represented. For a complete MeshSource with embedded Materials, an unresolved required texture should fail the owning Mesh source build rather than produce a broken source document.


## Version-Specific Conversion Boundaries

Keep all cooked-format knowledge in cooked class modules.

### Shared `1.06.63.0x` conversion

`bff/src/class/mesh/v1_06_63_0x.rs` converts common fields:

- Object metadata and Object flags.
- Drawing distances.
- Primitive collisions.
- Morph targets.
- Ordered material names.
- The Object `data_name` relationship as the required build-only `mesh_data_name`.

The common adapter must inspect AABB/morph presence and reject source conversion when unexplained non-empty data would otherwise be lost. AABB data can be omitted only after the ImHex analysis establishes that it is derived from source fields already represented.

Use small semantic accessors on cooked structs where privacy blocks a clean conversion. Do not mirror private cooked fields into the source layer or add ad hoc raw getters for every field.

### Paired MeshData adapter

Keep MeshData version/platform decoding in its cooked class module. Mesh assembly supplies the exact `mesh_data_name`, requires `ClassType::MeshData`, and dispatches the concrete cooked variant there. The adapter may contribute source-semantic fields to Mesh when such fields are identified, but it must not manufacture a separate source asset identity or serialize its own resource name.

For the first implementation, the adapter can validate the expected MeshData shape and report that its known volume arrays are generated data. Do not invent fields simply to make the conversion return a non-empty value.

### PC adapter

`bff/src/class/mesh/v1_06_63_02_pc.rs` owns PC geometry decoding.

Initial supported layouts:

- `NoBlend`: position, packed tangent, packed normal, UV0, and UV1/lightmap UV.

Reject in the first static milestone:

- `Volume`.
- `Skin1Blend`.
- `Skin4Blend`.
- `SkinVolume`.
- Morph layouts until morph conversion is implemented.
- Unknown layouts.

PC decoding steps:

1. Iterate primitive infos, not vertex buffers alone.
2. Validate primitive type is a triangle list.
3. Resolve each primitive's actual vertex and index buffer.
4. Resolve index-buffer start/count from `index_buffer_offset_in_shorts` and `face_count`.
5. Establish the exact semantics of `start_vertex` and `vertex_buffer_range_begin` against real files before applying indices.
6. Validate every resolved index against both the primitive range and selected vertex buffer.
7. Decode packed normal/tangent bytes in semantic component order; verify the byte-order reversal currently used by Blender against known geometry.
8. Intern source attributes independently and emit source corners.
9. Map each primitive to the correct material slot. Validate the observed identity behavior of `primitive_info_indices` rather than relying on it blindly.

Support multiple vertex and index buffers even if current samples usually contain one.

### GameCube adapter

`bff/src/class/mesh/v1_06_63_01_gc.rs` owns GameCube geometry decoding.

Initial non-skinned path:

1. Require `OptimizedVertices::Unskinned`.
2. Group optimized normals into semantic 3-component normals and optimized texcoords into semantic 2-component UV values, according to the actual GX array indexing convention.
3. Require display-list vertex format `GX_VTXFMT1`.
4. Decode the GX primitive opcode from `primitive_and_vtx_fmt`.
5. Convert triangle lists, triangle strips, and triangle fans to source triangles when observed; reject unsupported line/point/quad commands unless explicitly implemented.
6. Resolve each display-list vertex reference into independent position, normal, UV0, and UV1 indices.
7. Assign material indices using display-list/material-slot order only after validating it against real files.

`GX_VTXFMT2` is the skinned path and must fail source conversion for now.

If `DisplayList.trailing_data` is non-empty, do not assume it is disposable. Either decode it as additional GX commands or fail Mesh source conversion. Rich/raw fallback preserves it.

### Legacy source-like arrays

The shared Mesh body may still contain positions, normals, UVs, strips, and strip extensions in some files.

Use this policy:

1. Prefer the platform's authoritative cooked render representation when it is populated.
2. Use the legacy source-like arrays only as a fallback when the platform representation is absent.
3. If both are populated, decode the authoritative representation and optionally validate the legacy arrays; do not merge them blindly.

## Source Assembly and Fallback

Implement `MeshSource` manually in the current registry:

- Add `Mesh(MeshSource)` to `SourceAssetData`.
- Add `MeshSource::build_all` to `SOURCE_ASSET_BUILDERS` after Node.
- Add manual `SourcePart` and `ToSourcePart` dispatch matching the Node pattern.
- Do not implement `source_assets!` yet.

Build transaction:

1. Convert the cooked Mesh into `MeshSourcePartBuild`.
2. Use `mesh_data_name` to require the exact paired `ClassType::MeshData` and run its version/platform adapter.
3. Merge only genuine source-semantic MeshData contributions into the owning Mesh source value; generated values need no JSON representation.
4. Require every ordered Material source part.
5. Decode every required cooked Bitmap through its platform adapter and collect source texture artifacts.
6. Validate geometry, the Mesh/MeshData pair, material indices, primitive collisions, morph handling, AABB handling, and unsupported data.
7. Only after every step succeeds, mark Mesh, its paired MeshData, Materials, and successfully decoded Bitmaps as represented.

This ordering prevents a partially built Mesh from suppressing fallback dumps for dependencies it failed to preserve.

The Mesh owns and embeds its Material parts; do not emit the same cooked Material as a duplicate part of several Mesh source documents under the current evidence. Shared textures are different: multiple embedded Material parts may retain the same `TextureSourceRef`, and source artifact handling should resolve that stable Bitmap identity to one shared image artifact. Cooking emits each owned Material with its owning source asset and deduplicates shared Bitmap resources by name, rejecting conflicting texture definitions.

If Mesh conversion fails:

- Do not mark the Mesh, paired MeshData, or any of its other cooked resources represented.
- Let the existing source strategy try rich export and then raw fallback.
- Never emit both a successful MeshSource and a rich/raw dump of that same cooked Mesh.

## Implementation Order

### Phase 0: Port patterns and validate the binary corpus

1. Binary-extract the representative PC/GC corpus and record its local manifest.
2. Port the relevant Rat 010 templates into `patterns/rat`, beginning with PC/GC Mesh, PC MeshData, Material, and Bitmap.
3. Preserve the existing combined PC/GC Mesh dispatch and verify every ported offset, count, endian rule, and EOF condition against real resources.
4. Add `_Analysis.hexpat` variants for hypotheses that should not yet become production assertions.
5. Extend `validate.py` with an analysis-pattern override/suffix and, only when an investigation needs aggregate values, optional capture of tagged analysis records from successful runs.
6. Run strict and analysis patterns recursively and in parallel over all matching extracted resources after each meaningful pattern change.
7. Correlate ImHex-produced records across Mesh, MeshData, Material, and Bitmap, then complete the hypothesis ledger and required decision records.
8. Do not begin Phase 1 until the Phase 0 gate above is satisfied; unresolved cases must have an explicit rejection, fallback, or scoped-preservation policy.

### Phase 1: Source structs and invariants

1. Add Object source metadata and Object flags.
2. Add collision source structs and collision flags.
3. Add Mesh geometry, corner, face, and morph structs.
4. Record the ownership invariant that one Mesh source asset requires and represents its 1:1 cooked MeshData, without adding a standalone MeshData source type or an empty JSON field.
5. Add Material source parts and texture artifact references.
6. Add constructors/validators for index and cardinality invariants.

### Phase 2: Source artifacts

1. Add artifact collection to source asset build/export without putting bytes in JSON.
2. Add the neutral decoded-image boundary and deterministic texture artifact paths.
3. Update source extraction to write source artifacts after `source.json`.
4. Lay down the corresponding import hook, even if cooking is not implemented yet.

### Phase 3: Material and texture adapters

1. Implement PC payload classification using DDS magic together with Bitmap metadata and validated expected sizes.
2. Implement the embedded-PC-DDS-to-neutral-image adapter.
3. Implement neutral decoders for every raw PC pixel/palette/mip layout observed and confirmed during Phase 0; reject unsupported format values explicitly.
4. Locate/integrate the GameCube unswizzling code and implement the GC-to-neutral-image adapter.
5. Evaluate TGA versus PNG or another lossless source artifact and choose one without coupling the source model to it permanently.
6. Implement `MaterialV1_06_63_02PC -> MaterialSourcePartBuild` with `TextureSourceRef` slots.
7. Add semantic flag mappings for known Material flags.
8. Preserve texture slot order and optional slots.
9. Dispatch Bitmap payload conversion by the BffClass platform/version and payload metadata, not only the shared cooked enum variant.

### Phase 4: Mesh common adapter and assembly

1. Apply the Phase 0 AABB decision; if the relationship remains unresolved, retain the non-empty-AABB rejection gate.
2. Implement Object header conversion and the `1.06.63.0x` common Mesh conversion from the validated pattern layouts.
3. Implement primitive sphere/box/cylinder collision conversion.
4. Return `data_name` as build-only `mesh_data_name`, require that exact MeshData, and apply the confirmed derivation/preservation policy for its ObjectDatas flag and color.
5. Treat the paired MeshData as represented by the successful Mesh source build and include it in the same atomic represented-resource set.
6. Implement ordered Material/texture assembly using the confirmed material-slot rules.
7. Add Mesh manually to the source asset registry; do not register MeshData.

### Phase 5: Static PC non-skinned geometry

1. Implement `NoBlend` decoding.
2. Support every referenced vertex/index buffer rather than assuming index zero.
3. Validate index base/range semantics with real PC files.
4. Reject `Volume`, skinned, morph, and unknown layouts without marking resources represented.

### Phase 6: Static GameCube non-skinned geometry

1. Implement optimized position/normal/UV stream decoding.
2. Implement `GX_VTXFMT1` corner decoding.
3. Implement observed triangle primitive opcodes.
4. Validate display-list-to-material ordering and the two material-name arrays.
5. Reject Object `ShadowVolume`, `GX_VTXFMT2`, and undecoded trailing commands.

Completing both Phase 5 and Phase 6 is the proof that `MeshSource` is platform independent.

### Phase 7: Non-skinned morph targets

1. Convert common `1.06.63.xx` morph targets to semantic point displacements.
2. Reconcile cooked duplicated vertices with source point indices.
3. Add replacement normals where available.
4. Keep morph AABB/map structures cooked-only.
5. Enable non-skinned morph layouts only after their attribute semantics are understood.

### Phase 8: Validation and Blender consumption

1. Export known PC and `DREAM03.DGC` Mesh assets as source.
2. Add a Blender source importer that consumes `MeshSource` directly rather than version-specific rich JSON.
3. Keep Blender coordinate/winding/UV conversion in Blender.
4. Compare face counts, material assignments, bounds, UV seams, split normals, and collision primitives against the current rich importer.
5. Test source texture artifacts by resolving PC- and GC-derived images from Material slots.

### Phase 9: Cooking groundwork

Do not implement every cooker yet, but keep the source contracts cookable:

- PC cooker groups faces by material and interns cooked vertices by the tuple of source corner attributes required by the chosen layout.
- GameCube cooker writes semantic streams and display-list references from the same source corners.
- Every Mesh cooker emits the paired target MeshData resource and wires the cooked Mesh `data_name` to it in the same transaction. Initial groundwork covers identity and derivable non-volume values; Phase 12 supplies target-specific MeshVolume generation.
- Object and Material flags encode through version mappings.
- Bounds are rebuilt or retained through explicit cooking policies.
- AABB faces/nodes are rebuilt from Mesh geometry only if the equivalence analysis proves that relationship.
- Source texture images are encoded into target-specific embedded PC DDS, raw PC pixel/mip payloads, or GameCube swizzled Bitmap payloads according to the target profile.
- Each embedded Material is emitted once by its owning source asset. Shared Bitmap resources are deduplicated by stable resource name, and conflicting texture definitions are errors.
- Vertex layout is selected from usage and target profile, not stored in MeshSource.

The eventual API remains source-driven:

```rust
mesh_source.cook(&mut CookContext, target_profile)
```

Version/platform-specific code lives in cooked adapters, not in one large `MeshSource::cook` match.

The completed `CookContext`/compiled-resource collection is the future input boundary for `MANIFESTLESS_BIGFILE_BUILD_PLAN.md`. It must preserve deterministic emitted resource identities and order, but it must not assign bigfile blocks or compression decisions.

### Phase 10: Source-layer cleanup

After PC and GameCube MeshSource work:

1. Move `represented_resources` into `SourceAssetBuild<T>` and mark them in the generic successful-build path.
2. Update Node and Mesh to return represented resources instead of mutating the context manually.
3. Add successful source-asset lookup/caching by `(ClassType, Name)` so later Skin can read built MeshSource.
4. Keep build-only Mesh mappings on MeshSource with serde/schema skipping.
5. Review repetitive `SourcePart` dispatch only after Node, Mesh, Material, and cooked-Bitmap-to-texture conversion expose the real pattern.

### Phase 11: Skel and Skin

1. Add `SkelSource` with semantic bone hierarchy and local transforms.
2. Add `SkinSource` after Mesh and Skel are cached and queryable.
3. Let Skin read MeshSource's cooked-to-source point mapping.
4. Convert PC skin layouts and GameCube `GX_VTXFMT2` weights.
5. Keep cooked Mesh owned by MeshSource and cooked Skin owned by SkinSource; a Skin failure must never cause a successfully sourced Mesh to be dumped again.

### Phase 12: Shadow-volume path

After the normal Mesh and Skin foundations are stable:

1. Decide from the AABB/MeshVolume analysis whether the render Mesh is also the shadow source mesh.
2. Validate the selected source geometry is closed and manifold by deriving undirected edge adjacency from triangle indices.
3. If a distinct proxy is proven, add the narrowly scoped optional `ShadowVolumeSourceMesh`; otherwise reuse `MeshGeometrySource`.
4. Implement the GameCube cooker that builds `FaceVolume_Z` and `EdgeVolume_Z` with remapped face/vertex relationships.
5. Implement the PC cooker that expands the same neutral triangles into the target Volume vertex/index layout.
6. Add uncooking for PC `Volume`/`SkinVolume` and the corresponding GameCube path only when it can recover that neutral source without loss.
7. Extend the already-owned paired MeshData output with the target-specific MeshVolume representation; this does not change source ownership or create a MeshData source asset.

### Phase 13: Macro work

Only after Node + Mesh + Material + texture conversion + Skel + Skin establish stable repetition:

- Follow the focused plan in `SOURCE_MACRO_PLAN.md`.
- Extend `classes!` with cooked-family metadata so source lookups stop repeating `Class::X` matching and wrong-class errors.
- Add `source_assets!` only for the current top-level `SourceAssetData` and builder registry boilerplate.
- Preserve the outer `SourceAsset` document wrapper and use `ClassType` as source identity.
- Keep `ToSourcePart<P>` destination-generic; do not assume one cooked family has exactly one source part.
- Reassess generated cooked-variant dispatch only after the manual MeshData, Material, Skel, and Skin paths expose the real mappings.
- Do not macro field conversion, assembly, dependency cardinality, preservation, or represented-resource policy.

### Phase 14: Manifestless bigfile construction

Begin only after source cooking and its cleanup are stable enough to materialize a complete target-specific resource collection:

1. Follow `MANIFESTLESS_BIGFILE_BUILD_PLAN.md`.
2. Preserve the current existing-manifest path for extracted binary, rich, source, and mixed projects.
3. Add deterministic project materialization before layout so source assets may emit multiple cooked resources.
4. Add version/game-specific layout profiles; do not treat Garfield, Rat, Wall-E, APTR, or later formats as one universal planner.
5. Implement manifest generation first for supported legacy profiles, clearly separating exact uncompressed behavior from heuristic compressed behavior.
6. Support full explicit regeneration when new or removed source emissions make an extracted manifest stale; do not implement an incremental manifest patcher initially.
7. Keep APTR and other lossy/unwritable manifest versions explicitly unsupported until their richer grouping and writer requirements are modeled.
8. Save generated manifests as reproducible build locks and read back every emitted bigfile before reporting success.

## Validation Checklist

Run after each implementation phase:

- `cargo check`.
- `cargo build --workspace`.
- `cargo +nightly fmt --all -- --check`.
- `cargo +nightly clippy --locked --workspace --all-targets -- -D warnings`.

Mesh-specific checks:

- Every face has exactly three corners.
- Every corner position index is valid.
- Optional normal/color/tangent/UV indices are valid when present.
- Every face material index addresses the ordered Material vector.
- PC primitive ranges stay within their selected buffers.
- GC display-list references stay within optimized attribute arrays.
- No skinned data is silently accepted by the non-skinned path.
- No Object `ShadowVolume`, PC `Volume`, or `SkinVolume` enters the initial path.
- No non-empty morph or trailing display-list data is silently dropped.
- Non-empty AABB data is omitted only after its derivation from represented geometry is proven.
- `data_name` and the MeshData resource name never appear in Mesh source JSON.
- No standalone MeshData source document is emitted.
- Generated MeshVolume arrays are not mirrored into source JSON.
- Every accepted cooked Material has one owning source asset and is embedded as a reusable `MaterialSourcePart`; an unexpected multi-owner Material is rejected until an explicit policy exists.
- Multiple Material parts may reference the same texture identity and shared source image artifact.
- Successful Mesh source export marks both the Mesh and its exact paired MeshData represented and suppresses rich/raw output for both.
- Failed Mesh source export leaves the Mesh, paired MeshData, and every other involved cooked resource eligible for fallback.
- Node source export still includes UserDefine, AnimFrames, messages, and play flags.

Cross-platform comparison:

- A PC and GC version of equivalent content produce the same source-level topology shape.
- Platform-only fields never appear in `source.json`.
- Attribute values are in engine coordinates, not Blender coordinates.
- Material slot ordering is deterministic and matches faces.

## Assumptions

- First implementation is uncook-first; cooking APIs are shaped but not completed.
- Source export may fail and fall back for unsupported Meshes, but it must not silently lose recognized data.
- Initial non-skinned support excludes shadow-volume geometry even though it has no skin weights.
- AABB data is omitted only after analysis proves how it derives from source geometry. MeshVolume pointer/adjacency/runtime arrays are cooked data; analysis still must determine whether they derive from the normal Mesh geometry or require an additional semantic shadow proxy in `MeshSource`.
- DDS is one PC cooked payload form, not the universal PC representation and not the source texture format.
- Material parts are embedded by one owning source asset; texture references and artifacts may be shared across those owners by stable Bitmap resource identity.
- No source registry or conversion macro is added until the manual Mesh path proves the architecture.
- Manifestless bigfile construction is deferred until target cooking is stable; source JSON never stores block membership, compression, offset, or other bigfile layout policy.
