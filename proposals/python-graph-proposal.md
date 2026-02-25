# Proposal: Python Bindings Update for New Rust Features for Graphs

**Date**: 2026-02-25  
**Status**: Draft  
**Scope**: `crates/pattern-core/src/python.rs`, `python/relateby/`

---

## Summary

The Rust core has grown significantly since the Python bindings (`python.rs`) were last updated. New graph-level capabilities — `PatternGraph`, `GraphClassifier`, `GraphQuery`, `GraphView`, graph algorithms, graph transforms, and the `ReconciliationPolicy` system — are fully implemented in Rust but entirely absent from the Python API surface. This proposal defines what to expose, how to expose it, and the priority order.

---

## Current State

### What Python Already Has

The current `python.rs` exposes:

| Class | Coverage |
|---|---|
| `Value` | Complete — all variants with factory methods and extractors |
| `Subject` | Complete — identity, labels, properties, mutation |
| `Pattern` | Complete — constructors, traversal, map/fold/para, comonad ops, combine, validate, analyze |
| `ValidationRules` | Complete |
| `StructureAnalysis` | Complete |
| `ValidationError` | Complete |

### What Is Missing

The following Rust modules have **no Python exposure**:

| Rust Module | Key Types / Functions | Impact |
|---|---|---|
| `pattern_graph` | `PatternGraph`, `from_patterns`, `merge` | Cannot build graph containers from Python |
| `graph::graph_classifier` | `GraphClass`, `GraphClassifier`, `GraphValue` | Cannot classify patterns |
| `graph::graph_query` | `GraphQuery`, `TraversalDirection` | Cannot query graph structure |
| `graph::graph_view` | `GraphView`, `from_pattern_graph`, `materialize` | Cannot use view abstraction |
| `graph::algorithms` | `bfs`, `dfs`, `shortest_path`, `connected_components`, `has_cycle`, `topological_sort`, `degree_centrality`, `betweenness_centrality`, `minimum_spanning_tree`, `all_paths`, `query_walks_containing`, `query_co_members`, `query_annotations_of` | No graph algorithms from Python |
| `graph::transform` | `map_graph`, `filter_graph`, `fold_graph`, `para_graph`, `map_with_context`, `unfold_graph` | No graph transforms from Python |
| `reconcile` | `ReconciliationPolicy`, `ElementMergeStrategy`, `SubjectMergeStrategy`, `LabelMerge`, `PropertyMerge` | Cannot control merge behavior |

---

## Design Principles

1. **Python-idiomatic API** — use Python conventions (snake_case, keyword args, `__repr__`, iteration protocols) rather than exposing Rust internals directly.
2. **Opaque wrappers** — wrap complex Rust types as opaque Python objects; expose only what Python needs.
3. **Callable strategies** — where Rust uses trait objects or closures (e.g., `GraphClassifier`, `TraversalWeight`), accept Python callables.
4. **Enum as string** — expose Rust enums (`GraphClass`, `TraversalDirection`, `ReconciliationPolicy`) as Python string constants or lightweight classes, not raw integers.
5. **Incremental** — ship in phases so each phase is independently useful.

---

## Proposed API

### Phase 1 — PatternGraph and Classification (Highest Value)

#### `PatternGraph` class

```python
class PatternGraph:
    """Materialized graph container: nodes, relationships, walks, annotations."""

    @staticmethod
    def from_patterns(
        patterns: list[PatternSubject],
        policy: ReconciliationPolicy = ReconciliationPolicy.last_write_wins(),
    ) -> "PatternGraph": ...

    @staticmethod
    def empty() -> "PatternGraph": ...

    @property
    def nodes(self) -> list[PatternSubject]: ...

    @property
    def relationships(self) -> list[PatternSubject]: ...

    @property
    def walks(self) -> list[PatternSubject]: ...

    @property
    def annotations(self) -> list[PatternSubject]: ...

    @property
    def conflicts(self) -> dict[str, list[PatternSubject]]: ...

    def merge(self, other: "PatternGraph") -> "PatternGraph": ...

    def __len__(self) -> int: ...
    def __repr__(self) -> str: ...
```

#### `GraphClass` enum-like class

