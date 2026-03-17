# Tasks: Native TypeScript and Python Bindings

**Input**: Design documents from `specs/039-native-bindings/`
**Branch**: `039-native-bindings`
**Design reference**: `proposals/migrate-ts-python-proposal.md` (TypeScript sketch is normative)

## Format: `[ID] [P?] [Story?] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story this task belongs to (US1–US5)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Validate the starting state and wire up dependencies before any implementation begins.

- [X] T001 Verify `crates/gram-codec` compiles standalone for `wasm32-unknown-unknown`: run `cargo build -p gram-codec --target wasm32-unknown-unknown` and confirm it succeeds without `pattern-core`
- [X] T002 [P] Verify or add `serde_json` to `crates/gram-codec/Cargo.toml` dependencies (needed for JSON output functions)
- [X] T003 [P] Promote `effect` from `peerDependenciesMeta` optional to required `peerDependencies` in `typescript/@relateby/pattern/package.json`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Add `gram_parse_to_json` / `gram_stringify_from_json` to the Rust gram-codec. This JSON interchange surface is the sole contract all host-language implementations depend on. No user story implementation can begin until these functions exist and are verified.

**⚠️ CRITICAL**: All user story phases depend on this phase completing successfully.

- [X] T004 Create `crates/gram-codec/src/json.rs` implementing `gram_parse_to_json(input: &str) -> Result<String, String>` (calls existing `parse_gram`, serializes with `serde_json`) and `gram_stringify_from_json(input: &str) -> Result<String, String>` (deserializes JSON, calls existing `to_gram`)
- [X] T005 [P] Add Rust unit tests in `crates/gram-codec/src/json.rs`: round-trip `parse gram → JSON string → stringify back → parse again`, verify `Equal` on result; cover empty input (`"[]"`) and invalid input (error string returned)
- [X] T006 Expose `gram_parse_to_json` and `gram_stringify_from_json` via PyO3 in `crates/gram-codec/src/python.rs` (add two `#[pyfunction]` entries returning `PyResult<String>`)
- [X] T007 Expose `gram_parse_to_json` and `gram_stringify_from_json` via WASM in `crates/pattern-wasm/src/gram.rs` (add two `#[wasm_bindgen]` functions alongside existing `Gram` methods — keep existing methods working in parallel)
- [X] T008 Verify JSON output format against the contract in `specs/039-native-bindings/data-model.md`: confirm `"subject"` key (not `"value"`), `labels` as array, all 11 Value variant discriminants, and that `gram_parse_to_json` output matches existing `parse_patterns_as_dicts` Python output structure

**Checkpoint**: `gram_parse_to_json` and `gram_stringify_from_json` are callable from Rust tests, WASM, and Python. JSON format is verified against the interchange contract. User story work can now begin.

---

## Phase 3: User Story 1 — Native TypeScript Pattern (Priority: P1) 🎯 MVP

**Goal**: A TypeScript developer parses gram notation and receives `Pattern<Subject>` objects with directly readable fields, structural equality via `Equal.equals`, and synchronous operations (`map`, `fold`, `filter`) that never cross the WASM boundary.

**Independent Test**: `import { Gram, fold, findFirst } from "@relateby/pattern"` — parse a gram string, verify `Equal.equals(p1, p2)` is `true` for two parses of the same input, run `pipe(pattern, fold([], (acc, s) => [...acc, s.identity]))` synchronously, confirm `console.log(pattern)` shows plain data fields.

### Implementation

