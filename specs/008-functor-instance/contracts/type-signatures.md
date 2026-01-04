# Type Signatures: Functor Instance for Pattern

**Feature**: 008-functor-instance  
**Date**: 2026-01-04  
**Module**: `pattern_core::pattern`

## Core Method Signature

### `Pattern::map`

**Primary transformation method for Pattern<V>**

```rust
impl<V> Pattern<V> {
    pub fn map<W, F>(self, f: F) -> Pattern<W>
    where
        F: Fn(&V) -> W;
}
```

**Type Parameters**:
- `V`: Input value type (pattern being transformed)
- `W`: Output value type (resulting pattern)
- `F`: Function type for transformation

**Parameters**:
- `self: Pattern<V>` - The pattern to transform (consumed)
- `f: F` - Transformation function

**Return**:
- `Pattern<W>` - New pattern with transformed values

**Trait Bounds**:
- `F: Fn(&V) -> W` - Function must:
  - Take reference to input value (`&V`)
  - Return new output value (`W`)
  - Be callable multiple times (for recursion)

**Ownership**:
- Consumes `self` (takes ownership of input pattern)
- Returns new `Pattern<W>` (transfers ownership to caller)
- Function `f` is borrowed (via reference in recursive calls)

---

## Usage Examples with Type Annotations

### Example 1: Same-Type Transformation

```rust
fn transform_strings(pattern: Pattern<String>) -> Pattern<String> {
    pattern.map(|s: &String| s.to_uppercase())
}

// Type flow:
// Pattern<String> → (Fn(&String) -> String) → Pattern<String>
```

### Example 2: Type Conversion

```rust
fn stringify_numbers(pattern: Pattern<i32>) -> Pattern<String> {
    pattern.map(|n: &i32| n.to_string())
}

// Type flow:
// Pattern<i32> → (Fn(&i32) -> String) → Pattern<String>
```

### Example 3: Complex Transformation

```rust
#[derive(Clone, Debug, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

fn extract_names(pattern: Pattern<Person>) -> Pattern<String> {
    pattern.map(|person: &Person| person.name.clone())
}

// Type flow:
// Pattern<Person> → (Fn(&Person) -> String) → Pattern<String>
```

### Example 4: Chained Transformations

```rust
fn process_data(pattern: Pattern<i32>) -> Pattern<String> {
    pattern
        .map(|n: &i32| n * 2)           // Pattern<i32> → Pattern<i32>
        .map(|n: &i32| n + 1)           // Pattern<i32> → Pattern<i32>
        .map(|n: &i32| format!("{}", n)) // Pattern<i32> → Pattern<String>
}

// Type flow:
// Pattern<i32> → Pattern<i32> → Pattern<i32> → Pattern<String>
```

---

## Functor Laws (Type Level)

### Identity Law

```rust
fn identity_law<V: Clone + PartialEq>(pattern: Pattern<V>) -> bool {
    let original = pattern.clone();
    let transformed = pattern.map(|x: &V| x.clone());
    original == transformed
}

// Type signature: Pattern<V> → Pattern<V> (when f is identity)
```

### Composition Law

```rust
fn composition_law<V, W, X>(
    pattern: Pattern<V>,
    f: impl Fn(&V) -> W,
    g: impl Fn(&W) -> X,
) -> bool
where
    V: Clone,
    W: PartialEq,
    X: PartialEq,
{
    let composed = pattern.clone().map(|x: &V| g(&f(x)));
    let sequential = pattern.map(f).map(g);
    composed == sequential
}

// Type flow (composed):   Pattern<V> → Pattern<X>
// Type flow (sequential): Pattern<V> → Pattern<W> → Pattern<X>
```

---

## Type Constraints and Requirements

### Input Type Constraints

The input pattern type `Pattern<V>` has **no constraints** on `V`:
- `V` can be any type
- No trait bounds required on `V`
- Enables maximum flexibility

```rust
// All valid:
let p1: Pattern<i32> = Pattern::point(42);
let p2: Pattern<String> = Pattern::point("hello".to_string());
let p3: Pattern<Vec<u8>> = Pattern::point(vec![1, 2, 3]);
let p4: Pattern<MyCustomType> = Pattern::point(MyCustomType::new());
```

### Output Type Constraints

The output type `W` has **no constraints**:
- Function `f` determines `W` by its return type
- Type inference usually determines `W` automatically
- Can be same as `V` or different

### Function Constraints

