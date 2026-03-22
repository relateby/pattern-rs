# Gram Patterns

Patterns are the core gram concept: a value that decorates a sequence of
elements.

## Core Idea

- Elements form the pattern itself.
- The surrounding value names or describes the pattern.
- A pattern may contain no elements, one element, or many elements.
- Nested patterns are allowed because elements are themselves patterns.

## Pattern Shapes

### Atomic pattern

An atomic pattern has no elements.

```gram
(hello)
```

### Pattern with elements

A general pattern can hold a sequence of elements.

```gram
[route | (start), (end)]
```

### Nested pattern

Patterns can contain patterns.

```gram
[outer | [inner | (a), (b)]]
```

## Semantic Notes

- Order matters.
- The value is decoration, not the sequence itself.
- Atomic patterns are the simplest building block for all other shapes.
- The same core pattern idea appears whether gram is used for plain patterns or
  for graph-like forms.

## Related Topics

- `gram`
- `gram-values`
- `gram-graph_elements`
