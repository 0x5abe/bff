# Deferred Manifestless BigFile Construction Plan

## Status and Scheduling

This work is deliberately deferred until the source layer and target-specific
cooking APIs are stable. `NEXT_STEPS_LATEST.md` remains the active Mesh/source
roadmap, and `SOURCE_MACRO_PLAN.md` remains the later cleanup/macro reference.

The source work should establish that a target cooker can turn a complete
source project into a deterministic collection of cooked `Resource` values.
Manifestless bigfile construction begins after that boundary is proven. It must
not shape the source JSON around one bigfile version's packing rules.

The first implementation target is the legacy Asobo family currently studied
in the external bigfile-ordering research: Garfield 2, Ratatouille, and Wall-E
PC/PSP. Later formats such as APTR require separate manifest and layout work and
must remain explicitly unsupported until their richer structures are modeled
losslessly.

## Goal

Allow `bff create` to consume a project directory containing any mixture of:

- binary `BFF0` resources;
- rich `resource.json` resources and artifacts;
- target-independent `source.json` assets and artifacts;

and build a bigfile when `manifest.json` is absent by generating a deterministic
best-attempt manifest for a selected target profile.

When an extracted project already contains `manifest.json`, retain it as the
authoritative layout plan after materializing all source/rich/binary inputs.
Only regenerate the full layout when explicitly requested.

## Non-Goals for the First Version

- Byte-identical reproduction of an unknown historical builder.
- A universal packing rule shared by every engine version.
- An incremental algorithm that patches an existing manifest for newly added or
  removed resources.
- Silent fallback from an unsupported target to a similar legacy planner.
- APTR or another later format whose current bff manifest is lossy or whose
  writer is incomplete.
- Whole-block compression rules from unrelated engine branches.

## Current bff Baseline

`bff-cli/src/create.rs` currently:

1. Opens `manifest.json` before reading resources.
2. Uses the manifest version to choose `NameType` and construct `NameContext`.
3. Reads files in `resources/` as `BFF0` and directories as `resource.json`.
4. Stores resources in a `HashMap<Name, Resource>`.
5. Constructs `BigFile::new(manifest, resources)` and dispatches to the selected
   writer.

For the legacy writers, the manifest block/resource arrays are the actual write
plan. A folder resource absent from the manifest is omitted; a referenced name
absent from the resource map eventually reaches an `unwrap`.

The extracted cooked inputs retain enough information to serialize a resource:

- `BFF0` stores platform and version in its wrapper header, followed by the
  decoded/decompressed `Resource`;
- `resource.json` stores the same target information in `BffClass.header`;
- both preserve class/name identity and the resource link/body data.

They do not retain original block membership, resource compression decisions,
working offsets, block checksums, XPLE values, or pool topology. `source.json`
intentionally carries no cooked target at all.

The existing optional manifest defaults are not sufficient as a deliberate
planner:

- omitted resource `compress` defaults to false;
- omitted legacy block `offset` uses a local writer fallback that does not
  implement the researched minimum/right-alignment model;
- omitted checksum writes zero;
- omitted XPLE values become zeroes;
- omitted bigfile type generally becomes `Normal`.

A generated plan must fill every required decision explicitly.

## Architectural Boundaries

Keep source cooking, layout planning, manifest persistence, and binary writing
as separate stages:

```text
project directory
      |
      v
InputProject (BFF0 + rich + source)
      |
      v
CookProfile::<target>
      |
      v
CompiledProject (ordered target-specific Resources)
      |
      v
LayoutProfile::<game/build lineage>
      |
      v
BuildPlan::<target>
      |
      +--> lossless manifest JSON
      |
      +--> target writer --> read-back validation
```

The cooked resource collection is the boundary between source work and bigfile
layout work. The layout planner must not know whether a resource originated as
source, rich JSON, or `BFF0`.

One `source.json` may emit zero, one, or several cooked resources. A Node source
asset can, for example, represent the Node, UserDefine, and AnimFrame resources.
Therefore discovery cannot assume one project entry equals one manifest entry.

## Target and Profile Resolution

Version and platform alone are not always enough. A target profile represents a
game/build lineage and owns:

