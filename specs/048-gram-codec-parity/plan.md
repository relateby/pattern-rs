# Implementation Plan: Gram Codec Binding Parity

**Branch**: `048-gram-codec-parity` | **Date**: 2026-04-28 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/048-gram-codec-parity/spec.md`

## Summary

Expose the full four-method gram codec surface (`parse`, `parse_with_header`, `stringify`, `stringify_with_header`) in both Python (`relateby.gram`) and TypeScript (`Gram` namespace in `@relateby/pattern`), using language-native `Pattern<Subject>` objects as the pattern currency. The Rust implementations already exist. This work first fixes the FFI boundary — replacing JSON string round-tripping with direct object passing via `pythonize` (PyO3) and `serde-wasm-bindgen json_compatible()` (WASM) — then adds the header functions on top of the corrected layer.

## Technical Context

**Language/Version**: Rust 1.70.0 (MSRV), Edition 2021 · Python 3.8+ · TypeScript 5.x
**Primary Dependencies**: PyO3 0.23 (existing) · `pythonize 0.23` (new) · `serde-wasm-bindgen 0.6` (new) · wasm-bindgen 0.2 (existing) · effect ≥3.0 (existing)
**Storage**: N/A — in-memory only
**Testing**: `cargo test` (Rust) · `pytest` (Python) · `vitest` (TypeScript)
**Target Platform**: Native Rust · WASM (browser + Node.js) · Python 3.8–3.13
**Project Type**: Multi-target library with external language bindings
**Performance Goals**: Direct object passing at FFI boundary — no JSON string serialization overhead
**Constraints**: WASM-safe (pure computation) · Python 3.8+ compatible · `pythonize` version must match PyO3 version (both 0.23)

## Constitution Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | ✅ Pass | gram-hs has exact equivalents (`fromGramWithHeader`, `toGramWithHeader`). Rust already implements these faithfully. |
| II. Correctness & Compatibility | ✅ Pass | No new parse/serialize logic. Fixing the FFI layer does not change observable behavior, only efficiency. |
| III. Rust Native Idioms | ✅ Pass | `pythonize` and `serde-wasm-bindgen` are the idiomatic Rust solutions for their respective boundaries. |
| IV. Multi-Target Library Design | ✅ Pass | `ParseWithHeaderResult` is a plain serde struct — WASM-safe, no blocking I/O. |
| V. External Language Bindings & Examples | ✅ Pass | This feature IS the bindings work. Examples in quickstart.md. |

No violations. No complexity tracking required.

## Project Structure

### Documentation (this feature)

```text
specs/048-gram-codec-parity/
├── plan.md              ← this file
├── research.md          ← complete
├── data-model.md        ← complete
├── quickstart.md        ← complete
├── contracts/
│   ├── python-api.md    ← complete
│   └── typescript-api.md ← complete
└── tasks.md             ← Phase 2 output (/speckit.tasks)
```

### Source Code (files to change)

```text
adapters/wasm/pattern-wasm/
└── Cargo.toml           # + serde-wasm-bindgen = "0.6", serde with derive

adapters/wasm/pattern-wasm/src/
└── gram.rs              # Replace JSON strings with JsValue via json_compatible();
                         # add parseWithHeader, stringifyWithHeader

crates/gram-codec/
└── Cargo.toml           # + pythonize under python feature

crates/gram-codec/src/
└── python.rs            # Replace JSON strings with pythonize/depythonize;
                         # add parse_with_header_py, stringify_with_header_py;
                         # update module registration

python/packages/relateby/relateby/gram/
├── __init__.py          # Remove json.loads/dumps; add parse, stringify,
                         # parse_with_header, stringify_with_header; alias old names
└── __init__.pyi         # Add type signatures for all four canonical functions

typescript/packages/pattern/src/
└── gram.ts              # Remove JSON.parse; update WASM call sites;
                         # add Gram.parseWithHeader, Gram.stringifyWithHeader

typescript/packages/pattern/dist/
└── gram.d.ts            # Regenerated from source

tests/python/gram/
└── test_gram_parity.py  # New: tests for all four canonical functions + round-trips

