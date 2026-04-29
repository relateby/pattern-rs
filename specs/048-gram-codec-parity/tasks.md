# Tasks: Gram Codec Binding Parity

**Input**: Design documents from `/specs/048-gram-codec-parity/`
**Prerequisites**: plan.md ✓ spec.md ✓ research.md ✓ data-model.md ✓ contracts/ ✓

**Organization**: Bottom-up dependency chain. Phase 2 fixes the FFI layer (prerequisite for all user stories). Phases 3–5 implement user stories in dependency order. Phase 3 (US3) produces working `parse`/`stringify` with native objects; Phases 4–5 add the header variants on top.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel with other [P] tasks at the same level (touches different files)
- **[Story]**: Which user story this task belongs to

---

## Phase 1: Setup — Add Dependencies

**Purpose**: Add `serde-wasm-bindgen` and `pythonize` to the relevant Cargo manifests. Nothing else is possible until the build succeeds.

- [X] T001 Add `serde-wasm-bindgen = "0.6"` and `serde = { version = "1.0", features = ["derive"] }` to `[dependencies]` in `adapters/wasm/pattern-wasm/Cargo.toml`
- [X] T002 [P] Add `pythonize = { version = "0.23", optional = true }` to `[dependencies]` in `crates/gram-codec/Cargo.toml`; add `pythonize` to the `python` feature list alongside `pyo3`
- [X] T003 Verify `cargo build --workspace` succeeds with the new deps and no feature flags (catches version conflicts early)

**Checkpoint**: Workspace compiles. New crates are resolved.

---

## Phase 2: Foundational — Fix the FFI Layer

**Purpose**: Replace JSON string round-tripping at the WASM and PyO3 boundaries with direct native-object passing. This is a prerequisite for every user story — no new functions are added here, only existing ones are corrected.

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

### Define shared wire type

- [X] T004 Add `ParseWithHeaderResult` struct to `crates/gram-codec/src/ast.rs` with `#[derive(serde::Serialize, serde::Deserialize)]`, fields `header: Option<std::collections::HashMap<String, pattern_core::Value>>` and `patterns: Vec<AstPattern>`; make it `pub(crate)`

### Fix WASM boundary (`adapters/wasm/pattern-wasm/src/gram.rs`)

- [X] T005 Replace the `parse_to_json` method on `Gram` (returns `Result<String, String>`) with `parse` (returns `Result<JsValue, JsValue>`): call `gram_codec::parse_gram`, map to `Vec<AstPattern>`, serialize with `serde_wasm_bindgen::Serializer::json_compatible()`
- [X] T006 Replace the `stringify_from_json` method on `Gram` (accepts `&str` JSON) with `stringify` (accepts `JsValue`): deserialize patterns with `serde_wasm_bindgen::from_value::<Vec<AstPattern>>()`, convert to `Vec<Pattern<Subject>>` via `ast_to_pattern`, call `gram_codec::to_gram`
- [X] T007 Replace the `validate` method body: serialize `Vec<String>` errors directly with `serde_wasm_bindgen::Serializer::json_compatible()` instead of serializing to JSON then calling `js_sys::JSON::parse`

### Fix PyO3 boundary (`crates/gram-codec/src/python.rs`)

- [X] T008 [P] Replace `gram_parse_to_json_py` (returns `PyResult<String>`) with `parse_py(py: Python, input: &str) -> PyResult<PyObject>`: call `gram_codec::parse_gram`, map to `Vec<AstPattern>`, serialize with `pythonize::pythonize(py, &asts)?`; update the module registration in `gram_codec()` to use the new name `parse`
- [X] T009 [P] Replace `gram_stringify_from_json_py` (accepts JSON `&str`) with `stringify_py(py: Python, patterns_obj: &PyAny) -> PyResult<String>`: deserialize with `pythonize::depythonize::<Vec<AstPattern>>(patterns_obj)?`, convert to `Vec<Pattern<Subject>>` via `ast_to_pattern`, call `gram_codec::to_gram`; update module registration to name `stringify`

