# Tasks: Unified Gram WASM Package

**Input**: Design documents from `/specs/028-unified-gram-wasm/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not explicitly requested in the feature specification; no dedicated test tasks. Validation via quickstart and CI (Polish phase).

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: User story (US1, US2, US3) for story-phase tasks only
- Include exact file paths in descriptions

## Path Conventions

- **Crate**: `crates/pattern-wasm/` (new crate in existing workspace)
- **Source**: `crates/pattern-wasm/src/`
- **TypeScript**: `crates/pattern-wasm/typescript/`

---

## Phase 1: Setup (Shared Infrastructure) ✅ COMPLETE

**Purpose**: Create the pattern-wasm crate and minimal structure so it compiles and is part of the workspace.

- [x] T001 Create crates/pattern-wasm/Cargo.toml with package name `pattern-wasm`, `crate-type = ["cdylib"]`, and dependencies: pattern-core (path `../pattern-core`, features `["wasm"]`), gram-codec (path `../gram-codec`, features `["wasm"]`), wasm-bindgen 0.2, js-sys 0.3
- [x] T002 Create crates/pattern-wasm/src/lib.rs with `#![allow(clippy::all)]`, wasm_bindgen prelude, `mod convert; mod gram;`, and re-exports of Pattern, Subject, Value from pattern_core (with wasm feature)
- [x] T003 Create crates/pattern-wasm/src/gram.rs with a stub Gram namespace (empty or placeholder functions) so the crate compiles
- [x] T004 Create crates/pattern-wasm/src/convert.rs with stub module (e.g. placeholder fn) so the crate compiles

**Status**: All tasks complete. Crate builds successfully for both wasm32-unknown-unknown and native targets.

---

## Phase 2: Foundational (Blocking Prerequisites) ✅ COMPLETE

**Purpose**: Conversion layer between Rust Pattern&lt;Subject&gt; and JS Pattern/Subject. Required before Gram.stringify and Gram.parse can be implemented.

**CRITICAL**: No user story implementation can begin until this phase is complete.

- [x] T005 Implement Rust Pattern&lt;Subject&gt; to JS Pattern/Subject conversion in crates/pattern-wasm/src/convert.rs (function that takes Rust pattern from gram_codec::parse_gram_notation and builds equivalent JS Pattern/Subject using pattern_core types or equivalent JS shape)
- [x] T006 Implement JS Pattern/Subject to Rust Pattern&lt;Subject&gt; conversion in crates/pattern-wasm/src/convert.rs (function that takes JS pattern and returns Rust Pattern&lt;Subject&gt; for to_gram_pattern)

**Checkpoint**: ✅ Conversion layer ready; Gram.stringify and Gram.parse can be implemented.

**Status**: Implemented using **Approach A** (proper WasmSubject instances):

1. Added to `pattern-core/src/wasm.rs`:
   - `WasmSubject::from_subject(Subject)` - create WasmSubject from Rust Subject
   - `WasmSubject::into_subject()` - extract Rust Subject from WasmSubject
   - `WasmSubject::as_subject()` - borrow inner Subject
   - `WasmPattern::from_pattern(Pattern<JsValue>)` - create WasmPattern
   - `WasmPattern::into_pattern()` - extract inner Pattern
   - `WasmPattern::as_pattern()` - borrow inner Pattern

2. Implemented in `pattern-wasm/src/convert.rs`:
   - `rust_pattern_to_wasm()` - converts Pattern&lt;Subject&gt; to WasmPattern with real WasmSubject instances
   - `wasm_pattern_to_rust()` - extracts Pattern&lt;Subject&gt; from WasmPattern

This ensures `pattern.value instanceof Subject` is true for both parsed and manually constructed patterns.

---

## Phase 3: User Story 1 - Single package for pattern and gram (Priority: P1) — MVP ✅ COMPLETE

**Goal**: One dependency provides Pattern, Subject, Value, and gram serialization (stringify/parse). Round-trip works from a single import.

