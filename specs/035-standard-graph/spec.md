# Feature Specification: StandardGraph

**Feature Branch**: `035-standard-graph`
**Created**: 2026-03-15
**Status**: Draft
**Input**: User description: "A StandardGraph as described in proposals/standard-graph-proposal.md"

## Clarifications

### Session 2026-03-15

- Q: Should neighbor/degree queries consider both incoming and outgoing relationships, or only one direction? → A: Both directions — neighbors includes nodes connected by any relationship (in or out); degree counts all relationships (undirected view).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Build a Graph Element by Element (Priority: P1)

A library user constructs a graph by adding individual nodes and relationships. They think in terms of graph elements (nodes, relationships) rather than abstract pattern structures. They should be able to create a graph, add named nodes with labels and properties, and connect them with relationships — all without configuring classifiers, reconciliation policies, or type parameters.

**Why this priority**: Element-by-element construction is the foundational operation. Without it, users cannot build graphs at all. Every other capability depends on having a populated graph.

**Independent Test**: Can be fully tested by creating a graph, adding nodes and relationships, and verifying element counts and retrieval. Delivers a working graph construction experience.

**Acceptance Scenarios**:

1. **Given** an empty graph, **When** the user adds a node with an identity, labels, and properties, **Then** the graph contains exactly one node retrievable by that identity.
2. **Given** a graph with two nodes, **When** the user adds a relationship between them with its own identity, labels, and properties, **Then** the graph contains one relationship, and both source and target nodes are accessible from it.
3. **Given** a graph with no node for a given identity, **When** the user adds a relationship referencing that identity as a source or target, **Then** a minimal placeholder node is automatically created for the missing identity.
4. **Given** a graph with existing elements, **When** the user adds a walk referencing existing relationships, **Then** the walk is stored and retrievable by its identity.
5. **Given** a graph with an existing element, **When** the user adds an annotation referencing that element, **Then** the annotation wraps the referenced element and is retrievable by its identity.

---

### User Story 2 - Fluent Value Construction (Priority: P2)

A library user creates element descriptors (values with identity, labels, and properties) using a fluent builder interface. Instead of manually assembling identities from string wrappers, label sets, and property maps, they chain method calls to build values concisely and readably.

**Why this priority**: Fluent construction eliminates pervasive boilerplate and is independently useful in tests and application code even before the graph itself is used.

**Independent Test**: Can be fully tested by building values with various combinations of identity, labels, and properties, and verifying the resulting value has the correct fields.

**Acceptance Scenarios**:

1. **Given** a desire to create a value, **When** the user provides an identity string and chains label and property calls, **Then** the resulting value has the specified identity, all labels, and all properties.
2. **Given** a builder with no labels or properties added, **When** the user finalizes the value, **Then** the resulting value has the specified identity with empty labels and empty properties.
3. **Given** a builder, **When** the user adds multiple labels, **Then** all labels appear in the resulting value's label set.
4. **Given** a builder, **When** the user adds properties of different types (text, integer, decimal, boolean), **Then** all properties appear with their correct types in the resulting value's property map.

---

### User Story 3 - Build a Graph from Text Notation (Priority: P3)

A library user creates a graph from gram notation strings. They write a human-readable expression like `(alice:Person)-[:KNOWS]->(bob:Person)` and get back a fully classified graph with nodes and relationships in the correct buckets — without manually invoking a classifier.

**Why this priority**: Gram notation ingestion is the fastest path from human intent to a graph. It enables round-trip workflows (text to graph to text) and is essential for testing and interactive use.

**Independent Test**: Can be fully tested by parsing a gram notation string and verifying the resulting graph has the expected nodes, relationships, and properties.

**Acceptance Scenarios**:

1. **Given** a valid gram notation string describing nodes and relationships, **When** the user creates a graph from it, **Then** the graph contains the correct number of nodes and relationships, each with correct identities, labels, and properties.
2. **Given** a valid gram notation string, **When** the user creates a graph from it, **Then** each pattern is automatically classified into the appropriate bucket (node, relationship, walk, or annotation) without user intervention.
3. **Given** an invalid gram notation string, **When** the user attempts to create a graph from it, **Then** the operation returns an error rather than producing a corrupted graph.
4. **Given** an existing graph, **When** the user adds individual patterns to it, **Then** each pattern is classified and added to the appropriate bucket.

---

### User Story 4 - Query Graph Elements (Priority: P4)

