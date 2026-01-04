# Type Signatures: Traversable Instance for Pattern

**Feature**: 010-traversable-instance  
**Date**: 2026-01-04

## Overview

This document defines the API contracts (type signatures) for all traversable operations on the Pattern type. Each method provides structure-preserving effectful transformation with proper effect sequencing.

## Core Traverse Operations

### traverse_option

Applies an effectful function returning Option to all pattern values, short-circuiting on first None.

```rust
impl<V> Pattern<V> {
    pub fn traverse_option<W, F>(&self, f: F) -> Option<Pattern<W>>
    where
        F: Fn(&V) -> Option<W>,
}
```

**Parameters**:
- `&self` - Borrows the pattern (non-consuming)
- `f: F where F: Fn(&V) -> Option<W>` - Function that may return None

**Returns**:
- `Option<Pattern<W>>` - Some if all transformations succeed, None if any fails

**Guarantees**:
- If returned Some, structure matches input (same size, depth, length)
- Short-circuits on first None encountered
- Processes values in depth-first, root-first order
- Does not modify original pattern

**Example**:
```rust
let pattern: Pattern<&str> = Pattern::pattern("1", vec![Pattern::point("2")]);
let result: Option<Pattern<i32>> = pattern.traverse_option(|s| s.parse().ok());
// result == Some(Pattern(1, [Pattern(2)]))
```

---

### traverse_result

Applies an effectful function returning Result to all pattern values, short-circuiting on first error.

```rust
impl<V> Pattern<V> {
    pub fn traverse_result<W, E, F>(&self, f: F) -> Result<Pattern<W>, E>
    where
        F: Fn(&V) -> Result<W, E>,
}
```

**Parameters**:
- `&self` - Borrows the pattern (non-consuming)
- `f: F where F: Fn(&V) -> Result<W, E>` - Function that may return Err

**Returns**:
- `Result<Pattern<W>, E>` - Ok if all transformations succeed, Err on first error

**Guarantees**:
- If returned Ok, structure matches input (same size, depth, length)
- Short-circuits on first Err encountered (fail-fast)
- Processes values in depth-first, root-first order
- Error type E is preserved from function
- Does not modify original pattern

**Example**:
```rust
let pattern: Pattern<&str> = Pattern::pattern("1", vec![Pattern::point("2")]);
let result: Result<Pattern<i32>, ParseIntError> = 
    pattern.traverse_result(|s| s.parse::<i32>());
// result == Ok(Pattern(1, [Pattern(2)]))
```

---

### validate

Applies an effectful function returning Result to all pattern values, collecting ALL errors instead of short-circuiting.

```rust
impl<V> Pattern<V> {
    pub fn validate<W, E, F>(&self, f: F) -> Result<Pattern<W>, Vec<E>>
    where
        F: Fn(&V) -> Result<W, E>,
}
```

**Parameters**:
- `&self` - Borrows the pattern (non-consuming)
- `f: F where F: Fn(&V) -> Result<W, E>` - Validation function that may return Err

**Returns**:
- `Result<Pattern<W>, Vec<E>>` - Ok if all succeed, Err with ALL errors if any fail

**Guarantees**:
- If returned Ok, structure matches input (same size, depth, length)
- Does NOT short-circuit - processes all values
- Collects all errors in Vec<E>
- Root value error appears first in error vec
- Processes values in depth-first, root-first order
- Does not modify original pattern

**Example**:
```rust
let pattern: Pattern<&str> = Pattern::pattern("1", vec![
    Pattern::point("invalid1"),
    Pattern::point("2"),
    Pattern::point("invalid2"),
]);
let result: Result<Pattern<i32>, Vec<_>> = 
    pattern.validate(|s| s.parse::<i32>().map_err(|e| e.to_string()));
// result == Err(vec!["invalid digit found in string", "invalid digit found in string"])
// Both errors collected, not just first
```

---

### traverse_future (Feature-gated: async)

Applies an async function to all pattern values sequentially, short-circuiting on first error.

```rust
#[cfg(feature = "async")]
impl<V> Pattern<V> {
    pub async fn traverse_future<W, E, F, Fut>(&self, f: F) -> Result<Pattern<W>, E>
    where
        F: Fn(&V) -> Fut,
        Fut: Future<Output = Result<W, E>>,
}
```

**Parameters**:
- `&self` - Borrows the pattern (non-consuming)
- `f: F where F: Fn(&V) -> Fut` - Function returning a Future

**Returns**:
- `Future<Result<Pattern<W>, E>>` - Async result, Ok if all succeed

**Guarantees**:
- If returned Ok, structure matches input (same size, depth, length)
- Executes sequentially (one value at a time), not concurrently
- Short-circuits on first error
- Processes values in depth-first, root-first order
- Preserves strict ordering guarantees
- Does not modify original pattern

