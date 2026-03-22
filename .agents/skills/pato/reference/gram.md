# Gram

Gram is the compact text format used by `pato` to serialize and deserialize
`Pattern<Subject>` structures.

## Syntax Rules

- Gram has two complementary styles for the same underlying data model:
  pattern notation and path notation.
- Pattern notation uses square brackets for general pattern construction.
- Path notation uses parentheses and arrows for nodes, relationships, and
  graph-like traversals.
- A top-level record may appear before any patterns; in practice it acts as
  document metadata or a header.

## Semantic Rules

- Gram is the serialized form of `Pattern<Subject>`, not a separate graph
  language.
- The elements of a pattern define its structure; the subject/value decorates
  that structure.
- Graph-like forms are specialized syntactic sugar over core pattern shapes.
- Exact forward-reference and other edge-case resolution rules are intentionally
  left to the implementation and should be documented more precisely once they
  settle.

## Examples

Pattern notation:

```gram
[person:Human {name: "Ada"} | friend, colleague]
```

Path notation:

```gram
(alice)-[knows]->(bob)
```

Top-level record header:

```gram
{ kind: "social" }
```

## Related Topics

- `gram-patterns`
- `gram-values`
- `gram-records`
- `gram-annotations`
- `gram-graph_elements`
- `gram-path_equivalences`
- `gram-graph_gram`
- `stdout-stderr-contracts`
- `pato help`