### Verify

- [X] T010 `cargo build -p pattern-wasm --target wasm32-unknown-unknown` — verify WASM compiles without `JSON.parse` calls
- [X] T011 [P] `cargo build -p relateby-gram --features python` — verify PyO3 compiles with `pythonize`

**Checkpoint**: FFI layer is clean. WASM methods `parse`/`stringify`/`validate` return and accept `JsValue`. PyO3 functions `parse`/`stringify` return and accept Python objects directly.

---

## Phase 3: User Story 3 — `parse` and `stringify` with native objects (Priority: P1)

**Goal**: Python gains canonical `parse` / `stringify` functions that return and accept language-native `Pattern[Subject]` objects with no JSON string overhead. TypeScript's existing `Gram.parse` / `Gram.stringify` are updated to call the fixed WASM methods.

**Independent Test**: `from relateby.gram import parse, stringify` — call `parse("(a)-->(b)")`, assert result is `list[Pattern[Subject]]`; call `stringify(result)`, assert output is valid gram notation. Run `npx vitest run` in `typescript/packages/pattern` — existing tests pass.

### TypeScript (`typescript/packages/pattern/src/gram.ts`)

- [X] T012 [US3] Update `Gram.parse`: replace `JSON.parse(wasm.parseToJson(input))` with `wasm.parse(input)` (the WASM method now returns a `JsValue` directly); remove `JSON.parse` call; keep `decodePayload` / `patternFromRaw` pipeline unchanged
- [X] T013 [US3] Update `Gram.stringify`: replace `JSON.stringify(patterns.map(patternToRaw))` + `wasm.stringifyFromJson(json)` with `wasm.stringify(patternsJsValue)` where `patternsJsValue` is built from `patternToRaw` mapped results passed as `JsValue` via `serde_wasm_bindgen`

### Python wrapper (`python/packages/relateby/relateby/gram/__init__.py`)

- [X] T014 [P] [US3] Add `parse(input: str) -> list[Pattern[Subject]]` function: call `_gram.parse(input)` (now returns Python list of dicts directly), pass each dict to `pattern_from_dict`; keep `parse_gram` as an alias pointing to `parse`
- [X] T015 [P] [US3] Add `stringify(patterns: list[Pattern[Subject]]) -> str` function: convert each pattern to dict using the existing `_pattern_to_dict` helper, pass the list directly to `_gram.stringify()` (now accepts Python list of dicts); keep `gram_stringify` as an alias pointing to `stringify`
- [X] T016 [P] [US3] Update `__all__` in `python/packages/relateby/relateby/gram/__init__.py` to include `parse` and `stringify` alongside existing exports

### Type stubs (`python/packages/relateby/relateby/gram/__init__.pyi`)

- [X] T017 [P] [US3] Add type stubs for `parse(input: str) -> list[Pattern[Subject]]` and `stringify(patterns: list[Pattern[Subject]]) -> str` to `python/packages/relateby/relateby/gram/__init__.pyi`

### Verify

- [X] T018 [US3] Run `cd typescript/packages/pattern && npx vitest run` — all existing tests pass with updated WASM call sites (99/99 pass after WASM rebuild)
- [X] T019 [P] [US3] Run Python smoke test: `python -c "from relateby.gram import parse, stringify; p = parse('(a)-->(b)'); print(stringify(p))"` — assert round-trip works (verified via pytest: 141/141 pass)

**Checkpoint**: `parse` and `stringify` work in both languages with native Pattern objects and no JSON string overhead.

---

## Phase 4: User Story 1 — `parse_with_header` (Priority: P1)

**Goal**: Both Python and TypeScript can parse a gram document and receive the optional header record and the pattern list as separate values, each in their native types.

