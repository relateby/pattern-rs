# WASM Usage Guide

This guide covers using pattern-rs from JavaScript/TypeScript via WebAssembly.

## Packages

Two npm packages are involved:

- `@relateby/gram` holds the WASM-backed Gram codec, exposed through `Gram` (plus `init()`). The WASM build output (`wasm/`, `wasm-node/`) lives here.
- `@relateby/pattern` holds the pure TypeScript `Pattern`, `Subject`, `Value`, and `StandardGraph`, graph interfaces and transforms. It has no WASM of its own.

## Graph API

For the complete TypeScript graph API reference, see **[docs/typescript-graph.md](./typescript-graph.md)**.

This covers:
- `Gram` and the Effect-based codec workflow
- `Pattern`, `Subject`, `Value`, and `StandardGraph`
- pure TypeScript transforms (`mapGraph`, `filterGraph`, `foldGraph`, `paraGraph`, etc.)

## Building the WASM Module

```bash
# Install wasm-pack
cargo install wasm-pack

# Build the WASM module (from repo root)
cd typescript/packages/gram
npm run build:wasm  # runs wasm-pack --target bundler
npm run build:ts    # compiles TypeScript
```

The `build:wasm` script runs:
```bash
wasm-pack build ../../../adapters/wasm/pattern-wasm --target bundler --out-dir ../../../typescript/packages/gram/wasm
```

## Quick Start

```typescript
import { Effect, Option } from "effect"
import { init, Gram } from "@relateby/gram"
import { StandardGraph } from "@relateby/pattern"

await init()

const patterns = await Effect.runPromise(
  Gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)")
)

const graph = StandardGraph.fromPatterns(patterns)

console.log(graph.nodeCount)
console.log(graph.relationshipCount)
console.log(Option.getOrUndefined(graph.node("alice"))?.value.identity)

await Effect.runPromise(Gram.validate("(alice:Person)"))
```

The WASM boundary is intentionally narrow in this branch: the `adapters/wasm/pattern-wasm` crate exists to support the Rust gram codec, while the higher-level Pattern and graph APIs are implemented natively in TypeScript.

## CI/CD

Before pushing, run all CI checks:

```bash
./scripts/ci-local.sh
```

For release-grade package validation, run:

```bash
./scripts/ci-local.sh --release
```
