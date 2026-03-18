# Implementation Plan: Native TypeScript and Python Bindings

**Branch**: `039-native-bindings` | **Date**: 2026-03-17 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/039-native-bindings/spec.md`

---

## Summary

Move `Pattern`, `Subject`, `Value`, and `StandardGraph` from Rust-backed WASM/PyO3 objects to native TypeScript (using effect-ts) and Python (using dataclasses). The Rust gram-codec parser/serializer remains unchanged; it gains two new thin wrapper functions (`gram_parse_to_json`, `gram_stringify_from_json`) that return plain JSON strings. A stable JSON interchange format becomes the sole contract crossing the native extension boundary. All Pattern operations (`map`, `fold`, `filter`, `extend`, etc.) run entirely in the host language, eliminating per-operation FFI overhead.

**Behavioral reference**: gram-hs at `../pattern-hs/libs/` is authoritative for all operation semantics, Value variants, and StandardGraph classification rules.

---

## Technical Context

**Languages**:
- TypeScript 5.4+ (`strict: true`), effect >=3.0.0 (promoted from optional peer dep to required)
- Python 3.8+, stdlib only (`dataclasses`, `typing`, standard collections)
- Rust 1.70+ (MSRV unchanged), gram-codec crate only (no changes to pattern-core)

**Primary Dependencies**:
- TypeScript: `effect` (Data, Schema, Effect, Option, pipe), `vitest`, `fast-check`
- Python: `dataclasses` (stdlib), `pytest`, `hypothesis`
- Rust: `serde_json` (adding to gram-codec for JSON output)

**Storage**: N/A (in-memory only)

**Testing**:
- TypeScript: vitest + fast-check (property-based law tests)
- Python: pytest + hypothesis (property-based law tests)
- Rust: cargo test (gram-codec codec round-trip tests)

**Target Platform**: Browser (WASM/ESM), Node.js (WASM/CJS), Python 3.8+

**Performance Goals**:
- Pattern `fold`/`map` ≥5× faster than current WASM-bridge (TypeScript)
- Python `fold` over 1,000-node tree ≥10× faster than current PyO3 per-node round-trips
- Compiled WASM binary ≥40% smaller (pattern-core excluded)

**Constraints**:
- Import paths unchanged: `@relateby/pattern`, `@relateby/gram`, `relateby.pattern`, `relateby.gram`
- All existing tests must pass (no behavior regressions)
- `Gram.parse` return type changes from `Promise` to `Effect` (documented breaking change)

**Project Type**: Multi-language library (TypeScript/WASM + Python/PyO3 + native Rust)

**Scale/Scope**:
- ~1,200 lines new TypeScript (replaces ~610 lines WASM-backed TS + 333-line convert.rs)
- ~500 lines new Python (replaces ~1,657-line python.rs)
- ~50 lines new Rust (gram-codec JSON wrappers; all else removed from WASM surface)

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| **I. Reference Implementation Fidelity** | ✅ PASS | gram-hs cross-check completed in research phase. 4 additional Value variants identified (TaggedString, Array, Map, Range, Measurement) vs. proposal sketch. Fold order confirmed pre-order. Walk classification rules confirmed. All findings encoded in research.md and data-model.md. |
| **II. Correctness & Compatibility** | ✅ PASS | Existing test suites are the regression baseline throughout all phases. Property-based law tests (functor, comonad, foldable) added in Phases 2 and 3. No behavior regressions permitted. |
| **III. Rust Native Idioms** | ✅ PASS | The Rust that remains (gram-codec) is untouched except for two new thin JSON wrapper functions using existing `serde_json`. No new Rust patterns introduced. |
| **IV. Multi-Target Library Design** | ✅ PASS with action | `crates/gram-codec` must be verified to compile standalone for `wasm32-unknown-unknown` (currently it depends on `pattern-core` for return types). The new `gram_parse_to_json` returns `String` — no `pattern-core` dependency. Verification step added to Phase 1. |
| **V. External Language Bindings & Examples** | ✅ PASS with action | Examples in `examples/` must be updated to use the new native API surface (Phase 6). |

**No Complexity Tracking violations** — this feature reduces complexity (deletes convert.rs, shrinks python.rs).

---

## Project Structure

### Documentation (this feature)

```text
specs/039-native-bindings/
├── plan.md              # This file
├── research.md          # Phase 0: gram-hs cross-check findings
├── data-model.md        # Phase 1: entity definitions and JSON interchange format
├── contracts/
│   ├── rust-codec-api.md      # New gram-codec JSON functions
│   ├── typescript-public-api.md  # @relateby/pattern public API
│   └── python-public-api.md   # relateby.pattern public API
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code

