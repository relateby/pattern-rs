# Quickstart: Traversable Instance for Pattern

**Feature**: 010-traversable-instance  
**Date**: 2026-01-04

## Overview

This quickstart guide demonstrates how to use traversable operations on Pattern to handle effectful transformations. Traversable enables you to apply functions that may fail (returning Option or Result) or are asynchronous (returning Future) to all values in a pattern while properly handling the effects.

## Basic Concepts

**Traversable** = Functor (map) + Effects

- **Functor (map)**: Transform values with pure functions
- **Traversable**: Transform values with effectful functions (Option, Result, Future)

The key difference: traverse automatically sequences and combines effects for you.

## Quick Examples

### Example 1: Parsing Strings to Numbers (Option)

```rust
use pattern_core::Pattern;

// Pattern with string values
let pattern = Pattern::pattern("42", vec![
    Pattern::point("10"),
    Pattern::point("20"),
]);

// Parse all strings to integers (may fail → Option)
let result: Option<Pattern<i32>> = pattern.traverse_option(|s| s.parse().ok());

match result {
    Some(numbers) => {
        // Success! All values parsed
        println!("Root: {}", numbers.value); // 42
        println!("First: {}", numbers.elements[0].value); // 10
    }
    None => {
        // At least one parse failed
        println!("Parsing failed");
    }
}
```

### Example 2: Validation with Error Messages (Result)

```rust
use pattern_core::Pattern;

// Pattern with numbers to validate
let pattern = Pattern::pattern(5, vec![
    Pattern::point(10),
    Pattern::point(15),
]);

// Validate all numbers are in range [0, 100]
let result: Result<Pattern<i32>, String> = pattern.traverse_result(|&n| {
    if n >= 0 && n <= 100 {
        Ok(n)
    } else {
        Err(format!("{} is out of range", n))
    }
});

match result {
    Ok(valid_pattern) => println!("All values valid"),
    Err(error) => println!("Validation failed: {}", error), // First error
}
```

### Example 3: Collecting All Errors (validate)

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("1", vec![
    Pattern::point("invalid1"),
    Pattern::point("2"),
    Pattern::point("invalid2"),
]);

// Collect ALL parse errors, not just the first
let result: Result<Pattern<i32>, Vec<String>> = pattern.validate(|s| {
    s.parse::<i32>().map_err(|e| e.to_string())
});

match result {
    Ok(numbers) => println!("All parsed successfully"),
    Err(errors) => {
        // Show all errors to user
        println!("Found {} errors:", errors.len());
        for error in errors {
            println!("  - {}", error);
        }
    }
}
```

## Common Use Cases

### Use Case 1: Configuration Validation

```rust
use pattern_core::Pattern;

#[derive(Debug)]
struct Config {
    name: String,
    value: i32,
}

// Pattern of config strings
let config_pattern = Pattern::pattern(
    "timeout=30",
    vec![
        Pattern::point("retries=3"),
        Pattern::point("max_connections=100"),
    ],
);

// Parse all config entries
let parsed: Result<Pattern<Config>, String> = config_pattern.traverse_result(|entry| {
    let parts: Vec<&str> = entry.split('=').collect();
    if parts.len() != 2 {
        return Err(format!("Invalid format: {}", entry));
    }
    
    let value = parts[1].parse::<i32>()
        .map_err(|_| format!("Invalid number in: {}", entry))?;
    
    Ok(Config {
        name: parts[0].to_string(),
        value,
    })
});

// Use parsed configuration or report error
match parsed {
    Ok(configs) => {
        println!("Timeout: {}", configs.value.value);
        // Use configs...
    }
    Err(e) => eprintln!("Configuration error: {}", e),
}
```

### Use Case 2: Database ID Resolution

```rust
use pattern_core::Pattern;

#[derive(Debug, Clone)]
struct User {
    id: i32,
    name: String,
}

// Simulated database lookup
fn fetch_user(id: i32) -> Result<User, String> {
    // In real code, this would query a database
    if id > 0 && id < 1000 {
        Ok(User { id, name: format!("User{}", id) })
    } else {
        Err(format!("User {} not found", id))
    }
}

// Pattern of user IDs
let id_pattern = Pattern::pattern(1, vec![
    Pattern::point(2),
    Pattern::point(3),
]);

