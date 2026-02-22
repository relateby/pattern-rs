# Quickstart: GraphQuery (031-graph-query)

**Date**: 2026-02-22
**Branch**: `031-graph-query`

---

## Overview

`GraphQuery<V>` is a portable graph query interface. You wrap any graph representation (currently `PatternGraph`) to get a uniform query surface, then pass it to algorithms with a traversal weight.

---

## 1. Construct from a PatternGraph

```rust
use std::rc::Rc;
use pattern_core::{
    PatternGraph, Subject,
    from_pattern_graph,
    directed, undirected,
    algorithms,
};

// Build a graph (from 030-graph-classifier feature)
let pg: PatternGraph<Subject> = /* ... */;
let pg = Rc::new(pg);

// Wrap in a GraphQuery
let gq = from_pattern_graph(Rc::clone(&pg));
```

---

## 2. Run algorithms

```rust
// BFS from a starting node (undirected)
let start = gq.query_node_by_id(&my_id).unwrap();
let visited = algorithms::bfs(&gq, &undirected(), &start);

// Shortest path (directed, forward-only)
let path = algorithms::shortest_path(&gq, &directed(), &node_a, &node_b);
match path {
    Some(nodes) => println!("path length: {}", nodes.len()),
    None        => println!("no path"),
}

// Connected components (undirected)
let components = algorithms::connected_components(&gq, &undirected());
println!("{} components", components.len());

// Check if graph has a cycle
let cyclic = algorithms::has_cycle(&gq);
```

---

## 3. Custom traversal weight

```rust
use pattern_core::{TraversalDirection, TraversalWeight, Pattern, Subject};
use std::rc::Rc;

// Weight relationship by a "distance" property on the relationship value
let weighted: TraversalWeight<Subject> = Rc::new(|rel: &Pattern<Subject>, dir| {
    match dir {
        TraversalDirection::Forward => {
            rel.value.properties
                .get("distance")
                .and_then(|v| v.as_f64())
                .unwrap_or(1.0)
        }
        TraversalDirection::Backward => f64::INFINITY, // directed only
    }
});

let path = algorithms::shortest_path(&gq, &weighted, &node_a, &node_b);
```

---

## 4. Frame query (subgraph view)

```rust
use pattern_core::frame_query;

// Restrict graph to nodes with label "Person"
let include: Rc<dyn Fn(&Pattern<Subject>) -> bool> = Rc::new(|p: &Pattern<Subject>| {
    p.value.labels.contains("Person")
});

let person_graph = frame_query(include, gq.clone());

// Algorithms on the framed graph see only "Person" nodes
let person_components = algorithms::connected_components(&person_graph, &undirected());
```

---

## 5. Memoize for expensive algorithms

```rust
use pattern_core::memoize_incident_rels;

// Betweenness centrality calls query_incident_rels many times.
// Memoize first to avoid repeated O(r) scans.
let gq_memo = memoize_incident_rels(gq.clone());
let centrality = algorithms::betweenness_centrality(&gq_memo, &undirected());
```

---

## 6. Context query helpers

```rust
use pattern_core::{canonical_classifier, algorithms};

let classifier = canonical_classifier::<(), Subject>();

// Find all annotations on a node
let node = gq.query_node_by_id(&my_id).unwrap();
let annotations = algorithms::query_annotations_of(&classifier, &gq, &node);

// Find all walks containing a relationship
let rel = gq.query_relationship_by_id(&rel_id).unwrap();
let walks = algorithms::query_walks_containing(&classifier, &gq, &rel);

// Find co-members of an element within a specific container
let container = /* a walk pattern */;
let co_members = algorithms::query_co_members(&gq, &node, &container);
```

---

## 7. Clone is cheap

```rust
// Cloning shares the underlying closures — no data copy
let gq2 = gq.clone();
let gq3 = gq.clone();

// Both reference the same PatternGraph via Rc
let nodes_a = (gq2.query_nodes)();
let nodes_b = (gq3.query_nodes)();
assert_eq!(nodes_a.len(), nodes_b.len());
```

---

## 8. Degree centrality

```rust
// degree_centrality does not take a weight — it counts all incident rels
let centrality = algorithms::degree_centrality(&gq);
for (id, score) in &centrality {
    println!("node {:?}: degree centrality = {:.3}", id, score);
}
```

---

## Traversal weight quick reference

| Constant          | Forward cost | Backward cost | Use case                        |
|-------------------|-------------|---------------|---------------------------------|
| `undirected()`    | 1.0         | 1.0           | Treat all edges as bidirectional |
| `directed()`      | 1.0         | INFINITY      | Follow edge direction only       |
| `directed_reverse()` | INFINITY | 1.0          | Follow edges in reverse only     |
| Custom `Rc<dyn Fn>` | any      | any           | Weighted or hybrid traversal     |