**Independent Test**: `header, patterns = parse_with_header("{version: 1} (a)-->(b)")` in Python — assert `header == {"version": 1}` and `len(patterns) == 1`. In TypeScript: `const { header, patterns } = await Effect.runPromise(Gram.parseWithHeader("{version: 1} (a)-->(b)"))` — assert `header.version === 1`.

### WASM (`adapters/wasm/pattern-wasm/src/gram.rs`)

- [X] T020 [US1] Add `parse_with_header(gram: &str) -> Result<JsValue, JsValue>` method to `Gram`: call `gram_codec::parse_gram_with_header`, construct a `ParseWithHeaderResult` from the `(Option<Record>, Vec<Pattern<Subject>>)` return value (converting patterns to `Vec<AstPattern>`), serialize with `serde_wasm_bindgen::Serializer::json_compatible()`

### PyO3 (`crates/gram-codec/src/python.rs`)

- [X] T021 [P] [US1] Add `parse_with_header_py(py: Python, input: &str) -> PyResult<PyObject>` function: call `gram_codec::parse_gram_with_header`, construct `ParseWithHeaderResult`, serialize with `pythonize::pythonize(py, &result)?`; register in `gram_codec` module as `parse_with_header`

### TypeScript (`typescript/packages/pattern/src/gram.ts`)

- [X] T022 [US1] Add `Gram.parseWithHeader(input: string): Effect<{ header: Record<string, unknown> | undefined, patterns: ReadonlyArray<Pattern<Subject>> }, GramParseError>`: call `wasm.parseWithHeader(input)`, read `result.header` and `result.patterns` from the returned `JsValue` object, decode patterns with `patternFromRaw` pipeline, return `{ header, patterns }`

### Python wrapper (`python/packages/relateby/relateby/gram/__init__.py`)

- [X] T023 [P] [US1] Add `parse_with_header(input: str) -> tuple[dict | None, list[Pattern[Subject]]]`: call `_gram.parse_with_header(input)` (returns `{"header": dict|None, "patterns": [...]}`), extract `raw["header"]` and convert `raw["patterns"]` via `pattern_from_dict`; return as two-tuple
- [X] T024 [P] [US1] Add `parse_with_header` to `__all__` in `python/packages/relateby/relateby/gram/__init__.py`

### Type stubs

- [X] T025 [P] [US1] Add stub for `parse_with_header(input: str) -> tuple[dict | None, list[Pattern[Subject]]]` in `python/packages/relateby/relateby/gram/__init__.pyi`
- [X] T026 [P] [US1] Rebuild `typescript/packages/pattern/dist/gram.d.ts` to include `parseWithHeader` signature (run `tsc` or the package build script)

### Tests

- [X] T027 [US1] Add `parse_with_header` tests to `python/packages/relateby/tests/test_gram_parity.py` (created): test with header, without header, header-only (no patterns), empty input, invalid gram string raises `GramParseError`
- [X] T028 [P] [US1] Add `parseWithHeader` tests to `typescript/packages/pattern/tests/gram-parity.test.ts` (created): test with header, without header, invalid input fails with `GramParseError`

**Checkpoint**: `parse_with_header` / `parseWithHeader` work in both languages and are tested.

---

## Phase 5: User Story 2 — `stringify_with_header` (Priority: P1)

**Goal**: Both Python and TypeScript can serialize a header record alongside a list of patterns into gram notation in a single call.

**Independent Test**: `gram = stringify_with_header({"version": 1}, patterns)` in Python — assert output starts with `{` and contains `version`; `header2, patterns2 = parse_with_header(gram)` — assert round-trip equality. Same in TypeScript with Effect pipeline.

### WASM (`adapters/wasm/pattern-wasm/src/gram.rs`)

