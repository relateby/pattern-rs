# Data Model: WASM/JavaScript and TypeScript Pattern-Core

**Feature**: 027-wasm-pattern-typescript-parity  
**Date**: 2026-01-31

## Overview

This document describes the JavaScript/TypeScript data model for pattern-core WASM bindings. The API exposes Pattern and Subject with behavior equivalent to the Python binding (024-python-pattern-core). At runtime, values are plain JavaScript; TypeScript declarations add generic `Pattern<V>` and typed APIs for type checking and IDE support.

## Core Types

### Pattern (runtime: JS object; TypeScript: `Pattern<V>`)

Recursive, nested structure (s-expression-like) generic over value type V. In JS there is a single “Pattern” type at runtime; in TypeScript it is modeled as `Pattern<V>` so that `Pattern<Subject>` and `Pattern<unknown>` can be expressed.

**Runtime (JavaScript)**:
- Exposed as an opaque object with properties/methods provided by WASM.
- `value`: any (primitive, plain object, or Subject)
- `elements`: array of Pattern (same shape)

**TypeScript**:
- `interface Pattern<V> { readonly value: V; readonly elements: Pattern<V>[]; ... }` plus methods (see contracts/typescript-types.md).

**Construction** (parity with Python):
- `Pattern.point(value)` → Pattern: atomic pattern (no elements)
- `Pattern.of(value)` → Pattern: alias for point
- `Pattern.pattern(value, elements)` → Pattern: pattern with value and child patterns
- `Pattern.fromValues(values)` → Pattern[]: lift each value to a Pattern (point per value)

**Inspection**:
- `length()` → number
- `size()` → number
- `depth()` → number
- `isAtomic()` → boolean
- `values()` → array of value (pre-order)

**Query**:
- `anyValue(predicate: (v: V) => boolean)` → boolean
- `allValues(predicate: (v: V) => boolean)` → boolean
- `filter(predicate: (p: Pattern<V>) => boolean)` → Pattern<V>
- `findFirst(predicate: (p: Pattern<V>) => boolean)` → Pattern<V> | null
- `matches(other: Pattern<V>)` → boolean
- `contains(other: Pattern<V>)` → boolean

**Transformation**:
- `map<W>(fn: (v: V) => W)` → Pattern<W>
- `fold(init: T, fn: (acc: T, v: V) => T)` → T
- `para<R>(fn: (value: V, elementResults: R[]) => R)` → R — paramorphism: bottom-up aggregation with (node value, array of child results). Equivalent to Rust pattern-core para (FR-017, SC-002).

**Combination**:
- `combine(other: Pattern<V>)` → Pattern<V> (when V is Combinable / Subject)

**Comonad**:
- `extract()` → V
- `extend<W>(fn: (p: Pattern<V>) => W)` → Pattern<W>
- `depthAt()` → Pattern<number>
- `sizeAt()` → Pattern<number>
- `indicesAt()` → Pattern<number[]>

**Validation / Analysis**:
- `validate(rules: ValidationRules)` → Either-like value: `Right(undefined)` on success, `Left(ValidationError)` on failure. Does NOT throw. Return shape is trivially convertible to effect-ts Either (see spec FR-016, SC-009). Rust implementation returns `Result<(), ValidationError>`; WASM preserves that at the boundary.
- `analyzeStructure()` → StructureAnalysis (infallible)

### Subject

Self-descriptive value type: identity (Symbol), labels (set of strings), properties (map of string to Value). Exposed to JS/TS with the same structure as in Python.

**Runtime (JavaScript)**:
- Plain object: `{ identity: string, labels: string[] | Set<string>, properties: Record<string, Value> }`. Labels may be array or set depending on binding; semantics are set-of-strings.

**TypeScript**:
- `interface Subject { identity: Symbol; labels: Set<string>; properties: Record<string, Value>; ... }` plus optional methods (addLabel, removeLabel, hasLabel, getProperty, setProperty, removeProperty) if exposed.

**Construction**:
- `Subject.new(identity: string, labels?: string[] | Set<string>, properties?: Record<string, Value>)` (or equivalent constructor) → Subject

### Value

Enum-like representation of property value types. Same variants as Python.

**Variants** (factory functions or constructors):
- `Value.string(s: string)`
- `Value.int(i: number)`
- `Value.decimal(n: number)`
- `Value.boolean(b: boolean)`
- `Value.symbol(s: string)`
- `Value.array(items: Value[])`
- `Value.map(entries: Record<string, Value>)`
- `Value.range(lower?: number, upper?: number)`
- `Value.measurement(value: number, unit: string)`

**Extractors** (for type narrowing / access):
- `asString()`, `asInt()`, `asDecimal()`, `asBoolean()`, `asArray()`, `asMap()`, etc., or equivalent property checks so TypeScript can discriminate.

### Symbol

Identifier type. In Python it is a string; in JS/TS it can be a string or a small wrapper. Exposed in a way that aligns with Python (string or `Symbol` type).

**TypeScript**: `type Symbol = string` or `interface Symbol { readonly value: string }` depending on binding.

### ValidationRules

Configuration for pattern validation (parity with Python).

**Fields**: `maxDepth?: number`, `maxElements?: number` (or equivalent).

### ValidationError

Thrown when validation fails. Message and optional rule/location; represented as a JS Error or subclass.

### StructureAnalysis

Result of structure analysis: summary string, depth distribution, element counts, nesting descriptions (parity with Python).

## Type Relationships

```
Pattern<V>
├── value: V
├── elements: Pattern<V>[]
└── methods (inspection, query, transformation, combine, comonad, validate, analyze)

Subject
├── identity: Symbol (string)
├── labels: Set<string> (or string[])
└── properties: Record<string, Value>

Value
├── string | int | decimal | boolean | symbol | array | map | range | measurement
└── extractors / discriminators
```

## Validation Rules (from requirements)

- Pattern structure: recursive (value, elements) pairing; elements are Patterns with same value type.
- Subject: identity required; labels set of strings; properties map string → Value.
- Value: one of the defined variants; nested arrays/maps contain Value.
- Round-trip: data passed from JS/TS into WASM and returned must preserve structure and value types (SC-007).

## State and Mutability

- Pattern and Subject are immutable from the JS/TS perspective (read-only value and elements; methods return new instances where applicable), matching Python binding semantics.
- Any mutating methods (e.g. Subject addLabel) if exposed return void and mutate the underlying representation only where the Python API allows; otherwise prefer immutable semantics.
