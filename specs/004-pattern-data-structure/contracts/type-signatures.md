# Type Signatures: Core Pattern Data Structure

**Feature**: 004-pattern-data-structure  
**Date**: 2025-01-27

## Overview

This document defines the public API type signatures for the core Pattern data structure. These serve as the contracts that define the interface users will interact with.

## Core Module: pattern_core

### Pattern Type

```rust
/// A recursive, nested structure (s-expression-like) that is generic over value type `V`.
/// 
/// The value provides "information about the elements" - they form an intimate pairing.
/// Elements are themselves patterns, creating the recursive structure.
/// 
/// Patterns are s-expression-like structures, not trees, though they may appear tree-like
/// and accept tree-like operations.
pub struct Pattern<V> {
    /// The value component, which provides information about the elements
    pub value: V,
    /// The nested collection of patterns that form the recursive structure
    pub elements: Vec<Pattern<V>>,
}
```

**Traits**:
- `Clone` (where `V: Clone`)
- `PartialEq` (where `V: PartialEq`)
- `Eq` (where `V: Eq`)
- `Debug` (custom implementation)
- `Display` (custom implementation)

## Subject Type

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

/// Self-descriptive object with identity, labels, and properties.
/// 
/// Subject is designed to be the primary content type for patterns
/// (i.e., `Pattern<Subject>` will be the common use case).
/// 
/// A Subject contains:
/// - **Identity**: A required symbol identifier that uniquely identifies the subject
/// - **Labels**: A set of label strings that categorize or classify the subject
/// - **Properties**: A key-value map storing properties with rich value types
pub struct Subject {
    /// Symbol identifier that uniquely identifies the subject
    pub identity: Symbol,
    /// Set of label strings that categorize or classify the subject
    pub labels: std::collections::HashSet<String>,
    /// Key-value property map storing structured data about the subject
    pub properties: PropertyRecord,
}
```

**Traits**:
- `Clone` (where components implement Clone)
- `PartialEq` (where components implement PartialEq)
- `Eq` (where components implement Eq)
- `Debug` (custom implementation)
- `Display` (custom implementation)

## Module Structure

```rust
// crates/pattern-core/src/lib.rs
pub mod pattern;
pub mod subject;

pub use pattern::Pattern;
pub use subject::{Subject, Symbol, Value, PropertyRecord, RangeValue};
```

## Trait Implementations

### Clone

```rust
impl<V: Clone> Clone for Pattern<V> {
    fn clone(&self) -> Self {
        Pattern {
            value: self.value.clone(),
            elements: self.elements.clone(), // Recursive clone
        }
    }
}
```

### PartialEq / Eq

```rust
impl<V: PartialEq> PartialEq for Pattern<V> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.elements == other.elements
    }
}

impl<V: Eq> Eq for Pattern<V> {}
```

### Debug

```rust
impl<V: Debug> Debug for Pattern<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Custom implementation with truncation for deep nesting
        // Shows structure clearly for debugging
    }
}
```

### Display

```rust
impl<V: Display> Display for Pattern<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Custom implementation for human-readable output
        // May differ from gram-hs format but should be clear
    }
}
```

## Usage Examples

### Creating Patterns

```rust
use pattern_core::Pattern;

// Pattern with string value
let pattern: Pattern<String> = Pattern {
    value: "example".to_string(),
    elements: vec![],
};

// Pattern with integer value
let int_pattern: Pattern<i32> = Pattern {
    value: 42,
    elements: vec![],
};

// Pattern with nested elements
let nested: Pattern<String> = Pattern {
    value: "parent".to_string(),
    elements: vec![
        Pattern {
            value: "child1".to_string(),
            elements: vec![],
        },
        Pattern {
            value: "child2".to_string(),
            elements: vec![],
        },
    ],
};

// Pattern with custom type as value
struct MyValue {
    data: String,
}

let custom_pattern: Pattern<MyValue> = Pattern {
    value: MyValue { data: "test".to_string() },
    elements: vec![],
};
```

### Equality Comparison

```rust
let p1 = Pattern { value: 42, elements: vec![] };
let p2 = Pattern { value: 42, elements: vec![] };
assert_eq!(p1, p2);
```

### Debug Output

```rust
let pattern = Pattern {
    value: "test",
    elements: vec![],
};
println!("{:?}", pattern); // Structured debug output
```

### Display Output

```rust
let pattern = Pattern {
    value: "test",
    elements: vec![],
};
println!("{}", pattern); // Human-readable output
```

## WASM Compatibility

All types and traits are WASM-compatible:
- Standard library types (struct, Vec)
- Standard traits (Clone, PartialEq, Eq, Debug, Display)
- No platform-specific code required

## Notes on Value Types

The Pattern type is generic over value type `V`. Any Rust type can be used as a pattern value:
- Primitive types (i32, String, bool, etc.)
- Subject type (defined in this feature) - `Pattern<Subject>` is a common use case
- Custom structs and enums
- Any type that implements the required traits (Clone, PartialEq, Eq for Pattern to implement those traits)

The Subject type is defined in this feature as a self-descriptive value type that can be used with `Pattern<Subject>`.

## References

- **Primary Source (Authoritative)**: gram-hs Implementation: `../gram-hs/libs/`
  - Pattern: `../gram-hs/libs/pattern/src/Pattern.hs`
  - Subject: `../gram-hs/libs/subject/src/Subject/Core.hs`
- **Secondary Source (Context Only)**: gram-hs Design Documents: `../gram-hs/specs/001-pattern-data-structure/`
  - Type Signatures: `../gram-hs/specs/001-pattern-data-structure/contracts/type-signatures.md` (may be outdated)
- Data Model: `../data-model.md`
- Feature Spec: `../spec.md`

**Note**: This document defines the Rust API contracts. The actual type definitions were verified against the Haskell source code in `../gram-hs/libs/`, not just the design documents.