- [X] T009 [P] [US1] Implement `typescript/@relateby/pattern/src/value.ts`: 11 `Data.Case` interfaces (StringVal through MeasurementVal), `Value` union type, `Value` constructor namespace using `Data.tagged<Interface>("Tag")` for each variant, and `ValueSchema` as `Schema.Union` of `Schema.TaggedStruct` branches (ArrayVal and MapVal branches use `Schema.suspend(() => ValueSchema)` for nested values)
- [X] T010 [P] [US1] Implement `typescript/@relateby/pattern/src/subject.ts`: `Subject extends Data.Class<{ identity: string; labels: ReadonlySet<string>; properties: ReadonlyMap<string, Value> }>` with `Subject.fromId`, `withLabel`, and `withProperty` immutable builder methods
- [X] T011 [US1] Implement `typescript/@relateby/pattern/src/pattern.ts`: `Pattern<V> extends Data.Class<{ value: V; elements: ReadonlyArray<Pattern<V>> }>` with `Pattern.point` and `Pattern.of` static constructors and `isAtomic`, `length`, `size`, `depth` getters (depends on T009, T010)
- [X] T012 [US1] Implement `typescript/@relateby/pattern/src/ops.ts`: all seven standalone curried pipeable functions — `map`, `fold` (pre-order: `fn(init, p.value)` then recurse), `filter`, `findFirst` (returns `Option.Option<V>` using `Option.orElse` to short-circuit across elements), `extend` (comonad: `new Pattern({ value: fn(p), elements: p.elements.map(extend(fn)) })`), `extract` (`p.value`), `duplicate` (`new Pattern({ value: p, elements: p.elements.map(duplicate) })`) (depends on T011)
- [X] T013 [US1] Implement `typescript/@relateby/pattern/src/schema.ts`: `RawSubjectSchema` (Schema.Struct with identity/labels/properties), `RawPatternSchema` (Schema.Struct with `subject` field and `elements: Schema.Array(Schema.suspend(...))` — note field name is `"subject"` not `"value"`), `decodePayload = Schema.decodeUnknownSync(Schema.Array(RawPatternSchema))`, and `patternFromRaw(raw): Pattern<Subject>` pure recursive constructor (depends on T009, T010, T011)
- [X] T014 [P] [US1] Implement `typescript/@relateby/pattern/src/errors.ts`: `GramParseError extends Data.TaggedError("GramParseError")<{ readonly input: string; readonly cause: unknown }>` — `Data.TaggedError` gives it a `_tag`, structured fields, and a proper `Error` prototype chain
- [X] T015 [US1] Update `typescript/@relateby/pattern/src/gram.ts`: replace `async parse(): Promise<...>` with `parse(input): Effect.Effect<ReadonlyArray<Pattern<Subject>>, GramParseError>` implemented as `pipe(Effect.tryPromise({ try: async () => JSON.parse(wasm.gram_parse_to_json(input)), catch: cause => new GramParseError(...) }), Effect.flatMap(raw => Effect.try({ try: () => decodePayload(raw).map(patternFromRaw), catch: cause => new GramParseError(...) })))`; add `stringify` and `validate` with the same `Effect.tryPromise` pattern (depends on T013, T014)
- [X] T016 [US1] Update `typescript/@relateby/pattern/src/index.ts` to export all new native types (`Pattern`, `Subject`, `Value`, `map`, `fold`, `filter`, `findFirst`, `extend`, `extract`, `duplicate`, `GramParseError`, `Gram`) alongside existing WASM-backed exports (cutover happens in Polish phase) (depends on T009–T015)
- [X] T017 [P] [US1] Add property-based law tests in `typescript/@relateby/pattern/tests/pattern-laws.test.ts` using `fast-check`: functor identity law (`map(id)(p)` equals `p` via `Equal.equals`), functor composition law, foldable pre-order traversal order, comonad laws (`extract(extend(f)(p)) == f(p)`, `extend(extract)(p)` equals `p`) (depends on T012)
- [X] T018 [US1] Run existing `vitest` test suite in `typescript/@relateby/pattern/tests/` against the new native implementation; fix any regressions (depends on T016)

**Checkpoint**: `Equal.equals(p1, p2)` works for two parsed Patterns. `pipe(pattern, fold([], fn))` runs synchronously. `console.log(pattern)` shows plain fields. All existing TS tests pass.

---

## Phase 4: User Story 2 — Native Python Pattern (Priority: P1)

**Goal**: A Python developer parses gram notation and receives `Pattern` objects that are plain Python dataclasses — `repr()` correctly, `==` works structurally, and `fold` executes with a Python lambda without any PyO3 round-trip per node.

**Independent Test**: `from relateby.pattern import Pattern, Subject, parse_gram` — parse a gram string, verify `p1 == p2` for two parses of the same input, call `pattern.fold([], lambda acc, s: acc + [s.identity])` with a Python lambda, confirm `print(pattern)` shows fields.

### Implementation

