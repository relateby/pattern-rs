# Subject Combination Strategies

## Overview

Subject now supports multiple combination strategies, each satisfying the semigroup laws (associativity). These strategies are implemented in Rust and exposed to Python through the pattern-core bindings.

## Rust Implementation

### 1. Default `Combinable` for `Subject` (Merge Strategy)

```rust
impl Combinable for Subject {
    fn combine(self, other: Self) -> Self {
        Subject {
            identity: self.identity,  // Keep first identity (leftmost)
            labels: self.labels.union(&other.labels).cloned().collect(),  // Union
            properties: {
                let mut props = self.properties;
                props.extend(other.properties);  // Right overwrites
                props
            }
        }
    }
}
```

**Associativity**: ✅
- Identity selection is associative (always picks leftmost)
- Set union is associative
- Property merge with right-bias is associative

**Use cases**: Merging subjects with complementary information

### 2. `FirstSubject` (First Wins Strategy)

```rust
#[derive(Clone, PartialEq)]
pub struct FirstSubject(pub Subject);

impl Combinable for FirstSubject {
    fn combine(self, _other: Self) -> Self {
        self  // Always return first, discard second
    }
}
```

**Associativity**: ✅ Trivially associative: `first(first(a, b), c) = first(a, first(b, c)) = a`

**Use cases**: Keep initial subject, ignore subsequent ones

### 3. `LastSubject` (Last Wins Strategy)

```rust
#[derive(Clone, PartialEq)]
pub struct LastSubject(pub Subject);

impl Combinable for LastSubject {
    fn combine(self, other: Self) -> Self {
        other  // Always return second, discard first
    }
}
```

**Associativity**: ✅ Trivially associative: `last(last(a, b), c) = last(a, last(b, c)) = c`

**Use cases**: Most recent subject takes precedence

### 4. `EmptySubject` (Empty/Identity Strategy)

```rust
#[derive(Clone, PartialEq)]
pub struct EmptySubject(pub Subject);

impl Combinable for EmptySubject {
    fn combine(self, _other: Self) -> Self {
        EmptySubject(Subject {
            identity: Symbol("_".to_string()),
            labels: Default::default(),
            properties: Default::default(),
        })
    }
}

impl Default for EmptySubject {
    fn default() -> Self {
        EmptySubject(Subject {
            identity: Symbol("_".to_string()),
            labels: Default::default(),
            properties: Default::default(),
        })
    }
}
```

**Associativity**: ✅ Trivially associative: `empty(empty(a, b), c) = empty(a, empty(b, c)) = empty`

**Monoid Laws**: ✅
- Left identity: `empty.combine(s) = empty`
- Right identity: `s.combine(empty) = empty`

**Use cases**: Anonymous subjects, identity element for monoid operations

## Python Bindings

### Usage

```python
import pattern_core

# Create subjects
s1 = pattern_core.Subject(
    identity="alice",
    labels={"Person"},
    properties={"name": pattern_core.Value.string("Alice")}
)
s2 = pattern_core.Subject(
    identity="bob",
    labels={"Employee"},
    properties={"role": pattern_core.Value.string("Engineer")}
)

# Create patterns
p1 = pattern_core.PatternSubject.point(s1)
p2 = pattern_core.PatternSubject.point(s2)

# Strategy 1: Merge (default)
merged = p1.combine(p2)
# Result: identity="alice", labels={"Person", "Employee"}, 
#         properties={"name": "Alice", "role": "Engineer"}

# Strategy 2: First wins
first = p1.combine(p2, strategy="first")
# Result: identity="alice", labels={"Person"}, properties={"name": "Alice"}

# Strategy 3: Last wins
last = p1.combine(p2, strategy="last")
# Result: identity="bob", labels={"Employee"}, properties={"role": "Engineer"}

# Strategy 4: Empty (anonymous)
empty = p1.combine(p2, strategy="empty")
# Result: identity="_", labels={}, properties={}

# Custom function
def custom_merge(s1, s2):
    """Custom combination logic"""
    return pattern_core.Subject(
        identity=s1.identity,
        labels=s1.get_labels() | s2.get_labels(),
        properties={**s1.get_properties(), **s2.get_properties()}
    )

custom = p1.combine(p2, combine_func=custom_merge)
```

### API Signature

```python
def combine(
    self,
    other: PatternSubject,
    *,
    strategy: str = "merge",
    combine_func: Optional[Callable[[Subject, Subject], Subject]] = None
) -> PatternSubject:
    """
    Combine two patterns associatively.
    
    Args:
        other: The pattern to combine with
        strategy: Combination strategy ("merge", "first", "last", "empty")
        combine_func: Optional custom function (Subject, Subject) -> Subject
                     If provided, overrides the strategy parameter
    
    Returns:
        PatternSubject: Combined pattern with concatenated elements
    """
```

## Design Benefits

1. **Rust-First**: Strategies defined in Rust with strong type safety
2. **Performance**: Rust implementations are optimized
3. **Consistency**: Same semantics in Rust and Python
4. **Flexibility**: Python users can override with custom functions
5. **Semigroup Laws**: All strategies satisfy associativity
6. **Monoid Laws**: `EmptySubject` provides identity element

## Pattern Combination

All strategies combine patterns by:
1. **Value Combination**: Using the selected strategy for Subject values
2. **Element Concatenation**: Concatenating elements from both patterns (left first, then right)

```rust
// General pattern combination structure
Pattern {
    value: strategy.combine(p1.value, p2.value),
    elements: [p1.elements..., p2.elements...]
}
```

This ensures that:
- Pattern structure is preserved
- Element order is maintained
- Associativity holds for the entire pattern

## Testing

All strategies have comprehensive tests verifying:
- ✅ Associativity: `(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)`
- ✅ Correct value combination
- ✅ Element concatenation
- ✅ Edge cases (empty labels, empty properties)

Run tests:
```bash
cargo test --package pattern-core --lib
```