**Independent Test**: Import from one package, construct a Pattern&lt;Subject&gt;, call Gram.stringify, then Gram.parse, assert result is equivalent to original.

- [x] T007 [US1] Implement Gram::stringify (single pattern) in crates/pattern-wasm/src/gram.rs using convert (JS→Rust) and gram_codec::to_gram_pattern
- [x] T008 [US1] Implement Gram::parse in crates/pattern-wasm/src/gram.rs using gram_codec::parse_gram_notation and convert (Rust→JS), returning array of JS Pattern&lt;Subject&gt;
- [x] T009 [US1] Ensure single entry point in crates/pattern-wasm/src/lib.rs: export Pattern, Subject, Value, and Gram from one module so consumers get `import { Pattern, Subject, Value, Gram } from '…'`

**Status**: ✅ Phase 3 Complete (2026-01-31)

**Implementation Notes**:
1. Implemented in `pattern-wasm/src/gram.rs`:
   - `Gram::stringify()` - accepts &WasmPattern, converts to Rust Pattern&lt;Subject&gt;, serializes via gram_codec::to_gram_pattern
   - `Gram::parse()` - parses gram notation via gram_codec::parse_gram, converts each Pattern&lt;Subject&gt; to WasmPattern, returns js_sys::Array
   
2. Entry point in `pattern-wasm/src/lib.rs` already exports Pattern, Subject, Value, and Gram

3. Note: Initially attempted array overload for stringify but encountered wasm_bindgen JsCast trait issues across crate boundaries. Simplified to single pattern only. Array serialization can be handled in TypeScript wrapper by calling stringify on each pattern and joining results.

**Checkpoint**: User Story 1 is complete; round-trip (build pattern → stringify → parse → equivalent) works from one import.

---

## Phase 4: User Story 2 - Serialize and parse feel like JSON (Priority: P2)

**Goal**: parseOne available; empty/whitespace input returns [] and null; invalid input reports a clear error without exposing internal AST.

**Independent Test**: Call Gram.parseOne on valid string (get first pattern or null); on empty/whitespace get null; on invalid string get clear error message, not parser internals.

- [ ] T010 [US2] Implement Gram::parseOne in crates/pattern-wasm/src/gram.rs (parse and return first pattern or null)
- [ ] T011 [US2] Implement empty and whitespace-only parse behavior in crates/pattern-wasm/src/gram.rs: parse returns [], parseOne returns null; no throw
- [ ] T012 [US2] Implement parse error handling in crates/pattern-wasm/src/gram.rs so invalid gram notation produces a clear error (e.g. thrown Error with message) and does not expose internal parser or AST types

**Checkpoint**: User Story 2 is complete; parseOne, empty input, and error behavior match contract.

---

## Phase 5: User Story 3 - Convert generic patterns to serializable form (Priority: P3)

**Goal**: Subject.fromValue(value, options?) implements conventional mapping; Gram.from(pattern, options?) is implemented as pattern.map(v => Subject.fromValue(v, options)).

**Independent Test**: Build Pattern of primitives (e.g. numbers), call Gram.from with default or minimal options, then Gram.stringify; parse back and assert round-trip. Optionally use pattern.map(Subject.fromValue) explicitly.

- [ ] T013 [US3] Implement Subject.fromValue(value, options?) in crates/pattern-wasm/src/convert.rs (or extend Subject surface) with conventional mapping for string, number, boolean, object, Subject passthrough; options: label, valueProperty, identity (e.g. (value, index) => string)
- [ ] T014 [US3] Implement Gram::from(pattern, options?) in crates/pattern-wasm/src/gram.rs as pattern.map(v => Subject.fromValue(v, options)), passing options through to Subject.fromValue

**Checkpoint**: User Story 3 is complete; conventional conversion and Gram.from work per data-model.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: TypeScript definitions, quickstart validation, and code quality.

### Documentation & Types

- [ ] T015 [P] Add unified TypeScript definitions in crates/pattern-wasm/typescript/gram.d.ts (Pattern, Subject, Value, Gram namespace, FromOptions/FromValueOptions, single-entry export shape per contracts/typescript-gram.md)