// Resolve all IDs to user objects
let users: Result<Pattern<User>, String> = id_pattern.traverse_result(|&id| fetch_user(id));

match users {
    Ok(user_pattern) => {
        println!("Loaded users:");
        for user in user_pattern.values() {
            println!("  - {}: {}", user.id, user.name);
        }
    }
    Err(e) => eprintln!("Failed to load users: {}", e),
}
```

### Use Case 3: Type Conversion with Validation

```rust
use pattern_core::Pattern;
use std::num::ParseIntError;

// Pattern of various string formats
let data = Pattern::pattern("  42  ", vec![
    Pattern::point("100"),
    Pattern::point("  200  "),
]);

// Clean and parse in one step
let numbers: Result<Pattern<i32>, ParseIntError> = data.traverse_result(|s| {
    s.trim().parse::<i32>()
});

// Can also map first, then traverse
let numbers_alt: Result<Pattern<i32>, ParseIntError> = data
    .map(|s| s.trim())
    .traverse_result(|s| s.parse::<i32>());

// Both approaches work!
```

### Use Case 4: Optional Value Extraction

```rust
use pattern_core::Pattern;
use std::collections::HashMap;

// Lookup table
let mut lookup = HashMap::new();
lookup.insert("key1", 100);
lookup.insert("key2", 200);

// Pattern of keys
let keys = Pattern::pattern("key1", vec![
    Pattern::point("key2"),
    Pattern::point("key3"), // This one doesn't exist
]);

// Try to look up all keys
let values: Option<Pattern<i32>> = keys.traverse_option(|key| {
    lookup.get(key).copied()
});

// values == None because "key3" lookup returns None
assert!(values.is_none());

// With all valid keys
let valid_keys = Pattern::pattern("key1", vec![Pattern::point("key2")]);
let values: Option<Pattern<i32>> = valid_keys.traverse_option(|key| {
    lookup.get(key).copied()
});

// values == Some(Pattern(100, [Pattern(200)]))
assert!(values.is_some());
```

## Comparison: map vs traverse_option vs traverse_result

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("10", vec![Pattern::point("20")]);

// 1. map: Pure function (always succeeds)
let result1: Pattern<usize> = pattern.map(|s| s.len());
// result1 == Pattern(2, [Pattern(2)])

// 2. traverse_option: May fail (returns None)
let result2: Option<Pattern<i32>> = pattern.traverse_option(|s| s.parse().ok());
// result2 == Some(Pattern(10, [Pattern(20)]))

// 3. traverse_result: May fail with error details
let result3: Result<Pattern<i32>, _> = pattern.traverse_result(|s| s.parse::<i32>());
// result3 == Ok(Pattern(10, [Pattern(20)]))

// With invalid data
let invalid = Pattern::pattern("10", vec![Pattern::point("invalid")]);

// map still works (no validation)
let _ = invalid.map(|s| s.len()); // Pattern(2, [Pattern(7)])

// traverse_option returns None
let result = invalid.traverse_option(|s| s.parse::<i32>().ok());
assert!(result.is_none());

// traverse_result returns Err with details
let result = invalid.traverse_result(|s| s.parse::<i32>());
assert!(result.is_err());
```

## Comparison: traverse_result vs validate

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("1", vec![
    Pattern::point("invalid1"),
    Pattern::point("2"),
    Pattern::point("invalid2"),
]);

// traverse_result: Returns on FIRST error
let result1: Result<Pattern<i32>, String> = pattern.traverse_result(|s| {
    s.parse::<i32>().map_err(|e| e.to_string())
});
// result1 == Err("invalid digit found in string")
// Only first error reported!

// validate: Collects ALL errors
let result2: Result<Pattern<i32>, Vec<String>> = pattern.validate(|s| {
    s.parse::<i32>().map_err(|e| e.to_string())
});
// result2 == Err(vec!["invalid digit...", "invalid digit..."])
// Both errors reported!

// When to use which:
// - traverse_result: Fail-fast, performance-critical, only need to know IF there's an error
// - validate: User feedback, show all issues, form validation
```

## Sequencing: Flipping Structure Layers

```rust
use pattern_core::Pattern;

