# Feature Specification: GraphQuery — Portable, Composable Graph Query Interface

**Feature Branch**: `031-graph-query`
**Created**: 2026-02-22
**Status**: Draft
**Depends on**: 030-graph-classifier
**Mirrors**: `proposals/graph-query-porting-guiide.md`

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Run Graph Algorithms on Any Backing Store (Priority: P1)

A library user wants to run graph traversal algorithms (BFS, DFS, shortest path, connected components) against a PatternGraph without caring how the graph is stored or whether it will change to a different representation later. They write the algorithm once and it works regardless of whether the graph lives in memory, behind a database, or in any other representation that provides the query interface.

**Why this priority**: This is the core value proposition — decoupling algorithms from representations. Without it, the feature delivers no value.

**Independent Test**: Can be tested by constructing a `GraphQuery` from a `PatternGraph`, running each algorithm, and verifying correct results. Delivers the ability to use the full algorithm suite on pattern graphs.

**Acceptance Scenarios**:

1. **Given** a PatternGraph with nodes and relationships, **When** a developer wraps it in a GraphQuery and runs BFS from a starting node, **Then** the traversal visits nodes in breadth-first order with correct results.
2. **Given** a PatternGraph, **When** a developer runs shortest path between two nodes, **Then** the result is the minimum-cost path or None if no path exists.
3. **Given** a PatternGraph, **When** a developer runs connected components, **Then** the result is a collection of node groups where each group is a connected subgraph.
4. **Given** a future graph representation (e.g., a mock graph), **When** a developer constructs a GraphQuery from it and runs the same algorithms, **Then** the algorithms produce correct results without modification.

---

### User Story 2 - Control Traversal Direction at the Call Site (Priority: P2)

A library user wants to traverse a directed graph in forward, backward, or undirected mode depending on the query context — without changing the graph itself or creating separate typed representations. They pass a traversal weight function when calling an algorithm to specify how edges should be traversed.

**Why this priority**: Without call-site traversal control, users need multiple graph copies or redundant algorithm variants. This unifies the model.

**Independent Test**: Can be tested by running the same algorithm on the same graph with different traversal weight functions and verifying that results differ appropriately (e.g., forward-only vs. undirected reachability).

**Acceptance Scenarios**:

1. **Given** a directed graph A→B→C, **When** a developer runs BFS from A with forward-only traversal, **Then** only B and C are reachable from A.
2. **Given** a directed graph A→B→C, **When** a developer runs BFS from C with backward-only traversal, **Then** A and B are reachable from C.
3. **Given** a directed graph A→B→C, **When** a developer runs BFS from A with undirected traversal, **Then** all nodes are reachable from any starting node.
4. **Given** a weighted graph, **When** a developer provides a custom weight function assigning costs to edges, **Then** shortest path respects those costs.

---

### User Story 3 - Compose and Restrict Graph Views (Priority: P3)

A library user wants to create a restricted "frame" view of a larger graph — for example, only nodes and relationships matching a predicate — and run algorithms on that restricted view without copying graph data or writing new algorithm variants. The restricted view is itself a full graph query interface.

**Why this priority**: Composability avoids data duplication and enables filtering, scoping, and layering of graph views declaratively.

**Independent Test**: Can be tested by creating a frame query with a simple predicate, running an algorithm on the frame, and verifying that only elements satisfying the predicate are visited.

**Acceptance Scenarios**:

1. **Given** a graph with mixed node types, **When** a developer creates a frame that includes only nodes with a specific label, **Then** algorithms on the frame see only matching nodes.
2. **Given** a frame query, **When** a developer runs connected components, **Then** only edges where both endpoints satisfy the predicate appear in results.
3. **Given** a deeply nested frame composition (frame over frame), **When** algorithms are run, **Then** results are consistent with the innermost filtering predicate applied at every level.

---

### User Story 4 - Query Context and Containers (Priority: P4)

A library user needs to find all annotations attached to an element, all walks containing a relationship, or all elements that co-occur in the same container. These contextual queries are needed for impact analysis, pattern matching, and preparing deletions safely.

**Why this priority**: Required for higher-level graph operations (GraphTransform, GraphMutation) and independently useful for analysis.

**Independent Test**: Can be tested on a graph with annotations and walks by calling each context helper and verifying that only the correct elements are returned.

**Acceptance Scenarios**:

1. **Given** a node with attached annotation patterns, **When** a developer queries annotations of that node, **Then** only directly-attached annotations are returned (not transitive).
2. **Given** a relationship included in one or more walk patterns, **When** a developer queries walks containing that relationship, **Then** all direct container walks are returned.
3. **Given** an element in a multi-element container, **When** a developer queries co-members, **Then** all other elements sharing the same direct container are returned, excluding the element itself.

---

### Edge Cases

