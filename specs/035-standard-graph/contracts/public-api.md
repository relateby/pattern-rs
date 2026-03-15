# Public API Contract: StandardGraph

**Feature**: 035-standard-graph
**Date**: 2026-03-15

## StandardGraph

### Construction

```
StandardGraph::new() -> StandardGraph
StandardGraph::from_gram(input: &str) -> Result<StandardGraph, ParseError>
StandardGraph::from_patterns(patterns: impl IntoIterator<Item = Pattern<Subject>>) -> StandardGraph
StandardGraph::from_pattern_graph(graph: PatternGraph<(), Subject>) -> StandardGraph
```

### Atomic Element Addition

```
g.add_node(subject: Subject) -> &mut Self
g.add_relationship(subject: Subject, source: &Symbol, target: &Symbol) -> &mut Self
g.add_walk(subject: Subject, relationships: &[Symbol]) -> &mut Self
g.add_annotation(subject: Subject, element: &Symbol) -> &mut Self
```

### Pattern Ingestion

```
g.add_pattern(pattern: Pattern<Subject>) -> &mut Self
g.add_patterns(patterns: impl IntoIterator<Item = Pattern<Subject>>) -> &mut Self
```

### Element Access

```
g.node(id: &Symbol) -> Option<&Pattern<Subject>>
g.relationship(id: &Symbol) -> Option<&Pattern<Subject>>
g.walk(id: &Symbol) -> Option<&Pattern<Subject>>
g.annotation(id: &Symbol) -> Option<&Pattern<Subject>>

g.nodes() -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)>
g.relationships() -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)>
g.walks() -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)>
g.annotations() -> impl Iterator<Item = (&Symbol, &Pattern<Subject>)>
```

### Graph-Native Queries

```
g.source(rel_id: &Symbol) -> Option<&Pattern<Subject>>
g.target(rel_id: &Symbol) -> Option<&Pattern<Subject>>
g.neighbors(node_id: &Symbol) -> Vec<&Pattern<Subject>>
g.degree(node_id: &Symbol) -> usize
```

### Counts and Health

```
g.node_count() -> usize
g.relationship_count() -> usize
g.walk_count() -> usize
g.annotation_count() -> usize
g.is_empty() -> bool

g.has_conflicts() -> bool
g.conflicts() -> &HashMap<Symbol, Vec<Pattern<Subject>>>
g.other() -> &HashMap<Symbol, ((), Pattern<Subject>)>
```

### Escape Hatches

```
g.as_pattern_graph() -> &PatternGraph<(), Subject>
g.into_pattern_graph() -> PatternGraph<(), Subject>
g.as_query() -> GraphQuery<Subject>
g.as_snapshot() -> GraphView<(), Subject>
```

## SubjectBuilder

### Construction and Chaining

```
Subject::build(identity: impl Into<String>) -> SubjectBuilder
builder.label(label: impl Into<String>) -> SubjectBuilder
builder.property(key: impl Into<String>, value: impl Into<Value>) -> SubjectBuilder
builder.done() -> Subject
```

### Trait Implementations

```
impl Into<Subject> for SubjectBuilder
```

## Re-exports

The following will be re-exported from `pattern_core::graph`:
- `StandardGraph`

The following will be re-exported from `pattern_core::subject` (or `pattern_core`):
- `SubjectBuilder` (via `Subject::build`)

## Behavioral Contracts

1. **Atomic methods never fail**: `add_node`, `add_relationship`, `add_walk`, `add_annotation` always succeed. Missing references create placeholders.
2. **from_gram can fail**: Returns `Err(ParseError)` for invalid gram syntax. Valid gram with unclassifiable patterns succeeds (patterns go to "other").
3. **Last-write-wins**: Adding an element with a duplicate identity replaces the previous element. The replaced element is not preserved.
4. **Conflict accumulation**: When reconciliation fails during pattern ingestion, conflicting patterns are stored in `conflicts()`. The original element remains in its bucket.
5. **Undirected neighbor/degree**: `neighbors()` and `degree()` consider both incoming and outgoing relationships.
6. **Lossless conversion**: `into_pattern_graph()` preserves all data including conflicts and other. `from_pattern_graph()` accepts any `PatternGraph<(), Subject>` without reclassification.