The function `F` must satisfy `Fn(&V) -> W`:
- **`Fn`**: Can be called multiple times (required for recursion)
- **`&V`**: Takes reference (non-destructive inspection)
- **`W`**: Returns owned value (enables type transformation)

**Why `Fn` and not `FnMut` or `FnOnce`?**
- `FnOnce`: ❌ Can only be called once (fails on first recursive call)
- `FnMut`: ❌ Requires mutable borrow (complicates recursion)
- `Fn`: ✅ Can be called multiple times, no mutation needed

### Equality Constraints (for laws)

Functor laws require `PartialEq`:
```rust
impl<V: PartialEq> Pattern<V> {
    // Pattern equality already implemented in feature 004
}
```

This is only needed for testing, not for the `map` method itself.

---

## Relationship to gram-hs Type Signatures

### Haskell Signature

```haskell
fmap :: Functor f => (a -> b) -> f a -> f b

-- For Pattern specifically:
fmap :: (a -> b) -> Pattern a -> Pattern b
```

### Rust Signature

```rust
pub fn map<W, F>(self, f: F) -> Pattern<W>
where
    F: Fn(&V) -> W
```

### Semantic Mapping

| Haskell | Rust | Notes |
|---------|------|-------|
| `a` | `V` | Input value type |
| `b` | `W` | Output value type |
| `Pattern a` | `Pattern<V>` | Input pattern type |
| `Pattern b` | `Pattern<W>` | Output pattern type |
| `(a -> b)` | `Fn(&V) -> W` | Function type |
| `fmap` | `map` | Method name (Rust convention) |

**Key Differences**:
1. **Method vs function**: Rust uses method syntax (`pattern.map(f)`) vs Haskell's function syntax (`fmap f pattern`)
2. **Reference**: Rust function takes `&V` (reference) while Haskell takes `a` (value). This is a Rust idiom for efficiency.
3. **Ownership**: Rust makes ownership explicit (consumes `self`), Haskell's laziness handles this differently

**Behavioral Equivalence**:
Despite syntax differences, the semantics are identical:
- Both transform all values in the pattern
- Both preserve structure
- Both satisfy functor laws
- Both enable type transformations

---

## Type Inference Examples

Rust's type inference often eliminates need for explicit annotations:

### Inference from Return Type

```rust
fn double_ints(p: Pattern<i32>) -> Pattern<i32> {
    p.map(|n| n * 2)  // Type inferred: Fn(&i32) -> i32
}
```

### Inference from Closure Body

```rust
let p: Pattern<i32> = Pattern::point(42);
let result = p.map(|n| n.to_string());  // Infers: Pattern<String>
```

### Explicit Annotations When Needed

```rust
let p: Pattern<i32> = Pattern::point(42);
let result: Pattern<String> = p.map(|n: &i32| n.to_string());
//       ^^^^^^^^^^^^^^              ^^^^^   explicit types
```

---

## Error Cases and Type Safety

### Compile-Time Type Errors

These will not compile:

```rust
// Error: FnOnce cannot be called multiple times
let f = |x: i32| { println!("consume"); x };
pattern.map(f);  // ❌ Compile error

// Error: Mismatched types
let p: Pattern<i32> = Pattern::point(42);
let result: Pattern<String> = p.map(|n| n * 2);  // ❌ Returns i32, not String

// Error: Function doesn't match signature
let f = |x: &i32, y: &i32| x + y;  // Takes 2 args
pattern.map(f);  // ❌ Expected Fn(&V) -> W
```

### Runtime Behavior

If the transformation function panics:
```rust
let p: Pattern<i32> = Pattern::pattern(1, vec![Pattern::point(2)]);
let result = p.map(|n| {
    if *n == 2 { panic!("Error!"); }
    n * 2
});
// Panics when transforming second element
// Partially transformed pattern is dropped (no memory leak)
```

---

## Summary

The `Pattern::map` method signature provides:

✅ **Type Safety**: All transformations type-checked at compile time  
✅ **Flexibility**: No constraints on input/output types  
✅ **Efficiency**: Consumes input, avoids unnecessary copies  
✅ **Composability**: Can chain multiple transformations  
✅ **Correctness**: Type system enforces functor invariants  
✅ **Idioms**: Follows Rust conventions for `map` methods

The signature balances:
- **Generality**: Works with any types `V` and `W`
- **Safety**: Compiler prevents invalid transformations
- **Ergonomics**: Type inference minimizes annotations
- **Performance**: Zero-copy when possible

This contract ensures the implementation is both correct (satisfies functor laws) and idiomatic (follows Rust conventions).

