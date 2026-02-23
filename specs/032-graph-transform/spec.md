# Feature Specification: GraphTransform — View-Based Graph Transformations

**Feature Branch**: `032-graph-transform`  
**Created**: 2026-02-23  
**Status**: Draft  
**Depends on**: 030-graph-classifier, 031-graph-query  
**Does not depend on**: GraphLens (optional; see Clarifications). GraphTransform operates on GraphView; views can be built from PatternGraph now or from GraphLens when that feature is ported.  
**Input**: User description: "graph-transform as described in proposals/graph-transform-porting-guide.md and aligned with the behavior of ../pattern-hs/"

## Clarifications

### Session 2026-02-23

- Q: Does GraphLens (interpretive view of a Pattern as a graph structure) already exist in the Rust port, or will it need to be ported as well? → A: GraphLens does not exist in the Rust port; it must be ported as a separate feature (e.g. 026-graph-lens). This feature (032) defers the `from_graph_lens` constructor until that port exists; a documented placeholder is in scope.
- Q: Does ordering matter; should we hold off on graph-transform until after GraphLens has been ported? → A: No. In pattern-hs, GraphTransform (Transform.hs) depends only on GraphView; it does not import or use GraphLens. GraphView is built from either PatternGraph (toGraphView in PatternGraph.hs) or GraphLens (toGraphView in Graph.hs). GraphLens is an optional second source for GraphView. This feature can proceed using only the PatternGraph → GraphView path; GraphLens can be ported later and will simply add that second constructor.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Work with a Universal Graph View (Priority: P1)

A library user has a graph (e.g., a PatternGraph or a lens over one) and wants to run transformations and queries against it through a single, uniform interface. They do not want to copy the graph or depend on its storage format. They build a view from the graph (and a classifier), and all subsequent operations — mapping, filtering, folding, materializing — work over that view.

**Why this priority**: The graph view is the central abstraction. Without it, there is no single pipeline surface; transformations would be tied to concrete graph types. This story delivers the universal interface that all other stories depend on.

**Independent Test**: Build a view from a PatternGraph, run a trivial transformation (e.g., map-all with identity), materialize the result, and assert the output graph is equivalent to the input. Delivers the ability to treat any classified graph as a transformable view.

**Acceptance Scenarios**:

1. **Given** a PatternGraph and a graph classifier, **When** a developer constructs a graph view from the graph, **Then** the view exposes the same elements (nodes, relationships, walks, annotations, other) as the source, organized by classification.
2. **Given** a graph view, **When** a developer materializes it with a reconciliation policy, **Then** the result is a PatternGraph whose contents match the view and respect the policy for any identity collisions.
3. **Given** a graph view built from a graph lens (when available), **When** the same operations are applied as for a PatternGraph-backed view, **Then** behavior is equivalent; the view abstracts over the backing source.

---

### User Story 2 - Build a Graph from Seeds (Unfold) (Priority: P2)

A library user has a collection of seeds (e.g., database rows, API responses) and wants to produce a PatternGraph by expanding each seed into one or more patterns and merging the results. They do not want to hand-build graph structures; they supply an expansion function and a list of seeds, and receive a single graph.

**Why this priority**: Unfold is the primary way to construct graphs from external data (ETL). It avoids manual graph assembly and ensures a consistent merge policy. Without it, users must build graphs element-by-element and handle reconciliation themselves.

**Independent Test**: Provide a list of seeds and an expander that returns one node per seed. Unfold into a graph and assert the graph has one node per seed and that identities are stable. Delivers reproducible graph construction from external sources.

**Acceptance Scenarios**:

1. **Given** a single seed and an expander that returns one pattern, **When** the user runs unfold-graph with that seed, **Then** the resulting graph contains exactly that pattern, classified correctly.
2. **Given** multiple seeds and an expander that returns multiple patterns per seed (e.g., node + relationship + annotation), **When** the user runs unfold-graph, **Then** all patterns are merged into one graph according to the reconciliation policy; duplicate identities are resolved by the policy.
3. **Given** a recursive structure (e.g., tree of nodes), **When** the user runs the single-pattern unfold (expand seed to value and child seeds), **Then** the result is a single Pattern tree whose structure matches the expansion; deep hierarchies do not cause stack overflow (e.g., iterative implementation or documented depth limit).