```text
crates/gram-codec/src/
├── lib.rs                   UPDATED: add gram_parse_to_json, gram_stringify_from_json
├── json.rs                  NEW: JSON serialization of Pattern<Subject> (thin wrapper)
└── python.rs                UPDATED: expose gram_parse_to_json via PyO3

crates/pattern-wasm/src/
├── lib.rs                   UPDATED: export only gram codec functions
├── gram.rs                  UPDATED: slim to gram_parse_to_json, gram_stringify_from_json, gram_validate
├── convert.rs               DELETED
└── standard_graph.rs        DELETED

crates/pattern-core/src/
└── python.rs                SLIMMED: ~50 lines (codec only, rest deleted after Phase 5)

typescript/@relateby/pattern/src/
├── value.ts                 NEW: Value tagged union (Data.tagged, 10 variants)
├── subject.ts               NEW: Subject (Data.Class)
├── pattern.ts               NEW: Pattern<V> (Data.Class + static constructors)
├── ops.ts                   NEW: pipeable operations (map, fold, filter, findFirst, extend, extract, duplicate)
├── schema.ts                NEW: Schema.suspend recursive decoder for JSON payload
├── standard-graph.ts        NEW: StandardGraph native classification
├── errors.ts                NEW: GramParseError (Data.TaggedError)
├── gram.ts                  UPDATED: Gram using Effect + gram_parse_to_json
├── graph/transforms.ts      UPDATED: adapt to native Pattern (or remove if superseded)
└── index.ts                 UPDATED: re-export new native types; remove WASM wrappers

python/relateby/relateby/pattern/
├── _value.py                NEW: Value dataclass hierarchy (11 classes + union type)
├── _subject.py              NEW: Subject dataclass
├── _pattern.py              NEW: Pattern dataclass + all operations
├── _standard_graph.py       NEW: StandardGraph classification
├── _decode.py               NEW: pattern_from_dict() decoder for gram-codec dict output
├── __init__.py              UPDATED: re-export from new modules; remove PyO3 wrappers
└── __init__.pyi             UPDATED: type stubs for all new native types

python/relateby/relateby/gram/
├── __init__.py              UPDATED: use gram_parse_to_json; return list[Pattern[Subject]]
└── __init__.pyi             UPDATED: type stubs

typescript/@relateby/pattern/tests/
└── pattern.test.ts          UPDATED: tests for native Pattern, law tests (fast-check)

crates/pattern-core/tests/python/
└── (all test files)         UPDATED: tests against native Python Pattern/Subject/Value
```

---

## TypeScript Architectural Design

> The implementation sketch in `proposals/migrate-ts-python-proposal.md` is normative for the TypeScript layer. The choices below are not incidental — each effect-ts tool was selected because the previous WASM approach made it impossible or awkward.

### Why `Data.Class` for `Pattern<V>` and `Subject`

`Data.Class` gives structural equality via `Equal.equals` at no extra cost. The WASM-handle approach made `p1 === p2` always false for separately-parsed patterns representing the same structure. With `Data.Class`, correctness tests can use `Equal.equals` directly and property-based tests can generate and compare arbitrary patterns without custom comparators.

### Why `Data.Case` + `Data.tagged` for `Value`

`Data.tagged("StringVal")` creates a constructor that automatically fills `_tag: "StringVal"` and implements structural equality. The 11-variant `Value` union maps cleanly to TypeScript's exhaustive `switch (v._tag)`. Variant-level equality (e.g. `Equal.equals(Value.Int({value: 42}), Value.Int({value: 42}))`) works without writing comparators.

### Why `Schema.suspend` for the decode pipeline

The JSON payload from the WASM codec contains a self-referential `elements` array. `Schema.suspend(() => RawPatternSchema)` defers the self-reference so TypeScript's type-checker and the Schema runtime can both handle it. The full tree is validated in a single `Schema.decodeUnknownSync` call before any `Pattern` is constructed. If the codec returns unexpected output (e.g. a missing `"subject"` key), the error is caught here and surfaced as a `GramParseError` — not a `TypeError` mid-recursion.

