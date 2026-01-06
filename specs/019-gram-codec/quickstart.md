# Quickstart: Basic Gram Codec

**Feature**: 019-gram-codec  
**Created**: 2026-01-06

## What is the Gram Codec?

The gram codec provides bidirectional transformation between gram notation (human-readable text) and Pattern data structures (programmatic representation).

- **Parser**: Transforms gram notation text → Pattern structures
- **Serializer**: Transforms Pattern structures → gram notation text

## Why Use Gram Notation?

Gram notation provides a concise, readable syntax for patterns:

```gram
// Simple node
(alice:Person {name: "Alice"})

// Relationship between nodes
(alice)-[:KNOWS]->(bob)

// Pattern with elements
[team:Team {name: "DevRel"} | alice, bob, charlie]

// Annotated pattern
@depth(2) [nested | [inner | leaf]]
```

## Key Use Cases

### 1. Load Patterns from Files

Read patterns from `.gram` files:

```gram
// social.gram
(alice:Person {name: "Alice"})
(bob:Person {name: "Bob"})
(alice)-[:KNOWS]->(bob)
```

### 2. Accept User Input

Allow users to create patterns using familiar syntax:

```gram
(start)-->(middle)-->(end)
```

### 3. Save and Share Patterns

Serialize patterns to gram notation for persistence and interoperability:

```gram
[result:Query {time: 2024} | 
  (a:Person {name: "Alice"}),
  (b:Person {name: "Bob"}),
  (a)-[:KNOWS]->(b)
]
```

### 4. Debug and Inspect

View patterns in human-readable form during development:

```gram
// Before transformation
[input | (a), (b), (c)]

// After transformation  
[output:Sorted | (a), (b), (c)]
```

## Grammar Reference

The gram codec follows the **tree-sitter-gram** grammar specification:

- **Repository**: `external/tree-sitter-gram/` (git submodule)
- **Grammar**: `grammar.js` defines all syntax rules
- **Examples**: `examples/data/*.gram` provide sample gram notation
- **Validation**: Use `gram-lint` CLI to validate gram notation

### Submodule Setup

```bash
# Initialize submodule after cloning gram-rs
git submodule update --init --recursive
```

## Validation Tool

All gram notation can be validated using the `gram-lint` CLI:

```bash
# Validate a gram file
gram-lint examples/data/social.gram

# Validate gram expression
gram-lint -e "(hello)-->(world)"

# Show parse tree (S-expression format)
gram-lint -e "(hello)-->(world)" --tree
# Output: (gram_pattern (relationship_pattern left: (node_pattern ...) kind: (right_arrow) right: (node_pattern ...)))

# Multiple expressions
gram-lint -e "(a)-->(b)" -e "[team | alice, bob]"
```

**Exit Codes**:
- `0`: Valid gram notation
- `1`: Parse error detected

**Important**: All gram notation examples in specifications, plans, and tasks MUST be validated with `gram-lint` before inclusion. See [VALIDATION.md](VALIDATION.md) for comprehensive validated examples with parse trees.

## Supported Syntax

### Node Patterns (0 elements)

```gram
()                              // Empty node
(hello)                         // Node with identifier
(a:Person)                      // Node with label
(a:Person {name: "Alice"})      // Node with label and properties
```

### Relationship Patterns (2 elements)

The grammar accepts **multiple visual arrow styles** that are normalized to **4 semantic arrow kinds**:

#### Right Arrow (directed left-to-right)
```gram
(a)-->(b)                       // Single-stroke ✓
(a)==>(b)                       // Double-stroke ✓
(a)~~>(b)                       // Squiggle ✓
(a)-[:KNOWS]->(b)               // With label ✓
(a)-[:KNOWS {since: 2020}]->(b) // With properties ✓
```

#### Left Arrow (directed right-to-left, elements reversed!)
```gram
(a)<--(b)                       // Single-stroke ✓ → stored as [b, a]
(a)<==(b)                       // Double-stroke ✓ → stored as [b, a]
(a)<~~(b)                       // Squiggle ✓ → stored as [b, a]
```

#### Bidirectional Arrow (mutual connection)
```gram
(a)<-->(b)                      // Single-stroke ✓
(a)<==>(b)                      // Double-stroke ✓
```

#### Undirected Arrow (no directionality)
```gram
(a)~~(b)                        // Squiggle ✓
(a)==(b)                        // Double-stroke ✓
```

### Subject Patterns (N elements)

```gram
[team | alice, bob]             // Pattern with elements
[outer | [inner | leaf]]        // Nested patterns
[root:Type {prop: 1} | e1, e2]  // Pattern with identifier, label, properties
```

### Annotated Patterns (1 element)

```gram
@type(node) (a)                 // Annotation on node
@depth(2) [x | y, z]            // Annotation on subject pattern
```

### Property Values

```gram
// Strings
{name: "Alice", role: admin}

// Numbers
{age: 30, score: 95.5}

// Booleans
{active: true, verified: false}

// Arrays
{tags: ["rust", "wasm"], scores: [1, 2, 3]}

// Ranges
{range: 1..10, scores: 0..100}

// Tagged strings
{doc: """markdown content"""}
```

### Comments

```gram
// This is a line comment
(hello)-->(world)  // End-of-line comment
```

## Round-Trip Property

The codec ensures round-trip correctness:

1. Parse gram notation → Pattern
2. Serialize Pattern → gram notation  
3. Re-parse gram notation → Pattern (structurally equivalent)

## Next Steps

After this specification is approved:

1. **Research**: Evaluate parser libraries (WASM + Python support)
2. **Plan**: Design codec architecture and API
3. **Implement**: Build parser and serializer
4. **Test**: Validate against tree-sitter-gram test corpus
5. **Integrate**: Add to `crates/gram-codec/` module

## References

- **Grammar**: `external/tree-sitter-gram/grammar.js`
- **Examples**: `external/tree-sitter-gram/examples/data/`
- **Tests**: `external/tree-sitter-gram/test/corpus/`
- **Validator**: `gram-lint` CLI tool
- **Spec**: `spec.md` (this feature)
- **Data Model**: `data-model.md`

