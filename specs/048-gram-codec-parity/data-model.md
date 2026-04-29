# Data Model: Gram Codec Binding Parity (048)

## Entities

### HeaderRecord

A flat map of string keys to gram-compatible values. Appears as the optional first element of a gram document. Has no identity or labels — properties only.

**In Rust**: `HashMap<String, pattern_core::Value>` (type alias `PropertyRecord` / `Record`)
**In Python**: `dict[str, Any]` — produced directly by `pythonize` from the Rust map; consumed by `depythonize` when stringifying
**In TypeScript**: `Record<string, unknown>` — produced directly by `serde-wasm-bindgen json_compatible()` from the Rust map as a plain JS object; consumed by `serde_wasm_bindgen::from_value()` when stringifying

**Constraints**:
- Keys: non-empty strings
- Values: gram-compatible types only (string, integer, decimal, boolean, array of those, symbol, tagged string, range) — nested maps rejected at serialization time by the Rust layer
- May be absent (`None` / `undefined`) when a gram document has no leading record

---

### Pattern\<Subject\>

The core data type — a decorated sequence. Already fully defined across all three languages; this feature does not change its shape.

**In Rust**: `pattern_core::Pattern<Subject>`
**In Python**: `relateby.pattern.Pattern[Subject]` — pure-Python dataclass, reconstructed by `pattern_from_dict()` from the `AstPattern` dict that `pythonize` produces
**In TypeScript**: `Pattern<Subject>` from `@relateby/pattern` — reconstructed by `patternFromRaw()` from the plain JS object that `serde-wasm-bindgen json_compatible()` produces

---

### AstPattern (FFI wire type)

The intermediate Rust struct used at the FFI boundary. Not a public type in Python or TypeScript.

```rust
struct AstPattern {
    subject: AstSubject,    // identity, labels, properties
    elements: Vec<AstPattern>,
}
```

**At the Python boundary**: `pythonize` converts `AstPattern` to a Python dict `{"subject": {...}, "elements": [...]}` — then `pattern_from_dict()` converts that to `Pattern[Subject]`. Reverse: Python dict → `depythonize` → `AstPattern`.

**At the WASM boundary**: `serde-wasm-bindgen json_compatible()` converts `AstPattern` to a plain JS object `{subject: {...}, elements: [...]}` — then `patternFromRaw()` converts that to `Pattern<Subject>`. Reverse: JS object → `serde_wasm_bindgen::from_value()` → `AstPattern`.

---

### ParseWithHeaderResult (new Rust struct for FFI)

A new dedicated struct that carries both the optional header and the pattern list across the FFI boundary atomically.

```rust
#[derive(Serialize, Deserialize)]
struct ParseWithHeaderResult {
    header: Option<HashMap<String, Value>>,
    patterns: Vec<AstPattern>,
}
```

**At the Python boundary**: `pythonize` produces `{"header": dict | None, "patterns": [dict, ...]}`. The Python wrapper unpacks this into the two-tuple `(dict | None, list[Pattern[Subject]])`.

**At the WASM boundary**: `serde-wasm-bindgen json_compatible()` produces `{header: Record<string, unknown> | undefined, patterns: AstPattern[]}` as a plain JS object. TypeScript reads `result.header` and `result.patterns` directly.

---

## No JSON string interchange

The FFI boundary does **not** use JSON strings. `serde_json` is not involved in the Python or TypeScript binding path. The only serialization at the boundary is:
- `pythonize` / `depythonize` for PyO3
- `serde-wasm-bindgen json_compatible()` / `from_value()` for WASM

---

## Validation rules

| Rule | Enforced by |
|------|------------|
| Mid-document bare records are rejected | Rust parser (existing) |
| Only one leading header record per document | Rust `parse_gram_with_header` (existing) |
| Header value types must be gram-serializable | Rust `to_gram_with_header` (existing) — returns `SerializeError` for unsupported types |
| Empty / whitespace-only input → absent header + empty pattern list | Rust (existing) |