`Schema.TaggedStruct` handles the `Value` variants: it attaches the `_tag` literal, validates the fields, and produces a typed value in one step.

### Why `Effect` replaces `Promise`

`Gram.parse` involves two failure modes — WASM load/invocation failure and Schema decode failure — which previously produced untyped thrown exceptions. `Effect<A, GramParseError>` puts both failure modes in the type signature. Callers use `Effect.match` or `Effect.catchAll` instead of `try/catch`. For callers that need a `Promise`, `Effect.runPromise` converts at the boundary.

### Why standalone curried functions instead of methods

`map`, `fold`, `filter`, `findFirst`, `extend`, `extract`, `duplicate` are exported as standalone curried functions and composed with `pipe`. This enables:
- **Point-free style**: `pipe(pattern, fold([], fn))` without intermediate variables
- **Tree-shaking**: unused operations are not bundled
- **Composability**: `pipe(Gram.parse(s), Effect.map(patterns => patterns.map(fold([], fn))))` chains naturally

### Key composition pattern

```typescript
import { Effect, Option, pipe } from "effect"
import { Gram, StandardGraph, fold, findFirst } from "@relateby/pattern"

// Parse, classify, query — all composed with pipe, one Effect.runPromise at the edge
const result = await pipe(
  Gram.parse(gramString),
  Effect.map(StandardGraph.fromPatterns),
  Effect.flatMap((graph) =>
    Effect.fromOption(graph.node("alice"), () => new GramParseError({ input: gramString, cause: "node not found" }))
  ),
  Effect.runPromise
)

// Fold over a pattern tree — synchronous, no WASM, uses pipe
const allIdentities = pipe(
  somePattern,
  fold([] as string[], (acc, subject) => [...acc, subject.identity])
)

// findFirst returns Option — compose with Option utilities
const maybePerson = pipe(
  somePattern,
  findFirst(s => s.labels.has("Person"))
)
const personOrUndefined = Option.getOrUndefined(maybePerson)
```

---

## Phase 1: Rust Codec JSON Surface

**Goal**: Add `gram_parse_to_json` and `gram_stringify_from_json` to the gram-codec crate. Keep all existing bindings working in parallel (no cutover yet).

### Tasks

**P1-1**: Verify `crates/gram-codec` standalone WASM compilation
```bash
cargo build -p gram-codec --target wasm32-unknown-unknown
```
Expected: compiles without `pattern-core`. If it fails (because of `parse_gram` return type), the new JSON functions still work because they return `String`.

**P1-2**: Add `crates/gram-codec/src/json.rs`
- Implement `gram_parse_to_json(input: &str) -> Result<String, String>`: call existing `parse_gram()`, serialize result with `serde_json`.
- Implement `gram_stringify_from_json(input: &str) -> Result<String, String>`: deserialize JSON to `Vec<Pattern<Subject>>`, call existing `to_gram()`.
- Add `serde_json` to gram-codec `Cargo.toml` (already present as a dependency? verify).

**P1-3**: Verify JSON output matches gram-hs format
- Compare `gram_parse_to_json` output against `data-model.md` JSON interchange format.
- Verify `"subject"` key (not `"value"`), label arrays, property encoding, and all Value variant discriminants.
- Cross-check against existing `parse_patterns_as_dicts` Python output.

**P1-4**: Expose via PyO3 (`crates/gram-codec/src/python.rs`)
- Add `gram_parse_to_json(input: &str) -> PyResult<String>`
- Add `gram_stringify_from_json(input: &str) -> PyResult<String>`

**P1-5**: Expose via WASM (`crates/pattern-wasm/src/gram.rs`)
- Add `gram_parse_to_json(input: &str) -> Result<String, JsValue>`
- Add `gram_stringify_from_json(input: &str) -> Result<String, JsValue>`
- Keep existing `Gram.parse` / `Gram.stringify` working (parallel operation)

**P1-6**: Add Rust tests for JSON round-trip
```rust
// Round-trip: parse gram → JSON → stringify → parse again → compare
```

---

## Phase 2: Native TypeScript

**Goal**: Implement all types and operations natively in TypeScript using effect-ts. The new implementation runs alongside the existing WASM-backed one; both are tested against the same inputs.

