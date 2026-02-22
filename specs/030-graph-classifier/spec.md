# Feature Specification: Graph Classifier Port

**Feature Branch**: `030-graph-classifier`
**Created**: 2026-02-22
**Status**: Draft
**Input**: User description: "Port graph-classifier from Haskell (located at ../pattern-hs) as described in proposals/graph-classifier-porting-guide.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Classify a Pattern by Shape (Priority: P1)

A library user has a `Pattern<V>` value and wants to know its structural role in a graph — is it a node, a relationship, an annotation, a walk, or something unrecognized? The user calls the standard shape classifier and gets back a named category without needing to write custom shape-detection logic.

**Why this priority**: Structural classification is the foundational primitive for everything else in this feature. All container operations depend on classification. Correct classification of nodes, relationships, annotations, and walks enables downstream graph operations.

**Independent Test**: Fully tested by calling the standard classifier on individual patterns of each shape and asserting the returned category matches the expected structural class.

**Acceptance Scenarios**:

1. **Given** a pattern with no sub-elements, **When** it is classified, **Then** the result is "node".
2. **Given** a pattern with exactly one sub-element, **When** it is classified, **Then** the result is "annotation".
3. **Given** a pattern with exactly two node-shaped sub-elements, **When** it is classified, **Then** the result is "relationship".
4. **Given** a sequence of properly chaining relationships (each consecutive pair shares an endpoint), **When** it is classified, **Then** the result is "walk".
5. **Given** a set of relationships sharing a center node (star pattern, not a chain), **When** it is classified, **Then** the result is "other" — not "walk".
6. **Given** a pattern with three or more bare node sub-elements, **When** it is classified, **Then** the result is "other".

---

### User Story 2 - Organize Patterns into a Typed Graph Container (Priority: P2)

A library user has a flat list of patterns (mixed nodes, relationships, annotations, walks) and wants to organize them into separate named collections for downstream processing. The user passes the list through a builder function and receives a typed container where each category is accessible by name.

**Why this priority**: The typed container (`PatternGraph`) is the primary artifact users interact with. Without it, classification is useful only for single-pattern inspection. The container unlocks bulk operations and graph traversal.

**Independent Test**: Build a `PatternGraph` from a known mixed list and assert that each expected pattern appears in the correct named collection and that no pattern is silently dropped.

**Acceptance Scenarios**:

1. **Given** an empty list of patterns, **When** a graph is built from it, **Then** all six named collections (nodes, relationships, annotations, walks, other, conflicts) are empty.
2. **Given** a list containing one node pattern and one relationship pattern, **When** a graph is built, **Then** the node is in the nodes collection and the relationship is in the relationships collection.
3. **Given** a list where two patterns share the same identity, **When** a graph is built with default policy, **Then** the second pattern replaces the first (last-write-wins).
4. **Given** a list where two patterns share the same identity and the reconciliation policy cannot choose a winner, **When** a graph is built, **Then** the conflict is recorded in the conflicts collection rather than silently dropped.
5. **Given** a list containing a walk pattern (chain of three relationships), **When** a graph is built, **Then** the walk appears in the walks collection.

---

### User Story 3 - Provide a Custom Domain Classifier (Priority: P3)

A library user wants to classify patterns according to domain-specific rules beyond the standard structural shapes. The user supplies their own classification function and builds a container that uses it. Custom categories are preserved alongside each pattern in the "other" collection, tagged with the user's category value.

**Why this priority**: Extensibility allows the library to serve specialized graph domains (e.g., RDF triples, hyperedges, labeled properties) without requiring core changes. Custom classifiers with typed tags make domain classification first-class.

**Independent Test**: Implement a custom classifier that emits a custom category for patterns matching a domain rule. Build a `PatternGraph` and verify that patterns classified as custom categories appear in the "other" collection with their tag intact.

**Acceptance Scenarios**:

1. **Given** a custom classifier that assigns a custom tag to patterns meeting a domain condition, **When** a graph is built, **Then** those patterns appear in the "other" collection with the correct tag.
2. **Given** a custom classifier and a pattern it does not recognize, **When** the graph is built, **Then** the unrecognized pattern appears with a fallback tag in the "other" collection.

---

### User Story 4 - Bridge Existing Node-Predicate Code to the Classifier (Priority: P4)

Existing code uses a simpler two-category predicate (is-node vs. not-node). The library must allow this predicate to be wrapped into a full classifier so existing code continues to work without rewriting its classification logic.

**Why this priority**: Backward compatibility with the `GraphLens` pattern. The bridge function keeps the migration path smooth for consumers already using single-predicate classification.

**Independent Test**: Wrap a node-predicate into a classifier. Verify that patterns satisfying the predicate are classified as "node" and all others as "other".

**Acceptance Scenarios**:

1. **Given** a predicate that identifies node-shaped patterns, **When** it is wrapped into a classifier and applied to a node pattern, **Then** the result is "node".
2. **Given** the same wrapped classifier applied to a non-node pattern, **Then** the result is "other".

---

### Edge Cases

