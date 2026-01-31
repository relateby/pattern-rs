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
- **Conventional conversion**: **Subject.fromValue(value, options?)** turns an arbitrary JS value into a Subject (string, number, boolean, object, Subject passthrough). Options: label, valueProperty, identity (e.g. (value, index) => string). Gram.from uses this under the hood.

### Value

- **Source**: pattern-core (re-exported or wrapped by pattern-wasm).
- **Fields**: Typed variants (string, int, decimal, boolean, etc.) used as property values.
- **Relationships**: Attached to Subject via `properties`; used when constructing or reading Subjects.

### Gram (namespace)

- **Not an entity**: Namespace of operations.
- **Operations**: `stringify(pattern | patterns[])`, `parse(text)`, `parseOne(text)`, `from(pattern, options?)`.
- **Inputs/Outputs**:
  - stringify: Pattern&lt;Subject&gt; or Pattern&lt;Subject&gt;[] → string (gram notation).
  - parse: string → Pattern&lt;Subject&gt;[].
  - parseOne: string → Pattern&lt;Subject&gt; | null.
  - from: Pattern&lt;V&gt; + optional FromOptions → Pattern&lt;Subject&gt;. Implemented as `pattern.map(v => Subject.fromValue(v, options))`.

## Conventional conversion (Subject.fromValue)

The convention is implemented by **Subject.fromValue(value, options?)**. Gram.from(pattern, options?) delegates to it (pattern.map(Subject.fromValue(·, options))).

- **Source type** → **Subject identity** → **Labels** → **Properties**
- string → generated (e.g. _0, _1) → ["String"] → { value: Value.string(s) }
- number → generated → ["Number"] → { value: Value.decimal(n) }
- boolean → generated → ["Boolean"] → { value: Value.boolean(b) }
- object (with id/labels/properties) → obj.id or generated → passthrough or ["Object"] → object entries as properties
- Subject → passthrough → passthrough → passthrough

Optional overrides: label, valueProperty, identity generator (e.g. (value, index) => string).

## State and lifecycle

- No server-side state; patterns are in-memory. Parse produces new Pattern&lt;Subject&gt; instances; stringify reads existing instances. Empty or whitespace parse input returns empty list (or parseOne returns null) per spec.

## Identity and uniqueness

- Subject identity is a string; uniqueness within a pattern is per pattern-core semantics. Duplicate identity in multiple places in a pattern: behavior is consistent and predictable (structure preserved); documented in edge cases.
