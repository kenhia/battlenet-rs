# Supplemental: Migrate Endpoint Modules to `bendpoint` Macro

**Branch**: `003-lib-wow-examples` | **Date**: 2026-04-10

## Problem

The `bendpoint` proc macro was designed to reduce boilerplate for endpoint structs — a single annotation replaces the `#[derive(Debug, Deserialize)]`, `pub type ...Result`, `pub type ...JsonResult`, and `impl GenerateUrl for ...` blocks. However, during Sprint 003 implementation only 2 of 171 endpoint structs use it. The remaining 169 are manually implemented with ~15–25 lines of boilerplate each.

### Root Cause

The macro's `StructField` parser uses `Punctuated<Ident, Colon>` to parse field types, which only handles `name: SimpleIdent`. It silently breaks on:

- **Generic types**: `Option<String>`, `Vec<NameAndId>`, `KeyAndId` (works), but `Option<Vec<NameAndId>>` does not
- **Field attributes**: `#[serde(alias = "_links")]` are dropped during re-emission
- **Complex paths**: any type that isn't a single `Ident`

These patterns appear in virtually every endpoint struct.

### Why This Matters

- **Cognitive load**: A `bendpoint`-annotated struct is ~5 lines. The manual equivalent is ~20 lines. Across 169 structs, that's ~2500 lines of pattern-identical boilerplate.
- **Refactorability**: Changing the URL construction scheme, error types, or namespace handling requires touching every manual `GenerateUrl` impl. With the macro, it's a one-line change in `model-macro/src/lib.rs`.
- **Consistency**: The macro enforces uniform patterns. Manual impls can drift.

## Solution

### Phase 1: Extend `bendpoint` Macro

The fix is to **stop re-parsing fields** and instead pass through the original `syn::Field` tokens. The current approach:

```rust
// Current: re-parses each field, losing attributes and generics
struct StructField { name: Ident, ty: Ident }
let builder_fields = fields.iter()
    .map(|f| syn::parse2::<StructField>(f.to_token_stream()).unwrap());
```

Replace with direct pass-through of `syn::Field`, ensuring `pub` visibility:

```rust
// New: preserve original field tokens (attributes, generics, paths, etc.)
let builder_fields = fields.iter().map(|f| {
    let attrs = &f.attrs;
    let name = &f.ident;
    let ty = &f.ty;
    quote! { #(#attrs)* pub #name: #ty }
});
```

This preserves:
- `#[serde(alias = "...")]` and any other field attributes
- `Option<T>`, `Vec<T>`, `Option<Vec<T>>`, and arbitrarily nested generics
- Full type paths like `crate::foo::Bar`

No changes needed to `input.rs` — the attribute parser (endpoint, namespace, url_args) already supports all 9 `UrlArgs` variants.

### Phase 2: Migrate Endpoint Structs

Convert all eligible manual endpoint structs to use `#[bendpoint(...)]`. Each conversion:

1. Removes `#[derive(Debug, Deserialize)]` (macro emits it)
2. Removes `pub type XxxResult = ...` and `pub type XxxJsonResult = ...` (macro emits them)
3. Removes `impl GenerateUrl for Xxx { ... }` (macro emits it)
4. Adds `#[bendpoint(endpoint = "..." url_args = "..." namespace = "...")]`
5. Removes `pub` from struct declaration (macro adds it)

**Before** (20 lines):
```rust
#[derive(Debug, Deserialize)]
pub struct Mount {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub source: Option<TypeAndName>,
    pub faction: Option<TypeAndName>,
}

pub type MountResult = Result<Mount, BattleNetClientError>;
pub type MountJsonResult = Result<String, BattleNetClientError>;

impl GenerateUrl for Mount {
    fn url(client: &BattleNetClient, url_args: &UrlArgs) -> String {
        let id = match url_args { UrlArgs::Id { id } => id, _ => panic!("UrlArgs::Id expected") };
        let endpoint = format!("data/wow/mount/{id}");
        let namespace = WowNamespace::Static.to_region_string(&client.region);
        let base = client.region.base_url();
        let locale = &client.locale;
        format!("{base}/{endpoint}?namespace={namespace}&locale={locale}")
    }
}
```

**After** (9 lines):
```rust
#[bendpoint(endpoint = "data/wow/mount/{id}" url_args = "Id" namespace = "static")]
struct Mount {
    #[serde(alias = "_links")]
    pub links: LinksRef,
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub source: Option<TypeAndName>,
    pub faction: Option<TypeAndName>,
}
```

### Note on `pygen`

The `pygen/gen_models.py` Python code generator predates `bendpoint` and generates full manual Rust files from YAML definitions. After this migration, `pygen`'s output should be updated to emit `#[bendpoint(...)]` annotated structs instead of manual `GenerateUrl` impls. This is a lower-priority follow-up — the immediate value is migrating the existing hand-written modules.

## Scope

### Eligible for Migration: ~160 endpoint structs across 47 files

Every manually-implemented endpoint struct with a `GenerateUrl` impl that follows the standard URL pattern is eligible. This includes structs with `Option<T>`, `Vec<T>`, `#[serde(alias = "...")]`, and all `UrlArgs` variants.

### Excluded from Migration: ~11 structs

