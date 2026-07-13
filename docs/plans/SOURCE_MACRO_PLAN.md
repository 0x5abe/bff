# Deferred Source Registry Macro Plan

## Status

Do not implement these macros until the manual source paths for Node, Mesh, Material/texture, Skel, and Skin have exposed stable repetition. This plan describes the current architectural constraints and likely generation boundaries. It intentionally does not lock in a source-part mapping syntax yet.

`NEXT_STEPS_LATEST.md` remains the active implementation plan. This document is only the focused Phase 13 macro reference.

## Current Baseline

The source layer currently has:

- `SourceAsset`: the serialized document wrapper containing resource name, `ClassType`, flattened `SourceAssetData`, and preserved fragments.
- `SourceAssetData`: the untagged enum of source asset values such as `NodeSource`.
- `SourceAssetClass`: implemented by source asset value types and used to build all candidates of one `ClassType`.
- `SOURCE_ASSET_BUILDERS`: the manual top-level builder registry.
- `SourcePartBuild<T>`: a source value plus represented resources and preserved fragments.
- `SourcePart`: project lookup/conversion into either a serialized part or a build-only part result.
- `ToSourcePart<P>`: generic cooked-family dispatch to a destination part/build type.
- `classes!`: the single registry of cooked class families and their version/platform variants.

The macro design must fit this shape instead of regenerating the older architecture.

## Superseded Assumptions

Do not revive these ideas from earlier drafts:

- Do not replace the `SourceAsset` document wrapper with `enum SourceAsset { Node(NodeSource), ... }`.
- Do not add `SourceAssetType`; `ClassType` is the source asset identity.
- Do not assume every cooked family has exactly one associated source part.
- Do not add `SourcePartClass::Part` or `SourcePartOf<CookedVariant>` until the one-family/one-part assumption is proven valid. Current evidence argues against it.
- Do not require every owned cooked resource to produce a serialized source object. MeshData may contribute ownership, validation, or derived data without producing `MeshDataSourcePart` JSON.
- Do not assume a cooked class can belong to only one source asset workflow. Different source assets may read the same cooked class without jointly owning its fallback behavior.
- Do not generate dependency direction, cardinality, represented-resource policy, or semantic source assembly in the first macro version.

## First Macro Scope

The first version should remove only stable registry and type-dispatch boilerplate:

1. Generate cooked-family metadata from `classes!`.
2. Generate the top-level source asset registry from `source_assets!`.
3. Consider generated cooked-variant dispatch only after its destination mapping is proven.

Field conversion, grouping, and ownership transactions remain normal Rust.

## 1. Cooked Family Metadata

Extend `classes!` to generate a trait for every cooked family:

```rust
pub trait CookedClassFamily: Sized {
    const CLASS_TYPE: ClassType;

    fn from_class(class: &Class) -> Option<&Self>;
}
```

Illustrative generated output:

```rust
impl CookedClassFamily for Node {
    const CLASS_TYPE: ClassType = ClassType::Node;

    fn from_class(class: &Class) -> Option<&Self> {
        match class {
            Class::Node(value) => Some(value),
            _ => None,
        }
    }
}
```

This removes repeated source-side family matching such as:

```rust
let Class::Node(node) = &bff_class.class else {
    return Err(WrongSourceClassError::new(...).into());
};
```

Add a generic lookup helper around this metadata, for example on `CookedProject`:

```rust
pub fn class_family<C: CookedClassFamily>(&self, name: &Name) -> BffResult<&C>;
```

The exact helper location should follow the source cleanup completed before Phase 13. It should perform project lookup and expected/actual class errors, but it must not mark resources represented.

Removed repetition:

- `Class::X` matching in each `SourcePart::from_project` implementation.
- Repeated expected `ClassType` declarations.
- Repeated missing/wrong-class error construction.

Still manual:

- Which cooked family a particular source conversion requests.
- What destination part/build type is produced.
- All field interpretation.

## 2. Top-Level `source_assets!` Registry

Use a compact registry based on the existing naming convention:

```rust
source_assets! {
    Node,
    Mesh,
    Skel,
    Skin,
}
```

By convention, `Node` maps to:

- `ClassType::Node`.
- `crate::source::classes::node::NodeSource`.
- `SourceAssetData::Node(NodeSource)`.

Use `pastey` as `classes!` already does. Add an explicit path/type escape hatch only when a real source asset breaks the convention.

Generate the current architecture's boilerplate:

- Variants of `SourceAssetData`.
- `SourceAssetData::class_type()` matching.
- `SourceAssetClass` implementations that delegate to each source type's manual build entry point.
- `into_asset_data()` wrapping.
- `SOURCE_ASSET_BUILDERS` entries.
- Optional `source_asset_class_types()` and `is_source_asset_class_type()` helpers if actual call sites need them.

Do not generate or replace the outer `SourceAsset` struct. Its common document metadata must remain at one stable serialization level.

An illustrative generated trait implementation is:

