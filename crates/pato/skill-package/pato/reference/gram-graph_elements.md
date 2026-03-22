# Gram Graph Elements

Gram can describe graph-like structures through nodes, relationships, and
paths.

## Nodes

- A node is an atomic pattern with no elements.
- Nodes are the simplest graph element.
- Nodes map directly to the atomic pattern form.

```gram
(alice)
```

## Relationships

- A relationship connects two nodes.
- Direction is expressed by the path punctuation.
- The relationship itself carries the relationship value or label.
- A single relationship is the direct two-element graph form.

```gram
(alice)-[knows]->(bob)
```

## Paths

- A path is a sequence of connected relationships.
- Paths make graph-like traversals explicit.
- Consecutive relationships share endpoints through the path structure.
- Multi-hop paths chain together into an anonymous pattern of relationships.

```gram
(alice)-[knows]->(bob)-[works_with]->(carol)
```

## Directionality

- `->` means left to right.
- `<-` means right to left.
- `-` means no direction is being asserted.
- The direction choice is a graph-syntax convenience, not a separate data
  model.

## Related Topics

- `gram`
- `gram-patterns`
- `gram-path_equivalences`
