# Quickstart: Core Pattern Data Structure

**Feature**: 004-pattern-data-structure  
**Date**: 2025-01-27

## Overview

This quickstart guide demonstrates how to use the core Pattern data structure in gram-rs. Patterns are recursive, nested structures (s-expression-like) that are generic over value type `V`. Any Rust type can be used as a pattern value.

## Basic Usage

### Creating Patterns

```rust
use pattern_core::Pattern;

// Create a simple pattern with a string value
let pattern = Pattern {
    value: "hello".to_string(),
    elements: vec![],
};

// Create a pattern with an integer value
let int_pattern = Pattern {
    value: 42,
    elements: vec![],
};

// Create a pattern with nested elements
let nested = Pattern {
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
```

### Using Custom Types as Values

```rust
use pattern_core::Pattern;

// Define a custom type
struct MyData {
    id: u32,
    name: String,
}

// Use it as a pattern value
let custom_pattern: Pattern<MyData> = Pattern {
    value: MyData {
        id: 1,
        name: "example".to_string(),
    },
    elements: vec![],
};
```

### Inspecting Patterns

```rust
use pattern_core::Pattern;

let pattern = Pattern {
    value: "test",
    elements: vec![],
};

// Debug output (structured)
println!("{:?}", pattern);

// Display output (human-readable)
println!("{}", pattern);
```

### Equality Comparison

```rust
use pattern_core::Pattern;

let p1 = Pattern {
    value: 42,
    elements: vec![],
};

let p2 = Pattern {
    value: 42,
    elements: vec![],
};

assert_eq!(p1, p2); // Patterns are equal if values and elements match
```

### Cloning Patterns

```rust
use pattern_core::Pattern;

let original = Pattern {
    value: "original",
    elements: vec![],
};

let cloned = original.clone(); // Deep clone of pattern structure
```

## Advanced Usage

### Deeply Nested Patterns

```rust
use pattern_core::Pattern;

// Create a deeply nested pattern
fn create_nested(depth: usize) -> Pattern<i32> {
    if depth == 0 {
        Pattern {
            value: 0,
            elements: vec![],
        }
    } else {
        Pattern {
            value: depth as i32,
            elements: vec![create_nested(depth - 1)],
        }
    }
}

let deep = create_nested(10);
```

### Patterns with Many Elements

```rust
use pattern_core::Pattern;

// Create a pattern with many elements
let wide = Pattern {
    value: "root",
    elements: (0..1000)
        .map(|i| Pattern {
            value: i,
            elements: vec![],
        })
        .collect(),
};
```

## Common Patterns

### Atomic Pattern (No Elements)

```rust
use pattern_core::Pattern;

let atomic = Pattern {
    value: "atomic",
    elements: vec![], // Empty elements = atomic pattern
};
```

### Pattern with Single Element

```rust
use pattern_core::Pattern;

let single = Pattern {
    value: "parent",
    elements: vec![
        Pattern {
            value: "child",
            elements: vec![],
        },
    ],
};
```

## WASM Usage

Patterns compile to WASM without modification:

```rust
// Same code works for native Rust and WASM
use pattern_core::Pattern;

let pattern = Pattern {
    value: "wasm-compatible",
    elements: vec![],
};
```

Compile for WASM:
```bash
cargo build --target wasm32-unknown-unknown
```

## Testing

### Basic Tests

```rust
use pattern_core::Pattern;

#[test]
fn test_pattern_creation() {
    let pattern = Pattern {
        value: "test",
        elements: vec![],
    };
    assert_eq!(pattern.value, "test");
    assert_eq!(pattern.elements.len(), 0);
}

#[test]
fn test_pattern_equality() {
    let p1 = Pattern { value: 1, elements: vec![] };
    let p2 = Pattern { value: 1, elements: vec![] };
    assert_eq!(p1, p2);
}
```

### Equivalence Testing with gram-hs

```rust
use pattern_core::{Pattern, test_utils::equivalence::*};

#[test]
fn test_gram_hs_equivalence() {
    // Create pattern matching gram-hs structure
    let pattern = Pattern {
        value: "test",
        elements: vec![],
    };
    
    // Use equivalence checking utilities
    // (Implementation details depend on test infrastructure)
}
```

## Notes on Value Types

The Pattern type is generic over value type `V`. You can use any Rust type as a pattern value:
- Primitive types: `Pattern<i32>`, `Pattern<String>`, `Pattern<bool>`, etc.
- Custom types: Your own structs, enums, or any other Rust type
- Any Rust type: Since Pattern<V> is generic, any type that implements the required traits can be used as a pattern value

### Using Subject Type

The Subject type is a self-descriptive value type that can be used with patterns:

```rust
use pattern_core::{Pattern, Subject, Symbol, Value};
use std::collections::{HashSet, HashMap};

// Create a Subject with identity, labels, and properties
let subject = Subject {
    identity: Symbol("n".to_string()),
    labels: {
        let mut set = HashSet::new();
        set.insert("Person".to_string());
        set
    },
    properties: {
        let mut map = HashMap::new();
        map.insert("name".to_string(), Value::VString("Alice".to_string()));
        map.insert("age".to_string(), Value::VInteger(30));
        map
    },
};

// Use Subject as a pattern value
let pattern: Pattern<Subject> = Pattern {
    value: subject,
    elements: vec![],
};
```

## Next Steps

- See `data-model.md` for detailed data structure documentation
- See `contracts/type-signatures.md` for complete API reference
- See `../005-basic-pattern-type/` for pattern construction and access functions (future feature)

## References

- Feature Spec: `spec.md`
- Data Model: `data-model.md`
- Type Signatures: `contracts/type-signatures.md`
- **Primary Source (Authoritative)**: gram-hs Implementation: `../gram-hs/libs/`
  - Pattern: `../gram-hs/libs/pattern/src/Pattern.hs`
  - Subject: `../gram-hs/libs/subject/src/Subject/Core.hs`
- **Secondary Source (Context Only)**: gram-hs Design Documents: `../gram-hs/specs/001-pattern-data-structure/`