See `contracts/typescript-public-api.md` for the full type signatures and usage examples. See the "TypeScript Architectural Design" section above for why each effect-ts tool is used.

### Tasks

**P2-1**: Promote `effect` from optional peer dep to required in `package.json`
- Update `peerDependencies` and `peerDependenciesMeta` in `typescript/@relateby/pattern/package.json`

**P2-2**: Implement `src/value.ts`
- 11 `Data.Case` interfaces (one per variant), each with `readonly _tag: "VariantName"`
- `Value` union type alias
- `Value` constructor namespace using `Data.tagged<VariantInterface>("VariantName")` for each variant
- `ValueSchema`: `Schema.Union` of `Schema.TaggedStruct` — one branch per variant
- `ArrayVal` and `MapVal` entries use `Schema.suspend(() => ValueSchema)` to handle nesting
- Verify all 11 variants from `data-model.md` are present (5 primitive + 6 structured)

**P2-3**: Implement `src/subject.ts`
- `Subject extends Data.Class<{ identity, labels: ReadonlySet<string>, properties: ReadonlyMap<string, Value> }>`
- `Subject.fromId(identity)` static constructor
- `withLabel(label): Subject` — returns `new Subject({ ...this, labels: new Set([...this.labels, label]) })`
- `withProperty(name, value): Subject` — same immutable builder pattern

**P2-4**: Implement `src/pattern.ts`
- `Pattern<V> extends Data.Class<{ value: V, elements: ReadonlyArray<Pattern<V>> }>`
- `Pattern.point<V>(value)` and `Pattern.of<V>(value)` static constructors
- `isAtomic`, `length`, `size`, `depth` as getters
- No operations on the class — those live in `ops.ts`

**P2-5**: Implement `src/ops.ts`
- All operations are **standalone curried functions** — not class methods — so they compose with `pipe`
- `map<V, U>(fn) => (p) => Pattern<U>`: transforms values recursively, pre-order
- `fold<V, R>(init, fn) => (p) => R`: accumulates via `fn(init, p.value)` then recurse into elements
- `filter<V>(pred) => (p) => ReadonlyArray<Pattern<V>>`: collects matching subtrees, pre-order
- `findFirst<V>(pred) => (p) => Option.Option<V>`: returns `Option.some(v)` on first match, `Option.none()` if absent; uses `Option.orElse` to short-circuit across elements
- Comonad:
  - `extend<V, U>(fn) => (p) => Pattern<U>`: `new Pattern({ value: fn(p), elements: p.elements.map(extend(fn)) })`
  - `extract<V>(p) => V`: `p.value`
  - `duplicate<V>(p) => Pattern<Pattern<V>>`: `new Pattern({ value: p, elements: p.elements.map(duplicate) })`

**P2-6**: Implement `src/schema.ts`
- `RawSubjectSchema` using `Schema.Struct({ identity: Schema.String, labels: Schema.Array(Schema.String), properties: Schema.Record(...) })`
- `RawPatternSchema` using `Schema.Struct({ subject: RawSubjectSchema, elements: Schema.Array(Schema.suspend(...)) })`
  - Note: the field is `subject`, not `value` — see `data-model.md` JSON interchange format
  - `Schema.suspend((): Schema.Schema<RawPattern> => RawPatternSchema)` is required for the recursive `elements` field
- `decodePayload = Schema.decodeUnknownSync(Schema.Array(RawPatternSchema))`
- `patternFromRaw(raw: RawPattern): Pattern<Subject>` — pure recursive constructor

**P2-7**: Implement `src/errors.ts`
- `GramParseError extends Data.TaggedError("GramParseError")<{ readonly input: string; readonly cause: unknown }>`
- `Data.TaggedError` gives it a `_tag` field, structured fields, and proper `Error` prototype chain

**P2-8**: Update `src/gram.ts`
- `Gram.parse(input)` returns `Effect.Effect<ReadonlyArray<Pattern<Subject>>, GramParseError>`
- Implementation uses `pipe` + two `Effect` steps:
  1. `Effect.tryPromise({ try: async () => JSON.parse(wasm.gram_parse_to_json(input)), catch: cause => new GramParseError(...) })`
  2. `Effect.flatMap(raw => Effect.try({ try: () => decodePayload(raw).map(patternFromRaw), catch: cause => new GramParseError(...) }))`