- [X] T019 [P] [US2] Implement `python/relateby/relateby/pattern/_value.py`: 11 `@dataclass` classes (StringVal, IntVal, FloatVal, BoolVal, NullVal, SymbolVal, TaggedStringVal, ArrayVal, MapVal, RangeVal, MeasurementVal), `Value` union type alias, and `value_from_dict(d: dict | str | int | float | bool | None) -> Value` decoder that handles all 11 variants from the JSON interchange format
- [X] T020 [P] [US2] Implement `python/relateby/relateby/pattern/_subject.py`: `@dataclass Subject` with `identity: str`, `labels: set[str]`, `properties: dict[str, Value]`, `Subject.from_id` classmethod, `with_label` and `with_property` immutable builder methods
- [X] T021 [US2] Implement `python/relateby/relateby/pattern/_pattern.py`: generic `@dataclass Pattern(Generic[V])` with `value: V` and `elements: list[Pattern[V]]`, `Pattern.point` and `Pattern.of` classmethods, `is_atomic`/`length`/`size`/`depth` properties, and all seven operations as methods: `map`, `fold` (pre-order), `filter`, `find_first` (returns `Optional[V]`), `extend`, `extract`, `duplicate` (depends on T019, T020)
- [X] T022 [US2] Implement `python/relateby/relateby/pattern/_decode.py`: `pattern_from_dict(d: dict) -> Pattern[Subject]` recursive decoder that constructs native `Subject` (handling labels as list→set, properties via `value_from_dict`) and recursively decodes `elements` (depends on T019, T020, T021)
- [X] T023 [P] [US2] Add property-based law tests in `python/relateby/tests/test_pattern_laws.py` using `hypothesis`: functor identity law (`p.map(lambda x: x) == p`), functor composition, foldable pre-order order, comonad laws (depends on T021)
- [X] T024 [US2] Update `python/relateby/relateby/pattern/__init__.py` to import `Pattern`, `Subject`, `Value` and all Value variant classes from the new native modules; keep existing PyO3-backed classes available under aliased names during transition (depends on T019–T022)
- [X] T025 [US2] Update `python/relateby/relateby/pattern/__init__.pyi` type stubs to reflect all new native types, `Pattern[V]` generic, `Value` union, and all builder methods (depends on T024)
- [X] T026 [US2] Update `python/relateby/relateby/gram/__init__.py` to call `gram_codec.gram_parse_to_json(input)`, JSON-decode the result, and feed to `pattern_from_dict` — return `list[Pattern[Subject]]` directly instead of the old `ParseResult` wrapper (depends on T022, T006)
- [X] T027 [US2] Run full `pytest` suite in `python/relateby/tests/` and `crates/pattern-core/tests/python/` against the new native implementation; fix any regressions (depends on T024–T026)

**Checkpoint**: `Pattern` prints as a dataclass. `p1 == p2` is `True` for same structure. `fold` with a Python lambda runs purely in Python. All existing Python tests pass.

---

## Phase 5: User Story 3 — Gram Parsing Surfaces Errors Explicitly (Priority: P2)

**Goal**: Developers receive a structured error value — not a thrown exception — when gram parsing fails. TypeScript callers use `Effect.match` or `Effect.catchAll`; Python callers catch a `GramParseError` exception with structured fields.

**Independent Test**: Pass `"not valid gram ##!!"` to `Gram.parse` (TS) and `parse_gram` (Python). In TS: verify the return value is an `Effect` failure carrying a `GramParseError` with `.input` and `.cause` fields without throwing. In Python: verify a `GramParseError` is raised with `.input` and `.cause` attributes.

### Implementation

- [X] T028 [US3] Implement `GramParseError` exception class in `python/relateby/relateby/gram/__init__.py` with `input: str` and `cause: str` attributes; update `parse_gram` to catch PyO3 errors and re-raise as `GramParseError` (depends on T026)
- [X] T029 [P] [US3] Add error scenario tests in `typescript/@relateby/pattern/tests/gram-errors.test.ts`: invalid input produces `Effect` failure with `GramParseError` instance; valid input produces `Effect` success; `GramParseError` carries original `input` string (depends on T015)
- [X] T030 [P] [US3] Add error scenario tests in `python/relateby/tests/test_gram_errors.py`: invalid input raises `GramParseError`; valid input returns `list[Pattern[Subject]]`; error has `.input` attribute matching the original string (depends on T028)
- [X] T031 [US3] Implement `Gram.validate(input): Effect.Effect<void, GramParseError>` in `typescript/@relateby/pattern/src/gram.ts` using `Effect.tryPromise` wrapping the WASM `gram_validate` function (depends on T015)
- [X] T032 [US3] Implement `gram_validate(input: str) -> list[str]` in `python/relateby/relateby/gram/__init__.py` returning a list of error strings (empty = valid); update `python/relateby/relateby/gram/__init__.pyi` stubs (depends on T026)