**Example**:
```rust
let pattern: Pattern<UserId> = Pattern::pattern(id1, vec![Pattern::point(id2)]);
let result: Result<Pattern<User>, DbError> = 
    pattern.traverse_future(|id| async { fetch_user(id).await }).await;
// Fetches id1, then id2 (sequential), returns Result<Pattern<User>, DbError>
```

---

## Sequence Operations

### sequence_option

Flips the layers of Pattern<Option<T>> to Option<Pattern<T>>.

```rust
impl<V> Pattern<V> {
    pub fn sequence_option<W>(&self) -> Option<Pattern<W>>
    where
        V: AsRef<Option<W>>,
        W: Clone,
}
```

**Parameters**:
- `&self` - Pattern where values are Options

**Returns**:
- `Option<Pattern<W>>` - Some if all values are Some, None if any is None

**Guarantees**:
- Equivalent to `traverse_option(|opt| opt.clone())`
- If returned Some, structure matches input
- Short-circuits on first None

**Example**:
```rust
let pattern: Pattern<Option<i32>> = Pattern::pattern(
    Some(1),
    vec![Pattern::point(Some(2))],
);
let result: Option<Pattern<i32>> = pattern.sequence_option();
// result == Some(Pattern(1, [Pattern(2)]))
```

---

### sequence_result

Flips the layers of Pattern<Result<T, E>> to Result<Pattern<T>, E>.

```rust
impl<V> Pattern<V> {
    pub fn sequence_result<W, E>(&self) -> Result<Pattern<W>, E>
    where
        V: AsRef<Result<W, E>>,
        W: Clone,
        E: Clone,
}
```

**Parameters**:
- `&self` - Pattern where values are Results

**Returns**:
- `Result<Pattern<W>, E>` - Ok if all values are Ok, Err on first error

**Guarantees**:
- Equivalent to `traverse_result(|res| res.clone())`
- If returned Ok, structure matches input
- Short-circuits on first Err

**Example**:
```rust
let pattern: Pattern<Result<i32, String>> = Pattern::pattern(
    Ok(1),
    vec![Pattern::point(Ok(2))],
);
let result: Result<Pattern<i32>, String> = pattern.sequence_result();
// result == Ok(Pattern(1, [Pattern(2)]))
```

---

## Internal Helper Methods

These methods are `pub(crate)` or private, used for efficient recursion.

### traverse_option_with

```rust
impl<V> Pattern<V> {
    fn traverse_option_with<W, F>(&self, f: &F) -> Option<Pattern<W>>
    where
        F: Fn(&V) -> Option<W>,
}
```

**Purpose**: Internal helper for `traverse_option` that takes function by reference for efficient recursion.

**Why needed**: Avoids cloning closure on each recursive call.

---

### traverse_result_with

```rust
impl<V> Pattern<V> {
    fn traverse_result_with<W, E, F>(&self, f: &F) -> Result<Pattern<W>, E>
    where
        F: Fn(&V) -> Result<W, E>,
}
```

**Purpose**: Internal helper for `traverse_result` that takes function by reference for efficient recursion.

**Why needed**: Avoids cloning closure on each recursive call.

---

### validate_with

```rust
impl<V> Pattern<V> {
    fn validate_with<W, E, F>(&self, f: &F) -> Result<Pattern<W>, Vec<E>>
    where
        F: Fn(&V) -> Result<W, E>,
}
```

**Purpose**: Internal helper for `validate` that takes function by reference for efficient recursion.

**Why needed**: Avoids cloning closure on each recursive call.

---

## Trait Bounds Summary

### Function Bounds