- source-to-cooked conversions;
- resource class variants and serialization;
- name hashing/`NameType`;
- compression algorithms and thresholds;
- manifest representation;
- block membership, ordering, capacity, offset, checksum, and pool rules;
- read/write/synthesis capability declarations.

Resolution priority should be:

1. Existing lossless manifest when using it as the write plan.
2. Explicit `--target-profile` and compatible explicit overrides.
3. Unanimous target metadata from all cooked `BFF0`/`resource.json` entries.
4. Otherwise fail with the missing or conflicting target information.

A manifestless source-only project always requires an explicit target profile.
A hybrid project may infer a candidate from cooked fallbacks, but every fallback
must agree and the selected cooker must support the target.

Resolve the target before deserializing target-dependent `Name` values. Create
the target `NameContext`, then load cooked and source entries. Source JSON should
prefer symbolic names. Numeric-only hashes cannot in general be translated to a
different name scheme and must require a mapping or fail.

## Project Materialization

Discovery must be deterministic:

1. Resolve `directory/resources`, or an explicitly supplied resource root.
2. Sort entry paths using a stable, documented comparison.
3. Classify each entry as `BFF0`, `resource.json`, or `source.json` before
   decoding it.
4. Preserve a stable discovery ordinal through cooking and planning.

Materialize the full project before consulting block membership:

1. Load all source assets and source-side artifacts into a project graph.
2. Validate source identity, references, and cooker availability.
3. Cook the whole source graph for the selected target; do not cook isolated
   source files independently.
4. Import rich resources and artifacts for the same target.
5. Read binary resources and validate their embedded target.
6. Merge all emitted resources by name, rejecting source/source,
   source/cooked, and cooked/cooked collisions.
7. Reject partial source builds: every source asset must be cooked successfully
   or reported as unsupported.

Target-specific preserved fragments require explicit compatibility metadata.
Never copy preserved cooked bytes into another target blindly.

## Existing Manifest, Missing Manifest, and Regeneration

bff extraction always writes `manifest.json`, including Source exports. The
normal extracted mixed project therefore follows the existing-manifest path:

1. Materialize the complete source/rich/binary project.
2. Verify every manifest resource is emitted exactly once and no emitted
   resource is unlisted.
3. Preserve stored membership, relative order, compression choices, and offsets.
4. Reserialize all resources and recalculate byte-derived checksums/sizes.
5. Validate that edited content still satisfies ordering, capacity, and
   in-place decompression requirements.

Strict reuse fails if new resources were emitted, referenced resources are
missing, or edits make the stored layout unsafe. Report the exact added,
missing, duplicate, or unsafe resources.

Do not implement an incremental updater initially. An explicit `regenerate`
mode instead:

1. Uses the existing manifest only as a source of compatible target/top-level
   metadata.
2. Materializes all source/rich/binary entries.
3. Discards old blocks, resource order, compression flags, offsets, and
   checksums.
4. Runs the selected planner over every emitted cooked resource.
5. Rebuilds resource-dependent pools, common references, and version-specific
   groupings; if the profile cannot do so, regeneration is unsupported.
6. Writes and validates a new manifest/bigfile pair.

The same full planning path applies when no manifest exists. A generated
manifest becomes the new build lock so subsequent builds do not rerun heuristic
decisions unless regeneration is requested.

## Version-Specific Build Capabilities

Each target profile should report capabilities rather than relying on a broad
version match:

```text
can_read
can_write_existing_manifest
manifest_is_lossless
can_synthesize_from_cooked
can_cook_source
layout_confidence = exact | heuristic | unsupported
```

`auto` may select only a profile declaring the requested capability. Never use
the nearest older planner for an unsupported version.

Current planning confidence is:

| Target family | Known planning behavior |
| --- | --- |
| Garfield 2 | Scalar order, anchors, and capacity are known; compressed filler membership and some historical offset choices remain unknown. |
| Rat | Uncompressed RTC membership is known; compressed membership, capacity excess, and historical offsets remain heuristic. |
| Wall-E PSP | Uncompressed membership is known from DPP/NPP/DPP.LAYOUT evidence. |
| Wall-E PC | Uncompressed RTC/RTE behavior is known; compressed membership/capacity/offset behavior remains heuristic. |
| APTR `v2_128_52_19` | Unsupported for synthesis. Current manifest flattening is lossy and the writer is unimplemented. |

