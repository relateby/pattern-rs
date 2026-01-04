# Data Model: Traversable Instance for Pattern

**Feature**: 010-traversable-instance  
**Date**: 2026-01-04

## Overview

This document defines the data model for traversable operations on the Pattern type. Traversable enables structure-preserving effectful transformations, where effects (Option, Result, Future) are sequenced and combined according to their semantics while maintaining the pattern's structural integrity.

## Core Concepts

### Effect Types

**Effect Type F<T>**: A computation context that may produce a value of type T along with some effect.

Examples:
- `Option<T>` - May or may not have a value (effect: optionality)
- `Result<T, E>` - May succeed with T or fail with E (effect: error handling)
- `Future<T>` - Will eventually produce T (effect: asynchrony)

### Traversable Transformation

**Concept**: Apply an effectful function to all values in a pattern, sequencing the effects and rebuilding the pattern structure.

**Input**: Pattern<V> + effectful function (V → F<W>)
**Output**: F<Pattern<W>>

**Key Properties**:
1. **Structure preservation**: Pattern shape unchanged (element count, nesting depth, order)
2. **Effect sequencing**: Effects combined according to effect type semantics
3. **Order guarantee**: Values processed depth-first, root-first (same as Foldable)

## Type Relationships

### Pattern Type

```rust
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

**Invariants**:
- Pattern always has exactly one value
- Elements vec may be empty (atomic pattern)
- Recursive structure allows arbitrary nesting

### Transformation Types

#### Option Transformation

```
Pattern<V> + (V → Option<W>) → Option<Pattern<W>>
```

**Semantics**: All-or-nothing
- If all transformations return Some, result is Some(Pattern<W>)
- If any transformation returns None, entire result is None
- Short-circuits on first None

**Example**:
```
Pattern("1", [Pattern("2")]) + parse_int
  → All values parse successfully
  → Some(Pattern(1, [Pattern(2)]))

Pattern("1", [Pattern("invalid")]) + parse_int
  → Second value fails to parse
  → None
```

#### Result Transformation

```
Pattern<V> + (V → Result<W, E>) → Result<Pattern<W>, E>
```

**Semantics**: Fail-fast
- If all transformations return Ok, result is Ok(Pattern<W>)
- If any transformation returns Err, entire result is Err(e)
- Short-circuits on first error

**Example**:
```
Pattern("1", [Pattern("2")]) + parse_int_result
  → All values parse successfully
  → Ok(Pattern(1, [Pattern(2)]))

Pattern("1", [Pattern("invalid"), Pattern("2")]) + parse_int_result
  → Second value fails
  → Err(ParseIntError)
  → Third value not processed (short-circuit)
```

#### Result Transformation (Validation)

```
Pattern<V> + (V → Result<W, E>) → Result<Pattern<W>, Vec<E>>
```

**Semantics**: Collect all errors
- If all transformations return Ok, result is Ok(Pattern<W>)
- If any transformations return Err, result is Err(Vec<E>) with all errors
- Does NOT short-circuit, processes all values

**Example**:
```
Pattern("1", [Pattern("invalid1"), Pattern("2"), Pattern("invalid2")]) + parse_int_result
  → Processes all values
  → Collects errors from invalid1 and invalid2
  → Err(vec![ParseIntError, ParseIntError])
```

#### Future Transformation (Async)

```
Pattern<V> + (V → Future<Result<W, E>>) → Future<Result<Pattern<W>, E>>
```

**Semantics**: Sequential async operations
- Processes values one at a time in order
- Awaits each Future before proceeding to next
- Short-circuits on first error
- Preserves traversal order

**Example**:
```
Pattern(id1, [Pattern(id2)]) + async_fetch_entity
  → await fetch(id1)  // Root first
  → await fetch(id2)  // Then elements
  → Future<Result<Pattern<Entity>, FetchError>>
