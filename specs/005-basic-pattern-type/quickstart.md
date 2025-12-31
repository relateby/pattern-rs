# Quickstart: Pattern Construction & Access

**Feature**: 005-basic-pattern-type  
**Date**: 2025-01-27

This guide provides quick examples for using pattern construction functions, accessor methods, and inspection utilities. All functions match the gram-hs reference implementation.

## Basic Usage

### Creating Patterns

#### Using `Pattern::point()`

Create an atomic pattern (no elements):

```rust
use pattern_core::Pattern;

let atomic = Pattern::point("hello".to_string());
```

#### Using `Pattern::pattern()`

Create a pattern with a value and elements (primary constructor):

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern(
    "parent".to_string(),
    vec![
        Pattern::point("child1".to_string()),
        Pattern::point("child2".to_string()),
    ],
);
```

#### Using `Pattern::from_list()`

Create a pattern from a list of values:

```rust
use pattern_core::Pattern;

let pattern = Pattern::from_list("root".to_string(), vec![
    "a".to_string(),
    "b".to_string(),
    "c".to_string(),
]);
// Equivalent to:
// Pattern::pattern("root".to_string(), vec![
//     Pattern::point("a".to_string()),
//     Pattern::point("b".to_string()),
//     Pattern::point("c".to_string()),
// ])
```

### Accessing Pattern Components

#### Getting the Value

```rust
use pattern_core::Pattern;

let pattern = Pattern::point("hello".to_string());
let value = pattern.value(); // &String
println!("Value: {}", value);
```

#### Getting the Elements

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("parent".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
]);

let elements = pattern.elements(); // &[Pattern<String>]
for (i, elem) in elements.iter().enumerate() {
    println!("Element {}: {}", i, elem.value());
}
```

### Inspecting Pattern Structure

#### Checking if Atomic

```rust
use pattern_core::Pattern;

let atomic = Pattern::point("hello".to_string());
assert!(atomic.is_atomic());

let nested = Pattern::pattern("parent".to_string(), vec![
    Pattern::point("child".to_string()),
]);
assert!(!nested.is_atomic());
```

#### Getting Length (Direct Element Count)

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("parent".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
    Pattern::point("child3".to_string()),
]);
assert_eq!(pattern.length(), 3);
```

#### Getting Size (Total Node Count)

```rust
use pattern_core::Pattern;

let atomic = Pattern::point("atom".to_string());
assert_eq!(atomic.size(), 1); // Just the root node

let pattern = Pattern::pattern("root".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
]);
assert_eq!(pattern.size(), 3); // root + 2 children
```

#### Calculating Depth

```rust
use pattern_core::Pattern;

let atomic = Pattern::point("hello".to_string());
assert_eq!(atomic.depth(), 0); // Atomic patterns have depth 0

let nested = Pattern::pattern("parent".to_string(), vec![
    Pattern::pattern("child".to_string(), vec![
        Pattern::point("grandchild".to_string()),
    ]),
]);
assert_eq!(nested.depth(), 2); // parent (0) -> child (1) -> grandchild (2)
```

## Advanced Patterns

### Nested Pattern Construction

Create deeply nested patterns:

```rust
use pattern_core::Pattern;

let deep = Pattern::pattern("level1".to_string(), vec![
    Pattern::pattern("level2".to_string(), vec![
        Pattern::pattern("level3".to_string(), vec![
            Pattern::point("level4".to_string()),
        ]),
    ]),
]);

assert_eq!(deep.depth(), 3);
assert_eq!(deep.size(), 4);
```

### Working with Different Value Types

Patterns work with any value type:

```rust
use pattern_core::Pattern;

// String values
let string_pattern = Pattern::point("hello".to_string());

// Integer values
let int_pattern = Pattern::point(42);

// Subject values (from feature 004)
use pattern_core::{Subject, Symbol};
use std::collections::HashSet;

let subject = Subject {
    identity: Symbol("n1".to_string()),
    labels: HashSet::new(),
    properties: std::collections::HashMap::new(),
};
let subject_pattern = Pattern::point(subject);
```

### Chaining Operations

Combine construction, access, and inspection:

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("parent".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::point("child2".to_string()),
]);

// Access and inspect
if !pattern.is_atomic() {
    println!("Pattern has {} direct elements", pattern.length());
    println!("Pattern has {} total nodes", pattern.size());
    println!("Pattern depth is {}", pattern.depth());
    for elem in pattern.elements() {
        println!("  - {}", elem.value());
    }
}
```

## Common Patterns

### Building Patterns Programmatically

