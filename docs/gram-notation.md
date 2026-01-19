# Gram Notation Reference

Gram notation provides a concise and readable way to express Pattern structures. This guide explains how different Gram constructs map to the underlying Pattern data structure.

## Core Principle: Syntactic Sugar

Gram notation is primarily syntactic sugar for Pattern structures. Every Gram construct can be translated into a standard `Pattern` with a value and a sequence of elements. See the **[Introduction](introduction.md)** for a deeper look at the core Pattern concepts.

## Nodes (Atomic Patterns)

A **Node** represents an atomic pattern—a pattern with zero elements.

**Gram Notation:**
```gram
(n:Person)
```

**Equivalent Pattern Representation:**
- **Value**: `n:Person`
- **Elements**: `[]` (empty)

In Gram, nodes are the building blocks. Even a simple value in brackets `[value]` is conceptually an atomic pattern if it has no elements.

## Relationships

A **Relationship** is a pattern with exactly two elements, where those elements are typically nodes.

**Gram Notation:**
```gram
(a)-[r:KNOWS]->(b)
```

**Equivalent Pattern Representation:**
- **Value**: `r:KNOWS`
- **Elements**: `[(a), (b)]` (two atomic patterns)

### Directed vs. Undirected
Gram supports directed (`->`, `<-`) and undirected (`-`) relationships. While the underlying Pattern structure is simply a sequence of elements, the direction information is typically encoded in the relationship value or its properties.

## Annotations

An **Annotation** is a way to wrap a pattern with additional metadata. Conceptually, it's a pattern with one element.

**Gram Notation:**
```gram
@k("v") (n)
```

**Equivalent Pattern Representation:**
- **Value**: `{k: "v"}` (or whatever the annotation represents)
- **Elements**: `[(n)]` (one element)

Annotations are powerful for adding context (like timestamps, source info, or weights) to any pattern without changing the pattern's own elements.

## Nesting and Paths

Because Patterns are recursive, Gram notation can express complex nested structures and paths.

### Lists of Elements
A general pattern with multiple elements uses the pipe `|` syntax:
```gram
["root" | (a), (b), (c)]
```
- **Value**: `"root"`
- **Elements**: `[(a), (b), (c)]`

### Nested Patterns
Patterns can contain other patterns as elements:
```gram
["outer" | ["inner" | (a), (b)]]
```
- **Value**: `"outer"`
- **Elements**: One element, which is itself a pattern with value `"inner"` and two elements `(a)` and `(b)`.

### Paths
A path is simply a sequence of relationships:
```gram
(a)-[:STEP1]->(b)-[:STEP2]->(c)
```
This is interpreted as a set of relationships or a larger pattern containing these relationships, depending on the context of the parser.

## Summary Mapping Table

| Gram Construct | Elements Count | Concept |
|----------------|----------------|---------|
| `(n)`          | 0              | Atomic Pattern (Node) |
| `@a (n)`       | 1              | Annotation |
| `(a)-[r]->(b)` | 2              | Relationship |
| `[v | a, b, c]` | N              | General Pattern |