// Pattern of Options → Option of Pattern
let pattern_of_options: Pattern<Option<i32>> = Pattern::pattern(
    Some(1),
    vec![Pattern::point(Some(2)), Pattern::point(Some(3))],
);

let option_of_pattern: Option<Pattern<i32>> = pattern_of_options.sequence_option();
// option_of_pattern == Some(Pattern(1, [Pattern(2), Pattern(3)]))

// With a None value
let with_none: Pattern<Option<i32>> = Pattern::pattern(
    Some(1),
    vec![Pattern::point(None), Pattern::point(Some(3))],
);

let result = with_none.sequence_option();
// result == None (because one value was None)

// Same works for Result
let pattern_of_results: Pattern<Result<i32, String>> = Pattern::pattern(
    Ok(1),
    vec![Pattern::point(Ok(2))],
);

let result_of_pattern: Result<Pattern<i32>, String> = pattern_of_results.sequence_result();
// result_of_pattern == Ok(Pattern(1, [Pattern(2)]))
```

## Composition Patterns

### Pattern 1: map → traverse_result

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("  hello  ", vec![Pattern::point("  WORLD  ")]);

// Preprocess with map (pure), then validate with traverse
let result: Result<Pattern<String>, String> = pattern
    .map(|s| s.trim().to_lowercase())  // Clean up
    .traverse_result(|s| {               // Validate
        if s.len() > 0 && s.len() < 20 {
            Ok(s.to_string())
        } else {
            Err(format!("Invalid length: {}", s.len()))
        }
    });
```

### Pattern 2: traverse_result → map

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("10", vec![Pattern::point("20")]);

// Parse with traverse (effectful), then transform with map (pure)
let result: Result<Pattern<String>, _> = pattern
    .traverse_result(|s| s.parse::<i32>())?  // Effect first
    .map(|n| format!("Number: {}", n));       // Pure transform after
// result == Ok(Pattern("Number: 10", [Pattern("Number: 20")]))
```

### Pattern 3: traverse → fold

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("5", vec![
    Pattern::point("10"),
    Pattern::point("15"),
]);

// Parse, then sum
let sum: Result<i32, _> = pattern
    .traverse_result(|s| s.parse::<i32>())?  // Parse all
    .fold(0, |acc, n| acc + n);               // Sum results

// sum == Ok(30)
```

### Pattern 4: Complex Pipeline

```rust
use pattern_core::Pattern;

#[derive(Debug, Clone)]
struct ProcessedData {
    value: i32,
    formatted: String,
}

let pattern = Pattern::pattern("  42  ", vec![
    Pattern::point("  100  "),
]);

let result = pattern
    .map(|s| s.trim())                           // 1. Clean (pure)
    .traverse_result(|s| s.parse::<i32>())?      // 2. Parse (effect)
    .map(|&n| ProcessedData {                    // 3. Transform (pure)
        value: n,
        formatted: format!("#{}", n),
    });

// result == Ok(Pattern(ProcessedData { value: 42, ... }, [...]))
```

## Async Example (Feature-gated)

```rust
#[cfg(feature = "async")]
use pattern_core::Pattern;

#[derive(Debug, Clone)]
struct Entity {
    id: i32,
    data: String,
}

// Async database lookup
async fn fetch_entity(id: i32) -> Result<Entity, String> {
    // Simulated async database call
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    if id > 0 {
        Ok(Entity {
            id,
            data: format!("Entity #{}", id),
        })
    } else {
        Err("Invalid ID".to_string())
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let id_pattern = Pattern::pattern(1, vec![
        Pattern::point(2),
        Pattern::point(3),
    ]);
    
    // Fetch all entities asynchronously (sequentially)
    let entities: Pattern<Entity> = id_pattern
        .traverse_future(|&id| fetch_entity(id))
        .await?;
    
    println!("Loaded entities:");
    for entity in entities.values() {
        println!("  - {}: {}", entity.id, entity.data);
    }
    
    Ok(())
}
```

## Error Handling Best Practices

### Practice 1: Add Context to Errors

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("value1", vec![Pattern::point("value2")]);

// Bad: Generic error
let result = pattern.traverse_result(|s| s.parse::<i32>());

