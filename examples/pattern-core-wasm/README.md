# pattern-core WASM Example

This example demonstrates how to use the `pattern-core` library from JavaScript/TypeScript in both browser and Node.js environments.

## Prerequisites

- Rust 1.70.0 or later
- Node.js 18+ (for Node.js example)
- `wasm-pack`: `cargo install wasm-pack`

## Building the WASM Package

From the repository root:

```bash
cd crates/pattern-core
wasm-pack build --target web --features wasm
```

This generates the WASM package in `crates/pattern-core/pkg/` with:
- `pattern_core_bg.wasm` - the compiled WebAssembly module
- `pattern_core.js` - JavaScript glue code
- `pattern_core.d.ts` - TypeScript type definitions

For Node.js, build with:

```bash
wasm-pack build --target nodejs --features wasm
```

## Examples

### Browser Example

The `browser.html` file demonstrates basic usage in a web browser:

1. Build the WASM package (see above)
2. Serve the example directory with a local web server:
   ```bash
   # From examples/pattern-core-wasm/
   python3 -m http.server 8000
   # or use any other static file server
   ```
3. Open http://localhost:8000/browser.html in your browser
4. Open the browser console to see the output

### Node.js Example

The `node.mjs` file demonstrates usage in Node.js:

1. Build the WASM package for Node.js target (see above)
2. Copy or link the package:
   ```bash
   # From examples/pattern-core-wasm/
   ln -s ../../crates/pattern-core/pkg pkg
   ```
3. Run the example:
   ```bash
   node node.mjs
   ```

### TypeScript Example

The `typescript-demo.ts` file shows TypeScript usage with full type safety:

1. Build the WASM package (see above)
2. Install TypeScript if needed: `npm install -g typescript`
3. Compile and run:
   ```bash
   # Compile TypeScript (type-check only, don't execute)
   tsc --noEmit typescript-demo.ts
   
   # Or use ts-node to run directly:
   # npm install -g ts-node
   # ts-node typescript-demo.ts
   ```

## What's Demonstrated

All examples show the same core functionality:

1. **Construction**: Creating atomic patterns and nested patterns with `Pattern.point()` and `Pattern.pattern()`
2. **Inspection**: Using `length()`, `size()`, `depth()`, `isAtomic()`, `values()`
3. **Query**: Filtering with `filter()`, finding with `findFirst()`, checking containment
4. **Transformation**: Mapping values with `map()`, folding with `fold()`, paramorphism with `para()`
5. **Combination**: Combining patterns with `combine()`
6. **Comonad Operations**: Using `extract()`, `extend()`, `depthAt()`, `sizeAt()`
7. **Validation**: Validating patterns with `validate()` (returns Either-like result)
8. **Analysis**: Analyzing structure with `analyzeStructure()`
9. **Subject Patterns**: Creating patterns with `Subject` values (identity, labels, properties)

## effect-ts Integration

The validation example shows how to use the Either-like return values with effect-ts:

```typescript
import { Either } from 'effect';

const result = pattern.validate(rules);
// result is Either<ValidationError, void>

Either.match(result, {
  onLeft: (err) => console.error('Validation failed:', err.message),
  onRight: () => console.log('Valid'),
});
```

The return shape `{ _tag: 'Right' | 'Left', right: T, left: E }` is directly compatible with effect-ts `Either` type.

## API Reference

See the main pattern-core documentation and TypeScript definitions in `crates/pattern-core/typescript/pattern_core.d.ts` for the complete API.
