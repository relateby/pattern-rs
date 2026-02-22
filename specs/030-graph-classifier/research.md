# Research: Graph Classifier Port (030)

**Phase**: 0 — Outline & Research
**Date**: 2026-02-22
**Status**: Complete — all unknowns resolved

---

## Finding 1: `Pattern<V>` Structure (Confirmed)

**Decision**: Use the public fields `value: V` and `elements: Vec<Pattern<V>>` directly.

**Rationale**: The Rust `Pattern<V>` struct has both fields as `pub`, which exactly matches the Haskell `Pattern v` record accessors `value` and `elements`. The porting guide's assumptions about field names are correct.

**Confirmed paths**:
- Type definition: `crates/pattern-core/src/pattern.rs:122-134`
- Pub fields: `pub value: V`, `pub elements: Vec<Pattern<V>>`

---

## Finding 2: `Subject.identity` Type is `Symbol`, Not `String`

**Decision**: The `GraphValue` impl for `Subject` must use `Symbol` as `Id`, not `String`.

**Rationale**: The Haskell uses `Symbol` as `Id Subject`. In pattern-rs, `Subject.identity` is of type `Symbol(pub String)`, where `Symbol` already implements `Clone`, `PartialEq`, `Eq`, and `Hash`. The porting guide says "use `String` as `Id`" but this is wrong — `Symbol` must be used to match the Haskell type signature and to allow the identity field to be used as a `HashMap` key.

```rust
impl GraphValue for Subject {
    type Id = Symbol;
    fn identify(&self) -> &Symbol {
        &self.identity
    }
}
```

`Symbol` currently derives `Clone, PartialEq, Eq, Hash` but does NOT derive `Ord` or `PartialOrd`. **`Ord` is required** and must be added. Evidence from the Haskell reference:

1. **Superclass constraint on `GraphValue` itself**: `class Ord (Id v) => GraphValue v` — `Ord (Id v)` is a superclass, meaning every `GraphValue` instance unconditionally requires it. This is not a `PatternGraph`-specific detail; it is baked into the typeclass.

2. **`HasIdentity` also requires `Ord`**: `class Ord i => HasIdentity v i` — independently mandates `Ord` on the identity type.

3. **`PatternGraph` uses `Map (Id v)`** — Haskell's `Data.Map.Strict` is an ordered map (balanced BST) and requires `Ord` on its keys. All six collections in `PatternGraph` are `Map (Id v)`.

4. **Graph algorithms use `Set (Id v)` for visited tracking**: `bfs`, `dfs`, `shortestPath`, `topologicalSort`, `connectedComponents`, `hasCycle`, `minimumSpanningTree`, `degreeCentrality`, `betweennessCentrality` — all carry `(GraphValue v, Ord (Id v))` in their explicit signatures and use `Set.insert (identify (value n))`. `Data.Set` requires `Ord`.

5. **`Transform.hs` fold operations** use `Map (Id v)` for accumulation.

The earlier reasoning that "Rust uses `HashMap` so `Ord` is not required for this feature" was wrong in two ways: it ignored that `Ord` is a superclass of `GraphValue` (not just an incidental use), and it ignored that downstream graph operations (algorithms, transforms) will require `Set`-based visited tracking where `Ord` is essential. Using `HashSet` instead of `BTreeSet` could sidestep `Ord` for visited-set purposes, but the superclass constraint and `HasIdentity` requirement make `Ord` non-negotiable at the trait definition level.

**Correction to the plan**: `Symbol` MUST derive `Ord + PartialOrd`. The porting guide's explicit `type Id: Ord + Clone + Hash` bound is correct and must not be weakened.

**Alternative considered**: Using `String` directly. Rejected because it would diverge from the reference implementation's type, require converting `Symbol` to `String` at API boundaries, and break future identity-based operations that treat `Symbol` as the canonical id type.

---

## Finding 3: `Pattern.Reconcile` Does Not Exist in pattern-rs

**Decision**: Port `Pattern.Reconcile` as the first task within this feature, before implementing `PatternGraph`.

**Rationale**: `PatternGraph`'s merge and insert operations all require `ReconciliationPolicy`, `HasIdentity`, `Mergeable`, and `Refinable`. These types do not exist in pattern-rs today. The Haskell `Pattern.Reconcile` is a full module with 200+ lines. It must be ported as `src/reconcile.rs` (module `reconcile`) in the `pattern-core` crate.

**What exists today**: The `Combinable` trait and `FirstSubject`/`LastSubject`/`EmptySubject` wrappers in `lib.rs` partially overlap but are not equivalent. They will not be replaced — `Reconcile` and `Combinable` serve different abstraction levels. `Combinable` is value-level combination; `ReconciliationPolicy` is pattern-level deduplication policy.

**Scope decision**: Port only the subset of `Reconcile` used by `PatternGraph`:
- Traits: `HasIdentity`, `Mergeable`, `Refinable`
- Types: `ReconciliationPolicy<S>`, `ElementMergeStrategy`, `SubjectMergeStrategy`
- Function: `reconcile` (returns `Either<ReconcileError, Pattern<V>>` → `Result<Pattern<V>, ReconcileError>` in Rust)
- Subject instances of all three traits