```

### Sequence Operations

**Concept**: Flip the layers of nested structures

```
Pattern<F<W>> → F<Pattern<W>>
```

Where F is an effect type (Option, Result, etc.)

**Implementation**: Sequence is traverse with identity function
```rust
sequence_option() ≡ traverse_option(|opt| opt.clone())
```

**Examples**:

```
Pattern<Option<i32>>  →  Option<Pattern<i32>>
Pattern(Some(1), [Pattern(Some(2))])  →  Some(Pattern(1, [Pattern(2)]))
Pattern(Some(1), [Pattern(None)])     →  None

Pattern<Result<i32, E>>  →  Result<Pattern<i32>, E>
Pattern(Ok(1), [Pattern(Ok(2))])  →  Ok(Pattern(1, [Pattern(2)]))
Pattern(Ok(1), [Pattern(Err(e))]) →  Err(e)
```

## Data Flow

### traverse_option Data Flow

```
Input: Pattern<V>
       Function: V → Option<W>

Processing Order (depth-first, root-first):
1. Apply function to root value
   - If None → return None (short-circuit)
   - If Some(w) → continue with w
   
2. For each element (left to right):
   - Recursively traverse element
   - If any returns None → return None (short-circuit)
   - Collect all Some results
   
3. Build new pattern:
   - Pattern { value: new_root, elements: new_elements }
   
4. Wrap in Some:
   - Some(new_pattern)

Output: Option<Pattern<W>>
```

### traverse_result Data Flow

```
Input: Pattern<V>
       Function: V → Result<W, E>

Processing Order (depth-first, root-first):
1. Apply function to root value
   - If Err(e) → return Err(e) (short-circuit)
   - If Ok(w) → continue with w
   
2. For each element (left to right):
   - Recursively traverse element
   - If any returns Err(e) → return Err(e) (short-circuit)
   - Collect all Ok results
   
3. Build new pattern:
   - Pattern { value: new_root, elements: new_elements }
   
4. Wrap in Ok:
   - Ok(new_pattern)

Output: Result<Pattern<W>, E>
```

### validate Data Flow

```
Input: Pattern<V>
       Function: V → Result<W, E>

Processing Order (depth-first, root-first):
1. Apply function to root value
   - Record Ok(w) or Err(e)
   
2. For each element (left to right):
   - Recursively validate element
   - Collect successful patterns
   - Accumulate all errors
   
3. If errors accumulated:
   - Return Err(all_errors)
   
4. If no errors:
   - Build new pattern: Pattern { value: new_root, elements: new_elements }
   - Return Ok(new_pattern)

Output: Result<Pattern<W>, Vec<E>>
```

## Structural Invariants

### Structure Preservation

For any traverse operation, if transformation succeeds:
```
input_pattern.size() == output_pattern.size()
input_pattern.depth() == output_pattern.depth()
input_pattern.length() == output_pattern.length()
```

**Element order preserved**:
```
input_pattern.elements[i].value → output_pattern.elements[i].value
```

**Recursion depth**:
```
If input has N levels of nesting, output has N levels of nesting
```

### Value Correspondence

Each value in output corresponds to exactly one value in input:
```
input_pattern.values().len() == output_pattern.values().len()
```

Values processed in same order as Foldable:
```
input_pattern.values()[i] transformed → output_pattern.values()[i]
```

## Effect Sequencing

### Option Sequencing

Uses `Option`'s monadic bind (`and_then`/`?` operator):
```
Some(v1) *and* Some(v2) *and* ... → Some((v1, v2, ...))
Some(v1) *and* None *and* ...     → None
```

Sequencing happens via `Iterator::collect::<Option<Vec<_>>>()`:
```rust
let elements: Option<Vec<Pattern<W>>> = self.elements
    .iter()
    .map(|e| e.traverse_option_with(f))
    .collect();  // Short-circuits on first None
```

### Result Sequencing

Uses `Result`'s monadic bind (`and_then`/`?` operator):
```
Ok(v1) *and* Ok(v2) *and* ... → Ok((v1, v2, ...))
Ok(v1) *and* Err(e) *and* ... → Err(e)
```

Sequencing happens via `Iterator::collect::<Result<Vec<_>, E>>()`:
```rust
let elements: Result<Vec<Pattern<W>>, E> = self.elements
    .iter()
    .map(|e| e.traverse_result_with(f))
    .collect();  // Short-circuits on first error
