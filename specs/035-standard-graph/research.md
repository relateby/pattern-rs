# Research: StandardGraph

**Feature**: 035-standard-graph
**Date**: 2026-03-15

## R1: StandardGraph in gram-hs Reference

**Decision**: StandardGraph is a Rust-native addition with no gram-hs equivalent.

**Rationale**: The gram-hs codebase (`../pattern-hs/libs/pattern/src/`) contains `PatternGraph`, `GraphClassifier`, `GraphQuery`, and `Algorithms` modules but no simplified/concrete graph type. In Haskell, type aliases and type class inference provide some of the ergonomic benefits that StandardGraph provides explicitly in Rust. Since StandardGraph wraps existing faithfully-ported types without changing behavior, it does not violate reference implementation fidelity.

**Alternatives considered**:
- Port a non-existent Haskell type — rejected (doesn't exist)
- Skip StandardGraph entirely — rejected (Rust's explicit type parameters create real friction that Haskell's type inference avoids)

## R2: Composition Strategy

**Decision**: StandardGraph wraps `PatternGraph<(), Subject>` via composition (private inner field), not inheritance or re-export.

**Rationale**: Composition provides a clean API boundary. StandardGraph methods delegate to PatternGraph internals but present a simpler interface. The `()` extra type parameter is fixed (unclassifiable patterns carry no metadata beyond their identity). Users who need custom `Extra` types or non-Subject value types use PatternGraph directly.

**Alternatives considered**:
- Type alias (`type StandardGraph = PatternGraph<(), Subject>`) — rejected (exposes all PatternGraph methods, including classifier/policy parameters)
- Newtype with Deref — rejected (leaks the inner type's full API)
- Trait-based abstraction — rejected (over-engineering for a concrete convenience type)

## R3: SubjectBuilder Location

**Decision**: SubjectBuilder is implemented in `subject.rs` as a companion to Subject, with a `Subject::build(identity)` entry point.

**Rationale**: The builder constructs Subject values and is tightly coupled to Subject's fields (identity, labels, properties). Placing it in `subject.rs` keeps related code together. The `Subject::build()` associated function is the idiomatic Rust entry point (like `Command::new()` in std).

**Alternatives considered**:
- Separate `graph/builder.rs` file — rejected (SubjectBuilder is independent of graph concepts)
- Standalone `SubjectBuilder::new(identity)` without Subject entry point — rejected (less discoverable)

## R4: Pattern Construction in Atomic Methods

**Decision**: `add_node`, `add_relationship`, `add_walk`, and `add_annotation` construct the correct `Pattern<Subject>` structure internally, then delegate to PatternGraph's existing merge/insert logic.

**Rationale**: The canonical classifier classifies by pattern shape (element count and structure). Atomic methods know the target classification upfront, so they construct patterns with the right shape and insert directly into the correct bucket — bypassing classification entirely. This is both simpler and avoids edge cases where the classifier might misclassify a manually-constructed pattern.

**Details**:
- `add_node(subject)` → `Pattern::point(subject)` → insert into `pg_nodes`
- `add_relationship(subject, source_id, target_id)` → `Pattern::pattern(subject, [source_pattern, target_pattern])` → insert into `pg_relationships` (source/target looked up or created as placeholder nodes)
- `add_walk(subject, rel_ids)` → `Pattern::pattern(subject, [rel_patterns...])` → insert into `pg_walks`
- `add_annotation(subject, element_id)` → `Pattern::pattern(subject, [element_pattern])` → insert into `pg_annotations`

**Alternatives considered**:
- Classify every pattern even in atomic mode — rejected (unnecessary overhead and potential misclassification)
- Bypass PatternGraph entirely, manage raw HashMaps — rejected (loses reconciliation and conflict tracking)

## R5: Placeholder Node Creation

**Decision**: When `add_relationship` or `add_walk` references a non-existent node/relationship identity, a minimal placeholder is created with that identity and empty labels/properties.

**Rationale**: This matches gram notation semantics where `(a)-[:KNOWS]->(b)` implicitly introduces nodes `a` and `b`. The existing `PatternGraph::merge` machinery already handles this — when inserting a relationship, it merges endpoint patterns into the node bucket. If the endpoint doesn't exist yet, the merge effectively inserts it.

**Alternatives considered**:
- Error on missing references — rejected (breaks gram notation compatibility and forces ordering-dependent construction)
- Require explicit node creation before relationship — rejected (poor ergonomics)

## R6: Neighbor and Degree Query Direction

**Decision**: Neighbor queries and degree counts consider both incoming and outgoing relationships (undirected view).

**Rationale**: Clarified during spec session. Most graph libraries (Neo4j, NetworkX) default to bidirectional view. The existing `GraphQuery::query_incident_rels` already provides both-direction behavior, and `GraphQuery::query_degree` counts all incident relationships.

**Alternatives considered**:
- Outgoing only — rejected (surprising for graph-people)
- Separate directional methods — rejected (deferred to future work; current scope is the simple case)

## R7: Error Handling Strategy

**Decision**: `from_gram` returns `Result<StandardGraph, ParseError>`. Atomic methods are infallible (return `&mut Self` for chaining). Classification failures go to "other" bucket, reconciliation failures go to "conflicts".

**Rationale**: The "segregate, don't reject" principle means construction never fails for data-quality reasons. Only parsing (gram notation syntax) can fail. This matches PatternGraph's existing behavior where `from_patterns` is infallible (always produces a graph, possibly with conflicts/other).

**Alternatives considered**:
- Return Result from all methods — rejected (most operations are infallible by design)
- Panic on conflicts — rejected (violates segregation principle)

## R8: GraphQuery Construction for StandardGraph

**Decision**: `as_query()` creates a `GraphQuery<Subject>` using the existing `graph_query::from_pattern_graph()` function, which takes an `Rc<PatternGraph<Extra, V>>`.

**Rationale**: The existing GraphQuery construction infrastructure handles all the closure creation (query_nodes, query_source, query_target, etc.). StandardGraph just needs to provide its inner PatternGraph wrapped in Rc. The `as_snapshot()` method uses `GraphView::from_pattern_graph()` with the canonical classifier.

**Alternatives considered**:
- Reimplement query closures directly — rejected (duplicates existing, tested code)
- Store a pre-built GraphQuery inside StandardGraph — rejected (would need to be rebuilt on every mutation)