```python
class GraphClass:
    """Structural classification of a pattern."""
    NODE = "node"
    RELATIONSHIP = "relationship"
    ANNOTATION = "annotation"
    WALK = "walk"
    OTHER = "other"

    @property
    def name(self) -> str: ...
    def is_node(self) -> bool: ...
    def is_relationship(self) -> bool: ...
```

#### `ReconciliationPolicy` class

```python
class ReconciliationPolicy:
    """Controls how duplicate identities are resolved when building a PatternGraph."""

    @staticmethod
    def last_write_wins() -> "ReconciliationPolicy": ...

    @staticmethod
    def first_write_wins() -> "ReconciliationPolicy": ...

    @staticmethod
    def strict() -> "ReconciliationPolicy": ...

    @staticmethod
    def merge(
        element_strategy: str = "append",  # "replace" | "append" | "union"
        label_merge: str = "union",         # "union" | "intersect" | "left" | "right"
        property_merge: str = "right",      # "left" | "right" | "merge"
    ) -> "ReconciliationPolicy": ...
```

### Phase 2 — Graph Query and Algorithms

#### `GraphQuery` class

```python
class GraphQuery:
    """Read-only query interface over a graph. Built from a PatternGraph."""

    @staticmethod
    def from_pattern_graph(graph: PatternGraph) -> "GraphQuery": ...

    def nodes(self) -> list[PatternSubject]: ...
    def relationships(self) -> list[PatternSubject]: ...
    def source(self, rel: PatternSubject) -> PatternSubject | None: ...
    def target(self, rel: PatternSubject) -> PatternSubject | None: ...
    def incident_rels(self, node: PatternSubject) -> list[PatternSubject]: ...
    def degree(self, node: PatternSubject) -> int: ...
    def node_by_id(self, identity: str) -> PatternSubject | None: ...
```

#### Graph algorithm functions (module-level)

```python
# in relateby.pattern (or relateby.graph submodule)

def bfs(
    query: GraphQuery,
    start: PatternSubject,
    weight: str | Callable = "undirected",  # "undirected" | "directed" | "directed_reverse" | callable
) -> list[PatternSubject]: ...

def dfs(
    query: GraphQuery,
    start: PatternSubject,
    weight: str | Callable = "undirected",
) -> list[PatternSubject]: ...

def shortest_path(
    query: GraphQuery,
    start: PatternSubject,
    end: PatternSubject,
    weight: str | Callable = "undirected",
) -> list[PatternSubject] | None: ...

def all_paths(
    query: GraphQuery,
    start: PatternSubject,
    end: PatternSubject,
    weight: str | Callable = "undirected",
) -> list[list[PatternSubject]]: ...

def connected_components(
    query: GraphQuery,
    weight: str | Callable = "undirected",
) -> list[list[PatternSubject]]: ...

def has_cycle(
    query: GraphQuery,
    weight: str | Callable = "undirected",
) -> bool: ...

def is_connected(
    query: GraphQuery,
    weight: str | Callable = "undirected",
) -> bool: ...

def topological_sort(
    query: GraphQuery,
) -> list[PatternSubject] | None: ...

def degree_centrality(
    query: GraphQuery,
    weight: str | Callable = "undirected",
) -> dict[str, float]: ...

def betweenness_centrality(
    query: GraphQuery,
    weight: str | Callable = "undirected",
) -> dict[str, float]: ...

def minimum_spanning_tree(
    query: GraphQuery,
    weight: str | Callable = "undirected",
) -> list[PatternSubject]: ...
```

### Phase 3 — Graph Transforms

```python
def map_graph(
    view: GraphView,
    f: Callable[[PatternSubject], PatternSubject],
) -> GraphView: ...

def filter_graph(
    view: GraphView,
    predicate: Callable[[GraphClass, PatternSubject], bool],
) -> GraphView: ...

def fold_graph(
    view: GraphView,
    init: Any,
    f: Callable[[Any, GraphClass, PatternSubject], Any],
) -> Any: ...

def para_graph(
    view: GraphView,
    f: Callable[[PatternSubject, list[Any]], Any],
) -> Any: ...

def unfold_graph(
    seed: Any,
    f: Callable[[Any], tuple[PatternSubject, list[Any]]],
) -> GraphView: ...
```

---

## Implementation Plan

### Phase 1 — PatternGraph + Classification (2–3 days)

