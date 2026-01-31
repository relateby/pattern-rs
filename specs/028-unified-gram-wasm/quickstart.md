# Quickstart: Unified Gram WASM Package

**Feature**: 028-unified-gram-wasm  
**Date**: 2026-01-31

## Goal

Get from zero to a working round-trip (create Pattern&lt;Subject&gt;, stringify to gram notation, parse back, assert equivalence) using the unified pattern-wasm package in under 10 minutes (per spec SC-001).

## Prerequisites

- Node.js and npm (or equivalent); Rust toolchain and wasm-pack if building from source.
- Package installed (e.g. `npm install @gram-data/gram` or local build from `crates/pattern-wasm`).

## Steps (consumer)

1. **Install** the unified package (once published or linked from build).
2. **Import** from one entry point:
   ```js
   import init, { Pattern, Subject, Value, Gram } from 'gram';
   ```
3. **Initialize** WASM (once per app):
   ```js
   await init();
   ```
4. **Build** a small Pattern&lt;Subject&gt;:
   ```js
   const alice = Subject.new("alice", ["Person"], { name: Value.string("Alice") });
   const pattern = Pattern.point(alice);
   ```
5. **Serialize** to gram notation:
   ```js
   const text = Gram.stringify(pattern);
   ```
6. **Parse** back:
   ```js
   const parsed = Gram.parse(text);
   ```
7. **Assert** round-trip: compare structure and Subject data of `pattern` and `parsed[0]` (e.g. identity, labels, properties).

## Optional: Pattern&lt;V&gt; → Pattern&lt;Subject&gt;

If you have a pattern of primitives (e.g. numbers), convert then stringify. **Gram.from** delegates to **Subject.fromValue** (Gram.from(pattern, options?) = pattern.map(v => Subject.fromValue(v, options))). You can use either:

```js
const numbers = Pattern.pattern(1, [Pattern.point(2), Pattern.point(3)]);
const asSubjects = Gram.from(numbers);  // or: numbers.map(Subject.fromValue)
const text = Gram.stringify(asSubjects);
```

## Build from source (developer)

1. From repo root: `cargo build -p pattern-wasm --target wasm32-unknown-unknown` (or use wasm-pack for the crate).
2. Use wasm-pack build in `crates/pattern-wasm` to produce JS and TypeScript definitions.
3. Consume the generated package or link it locally for testing.

## Success

- Single import provides Pattern, Subject, Value, and Gram.
- Gram.stringify(pattern) and Gram.parse(text) work without extra setup.
- Round-trip preserves structure and data.
