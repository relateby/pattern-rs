# Quickstart: Graph Classifier Port (030)

**Audience**: Developers implementing this feature (pattern-rs contributors)
**Date**: 2026-02-22

---

## What Gets Built

This feature adds three capabilities to `pattern-core`:

1. **Classify any `Pattern<V>`** by structural shape → `GraphClass` enum
2. **Build a typed graph container** (`PatternGraph`) from a list of patterns
3. **Plug in custom classifiers** with domain-specific tags

---

## Implementation Order

```
Step 1: reconcile.rs   ← prerequisite for PatternGraph
Step 2: subject.rs     ← add Ord derive to Symbol
Step 3: graph/         ← GraphClass, GraphValue, GraphClassifier, classify_by_shape
Step 4: pattern_graph  ← PatternGraph, merge, from_patterns
Step 5: Tests          ← port all tests from Haskell spec
Step 6: lib.rs exports ← wire up public API
```

---

## Step 1: Port `Pattern.Reconcile`

**File to create**: `crates/pattern-core/src/reconcile.rs`
**Reference**: `../pattern-hs/libs/pattern/src/Pattern/Reconcile.hs`

Port these items:
- Trait `HasIdentity<V, I>`
- Trait `Mergeable` with associated `MergeStrategy`
- Trait `Refinable`
- Enum `ReconciliationPolicy<S>` with variants `LastWriteWins`, `FirstWriteWins`, `Merge(ElementMergeStrategy, S)`, `Strict`
- Enum `ElementMergeStrategy`
- Struct `SubjectMergeStrategy` with `label_merge` and `property_merge`
- Enums `LabelMerge`, `PropertyMerge`
- Instances: `HasIdentity<Subject, Symbol>`, `Mergeable for Subject`, `Refinable for Subject`
- Function `reconcile<V>(policy, pattern) -> Result<Pattern<V>, ReconcileError>`
- Error type `ReconcileError`

Add `pub mod reconcile;` to `lib.rs`.

---

## Step 2: Add `Ord` to `Symbol`

**File to modify**: `crates/pattern-core/src/subject.rs`

Change:
```rust
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Symbol(pub String);
```

To:
```rust
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Symbol(pub String);
```

Run `cargo test --workspace` to confirm nothing breaks.

---

## Step 3: Create the `graph/` Module

**Files to create**:
- `crates/pattern-core/src/graph/mod.rs`
- `crates/pattern-core/src/graph/graph_classifier.rs`

`mod.rs` re-exports from `graph_classifier`. `graph_classifier.rs` contains:

- `GraphClass<Extra>` enum with `map_other` impl
- `GraphValue` trait
- `GraphClassifier<Extra, V>` struct with `new()` constructor
- Private functions `is_node_like`, `is_relationship_like`, `is_valid_walk`
- Public function `classify_by_shape`
- Public function `canonical_classifier`
- Public function `from_test_node`

**Walk validation reference** (see `data-model.md`): Frontier algorithm, direction-agnostic, checks `identify()` equality not structural equality.

Add `pub mod graph;` to `lib.rs`.

---

## Step 4: Create `pattern_graph.rs`

**File to create**: `crates/pattern-core/src/pattern_graph.rs`

Implement `PatternGraph<Extra, V>` struct, `empty()`, and the insert functions:
- `insert_node` — reconcile collision or push to `pg_conflicts`
- `insert_relationship` — merge endpoint nodes first, then insert
- `insert_walk` — merge component relationships first (they bring their nodes), then insert
- `insert_annotation` — merge inner element first, then insert
- `insert_other` — reconcile collision or push to `pg_conflicts`

Wire through `merge_with_policy` dispatching to these private insert functions. `merge` defaults to `LastWriteWins`. `from_patterns` folds `merge`. `from_patterns_with_policy` folds `merge_with_policy`.

**Reconciliation pattern** (twoOccurrences):
```rust
// When a collision is found:
let synthetic = Pattern { value: existing.value.clone(), elements: vec![incoming] };
match reconcile(&policy, &synthetic) {
    Ok(merged) => { /* update map */ }
    Err(_) => { /* push incoming to pg_conflicts */ }
}
```

Add `pub mod pattern_graph;` to `lib.rs`.

---

## Step 5: Port Tests

**Files to create**:
- `crates/pattern-core/tests/graph_classifier.rs`
- `crates/pattern-core/tests/pattern_graph.rs`

**Test helpers** (define locally in each file):
```rust
fn node(s: &str) -> Pattern<Subject> {
    Pattern { value: Subject { identity: Symbol(s.to_string()), labels: HashSet::new(), properties: HashMap::new() }, elements: vec![] }
}

fn rel(r: &str, a: &str, b: &str) -> Pattern<Subject> {
    Pattern { value: Subject { identity: Symbol(r.to_string()), ..Default::default() }, elements: vec![node(a), node(b)] }
}
```

**Required test cases** (from Haskell reference — all must pass):

### `graph_classifier.rs`

1. Atomic pattern (0 elements) → `GNode`
2. Pattern with 1 element → `GAnnotation`
3. Pattern with 2 node elements → `GRelationship`
4. Chain `r1=[A,B], r2=[B,C], r3=[D,C]` → `GWalk` (direction-agnostic)
5. Star `r1=[A,B], r2=[A,C], r3=[A,D]` → `GOther(())`
6. Pattern with non-node element inside relationship → `GOther(())`
7. Pattern mixing relationships and nodes → `GOther(())`
8. `canonical_classifier.classify(n)` == `classify_by_shape(n)` for all shapes

### `pattern_graph.rs`

1. `PatternGraph::empty()` — all six maps have size 0
2. Merge a node → appears in `pg_nodes`, size 1
3. Merge a relationship → appears in `pg_relationships`; its endpoints in `pg_nodes`
4. `from_patterns` with mixed list → correct counts in each collection
5. Pattern with 3 node elements → appears in `pg_other`
6. Custom classifier with typed tag → `pg_other` stores `(DomainHyperedge, pattern)`
7. Duplicate identity with `LastWriteWins` → single entry in map
8. Merge a walk with 2 chaining relationships → `pgWalks=1`, `pgRelationships=2`, `pgNodes=3`

---

## Step 6: Wire Up `lib.rs` Exports

Add to `crates/pattern-core/src/lib.rs`:

```rust
pub mod graph;
pub mod reconcile;
pub mod pattern_graph;

pub use graph::graph_classifier::{
    GraphClass, GraphClassifier, GraphValue,
    classify_by_shape, canonical_classifier, from_test_node,
};
pub use reconcile::{
    ReconciliationPolicy, ElementMergeStrategy,
    HasIdentity, Mergeable, Refinable,
};
pub use pattern_graph::{
    PatternGraph,
    merge as pg_merge,
    merge_with_policy as pg_merge_with_policy,
    from_patterns,
    from_patterns_with_policy,
};
```

---

## Validation Checklist

After implementation, run:

```bash
# Format
cargo fmt --all

# Lint (zero warnings)
cargo clippy --workspace -- -D warnings

# All tests (native)
cargo test --workspace

# WASM build (no blocking I/O or filesystem in new code)
cargo build --workspace --target wasm32-unknown-unknown

# Full CI
./scripts/ci-local.sh
```

All must pass before the feature is complete.

---

## Out of Scope (Do Not Implement)

- `to_graph_view` / `materialize` — belongs to GraphTransform
- `from_pattern_graph` returning `GraphQuery` — belongs to GraphQuery
- Python bindings for new types — can be added in a follow-up feature
- WASM bindings for new types — can be added in a follow-up feature
