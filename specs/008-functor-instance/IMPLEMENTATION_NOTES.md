# Implementation Notes: Idiomatic Functor for Pattern

## Key Decision: Direct Method vs Trait

After analyzing Rust idioms and standard library conventions, we've decided to implement Functor functionality as a **direct `map` method** on `Pattern<V>` rather than creating a `Functor` trait.

### Rationale

**Why NOT a Functor trait**:
1. Rust lacks Higher-Kinded Types (HKTs), making generic Functor traits awkward and verbose
2. Rust standard library doesn't have a Functor trait - it uses concrete methods
3. Most Rust developers wouldn't know what a "Functor" is, but they all know `map`
4. A trait would add complexity without providing value (no generic code would use it)

**Why a direct `map` method**:
1. ✅ Follows standard library conventions (`Option::map`, `Result::map`, `Iterator::map`)
2. ✅ Immediately understandable to any Rust developer
3. ✅ Works seamlessly with type inference
4. ✅ Simple to use and compose
5. ✅ No trait bounds needed in user code

### Comparison with Haskell

**Haskell** (abstract typeclass):
```haskell
class Functor f where
  fmap :: (a -> b) -> f a -> f b

instance Functor Pattern where
  fmap f (Pattern v es) = Pattern (f v) (map (fmap f) es)
```

**Rust** (concrete method with helper):
```rust
impl<V> Pattern<V> {
    // Public API - ergonomic
    pub fn map<W, F>(self, f: F) -> Pattern<W>
    where
        F: Fn(&V) -> W,
    {
        self.map_with(&f)
    }

    // Internal helper - efficient recursion
    fn map_with<W, F>(self, f: &F) -> Pattern<W>
    where
        F: Fn(&V) -> W,
    {
        Pattern {
            value: f(&self.value),
            elements: self.elements.into_iter().map(|e| e.map_with(f)).collect(),
        }
    }
}
```

**Implementation Note**: The helper function pattern allows the public API to take `F` by value (ergonomic, matches standard library conventions) while internal recursion uses `&F` (efficient, avoids cloning). This avoids requiring `F: Clone` bound, making the API more flexible.

### Behavioral Equivalence

Despite the syntactic differences, the implementations are **behaviorally equivalent**:

1. **Structure Preservation**: Both preserve the pattern structure (element count, nesting, order)
2. **Recursive Application**: Both apply the function recursively to all nested values
3. **Type Transformation**: Both allow transforming `Pattern<V>` to `Pattern<W>`
4. **Functor Laws**: Both satisfy identity and composition laws (verified via tests)

### Testing Strategy

We verify behavioral equivalence through property-based tests ported from gram-hs:

```rust
// Identity law: pattern.map(|x| x) == pattern
#[test]
fn identity_law() {
    proptest!(|(pattern: Pattern<i32>)| {
        let original = pattern.clone();
        let mapped = pattern.map(|x| x.clone());
        assert_eq!(original, mapped);
    });
}

// Composition law: pattern.map(|x| g(f(x))) == pattern.map(f).map(g)
#[test]
fn composition_law() {
    proptest!(|(pattern: Pattern<i32>)| {
        let f = |x: &i32| x * 2;
        let g = |x: &i32| x + 1;
        
        let composed = pattern.clone().map(|x| g(&f(x)));
        let sequential = pattern.map(f).map(g);
        
        assert_eq!(composed, sequential);
    });
}
```

### Usage Examples

The idiomatic Rust API is simpler and more intuitive:

```rust
// String transformation
let pattern = Pattern::point("hello");
let upper = pattern.map(|s| s.to_uppercase());

// Type conversion
let numbers = Pattern::point(42);
let strings = numbers.map(|n| n.to_string());

// Composition
let result = Pattern::point(5)
    .map(|n| n * 2)
    .map(|n| n + 1);
```

Compare to hypothetical trait-based approach:
```rust
// More verbose, less idiomatic
let result = pattern.fmap(|n| n * 2).fmap(|n| n + 1);
// or
let result = Functor::fmap(pattern, |n| n * 2);
```

## Conclusion

By implementing a direct `map` method following Rust conventions, we:
- ✅ Maintain complete behavioral equivalence with gram-hs
- ✅ Provide a more idiomatic and user-friendly API
- ✅ Follow Rust standard library patterns
- ✅ Simplify the implementation and usage

This demonstrates the principle: **port the concept and behavior, not the syntax**.

