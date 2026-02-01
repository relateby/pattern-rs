# Data Model: Unified Gram WASM Package

**Feature**: 028-unified-gram-wasm  
**Date**: 2026-01-31

This feature does not introduce new persistent storage or a new domain data model. It exposes existing pattern-core and gram-codec types through a single WASM/JS surface. The data model is that of **Pattern&lt;V&gt;** and **Pattern&lt;Subject&gt;** as defined in pattern-core and used by gram-codec.

## Entities (JS/WASM surface)

### Pattern&lt;V&gt;

- **Source**: pattern-core (re-exported or wrapped by pattern-wasm).
- **Fields**: `value: V`, `elements: Pattern&lt;V&gt;[]` (conceptually; JS may expose getters).
- **Relationships**: Recursive; elements are child patterns. Value type V is Subject for gram serialization.
- **Validation**: Same as pattern-core (depth, size, etc.); ValidationRules and analyzeStructure available where exposed.

### Subject

- **Source**: pattern-core (re-exported by pattern-wasm).
- **Fields**: `identity: string`, `labels: string[] | Set<string>`, `properties: Record<string, Value>`.
- **Relationships**: Used as the value type of Pattern&lt;Subject&gt;; gram notation serializes Pattern&lt;Subject&gt;.
- **Validation**: Identity and labels per pattern-core; no new rules in pattern-wasm.
- **Construction**: Use the Subject constructor: `new Subject(identity, labels, properties)`. Data transformation from other formats is out of scope for pattern-core/gram-codec.

### Value

- **Source**: pattern-core (re-exported or wrapped by pattern-wasm).
- **Fields**: Typed variants (string, int, decimal, boolean, etc.) used as property values.
- **Relationships**: Attached to Subject via `properties`; used when constructing or reading Subjects.

### Gram (namespace)

- **Not an entity**: Namespace of operations.
- **Operations**: `stringify(pattern)`, `parse(text)`, `parseOne(text)`.
- **Inputs/Outputs**:
  - stringify: Pattern&lt;Subject&gt; → string (gram notation). Single pattern only.
  - parse: string → Pattern&lt;Subject&gt;[].
  - parseOne: string → Pattern&lt;Subject&gt; | null.

## Data transformation

Data transformation (converting JavaScript primitives, arrays, objects, JSON, CSV, etc. to Pattern<Subject>) is **out of scope** for pattern-core and gram-codec.

Users should:
- **Use constructors directly**: `new Subject(...)`, `Pattern.point(...)`, `Pattern.pattern(...)`
- **Implement custom conversion logic** for their specific use cases
- **Wait for pattern-io module**: A future `pattern-io` crate will provide standardized conversion utilities with configurable strategies for common formats

**Rationale**: 
- pattern-core focuses on pure data structures and operations
- gram-codec focuses solely on gram notation serialization/deserialization  
- Transformation logic deserves its own module with multiple strategies

## State and lifecycle

- No server-side state; patterns are in-memory. Parse produces new Pattern&lt;Subject&gt; instances; stringify reads existing instances. Empty or whitespace parse input returns empty list (or parseOne returns null) per spec.

## Identity and uniqueness

- Subject identity is a string; uniqueness within a pattern is per pattern-core semantics. Duplicate identity in multiple places in a pattern: behavior is consistent and predictable (structure preserved); documented in edge cases.
