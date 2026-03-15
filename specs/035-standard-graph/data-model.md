# Data Model: StandardGraph

**Feature**: 035-standard-graph
**Date**: 2026-03-15

## Entities

### StandardGraph

The primary user-facing graph type. A concrete wrapper with no type parameters.

**Fields**:
- `inner`: PatternGraph<(), Subject> — the wrapped abstract graph containing all classified elements

**Derived collections** (accessed via `inner`):
- Nodes: identity → Pattern<Subject> (atomic patterns)
- Relationships: identity → Pattern<Subject> (2-element patterns: [source, target])
- Walks: identity → Pattern<Subject> (N-element patterns: [rel1, rel2, ...])
- Annotations: identity → Pattern<Subject> (1-element patterns: [wrapped element])
- Other: identity → ((), Pattern<Subject>) (unclassifiable patterns)
- Conflicts: identity → Vec<Pattern<Subject>> (irreconcilable duplicates)

**Identity**: Elements are identified by Symbol (string-based unique identifier).

**Invariants**:
- All nodes are atomic patterns (no elements)
- All relationships have exactly 2 elements (source node, target node)
- All walks have 2+ elements (each a relationship)
- All annotations have exactly 1 element
- No element appears in more than one bucket (nodes, relationships, walks, annotations, other)
- Conflicts accumulate per identity; the last-write value stays in the main bucket

### SubjectBuilder

A fluent builder for constructing Subject values.

**Fields**:
- `identity`: Symbol — the required unique identifier (set at creation)
- `labels`: HashSet<String> — accumulated labels (starts empty)
- `properties`: HashMap<String, Value> — accumulated properties (starts empty)

**Lifecycle**: Created → labels/properties added → finalized to Subject

### Existing Types (unchanged)

| Type | Description | Key Fields |
|------|-------------|------------|
| Subject | Self-descriptive value | identity: Symbol, labels: HashSet<String>, properties: PropertyRecord |
| Symbol | String-based identity | Symbol(String) |
| Value | Property value | VInteger(i64), VDecimal(f64), VBoolean(bool), VString(String), VSymbol(String), VTaggedString, VArray, VMap, VRange, VMeasurement |
| Pattern<V> | Recursive structure | value: V, elements: Vec<Pattern<V>> |
| PatternGraph<Extra, V> | Abstract classified graph | pg_nodes, pg_relationships, pg_walks, pg_annotations, pg_other, pg_conflicts |
| GraphClass<Extra> | Classification enum | GNode, GRelationship, GWalk, GAnnotation, GOther(Extra) |
| GraphQuery<V> | Closure-based query interface | 9 query closures |
| GraphView<Extra, V> | Classified element view | view_query, view_elements |

## Relationships

```
SubjectBuilder ---builds--> Subject
                               |
                          used as value in
                               |
                               v
StandardGraph ---wraps--> PatternGraph<(), Subject>
      |                        |
      |                   contains classified
      |                        |
      v                        v
  GraphQuery<Subject>     Pattern<Subject>
  GraphView<(), Subject>       |
                          identified by
                               |
                               v
                            Symbol
```

## State Transitions

### StandardGraph Lifecycle

```
Empty (new)
  │
  │ add_node / add_relationship / add_walk / add_annotation
  │ add_pattern / add_patterns / from_gram
  │
  v
Populated (elements in buckets)
  │
  │ more additions (append-only, no removal)
  │
  v
Populated (more elements, possible conflicts/other)
  │
  │ as_pattern_graph / as_query / as_snapshot
  │
  v
Converted (read-only abstract form)
```

### SubjectBuilder Lifecycle

```
Created (identity set, empty labels/properties)
  │
  │ .label() / .property()  (repeatable, any order)
  │
  v
Configured (identity + labels + properties)
  │
  │ .done() / Into<Subject>
  │
  v
Subject (finalized, builder consumed)
```
