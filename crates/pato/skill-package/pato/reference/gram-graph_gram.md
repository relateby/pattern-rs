# Gram Graph Gram

`.graph.gram` is the graph-oriented subset of gram notation.

## Allowed Syntax

- Nodes
- Relationships
- Paths
- Annotations
- A top-level record header

## Not Allowed

- General nested pattern forms that are not graph-shaped
- Extra syntax that does not map cleanly to graph interpretation

## Semantics

- Graph gram focuses on graph-like structure first.
- Nodes become graph nodes.
- Relationships become graph edges.
- Paths become graph walks.
- Annotations decorate the graph structure without replacing it.
- The top-level record is file metadata, not graph structure.

## Example

```gram
{ kind: "graph" }

@source("imported")
(alice)-[knows]->(bob)-[works_with]->(carol)
```

## Related Topics

- `gram`
- `gram-graph_elements`
- `gram-annotations`
- `gram-path_equivalences`