**Checkpoint**: `Gram.parse("bad input")` returns an `Effect` failure — no `throw`. Python `parse_gram("bad input")` raises `GramParseError` with structured fields.

---

## Phase 6: User Story 4 — Maintainer Adds Operations Without Touching Rust (Priority: P2)

**Goal**: Demonstrate that the native Pattern layer is self-contained by adding a new operation entirely in TypeScript and Python, with no Rust changes, no `wasm-pack` rebuild, and no `maturin develop` invocation.

**Independent Test**: Add `values()` (returns all values in pre-order traversal order) to `ops.ts` and `_pattern.py`, run `vitest` and `pytest` — neither requires a Rust rebuild.

### Implementation

- [X] T033 [P] [US4] Add `values<V>(p: Pattern<V>): ReadonlyArray<V>` to `typescript/@relateby/pattern/src/ops.ts` (implemented as `pipe(p, fold([] as V[], (acc, v) => [...acc, v]))`); export from `typescript/@relateby/pattern/src/index.ts` — verify no Rust file was touched (depends on T012, T016)
- [X] T034 [P] [US4] Add `def values(self) -> list[V]` method to `python/relateby/relateby/pattern/_pattern.py` (implemented using `self.fold([], lambda acc, v: acc + [v])`); update `python/relateby/relateby/pattern/__init__.pyi` — verify no Rust file was touched (depends on T021, T025)
- [X] T035 [US4] Add tests in `typescript/@relateby/pattern/tests/pattern-ops.test.ts` and `python/relateby/tests/test_pattern_ops.py` verifying `values()` returns the correct pre-order sequence (depends on T033, T034)

**Checkpoint**: `values()` works in both languages. No `cargo build`, `wasm-pack`, or `maturin` commands were needed.

---

## Phase 7: User Story 5 — Native StandardGraph (Priority: P3)

**Goal**: A developer constructs a `StandardGraph` from parsed patterns entirely in native TypeScript or Python. The graph correctly classifies patterns as Nodes, Relationships, Annotations, Walks, or Other using the gram-hs classification rules — with no WASM boundary calls after the initial parse.

**Independent Test**: Parse `"(a:Person)-->(b:Person)"`, call `StandardGraph.fromPatterns(patterns)`, verify `nodeCount === 2`, `relationshipCount === 1`. Parse `"(a) [r] ->(b)"` (annotation), verify `annotationCount === 1`.

### Implementation

- [ ] T036 [P] [US5] Implement `typescript/@relateby/pattern/src/standard-graph.ts`: `StandardGraph` class with private maps for nodes/relationships/annotations/walks/other; `fromPatterns(patterns)` applies the 5-class classification (0 elements→Node, 1→Annotation, 2+both-nodes→Relationship, valid identity chain→Walk, else→Other); walk validity check ports the identity-chain predicate from gram-hs `GraphClassifier.hs`; `fromGram(input)` is `pipe(Gram.parse(input), Effect.map(StandardGraph.fromPatterns))`; `node(id)` and `relationship(id)` return `Option.Option<...>` (depends on T015, T012)
- [ ] T037 [P] [US5] Implement `python/relateby/relateby/pattern/_standard_graph.py`: `StandardGraph` class with the same 5-class classification logic; `from_patterns(patterns)` class method; `from_gram(input)` class method using `parse_gram` + `from_patterns`; `node(id)` and `relationship(id)` return `Pattern[Subject] | None` (depends on T026, T021)
- [ ] T038 [US5] Update `typescript/@relateby/pattern/src/index.ts` to export `StandardGraph` (depends on T036, T016)
- [ ] T039 [US5] Update `python/relateby/relateby/pattern/__init__.py` and `__init__.pyi` to export `StandardGraph` from `_standard_graph.py` (depends on T037, T024)
- [ ] T040 [P] [US5] Add tests in `typescript/@relateby/pattern/tests/standard-graph.test.ts`: node/relationship/annotation/walk classification; `fromGram` convenience constructor; `node(id)` returns `Option.some`/`Option.none` correctly (depends on T036)
- [ ] T041 [P] [US5] Add tests in `python/relateby/tests/test_standard_graph.py`: node/relationship/annotation/walk classification; `from_gram` convenience constructor (depends on T037)