```

### Future Sequencing (Sequential)

Uses sequential await (not parallel join):
```rust
for element in &self.elements {
    let new_elem = element.traverse_future(&f).await?;
    new_elements.push(new_elem);
}
```

**Guarantees**:
- Operations happen in order (root, then elements left-to-right)
- Each future completes before next starts
- Errors propagate immediately (short-circuit)

## Error Handling

### Short-Circuit Errors (traverse_result)

**Behavior**: Return immediately on first error
**Type**: `Result<Pattern<W>, E>` - single error
**Use case**: Fail-fast validation, early termination on error

**Properties**:
- Minimal computation (stops at first failure)
- Single error reported
- Order-dependent (which error you get depends on traversal order)

### Accumulated Errors (validate)

**Behavior**: Process all values, collect all errors
**Type**: `Result<Pattern<W>, Vec<E>>` - multiple errors
**Use case**: Comprehensive validation, show all issues to user

**Properties**:
- Maximum computation (processes all values)
- All errors reported
- Better user feedback (fix all issues at once)

**Error ordering**: Root value errors appear first, then element errors in traversal order

## Integration with Pattern Operations

### With Functor (map)

```rust
// Map then traverse
let result: Option<Pattern<i32>> = pattern
    .map(|s: &str| s.trim())      // Pure transformation
    .traverse_option(|s| s.parse().ok());  // Effectful transformation

// Traverse then map
let result: Option<Pattern<String>> = pattern
    .traverse_option(|s| s.parse::<i32>().ok())  // Effect first
    .map(|p| p.map(|n| n.to_string()));          // Pure transformation after
```

### With Foldable (fold)

```rust
// Traverse then fold
let result: Option<i32> = pattern
    .traverse_option(|s| s.parse::<i32>().ok())  // Transform with effects
    .map(|p| p.fold(0, |acc, n| acc + n));       // Aggregate results

// Can't fold then traverse (fold consumes structure)
```

### Composition Pattern

```
Pattern<V>
  → map(preprocess)           -- Pure transformation
  → traverse_result(validate) -- Effectful transformation
  → map(map(postprocess))     -- Pure transformation on result
  → fold inside               -- Aggregation
```

This composition enables powerful data processing pipelines with proper effect handling.

## Memory Considerations

### Stack Usage

Each recursive level uses stack space for:
- Function call frame
- New value (W)
- Temporary Vec<Pattern<W>> for elements

**Depth limit**: Should handle 100+ levels without overflow (tested)

### Heap Allocations

- New Pattern<W> allocated for each node
- Vec<Pattern<W>> allocated for element collections
- Effect wrappers (Option/Result/Future) are stack-allocated

**Total allocations**: O(n) where n is pattern size

### Performance

**Time complexity**: O(n) where n is total nodes
- Each value processed exactly once
- Effect sequencing overhead depends on effect type
  - Option/Result: Minimal (just checking variants)
  - Future: Async runtime overhead (I/O, scheduling)

**Space complexity**: O(n) for new pattern + O(d) for recursion stack where d is depth

## Testing Considerations

### Property Testing

Test that transformations preserve structure:
```rust
∀ pattern, f: V → Option<W>
  pattern.traverse_option(f).map(|p| p.size()) == Some(pattern.size())
```

### Order Testing

Test that traversal order matches Foldable:
```rust
pattern.values() corresponds to pattern.traverse_result(track_order)
```

### Effect Testing

Test effect sequencing:
```rust
// Option: first None short-circuits
// Result: first Err short-circuits
// validate: collects all errors
```

### Law Testing

Test traversable laws for each effect type:
```rust
// Identity: traverse(Some) == Some(pattern)
// Composition: complex, adapt for Rust
// Naturality: natural transformations commute
```

