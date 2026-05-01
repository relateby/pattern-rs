# How do the three language bindings relate to each other?

The library has one canonical implementation — the Rust crates `relateby-pattern` and `relateby-gram`. The Python and TypeScript bindings expose the same API surface via different compilation targets.

**Rust** is the native implementation. All core types (`Pattern<V>`, `Subject`, `StandardGraph`) and all operations (`map`, `fold`, `combine`, `parse_gram`, `to_gram`) are defined here. Use Rust when you need maximum performance or when you are building a Rust application.

**Python** bindings are compiled via PyO3. The package `relateby-pattern` (installable as `pip install relateby-pattern`) provides `relateby.pattern` and `relateby.gram`. The API follows Python naming conventions. Type stubs (`.pyi` files) describe the full surface. Install the package for type checking and IDE autocompletion.

**TypeScript** bindings are compiled to WebAssembly via wasm-bindgen. The packages `@relateby/pattern`, `@relateby/gram`, and `@relateby/graph` expose equivalent operations using TypeScript idioms. Because WebAssembly loads asynchronously, the API uses `Effect` (from the `effect` library) to model asynchronous operations cleanly.

All three expose equivalent operations, but naming follows each language's conventions:
- Rust: `Pattern::point`, `parse_gram`, `to_gram`
- Python: `Pattern.point`, `gram.parse`, `gram.stringify`
- TypeScript: `Pattern.point`, `Gram.parse`, `Gram.stringify`

See the [API Reference](/reference/) for the full surface in each language.
