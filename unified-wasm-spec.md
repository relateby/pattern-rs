# Unified Gram WASM Package

**Feature**: 028-unified-gram-wasm  
**Date**: 2026-01-31  
**Status**: Draft

## Goal

Provide a seamless JavaScript/TypeScript experience for working with gram, analogous to how `JSON.stringify()` and `JSON.parse()` work with JavaScript objects.

```typescript
import { Pattern, Subject, Value, Gram } from 'gram';

const person = Subject.new("alice", ["Person"], { name: Value.string("Alice") });
const pattern = Pattern.point(person);

// Serialize - like JSON.stringify()
const text = Gram.stringify(pattern);

// Parse - like JSON.parse()  
const parsed = Gram.parse(text);
```

## Design Principles

1. **Single import** - One package, one entry point
2. **Domain-focused API** - Users work with `Pattern<Subject>`, not parser ASTs
3. **Conversion is invisible** - Internal detail, never exposed
4. **Familiar idioms** - Mirror JSON API conventions

## Public API

### Core Types (re-exported from pattern-core)

```typescript
class Pattern<V> {
  static point<V>(value: V): Pattern<V>;
  static of<V>(value: V): Pattern<V>;
  static pattern<V>(value: V, elements: Pattern<V>[]): Pattern<V>;
  
  value: V;
  elements: Pattern<V>[];
  
  map<W>(fn: (v: V) => W): Pattern<W>;
  // ... other pattern operations
}

class Subject {
  static new(identity: string, labels?: string[], properties?: Record<string, Value>): Subject;
  identity: string;
  labels: Set<string>;
  properties: Record<string, Value>;
}

namespace Value {
  function string(s: string): Value;
  function int(n: number): Value;
  function decimal(n: number): Value;
  function boolean(b: boolean): Value;
  // ... other value factories
}
```

### Gram Namespace (new)

```typescript
namespace Gram {
  /** Serialize a Pattern<Subject> to gram notation */
  function stringify(pattern: Pattern<Subject>): string;
  
  /** Serialize multiple patterns to gram notation */
  function stringify(patterns: Pattern<Subject>[]): string;
  
  /** Parse gram notation into Pattern<Subject> */
  function parse(gram: string): Pattern<Subject>[];
  
  /** Parse gram notation, returning first pattern or null */
  function parseOne(gram: string): Pattern<Subject> | null;
}
```

## Serializing Arbitrary Pattern<V>

`Gram.stringify()` only accepts `Pattern<Subject>` because gram notation is specifically designed for graph data (nodes with identity, labels, properties).

For arbitrary `Pattern<V>`, users must map to `Pattern<Subject>` first:

```typescript
// Pattern<string> cannot be directly serialized
const strings: Pattern<string> = Pattern.pattern("root", [
  Pattern.point("child1"),
  Pattern.point("child2")
]);

// Map to Pattern<Subject> first
const asSubjects: Pattern<Subject> = strings.map(s => 
  Subject.new(s, ["StringValue"], { raw: Value.string(s) })
);

const text = Gram.stringify(asSubjects); // Now works
```

### Conventional Mapping: `Gram.from()`

For convenience, provide a helper that applies conventional mappings:

```typescript
namespace Gram {
  /** Convert Pattern<V> to Pattern<Subject> using conventional mapping */
  function from<V>(pattern: Pattern<V>, options?: FromOptions): Pattern<Subject>;
}

interface FromOptions {
  /** Label to apply to converted subjects (default: type name or "Value") */
  label?: string;
  /** Property name for the original value (default: "value") */
  valueProperty?: string;
  /** Custom identity generator (default: auto-generated) */
  identity?: (value: V, index: number) => string;
}
```

**Conventional mappings:**

| Source Type | Subject Identity | Labels | Properties |
|-------------|------------------|--------|------------|
| `string` | auto-generated | `["String"]` | `{ value: Value.string(s) }` |
| `number` | auto-generated | `["Number"]` | `{ value: Value.decimal(n) }` |
| `boolean` | auto-generated | `["Boolean"]` | `{ value: Value.boolean(b) }` |
| `object` | `obj.id` or auto | `["Object"]` | object entries as properties |
| `Subject` | passthrough | passthrough | passthrough |

**Example:**

