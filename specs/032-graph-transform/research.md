# Research: GraphTransform Port (032-graph-transform)

**Date**: 2026-02-23  
**Branch**: `032-graph-transform`  
**Reference**: `proposals/graph-transform-porting-guide.md`, `../pattern-hs` (GraphTransform / GraphView / Transform modules)

---

## Implementation order: GraphLens not required first

In pattern-hs, `Pattern.Graph.Transform` depends only on `GraphView`; it does not import or use `GraphLens`. `GraphView` is constructed from either `PatternGraph` (`Pattern.PatternGraph.toGraphView`) or `GraphLens` (`Pattern.Graph.toGraphView`). This feature can be implemented using only the PatternGraph → GraphView path; GraphLens can be ported later as a separate feature and will add the second constructor only.

---

## Decision 1: GraphView without lifetime (owned GraphQuery)

**Decision**: `GraphView<Extra, V>` has no lifetime parameter. `GraphQuery<V>` is owned by the view (e.g. `Rc`/`Arc`-wrapped closures from 031-graph-query), so the view does not borrow storage.

**Rationale**: Per porting guide, using the same choice as 031 (owned GraphQuery) avoids threading a lifetime through every transformation. Pipeline composition (view → map → filter → materialize) stays simple and does not require a single long-lived borrow.

**Alternatives considered**:
- `GraphView<'a, Extra, V>` with `GraphQuery<'a, V>` — rejected: complicates function signatures and chaining across transformation steps.

---

## Decision 2: materialize and view-consuming transforms consume the view

**Decision**: `materialize` and all view-producing transforms (`map_graph`, `map_all_graph`, `filter_graph`, `map_with_context`) take `GraphView` by value (move). Callers that need to reuse a view must clone it before calling.

**Rationale**: Matches porting guide and common Rust style: one logical owner per view; after materialize the view is no longer needed. Clone is explicit and rare.

**Alternatives considered**:
- Take `&GraphView` and return a new view while keeping the old — rejected: would require cloning inside the transform anyway to build new view_elements; ownership is clearer.

---

## Decision 3: unfold — iterative with explicit work stack

**Decision**: The single-pattern `unfold(expand, seed)` is implemented iteratively using a `Vec`-based work stack, not recursion.

**Rationale**: Deep hierarchies (e.g. large trees) would overflow the stack with a naive recursive implementation. The porting guide recommends the iterative approach for production. The public API is unchanged.

**Alternatives considered**:
- Recursive implementation with `#[allow(clippy::only_used_in_recursion)]` and a documented depth limit — rejected: iterative is safer and avoids stack overflow for arbitrary depth.

---

## Decision 4: map_graph — CategoryMappers struct with struct-update

**Decision**: Per-category mappers are passed via a `CategoryMappers<Extra, V>` struct (nodes, relationships, walks, annotations, other). Callers use `CategoryMappers::identity()` and override only the categories they need via struct update (`..CategoryMappers::identity()`).

**Rationale**: The Haskell API uses five positional function arguments; in Rust that forces identity for every unused category. A struct with a default identity and struct-update is idiomatic and matches the porting guide.

**Alternatives considered**:
- Five optional `Option<Fn(...)>` parameters — rejected: call site remains noisy. Builder pattern — acceptable but struct-update is simpler to implement and use.

---

## Decision 5: fold_graph — explicit init + fold function (Option A)

**Decision**: `fold_graph` takes an initial accumulator value and a function `(acc, &GraphClass, &Pattern) -> acc`. No `Monoid`/`Default + Add` constraint.

**Rationale**: HashMap-based folds (e.g. count-by-class) are common and require neither `Default` nor `Add` for the accumulator. Option A is strictly more general; the porting guide recommends it.

**Alternatives considered**:
- Option B: `M: Default + Add` — rejected: excludes HashMap and other non-Add accumulators.

---

## Decision 6: filter_graph — predicate takes references; Substitution enum

**Decision**: The filter predicate has signature `(&GraphClass<Extra>, &Pattern<V>) -> bool`. Container behavior when a contained element is removed is specified by `Substitution<V>`: `NoSubstitution`, `ReplaceWith(Pattern<V>)`, or `RemoveContainer`.

**Rationale**: Filtering only reads elements; references avoid unnecessary clones. Substitution is a required part of the Haskell design and must be first-class in the API.