- [X] T029 [US2] Add `stringify_with_header(input: JsValue) -> Result<String, JsValue>` method to `Gram`: deserialize via `serde_wasm_bindgen::from_value::<ParseWithHeaderResult>(input)?`, convert `patterns` to `Vec<Pattern<Subject>>` via `ast_to_pattern`, call `gram_codec::to_gram_with_header(header.unwrap_or_default(), &patterns)`, return gram notation string

### PyO3 (`crates/gram-codec/src/python.rs`)

- [X] T030 [P] [US2] Add `stringify_with_header_py(py: Python, result_obj: &PyAny) -> PyResult<String>` function: deserialize with `pythonize::depythonize::<ParseWithHeaderResult>(result_obj)?`, convert patterns to `Vec<Pattern<Subject>>` via `ast_to_pattern`, call `gram_codec::to_gram_with_header(header.unwrap_or_default(), &patterns)`; register in module as `stringify_with_header`

### TypeScript (`typescript/packages/pattern/src/gram.ts`)

- [X] T031 [US2] Add `Gram.stringifyWithHeader(header: Record<string, unknown> | undefined, patterns: ReadonlyArray<Pattern<Subject>>): Effect<string, GramParseError>`: convert patterns with `patternToRaw`, construct a `ParseWithHeaderResult`-shaped JS object `{ header: header ?? null, patterns: rawPatterns }`, call `wasm.stringifyWithHeader(inputJsValue)`, return the result string

### Python wrapper (`python/packages/relateby/relateby/gram/__init__.py`)

- [X] T032 [P] [US2] Add `stringify_with_header(header: dict | None, patterns: list[Pattern[Subject]]) -> str`: build `{"header": header, "patterns": [_pattern_to_dict(p) for p in patterns]}`, call `_gram.stringify_with_header(raw_obj)`, return gram notation string
- [X] T033 [P] [US2] Add `stringify_with_header` to `__all__` in `python/packages/relateby/relateby/gram/__init__.py`

### Type stubs

- [X] T034 [P] [US2] Add stub for `stringify_with_header(header: dict | None, patterns: list[Pattern[Subject]]) -> str` in `python/packages/relateby/relateby/gram/__init__.pyi`
- [X] T035 [P] [US2] Rebuild `typescript/packages/pattern/dist/gram.d.ts` to include `stringifyWithHeader` signature

### Tests

- [X] T036 [US2] Add `stringify_with_header` tests to `python/packages/relateby/tests/test_gram_parity.py`: non-empty header + patterns, empty header, header only (empty patterns), full round-trip (`parse_with_header → stringify_with_header → parse_with_header` equals originals)
- [X] T037 [P] [US2] Add `stringifyWithHeader` tests to `typescript/packages/pattern/tests/gram-parity.test.ts`: non-empty header + patterns, `undefined` header (output equals plain `stringify`), full round-trip (9 new TS tests pass)

**Checkpoint**: All four functions work in both languages. Round-trip guarantee holds.

---

## Phase 6: Polish & Cross-Cutting Concerns

- [X] T038 `cargo fmt --all` — format all Rust code
- [X] T039 [P] `cargo clippy --workspace -- -D warnings` — fix any lints introduced by new code (clean)
- [X] T040 [P] `cargo test --workspace` — all Rust unit tests pass (113/113 pass)
- [X] T041 [P] Run full Python test suite: `cd python/packages/relateby && pytest` — 141/141 pass including new parity tests (required: build wheel + copy .so into relateby/_native/)
- [X] T042 [P] Run full TypeScript test suite: `cd typescript/packages/pattern && npx vitest run` — 99/99 pass including new parity tests (required WASM rebuild after gram.rs changes)
- [X] T043 Run `./scripts/ci-local.sh` — full CI validation passes (all checks pass)
- [X] T044 [P] Verify quickstart.md examples execute correctly in both Python and TypeScript (Python verified; TS verified via 99/99 vitest passing)

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — **BLOCKS all user stories**
- **US3 (Phase 3)**: Depends on Phase 2 — TypeScript (T012–T013) and Python (T014–T016) are independent of each other
- **US1 (Phase 4)**: Depends on Phase 2 — WASM (T020) and PyO3 (T021) are independent; language wrappers (T022–T023) depend on their respective FFI tasks
- **US2 (Phase 5)**: Depends on Phase 4 (uses `parse_with_header` for round-trip tests); WASM (T029) and PyO3 (T030) can start as soon as Phase 2 is done
- **Polish (Phase 6)**: Depends on Phases 3–5