**Files to modify**:
- `crates/pattern-core/src/python.rs` — add `PyPatternGraph`, `PyGraphClass`, `PyReconciliationPolicy`
- `crates/pattern-core/src/lib.rs` — add new Python classes to `#[pymodule]` init
- `python/relateby/relateby/pattern/__init__.pyi` — add type stubs
- `docs/python-usage.md` — add PatternGraph section

**Key implementation notes**:
- `PyPatternGraph` wraps `PatternGraph<(), Subject>` (the canonical concrete type)
- `from_patterns` calls `pattern_graph::from_patterns_with_policy`
- Expose `pg_nodes`, `pg_relationships`, `pg_walks`, `pg_annotations` as Python lists
- `PyReconciliationPolicy` maps Python string args to `ReconciliationPolicy<SubjectMergeStrategy>`

### Phase 2 — GraphQuery + Algorithms (3–4 days)

**Files to modify**:
- `crates/pattern-core/src/python.rs` — add `PyGraphQuery`; add free functions for algorithms
- Type stubs and docs

**Key implementation notes**:
- `PyGraphQuery` wraps `GraphQuery<Subject>` via `Rc`; Python holds a reference-counted handle
- Weight functions: accept `"undirected"` / `"directed"` / `"directed_reverse"` strings, or a Python callable `(rel, direction: str) -> float`
- Algorithm results return `list[PyPatternSubject]` (wrapping `Pattern<Subject>`)
- `shortest_path` returns `None` if no path exists

### Phase 3 — Graph Transforms (2–3 days)

**Files to modify**:
- `crates/pattern-core/src/python.rs` — add `PyGraphView`, transform free functions
- Type stubs and docs

**Key implementation notes**:
- `PyGraphView` wraps `GraphView<(), Subject>`
- Transform functions accept Python callables; convert results back to `Pattern<Subject>`

---

## Type Stub Updates

Add to `python/relateby/relateby/pattern/__init__.pyi`:

```python
class GraphClass:
    NODE: str
    RELATIONSHIP: str
    ANNOTATION: str
    WALK: str
    OTHER: str
    @property
    def name(self) -> str: ...
    def is_node(self) -> bool: ...
    def is_relationship(self) -> bool: ...

class ReconciliationPolicy:
    @staticmethod
    def last_write_wins() -> ReconciliationPolicy: ...
    @staticmethod
    def first_write_wins() -> ReconciliationPolicy: ...
    @staticmethod
    def strict() -> ReconciliationPolicy: ...
    @staticmethod
    def merge(
        element_strategy: str = ...,
        label_merge: str = ...,
        property_merge: str = ...,
    ) -> ReconciliationPolicy: ...

class PatternGraph:
    @staticmethod
    def from_patterns(
        patterns: list[PatternSubject],
        policy: ReconciliationPolicy = ...,
    ) -> PatternGraph: ...
    @staticmethod
    def empty() -> PatternGraph: ...
    @property
    def nodes(self) -> list[PatternSubject]: ...
    @property
    def relationships(self) -> list[PatternSubject]: ...
    @property
    def walks(self) -> list[PatternSubject]: ...
    @property
    def annotations(self) -> list[PatternSubject]: ...
    @property
    def conflicts(self) -> dict[str, list[PatternSubject]]: ...
    def merge(self, other: PatternGraph) -> PatternGraph: ...
    def __len__(self) -> int: ...

class GraphQuery:
    @staticmethod
    def from_pattern_graph(graph: PatternGraph) -> GraphQuery: ...
    def nodes(self) -> list[PatternSubject]: ...
    def relationships(self) -> list[PatternSubject]: ...
    def source(self, rel: PatternSubject) -> PatternSubject | None: ...
    def target(self, rel: PatternSubject) -> PatternSubject | None: ...
    def incident_rels(self, node: PatternSubject) -> list[PatternSubject]: ...
    def degree(self, node: PatternSubject) -> int: ...
    def node_by_id(self, identity: str) -> PatternSubject | None: ...

def bfs(query: GraphQuery, start: PatternSubject, weight: str = ...) -> list[PatternSubject]: ...
def dfs(query: GraphQuery, start: PatternSubject, weight: str = ...) -> list[PatternSubject]: ...
def shortest_path(query: GraphQuery, start: PatternSubject, end: PatternSubject, weight: str = ...) -> list[PatternSubject] | None: ...
def connected_components(query: GraphQuery, weight: str = ...) -> list[list[PatternSubject]]: ...
def has_cycle(query: GraphQuery, weight: str = ...) -> bool: ...
def topological_sort(query: GraphQuery) -> list[PatternSubject] | None: ...
def degree_centrality(query: GraphQuery, weight: str = ...) -> dict[str, float]: ...
def betweenness_centrality(query: GraphQuery, weight: str = ...) -> dict[str, float]: ...
```