**Alternatives considered**:
- Predicate takes ownership — rejected: filter does not need to consume elements.

---

## Decision 7: map_with_context — snapshot semantics and Pattern by value

**Decision**: The mapping function receives `(&GraphQuery<V>, Pattern<V>) -> Pattern<V>`. The `GraphQuery` reference is a snapshot of the view’s query taken before any element is transformed; every element sees the same snapshot. The pattern is passed by value (consumed).

**Rationale**: Snapshot semantics keep behavior deterministic and avoid order-dependent bugs. Passing the pattern by value matches “produce new element from old” without forcing a clone at the call site when not needed.

**Alternatives considered**:
- Incremental query (updated as elements are transformed) — rejected: spec and porting guide require snapshot semantics.

---

## Decision 8: para_graph and para_graph_fixed — &GraphView, R: Clone for fixed

**Decision**: Both take `&GraphView` (read-only). `para_graph_fixed` requires `R: Clone` so the convergence loop can compare previous and next round. `para_graph` on DAGs uses a defined order (e.g. topological); on cyclic graphs the result may be order-dependent and para_graph_fixed is the right tool.

**Rationale**: Per porting guide. Para does not consume the view; Clone on R is necessary to implement the fixpoint loop without complex borrowing.

**Alternatives considered**:
- Consume view — rejected: para is read-only. Avoid R: Clone by storing references — rejected: convergence needs to compare two full maps of R values.

---

## Decision 9: Eager Vec per transformation step

**Decision**: View elements are stored and produced as `Vec<(GraphClass<Extra>, Pattern<V>)>`. Each transformation allocates a new Vec; no iterator-based laziness in the first implementation.

**Rationale**: Porting guide recommends starting with eager allocation: correct, simple, easy to profile. Laziness can be added later if profiling shows benefit.

**Alternatives considered**:
- Box<dyn Iterator> or chained iterators — rejected for v1: more complex types and harder to debug; can revisit after profiling.

---

## Decision 10: impl Fn for per-element callbacks; Box<dyn Fn> only where stored

**Decision**: Transformation APIs use `impl Fn(...)` for one-off callbacks (e.g. in `map_all_graph`, `filter_graph`, `map_with_context`) so the compiler can monomorphize and inline. `Box<dyn Fn>` (or similar) is used only where the function must be stored (e.g. inside `CategoryMappers`).

**Rationale**: Porting guide: “prefer impl Fn … over Box<dyn Fn> for the per-element callbacks”. Improves performance without changing semantics.

---

## Decision 11: from_graph_lens deferred

**Decision**: `from_graph_lens` is not implemented in this feature. A placeholder (e.g. `todo!()` or `unimplemented!()` with a comment) is added where the constructor would live; it will be implemented when GraphLens is ported.

**Rationale**: Spec FR-003 and 031-graph-query plan state that GraphLens does not yet exist in pattern-rs. Deferring is consistent and documented.

---

## Decision 12: Module layout (Haskell → Rust)

**Decision**: Follow the porting guide table:

- `Pattern.Core.unfold` → `pattern_core::pattern::unfold` (or `pattern::unfold`)
- `Pattern.Graph.GraphView` / materialize / fromPatternGraph / fromGraphLens → `graph::graph_view` (and re-exports from `pattern_graph` if desired for naming)
- `Pattern.Graph.Transform.*` → `graph::transform::*` (map_graph, filter_graph, fold_graph, map_with_context, para_graph, para_graph_fixed, unfold_graph)

**Rationale**: Keeps graph-related types under `graph/` and pattern-tree operations under `pattern/`; aligns with existing layout (graph_classifier, graph_query, algorithms).

---

## Summary table (from porting guide)

| Topic | Choice |
|-------|--------|
| GraphView lifetime | No `'a`; owned GraphQuery |
| materialize | Consumes view |
| unfold | Iterative with work stack |
| map_graph | CategoryMappers struct + struct-update |
| fold_graph | Explicit init + fold function |
| filter_graph | Predicate `&GraphClass`, `&Pattern`; Substitution enum |
| map_with_context | Snapshot query; f takes Pattern by value |
| para_graph_fixed | R: Clone; &GraphView |
| Laziness | Eager Vec per step |
| Inlining | impl Fn for callbacks; #[inline] on transform fns |
| GraphClass | Clone, Debug, PartialEq, Eq, Hash (already in 030) |