The APTR reader currently flattens multiple resource lists and as many as 52
`DataDescription` groups into one `ManifestBlock.resources` array. It also loses
local/external placement distinctions needed by the richer map layout. APTR
requires a tagged version-specific manifest payload or a versioned manifest
type before its writer or planner can be advertised.

## Legacy Layout Model

Use a sector size of `0x800` for the scoped legacy profiles. Define:

```text
Garfield:
    D = 0x10 + (compressed ? P : S)
    K = S + P

Rat / Wall-E PC:
    D = 0x18 + data_size
    K = L + U + P

Wall-E PSP:
    D = 0x18 + data_size
    K = D for the confirmed uncompressed packing model
```

Where:

- `D` is serialized disk size before block padding;
- `K` is the scalar used by the historical final order and anchor choice;
- `L` is link-header size;
- `U` is decompressed body size;
- `P` is compressed body size including the compression header where the
  target format counts it that way;
- `S` is Garfield's stored/decompressed payload size field.

Within every studied compressed legacy block, final resources are nondecreasing
by that target's `K`, and the block anchor is the maximum `K` remaining when the
block is created. Equal-key historical order is unavailable; preserve a stable
input ordinal for deterministic ties.

### Exact uncompressed planning

For Wall-E PSP, Rat RTC, and Wall-E PC RTC/RTE, use the confirmed two-parity
largest-fitting algorithm:

1. The first block of each parity takes the largest remaining resource and
   establishes that parity's capacity from the aligned anchor plus the observed
   sector policy.
2. Repeatedly take the largest remaining resource that fits.
3. Serialize selected resources in ascending `D`.
4. Reuse the parity capacity for later blocks.
5. Apply the documented final-block/profile exceptions where corpus evidence
   requires them.

### Heuristic compressed planning

The final scalar order and anchor are known, but filler selection is not fully
recoverable for compressed Garfield, Rat, or Wall-E PC. Provide explicit,
deterministic filler policies such as:

- stable discovery-order scan;
- descending `K` largest-fitting;
- descending stored-size largest-fitting.

Do not label a structurally valid heuristic plan as an exact reproduction.
Record the selected policy in build diagnostics and, if useful, non-binary
manifest metadata.

Garfield's researched parity-capacity formula may be used for its profile.
Compressed Rat and Wall-E PC must use a safe declared capacity policy; their
historical capacity excess is not known.

## Compression Planning

Extraction stores decompressed resource bodies, so a resource-only project has
no exact preserve-compression mode without an existing manifest.

Support explicit policies:

```text
none
all                 # diagnostic; may produce poor/larger output
auto                # use actual target compressor and profile threshold
per-resource         # future explicit map
```

RTC profiles force `none` and reject conflicting flags. For the studied
Rat/Wall-E resource-level LZRS policy, the current corpus supports using a
default acceptance threshold of:

```text
compressed_size <= decompressed_size * 0.80
```

Precompress before membership planning because compressed size affects both `D`
and `K`. A first implementation may compress again in the existing deterministic
writer; a later `PreparedResource` path should pass the exact planned bytes to
the writer so measurement and emission cannot diverge.

## Working Buffers, Capacities, and Checksums

For each compressed resource, derive its minimum safe in-place decompression
offset from its decompressed size and source position within the serialized
block. The block minimum is the sector-aligned maximum requirement:

```text
R_min = align_sector(max(max(0, U_i - source_offset_i)))
```

Every generated block must satisfy:

```text
W >= R_min
C_parity >= B + W
C_parity = max(B + W for blocks of that parity)
```

Where `B` is padded block size, `W` working-buffer offset, and `C_parity` the
even/odd working-buffer capacity. Right-aligning with `W = C_parity - B` is a
safe deterministic best-attempt finalization policy, not a claim about every
historical offset.

Generated manifests should store explicit offsets for legacy blocks instead of
using current bff writer fallbacks.

For profiles that use it, calculate the block checksum from the final unpadded
serialized block using bff's `asobo_alternate32`. Corpus checks reproduce Rat
and Garfield checksums with this algorithm. Profiles known to store zero should
continue to store zero.

