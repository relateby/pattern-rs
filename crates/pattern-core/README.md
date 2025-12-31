# pattern-core

Core pattern data structures for the gram-rs library.

This crate provides the foundational `Pattern<V>` type and `Subject` type, ported from the gram-hs reference implementation.

## Features

- **Pattern<V>**: A recursive, nested structure (s-expression-like) that is generic over value type `V`
- **Subject**: A self-descriptive value type with identity, labels, and properties
- **WASM Compatible**: All types compile successfully for `wasm32-unknown-unknown` target

## Usage

```rust
use pattern_core::{Pattern, Subject, Symbol, Value};
use std::collections::{HashSet, HashMap};

// Create a pattern with a string value
let pattern = Pattern {
    value: "hello".to_string(),
    elements: vec![],
};

// Create a pattern with Subject value
let subject = Subject {
    identity: Symbol("n".to_string()),
    labels: {
        let mut s = HashSet::new();
        s.insert("Person".to_string());
        s
    },
    properties: {
        let mut m = HashMap::new();
        m.insert("name".to_string(), Value::VString("Alice".to_string()));
        m
    },
};

let pattern_with_subject: Pattern<Subject> = Pattern::point(subject);
```

## WASM Compilation

This crate is fully compatible with WebAssembly targets. To compile for WASM:

```bash
cargo build --package pattern-core --target wasm32-unknown-unknown
```

### WASM Compatibility Verification

All types in this crate are WASM-compatible:

- ✅ `Pattern<V>` - Uses only standard library types (Vec, generics)
- ✅ `Subject` - Uses only standard library types (HashSet, HashMap)
- ✅ `Symbol`, `Value`, `RangeValue` - Standard Rust types
- ✅ All traits (Clone, PartialEq, Eq, Debug, Display) - WASM-compatible

No platform-specific code is used. The crate compiles successfully for `wasm32-unknown-unknown` without any modifications.

### Verification Status

- **Last Verified**: 2025-01-27
- **Target**: `wasm32-unknown-unknown`
- **Status**: ✅ Compiles successfully
- **Notes**: All types use only standard library collections and traits that are WASM-compatible

## Traits

All types implement standard Rust traits:

- `Clone` - Value semantics for copying
- `PartialEq`, `Eq` - Equality comparison (Note: `RangeValue`, `Value`, and `Subject` only implement `PartialEq` due to `f64` usage)
- `Debug` - Structured representation for debugging
- `Display` - Human-readable representation

## Testing

Run tests with:

```bash
cargo test --package pattern-core
```

Tests include:
- Unit tests for pattern creation and manipulation
- Equivalence tests comparing with gram-hs reference implementation
- WASM compatibility tests

## Reference Implementation

This crate is a faithful port of the gram-hs reference implementation:
- Reference: `../gram-hs/libs/pattern/` and `../gram-hs/libs/subject/`
- Feature Spec: `../gram-hs/specs/001-pattern-data-structure/`

