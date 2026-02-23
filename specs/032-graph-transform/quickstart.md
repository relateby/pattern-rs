# Quickstart: GraphTransform (032-graph-transform)

**Date**: 2026-02-23  
**Branch**: `032-graph-transform`

---

## Overview

GraphTransform adds a **view** over a classified graph and operations that work on that view: **materialize** (view → PatternGraph), **unfold** / **unfold_graph** (build from seeds), **map_graph** / **map_all_graph**, **filter_graph**, **fold_graph**, **map_with_context**, and **para_graph** / **para_graph_fixed**. You build a view once, chain transformations, then materialize when you need an owned graph.

---

## 1. Build a view and materialize

```rust
use pattern_core::{
    GraphView, PatternGraph, GraphClassifier, canonical_classifier,
    from_pattern_graph, materialize, map_all_graph,
};
use pattern_core::reconcile::ReconciliationPolicy;

// Assume you have a PatternGraph (from 030) and the canonical classifier
let graph: PatternGraph<(), Subject> = /* ... */;
let classifier = canonical_classifier();

// Build view from graph
let view = from_pattern_graph(&classifier, &graph);

// Optional: identity map (no-op) then materialize
let view = map_all_graph(|p| p, view);
let back: PatternGraph<(), Subject> = materialize(&classifier, &ReconciliationPolicy::LastWriteWins, view);
// back is equivalent to graph (same elements, policy permitting)
```

---

## 2. Unfold from seeds (ETL-style)

```rust
use pattern_core::{unfold_graph, canonical_classifier, Pattern, Subject};
use pattern_core::reconcile::ReconciliationPolicy;

struct Row { id: String, name: String }

fn row_to_patterns(row: &Row) -> Vec<Pattern<Subject>> {
    vec![
        /* node pattern for row.id, row.name */,
        /* relationship or annotation as needed */,
    ]
}

let rows: Vec<Row> = /* from DB or CSV */;
let classifier = canonical_classifier();
let graph = unfold_graph(
    &classifier,
    &ReconciliationPolicy::LastWriteWins,
    |row| row_to_patterns(row),
    rows,
);
```

---

## 3. Map by category

```rust
use pattern_core::{
    GraphView, from_pattern_graph, map_graph, materialize,
    CategoryMappers, GraphClass, canonical_classifier,
};
use pattern_core::reconcile::ReconciliationPolicy;

let view = from_pattern_graph(&canonical_classifier(), &graph);

// Normalize only nodes; leave relationships, walks, annotations unchanged
let mappers = CategoryMappers {
    nodes: Box::new(|p| normalize_node(p)),
    ..CategoryMappers::identity()
};
let view = map_graph(&canonical_classifier(), mappers, view);

let out = materialize(&canonical_classifier(), &ReconciliationPolicy::LastWriteWins, view);
```

---

## 4. Map all elements uniformly

```rust
use pattern_core::{from_pattern_graph, map_all_graph, materialize};

let view = from_pattern_graph(&classifier, &graph);
let view = map_all_graph(|p| p.map_values(|v| uppercase_labels(v)), view);
let out = materialize(&classifier, &policy, view);
```

---

## 5. Filter with substitution policy

```rust
use pattern_core::{
    from_pattern_graph, filter_graph, materialize,
    Substitution, GraphClass,
};

let view = from_pattern_graph(&classifier, &graph);

// Keep only nodes with label "Person"
let view = filter_graph(
    &classifier,
    |cls, p| matches!(cls, GraphClass::Node) && has_label(p, "Person"),
    Substitution::NoSubstitution,
    view,
);

let out = materialize(&classifier, &policy, view);
```

---

## 6. Fold (e.g. count by class)

```rust
use pattern_core::{from_pattern_graph, fold_graph};
use std::collections::HashMap;

let view = from_pattern_graph(&classifier, &graph);

let counts: HashMap<GraphClass<()>, usize> = fold_graph(
    |mut acc, cls, _pat| {
        *acc.entry(cls.clone()).or_insert(0) += 1;
        acc
    },
    HashMap::new(),
    &view,
);
```

---

## 7. Map with context (enrich using graph query)

```rust
use pattern_core::{from_pattern_graph, map_with_context, query_annotations_of, GraphQuery, Pattern, Subject};

fn enrich_annotation_count(
    query: &GraphQuery<Subject>,
    node: Pattern<Subject>,
) -> Pattern<Subject> {
    let count = query_annotations_of(&canonical_classifier(), query, &node).len();
    set_annotation_count(count, node)
}

let view = from_pattern_graph(&classifier, &graph);
let view = map_with_context(&classifier, enrich_annotation_count, view);
let out = materialize(&classifier, &policy, view);
```

---

## 8. Para-graph (DAG) and para-graph-fixed (cyclic)

```rust
// DAG: one pass in topological order
let results = para_graph(
    |query, pat, neighbor_results| {
        let depth = 1 + neighbor_results.iter().cloned().max().unwrap_or(0);
        depth
    },
    &view,
);

// Cyclic: iterate until convergence
let pagerank = para_graph_fixed(
    |old, new| (old - new).abs() < 1e-6,
    compute_pagerank_step,
    1.0_f64 / node_count as f64,
    &view,
);
```

---

## 9. Full pipeline (view → filter → map_with_context → materialize)

```rust
let view = from_pattern_graph(&classifier, &graph);
let view = filter_graph(&classifier, is_relevant, Substitution::NoSubstitution, view);
let view = map_with_context(&classifier, enrich_with_annotation_count, view);
let graph = materialize(&classifier, &ReconciliationPolicy::LastWriteWins, view);
```

This matches the canonical pipeline shape from the porting guide: all steps work over `GraphView`; only the last step produces an owned `PatternGraph`.
