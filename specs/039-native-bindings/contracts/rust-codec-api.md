# Contract: Rust Gram Codec API

**Feature**: 039-native-bindings
**Date**: 2026-03-17

This is the sole surface that crosses the Rust/native boundary after migration. All other types (`Pattern`, `Subject`, `Value`, `StandardGraph`) are implemented natively in TypeScript and Python and never cross this boundary.

---

## New Rust Functions (gram-codec additions)

### `gram_parse_to_json(input: &str) -> Result<String, String>`

Parses a gram notation string and returns the result as a JSON string.

**Input**: UTF-8 gram notation string.

**Output (success)**: A JSON array of `RawPattern` objects (see JSON Interchange Format in `data-model.md`). Empty input returns `"[]"`.

**Output (failure)**: A JSON error object: `{"error": "...", "location": {"line": N, "column": N}}`.

**WASM export name**: `gram_parse_to_json`
**PyO3 export name**: `gram_parse_to_json`

---

### `gram_stringify_from_json(input: &str) -> Result<String, String>`

Serializes a JSON array of `RawPattern` objects back to gram notation string.

**Input**: A JSON string matching the `RawPattern[]` schema.

**Output (success)**: A gram notation string.

**Output (failure)**: An error string describing the serialization failure.

**WASM export name**: `gram_stringify_from_json`
**PyO3 export name**: `gram_stringify_from_json`

---

### `gram_validate(input: &str) -> String`

Validates gram notation. Returns `"[]"` (empty JSON array) on success; returns a JSON array of error objects on failure.

**WASM export name**: `gram_validate` (already exists; verify signature matches)
**PyO3 export name**: `gram_validate`

---

## Removed from WASM Surface (after Phase 4 cutover)

These exports are removed when `convert.rs` and `standard_graph.rs` are deleted from `crates/pattern-wasm`:

- `WasmPattern` (class)
- `WasmSubject` (class)
- `WasmValue` (class)
- `WasmStandardGraph` (class)
- `Gram.parse` (returns `WasmPattern[]`) → replaced by `gram_parse_to_json`
- `Gram.parseOne` (returns `WasmPattern`) → removed
- `Gram.stringify` (takes `WasmPattern`) → replaced by `gram_stringify_from_json`

---

## Removed from PyO3 Surface (after Phase 5 cutover)

These exports are removed when `crates/pattern-core/src/python.rs` is slimmed from ~1,657 lines to ~50 lines:

- `PyPattern` (class)
- `PySubject` (class)
- `PyValue` (class)
- `PyStandardGraph` (class)
- `PyValidationRules` (class)
- `PyStructureAnalysis` (class)
- `PySubjectBuilder` (class)
- `PyValidationError` (class)

**Retained** in the PyO3 surface:
- `parse_patterns_as_dicts` (or replaced by `gram_parse_to_json`)
- `gram_parse_to_json`
- `gram_stringify_from_json`
- `gram_validate`

---

## WASM Boundary Characteristics

- **Encoding**: JSON string (one `JSON.parse` per call on the TypeScript side)
- **Direction**: Rust → TypeScript (parse), TypeScript → Rust (stringify)
- **Frequency**: Once per `Gram.parse()` or `Gram.stringify()` call; never per Pattern operation
- **Error model**: Errors returned as values (JSON error objects), not thrown exceptions

---

## Backward Compatibility

The public TypeScript API (`Gram.parse`, `Gram.stringify`) and Python API (`parse_gram`, `gram_stringify`) maintain the same import paths and return types from the consumer's perspective. The internal WASM/PyO3 mechanism changes, but the calling code does not need to change.