---

## Usage Examples (Target API)

```python
from relateby.pattern import (
    Subject, Value, PatternSubject,
    PatternGraph, GraphQuery, ReconciliationPolicy,
    bfs, shortest_path, connected_components, topological_sort,
    degree_centrality,
)

# Build subjects
alice = Subject("alice", {"Person"}, {"name": Value.string("Alice")})
bob   = Subject("bob",   {"Person"}, {"name": Value.string("Bob")})
carol = Subject("carol", {"Person"}, {"name": Value.string("Carol")})
knows = Subject("r1", {"KNOWS"}, {})

# Build patterns
p_alice = PatternSubject.point(alice)
p_bob   = PatternSubject.point(bob)
p_carol = PatternSubject.point(carol)
p_rel   = PatternSubject.pattern(knows, [p_alice, p_bob])

# Build graph
graph = PatternGraph.from_patterns(
    [p_alice, p_bob, p_carol, p_rel],
    policy=ReconciliationPolicy.last_write_wins(),
)

print(f"Nodes: {len(graph.nodes)}")          # 3
print(f"Relationships: {len(graph.relationships)}")  # 1

# Query
query = GraphQuery.from_pattern_graph(graph)

# BFS traversal
visited = bfs(query, p_alice, weight="undirected")
print([n.value.identity for n in visited])  # ["alice", "bob"]

# Shortest path
path = shortest_path(query, p_alice, p_bob)
print(path)  # [p_alice, p_bob]

# Centrality
centrality = degree_centrality(query)
print(centrality)  # {"alice": 0.5, "bob": 0.5, "carol": 0.0}

# Connected components
components = connected_components(query)
print(len(components))  # 2 (alice+bob, carol)
```

---

## Testing Plan

For each phase, add tests to `crates/pattern-core/tests/python/`:

- `test_pattern_graph.py` — construction, merge, conflict detection
- `test_reconciliation.py` — all four policy variants
- `test_graph_query.py` — node/relationship accessors, source/target
- `test_algorithms.py` — BFS, DFS, shortest path, connected components, cycle detection, topological sort, centrality
- `test_graph_transforms.py` — map, filter, fold, para, unfold

---

## Open Questions

1. **`PatternSubject` vs `Pattern`**: The current Python API has `Pattern` (generic, holds `PyAny`) but graph operations are concrete over `Pattern<Subject>`. Should graph APIs accept `Pattern` (with runtime type check) or require a new `PatternSubject` class that wraps `Pattern<Subject>` specifically? Recommendation: introduce `PatternSubject` as a concrete typed wrapper.

2. **`GraphView` exposure**: `GraphView` is primarily a Rust-internal pipeline type. Python users may not need it directly if `PatternGraph` and the algorithm functions cover their use cases. Defer `GraphView` to Phase 3 and only expose if needed.

3. **Custom classifiers**: The `GraphClassifier` is injectable in Rust. For Python, the canonical `canonical_classifier` (which classifies by Subject structure) should be the default. Exposing custom Python classifiers adds complexity; defer unless requested.

4. **Thread safety**: Rust's `GraphQuery` uses `Rc` (not `Arc`) by default. PyO3 requires `Send` for objects shared across threads. This may require either the `thread-safe` feature flag or ensuring `GraphQuery` is not shared across Python threads. Needs investigation before Phase 2.

---

## Estimated Effort

| Phase | Work | Estimate |
|---|---|---|
| 1 | PatternGraph, GraphClass, ReconciliationPolicy | 2–3 days |
| 2 | GraphQuery, 12 algorithm functions | 3–4 days |
| 3 | GraphView, 5 transform functions | 2–3 days |
| **Total** | | **7–10 days** |
