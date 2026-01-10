# Gram Codec WASM - Node.js Example

Node.js example demonstrating the gram-codec WASM bindings.

> **Note**: This example currently shows the basic validation API. For full data access, see the **AST Output** section below (coming in Phase 7).

## Prerequisites

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Or with cargo
cargo install wasm-pack
```

## Building the WASM Package

```bash
# From the gram-codec crate directory
cd ../../crates/gram-codec

# Build for Node.js
wasm-pack build --target nodejs --release --features wasm

# The package is now in pkg/
ls -lh pkg/
```

## Installing Dependencies

```bash
# From this example directory
cd examples/gram-codec-wasm-node

# Install the locally built WASM package
npm install

# Or install directly from the pkg directory
npm install ../../crates/gram-codec/pkg
```

## Running the Example

```bash
npm start

# Or directly
node index.js
```

## Current API (Validation Only)

The following functions are currently available from the WASM module:

### `parse_gram(input: string): ParseResult`

Parse gram notation and return metadata about the parsed patterns.

```javascript
const result = parse_gram("(alice)-[:KNOWS]->(bob)");
console.log(result.pattern_count);  // Number of patterns
console.log(result.identifiers);    // Array of root identifiers
```

**Note**: This returns only metadata, not the full pattern structure.

### `validate_gram(input: string): boolean`

Quick validation check without parsing.

```javascript
const isValid = validate_gram("(hello)");
console.log(isValid);  // true

const isInvalid = validate_gram("(unclosed");
console.log(isInvalid);  // false
```

### `round_trip(input: string): string`

Parse and serialize back to gram notation.

```javascript
const serialized = round_trip("(alice)-->(bob)");
console.log(serialized);  // "(alice)-->(bob)"
```

### `version(): string`

Get the gram-codec version.

```javascript
console.log(version());  // "0.1.0"
```

## AST Output (Coming in Phase 7)

For building JavaScript applications that need full access to pattern data, we're adding **AST output**. The AST provides the complete `Pattern<Subject>` structure in a JSON-friendly format.

### Why AST?

The current API returns only metadata (pattern count, identifiers). The AST will provide:
- ✅ **Complete pattern structure** - subjects, elements, properties
- ✅ **Language agnostic** - Pure JSON, no opaque types
- ✅ **Ready for gram-js** - Native JavaScript Pattern library (separate project)
- ✅ **Serializable** - Can store, transmit, or cache as JSON

### Future Usage (Phase 7)

```javascript
import init, { parse_to_ast } from './gram_codec.js';
await init();

// Parse to AST
const ast = parse_to_ast("(alice:Person {name: 'Alice', age: 30})");

// Access pattern data
console.log(ast.subject.identity);    // "alice"
console.log(ast.subject.labels);      // ["Person"]
console.log(ast.subject.properties);  // {name: "Alice", age: 30}
console.log(ast.elements);            // [] (no child patterns)

// AST is just JSON - serialize it
const json = JSON.stringify(ast);
```

### Architecture

```
gram-rs (this project)
  └─> parse_to_ast() → AST (JSON)
       └─> gram-js (separate project)
            └─> Pattern.fromAst(ast) → Full Pattern API
                 └─> map(), fold(), filter(), etc.
```

**gram-rs** responsibilities:
- ✅ Parse gram notation to AST
- ✅ Validate syntax
- ✅ Serialize patterns back to gram

**gram-js** responsibilities (future):
- ✅ Native JavaScript Pattern<V> implementation
- ✅ Full FP API (map, fold, traverse, comonad operations)
- ✅ Pattern queries and transformations
- ✅ TypeScript types and IntelliSense

This separation keeps the WASM binary small (~88KB) while enabling full Pattern operations in pure JavaScript (zero FFI overhead).

## Example Output

```
=== Gram Codec WASM - Node.js Example ===

Version: 0.1.0

=== Basic Parsing ===
Parsed '(hello)': 1 pattern(s)

=== Validation ===
'(hello)' is valid: true
'(unclosed' is valid: false

=== Round-Trip ===
Input:  (alice)-->(bob)
Output: (alice)-->(bob)
Match: true

=== Complex Patterns ===
Relationship: (alice)-[:KNOWS]->(bob)
  Valid: true
  Patterns: 1

Subject pattern: [team | (alice), (bob)]
  Valid: true
  Patterns: 1

=== All Examples Complete ===
```

## Binary Size

The WASM binary is impressively small:
- **Uncompressed**: 199 KB
- **Gzipped**: 88.5 KB

This makes it suitable for browser use and fast to load in Node.js.

## Platform Support

The WASM module works in:
- ✅ Node.js (CommonJS and ES modules)
- ✅ Modern browsers
- ✅ Deno (with appropriate imports)

## Troubleshooting

### Module not found

Make sure you've installed the package:
```bash
npm install ../../crates/gram-codec/pkg
```

### WASM compilation errors

Rebuild the WASM package:
```bash
cd ../../crates/gram-codec
wasm-pack build --target nodejs --release --features wasm
```

### Import errors

The package.json has been configured to use the local WASM package. If you see import errors, check that:
1. The WASM package was built successfully
2. The `gram-codec` dependency points to the correct path

## Next Steps

- Explore the Python bindings in `../gram-codec-python/`
- Check the browser example in `../gram-codec-wasm-web/`
- Read the AST design document in `../../specs/021-pure-rust-parser/AST-DESIGN.md`
- Star awaiting **gram-js** for native JavaScript Pattern API

## License

Apache-2.0