### User Story Dependencies

- **US3 (Phase 3)**: Can start after Phase 2 — independent of US1 and US2
- **US1 (Phase 4)**: Can start after Phase 2 — independent of US3 (different functions)
- **US2 (Phase 5)**: Can start FFI work (T029–T030) after Phase 2; language wrappers and tests depend on US1 being complete for round-trip tests

### Within Each Phase

- WASM tasks and PyO3 tasks within the same phase touch different files and can always run in parallel
- TypeScript wrapper tasks and Python wrapper tasks within the same phase touch different files and can always run in parallel
- FFI tasks must complete before their respective language wrapper tasks

---

## Parallel Opportunities per Phase

### Phase 2 — FFI Fix

```
T004 (define struct)
  ↓ (parallel)
T005 (WASM parse)    T006 (WASM stringify)   T007 (WASM validate)
T008 (PyO3 parse)    T009 (PyO3 stringify)
  ↓ (parallel)
T010 (verify WASM)   T011 (verify PyO3)
```

### Phase 4 — US1

```
T020 (WASM parse_with_header)    T021 (PyO3 parse_with_header_py)
  ↓                                  ↓
T022 (TS parseWithHeader)        T023 (Py parse_with_header)
  ↓ (parallel)
T025 (Py stub)   T026 (TS dist)   T027 (Py tests)   T028 (TS tests)
```

### Phase 5 — US2

```
T029 (WASM stringify_with_header)    T030 (PyO3 stringify_with_header_py)
  ↓                                       ↓
T031 (TS stringifyWithHeader)        T032 (Py stringify_with_header)
  ↓ (parallel)
T034 (Py stub)   T035 (TS dist)   T036 (Py tests)   T037 (TS tests)
```

---

## Implementation Strategy

### MVP (US3 + US1 only)

1. Phase 1: Add deps
2. Phase 2: Fix FFI layer
3. Phase 3 (US3): `parse` / `stringify` in both languages
4. Phase 4 (US1): `parse_with_header` in both languages
5. **STOP and validate**: round-trip `parse_with_header → stringify_with_header` is not yet available, but reading gram files with headers works

### Full delivery

1. Phases 1–3: Foundation + basic parse/stringify
2. Phase 4: Add `parse_with_header`
3. Phase 5: Add `stringify_with_header` — completes the round-trip guarantee
4. Phase 6: Polish and CI

### Parallel team strategy

- Developer A: WASM tasks (T005–T007, T010, T020, T022, T026, T028–T029, T031, T035, T037)
- Developer B: PyO3 + Python tasks (T008–T009, T011, T014–T017, T021, T023–T025, T027, T030, T032–T034, T036)

Both can work in parallel from Phase 2 onwards.

---

## Notes

- `pythonize` version must match PyO3 version — both must be `0.23`
- `serde-wasm-bindgen::Serializer::json_compatible()` is required (not the default serializer) to map `HashMap` to plain JS objects instead of `Map`
- `ast_to_pattern` already exists in `crates/gram-codec/src/json.rs` — reuse it in new WASM/PyO3 functions
- `pattern_from_dict` and `_pattern_to_dict` already exist in the Python package — reuse them
- The JSON interchange functions in `gram-codec/src/json.rs` (`gram_parse_to_json`, `gram_stringify_from_json`) are not removed — they remain as a public utility but are no longer the FFI bridge
- Commit after each phase checkpoint, not after each individual task