All traverse methods use `Fn(&V) -> F<W>` where:
- `Fn` - Can be called multiple times (required for recursion)
- `&V` - Borrows value (doesn't consume it)
- `F<W>` - Returns effect-wrapped new value

**Why `Fn` and not `FnOnce` or `FnMut`**:
- Need to call function multiple times (once per value)
- Need to share function across recursive calls
- FnOnce would only work for single-value patterns
- FnMut would require mutable state management

### Value Bounds

- `V` - No bounds required for traverse operations (generic over any value type)
- `W` - No bounds required (can transform to any type)
- `E` - No bounds required (any error type works)

### Sequence Bounds

sequence operations require additional bounds:
- `V: AsRef<Option<W>>` or `V: AsRef<Result<W, E>>` - Values must be effects
- `W: Clone`, `E: Clone` - Need to clone inner values

**Why Clone**: Sequence needs to extract values from existing effects, which requires cloning.

---

## Method Comparison Table

| Method | Effect Type | Short-circuit | Error Collection | Async | Feature Gate |
|--------|-------------|---------------|------------------|-------|--------------|
| `traverse_option` | `Option<W>` | Yes (on None) | N/A | No | None |
| `traverse_result` | `Result<W, E>` | Yes (on Err) | First error only | No | None |
| `validate` | `Result<W, Vec<E>>` | No | All errors | No | None |
| `traverse_future` | `Future<Result<W, E>>` | Yes (on Err) | First error only | Yes (sequential) | `async` |
| `sequence_option` | `Option<W>` | Yes (on None) | N/A | No | None |
| `sequence_result` | `Result<W, E>` | Yes (on Err) | First error only | No | None |

---

## Usage Patterns

### Validation Pipeline

```rust
// Fail-fast: Stop on first error
pattern.traverse_result(|v| validate_strict(v))

// Collect all errors: Show user all issues
pattern.validate(|v| validate_and_report(v))
```

### Transformation Pipeline

```rust
// Pure transform → Effectful transform
pattern
    .map(|v| preprocess(v))
    .traverse_option(|v| try_parse(v))

// Effectful transform → Pure transform
pattern
    .traverse_result(|v| fetch_data(v))?
    .map(|v| format_data(v))
```

### Async Operations

```rust
// Sequential async fetches
pattern
    .traverse_future(|id| async { 
        database.fetch(id).await 
    })
    .await?
```

### Composition with Fold

```rust
// Transform → Aggregate
let sum: Option<i32> = pattern
    .traverse_option(|s| s.parse().ok())?
    .fold(0, |acc, n| acc + n);
```

---

## Type Inference Examples

Rust's type inference works well with traverse operations:

```rust
// Inferred from Result return type
let result = pattern.traverse_result(|s| s.parse::<i32>());
// result: Result<Pattern<i32>, ParseIntError>

// Inferred from Option context
if let Some(numbers) = pattern.traverse_option(|s| s.parse().ok()) {
    // numbers: Pattern<i32>
}

// Explicit turbofish when needed
let result: Result<Pattern<i32>, _> = pattern.traverse_result(|s| s.parse());
```

---

## Error Handling Examples

```rust
// Short-circuit pattern with ?
fn process(pattern: &Pattern<&str>) -> Result<Pattern<i32>, ParseIntError> {
    pattern.traverse_result(|s| s.parse::<i32>())
}

// Error context with map_err
pattern.traverse_result(|s| {
    s.parse::<i32>()
        .map_err(|e| format!("Failed to parse {}: {}", s, e))
})

// Multiple error types with custom enum
enum ValidationError {
    ParseError(ParseIntError),
    RangeError(String),
}

pattern.traverse_result(|s| {
    let n: i32 = s.parse().map_err(ValidationError::ParseError)?;
    if n < 0 || n > 100 {
        Err(ValidationError::RangeError(format!("{} out of range", n)))
    } else {
        Ok(n)
    }
})
```

---

## Performance Characteristics

| Method | Time Complexity | Space Complexity | Stack Usage | Allocations |
|--------|----------------|------------------|-------------|-------------|
| `traverse_option` | O(n) | O(n) | O(d) | n nodes |
| `traverse_result` | O(n) worst, O(k) best* | O(n) worst | O(d) | n nodes worst, k nodes best |
| `validate` | O(n) | O(n + e) | O(d) | n nodes + error vec |
| `traverse_future` | O(n × async_cost) | O(n) | O(d) | n nodes |
| `sequence_option` | O(n) | O(n) | O(d) | n nodes |
| `sequence_result` | O(n) worst, O(k) best* | O(n) worst | O(d) | n nodes worst, k nodes best |

Where:
- n = total number of nodes in pattern
- d = maximum nesting depth
- e = number of errors encountered (validate only)
- k = number of nodes processed before first error (best-case for short-circuit)
- async_cost = time for each async operation

*Short-circuit methods may process fewer than n nodes if error occurs early.

---

## WASM Compatibility

| Method | WASM Compatible | Notes |
|--------|-----------------|-------|
| `traverse_option` | ✅ Yes | Pure computation, no platform dependencies |
| `traverse_result` | ✅ Yes | Pure computation, no platform dependencies |
| `validate` | ✅ Yes | Pure computation, no platform dependencies |
| `traverse_future` | ⚠️ Conditional | Requires async runtime, feature-gated |
| `sequence_option` | ✅ Yes | Pure computation, no platform dependencies |
| `sequence_result` | ✅ Yes | Pure computation, no platform dependencies |

---

## Integration with Existing Pattern Operations

All traverse methods integrate smoothly with existing Pattern operations:

- **With Functor (map)**: Can chain map before or after traverse
- **With Foldable (fold)**: Can fold result of successful traverse
- **With Constructors**: Can traverse patterns created by any constructor
- **With Accessors**: Can access values/elements of traversed patterns

**Consistency**: Same borrowing model, same traversal order, same structure preservation guarantees.