- What happens when BFS/DFS is called with a start node not in the graph? Returns an empty result or single-element result with the start node only if present.
- What happens when shortest path is called between nodes with no connecting path under the given traversal weight? Returns None (no path).
- What happens when a frame predicate excludes all nodes? Algorithms on the empty frame return empty results without error.
- What happens when degree is queried for a node with no incident relationships? Returns zero.
- What happens when a GraphQuery wraps another GraphQuery (composition depth > 1)? Algorithms produce correct results at any composition depth.
- What happens when a weight function returns infinity for all edges? Algorithms report no reachable neighbors (traversal is blocked in all directions).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Library MUST provide a graph query interface that abstracts over graph traversal and lookup, so algorithms do not depend on any specific graph representation.
- **FR-002**: Library MUST provide canonical traversal weight values for forward-only, backward-only, and undirected traversal.
- **FR-003**: Library users MUST be able to supply a custom traversal weight function at the algorithm call site; the same algorithm invocation can behave as directed, undirected, or weighted depending on the supplied function.
- **FR-004**: Library MUST provide a constructor that builds a graph query interface from a PatternGraph.
- **FR-005**: *(Deferred)* Library SHOULD provide a constructor that builds a graph query interface from a GraphLens. This is deferred because `GraphLens` does not yet exist in pattern-rs; it will be implemented when the GraphLens feature is ported. A TODO placeholder is added in the source during this feature's implementation.
- **FR-006**: Library MUST provide a frame combinator that restricts a graph query interface to elements satisfying a caller-supplied predicate; the resulting frame is itself a full graph query interface.
- **FR-007**: Library MUST provide a memoization combinator that caches incident-relationship lookups for a graph query interface, without changing observable behavior.
- **FR-008**: Library MUST provide graph traversal and analysis algorithms expressed as functions over the graph query interface. Required algorithms:
  - *Traversal*: breadth-first traversal (`bfs`), depth-first traversal (`dfs`)
  - *Paths*: shortest path (`shortest_path`), reachability check (`has_path`), all simple paths (`all_paths`)
  - *Boolean queries*: direct neighbor check (`is_neighbor`), global connectivity check (`is_connected`)
  - *Structural*: connected components (`connected_components`), topological sort (`topological_sort`), cycle detection (`has_cycle`)
  - *Spanning*: minimum spanning tree (`minimum_spanning_tree`)
  - *Centrality*: degree centrality (`degree_centrality`), betweenness centrality (`betweenness_centrality`)
- **FR-009**: Library MUST provide context query helpers for finding annotations attached to an element, walks containing a relationship, and co-members sharing a container.
- **FR-010**: The graph query interface MUST be cloneable with no deep copy of backing data; cloning shares the underlying query closures.
- **FR-011**: Existing algorithms that depend on GraphLens MUST continue to work after this feature is introduced; backward compatibility must be preserved.
- **FR-012**: Library MUST provide a multi-threaded variant of the query interface as an opt-in feature, without requiring changes to algorithm code.

### Key Entities

- **GraphQuery**: A composable query interface for a graph. Captures operations for listing nodes, listing relationships, finding incident relationships, looking up sources and targets of relationships, measuring node degree, looking up elements by identity, and finding direct containers. All operations are provided as first-class values that can be shared and composed.
- **TraversalWeight**: A function that assigns a traversal cost to a (relationship, direction) pair. Infinity means impassable. Used by algorithms to determine which neighbors are reachable and at what cost.
- **TraversalDirection**: An enumeration of the two directions in which a relationship can be traversed: from source to target (Forward) or from target to source (Backward).
- **Graph Algorithms**: A collection of reusable traversal and analysis algorithms — BFS, DFS, shortest path, has-path, all-paths, is-neighbor, is-connected, connected components, topological sort, cycle detection, minimum spanning tree, degree centrality, betweenness centrality — all expressed as functions over a GraphQuery and a TraversalWeight (except `topological_sort`/`has_cycle`/`degree_centrality` which are direction-independent).
- **Frame Query**: A restricted view of a GraphQuery, produced by a predicate filter. A frame is itself a GraphQuery and can be composed further.
- **Context Helpers**: Derived query functions for common contextual lookups: annotations of an element, walks containing a relationship, co-members sharing a container.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All graph algorithms (BFS, DFS, shortest path, has-path, connected components, degree centrality) produce correct results when run against a PatternGraph-backed query interface.
- **SC-002**: All graph algorithms produce correct results when run against a mock or stub graph query interface, confirming that no algorithm depends on PatternGraph internals.
- **SC-003**: All graph algorithms produce different (correct) results when supplied with forward-only vs. backward-only vs. undirected traversal weights on the same directed graph.
- **SC-004**: A frame query restricts algorithm results to elements satisfying the predicate; 100% of elements returned by any algorithm on the frame satisfy the predicate.
- **SC-005**: Cloning a graph query interface does not copy backing graph data; memory usage after N clones of a large graph query is O(1) relative to the number of clones.
- **SC-006**: Existing callers of GraphLens-based algorithms continue to work without source changes after this feature is introduced.
- **SC-007**: All algorithms and the graph query interface work correctly under the multi-threaded feature variant — all tests pass with that feature enabled.
- **SC-008**: The feature has full test coverage: every algorithm, constructor, combinator, and context helper has at least one test verifying correctness.

## Assumptions

- The `030-graph-classifier` feature (PatternGraph, GraphLens) is complete and stable before this feature is implemented.
- The primary users of this feature are developers writing graph algorithms or building higher-level graph tools; "user" in this spec means a developer consuming the library.
- Traversal weight infinity semantics (impassable) are sufficient for encoding directed/undirected traversal; no other encoding is needed.
- Performance requirements are correctness-first; O(1) vtable dispatch overhead per algorithm step is acceptable. Profiling-driven optimization is deferred.
- Bulk adjacency materialization (for large-scale algorithms like Louvain) is explicitly deferred — it can be added as a future extension without breaking this interface.
- The memoization combinator uses unbounded caching; cache invalidation is out of scope for this feature.
- `query_degree` is included as an explicit field (not derived from incident relationships) to allow O(1) implementations backed by a degree index; the default behavior derives it from incident relationships.
