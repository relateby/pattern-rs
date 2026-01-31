# Tasks: WASM Feature Parity with Python and Pattern&lt;V&gt; TypeScript Generics

**Input**: Design documents from `/specs/027-wasm-pattern-typescript-parity/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Not explicitly requested in the feature specification; no dedicated test tasks. Independent test criteria per user story are used as checkpoints.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g. US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Repository root**: paths relative to repo root (e.g. `crates/pattern-core/src/wasm.rs`)
- **pattern-core**: `crates/pattern-core/` (Cargo.toml, src/, typescript/, tests/)
- **Examples**: `examples/pattern-core-wasm/` or `examples/wasm-js/`

---

## Phase 1: Setup (Shared Infrastructure) ✅ COMPLETE

**Purpose**: Project initialization and WASM/TypeScript structure for pattern-core

- [x] T001 Add wasm feature and target cfg dependencies (wasm-bindgen, js-sys) to crates/pattern-core/Cargo.toml
- [x] T002 Create crates/pattern-core/src/wasm.rs with module skeleton and #[cfg(feature = "wasm")] conditional compile
- [x] T003 [P] Create crates/pattern-core/typescript/ directory for hand-written .d.ts
- [x] T004 Update crates/pattern-core/src/lib.rs to conditionally re-export wasm module when feature "wasm" is enabled

---

## Phase 2: Foundational (Blocking Prerequisites) ✅ COMPLETE

**Purpose**: Core WASM boundary infrastructure that MUST be complete before ANY user story implementation

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 Implement JsValue ↔ Rust value conversion helpers (primitives, Option) in crates/pattern-core/src/wasm.rs
- [x] T006 Implement Either-like return shape (Result → { _tag, right/left }) for fallible ops in crates/pattern-core/src/wasm.rs

**Checkpoint**: Foundation ready — user story implementation can now begin

---

## Phase 3: User Story 1 - Construct Patterns in Browser/Node (Priority: P1) ✅ COMPLETE

**Goal**: Developers can create Pattern and Subject instances from JavaScript/TypeScript (atomic, nested, Pattern with Subject) and access value/elements; behavior matches Python bindings.

**Independent Test**: Load WASM module, create atomic pattern and nested Pattern with Subject, verify structure and value/elements access (see spec acceptance scenarios 1–4).

### Implementation for User Story 1

- [x] T007 [P] [US1] Expose Pattern constructors (point, of, pattern, fromValues) in crates/pattern-core/src/wasm.rs
- [x] T008 [P] [US1] Expose Pattern accessors (value, elements) in crates/pattern-core/src/wasm.rs
- [x] T009 [P] [US1] Expose Value factories (string, int, decimal, boolean, symbol, array, map, range, measurement) in crates/pattern-core/src/wasm.rs
- [x] T010 [US1] Expose Subject constructor and accessors (identity, labels, properties) in crates/pattern-core/src/wasm.rs
- [x] T011 [US1] Wire Pattern constructors to accept JsValue/Subject from JS and round-trip in crates/pattern-core/src/wasm.rs

**Checkpoint**: User Story 1 fully functional — construct and access patterns from JS/TS; testable independently

### Phase 3 Implementation Notes

**Critical Design Corrections Applied** (2026-01-31):
- WasmPattern changed from `Pattern<Subject>` to `Pattern<JsValue>` to match Python's generic design
- `point()` and `of()` now accept any `JsValue` (not just Subject) - matches Python's `PyAny` approach
- `of()` is now a true alias for `point()` (just delegates) - matches Python exactly
- `fromValues()` correctly returns array of atomic patterns (not single nested pattern)
- `elements` property accessor added (returns JS array, not just `getElement(index)`)
- `value` property returns `JsValue` (not forced to Subject type)
- JavaScript exports renamed: `WasmPattern` → `Pattern`, `WasmSubject` → `Subject`, `ValueFactory` → `Value`

See `specs/027-wasm-pattern-typescript-parity/phase3-issues.md` and `phase3-fixes-applied.md` for detailed analysis and changes.

---

## Phase 4: User Story 2 - Perform Pattern Operations from WASM (Priority: P2) ✅ COMPLETE

**Goal**: Developers can run inspection, query, transformation (map, fold, para), combination, comonad, and validate/analyzeStructure from JS/TS with behavior equivalent to Rust and Python; fallible ops return Either-like.

**Independent Test**: Create pattern, apply map/filter/combine/para/depth/size, call validate and assert Either-like return; compare results to Python for same logical input (see spec acceptance scenarios 1–5).

### Implementation for User Story 2

- [x] T012 [P] [US2] Expose Pattern inspection methods (length, size, depth, isAtomic, values) in crates/pattern-core/src/wasm.rs
- [x] T013 [P] [US2] Expose Pattern query methods (anyValue, allValues, filter, findFirst, matches, contains) with JS callbacks in crates/pattern-core/src/wasm.rs
- [x] T014 [US2] Expose Pattern transformation methods (map, fold, para) with JS callbacks in crates/pattern-core/src/wasm.rs
- [x] T015 [US2] Expose Pattern combine and comonad methods (combine, extract, extend, depthAt, sizeAt, indicesAt) in crates/pattern-core/src/wasm.rs
- [x] T016 [US2] Expose validate returning Either-like and analyzeStructure in crates/pattern-core/src/wasm.rs
- [x] T017 [US2] Expose ValidationRules, StructureAnalysis, and ValidationError shape in crates/pattern-core/src/wasm.rs

**Checkpoint**: User Story 2 fully functional — all pattern operations callable from JS/TS; validate returns Either-like

### Phase 4 Implementation Notes

**Completed** (2026-01-31):

All pattern operations from the Rust core API have been successfully exposed to JavaScript/TypeScript:

- **Inspection Methods**: `size()`, `depth()`, `values()` (in addition to existing `length()`, `isAtomic()`)
- **Query Methods**: `anyValue()`, `allValues()`, `filter()`, `findFirst()`, `matches()`, `contains()` - all support JavaScript callback functions with proper error handling
- **Transformation Methods**: `map()`, `fold()`, `para()` - full support for JavaScript callbacks, including paramorphism (bottom-up fold)
- **Combination**: `combine()` with custom combiner function for JavaScript value types
- **Comonad Operations**: `extract()`, `extend()`, `depthAt()`, `sizeAt()`, `indicesAt()`
- **Validation**: `validate()` returns Either-like `{ _tag: 'Right'/'Left', right/left: ... }` compatible with effect-ts
- **Analysis**: `analyzeStructure()` returns `StructureAnalysis` with depth distribution, element counts, and summary

All methods properly handle:
- JavaScript callbacks with error propagation
- Either-like return values for fallible operations (no throwing)
- Pre-order traversal and short-circuit evaluation where applicable
- Type conversions between JavaScript and Rust at the WASM boundary

Build verification:
- `cargo build --package pattern-core --features wasm` ✅
- `cargo fmt --all` ✅
- `cargo clippy --package pattern-core --features wasm -- -D warnings` ✅

---

## Phase 5: User Story 3 - TypeScript Pattern&lt;V&gt; Generics and Type Safety (Priority: P3)

**Goal**: TypeScript definitions (generic Pattern&lt;V&gt;, Subject, Value, Either, ValidationError) cover full public WASM API; type checker and IDE give correct inference and autocomplete.

**Independent Test**: Write TypeScript using Pattern&lt;Subject&gt; and main operations; run tsc --noEmit; verify zero false positives and correct parameter/return types (see spec acceptance scenarios 1–4).

### Implementation for User Story 3

- [ ] T018 [P] [US3] Add Pattern&lt;V&gt; interface and static constructors (point, of, pattern, fromValues) to crates/pattern-core/typescript/pattern_core.d.ts
- [ ] T019 [P] [US3] Add Subject, Value, Symbol types and Value factories/extractors to crates/pattern-core/typescript/pattern_core.d.ts
- [ ] T020 [US3] Add ValidationRules, StructureAnalysis, Either, ValidationError and validate return type to crates/pattern-core/typescript/pattern_core.d.ts
- [ ] T021 [US3] Verify TypeScript definitions with tsc --noEmit on a sample consumer (e.g. crates/pattern-core/typescript/consumer_sample.ts or examples/pattern-core-wasm)

**Checkpoint**: User Story 3 complete — .d.ts covers full API; type check passes on sample consumer

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Examples, documentation, code quality, and final verification

### Documentation & Examples

- [ ] T022 [P] Add or update minimal WASM + JS/TS example (examples/pattern-core-wasm or examples/wasm-js) demonstrating load, construct, map, para, validate
- [ ] T023 [P] Document effect-ts usage and Either-like return shape in specs/027-wasm-pattern-typescript-parity/quickstart.md and package README per contracts/wasm-api.md

### Code Quality Checks (REQUIRED)

- [ ] T024 Run cargo fmt --all to ensure consistent code formatting
- [ ] T025 Run cargo clippy --workspace -- -D warnings to check for issues
- [ ] T026 Run cargo test --workspace; run wasm-pack build --target web --features wasm in crates/pattern-core and verify build
- [ ] T027 Run quickstart.md validation (build, load module, minimal workflow from quickstart)

### Final Verification

- [ ] T028 Update CHANGELOG.md or feature README with 027 completion status; ensure all spec acceptance criteria from spec.md are met

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion — BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational — no dependency on US2/US3
- **User Story 2 (Phase 4)**: Depends on US1 (operations need Pattern/Subject/Value)
- **User Story 3 (Phase 5)**: Can start after US1; full .d.ts best completed after US2 (covers full API)
- **Polish (Phase 6)**: Depends on all desired user stories complete

### User Story Dependencies

- **User Story 1 (P1)**: After Foundational — constructors and accessors only
- **User Story 2 (P2)**: After US1 — operations require constructible patterns
- **User Story 3 (P3)**: After US1; ideally after US2 so .d.ts matches full API

### Within Each User Story

- US1: Value factories and Subject before wiring Pattern to Subject (T009, T010 before T011); constructors and accessors can be parallel (T007, T008)
- US2: Inspection/query can be parallel (T012, T013); transformation (T014) and combine/comonad (T015) can proceed after; validate/analyze (T016, T017) after
- US3: Pattern interface and Subject/Value types can be parallel (T018, T019); T020 ties types together; T021 verification last

### Parallel Opportunities

- Phase 1: T003 [P] (create typescript/) can run in parallel with T001, T002
- Phase 3: T007, T008, T009 [P] can run in parallel after Phase 2
- Phase 4: T012, T013 [P] can run in parallel
- Phase 5: T018, T019 [P] can run in parallel
- Phase 6: T022, T023 [P] can run in parallel

---

## Parallel Example: User Story 1

```bash
# After Phase 2 complete, launch US1 constructor/accessor tasks together:
Task T007: "Expose Pattern constructors (point, of, pattern, fromValues) in crates/pattern-core/src/wasm.rs"
Task T008: "Expose Pattern accessors (value, elements) in crates/pattern-core/src/wasm.rs"
Task T009: "Expose Value factories (...) in crates/pattern-core/src/wasm.rs"
# Then T010 (Subject), then T011 (wire Pattern to Subject)
```

---

## Parallel Example: User Story 2

```bash
# After US1 complete, launch US2 inspection and query together:
Task T012: "Expose Pattern inspection methods (...) in crates/pattern-core/src/wasm.rs"
Task T013: "Expose Pattern query methods (...) in crates/pattern-core/src/wasm.rs"
# Then T014 (map, fold, para), T015 (combine, comonad), T016–T017 (validate, types)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup  
2. Complete Phase 2: Foundational  
3. Complete Phase 3: User Story 1  
4. **STOP and VALIDATE**: Load WASM, create atomic and nested Pattern with Subject, verify value/elements  
5. Demo/ship MVP (construct + access only)

### Incremental Delivery

1. Setup + Foundational → foundation ready  
2. Add User Story 1 → test independently → MVP (construct + access)  
3. Add User Story 2 → test independently → full operations (map, filter, para, validate, etc.)  
4. Add User Story 3 → test independently → TypeScript generics and type safety  
5. Polish → examples, docs, fmt/clippy/test, quickstart validation  

### Parallel Team Strategy

- Phase 1–2: Single stream (setup + foundational)  
- After Phase 2: US1 must complete before US2  
- After US1: US2 and US3 can be split (e.g. one dev on US2, one on US3 types); US3 T018–T019 can run in parallel with early US2 tasks  

---

## Notes

- [P] tasks = different files or independent subtrees; no dependencies on incomplete tasks in same phase  
- [USn] label maps task to spec user story for traceability  
- Each user story is independently testable per spec "Independent Test"  
- Commit after each task or logical group  
- Stop at any checkpoint to validate that story independently  
- File paths are relative to repository root; use crates/pattern-core/ for all pattern-core changes  