---

### User Story 3 - Transform Elements by Category (Priority: P3)

A library user wants to apply different transformations to nodes, relationships, walks, annotations, and other elements. They do not want to write one function that branches on type; they want to supply a separate mapper per category (or use identity for categories they do not care about) and get a new view where each element is transformed accordingly.

**Why this priority**: Category-specific mapping is the most common transformation pattern (e.g., normalize nodes, leave relationships unchanged, strip annotations). It keeps transformation logic clear and avoids boilerplate type dispatch.

**Independent Test**: Build a view, apply map-graph with a mapper that uppercases a property on nodes only and leaves other categories unchanged. Materialize and assert nodes are transformed and relationships/annotations/walks are unchanged. Delivers predictable, category-scoped transformation.

**Acceptance Scenarios**:

1. **Given** a graph view and a set of category mappers (nodes, relationships, walks, annotations, other), **When** the user applies map-graph with those mappers, **Then** the resulting view contains transformed elements per category; elements in categories not overridden are unchanged (identity).
2. **Given** a graph view, **When** the user applies map-all-graph with a single function, **Then** every element in the view is transformed by that function regardless of category.
3. **Given** map-graph with identity for all categories, **When** applied to a view, **Then** the resulting view is equivalent to the input (no observable change after materialization).

---

### User Story 4 - Filter Elements by Predicate (Priority: P4)

A library user wants to restrict a graph view to elements that satisfy a predicate (e.g., only nodes with a certain label, or only relationships after a date). When an element is removed, they want a defined behavior for containers that held it: leave a gap, replace with a filler, or remove the container.

**Why this priority**: Filtering is essential for building subgraphs, cleaning data, and preparing views for algorithms. Substitution policy (gap vs. replace vs. remove container) must be explicit so users can reason about partial removal.

**Independent Test**: Build a view, apply filter-graph with a predicate that keeps only half the nodes and a chosen substitution policy. Materialize and assert the output contains only matching elements and that containers (e.g., walks) behave according to the policy. Delivers predictable subgraph extraction.

**Acceptance Scenarios**:

1. **Given** a graph view and a predicate over (classification, pattern), **When** the user applies filter-graph with a substitution policy, **Then** the resulting view contains only elements for which the predicate holds; elements that fail are removed.
2. **Given** a walk that contains a relationship that is filtered out, **When** substitution is "no substitution", **Then** the walk remains in the view with a gap where the relationship was.
3. **Given** the same situation with substitution "replace with filler", **Then** the walk remains with the filler pattern in place of the removed relationship.
4. **Given** the same situation with substitution "remove container", **Then** the walk is removed from the view.

---

### User Story 5 - Aggregate Over a Graph View (Fold) (Priority: P5)

A library user wants to compute a single value from a graph view — e.g., count elements per category, sum a property, or build a histogram. They supply an initial value and a function that combines the accumulator with each (classification, pattern) pair, and receive the final aggregated value.

**Why this priority**: Folding enables analytics and validation without materializing the graph. Users can count nodes, sum weights, or collect statistics in one pass. Lower priority than mapping and filtering because it does not produce a new view.

**Independent Test**: Build a view, apply fold-graph with an accumulator (e.g., map from category to count) and a function that increments the count for each element’s category. Assert the final counts match the number of elements per category in the view. Delivers one-pass aggregation over views.

**Acceptance Scenarios**:

1. **Given** a graph view, an initial accumulator value, and a fold function (accumulator, classification, pattern) -> accumulator, **When** the user runs fold-graph, **Then** the result is the accumulator after processing every element in the view exactly once.
2. **Given** a view with no elements, **When** the user runs fold-graph, **Then** the result is the initial accumulator unchanged.
3. **Given** a fold that builds a map (e.g., category -> count), **When** the user runs fold-graph, **Then** the result is a correctly populated map without requiring a commutative/associative combine; the fold order is defined and consistent.

---

### User Story 6 - Map with Full Graph Context (Priority: P6)

A library user wants to transform each element in a view using the rest of the graph as context — e.g., "set this node’s annotation count to the number of annotations attached to it." The mapping function receives both the element and a read-only query over the graph, so it can look up neighbors, annotations, or walks. All elements must be transformed against the same snapshot of the graph (no incremental updates during the pass).

**Why this priority**: Context-aware mapping supports enrichment and derived fields (counts, flags, summaries). Snapshot semantics keep behavior deterministic and avoid order-dependent bugs. Lower priority than category mapping because it is a more advanced use case.

**Independent Test**: Build a view with nodes that have annotations. Apply map-with-context with a function that, for each node, queries annotations attached to that node and sets a "count" property. Materialize and assert each node’s count equals the number of annotations for that node. Delivers enrichment using full graph context with stable semantics.

**Acceptance Scenarios**:

1. **Given** a graph view and a function (query, pattern) -> pattern, **When** the user applies map-with-context, **Then** each element is replaced by the result of the function called with a snapshot of the graph query and the element; the snapshot is the same for every element (no in-pass updates).
2. **Given** a mapping function that queries annotations of the current element, **When** map-with-context is applied, **Then** the query returns results from the original view, not from already-transformed elements.
3. **Given** map-with-context with identity (return pattern unchanged), **When** applied to a view, **Then** the resulting view is equivalent to the input.

---

### User Story 7 - Topology-Aware Folding (Paramorphism) (Priority: P7)

A library user wants to compute a value per element (e.g., a label or score) that depends on the values of neighboring elements — for example, PageRank or "depth from root." For DAGs, they want one pass in a well-defined order (e.g., topological); for general graphs with cycles, they want an iterative pass that repeats until a convergence condition is met.

**Why this priority**: Paramorphism supports graph algorithms (centrality, propagation, fixed-point iteration). It is the foundation for Pregel-style computation. Lowest priority in this feature because it builds on all previous view and query machinery.

**Independent Test**: Build a small DAG view, run para-graph with a function that sets each element’s result to one plus the max of its predecessors’ results. Assert root nodes get 1 and each node gets max(predecessors)+1. Delivers topology-aware per-element computation. For cyclic graphs, run para-graph-fixed with a convergence predicate and assert results stabilize.

**Acceptance Scenarios**:

1. **Given** a graph view and a function (query, pattern, list of neighbor results) -> result, **When** the user runs para-graph on a DAG, **Then** the result is a map from element identity to result, and each element’s result is computed after its predecessors (e.g., topological order).
2. **Given** a graph with cycles, **When** the user runs para-graph-fixed with a convergence predicate and initial value, **Then** the implementation iterates until the predicate holds for all elements (old result vs. new result), and the final map is returned.
3. **Given** para-graph with a function that ignores neighbor results, **When** run on any view, **Then** the result map has one entry per element identity in the view; no elements are dropped.

---

### Edge Cases

