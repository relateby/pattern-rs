# Gram Path Equivalences

This topic explains the most important notation-to-structure equivalences in
gram.

## Equivalence Rules

- A node notation corresponds to an atomic pattern.
- A single-relationship path corresponds to a two-element relationship pattern.
- An annotation notation corresponds to a pattern with one element.
- A multi-hop path corresponds to an anonymous pattern whose elements are the
  chained relationships in order.

## Examples

Node to atomic pattern:

```gram
(alice)
```

Single relationship path to two-element pattern:

```gram
(alice)-[knows]->(bob)
```

This is the important asymmetry: a single relationship does not lower to an
anonymous one-element wrapper. It lowers directly to a two-element relationship
pattern.

Annotation to unary pattern:

```gram
@@someId:Label1:Label2
 @k1("v1") @k2(42)
(node)
```

Multi-hop path to chained relationship pattern sequence:

```gram
(alice)-[knows]->(bob)-[works_with]->(carol)
```

## Reading the Equivalences

- Use these mappings to mentally translate between notation and structure.
- The notation is a shorthand for the underlying pattern shape.
- Keep the equivalences in mind when reading or writing gram for graph-like
  data.
- Single-hop paths are a special case of the graph sugar.
- Multi-hop paths are the chain case, where relationships connect through the
  intermediate nodes.

## Related Topics

- `gram`
- `gram-patterns`
- `gram-annotations`
- `gram-graph_elements`
