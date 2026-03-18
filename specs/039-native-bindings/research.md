# Research: Native TypeScript and Python Bindings

**Feature**: 039-native-bindings
**Date**: 2026-03-17
**Sources**: pattern-hs `../pattern-hs/libs/`, existing codebase exploration

---

## Decision 1: Value variant completeness

**Decision**: The native implementations require 10 Value variants, not 6 as sketched in the migration proposal.

**Authoritative Haskell source** (`libs/subject/src/Subject/Value.hs`):

```haskell
data Value
  = VInteger Integer
  | VDecimal Double
  | VBoolean Bool
  | VString String
  | VSymbol String
  | VTaggedString String String   -- (tag, content) e.g. url`https://...`
  | VArray [Value]                -- nested lists
  | VMap (Map String Value)       -- nested key-value maps
  | VRange RangeValue             -- numeric ranges: 1..10, 1..., ...10, ...
  | VMeasurement String Double    -- (unit, value) e.g. "kg" 5.0
```

**Rationale**: The migration proposal sketched only the 6 primitive types. The full gram notation supports structured values (arrays, maps), tagged strings, ranges, and measurements. Locking the Schema.Union to 6 variants now would require a breaking change to add the rest.

**Alternatives considered**: Treat advanced variants as opaque strings or skip them initially — rejected because existing Rust codec correctly parses and round-trips all 10 variants; a native implementation that silently drops them would fail correctness tests.

**Impact on proposal**: The `ValueSchema` and `Value` namespace in TypeScript, and the `Value` hierarchy in Python, must include all 10 variants. The `VRange` variant requires a sub-type (`RangeValue` with optional `lower` and `upper` bounds).

---

## Decision 2: Fold traversal order

**Decision**: Fold is pre-order (root processed before elements). The proposal's implementation sketch was correct.

**Authoritative Haskell source** (`libs/pattern/src/Pattern/Core.hs`):

```haskell
instance Foldable Pattern where
  foldMap f (Pattern v es) = f v <> foldMap (foldMap f) es
```

Traversal: root value → element[0] subtree → element[1] subtree → … → element[n] subtree.

**Rationale**: Pre-order confirms the proposed TypeScript `fold`:
```typescript
fn(init, p.value) then recurse into p.elements
```
is semantically correct.

---

## Decision 3: Comonad operations (extend/extract)

**Decision**: Both must be implemented. They are used in graph algorithms and are tested against comonad laws.

**Authoritative Haskell source**:

```haskell
extract (Pattern v _) = v                                    -- root value

duplicate p@(Pattern _ es) = Pattern p (map duplicate es)   -- pattern of subtrees

extend f p@(Pattern _ es) = Pattern (f p) (map (extend f) es)
-- f receives the full subtree at each position; result value replaces that position's value
```

**Comonad laws (property-tested in gram-hs)**:
- `extract . extend f == f`
- `extend extract == id`
- `extend f . extend g == extend (f . extend g)`

**Rationale**: `extend` is what makes "annotate every node with context-aware information" possible. Required for graph algorithms that propagate information downward through the tree.

**Impact on proposal**: The proposal's implementation sketch omitted `extend`, `extract`, and `duplicate`. All three must be included in the native TypeScript and Python implementations.

---

## Decision 4: StandardGraph classification — 5 classes, not 4

**Decision**: There are 5 graph element classes: Node, Relationship, Annotation, Walk, and Other.

**Authoritative Haskell source** (`libs/pattern/src/Pattern/Graph/GraphClassifier.hs`):

```haskell
data GraphClass extra
  = GNode           -- elements.length == 0 (atomic)
  | GAnnotation     -- elements.length == 1
  | GRelationship   -- elements.length == 2 AND both elements are GNode
  | GWalk           -- elements form a valid identity-chained walk
  | GOther extra    -- everything else