// Good: Add context
let result = pattern.traverse_result(|s| {
    s.parse::<i32>()
        .map_err(|e| format!("Failed to parse '{}': {}", s, e))
});
```

### Practice 2: Custom Error Types

```rust
use pattern_core::Pattern;

#[derive(Debug)]
enum ValidationError {
    ParseError { value: String, reason: String },
    RangeError { value: i32, min: i32, max: i32 },
}

let pattern = Pattern::pattern("50", vec![Pattern::point("150")]);

let result = pattern.traverse_result(|s| {
    let n = s.parse::<i32>()
        .map_err(|e| ValidationError::ParseError {
            value: s.to_string(),
            reason: e.to_string(),
        })?;
    
    if n < 0 || n > 100 {
        Err(ValidationError::RangeError { value: n, min: 0, max: 100 })
    } else {
        Ok(n)
    }
});
```

### Practice 3: Recovering from Errors

```rust
use pattern_core::Pattern;

let pattern = Pattern::pattern("10", vec![
    Pattern::point("invalid"),
    Pattern::point("20"),
]);

// Approach 1: Use default values on error (with map, not traverse)
let with_defaults = pattern.map(|s| {
    s.parse::<i32>().unwrap_or(0)  // Default to 0 on error
});

// Approach 2: Filter to valid values only
let values: Vec<i32> = pattern
    .values()
    .iter()
    .filter_map(|s| s.parse::<i32>().ok())
    .collect();

// Approach 3: Collect errors separately
let mut successes = Vec::new();
let mut errors = Vec::new();

for value in pattern.values() {
    match value.parse::<i32>() {
        Ok(n) => successes.push(n),
        Err(e) => errors.push(e),
    }
}
```

## Performance Tips

1. **Use traverse_result for fail-fast**: If you only need to know IF validation fails, use `traverse_result` (short-circuits)
2. **Use validate for comprehensive feedback**: If you need to show ALL errors to users, use `validate`
3. **Avoid unnecessary cloning**: Traverse takes `&V` (reference), doesn't consume values
4. **Consider map first**: If you have expensive pure transformations, do them first with `map` so they're skipped if parse/validation fails

## Common Pitfalls

### Pitfall 1: Forgetting to handle None/Err

```rust
// Bad: Unwrap can panic
let result = pattern.traverse_option(|s| s.parse().ok()).unwrap();

// Good: Handle the case
match pattern.traverse_option(|s| s.parse().ok()) {
    Some(numbers) => // use numbers,
    None => // handle missing value,
}

// Or use ? operator in Result context
let numbers = pattern.traverse_result(|s| s.parse::<i32>())?;
```

### Pitfall 2: Confusing traverse with map

```rust
// map: Pure function (no effects)
let lengths = pattern.map(|s| s.len());  // Always succeeds

// traverse: Effectful function (may fail)
let numbers = pattern.traverse_option(|s| s.parse().ok());  // May return None
```

### Pitfall 3: Wrong error collection method

```rust
// If you want FIRST error only (fast)
let result = pattern.traverse_result(|s| validate(s));  // Short-circuits

// If you want ALL errors (comprehensive)
let result = pattern.validate(|s| validate(s));  // Collects all
```

## Next Steps

- Read [Type Signatures](./contracts/type-signatures.md) for detailed API reference
- Read [Data Model](./data-model.md) to understand effect sequencing
- Check the tests for more examples once implemented
- Explore async usage if you need Future support (feature-gated)

## Quick Reference Card

| Goal | Method | Returns | Short-circuit |
|------|--------|---------|---------------|
| Parse strings (may fail silently) | `traverse_option` | `Option<Pattern<T>>` | Yes (on None) |
| Validate values (need error details) | `traverse_result` | `Result<Pattern<T>, E>` | Yes (first error) |
| Validate values (need all errors) | `validate` | `Result<Pattern<T>, Vec<E>>` | No |
| Async operations (sequential) | `traverse_future` | `Future<Result<...>>` | Yes (first error) |
| Flip Pattern<Option> → Option<Pattern> | `sequence_option` | `Option<Pattern<T>>` | Yes (on None) |
| Flip Pattern<Result> → Result<Pattern> | `sequence_result` | `Result<Pattern<T>, E>` | Yes (first error) |

