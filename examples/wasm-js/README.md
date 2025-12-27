# WASM/JavaScript Example

This example demonstrates how to compile and use the gram library in a WebAssembly/JavaScript environment.

## Prerequisites

- Rust 1.70.0 or later
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- `wasm-bindgen-cli` (optional, for generating bindings): `cargo install wasm-bindgen-cli`

## Building

```bash
# Build for WASM
cargo build --target wasm32-unknown-unknown --release
```

## Usage

The compiled WASM module can be imported and used in JavaScript/TypeScript:

```javascript
// Example usage (after setting up WASM bindings)
import init, { greet, add } from './gram_wasm_example.js';

await init();
console.log(greet("World")); // "Hello, World! (from gram-rs)"
console.log(add(2, 3)); // 5
```

## Notes

- This is a minimal example demonstrating the structure
- Full functionality will be available as the gram library is ported from gram-hs
- See the main project README for more information

