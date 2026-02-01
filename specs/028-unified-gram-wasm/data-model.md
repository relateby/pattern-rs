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

- **Source**: pattern-core (re-exported or wrapped by pattern-wasm).
- **Fields**: `identity: string`, `labels: string[] | Set<string>`, `properties: Record<string, Value>`.
- **Relationships**: Used as the value type of Pattern&lt;Subject&gt;; gram notation serializes Pattern&lt;Subject&gt;.
- **Validation**: Identity and labels per pattern-core; no new rules in pattern-wasm.
- **Conventional conversion**: **Subject.fromValue(value)** turns primitives into Subjects using pattern-lisp compatible defaults. Subject instances are returned as-is (true passthrough). For custom identity/labels/properties, use the Subject constructor directly.

### Value

- **Source**: pattern-core (re-exported or wrapped by pattern-wasm).
- **Fields**: Typed variants (string, int, decimal, boolean, etc.) used as property values.
- **Relationships**: Attached to Subject via `properties`; used when constructing or reading Subjects.

### Gram (namespace)

- **Not an entity**: Namespace of operations.
- **Operations**: `stringify(pattern)`, `parse(text)`, `parseOne(text)`, `from(value)`.
- **Inputs/Outputs**:
  - stringify: Pattern&lt;Subject&gt; → string (gram notation). Single pattern only.
  - parse: string → Pattern&lt;Subject&gt;[].
  - parseOne: string → Pattern&lt;Subject&gt; | null.
  - from: any JS value → Pattern&lt;Subject&gt;. Handles primitives, arrays, objects, Patterns, and Subjects.

## Conventional conversion

**Subject.fromValue(value)** converts primitives to Subjects using fixed defaults. **Gram.from(value)** converts any JS value (including collections and Patterns) to Pattern&lt;Subject&gt;.

### Subject.fromValue mapping (primitives only)

- **Source type** → **Subject identity** → **Labels** → **Properties**
- string → "_0" → ["String"] → { value: Value.string(s) }
- number → "_0" → ["Number"] → { value: Value.decimal(n) or Value.int(n) }
- boolean → "_0" → ["Bool"] → { value: Value.boolean(b) }
- Subject → original instance returned (true passthrough - preserves === equality)

No options parameter. For custom identity/labels/properties, use the Subject constructor directly.

**Note**: Arrays and objects are rejected by `Subject.fromValue()` - use `Gram.from()` instead.

### Gram.from mapping (all types)

- Primitives → atomic Pattern with Subject (via Subject.fromValue)
- Arrays → Pattern with "List" label, elements as children
- Objects → Pattern with "Map" label, key-value pairs as alternating children  
- Pattern<V> → maps over structure, converting each value
- Subject → passthrough, wrapped in atomic Pattern

## State and lifecycle

- No server-side state; patterns are in-memory. Parse produces new Pattern&lt;Subject&gt; instances; stringify reads existing instances. Empty or whitespace parse input returns empty list (or parseOne returns null) per spec.

## Identity and uniqueness

- Subject identity is a string; uniqueness within a pattern is per pattern-core semantics. Duplicate identity in multiple places in a pattern: behavior is consistent and predictable (structure preserved); documented in edge cases.