### Validation & Code Quality

- [ ] T016 Run quickstart validation per specs/028-unified-gram-wasm/quickstart.md (build pattern-wasm, import Pattern/Subject/Value/Gram, build pattern, stringify, parse, assert equivalence)
- [ ] T017 Run `cargo fmt --all` and fix formatting in crates/pattern-wasm/
- [ ] T018 Run `cargo clippy --workspace -- -D warnings` and fix any issues in crates/pattern-wasm/
- [ ] T019 Run scripts/ci-local.sh (or equivalent) and fix any failures
- [ ] T020 Run `cargo test --workspace` and fix any test failures; ensure pattern-wasm builds for target wasm32-unknown-unknown

### Final Verification

- [ ] T021 Verify all acceptance scenarios from spec.md (US1–US3 and edge cases) are satisfied and document any gaps in specs/028-unified-gram-wasm/

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies — start immediately.
- **Phase 2 (Foundational)**: Depends on Phase 1 — blocks all user stories.
- **Phase 3 (US1)**: Depends on Phase 2 — MVP (single package, stringify, parse).
- **Phase 4 (US2)**: Depends on Phase 3 — parseOne, empty input, error behavior.
- **Phase 5 (US3)**: Depends on Phase 3 — Subject.fromValue, Gram.from (independent of US2).
- **Phase 6 (Polish)**: Depends on Phase 3–5 — types, quickstart, fmt, clippy, CI, tests.

### User Story Dependencies

- **US1 (P1)**: After Foundational; no other story required. Delivers single import + stringify + parse.
- **US2 (P2)**: After US1; adds parseOne and error/empty behavior.
- **US3 (P3)**: After US1; adds Subject.fromValue and Gram.from (no US2 dependency).

### Within Each User Story

- Phase 3: T007, T008 can be parallel after T005–T006; T009 after T007–T008.
- Phase 4: T010, T011, T012 can be parallel (all in gram.rs but distinct behaviors).
- Phase 5: T013 before T014 (Subject.fromValue before Gram.from).

### Parallel Opportunities

- T015 (TypeScript) can run in parallel with T016–T020 once Phase 5 is done.
- T017, T018, T019, T020 can be run in parallel after implementation (fixes may serialize).

---

## Parallel Example: User Story 1

```text
# After Phase 2 complete:
# Implement stringify and parse (T007, T008) — can parallelize if two owners:
T007: Gram::stringify in crates/pattern-wasm/src/gram.rs
T008: Gram::parse in crates/pattern-wasm/src/gram.rs
# Then T009: single entry in lib.rs
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (T001–T004).
2. Complete Phase 2: Foundational (T005–T006).
3. Complete Phase 3: User Story 1 (T007–T009).
4. **STOP and VALIDATE**: Run quickstart steps; confirm one import, stringify, parse, round-trip.
5. Optionally run Phase 6 T017–T020 for format/clippy/CI.

### Incremental Delivery

1. Setup + Foundational → conversion layer ready.
2. Add US1 → single package, stringify, parse → validate round-trip (MVP).
3. Add US2 → parseOne, empty input, clear errors → validate.
4. Add US3 → Subject.fromValue, Gram.from → validate conventional conversion.
5. Polish → TypeScript, quickstart, fmt, clippy, CI.

### Parallel Team Strategy

- Phase 1: Single owner (crate creation).
- Phase 2: Single owner (conversion layer).
- After Phase 2: One owner US1 (stringify/parse), another can start US2 or US3 prep (parseOne vs Subject.fromValue) once US1 is done or in parallel if gram.rs is split.

---

## Notes

- [P] tasks: different files or non-overlapping changes; no dependency on another incomplete task.
- [USn] label maps task to spec user story for traceability.
- Each user story phase is independently testable per spec “Independent Test”.
- Commit after each task or logical group.
- pattern-wasm must build with `cargo build -p pattern-wasm --target wasm32-unknown-unknown`.
- No test tasks were added; spec did not request TDD or explicit test-first tasks.
