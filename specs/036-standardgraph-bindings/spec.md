# Feature Specification: StandardGraph TypeScript/WASM and Python Bindings

**Feature Branch**: `036-standardgraph-bindings`
**Created**: 2026-03-15
**Status**: Draft
**Input**: User description: "TypeScript/WASM and Python bindings for StandardGraph as described in proposals/ts-py-standard-graph-proposal.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Build and Query a Graph in TypeScript (Priority: P1)

A TypeScript developer wants to programmatically construct a graph by adding nodes and relationships one at a time, then query the graph for neighbors and element details. They create a `StandardGraph`, add nodes and relationships using a fluent builder for subjects, and retrieve neighbors of a given node.

**Why this priority**: TypeScript/WASM is the highest-value target. Element-by-element graph construction and neighbor queries are the most common operations, and no graph-building capability currently exists beyond the immutable `fromPatterns` path.

**Independent Test**: Can be fully tested by creating a graph with 3 nodes and 2 relationships in TypeScript, querying neighbors, and verifying correct results. Delivers immediate value for any TypeScript consumer of the library.

**Acceptance Scenarios**:

1. **Given** an empty StandardGraph, **When** the user adds two nodes and one relationship between them, **Then** `nodeCount` returns 2, `relationshipCount` returns 1, and `neighbors(sourceId)` returns the target node's pattern.
2. **Given** a StandardGraph with nodes and relationships, **When** the user queries `source(relId)` and `target(relId)`, **Then** the correct endpoint patterns are returned.
3. **Given** a StandardGraph, **When** the user chains `addNode` and `addRelationship` calls, **Then** each call returns the graph instance for fluent chaining.

---

### User Story 2 - Parse Gram Notation into a Graph in TypeScript (Priority: P1)

A TypeScript developer wants to create a graph from a gram notation string (e.g., `(a:Person)-[:KNOWS]->(b:Person)`) without manually constructing each element.

**Why this priority**: `fromGram` is a critical shortcut that enables rapid graph construction from a human-readable notation. It is equally important as manual construction for onboarding and prototyping.

**Independent Test**: Can be fully tested by parsing a gram string and verifying the resulting graph has the expected nodes, relationships, labels, and properties.

**Acceptance Scenarios**:

1. **Given** a valid gram notation string, **When** the user calls `StandardGraph.fromGram(input)`, **Then** a StandardGraph is returned with the correct nodes, relationships, and properties.
2. **Given** an invalid gram notation string, **When** the user calls `StandardGraph.fromGram(input)`, **Then** an error is thrown with a descriptive message.

---

### User Story 3 - Build and Query a Graph in Python (Priority: P2)

A Python developer wants to construct a graph programmatically and query it. They use `StandardGraph` with snake_case methods, adding nodes and relationships and querying neighbors.

**Why this priority**: Python users currently have no graph capability at all. This unlocks graph functionality for the entire Python audience, but is prioritized after TypeScript because TypeScript already has partial graph bindings to build upon.

**Independent Test**: Can be fully tested by creating a graph in Python with nodes and relationships, querying neighbors and counts, and verifying correctness.

**Acceptance Scenarios**:

1. **Given** an empty StandardGraph in Python, **When** the user adds nodes and relationships using `add_node` and `add_relationship`, **Then** counts and neighbor queries return correct results.
2. **Given** a StandardGraph in Python, **When** the user calls `from_gram` with a valid gram string, **Then** the graph is correctly populated.
3. **Given** an invalid gram string in Python, **When** `from_gram` is called, **Then** a `ValueError` is raised with a descriptive message.

---

### User Story 4 - Fluent Subject Construction (Priority: P2)

A developer (TypeScript or Python) wants to construct Subject values using a fluent builder pattern: `Subject.build("id").label("Person").property("name", "Alice").done()`.

**Why this priority**: The builder pattern eliminates the boilerplate of constructing Subject objects with labels and properties. It is essential for ergonomic graph construction but depends on the graph type being available first.

**Independent Test**: Can be fully tested by building a Subject with multiple labels and properties, then verifying all attributes are correctly set.

**Acceptance Scenarios**:

1. **Given** a SubjectBuilder, **When** the user chains `.label()` and `.property()` calls followed by `.done()`, **Then** a Subject is returned with all specified labels and properties.
2. **Given** a SubjectBuilder in TypeScript, **When** methods are chained, **Then** each method returns the builder for continued chaining (not consuming the object).

---

### User Story 5 - Access Existing Graph Query Interface from StandardGraph (Priority: P3)

A TypeScript developer wants to use the existing `NativeGraphQuery` interface (for algorithms like BFS, shortest path) starting from a StandardGraph, by calling `asQuery()`.

**Why this priority**: This is an escape hatch that bridges StandardGraph to the existing algorithm functions. Important for advanced users but most users will not need it initially.

**Independent Test**: Can be fully tested by creating a StandardGraph, calling `asQuery()`, and verifying the returned object is a valid `NativeGraphQuery` that can be passed to existing algorithm functions.

**Acceptance Scenarios**:

1. **Given** a populated StandardGraph in TypeScript, **When** the user calls `asQuery()`, **Then** a `NativeGraphQuery` is returned that reflects the graph's current state.
2. **Given** a populated StandardGraph in TypeScript, **When** the user calls `asPatternGraph()`, **Then** a `NativePatternGraph` is returned containing all elements.

---

### Edge Cases