## Manifest Field Policy

| Field | Generated policy |
| --- | --- |
| `version` / `platform` | Resolved target; all cooked inputs must agree or be explicitly converted. |
| `version_xple` | Target-profile value, explicit option, or compatible template; do not silently guess unknown values. |
| `bigfile_type` | Explicit/profile value; RTC/RTE path inference may be an opt-in convenience. |
| block resource names | Final target-profile membership and serialization order. |
| resource `compress` | Existing manifest choice, or explicit generated compression policy. |
| block `offset` | Explicit value calculated after membership and parity capacity are final. |
| block checksum | Derived from final bytes for profiles that use it. |
| block `compress` | Only for formats with block compression; absent for the scoped resource-level LZRS profiles. |
| pool/common/group data | Rebuild through a profile that understands it, use a compatible lossless manifest, or reject synthesis. |
| builder tag | User/profile metadata only; never layout input. |

`version_to_write` changes only the version string emitted by current bff. It
must not select a resource/layout backend. Use the resolved target profile for
backend selection.

## Proposed CLI

Keep current behavior compatible by default while making synthesis explicit:

```text
bff manifest <directory> --output <manifest.json> [build options]
bff create <directory> <bigfile> --manifest-mode <mode> [build options]
```

Manifest modes:

- `require`: manifest must exist; materialize inputs and validate its exact plan.
- `auto`: use an existing valid manifest, otherwise generate one if supported.
- `generate`: manifest must be absent; generate it and optionally stop before
  writing the bigfile.
- `regenerate`: use compatible old metadata as a template but fully replace the
  resource layout using the selected planner.

Initial build options:

```text
--input-mode auto|cooked|source|hybrid
--target-profile <game-platform-version-profile>
--bigfile-type rtc|normal
--version <version>
--platform <platform>
--version-xple <a,b,c>
--compression none|all|auto
--compression-threshold <ratio>
--packing auto|exact-uncompressed|heuristic-compressed
--filler-order discovery|k-desc|d-desc
--template-manifest <path>
--write-generated-manifest <path>
```

Generated manifests should be saved by default. Build transaction order:

1. Write a candidate manifest without replacing an existing one.
2. Write the candidate bigfile.
3. Read it back and validate structure/resource identity.
4. Promote the candidate manifest only after success.

## Suggested Code Organization

```text
bff/src/source/cook.rs
    InputProject / ProjectEntry
    CookOptions / CookContext
    CookProfile
    CompiledProject
    cook_project(...)

bff/src/bigfile/layout/mod.rs
    BuildOptions
    CompressionPolicy / PackingPolicy / FillerOrder
    PreparedResource
    BuildCapabilities / LayoutConfidence
    LayoutProfile
    BuildPlan

bff/src/bigfile/layout/profiles/
    garfield.rs
    rat.rs
    walle.rs
    aptr.rs                       # explicit unsupported capability initially

bff/src/bigfile/manifest.rs
    legacy schema
    version-specific lossless layout payloads when needed

bff-cli/src/create.rs
    discover and resolve target
    materialize source/rich/binary project
    validate existing plan or invoke LayoutProfile
    write candidate, read back, promote manifest

bff-cli/src/manifest.rs
    manifest-only command using the same materialization/planning API
```

`BigFileIo` currently exposes binary read/write and resource type. Keep
`CookProfile` and `LayoutProfile` separate from it. A higher-level
`BuildProfile` may aggregate them, but semantic source conversion and block
packing must remain independently testable.

## Implementation Phases

### Phase A: Capability and manifest fidelity audit

1. Inventory every bff version backend as read-only, writable from existing
   manifest, lossless-manifest, and synthesizable.
2. Add explicit capability errors and stop unsupported legacy fallback.
3. Document which optional current manifest fields are required per profile.
4. Prove manifest read/write JSON compatibility before extending its schema.

### Phase B: Deterministic project materialization

1. Add sorted entry discovery and classification.
2. Probe cooked headers before constructing `NameContext`.
3. Load binary and rich resources into an ordered collection.
4. Integrate the stable source cooker and allow source-only/hybrid projects.
5. Validate targets, names, duplicate emissions, artifacts, and full source
   completion.