```

**Walk validation**: Consecutive relationship patterns must share endpoint node identities to form a valid chain. An invalid walk becomes `GOther`.

**Rationale**: The proposal mentioned 4 classes (nodes, relationships, walks, annotations). The Haskell source confirms Annotation is a distinct 5th class (a pattern with exactly one element, acting as a decorator on another pattern).

---

## Decision 5: JSON interchange format

**Decision**: Use the gram-hs JSON format as the authoritative interchange schema. The `"subject"` field key (not `"value"`) is used for the pattern's payload.

**Format from gram-hs** (`libs/gram/src/Gram/JSON.hs`):

```json
// Pattern<Subject>
{
  "subject": {
    "identity": "n",
    "labels": ["Person"],
    "properties": { "name": "Alice" }
  },
  "elements": [ ... ]
}
```

**Value JSON encoding**:
- Primitives as native JSON: integers → `42`, decimals → `3.14`, booleans → `true`, strings → `"text"`
- Symbol: `{"type": "symbol", "value": "myId"}`
- Tagged string: `{"type": "tagged", "tag": "url", "content": "https://..."}`
- Array: native JSON array `[...]`
- Map: native JSON object `{"key": value}` (without a `"type"` discriminant — distinguishable from Subject because it lacks `"identity"`)
- Range: `{"type": "range", "lower": 1.0, "upper": 10.0}` (bounds optional)
- Measurement: `{"type": "measurement", "unit": "kg", "value": 5.0}`

**Critical**: The proposal sketch used `"value"` as the pattern payload key; this is incorrect. The correct key is `"subject"`.

**Note**: The Rust gram-codec already exposes `parse_patterns_as_dicts` (Python) which returns dicts. The new `gram_parse_to_json` function should produce output consistent with this format. Verify alignment with `parse_patterns_as_dicts` output before finalizing the contract.

---

## Decision 6: effect peer dependency (TypeScript)

**Decision**: `effect` (>=3.0.0) is already listed as an optional peer dependency in `@relateby/pattern`'s `package.json`. No new dependency introduction is needed; it must be promoted from optional to required for the new native implementation.

**Rationale**: The package already anticipates effect usage. Promoting it to required aligns with the design and removes ambiguity.

---

## Decision 7: Python stdlib is sufficient

**Decision**: Python `dataclasses`, `typing`, and standard collection types (`set`, `dict`, `list`) are sufficient. No third-party Python libraries needed for the core data structures.

**Rationale**: The 10 Value variants, Subject with Set/Map fields, and Pattern recursion all map cleanly to `@dataclass` with standard types. No external dependency needed.

---

## Decision 8: gram-codec WASM standalone compatibility

**Decision**: Must verify that `crates/gram-codec` compiles for `wasm32-unknown-unknown` as a standalone crate (without `pattern-core` as a dependency in the WASM build).

**Current state**: The WASM build goes through `crates/pattern-wasm` which pulls in both `pattern-core` and `gram-codec`. After the migration, the WASM surface will only include gram-codec. This dependency path change needs verification.

**Rationale**: Constitution Principle IV (Multi-Target Library Design) requires WASM target support. The gram-codec crate currently depends on `pattern-core` for the `Pattern<Subject>` return type of `parse_gram()`. The new `gram_parse_to_json()` function returns a plain JSON string, which has no `pattern-core` dependency — but this needs to be confirmed in the build.

**Action**: Add `cargo build -p gram-codec --target wasm32-unknown-unknown` to the CI verification checklist for this feature.

---

## Decision 9: Existing Python dict serialization

**Decision**: `parse_patterns_as_dicts` in `crates/gram-codec/src/python.rs` already exists and returns `Vec<PyDict>`. This is the foundation for the new `gram_parse_to_json` approach. Phase 1 of implementation should verify its output matches the JSON interchange format and potentially expose a JSON-string variant.

**Rationale**: Avoids duplicating serialization logic. The existing dict output may only need a `serde_json::to_string()` wrapper on the Rust side to produce the JSON string needed by the WASM boundary.
