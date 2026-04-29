# Research: Gram Codec Binding Parity (048)

## Decision 1: Python FFI — how data crosses from Rust into Python

**Decision**: `pythonize` crate — converts any `serde`-serializable Rust type directly to Python objects (`PyDict`, `PyList`, etc.) without a JSON string intermediary.

**Rationale**: The current approach (`serde_json::to_string` → Python `json.loads`) is an unnecessary double-serialization: Rust serializes to a JSON string, crosses the FFI boundary, then Python parses that string back into objects. `pythonize` eliminates this entirely — it walks the Rust data structure and builds Python objects directly. The reverse (`depythonize`) does the same from Python objects back to Rust types.

**Implementation**:
```toml
# crates/gram-codec/Cargo.toml — under [dependencies], python feature
pythonize = { version = "0.23", optional = true }
```

```rust
// Before: serde_json::to_string(&asts) → String (crosses FFI as text)
// After:  pythonize::pythonize(py, &asts)? → PyObject (crosses FFI as native objects)

// Before: let asts: Vec<AstPattern> = serde_json::from_str(json_str)?
// After:  let asts: Vec<AstPattern> = pythonize::depythonize(py_obj)?
```

**Alternatives considered**:
- JSON string round-trip (current): Rejected — double serialization, unnecessary allocation and parse overhead.
- Direct `#[pyclass]` structs: Rejected — requires matching full Rust type hierarchy in PyO3, complex lifetime management, tight coupling of Python API to Rust internals.
- `pyo3-serde`: Less maintained than `pythonize`; `pythonize` is the community standard for this use case.

---

## Decision 2: WASM FFI — how data crosses from Rust into TypeScript

**Decision**: `serde-wasm-bindgen` with `Serializer::json_compatible()` — converts `serde`-serializable Rust types directly to `JsValue` (plain JS objects) without a JSON string intermediary.

**Rationale**: The current approach (`serde_json::to_string` → TypeScript `JSON.parse`) is a double-serialization. `serde-wasm-bindgen` was considered earlier but avoided due to a `HashMap` serialization issue: by default it maps Rust `HashMap` to a JS `Map` object (not a plain `{}` object). The `json_compatible()` serializer mode, added in `serde-wasm-bindgen` 0.6, specifically fixes this — it maps Rust `HashMap<String, V>` to plain JS objects, matching what `JSON.parse` would produce. The reverse (`from_value()`) deserializes a `JsValue` back to any `serde`-deserializable Rust type.

**Implementation**:
```toml
# adapters/wasm/pattern-wasm/Cargo.toml
serde-wasm-bindgen = "0.6"
serde = { version = "1.0", features = ["derive"] }
```

```rust
// Before: serde_json::to_string(&asts).map(JsValue::from_str)  (JSON text crossing boundary)
// After:
use serde::Serialize;
let serializer = serde_wasm_bindgen::Serializer::json_compatible();
asts.serialize(&serializer)  // JsValue — plain JS object, no JSON.parse needed

// Before: serde_json::from_str::<Vec<AstPattern>>(json_str)
// After:  serde_wasm_bindgen::from_value::<Vec<AstPattern>>(js_val)
```

**TypeScript impact**: The TypeScript `gram.ts` calls `JSON.parse(wasm.parseToJson(input))`. After this change, `JSON.parse` is removed — the WASM method returns a `JsValue` that is already a plain JS object, consumed directly.

**Alternatives considered**:
- JSON string round-trip (current): Rejected — double serialization; the codebase comment "workaround for serde-wasm-bindgen HashMap issue" acknowledges this is a stopgap.
- `tsify` crate: Also builds on `serde-wasm-bindgen`; adds TypeScript type generation but the core approach is identical. Not needed since TypeScript types are maintained by hand.
- `wasm-bindgen` with manual `js_sys::Object` construction: Verbose, error-prone, no type safety.

---

## Decision 3: JSON interchange format for header + patterns (FFI boundary)

**Decision**: A native Rust struct `ParseWithHeaderResult { header: Option<PropertyRecord>, patterns: Vec<AstPattern> }` — serialized directly to JS/Python via `serde-wasm-bindgen` / `pythonize`. No JSON string.

**Rationale**: Consistent with decisions 1 and 2. A dedicated struct gives `serde` a clear schema to serialize. On the TypeScript side this becomes `{ header: Record<string, unknown> | undefined, patterns: AstPattern[] }` as a plain JS object. On the Python side this becomes a Python dict `{ "header": dict | None, "patterns": list[dict] }`, from which the wrapper extracts the tuple.

**Alternatives considered**:
- Return a JSON string `{"header": {...}, "patterns": [...]}` — rejected, same double-serialization problem.
- Two separate WASM/PyO3 calls (one for header, one for patterns) — rejected, two-call contract is harder to error-handle atomically.

---

## Decision 4: Python function naming

**Decision**: `parse`, `parse_with_header`, `stringify`, `stringify_with_header` — matching TypeScript `Gram` names for cross-language consistency.

**Rationale**: Callers switching between Python and TypeScript see the same four names. Existing `parse_gram` and `gram_stringify` are retained as aliases.

---

## Decision 5: Backwards compatibility for existing Python and TypeScript internal names

**Decision**:
- Python: Keep `parse_gram`, `gram_stringify`, `gram_validate`, `round_trip` as aliases in `__all__`.
- TypeScript internal WASM method names change (`parseToJson` → `parse`, `stringifyFromJson` → `stringify`) but these are private to `gram.ts`; the public `Gram.parse` / `Gram.stringify` signatures are unchanged.

**Rationale**: The WASM method names are an implementation detail not visible to `@relateby/pattern` consumers. The Python aliases preserve any existing callers of the pure-Python wrapper.

---

## Existing file inventory

| File | Role | Change |
|------|------|--------|
| `adapters/wasm/pattern-wasm/Cargo.toml` | WASM crate deps | Add `serde-wasm-bindgen = "0.6"`, `serde` with derive |
| `crates/gram-codec/Cargo.toml` | gram-codec deps | Add `pythonize` under python feature |
| `adapters/wasm/pattern-wasm/src/gram.rs` | WASM `Gram` struct | Replace JSON string returns with `JsValue` via `json_compatible()`; add `parseWithHeader`, `stringifyWithHeader` |
| `crates/gram-codec/src/python.rs` | PyO3 module | Replace JSON string returns with `pythonize`/`depythonize`; add header functions; update module registration |
| `python/packages/relateby/relateby/gram/__init__.py` | Public Python API | Remove `json.loads`/`json.dumps`; add `parse`, `stringify`, `parse_with_header`, `stringify_with_header`; alias old names |
| `python/packages/relateby/relateby/gram/__init__.pyi` | Type stubs | Add signatures for all four canonical functions |
| `typescript/packages/pattern/src/gram.ts` | TypeScript `Gram` namespace | Remove `JSON.parse`; update WASM call sites; add `parseWithHeader`, `stringifyWithHeader` |
| `typescript/packages/pattern/dist/gram.d.ts` | Published types | Regenerated from source |

---

## Reference implementation alignment

| This feature | gram-hs equivalent |
|---|---|
| `parse` | `fromGram` |
| `parse_with_header` / `parseWithHeader` | `fromGramWithHeader` |
| `stringify` | `toGram` |
| `stringify_with_header` / `stringifyWithHeader` | `toGramWithHeader` |

The Rust `parse_gram`, `parse_gram_with_header`, `to_gram`, `to_gram_with_header` already implement gram-hs semantics faithfully. No new parse/serialize logic is required.

---

## No unresolved NEEDS CLARIFICATION items

All architectural decisions are resolved. Proceed to implementation.