| Struct | File | Reason |
|--------|------|--------|
| `SearchResult<AzeriteEssenceSearchData>` | azerite_essence.rs | `impl GenerateUrl for SearchResult<T>` — macro can't annotate a foreign generic |
| `SearchResult<ConnectedRealmSearchData>` | connected_realm.rs | Same — foreign generic |
| `SearchResult<CreatureSearchData>` | creature.rs | Same |
| `SearchResult<ItemSearchData>` | item.rs | Same |
| `SearchResult<JournalEncounterSearchData>` | journal.rs | Same |
| `SearchResult<MediaSearchData>` | media_search.rs | Same |
| `SearchResult<MountSearchData>` | mount.rs | Same |
| `SearchResult<RealmSearchData>` | realm.rs | Same |
| `SearchResult<SpellSearchData>` | spell.rs | Same |
| `CharacterProfile` | character_profile.rs | 35+ fields, doc comment block — already has manual impl alongside bendpoint `CharacterProfileStatus` |
| Core types in `core_structs.rs` | core_structs.rs | No `GenerateUrl` — these are shared data types, not endpoints |
| Core types in `core_other.rs` | core_other.rs | No `GenerateUrl` — helper types |

### Already Using `bendpoint`: 2 structs

- `WowTokenIndex` in `wow_token.rs`
- `CharacterProfileStatus` in `character_profile.rs`

## Tasks

### S001 — Extend `StructField` parser to preserve field attributes and generic types
- **File**: `model-macro/src/lib.rs`
- **Change**: Replace custom `StructField` parser with direct `syn::Field` token pass-through
- **Removes**: `struct StructField`, `impl ToTokens for StructField`, `impl Parse for StructField`
- **Validates**: `cargo test -p model-macro` + compile existing `wow_token.rs` and `character_profile.rs` usages

### S002 — Add `use model_macro::bendpoint;` import to all endpoint modules
- **Files**: All 47 endpoint module files in `src/wow_models/`
- **Change**: Add import line where missing (most files don't have it yet)
- **Validates**: `cargo check --all-features`

### S003 — Migrate Game Data index endpoints (UrlArgs::None, namespace static/dynamic)
- **Count**: ~34 structs
- **Files**: achievement.rs, auction_house.rs, azerite_essence.rs, connected_realm.rs, covenant.rs, creature.rs, guild.rs, heirloom.rs, item.rs, journal.rs, modified_crafting.rs, mount.rs, mythic_keystone.rs, pet.rs, playable_class.rs, playable_race.rs, playable_spec.rs, power_type.rs, profession.rs, pvp.rs, quest.rs, realm.rs, region_api.rs, reputation.rs, talent.rs, title.rs, toy.rs
- **Pattern**: Remove derive/type aliases/GenerateUrl impl, add `#[bendpoint(endpoint = "..." namespace = "...")]`
- **Validates**: `cargo test --all-features` (existing unit tests must still pass)

### S004 — Migrate Game Data detail endpoints (UrlArgs::Id)
- **Count**: ~78 structs
- **Files**: Same as S003
- **Pattern**: Same as S003 but with `url_args = "Id"`
- **Validates**: `cargo test --all-features`

### S005 — Migrate Game Data multi-param endpoints (UrlArgs::TwoIds, Guild, TwoStrings)
- **Count**: ~9 structs
- **Files**: item.rs, profession.rs, pvp.rs, talent.rs, guild.rs
- **Pattern**: Same as S003/S004 but with `url_args = "TwoIds"` / `url_args = "Guild"`
- **Validates**: `cargo test --all-features`

### S006 — Migrate Profile API endpoints (UrlArgs::Player, PlayerExtra; namespace profile)
- **Count**: ~26 structs
- **Files**: account_profile.rs, character_achievements.rs, character_appearance.rs, character_collections.rs, character_encounters.rs, character_equipment.rs, character_hunter_pets.rs, character_media.rs, character_mythic_keystone.rs, character_professions.rs, character_pvp.rs, character_quests.rs, character_reputations.rs, character_soulbinds.rs, character_specializations.rs, character_statistics.rs, character_titles.rs
- **Pattern**: Same as S003/S004 but with `url_args = "Player"` / `url_args = "PlayerExtra"` and `namespace = "profile"`
- **Validates**: `cargo test --all-features`

### S007 — Verify SearchResult endpoints remain functional (no migration)
- **Count**: 9 structs
- **Action**: Confirm all `impl GenerateUrl for SearchResult<T>` impls still compile and tests pass after macro changes
- **Validates**: `cargo test --all-features`

### S008 — Final validation and cleanup
- **Actions**:
  - `cargo fmt`
  - `cargo clippy --all-features`
  - `cargo test --all-features`
  - `cargo build --examples --all-features`
  - Remove any dead imports (`WowNamespace`, `BattleNetClient`, `UrlArgs` if no longer needed directly — though they still are for SearchResult impls)
  - Verify line count reduction
- **Validates**: Full quality gate passes

## Expected Impact

- **Lines removed**: ~2,500+ lines of boilerplate
- **Lines added**: ~160 `#[bendpoint(...)]` annotations + ~47 import lines
- **Net reduction**: ~2,300 lines
- **Behavioral change**: None — macro generates identical code to the manual impls