- What happens when materialize is given a view with no elements? The result is an empty PatternGraph (all collections empty), consistent with the reconciliation policy for zero elements.
- What happens when filter-graph removes all elements? The resulting view has no elements; materializing yields an empty graph.
- What happens when unfold-graph is given an empty list of seeds? The result is an empty PatternGraph.
- What happens when fold-graph is given a view with one element? The result is the accumulator after one application of the fold function to that element.
- What happens when map-with-context’s function performs a query that would see the current element? The snapshot includes the current element; behavior is defined and deterministic (e.g., self-loops in annotation queries are possible).
- What happens when para-graph is run on a cyclic graph without using para-graph-fixed? The result is defined but may depend on iteration order; the spec or implementation documents that para-graph on cycles is order-dependent and para-graph-fixed is the right choice for fixed-point semantics.
- What happens when a view is materialized twice (e.g., after cloning)? Each materialization is independent; both produce a PatternGraph from the same view contents and policy.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Library MUST provide a graph view abstraction that wraps a classified graph (PatternGraph or graph lens) and exposes its elements by category (nodes, relationships, walks, annotations, other) together with a single query interface over the same graph.
- **FR-002**: Library MUST provide a way to construct a graph view from a PatternGraph and a graph classifier.
- **FR-003**: Library MUST provide a way to construct a graph view from a graph lens and a graph classifier when GraphLens is ported (separate feature). Until then, this is deferred with a documented placeholder; GraphLens does not yet exist in pattern-rs.
- **FR-004**: Library MUST provide a materialize operation that consumes a graph view and a reconciliation policy and produces a PatternGraph; duplicate identities in the view are resolved according to the policy.
- **FR-005**: Library MUST provide an unfold operation that takes a seed and an expand function (seed -> value and list of child seeds) and produces a single Pattern tree; expansion must support deep structures without stack overflow (e.g., iterative implementation or documented depth limit).
- **FR-006**: Library MUST provide an unfold-graph operation that takes a list of seeds, an expand function (seed -> list of patterns), a classifier, and a reconciliation policy, and produces a PatternGraph; all expanded patterns are merged according to the policy.
- **FR-007**: Library MUST provide a map-graph operation that takes a graph view, a classifier, and per-category mappers (nodes, relationships, walks, annotations, other), and returns a new graph view whose elements are the result of applying the appropriate mapper to each element; categories not overridden behave as identity.
- **FR-008**: Library MUST provide a map-all-graph operation that takes a graph view and a single function, and returns a new graph view with every element transformed by that function.
- **FR-009**: Library MUST provide a filter-graph operation that takes a graph view, a classifier, a predicate (classification, pattern) -> bool, and a substitution policy; it returns a new graph view containing only elements that satisfy the predicate, with container behavior (gap, replace with filler, remove container) defined by the policy.
- **FR-010**: Library MUST provide a fold-graph operation that takes a graph view, an initial accumulator, and a function (accumulator, classification, pattern) -> accumulator, and returns the final accumulator after processing every element once; the operation does not consume the view (read-only).
- **FR-011**: Library MUST provide a map-with-context operation that takes a graph view, a classifier, and a function (query, pattern) -> pattern; each element is transformed by the function receiving a snapshot of the graph query (the same for all elements) and the element; the result is a new graph view.
- **FR-012**: Library MUST provide a para-graph operation that takes a graph view and a function (query, pattern, list of neighbor results) -> result, and returns a map from element identity to result; for DAGs, processing order must be defined (e.g., topological) so each element sees predecessor results.
- **FR-013**: Library MUST provide a para-graph-fixed operation that takes a graph view, a convergence predicate (old result, new result) -> bool, a function (query, pattern, list of neighbor results) -> result, and an initial result value; it iterates until the predicate holds for all elements and returns the final map from element identity to result.
- **FR-014**: Transformations that produce a new view (map-graph, map-all-graph, filter-graph, map-with-context) MUST consume the input view (single ownership) unless the implementation supports cloning the view for reuse; materialize MUST consume the view.
- **FR-015**: Behavior of all operations MUST align with the reference implementation in pattern-hs (../pattern-hs) for equivalent inputs and policies; equivalence tests or shared test data may be used to verify alignment.

### Key Entities