```typescript
const numbers: Pattern<number> = Pattern.pattern(1, [
  Pattern.point(2),
  Pattern.point(3)
]);

// Conventional mapping
const asGram = Gram.from(numbers);
const text = Gram.stringify(asGram);
// (_0:Number {value: 1}) (_1:Number {value: 2}) (_2:Number {value: 3})

// Or combined
const text = Gram.stringify(Gram.from(numbers));
```

## Implementation

### Crate Structure

```
crates/gram-wasm/
├── Cargo.toml          # Depends on pattern-core, gram-codec
├── src/
│   ├── lib.rs          # WASM entry point
│   ├── gram.rs         # Gram namespace (stringify, parse, from)
│   └── convert.rs      # Pattern<Subject> ↔ GramPattern conversion
└── typescript/
    └── gram.d.ts       # Unified TypeScript definitions
```

### Dependencies

```toml
[dependencies]
pattern-core = { path = "../pattern-core" }
gram-codec = { path = "../gram-codec" }
wasm-bindgen = "0.2"
js-sys = "0.3"
```

### Internal Conversion

The conversion between `Pattern<Subject>` and gram-codec AST is internal:

```rust
// Never exposed to users
impl From<&Pattern<Subject>> for GramPattern { ... }
impl From<&GramPattern> for Pattern<Subject> { ... }
```

## TypeScript Package

### Single Entry Point

```typescript
// index.d.ts - everything from one import
export { Pattern } from './pattern';
export { Subject } from './subject';
export { Value } from './value';
export { Gram } from './gram';

// Also export types
export type { ValidationRules, StructureAnalysis, Either, ValidationError } from './types';
```

### Package Name Options

- `gram` (if available on npm)
- `@gram-data/gram`
- `gram-js`

## Usage Examples

### Basic Graph Construction

```typescript
import { Pattern, Subject, Value, Gram } from 'gram';

// Create nodes
const alice = Subject.new("alice", ["Person"], { 
  name: Value.string("Alice"),
  age: Value.int(30)
});
const bob = Subject.new("bob", ["Person"], { 
  name: Value.string("Bob") 
});

// Create relationship
const knows = Subject.new("r1", ["KNOWS"], { 
  since: Value.int(2020) 
});

// Build pattern (alice knows bob)
const graph = Pattern.pattern(alice, [
  Pattern.pattern(knows, [Pattern.point(bob)])
]);

// Serialize
const gramText = Gram.stringify(graph);
console.log(gramText);
// (alice:Person {name: "Alice", age: 30})-[r1:KNOWS {since: 2020}]->(bob:Person {name: "Bob"})
```

### Parse and Transform

```typescript
import { Pattern, Gram } from 'gram';

// Parse gram notation
const patterns = Gram.parse(`
  (a:Person {name: "Alice"})-[:KNOWS]->(b:Person {name: "Bob"})
`);

// Transform (e.g., anonymize)
const anonymized = patterns[0].map(subject => 
  Subject.new(
    `anon_${subject.identity}`,
    subject.labels,
    { redacted: Value.boolean(true) }
  )
);

// Serialize back
console.log(Gram.stringify(anonymized));
```

### Converting Generic Patterns

```typescript
import { Pattern, Gram } from 'gram';

// Some computation produces Pattern<number>
const tree: Pattern<number> = Pattern.pattern(10, [
  Pattern.pattern(5, [Pattern.point(2), Pattern.point(3)]),
  Pattern.point(5)
]);

// Convert and serialize
const gramText = Gram.stringify(Gram.from(tree, { 
  label: "TreeNode",
  valueProperty: "weight"
}));
```

## Non-Goals

- **Exposing gram-codec AST** - Users should not need to know about `GramPattern`, `GramNode`, etc.
- **Arbitrary serialization formats** - This package is specifically for gram notation
- **Graph database protocols** - Bolt, HTTP APIs, etc. are separate concerns

## Success Criteria

1. Single `import { ... } from 'gram'` provides complete API
2. `Gram.stringify(pattern)` and `Gram.parse(text)` feel as natural as JSON methods
3. TypeScript provides full type safety with `Pattern<Subject>`
4. Users never encounter internal AST types
5. Round-trip: `Gram.parse(Gram.stringify(p))` preserves structure