A library user retrieves individual elements by identity and iterates over all elements of a given type. They also perform graph-native queries: finding a relationship's source and target nodes, listing a node's neighbors, and checking a node's degree.

**Why this priority**: Querying is essential for any useful graph interaction, but depends on having construction (P1/P3) working first.

**Independent Test**: Can be fully tested by constructing a known graph and verifying individual element retrieval, iteration, neighbor queries, and degree counts return expected results.

**Acceptance Scenarios**:

1. **Given** a graph with multiple nodes, **When** the user retrieves a node by its identity, **Then** the correct node is returned with all its labels and properties.
2. **Given** a graph with multiple nodes, **When** the user iterates over all nodes, **Then** every node in the graph is visited exactly once.
3. **Given** a graph with a relationship between two nodes, **When** the user queries the source of that relationship, **Then** the source node is returned.
4. **Given** a graph with a relationship between two nodes, **When** the user queries the target of that relationship, **Then** the target node is returned.
5. **Given** a graph where a node has both incoming and outgoing relationships, **When** the user queries that node's neighbors, **Then** all adjacent nodes (connected via either direction) are returned.
6. **Given** a graph where a node has both incoming and outgoing relationships, **When** the user queries that node's degree, **Then** the total count of all relationships (both directions) is returned.
7. **Given** a graph, **When** the user requests element counts, **Then** the counts for nodes, relationships, walks, and annotations are each accurate.

---

### User Story 5 - Convert to Abstract Graph Types (Priority: P5)

A library user who has built a graph using the simple interface needs to use it with advanced algorithms or transformations that operate on the abstract graph types. They convert their graph to the abstract representation without losing data.

**Why this priority**: Interoperability with the abstract layer ensures StandardGraph is not a dead end. Users can start simple and graduate to advanced operations when needed.

**Independent Test**: Can be fully tested by constructing a graph, converting it to abstract types, and verifying the converted graph contains identical elements.

**Acceptance Scenarios**:

1. **Given** a populated graph, **When** the user converts it to the abstract graph type, **Then** all nodes, relationships, walks, and annotations are preserved.
2. **Given** a populated graph, **When** the user converts it to a read-only query interface, **Then** the query interface provides access to all graph elements.
3. **Given** a populated graph, **When** the user converts it to a snapshot, **Then** the snapshot captures the full graph state.

---

### Edge Cases

- What happens when a relationship references a node identity that doesn't exist? The graph automatically creates a minimal placeholder node for each missing identity.
- What happens when a pattern cannot be classified into any standard graph element type? It is stored in a separate "other" bucket rather than being rejected, ensuring the graph is always constructable.
- What happens when two elements with the same identity are added? The last-write-wins policy applies, and the most recently added element replaces the prior one.
- What happens when identity conflicts arise during bulk pattern ingestion that cannot be reconciled? The conflicting patterns are accumulated in a "conflicts" collection, accessible for inspection.
- What happens when querying an identity that doesn't exist in the graph? The query returns a "not found" result (no crash, no exception).
- What happens when the graph is empty? Count methods return zero, iteration yields no elements, and the graph reports itself as empty.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The system MUST provide a graph type with no user-facing type parameters or configuration, fixed to the standard value type and canonical classification.
- **FR-002**: The system MUST support creating an empty graph.
- **FR-003**: The system MUST support adding nodes individually, each with an identity, optional labels, and optional properties.
- **FR-004**: The system MUST support adding relationships individually, each with an identity, a source node identity, a target node identity, optional labels, and optional properties.
- **FR-005**: The system MUST support adding walks individually, each with an identity and an ordered list of relationship identities.
- **FR-006**: The system MUST support adding annotations individually, each with an identity and a reference to an existing element.
- **FR-007**: When a relationship or walk references an element identity not yet in the graph, the system MUST automatically create a minimal placeholder element for that identity.
- **FR-008**: The system MUST support creating a graph from a gram notation string, automatically classifying each parsed pattern into the correct element bucket.
- **FR-009**: The system MUST support adding individual patterns or collections of patterns to an existing graph, with automatic classification.
- **FR-010**: Patterns that cannot be classified into any standard graph element type MUST be stored in a separate "other" collection rather than being rejected.
- **FR-011**: Identity conflicts that cannot be reconciled MUST be accumulated in a "conflicts" collection.
- **FR-012**: The system MUST support retrieving any single element (node, relationship, walk, annotation) by its identity.
- **FR-013**: The system MUST support iterating over all elements of a given type (all nodes, all relationships, all walks, all annotations).
- **FR-014**: The system MUST provide count methods for each element type and an emptiness check.
- **FR-015**: The system MUST support querying the source and target nodes of a relationship by the relationship's identity.
- **FR-016**: The system MUST support querying all neighbor nodes of a given node, considering both incoming and outgoing relationships (undirected view).
- **FR-017**: The system MUST support querying the degree of a given node, counting all relationships in both directions (total degree).
- **FR-018**: The system MUST support checking whether the graph has any conflicts and accessing the conflict details.
- **FR-019**: The system MUST support converting the graph to the underlying abstract graph type (lossless, bidirectional).
- **FR-020**: The system MUST support converting the graph to a read-only query interface and a snapshot for use with advanced algorithms.
- **FR-021**: The system MUST provide a fluent builder for constructing values with identity, labels, and properties, usable independently of the graph.
- **FR-022**: The fluent builder MUST support adding multiple labels and properties of different types (text, integer, decimal, boolean).
- **FR-023**: When an element with a duplicate identity is added via atomic methods (`add_node`, `add_relationship`, `add_walk`, `add_annotation`), the system MUST apply a last-write-wins policy, replacing the prior element silently. When a duplicate identity is encountered during pattern ingestion (`add_pattern`, `add_patterns`, `from_gram`), the system MUST attempt reconciliation; if reconciliation fails, the conflict MUST be stored in the conflicts collection per FR-011.
- **FR-024**: The system MUST support creating a graph from an existing abstract graph type (re-wrapping without reclassification).