- **GraphView**: A universal graph-like interface that holds a query over the graph and a collection of (classification, pattern) pairs. It is the input and output of view-level transformations (map, filter, map-with-context) and the input to materialize and fold/para operations.
- **Materialize**: The operation that turns a graph view into an owned PatternGraph by applying a reconciliation policy to resolve duplicate identities.
- **ReconciliationPolicy**: A policy that determines how to merge elements with the same identity (e.g., last-write-wins); same notion as in the graph classifier / PatternGraph builder.
- **CategoryMappers**: A set of optional per-category functions (nodes, relationships, walks, annotations, other) used by map-graph; missing categories behave as identity.
- **Substitution**: The policy for container elements when a contained element is removed by filter-graph: no substitution (leave gap), replace with a filler pattern, or remove the container.
- **Unfold / UnfoldGraph**: Unfold expands a single seed into a Pattern tree via (seed -> value, child seeds). Unfold-graph expands a list of seeds into a list of patterns per seed and merges them into a PatternGraph with a classifier and reconciliation policy.
- **Map-with-context**: A transformation that passes each element and a snapshot of the graph query to a function; all elements see the same snapshot (no in-pass updates).
- **Para-graph / Para-graph-fixed**: Topology-aware folding that produces a result per element; para-graph for DAGs (single pass in defined order), para-graph-fixed for cyclic graphs (iterate until convergence).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A graph view built from a PatternGraph, after identity map and materialize, produces a PatternGraph that is equivalent to the source (same elements, same classifications, policy permitting).
- **SC-002**: Unfold-graph with N seeds and an expander that returns one pattern per seed produces a graph with exactly N patterns (after reconciliation); no elements are dropped or duplicated beyond policy.
- **SC-003**: Map-graph with identity for all categories leaves the view unchanged; after materialize, the output matches the input view.
- **SC-004**: Filter-graph with a predicate that keeps K elements and substitution "no substitution" produces a view that materializes to a graph with exactly K elements; walks that lose a relationship retain a gap as specified.
- **SC-005**: Fold-graph over a view with known category counts produces an accumulator (e.g., category -> count) that matches those counts.
- **SC-006**: Map-with-context with a function that sets a "count" property from a query over the snapshot produces nodes whose count equals the number of annotations (or other queried relation) in the snapshot; running the same test twice yields the same result (deterministic).
- **SC-007**: Para-graph on a DAG with a max-of-predecessors-plus-one function yields one result per element and root elements get the smallest value; para-graph-fixed on a small cyclic graph with a convergence predicate terminates and yields one result per element.
- **SC-008**: All transformation and materialization operations complete successfully for views with zero elements, one element, and many elements; edge cases (empty view, full filter-out, empty unfold seeds) do not crash and produce defined results.
- **SC-009**: Behavior matches the reference implementation (pattern-hs) for a shared set of test cases covering view construction, materialize, unfold, map-graph, filter-graph, fold-graph, map-with-context, and para-graph/para-graph-fixed.

## Assumptions

- The 030-graph-classifier and 031-graph-query features are complete and stable before this feature is implemented; GraphView depends on PatternGraph, GraphClassifier, and GraphQuery.
- The primary users are developers building ETL pipelines, graph transformations, or analytics over pattern graphs; "user" in this spec means a developer consuming the library.
- GraphLens does not exist in pattern-rs; it will be ported as a separate feature (e.g. 026-graph-lens). This feature defers the "from graph lens" constructor until then; a documented placeholder (e.g. todo! or unimplemented! with a comment) is in scope. Implementation order does not require GraphLens first: in pattern-hs, GraphTransform depends only on GraphView; GraphView is built from PatternGraph or (optionally) GraphLens, so 032 can proceed with PatternGraph-backed views only.
- Reconciliation policy and merge strategy types are those already defined in the graph classifier / PatternGraph context; this feature does not introduce new policy types.
- Snapshot semantics for map-with-context are mandatory; no variant that exposes incrementally updated state is required.
- Para-graph on cyclic graphs without para-graph-fixed may be order-dependent; documentation or spec will state that para-graph-fixed is the right tool for fixed-point semantics on cycles.
- Performance is correctness-first; eager allocation per transformation step is acceptable; iterator-based laziness is a future optimization if needed.
- Substitution policy (NoSubstitution, ReplaceWith, RemoveContainer) is sufficient for filter-graph; no additional policies are in scope.