- `Gram.stringify(patterns)` returns `Effect.Effect<string, GramParseError>` via `Effect.tryPromise`
- `Gram.validate(input)` returns `Effect.Effect<void, GramParseError>`
- Remove the old `async parse(): Promise<...>` implementation

**P2-9**: Implement `src/standard-graph.ts`
- `StandardGraph.fromPatterns(patterns)`: pure synchronous classification
  - Classification is by element count: 0→Node, 1→Annotation, 2+both-nodes→Relationship, valid-chain→Walk, else→Other
  - Walk validity: port the identity-chain predicate from gram-hs `GraphClassifier.hs`
- `StandardGraph.fromGram(input)`: `pipe(Gram.parse(input), Effect.map(StandardGraph.fromPatterns))`
- `node(id)` returns `Option.Option<Pattern<Subject>>` (not `T | undefined`)
- `relationship(id)` returns `Option.Option<...>`

**P2-10**: Update `src/index.ts`
- Export new native types alongside existing during the parallel phase
- Do not yet remove WASM-backed exports (that happens in Phase 4)

**P2-11**: Add property-based law tests (`fast-check`)
- Functor laws for `map`: `map(id)(p)` equals `p` (using `Equal.equals`); `map(f∘g)(p)` equals `map(f)(map(g)(p))`
- Foldable: construct a known tree and verify the pre-order traversal sequence matches expected order
- Comonad laws: `extract(extend(f)(p)) == f(p)`; `extend(extract)(p)` equals `p`; associativity

**P2-12**: Validate against existing test suite
- Run `vitest` with both the old WASM-backed implementation and the new native implementation against the same gram inputs
- Both must produce `Equal.equals`-equivalent results

---

## Phase 3: Native Python

**Goal**: Implement all types and operations natively in Python using dataclasses. Same parallel strategy as Phase 2.

### Tasks

**P3-1**: Implement `_value.py`
- 11 `@dataclass` classes (StringVal through MeasurementVal)
- `Value` union type alias
- `value_from_dict(d)` decoder function for JSON interchange format

**P3-2**: Implement `_subject.py`
- `Subject` `@dataclass` with `identity`, `labels`, `properties`
- `from_id`, `with_label`, `with_property` builders

**P3-3**: Implement `_pattern.py`
- `Pattern` generic `@dataclass` with `value`, `elements`
- `point`, `of` class methods
- `is_atomic`, `length`, `size`, `depth` properties
- `map`, `fold`, `filter`, `find_first`, `extend`, `extract`, `duplicate` methods

**P3-4**: Implement `_decode.py`
- `pattern_from_dict(d: dict) -> Pattern[Subject]` — recursive decoder
- Handles all 10 Value variants

**P3-5**: Implement `_standard_graph.py`
- `StandardGraph` with 5-class classification
- `from_patterns`, `from_gram` constructors

**P3-6**: Update `python/relateby/relateby/pattern/__init__.py`
- Import from new native modules
- Keep existing PyO3-backed classes available under aliased names during transition
- `StandardGraph.from_gram` uses `gram_parse_to_json` + `pattern_from_dict`

**P3-7**: Update type stubs (`__init__.pyi`)
- Reflect all new native types and operations

**P3-8**: Add property-based tests (hypothesis)
- Functor laws for `map`
- Foldable pre-order traversal
- Comonad laws for `extend`/`extract`/`duplicate`

**P3-9**: Validate against existing pytest suite
- Run full test suite; all existing tests must pass

---

## Phase 4: TypeScript Cutover

**Goal**: Remove WASM-backed types from the TypeScript public API. Delete `convert.rs`.

**Prerequisites**: Phase 2 complete and all tests passing.

### Tasks

**P4-1**: Remove `WasmPattern`, `WasmSubject`, `WasmValue`, `WasmStandardGraph` from `index.ts`
**P4-2**: Remove `Gram.parseOne` from the public API
**P4-3**: Delete `crates/pattern-wasm/src/convert.rs`
**P4-4**: Delete `crates/pattern-wasm/src/standard_graph.rs`
**P4-5**: Slim `crates/pattern-wasm/src/gram.rs` — remove old `parse`/`stringify`; keep only `gram_parse_to_json`, `gram_stringify_from_json`, `gram_validate`
**P4-6**: Update `crates/pattern-wasm/src/lib.rs` — remove re-exports of pattern-core types
**P4-7**: Rebuild WASM and verify binary size reduction ≥40%
**P4-8**: Run full TypeScript test suite; fix any regressions