```rust
use pattern_core::Pattern;

fn build_tree(values: &[&str]) -> Pattern<String> {
    if values.is_empty() {
        Pattern::point("".to_string())
    } else if values.len() == 1 {
        Pattern::point(values[0].to_string())
    } else {
        let mid = values.len() / 2;
        Pattern::pattern(
            values[mid].to_string(),
            vec![
                build_tree(&values[..mid]),
                build_tree(&values[mid + 1..]),
            ],
        )
    }
}

let tree = build_tree(&["a", "b", "c", "d", "e"]);
assert_eq!(tree.depth(), 2);
assert_eq!(tree.size(), 5);
```

### Filtering Patterns

```rust
use pattern_core::Pattern;

fn filter_atomic(pattern: &Pattern<String>) -> Vec<&Pattern<String>> {
    let mut result = Vec::new();
    if pattern.is_atomic() {
        result.push(pattern);
    }
    for elem in pattern.elements() {
        result.extend(filter_atomic(elem));
    }
    result
}

let pattern = Pattern::pattern("parent".to_string(), vec![
    Pattern::point("child1".to_string()),
    Pattern::pattern("child2".to_string(), vec![
        Pattern::point("grandchild".to_string()),
    ]),
]);

let atomics = filter_atomic(&pattern);
assert_eq!(atomics.len(), 2);
```

### Using from_list for Convenience

```rust
use pattern_core::Pattern;

// Create pattern from list of strings
let pattern = Pattern::from_list("root".to_string(), vec![
    "a".to_string(),
    "b".to_string(),
    "c".to_string(),
]);

assert_eq!(pattern.value(), "root");
assert_eq!(pattern.length(), 3);
assert_eq!(pattern.size(), 4); // root + 3 children
```

## Testing Examples

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use pattern_core::Pattern;

    #[test]
    fn test_construction() {
        let pattern = Pattern::point("hello".to_string());
        assert_eq!(pattern.value(), "hello");
        assert!(pattern.is_atomic());
        assert_eq!(pattern.length(), 0);
        assert_eq!(pattern.depth(), 0); // Atomic patterns have depth 0
    }

    #[test]
    fn test_pattern() {
        let pattern = Pattern::pattern("parent".to_string(), vec![
            Pattern::point("child".to_string()),
        ]);
        assert_eq!(pattern.value(), "parent");
        assert_eq!(pattern.length(), 1);
        assert_eq!(pattern.size(), 2);
        assert_eq!(pattern.depth(), 1);
    }

    #[test]
    fn test_from_list() {
        let pattern = Pattern::from_list("root".to_string(), vec![
            "a".to_string(),
            "b".to_string(),
        ]);
        assert_eq!(pattern.value(), "root");
        assert_eq!(pattern.length(), 2);
        assert_eq!(pattern.size(), 3);
    }

    #[test]
    fn test_accessors() {
        let pattern = Pattern::pattern("parent".to_string(), vec![
            Pattern::point("child".to_string()),
        ]);
        assert_eq!(pattern.value(), "parent");
        assert_eq!(pattern.elements().len(), 1);
    }

    #[test]
    fn test_inspection() {
        let pattern = Pattern::pattern("parent".to_string(), vec![
            Pattern::pattern("child".to_string(), vec![
                Pattern::point("grandchild".to_string()),
            ]),
        ]);
        assert!(!pattern.is_atomic());
        assert_eq!(pattern.length(), 1);
        assert_eq!(pattern.size(), 3);
        assert_eq!(pattern.depth(), 2);
    }
}
```

## Performance Considerations

- Construction: `point()` and `pattern()` are O(1), `from_list()` is O(n)
- Access: `value()` and `elements()` are O(1) operations
- Inspection: 
  - `length()` and `is_atomic()` are O(1)
  - `size()` and `depth()` are O(n) where n is total nodes
- All functions handle patterns with 100+ nesting levels safely
- All functions handle patterns with 10,000+ elements efficiently

## WASM Usage

All functions compile for WebAssembly:

```bash
cargo build --package pattern-core --target wasm32-unknown-unknown
```

The functions can be used in WASM modules, though JavaScript bindings are deferred to later features.

## Behavioral Equivalence with gram-hs

All functions maintain behavioral equivalence with the gram-hs reference implementation:

- `Pattern::point()` matches `point :: v -> Pattern v`
- `Pattern::pattern()` matches `pattern :: v -> [Pattern v] -> Pattern v`
- `Pattern::from_list()` matches `fromList :: v -> [v] -> Pattern v`
- `pattern.length()` matches `length :: Pattern v -> Int`
- `pattern.size()` matches `size :: Pattern v -> Int`
- `pattern.depth()` matches `depth :: Pattern v -> Int` (note: atomic patterns have depth 0)

Reference: `../gram-hs/libs/pattern/src/Pattern/Core.hs`

## Next Steps

- See `specs/004-pattern-data-structure/` for Pattern type definition
- See `PORTING_GUIDE.md` for porting workflow
- See `docs/gram-rs-project-plan.md` for overall architecture