**Checkpoint**: `StandardGraph.fromPatterns` correctly classifies all 5 element classes. `fromGram` composes parse + classify. Querying nodes/relationships by id works.

---

## Phase 8: Polish & Cutover

**Purpose**: Remove the WASM and PyO3 type layers now that all native implementations are verified; update docs and examples; run full CI.

- [ ] T042 Delete `crates/pattern-wasm/src/convert.rs` (the bidirectional Rust↔WASM type conversion layer — only deletable after all TS tests pass with the new native path)
- [ ] T043 Delete `crates/pattern-wasm/src/standard_graph.rs` (WASM StandardGraph wrapper — superseded by native TypeScript)
- [ ] T044 Update `crates/pattern-wasm/src/lib.rs` to remove re-exports of `pattern-core` types; retain only `gram_parse_to_json`, `gram_stringify_from_json`, `gram_validate` re-exports
- [ ] T045 Update `crates/pattern-wasm/src/gram.rs` to remove the old `Gram.parse` / `Gram.parseOne` / `Gram.stringify` WASM methods; retain only the new JSON string functions
- [ ] T046 [P] Slim `crates/pattern-core/src/python.rs` to ~50 lines: remove `PyPattern`, `PySubject`, `PyValue`, `PyStandardGraph`, `PyValidationRules`, `PyStructureAnalysis`, `PySubjectBuilder`, `PyValidationError`; retain only the module registration boilerplate and any remaining codec-only PyO3 bindings
- [ ] T047 Update `typescript/@relateby/pattern/src/index.ts` to remove WASM-backed `NativePattern`, `NativeSubject`, `WasmStandardGraph` and related wrapper exports (depends on T042–T045, all US phases complete)
- [ ] T048 Update `python/relateby/relateby/pattern/__init__.py` to remove aliased PyO3-backed classes (depends on T046, all Python US phases complete)
- [ ] T049 Rebuild WASM (`wasm-pack build`) and measure binary size; verify ≥40% reduction from pre-migration baseline; document result in `specs/039-native-bindings/benchmarks.md`
- [ ] T050 [P] Run performance benchmarks: TypeScript `fold` over 10,000-node tree vs WASM-bridge baseline (≥5× faster); Python `fold` over 1,000-node tree vs PyO3 round-trip baseline (≥10× faster); document in `specs/039-native-bindings/benchmarks.md`
- [ ] T051 [P] Update `docs/python-usage.md` with native API examples (dataclass Pattern, `fold` with lambda, `GramParseError`)
- [ ] T052 [P] Update `typescript/@relateby/pattern/README.md` with effect-ts examples (`pipe`, `Effect.runPromise`, `Option.getOrUndefined`, `Equal.equals`)
- [ ] T053 [P] Update `examples/` for both TypeScript and Python to use the new native API
- [ ] T054 Run full CI validation: `cargo fmt --all`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace`, `./scripts/ci-local.sh` — all must pass clean

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — **BLOCKS all user story phases**
- **US1 TypeScript (Phase 3)**: Depends on Phase 2 (needs `gram_parse_to_json` in WASM)
- **US2 Python (Phase 4)**: Depends on Phase 2 (needs `gram_parse_to_json` in PyO3) — **independent of US1**
- **US3 Errors (Phase 5)**: Depends on Phase 3 (T015) and Phase 4 (T026)
- **US4 Ops Demo (Phase 6)**: Depends on Phase 3 (T012, T016) and Phase 4 (T021, T025) — can run in parallel with Phase 5
- **US5 StandardGraph (Phase 7)**: Depends on Phase 3 (T015) and Phase 4 (T026)
- **Polish (Phase 8)**: Depends on all user story phases being verified

### User Story Dependencies

- **US1 (P1)**: Unblocked after Phase 2 — no story dependencies
- **US2 (P1)**: Unblocked after Phase 2 — no story dependencies, **parallel with US1**
- **US3 (P2)**: Requires US1 (T015) and US2 (T026) — error surface sits atop both implementations
- **US4 (P2)**: Requires US1 (T012) and US2 (T021) — validates that ops layer is fully native
- **US5 (P3)**: Requires US1 (T015) and US2 (T026) for `fromGram` — graph classification itself is independent

### Within Each Phase

- `[P]`-marked tasks within a phase can run concurrently
- T009 and T010 (value.ts and subject.ts) are independent — launch together
- T019 and T020 (Python value and subject) are independent — launch together
- T017 (law tests) and T016 (index.ts exports) are independent once T012 is done
- T036 (TS StandardGraph) and T037 (Python StandardGraph) are independent — launch together
- T042–T046 (cutover) have no inter-dependencies — can be done in any order or together

---

## Parallel Execution Examples

### Phase 3 (US1): Launch value and subject together

```
Task T009: Implement typescript/@relateby/pattern/src/value.ts
Task T010: Implement typescript/@relateby/pattern/src/subject.ts
(wait for both)
Task T011: Implement typescript/@relateby/pattern/src/pattern.ts
Task T014: Implement typescript/@relateby/pattern/src/errors.ts
(wait for T011)
Task T012: Implement typescript/@relateby/pattern/src/ops.ts
Task T013: Implement typescript/@relateby/pattern/src/schema.ts
(wait for T012, T013, T014)
Task T015: Update typescript/@relateby/pattern/src/gram.ts
(wait for T015)
Task T016: Update typescript/@relateby/pattern/src/index.ts
Task T017: Add fast-check law tests
Task T018: Validate existing test suite
```

### Phase 4 (US2): Launch Python value and subject together

```
Task T019: Implement python/relateby/relateby/pattern/_value.py
Task T020: Implement python/relateby/relateby/pattern/_subject.py
(wait for both)
Task T021: Implement python/relateby/relateby/pattern/_pattern.py
Task T022: Implement python/relateby/relateby/pattern/_decode.py
Task T023: Add hypothesis law tests
(wait for T021, T022)
Task T024: Update python/relateby/relateby/pattern/__init__.py
Task T026: Update python/relateby/relateby/gram/__init__.py
(wait for T024, T025, T026)
Task T027: Validate existing pytest suite
```

### Phase 7 (US5): Launch TS and Python StandardGraph together

```
Task T036: Implement typescript/@relateby/pattern/src/standard-graph.ts
Task T037: Implement python/relateby/relateby/pattern/_standard_graph.py
(wait for both)
Task T038 + T039: Update index.ts and __init__.py exports
Task T040 + T041: Add tests
```

---

## Implementation Strategy

### MVP (User Stories 1 + 2 only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational JSON surface ← **CRITICAL GATE**
3. Complete Phase 3: US1 Native TypeScript Pattern
4. Complete Phase 4: US2 Native Python Pattern (can overlap with Phase 3)
5. **STOP and VALIDATE**: both languages return native objects, structural equality works, fold is synchronous
6. Phases 3 + 4 together represent the full core value of the migration

### Incremental Delivery

1. Setup + Foundational → JSON interchange ready
2. US1 + US2 → Native Pattern/Subject/Value in both languages (MVP)
3. US3 → Explicit typed error handling (quality-of-life)
4. US4 → Validated extensibility story
5. US5 → Native StandardGraph
6. Polish → Cutover, docs, CI

### Parallel Team Strategy

After Phase 2 completes:
- **Developer A**: Phase 3 (US1 TypeScript)
- **Developer B**: Phase 4 (US2 Python)
- Both streams are fully independent — different files, different languages

---

## Notes

- `[P]` tasks touch different files and have no dependencies on incomplete tasks in the same phase
- The JSON field key is `"subject"` (not `"value"`) in the interchange format — see `data-model.md`
- `Schema.suspend` in `schema.ts` is mandatory for the recursive `elements` field in `RawPatternSchema`
- `ops.ts` uses standalone curried functions (not methods) — this enables `pipe` composition and tree-shaking
- Do not remove WASM-backed exports from `index.ts` until Phase 8 — parallel period ensures no regressions
- `findFirst` returns `Option.Option<V>` — use `Option.getOrUndefined` at call sites that need `V | undefined`
- Comonad laws (`extend`/`extract`/`duplicate`) are tested in T017 — verify before cutover
