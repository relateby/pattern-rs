# Quickstart: WASM and TypeScript Pattern-Core

**Feature**: 027-wasm-pattern-typescript-parity  
**Date**: 2026-01-31

## Build (from repo root)

```bash
# Install wasm-pack if needed
# cargo install wasm-pack

# Build pattern-core for WASM (with wasm feature)
cd crates/pattern-core
wasm-pack build --target web --features wasm
# Or for Node: wasm-pack build --target nodejs --features wasm
```

Output appears in `pkg/` (or as configured): `.wasm`, `.js` glue, and optionally TypeScript definitions. If the project ships hand-written `.d.ts`, copy or link them into `pkg/` so consumers get types without extra steps.

## Load the module (browser)

```html
<script type="module">
  import init, { Pattern, Subject, Value } from './pkg/pattern_core.js';
  await init();

  const atomic = Pattern.point("hello");
  console.log(atomic.value);      // "hello"
  console.log(atomic.elements);   // []
  console.log(atomic.isAtomic()); // true
</script>
```

## Load the module (Node)

```js
const { default: init, Pattern, Subject, Value } = require('./pkg/pattern_core.js');
await init();

const child1 = Pattern.point("child1");
const child2 = Pattern.point("child2");
const parent = Pattern.pattern("parent", [child1, child2]);
console.log(parent.length()); // 2
console.log(parent.depth());  // 1
```

## TypeScript usage

Ensure the package (or `pkg/`) includes `.d.ts` and that your `tsconfig.json` resolves the module. Then:

```ts
import init, { Pattern, Subject, Value } from 'pattern_core';

await init();

// Generic Pattern<string>
const p: Pattern<string> = Pattern.point("hello");
const mapped: Pattern<number> = p.map(s => s.length);

// Pattern<Subject>
const subject = Subject.new("n", ["Person"], { name: Value.string("Alice") });
const patternSubject: Pattern<Subject> = Pattern.point(subject);
const depth = patternSubject.depth();
```

Type checker and IDE will infer `Pattern<V>` and return types (e.g. `map` → `Pattern<W>`) when definitions follow the contract in contracts/typescript-types.md.

## Minimal workflow (parity with Python quickstart)

1. **Atomic pattern**: `Pattern.point(value)` or `Pattern.of(value)`.
2. **Nested pattern**: `Pattern.pattern(value, [child1, child2, ...])`.
3. **Pattern with Subject**: Build `Subject` with identity, labels, properties; then `Pattern.point(subject)`.
4. **Inspection**: `pattern.length()`, `pattern.size()`, `pattern.depth()`, `pattern.isAtomic()`, `pattern.values()`.
5. **Query**: `pattern.filter(predicate)`, `pattern.findFirst(predicate)`, `pattern.matches(other)`, `pattern.contains(other)`.
6. **Transform**: `pattern.map(fn)`, `pattern.fold(init, fn)`, `pattern.para(fn)` (paramorphism: bottom-up aggregation with (value, elementResults[])).
7. **Combine**: `pattern.combine(other)` (for Subject or other combinable value types).
8. **Comonad**: `pattern.extract()`, `pattern.extend(fn)`, `pattern.depthAt()`, `pattern.sizeAt()`, `pattern.indicesAt()`.
9. **Validate**: `pattern.validate(rules)` returns an Either-like value (`Right(undefined)` on success, `Left(ValidationError)` on failure). Does NOT throw. Use with effect-ts below.
10. **Analyze**: `pattern.analyzeStructure()`.

Same operations and semantics as the Python binding; only syntax and naming (camelCase) differ where conventional for JS/TS.

## Using with effect-ts (Result / Either)

Fallible operations (e.g. `validate`) match Rust’s `Result` at the boundary: they return a value that is trivially convertible to effect-ts Either. The return shape is compatible with `Either.right(value)` / `Either.left(error)` (e.g. `{ _tag: 'Right', right: T }` | `{ _tag: 'Left', left: E }`). No helper is required.

**Example (validate and use with Effect):**

```ts
import { Either } from 'effect';
import init, { Pattern, ValidationRules } from 'pattern_core';

await init();

const pattern = Pattern.point("hello");
const rules = ValidationRules.new({ maxDepth: 10 });
const result = pattern.validate(rules);

// result is Either<void, ValidationError>; use with Effect
Either.match(result, {
  onLeft: (err) => console.error('Validation failed:', err.message),
  onRight: () => console.log('Valid'),
});
```

Documentation (README or quickstart) MUST describe the return shape and that it is compatible with Effect’s Either so that consumers can use it in Effect pipelines without a wrapper.

## Examples directory

After implementation, `examples/pattern-core-wasm/` will contain a minimal Node and/or browser example and a README with build/run instructions (see plan.md Project Structure).