### Key Entities

- **StandardGraph**: A concrete graph with no type parameters. Contains classified collections of nodes, relationships, walks, annotations, plus "other" and "conflicts" buckets. The primary user-facing graph type.
- **Node**: A graph element representing an entity, with identity, labels, and properties.
- **Relationship**: A directed graph element connecting a source node to a target node, with its own identity, labels, and properties.
- **Walk**: An ordered sequence of relationships, with its own identity.
- **Annotation**: A wrapper around an existing graph element, providing metadata with its own identity.
- **Subject/Value**: A self-descriptive value with identity (symbol), labels (set of strings), and properties (map of key-value pairs). The standard value type for all graph elements.
- **SubjectBuilder**: A fluent builder for constructing Subject values with chained method calls.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can construct a graph with 10 nodes and 10 relationships using fewer than 25 lines of code, with no configuration or setup beyond creating the graph itself.
- **SC-002**: A graph created from gram notation produces the same element counts and structure as the equivalent graph constructed element by element.
- **SC-003**: 100% of existing graph-related test scenarios can be replicated using the new type with less code (fewer lines, no classifier or policy setup).
- **SC-004**: All graph queries (element access, neighbor lookup, degree) return correct results for graphs with up to 1,000 nodes and 5,000 relationships.
- **SC-005**: Round-trip conversion (StandardGraph to abstract type and back) preserves all elements with zero data loss.
- **SC-006**: All unclassifiable patterns and identity conflicts are captured and inspectable — no silent data loss occurs during construction.
- **SC-007**: The fluent value builder reduces value construction code by at least 50% compared to manual construction (measured by line count in test code).

## Assumptions

- The existing canonical classifier (`classify_by_shape`) correctly classifies all standard pattern shapes (nodes, relationships, walks, annotations). StandardGraph relies on this classification being accurate.
- The existing `PatternGraph` type and reconciliation machinery are stable and correct. StandardGraph composes on top of them rather than reimplementing.
- Last-write-wins is an acceptable default reconciliation policy for the initial version. Policy configuration may be added in a future iteration.
- The gram notation parser (gram-codec) produces patterns that the canonical classifier can classify. If the parser evolves, StandardGraph's ingestion will continue to work as long as the classifier is kept in sync.
- This feature is append-oriented only. In-place mutation (removing elements, updating properties of existing elements) is explicitly out of scope and will be addressed in a future feature.

## Dependencies

- **PatternGraph**: The existing abstract graph type that StandardGraph wraps internally.
- **GraphClassifier / classify_by_shape**: The canonical pattern classifier used for automatic classification during pattern ingestion.
- **Reconciliation machinery**: The existing conflict resolution logic (LastWriteWins policy) used internally.
- **gram-codec**: The gram notation parser, required for `from_gram` construction.
- **Subject / Symbol / Value types**: The existing value types that StandardGraph is fixed to.
