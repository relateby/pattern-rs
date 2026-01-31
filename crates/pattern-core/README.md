# pattern-core

Core pattern data structures for the gram-rs library.

This crate provides the foundational `Pattern<V>` type and `Subject` type, ported from the gram-hs reference implementation.

## Features

- **Pattern<V>**: A recursive, nested structure (s-expression-like) that is generic over value type `V`
- **Functor Instance**: Transform pattern values while preserving structure with the `map` method
- **Combinable Trait**: Associative combination operations for composing patterns
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

// Transform pattern values (Functor)
let pattern = Pattern::pattern("root", vec![
    Pattern::point("child1"),
    Pattern::point("child2"),
]);
let upper = pattern.map(|s| s.to_uppercase());
assert_eq!(upper.value, "ROOT");
assert_eq!(upper.elements[0].value, "CHILD1");

// Combine patterns (Combinable)
let p1 = Pattern::point("hello".to_string());
let p2 = Pattern::point(" world".to_string());
let combined = p1.combine(p2);
assert_eq!(combined.value(), "hello world");

// Combine multiple patterns using iterators
let patterns = vec![
    Pattern::point("a".to_string()),
    Pattern::point("b".to_string()),
    Pattern::point("c".to_string()),
];
let result = patterns.into_iter()
    .reduce(|acc, p| acc.combine(p))
    .unwrap();
assert_eq!(result.value(), "abc");
```

## WASM Compilation

This crate is fully compatible with WebAssembly targets and provides JavaScript/TypeScript bindings.

### Building for WASM

Use `wasm-pack` to build the WASM module with JavaScript bindings:

```bash
cd crates/pattern-core

# For web (browser)
wasm-pack build --target web --features wasm

# For Node.js
wasm-pack build --target nodejs --features wasm
```

This generates a `pkg/` directory with:
- `pattern_core_bg.wasm` - compiled WebAssembly module
- `pattern_core.js` - JavaScript glue code
- `pattern_core.d.ts` - TypeScript type definitions

### JavaScript/TypeScript Usage

```typescript
import init, { Pattern, Subject, Value } from 'pattern_core';

await init();

// Create patterns
const atomic = Pattern.point("hello");

// Create nested pattern using builder pattern
const nested = Pattern.pattern("parent");
nested.addElement(Pattern.point("child"));

// Transform and query
const doubled = nested.map(x => x + x);
const filtered = nested.filter(p => p.value.length > 3);

// Validation returns Either-like result
const rules = ValidationRules.new({ maxDepth: 10 });
const result = nested.validate(rules);

if (result._tag === 'Right') {
    console.log('Valid pattern');
} else {
    console.error('Validation failed:', result.left.message);
}
```

### TypeScript Generics

The TypeScript bindings provide full generic type safety with `Pattern<V>`:

```typescript
// Type inference works across transformations
const strings: Pattern<string> = Pattern.point("hello");
const lengths: Pattern<number> = strings.map(s => s.length); // ✓ type-safe

// Pattern<Subject> for graph patterns
const subject = Subject.new("n", ["Person"], { name: Value.string("Alice") });
const pattern: Pattern<Subject> = Pattern.point(subject);
```

### effect-ts Integration

Fallible operations (e.g., `validate`) return an Either-like shape that is directly compatible with [effect-ts](https://effect.website/):

```typescript
import { Either } from 'effect';
import { Pattern, ValidationRules } from 'pattern_core';

const pattern = Pattern.point("hello");
const rules = ValidationRules.new({ maxDepth: 10 });
const result = pattern.validate(rules); // Either<ValidationError, void>

// Use with effect-ts directly - no wrapper needed
Either.match(result, {
    onLeft: (err) => console.error('Failed:', err.message),
    onRight: () => console.log('Valid')
});

// Or use in Effect pipelines
import { pipe } from 'effect';

pipe(
    pattern.validate(rules),
    Either.map(() => pattern),
    Either.flatMap(p => p.validate(otherRules)),
    Either.match({
        onLeft: handleError,
        onRight: processValid
    })
);
```

**Return Shape**: `{ _tag: 'Right' | 'Left', right: T, left: E }`

This shape matches the effect-ts `Either` type exactly, so no conversion is needed. All fallible operations (validation, etc.) follow this pattern and **never throw exceptions**.

### WASM Compatibility Verification

All types in this crate are WASM-compatible:

- ✅ `Pattern<V>` - Uses only standard library types (Vec, generics)
- ✅ `Subject` - Uses only standard library types (HashSet, HashMap)
- ✅ `Symbol`, `Value`, `RangeValue` - Standard Rust types
- ✅ All traits (Clone, PartialEq, Eq, Debug, Display) - WASM-compatible

No platform-specific code is used. The crate compiles successfully for `wasm32-unknown-unknown` without any modifications.

### Examples

See `examples/pattern-core-wasm/` for complete examples:
- `browser.html` - browser usage with visual output
- `node.mjs` - Node.js usage with all operations
- `typescript-demo.ts` - TypeScript with full type safety

### Verification Status

- **Last Verified**: 2026-01-31
- **Target**: `wasm32-unknown-unknown`
- **Status**: ✅ Compiles successfully with WASM bindings
- **Features**: Full API parity with Python bindings, TypeScript generics, effect-ts compatibility

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