- What happens when querying neighbors of a node that doesn't exist in the graph? Returns an empty list.
- What happens when adding a relationship referencing source/target nodes not yet in the graph? The relationship is added following existing Rust StandardGraph behavior.
- What happens when adding a node with an identity that already exists? The existing node is updated via the standard reconciliation/merge policy (combining labels and properties).
- What happens when parsing an empty gram string? Returns an empty graph.
- What happens when querying `degree` of a non-existent node? Returns 0.
- What happens when calling `source`/`target` on a non-existent relationship? Returns `undefined` (TypeScript) or `None` (Python).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The library MUST expose a `StandardGraph` class in TypeScript/WASM bindings with constructor, `fromGram`, `fromPatterns`, and `fromPatternGraph` static factory methods.
- **FR-002**: The library MUST expose `addNode`, `addRelationship`, `addWalk`, `addAnnotation`, `addPattern`, and `addPatterns` mutating methods on the WASM `StandardGraph`, each returning the graph instance for chaining.
- **FR-003**: The library MUST expose element access methods (`node`, `relationship`, `walk`, `annotation`) returning the pattern or `undefined` on the WASM `StandardGraph`.
- **FR-004**: The library MUST expose iteration getters (`nodes`, `relationships`, `walks`, `annotations`) returning arrays on the WASM `StandardGraph`.
- **FR-005**: The library MUST expose count properties (`nodeCount`, `relationshipCount`, `walkCount`, `annotationCount`) and health checks (`isEmpty`, `hasConflicts`) on the WASM `StandardGraph`.
- **FR-006**: The library MUST expose graph-native query methods (`source`, `target`, `neighbors`, `degree`) on the WASM `StandardGraph`.
- **FR-007**: The library MUST expose escape hatches (`asPatternGraph`, `asQuery`) on the WASM `StandardGraph` returning existing binding types.
- **FR-008**: The library MUST expose a `SubjectBuilder` class in WASM bindings with `label`, `property`, and `done` methods supporting method chaining.
- **FR-009**: The library MUST add a `build` static method to the existing WASM `Subject` class that returns a `SubjectBuilder`.
- **FR-010**: The library MUST expose a `StandardGraph` class in Python bindings with `__init__`, `from_gram`, and `from_patterns` class methods.
- **FR-011**: The library MUST expose `add_node`, `add_relationship`, `add_walk`, `add_annotation`, and `add_pattern` mutating methods on the Python `StandardGraph`, each returning `self` for chaining.
- **FR-012**: The library MUST expose element access methods (`node`, `relationship`, `walk`, `annotation`) returning the pattern or `None` on the Python `StandardGraph`.
- **FR-013**: The library MUST expose iteration methods (`nodes`, `relationships`, `walks`, `annotations`) returning lists of tuples on the Python `StandardGraph`.
- **FR-014**: The library MUST expose count properties and health checks as Python properties on the Python `StandardGraph`.
- **FR-015**: The library MUST expose graph-native query methods (`source`, `target`, `neighbors`, `degree`) on the Python `StandardGraph`.
- **FR-016**: The library MUST expose a `SubjectBuilder` class in Python with `label`, `property`, and `done` methods supporting method chaining.
- **FR-017**: The library MUST add a `build` static method to the existing Python `Subject` class.
- **FR-018**: Python's `SubjectBuilder.property()` MUST accept native Python types (`str`, `int`, `float`, `bool`) directly without requiring explicit Value wrapping.
- **FR-019**: The library MUST provide TypeScript type definitions (`.d.ts`) for all new WASM classes and methods.
- **FR-020**: The library MUST update Python type stubs (`.pyi`) for all new Python classes and methods.
- **FR-021**: Python `StandardGraph` MUST implement `__repr__` and `__len__` for Pythonic representation.
- **FR-022**: All `fromGram`/`from_gram` methods MUST throw/raise descriptive errors on invalid gram syntax.
- **FR-023**: All identity parameters at the binding boundary MUST accept strings, with internal conversion to the appropriate internal type.

### Key Entities

- **StandardGraph**: The primary graph type exposed to users — a mutable graph that supports element-by-element construction and graph-native queries.
- **SubjectBuilder**: A fluent builder for constructing `Subject` values with labels and properties before adding them to a graph.
- **Subject**: A self-descriptive value with identity, labels, and properties — the element type stored in the graph.
- **Pattern**: A recursive structure wrapping Subject values — returned by graph access and query methods.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: TypeScript users can create a StandardGraph, add nodes and relationships, and query neighbors in under 10 lines of code.
- **SC-002**: Python users can perform the same graph construction and querying workflow in under 10 lines of code using idiomatic snake_case API.
- **SC-003**: `StandardGraph.fromGram("(a)-[:KNOWS]->(b)")` produces a correct graph in both TypeScript and Python with no additional setup.
- **SC-004**: `Subject.build("id").label("L").property("k", v).done()` produces a valid Subject in both TypeScript and Python.
- **SC-005**: All existing binding tests continue to pass with zero regressions.
- **SC-006**: Integration tests cover graph construction, querying, gram parsing, and error handling in both TypeScript and Python.
- **SC-007**: Example files demonstrate the complete workflow (construction, querying, gram parsing) in each target language.

## Assumptions

- The Rust `StandardGraph` implementation (from `035-standard-graph`) is complete and stable.
- The existing WASM binding patterns (`WasmPatternGraph`, `WasmSubject`, `WasmPattern`) and Python binding patterns (`PySubject`, `PyPattern`) are the established conventions to follow.
- Graph sizes in practice are small enough that materializing iterators into arrays/lists at the boundary is acceptable.
- Python escape hatches (`as_query`, `as_snapshot`, `as_pattern_graph`) are deferred until the abstract graph layer has Python bindings.
- `GraphView` bindings (`asSnapshot`) are deferred until `GraphView` has its own binding story.
- The `wasm_bindgen` limitation on consuming `self` is handled by using `&mut self` internally in `SubjectBuilder`, constructing the Rust builder only at `.done()`.