typescript/packages/pattern/tests/
└── gram-parity.test.ts  # New: tests for parseWithHeader, stringifyWithHeader
```

## Implementation sequence

Changes form a strict bottom-up dependency chain. Steps at the same level are independent and can be parallelized.

```
Level 1 — New dependencies (prerequisite for everything below)
  ├── adapters/wasm/pattern-wasm/Cargo.toml  (add serde-wasm-bindgen 0.6, serde)
  └── crates/gram-codec/Cargo.toml           (add pythonize under python feature)

Level 2 — Fix existing FFI layer (prerequisite for new functions)
  ├── gram.rs (WASM)    Replace JSON strings → JsValue via json_compatible()
  │                     for existing parse_to_json and stringify_from_json
  └── python.rs (PyO3)  Replace JSON strings → pythonize/depythonize
                        for existing gram_parse_to_json_py and gram_stringify_from_json_py

Level 3 — Add new FFI functions (prerequisite for language wrappers)
  ├── gram.rs (WASM)    Add parseWithHeader → JsValue, stringifyWithHeader ← JsValue
  └── python.rs (PyO3)  Add parse_with_header_py, stringify_with_header_py using pythonize

Level 4 — Update language wrappers (prerequisite for type stubs and tests)
  ├── gram/__init__.py  Remove json.loads/dumps; add parse, stringify,
  │                     parse_with_header, stringify_with_header; alias old names
  └── gram.ts           Remove JSON.parse; update call sites; add parseWithHeader,
                        stringifyWithHeader to Gram namespace

Level 5 — Type stubs and published types
  ├── gram/__init__.pyi  Python type stubs for four canonical functions
  └── dist/gram.d.ts     Regenerated TypeScript declarations

Level 6 — Tests
  ├── test_gram_parity.py   Python: parse, stringify, parse_with_header,
  │                         stringify_with_header, round-trips, error cases
  └── gram-parity.test.ts   TypeScript: parseWithHeader, stringifyWithHeader,
                            round-trips, error cases

Level 7 — Code quality
  cargo fmt --all
  cargo clippy --workspace -- -D warnings
  ./scripts/ci-local.sh
```

## Key implementation notes

### `ParseWithHeaderResult` struct (new, in gram.rs / python.rs)

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct ParseWithHeaderResult {
    header: Option<std::collections::HashMap<String, pattern_core::Value>>,
    patterns: Vec<gram_codec::ast::AstPattern>,
}
```

Used as the Rust-side return type for `parse_with_header` across both WASM and PyO3. `serde-wasm-bindgen json_compatible()` serializes `header: None` as `null` in JS; `pythonize` serializes it as `None` in Python.

### WASM — before and after

```rust
// Before
pub fn parse_to_json(gram: &str) -> Result<String, String> {
    gram_codec::gram_parse_to_json(gram)  // returns JSON string
}

// After
pub fn parse(gram: &str) -> Result<JsValue, JsValue> {
    let patterns = gram_codec::parse_gram(gram).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let asts: Vec<AstPattern> = patterns.iter().map(AstPattern::from_pattern).collect();
    let serializer = serde_wasm_bindgen::Serializer::json_compatible();
    asts.serialize(&serializer).map_err(|e| JsValue::from_str(&e.to_string()))
}
```

### PyO3 — before and after

```rust
// Before
fn gram_parse_to_json_py(input: &str) -> PyResult<String> {
    gram_codec::json::gram_parse_to_json(input).map_err(...)  // returns JSON string
}

// After
fn parse_py(py: Python, input: &str) -> PyResult<PyObject> {
    let patterns = gram_codec::parse_gram(input).map_err(...)?;
    let asts: Vec<AstPattern> = patterns.iter().map(AstPattern::from_pattern).collect();
    pythonize::pythonize(py, &asts).map_err(...)  // returns PyObject directly
}
```

### Python wrapper — before and after

```python
# Before
def parse_gram(input: str) -> list[Pattern[Subject]]:
    json_str = _gram.gram_parse_to_json(input)   # JSON string from Rust
    raw = _json.loads(json_str)                   # parse JSON string
    return [pattern_from_dict(d) for d in raw]

# After
def parse(input: str) -> list[Pattern[Subject]]:
    raw = _gram.parse(input)                      # Python list of dicts directly
    return [pattern_from_dict(d) for d in raw]
```

### TypeScript — before and after

```typescript
// Before
const raw = JSON.parse(await loadWasm().then(w => w.parseToJson(input)))

// After
const raw = await loadWasm().then(w => w.parse(input))  // already a JS object
```