---

## Phase 5: Python Cutover

**Goal**: Remove PyO3-backed classes from the Python public API. Slim `python.rs` to ~50 lines.

**Prerequisites**: Phase 3 complete and all tests passing.

### Tasks

**P5-1**: Remove PyO3-backed classes from `python/relateby/relateby/pattern/__init__.py`
**P5-2**: Slim `crates/pattern-core/src/python.rs` to expose only codec functions (~50 lines)
  - Remove: `PyPattern`, `PySubject`, `PyValue`, `PyStandardGraph`, `PyValidationRules`, `PyStructureAnalysis`, `PySubjectBuilder`, `PyValidationError`
  - Keep: `gram_parse_to_json`, `gram_stringify_from_json`, `gram_validate` (or delegate entirely to gram-codec's python.rs)
**P5-3**: Update `python/relateby/relateby/gram/__init__.py` — use `gram_parse_to_json` path
**P5-4**: Run full pytest suite; fix any regressions

---

## Phase 6: Polish and Verification

### Tasks

**P6-1**: Update examples in `examples/`
- TypeScript examples: use native Pattern, show `pipe`, `Effect.runPromise`, `Option`
- Python examples: use dataclass Pattern, show `fold` with lambda

**P6-2**: Run performance benchmarks
- Verify SC-002 (TypeScript fold ≥5× faster)
- Verify SC-003 (Python fold ≥10× faster)
- Verify SC-004 (WASM binary ≥40% smaller)
- Document results in `specs/039-native-bindings/benchmarks.md`

**P6-3**: Update `docs/python-usage.md` and `typescript/@relateby/pattern/README.md`

**P6-4**: Code quality checks (Constitution Principle III)
```bash
cargo fmt --all
cargo clippy --workspace -- -D warnings
cargo test --workspace
./scripts/ci-local.sh
```

**P6-5**: Verify gram-hs behavioral equivalence
- Run property tests cross-checking native implementations against gram-hs reference outputs
- Document any intentional deviations

**P6-6**: Final: verify `@relateby/gram` package still works (depends on `@relateby/pattern`)

---

## Risk Register

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| JSON interchange format doesn't match `parse_patterns_as_dicts` output exactly | Medium | High | P1-3 verification step; fix in Rust before proceeding |
| `crates/gram-codec` doesn't compile standalone for WASM | Low | High | P1-1 verification; new JSON functions use `String` return type which has no `pattern-core` dep |
| `fast-check` property tests reveal fold order bug | Medium | High | Run property tests before cutover (Phase 2 Step 11) |
| TypeScript `Data.Class` generics cause unexpected type narrowing issues | Low | Medium | Prototype `Pattern<V>` with a simple test before building all ops |
| Walk classification logic is more complex than expected | Medium | Medium | Cross-check gram-hs `GraphClassifier.hs` directly; property-test classification against reference |
| Python recursion depth limit on large patterns | Low | Low | Document behavior; `fold`/`map` can be made iterative if needed |

---

## Verification Checklist (Definition of Done)

- [ ] All existing TypeScript vitest tests pass
- [ ] All existing Python pytest tests pass
- [ ] Functor, foldable, and comonad law property tests pass (both TS and Python)
- [ ] `Equal.equals(p1, p2)` works structurally in TypeScript
- [ ] Python `Pattern.__eq__` works structurally (dataclass default)
- [ ] `Pattern` objects are directly readable in debuggers (no opaque handles)
- [ ] WASM binary size ≥40% smaller than pre-migration baseline
- [ ] TypeScript `fold` ≥5× faster than WASM-bridge baseline
- [ ] Python `fold` ≥10× faster than PyO3 round-trip baseline
- [ ] `convert.rs` deleted
- [ ] `python.rs` reduced to ≤50 lines
- [ ] All 10 Value variants correctly encode/decode through JSON interchange
- [ ] `StandardGraph` correctly classifies all 5 element classes
- [ ] Examples updated and tested
- [ ] `cargo fmt`, `cargo clippy`, `cargo test --workspace` all clean
- [ ] `./scripts/ci-local.sh` passes