```rust
impl SourceAssetClass for NodeSource {
    const SOURCE_CLASS_TYPE: ClassType = ClassType::Node;

    fn build(
        name: Name,
        ctx: &mut UncookContext,
    ) -> BffResult<SourceAssetBuild<Self>> {
        Self::build_from_project(name, ctx)
    }

    fn into_asset_data(self) -> SourceAssetData {
        SourceAssetData::Node(self)
    }
}
```

The source type still owns `build_from_project`; the macro only registers and wraps it.

Removed repetition:

- Manual `SourceAssetData` variant additions.
- Manual `class_type()` match arms.
- Manual `SOURCE_ASSET_BUILDERS` entries.
- Mechanical `SourceAssetClass` identity/wrapping code.

Still manual:

- Candidate selection when it differs from the source asset's `ClassType`.
- Source assembly and build transaction.
- Represented resources and preservation.
- Error/fallback policy.

## 3. Cooked Variant Dispatch

Keep the current destination-generic trait shape:

```rust
pub trait ToSourcePart<P> {
    fn to_source_part(&self, name: Name) -> BffResult<P>;
}
```

This is more flexible than one associated `Part` because:

- A cooked family may contribute different build outputs to different source workflows.
- Some conversions return serialized parts while others return build-only metadata plus `SourcePartBuild<T>`.
- Generated data classes may contribute no serialized value.

Current family-level `match` implementations in cooked `shared.rs` modules are repetitive because every new cooked variant adds another arm. `classes!` is the only place that already knows those variants, so it is the eventual generation point.

Do not design the final syntax before the manual Mesh, MeshData, Material, Skel, and Skin conversions exist. The implementation must answer these questions first:

1. Can one cooked family implement `ToSourcePart<P>` for multiple destination `P` types?
2. How are unsupported cooked variants reported without requiring meaningless conversion implementations?
3. Can dispatch be generated without listing cooked variants anywhere outside `classes!`?
4. Does the destination mapping belong in `source_assets!`, a smaller source-part registry, or normal trait implementations?

Only generate the outer variant dispatch. Keep this code manual in version-specific cooked files:

```rust
impl TryFrom<Named<'_, NodeV1_06_63_02PC>> for NodeSourcePartBuild {
    type Error = Error;

    fn try_from(named: Named<'_, NodeV1_06_63_02PC>) -> Result<Self, Self::Error> {
        // Explicit semantic conversion.
    }
}
```

Do not add a macro around field conversion.

## Code That Must Remain Manual

- Source structs and their serialized shape.
- Build-only source-part result structs.
- Version/platform-specific `TryFrom<Named<'_, CookedVariant>>` conversions.
- Node, Mesh, Skel, and Skin assembly/grouping rules.
- Incoming versus outgoing dependency selection.
- Optional, required, and many cardinality decisions.
- Which cooked resources are represented by a successful source asset.
- Preserved fragments.
- Artifact decoding/encoding.
- Cooking transactions that emit multiple cooked resources.
- Fallback behavior.

These are semantic policy, not registry boilerplate.

## Prerequisites

Before macro implementation:

1. Complete the manual Mesh source path on PC and GameCube.
2. Complete Material/texture ownership sufficiently to distinguish reusable `MaterialSourcePart` types, single-owner Material instances, and shared texture references.
3. Add Skel and Skin so source-asset ordering and source-to-source lookup are real rather than hypothetical.
4. Move represented-resource collection into `SourceAssetBuild<T>` and commit it generically only after a successful build.
5. Inventory repeated code using the actual implementations.
6. Confirm module/type naming conventions are stable enough for `pastey` inference.

## Migration Order

1. Add generated `CookedClassFamily` metadata to `classes!`.
2. Replace repeated family lookup/error code with one generic helper.
3. Add `source_assets!` and migrate only the top-level `SourceAssetData`/builder registry.
4. Verify serialization/schema output is unchanged.
5. Reassess cooked variant dispatch using the accumulated manual conversions.
6. Generate only the dispatch shape proven safe by that inventory.
7. Remove superseded manual glue after each generated path is verified.

Do not land every macro change as one expansion. Each stage should compile and preserve source JSON behavior independently.

## Guardrails

- `classes!` remains the single source of truth for cooked families and variants.
- `source_assets!` declares only top-level source asset registration in v1.
- Use `ClassType` for source identity.
- No `SourceAssetType`.
- No cooked variant lists outside `classes!`.
- No version/platform matches in `bff/src/source`.
- No generated semantic field conversion.
- No generated assembly, dependency direction, cardinality, preservation, or represented-resource policy in v1.
- No one-family/one-part associated type unless later evidence proves it.
- Generated code must preserve the current `SourceAsset` wrapper and JSON/schema shape.

## Possible Later Expansion

After several source assets demonstrate genuinely identical grouping rules, reconsider:

- Declarative part requests.
- Declarative dependency direction and cardinality.
- Derive support for simple source-part project lookups.
- Generated assembly for only the simplest proven cases.

These are not part of the first macro implementation.