- What happens when a walk has only one relationship? (A single relationship is classified as a relationship, not a walk — the walk case requires at least one element where all elements are relationship-shaped and chain correctly; with one element all-relationship-shaped, the chaining condition is vacuously valid, but two relationship-like elements are needed for a walk. Confirm: single relationship → GRelationship, not GWalk.)
- How does the system handle a pattern whose identity collides with an existing entry in the container? (Reconciliation policy determines outcome; default is last-write-wins; irreconcilable collisions go to conflicts.)
- What happens when a relationship-shaped pattern appears inside a walk that does not connect to the current chain? (The walk becomes invalid and the enclosing pattern falls to "other".)
- What happens when the classifier itself is a no-op (always returns "other")? (All patterns accumulate in the "other" collection with no panics or data loss.)

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The library MUST provide a classification vocabulary with five named structural categories: node, relationship, annotation, walk, and other (with an optional typed tag).
- **FR-002**: The library MUST provide a standard shape-based classifier that assigns categories based purely on structural shape (element count and nesting), with no reference to external state.
- **FR-003**: The standard classifier MUST correctly distinguish a valid walk (chaining relationships) from a star pattern (relationships sharing a center but not chaining end-to-end).
- **FR-004**: Walk validation MUST be direction-agnostic — a relationship contributes to the chain whether traversed left-to-right or right-to-left.
- **FR-005**: The library MUST provide an injectable classification abstraction that accepts any user-defined function as a classifier, enabling custom categorization without modifying library code.
- **FR-006**: The library MUST provide a typed container that accepts a list of patterns and a classifier, and distributes each pattern into the correct named collection.
- **FR-007**: The container MUST support six named collections: nodes, relationships, annotations, walks, other, and conflicts.
- **FR-008**: The "other" collection MUST store both the typed tag from the classifier and the original pattern together, preserving the tag for downstream use.
- **FR-009**: The container MUST support pluggable reconciliation policies for handling identity collisions.
- **FR-010**: Irreconcilable identity collisions MUST be recorded in the conflicts collection — patterns MUST NOT be silently dropped.
- **FR-011**: The library MUST provide a bridge function that converts a single node-predicate into a full two-category classifier (node vs. other), preserving backward compatibility with existing predicate-based code.
- **FR-012**: Every input pattern MUST appear in exactly one collection after construction — no pattern may be dropped, duplicated across collections, or trigger a panic.
- **FR-013**: The value type used with the container MUST expose a stable, comparable identity so the container can detect collisions and route patterns to the correct slot.
- **FR-014**: The library MUST provide a pre-built standard classifier instance that applies the shape-based classification rules out of the box.
- **FR-015**: The library MUST provide a construction function that accepts a classifier and an iterable of patterns and returns a fully populated container.
- **FR-016**: The library MUST provide an explicit-policy construction variant that accepts a classifier, a reconciliation policy, and an iterable of patterns.

### Key Entities

- **GraphClass**: The classification result — one of five named categories. The "other" variant carries an optional typed tag that the classifier supplies.
- **GraphClassifier**: A first-class classification strategy. Accepts a pattern and returns a `GraphClass`. Callers supply the strategy; the library supplies the infrastructure.
- **PatternGraph**: The eagerly materialized container that holds classified patterns in named collections. Parameterized by both the classifier's tag type and the pattern's value type.
- **ReconciliationPolicy**: A rule that determines which pattern wins when two patterns with the same identity are inserted. Default is last-write-wins.
- **GraphValue identity**: A stable, orderable, hashable key derived from a pattern's value. Used as the map key in all container collections.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All six test cases from the reference test suite for shape classification pass, producing results identical to the Haskell reference implementation.
- **SC-002**: All seven test cases from the reference test suite for `PatternGraph` construction pass, producing containers with correct collection membership for every input.
- **SC-003**: The custom classifier test passes: a user-defined classifier with a typed tag routes patterns to the "other" collection with the correct tag value preserved.
- **SC-004**: The total-classification invariant holds for all test inputs — every input pattern appears in exactly one collection after construction, with no panics and no silent drops.
- **SC-005**: Walk classification correctly rejects star patterns (non-chaining relationships) and accepts only valid linear chains, matching the Haskell reference behavior for all test cases.
- **SC-006**: The feature compiles cleanly for native, WebAssembly, and Python binding targets with no new warnings or errors.

## Assumptions

- The `Pattern<V>` type already exists in pattern-rs with `value: V` and `elements: Vec<Pattern<V>>` fields. No changes to `Pattern<V>` itself are needed.
- A `ReconciliationPolicy` type or equivalent mechanism already exists in pattern-rs, or will be introduced as part of this feature. If it does not exist, a minimal last-write-wins default is sufficient for the initial port.
- The `Subject` value type already exists in pattern-rs with a string identity field. The `GraphValue` identity implementation for `Subject` uses that field directly.
- The porting guide's "out of scope" exclusions are honored: `to_graph_view`, `materialize`, and `from_pattern_graph` (GraphQuery bridge) are not included in this feature.
- Behavioral equivalence with the Haskell reference implementation is the primary correctness target. API surface follows Rust idioms.
