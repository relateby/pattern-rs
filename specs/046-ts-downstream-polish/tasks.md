# Tasks: TypeScript Polish for Downstream Projects

**Input**: Design documents from `specs/046-ts-downstream-polish/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅

**Tests**: Test expansion is an explicit deliverable of US2 (not a separate concern — the tests *are* the feature for that story).

**Organization**: Tasks are grouped by user story. All three stories are fully independent and can be executed in any order or in parallel.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to

## Path Conventions

All paths are from the repository root.

---

## Phase 1: Setup (Baseline Verification)

**Purpose**: Confirm the test suite is green before making any changes.

- [ ] T001 Run `npm test` in `typescript/packages/pattern` and `typescript/packages/gram` to confirm baseline passes (no changes made in this phase)

---

## Phase 2: Foundational

**⚠️ No blocking foundational work required.** All three user stories touch different files and are mutually independent. Proceed directly to user story phases.

---

## Phase 3: User Story 1 — Accurate WASM Type Declarations (Priority: P1) 🎯 MVP

**Goal**: Replace the stale 405-line `wasm-types.d.ts` with accurate declarations that exactly mirror the actual wasm-pack-generated output.

**Independent Test**: After T002, run `tsc --noEmit` in `typescript/packages/pattern` — zero type errors. Read `wasm-types.d.ts` and confirm no legacy types (`NativePatternGraph`, `WasmGraphQuery`, free traversal/centrality functions, etc.) remain.

### Implementation for User Story 1

- [ ] T002 [US1] Replace entire contents of `typescript/packages/pattern/src/wasm-types.d.ts` with accurate fallback declarations matching `typescript/packages/pattern/wasm/pattern_wasm.d.ts`: declare `Gram` class (`parseToJson`, `stringifyFromJson`, `validate`, `free`, `[Symbol.dispose]`), `ParseResult` class (`identifiers`, `pattern_count`, `free`, `[Symbol.dispose]`), and free functions (`parse_gram`, `parse_to_ast`, `round_trip`, `validate_gram`, `version`) — for both the `"../wasm/pattern_wasm.js"` and `"../wasm-node/pattern_wasm.js"` module blocks; remove all legacy interface types and declarations

**Checkpoint**: `wasm-types.d.ts` now matches the actual WASM adapter surface. Read it alongside `wasm/pattern_wasm.d.ts` — they should be structurally identical.

---

## Phase 4: User Story 2 — Meaningful Gram Package Tests (Priority: P2)

**Goal**: Expand `@relateby/gram` test coverage from 3 surface-existence assertions to 8+ behavioral test cases that exercise WASM-backed parse, stringify, and validate operations.

**Independent Test**: Run `npm test` in `typescript/packages/gram` — tests pass, output shows at least 8 passing assertions covering parse, stringify, validate, and init behaviors.

### Implementation for User Story 2

- [ ] T003 [US2] In `typescript/packages/gram/tests/public-api.test.ts`, add import of `Effect` from `"effect"` and `GramParseError` from `"@relateby/pattern"`; add a `describe("Gram.parse")` block with two tests: (1) parsing `"(a)"` resolves to a single-element array, and (2) parsing `"(a)-->(b)"` resolves to a pattern with two elements

- [ ] T004 [US2] In `typescript/packages/gram/tests/public-api.test.ts`, add a `describe("Gram.stringify")` block with one test: parsing `"(a:Person)"` then stringifying the result resolves to a non-empty string

- [ ] T005 [US2] In `typescript/packages/gram/tests/public-api.test.ts`, add a `describe("Gram.validate")` block with two tests: (1) validating `"(a)"` succeeds (Effect resolves), and (2) validating `"(unclosed"` fails with a `GramParseError`

- [ ] T006 [US2] In `typescript/packages/gram/tests/public-api.test.ts`, add a `describe("init()")` block with two tests: (1) calling `init()` resolves without error, and (2) calling `init()` a second time also resolves without error (idempotent)

**Checkpoint**: Run `npm test` in `typescript/packages/gram` — all new tests pass alongside the existing surface-existence test.

---

## Phase 5: User Story 3 — Documented Annotation Serialization Limitation (Priority: P3)

**Goal**: Add a clearly visible Known Limitation note to the Annotations section of `docs/gram-notation.md` so downstream consumers understand the current annotation round-trip behavior.

**Independent Test**: Read `docs/gram-notation.md` Annotations section — it contains a note explaining that annotation body content is stored as properties on the wrapping subject, that round-trips may not preserve original annotation syntax, and that configurable serialization format is planned future work.

### Implementation for User Story 3

- [ ] T007 [P] [US3] In `docs/gram-notation.md`, directly after the existing annotation example and explanation (after the paragraph beginning "Annotations are powerful for adding context"), add a **Known Limitation** blockquote noting: (1) annotation body content is currently stored as properties on the wrapping subject; (2) a parse-then-serialize round-trip may not reproduce the original annotation syntax; (3) future work will add configurable serialization — either inline unary notation (`@@a:L @k(37) (x)`) or property-map notation (`[a:L {k:37} | x]`), selectable at serialize time

**Checkpoint**: Read the Annotations section — the limitation note is clearly visible before the summary table, written in plain language accessible to downstream TypeScript consumers.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Verify all changes integrate cleanly and the full CI gate would pass.

- [ ] T008 [P] Run `npm test` in `typescript/packages/pattern` to confirm no regressions from `wasm-types.d.ts` replacement
- [ ] T009 [P] Run `npm test` in `typescript/packages/gram` to confirm all new tests pass
- [ ] T010 Run `tsc --noEmit` in `typescript/packages/pattern` to confirm no type errors after `wasm-types.d.ts` replacement

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — run immediately
- **Foundational (Phase 2)**: N/A — no blocking work
- **User Stories (Phases 3–5)**: All depend only on Setup (Phase 1); **all three can proceed in parallel**
- **Polish (Phase 6)**: Depends on all user story phases complete

### User Story Dependencies

- **US1 (P1)**: Independent — touches only `typescript/packages/pattern/src/wasm-types.d.ts`
- **US2 (P2)**: Independent — touches only `typescript/packages/gram/tests/public-api.test.ts`
- **US3 (P3)**: Independent — touches only `docs/gram-notation.md`

No cross-story dependencies exist.

### Within Each User Story

- US1: Single task (full file replacement) — no internal sequencing needed
- US2: T003 → T004 → T005 → T006 (all edit same file sequentially)
- US3: Single task — no internal sequencing needed

### Parallel Opportunities

- After T001 (baseline verified): T002, T003, T007 can all start simultaneously (different files)
- T008, T009, T010 (Polish phase) can all run in parallel

---

## Parallel Example: All Three Stories Together

```bash
# After T001 baseline verification, launch all stories in parallel:

# Terminal 1 — US1
Task: "Replace wasm-types.d.ts with accurate declarations (T002)"

# Terminal 2 — US2 (sequential within story)
Task: "Add Gram.parse tests (T003)"
Task: "Add Gram.stringify test (T004)"
Task: "Add Gram.validate tests (T005)"
Task: "Add init() tests (T006)"

# Terminal 3 — US3
Task: "Add annotation limitation note to gram-notation.md (T007)"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Baseline verification (T001)
2. Complete Phase 3: Replace `wasm-types.d.ts` (T002)
3. **STOP and VALIDATE**: `tsc --noEmit` passes, no legacy types in file
4. Ship — this alone unblocks any developer confused by the stale declarations

### Incremental Delivery

1. T001 → baseline green
2. T002 → US1 complete (mandatory fix delivered)
3. T003–T006 → US2 complete (test coverage delivered)
4. T007 → US3 complete (documentation delivered)
5. T008–T010 → all clean, ready to push

### Full Parallel Strategy

With a single developer: complete in order T001 → T002 → T003–T006 → T007 → T008–T010.  
With multiple developers: T002, T003–T006, and T007 can all proceed after T001 completes.

---

## Notes

- [P] tasks = different files, no shared state dependencies
- [Story] label maps each task to its user story for traceability
- US1 is a mandatory fix; US2 and US3 are high-value but non-blocking
- No Rust changes required for any task in this feature
- After all tasks complete, run `./scripts/ci-local.sh` for full CI validation before pushing
