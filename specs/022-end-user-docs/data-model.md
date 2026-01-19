# Data Model: Pattern Concepts

The Pattern data structure is the core entity.

## Entity: Pattern<V>

A recursive structure that pairs a value with a sequence of elements.

### Attributes
- **Value (`V`)**: The decoration or information *about* the pattern. 
- **Elements (`Vec<Pattern<V>>`)**: The constituents that *form* the pattern concept.

### Structural Types
- **Atomic Pattern**: A pattern with zero elements. Often represents a "node" or "leaf".
- **Nested Pattern**: A pattern where elements are themselves patterns.

## Concept: Gram Notation

The textual representation of a Pattern.

- `(a)` -> Atomic pattern with value "a".
- `(a)-[r]->(b)` -> Pattern with value "r" and elements `(a)` and `(b)`.
- `[:a | [:b]]` -> Pattern with value "a" and one element with value "b".
