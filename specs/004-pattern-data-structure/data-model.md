# Data Model: Core Pattern Data Structure

**Feature**: 004-pattern-data-structure  
**Date**: 2025-01-27

## Overview

This document defines the data structure for the core Pattern type. This is the foundational data structure for the gram-rs library, ported from the gram-hs reference implementation. The Pattern type is generic over value type `V`, allowing any type to be used as the pattern value.

## Core Entity

### Pattern<V>

The core recursive nested structure (s-expression-like) that is generic over value type `V`.

**Structure**:
```rust
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

**Characteristics**:
- **Generic over V**: The value type `V` can be any type, providing "information about the elements"
- **Recursive**: Elements are themselves `Pattern<V>`, creating recursive nested structure
- **S-expression-like**: Not a tree, but a recursive nested structure similar to s-expressions
- **Intimate pairing**: Value and elements form an intimate pairing where value provides information about elements

**Constraints**:
- Value and elements are always paired (cannot have value without elements field, though elements may be empty)
- Recursive structure allows arbitrary nesting depth (within reasonable limits: at least 100 levels)
- Supports arbitrary element counts (within reasonable limits: at least 10,000 elements)

**Traits**:
- `Debug`: Structured representation for debugging (with truncation for deep nesting)
- `Display`: Human-readable representation
- `Clone`: Value semantics for copying patterns
- `PartialEq`, `Eq`: Equality comparison based on structure and values

**Validation Rules**:
- No explicit validation rules for structure (patterns can have empty elements, deeply nested structures, etc.)
- Equality is structural: two patterns are equal if their values are equal and their elements are equal (recursively)
- Type safety: Value type `V` must implement required traits (Clone, PartialEq, Eq) for Pattern to implement those traits

## Subject Type

The Subject type is a self-descriptive value type that can be used as a pattern value. It contains identity, labels, and properties.

**Structure**:
```rust
/// Symbol identifier that uniquely identifies the subject
pub struct Symbol(pub String);

/// Range value for numeric ranges (lower and upper bounds, both optional)
pub struct RangeValue {
    /// Lower bound of the range (inclusive), None means unbounded below
    pub lower: Option<f64>,
    /// Upper bound of the range (inclusive), None means unbounded above
    pub upper: Option<f64>,
}

/// Property value types for Subject properties
pub enum Value {
    VInteger(i64),
    VDecimal(f64),
    VBoolean(bool),
    VString(String),
    VSymbol(String),
    VTaggedString { tag: String, content: String },
    VArray(Vec<Value>),
    VMap(std::collections::HashMap<String, Value>),
    VRange(RangeValue),
    VMeasurement { unit: String, value: f64 },
}

/// Property record type alias
pub type PropertyRecord = std::collections::HashMap<String, Value>;

/// Self-descriptive object with identity, labels, and properties
pub struct Subject {
    /// Symbol identifier that uniquely identifies the subject
    pub identity: Symbol,
    /// Set of label strings that categorize or classify the subject
    pub labels: std::collections::HashSet<String>,
    /// Key-value property map storing structured data about the subject
    pub properties: PropertyRecord,
}
```

**Characteristics**:
- **Identity**: Required symbol identifier (wrapped String)
- **Labels**: Set of strings (no duplicates, order doesn't matter)
- **Properties**: Map from string keys to Value types (rich structured data)

**Traits**:
- `Clone` (where components implement Clone)
- `PartialEq`, `Eq` (for equality comparison)
- `Debug`: Structured representation
- `Display`: Human-readable representation

**Usage with Pattern**:
- `Subject` can be used as the value type in `Pattern<Subject>`
- This enables patterns to contain subjects as their decoration values
- Pattern structure comes from recursive nested elements; Subject provides self-descriptive content

## Value Types

The Pattern type is generic over value type `V`. Any type can be used as a pattern value:

- **Primitive types**: `Pattern<i32>`, `Pattern<String>`, `Pattern<bool>`, etc.
- **Subject type**: `Pattern<Subject>` - common use case for replacing object-graphs with nested patterns
- **Custom types**: User-defined structs, enums, or any other Rust type
- **Any Rust type**: Since Pattern<V> is generic, any type that implements the required traits can be used as a pattern value

## Relationships

### Pattern Value → Pattern Elements

- **Relationship**: Intimate pairing
- **Cardinality**: One value to many elements (0..N)
- **Nature**: Value provides "information about the elements"
- **Constraint**: Elements are always `Pattern<V>` (recursive)

## State Transitions

N/A - Patterns are immutable data structures. No state transitions.

## Validation Rules

### Pattern Structure

1. **Value-Elements Pairing**: Every pattern has both value and elements (elements may be empty Vec)
2. **Recursive Structure**: Elements are always `Pattern<V>` (type system enforces this)
3. **Type Consistency**: All elements in a pattern must be `Pattern<V>` where `V` matches the pattern's value type

### Equality

1. **Structural Equality**: Two patterns are equal if:
   - Their values are equal (using `PartialEq` for `V`)
   - Their elements are equal (recursively, using `PartialEq` for `Pattern<V>`)
2. **Type Safety**: Patterns with different value types cannot be compared (type system prevents this)

### Trait Requirements

For `Pattern<V>` to implement traits, `V` must implement:
- `Clone` → Pattern implements `Clone`
- `PartialEq` → Pattern implements `PartialEq`
- `Eq` → Pattern implements `Eq` (if `V: Eq`)

## Edge Cases

### Deep Nesting

- **Scenario**: Pattern with very deep nesting (e.g., 100+ levels)
- **Handling**: Should handle without stack overflow (may require iterative algorithms for some operations in future features)
- **Current Scope**: Basic structure supports deep nesting; operations deferred to later features

### Many Elements

- **Scenario**: Pattern with many elements (e.g., 10,000+)
- **Handling**: Vec handles large collections efficiently
- **Current Scope**: Structure supports many elements; operations deferred to later features

### Empty Elements

- **Scenario**: Pattern with empty elements Vec
- **Handling**: Valid pattern (atomic pattern)
- **Current Scope**: Fully supported

### Circular References

- **Scenario**: Patterns that reference themselves (directly or indirectly)
- **Handling**: Rust ownership prevents true cycles (patterns own their elements)
- **Current Scope**: Not applicable (ownership model prevents cycles)

## Implementation Notes

1. **Derive Macros**: Use `#[derive(Clone, PartialEq, Eq)]` for Pattern where `V` implements these traits
2. **Custom Debug/Display**: Implement manually for readable output with truncation for deep nesting
3. **WASM Compatibility**: Use only standard library and workspace dependencies
4. **Reference Alignment**: Structure must match gram-hs `Pattern { value, elements }` exactly

## References

- **Primary Source (Authoritative)**: gram-hs Implementation: `../gram-hs/libs/`
  - Pattern: `../gram-hs/libs/pattern/src/Pattern.hs`
  - Subject: `../gram-hs/libs/subject/src/Subject/Core.hs`
- **Secondary Source (Context Only)**: gram-hs Design Documents: `../gram-hs/specs/001-pattern-data-structure/`
  - Type Signatures: `../gram-hs/specs/001-pattern-data-structure/contracts/type-signatures.md` (may be outdated)
- Feature Spec: `spec.md`
- Research: `research.md`