The full `reconcileWithReport`, `collectByIdentity`, `findConflicts` utilities are nice-to-have and can be ported if scope allows, but are not required for `PatternGraph` to function.

---

## Finding 4: No `graph/` Module Exists in pattern-rs

**Decision**: Create `crates/pattern-core/src/graph/` as a new module directory containing `mod.rs` and `graph_classifier.rs`.

**Rationale**: The `graph/` directory does not exist yet. Creating it mirrors the Haskell module layout (`Pattern.Graph.GraphClassifier`) and provides a clean namespace for graph-related functionality (`GraphClass`, `GraphClassifier`, `GraphValue`, `classify_by_shape`, `canonical_classifier`, `from_test_node`).

**Alternative considered**: Placing all types in a flat `graph_classifier.rs` at `src/` level. Rejected because it does not mirror the Haskell module hierarchy and would make future graph modules (GraphLens, GraphQuery, GraphTransform) hard to organize.

---

## Finding 5: No New Cargo.toml Dependencies Required

**Decision**: No new external crate dependencies are needed for this feature.

**Rationale**: `PatternGraph` uses `std::collections::HashMap` (already in std). `GraphClassifier` uses `Box<dyn Fn(...)>` (std). All trait bounds (`Hash`, `Eq`, `Clone`) are in std. No serialization, async, or platform-specific code is introduced. WASM compatibility is guaranteed because all new code is pure std.

---

## Finding 6: Walk Decomposition is Recursive

**Decision**: When inserting a walk into `PatternGraph`, recursively merge each component relationship (which in turn merges its endpoint nodes). When inserting a relationship, merge its two endpoint nodes. When inserting an annotation, merge its inner element.

**Rationale**: The Haskell `insertRelationship` explicitly merges endpoint nodes first, `insertWalk` merges each component relationship (via `mergeWithPolicy`), and `insertAnnotation` merges its inner element. This means inserting a walk like `w = [r1=[A,B], r2=[B,C]]` into an empty graph results in: `pgWalks = {w}`, `pgRelationships = {r1, r2}`, `pgNodes = {A, B, C}`. This decomposition behavior is part of the behavioral contract and is tested in `PatternGraphSpec.hs`.

---

## Finding 7: `from_test_node` Not Exported from Haskell Module

**Decision**: Include `from_test_node` in the Rust module but do not prioritize it. It is a utility bridge for future `GraphLens` integration.

**Rationale**: The Haskell `GraphClassifier.hs` does not export `from_test_node` (it's not in the module's export list). The porting guide describes it as a future compatibility bridge. Since `GraphLens` does not yet exist in pattern-rs, this function will be added as a `pub(crate)` or `pub` function but is not part of the minimum viable port. It should still be implemented to match the porting guide.

---

## Finding 8: `Symbol` Needs `Ord` Derived for Compatibility

**Decision**: Check whether `Symbol` needs `Ord` added. If the porting guide and Haskell both require `Ord (Id v)`, then `Symbol` needs `#[derive(Ord, PartialOrd)]`.

**Rationale**: The `GraphValue` trait in the porting guide has `type Id: Ord + Clone + std::hash::Hash`. The Haskell class has `Ord (Id v) =>`. For correctness, `Symbol` must implement `Ord`. Currently `Symbol` only derives `Clone, PartialEq, Eq, Hash`. Adding `Ord` and `PartialOrd` requires a single `#[derive]` change to `Symbol` in `subject.rs`. String comparison is the natural ordering for `Symbol`. This is a safe, non-breaking change.

---

## Finding 9: Test Helper Pattern for Tests

**Decision**: Use `Pattern { value: Subject { identity: Symbol("x".to_string()), labels: HashSet::new(), properties: HashMap::new() }, elements: vec![] }` or write local `node(s)` and `rel(r, a, b)` helpers in each test file.

**Rationale**: The Haskell tests define `node :: Symbol -> Pattern Subject` and `rel :: Symbol -> Symbol -> Symbol -> Pattern Subject` as local helpers. Rust tests should do the same. The existing `test_utils/generators.rs` does not have graph-specific helpers; new test helpers are best defined locally in each test file rather than added to shared test utils (to avoid over-engineering).

---

## Summary: Resolved Decisions

| Unknown | Resolution |
|---------|------------|
| `Pattern<V>` field names | `value` and `elements` — confirmed public fields |
| `Subject` identity type | `Symbol`, not `String` |
| `ReconciliationPolicy` existence | Does not exist; must be ported as `src/reconcile.rs` |
| `graph/` module existence | Does not exist; must be created |
| New Cargo dependencies | None needed |
| Walk decomposition behavior | Recursive: walks → rels → nodes |
| `from_test_node` priority | Implement but not critical path |
| `Symbol` `Ord` bound | **Required** — superclass constraint on `GraphValue`, `HasIdentity`, and all graph algorithms. Add `#[derive(Ord, PartialOrd)]` to `Symbol`. |