6. Preserve the existing-manifest path with exact resource-set validation.

This phase can begin with cooked-only folders while returning a precise
“source cooker unavailable for target” error. Its API must already support the
later source branch.

### Phase C: Legacy planning primitives

1. Serialize resources into scratch buffers using the exact target writer.
2. Precompress eligible bodies and record final `D`, `K`, `R_min`, and ordinal.
3. Implement deterministic exact uncompressed packing.
4. Implement declared compressed filler policies.
5. Finalize parity capacities, offsets, checksums, and header totals.
6. Produce a `BuildPlan` and lossless legacy manifest.

### Phase D: CLI and transactional builds

1. Add `bff manifest`.
2. Add manifest modes to `bff create`.
3. Persist generated manifests and names using the populated `NameContext`.
4. Add candidate write/read-back/promotion behavior.
5. Print target, confidence, compression policy, filler policy, exact/heuristic
   status, and resource/block totals.

### Phase E: Writer integration and cleanup

1. Calculate checksums from emitted block bytes.
2. Stop depending on omitted-offset fallbacks for generated plans.
3. Validate resource set before writer `unwrap` calls.
4. Pass prepared compressed bytes into writers to eliminate double work.
5. Keep existing manifest-driven output behavior compatible.

### Phase F: Later formats

1. Extend manifests only with evidence from each target format.
2. Model APTR resource lists, data groups, local/common references, maps, and
   compression losslessly.
3. Implement and test the APTR writer before advertising synthesis.
4. Add a separate APTR layout-analysis effort; do not reuse legacy ordering.

## Validation Matrix

At minimum test:

- deterministic output under shuffled directory enumeration;
- binary-only, rich-only, source-only, and mixed project folders;
- one source asset emitting several cooked resources;
- source/source and source/cooked name collisions;
- mixed target rejection and explicit supported conversions;
- symbolic names and numeric-hash portability failures;
- strict existing-manifest set validation;
- edited content preserving a still-safe existing plan;
- new/missing resources causing strict failure, then appearing correctly after
  full regeneration;
- RTC forcing all resource compression flags false;
- auto compression using actual emitted compressed size;
- exact uncompressed corpus packing for supported profiles;
- heuristic compressed output satisfying scalar order, anchors, `R_min`, and
  capacity invariants;
- Asobo Alternate checksums where required;
- block-count and header-capacity limits;
- pools/common/group structures being rebuilt or rejected, never copied stale;
- generated manifest create/extract/resource-byte round trips;
- a second build using the saved manifest producing the same bytes;
- unsupported APTR synthesis failing clearly;
- version-specific manifest JSON round trips preserving all writer-required
  structure before a new writer is enabled.

Compressed corpus tests should validate resource identity and known structural
invariants, not demand original historical block membership. Exact original
layout equality is a valid expectation only when a lossless original manifest
is supplied or the target profile is explicitly proven exact.

Run the normal workspace checks throughout:

```text
cargo check
cargo build --workspace
cargo +nightly fmt --all -- --check
cargo +nightly clippy --locked --workspace --all-targets -- -D warnings
```

## Definition of Done for the First Release

- Existing manifest-driven builds remain compatible.
- A manifestless cooked-only project can generate and save a deterministic
  legacy manifest.
- Once source cooking is available, the same path accepts source-only and mixed
  projects without layout code knowing their origin.
- Full regeneration handles newly added/removed source emissions by replanning
  the complete project.
- Exact and heuristic profiles are labeled accurately.
- Every generated block passes ordering, size, checksum, capacity, and in-place
  decompression validation.
- The emitted bigfile reads back with every expected resource exactly once.
- Unsupported targets fail before writing partial output.

## Open Questions

- Original compressed filler discovery order for Garfield, Rat, and Wall-E PC.
- Historical compressed Rat/Wall-E capacity excess and working-offset policy.
- Safe target defaults for XPLE, type, tags, pools, and common references where
  the corpus does not establish them.
- Whether future source preservation fragments need a formal target-compatibility
  identifier.
- The lossless manifest representation and complete writer model for APTR and
  later versions.
