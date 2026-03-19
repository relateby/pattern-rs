# Tasks: pato CLI Tool

**Input**: Design documents from `specs/041-pato-cli/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓, quickstart.md ✓

**Organization**: Tasks grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: Which user story this task belongs to (US1–US6)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the `crates/pato` workspace member with skeleton CLI wiring.

- [X] T001 Add `"crates/pato"` to `members` in root `Cargo.toml`; add `clap = { version = "4", features = ["derive"] }` and `strsim = "0.11"` to `[workspace.dependencies]`
- [X] T002 Create `crates/pato/Cargo.toml` with `[package] name = "relateby-pato"`, `[[bin]] name = "pato"`, and workspace-inherited fields; add dependencies: `relateby-pattern`, `relateby-gram`, `clap`, `serde`, `serde_json`, `thiserror`, `strsim`
- [X] T003 Create `crates/pato/src/cli.rs` — `Commands` enum with stub variants for `Lint`, `Fmt`, `Parse`, `Rule`, `Check`, and `#[command(external_subcommand)] External(Vec<String>)` for unknown-subcommand forwarding
- [X] T004 Create `crates/pato/src/main.rs` — parse args with clap, dispatch to stub handlers that `eprintln!("not yet implemented")` and exit 0; `External` variant prints error to stderr and exits 3
- [X] T005 Verify `cargo build -p relateby-pato` and `cargo run -p relateby-pato -- --version` succeed
- [X] T006 Create `crates/pato/tests/fixtures/valid/`, `tests/fixtures/invalid/`, and `tests/fixtures/schema/` directories with `.gitkeep` placeholders

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Diagnostic type system and gram serialization infrastructure. **Blocks all user stories.**

**⚠️ CRITICAL**: No user story work can begin until this phase is complete.

- [X] T007 Implement `crates/pato/src/diagnostics.rs` — define `Severity` (Error/Warning/Info), `DiagnosticCode` (P001–P008 with associated grade and rule name constants), `Location { line: u32, column: u32 }`, `Edit` (Replace/DeleteLine/Append), `RemediationSteps` (Inline/Structured), `RemediationOption`, `Remediation` (Auto/Guided/Ambiguous/None), `Diagnostic` struct with all fields per data-model.md
- [X] T008 Implement `crates/pato/src/output.rs` — `OutputFormat` enum (Gram/Text/Json); TTY detection via `use std::io::IsTerminal; stdout().is_terminal()` stored at startup; rendering dispatch function that routes a gram string, diagnostic list, or JSON value to stdout in the chosen format; ANSI color enabled in Text mode only when TTY detected
- [X] T009 Implement `crates/pato/src/diagnostic_gram.rs` — serialize `Vec<Diagnostic>` to a gram string per `specs/041-pato-cli/contracts/diagnostic-gram.md`; single-file header `{ kind: "diagnostics", pato_version: "...", file: "..." }`; `Summary` pattern for clean files; `Location > Diagnostic > Remediation` nesting; scalar `remediations` array for `Inline` steps; child `Remediation` patterns for `Structured` steps; `Option` child patterns for `Ambiguous`
- [X] T010 Implement multi-file wrapper in `diagnostic_gram.rs` — `Run > FileResult > Location > Diagnostic` grouping for when multiple files are processed
- [X] T011 Add `to_json` serialization to `diagnostic_gram.rs` using `serde_json` — convert `Vec<Diagnostic>` to a JSON structure mirroring the gram output shape
- [X] T012 Wire `OutputFormat` into `diagnostic_gram.rs` — top-level `render_diagnostics(diagnostics, output_format, writer)` function that dispatches to gram/text/json rendering
- [X] T013 Write smoke test in `crates/pato/tests/` verifying that a `Vec<Diagnostic>` with one diagnostic of each grade (auto/guided/ambiguous/none) serializes to a gram string that parses cleanly via `relateby_gram::parse_gram`

**Checkpoint**: Foundation ready — all user story phases can now begin.

---

## Phase 3: User Story 1 — Lint a Gram File (Priority: P1) 🎯 MVP

**Goal**: `pato lint` parses gram files, checks all P-codes, emits diagnostic gram on stdout, exits with correct code.

**Independent Test**: `cargo run -p relateby-pato -- lint crates/pato/tests/fixtures/invalid/P002.gram` emits gram with a P002 diagnostic on stdout, exits 2; output parses via `relateby_gram::parse_gram`.

- [X] T014 [US1] Create `crates/pato/tests/fixtures/invalid/P001.gram` — syntactically broken gram that triggers a parse error; `P002.gram` — two patterns sharing the same identity; `P003.gram` — annotation with duplicate property key; `P004.gram` — relationship with lowercase label; `P005.gram` — pattern referencing an identity not defined in file; `P006.gram` — `[]` used as a property value; `P008.gram` — document header `{ kind: "unknownkind" }`
- [X] T015 [US1] Create `crates/pato/tests/fixtures/valid/simple.gram` — a small valid gram file with no lint issues (node, relationship, annotation, document header with recognized kind)
- [X] T016 [US1] Implement `crates/pato/src/commands/lint.rs` — accept `files: Vec<PathBuf>` and `OutputFormat`; read each file; call `relateby_gram::parse_gram`; on `ParseError`, produce P001 `Diagnostic` with location from the error; on success, pass `Vec<Pattern<Subject>>` to the rule checkers below
- [X] T017 [P] [US1] Implement P002 checker in `commands/lint.rs` — collect all non-empty `subject.identity.0` strings into a `HashMap<String, Location>`; on second occurrence emit P002 guided diagnostic referencing both line numbers
- [X] T018 [P] [US1] Implement P003 checker in `commands/lint.rs` — for each pattern's `subject.properties`, detect duplicate keys by iterating the raw AST (use `parse_to_ast` for key-order access); emit P003 guided diagnostic with location of the duplicate key
- [X] T019 [P] [US1] Implement P004 checker in `commands/lint.rs` — for each pattern, determine arity (0/1 = node label → TitleCase; 2 = rel label → UPPERCASE); check each label string; emit P004 auto-grade warning with a `Replace` edit when casing is wrong
- [X] T020 [US1] Implement P005 checker in `commands/lint.rs` — collect defined identity set from all patterns; for each pattern using an identity as a reference not in the defined set, emit P005 ambiguous warning; use `strsim::levenshtein` to find the closest defined identity and offer it as Option 1; offer "add definition" as Option 2
- [X] T021 [P] [US1] Implement P006 checker in `commands/lint.rs` — scan `subject.properties` values for `Value::VArray(arr)` where `arr.is_empty()`; emit P006 info/guided diagnostic
- [X] T022 [P] [US1] Implement P008 checker in `commands/lint.rs` — detect bare record first pattern via `parse_gram_with_header`; if `kind` property is present and value is not in `["diagnostics", "rule"]`, emit P008 warning/guided diagnostic
- [X] T023 [US1] Implement `crates/pato/src/editor.rs` — `apply_edits(file: &Path, edits: &[Edit])` using reverse-order line sort to prevent drift; write to a temp file alongside the original, then atomically rename; report modified file name to stderr
- [X] T024 [US1] Wire `--fix` flag in `commands/lint.rs` — after collecting diagnostics, filter to `auto`-grade; if any `ambiguous` diagnostic exists for the file skip it entirely (report ambiguous on stdout, leave file unchanged); otherwise call `editor::apply_edits` with all auto edits; re-run lint on rewritten file to verify clean
- [X] T025 [US1] Add stdin support in `commands/lint.rs` — when file path is `-`, read from `std::io::stdin()` and use `"<stdin>"` as the filename in diagnostics
- [X] T026 [US1] Wire exit codes in `main.rs` lint dispatch — compute highest severity across all files; exit 0/1/2 per contract; exit 3 for file-not-found or unreadable files (report on stderr, skip file)
- [X] T027 [US1] Write integration test in `crates/pato/tests/lint_tests.rs` — for each `invalid/P00N.gram` fixture: run lint, assert correct P-code present in output, assert output parses via `parse_gram`, assert exit code is 2 for errors / 1 for warnings / 0 for valid fixture; also test `--fix` rewrites P004 fixture to clean

**Checkpoint**: `pato lint` is fully functional and independently testable.

---

## Phase 3b: CST Alignment After 042 Merge

**Purpose**: Rebase the completed lint MVP and the remaining parser-sensitive work onto the
merged CST parser so pato stops relying on raw text reconstruction for source fidelity.

- [X] T060 Refactor `crates/pato/Cargo.toml` to enable `gram_codec`'s `cst` feature for the pato crate dependency
- [X] T061 Create `crates/pato/src/source_map.rs` — helpers for converting `gram_codec::cst::SourceSpan` to `Location { line, column }`, slicing source text by span, and ordering edits safely
- [X] T062 Refactor `crates/pato/src/commands/lint.rs` — parse via `parse_gram_cst`; emit P001 from `CstParseResult.errors`; lower the valid CST tree only where semantic checks still require `Pattern<Subject>`
- [X] T063 Replace raw text scanning heuristics in `commands/lint.rs` with CST-backed traversals for identities, labels, annotations, and document/header inspection
- [X] T064 Expand `crates/pato/tests/lint_tests.rs` — add fixtures/assertions for precise duplicate-identity locations, identified annotation handling, and comment-bearing files

**Checkpoint**: Lint remains green, but its locations and syntax-aware checks are now derived from CST spans and preserved annotations.

---

## Phase 3c: Diagnostic Contract Realignment After Modeling Exploration

**Purpose**: Rework pato's unreleased diagnostic contract around the adopted compact
rule-driven model before building additional features on top of the older nested report shape.

- [X] T065 Update `specs/041-pato-cli/contracts/diagnostic-gram.md` — replace the nested/container-oriented examples with the adopted compact rule-driven schema; document canonical occurrence facts, stable rule/remediation identifiers, optional gram comments, and JSON parity without comments
- [X] T066 Refactor `crates/pato/src/diagnostics.rs` — model diagnostics as compact problem occurrences with stable rule/remediation ids and occurrence parameters; move reusable prose and remediation knowledge into a shared rule registry used by renderers and `pato rule`
- [X] T067 Refactor `crates/pato/src/diagnostic_gram.rs` — emit compact rule-driven gram/json output, including optional explanatory comments in gram output only; remove dependence on per-instance canonical `message` / `decision` / `summary` fields
- [X] T068 Refactor `crates/pato/src/output.rs` text rendering — render human-readable diagnostics from structured occurrence data plus the shared rule registry instead of stored per-instance prose
- [X] T069 Update `crates/pato/src/commands/lint.rs` — populate the new occurrence model, emit stable rule/remediation identifiers and fix parameters, and keep `--fix` behavior aligned with canonical edits/options
- [X] T070 Update `crates/pato/tests/lint_tests.rs` and serializer tests — assert the new gram/json output shape, verify comments are optional/non-canonical, and ensure emitted rule/remediation identifiers resolve through the registry

**Checkpoint**: Diagnostic output is realigned to the adopted v0.1 contract; lint, text, and JSON all derive from the same compact rule-driven data.

---

## Phase 4: User Story 2 — Format a Gram File (Priority: P2)

**Goal**: `pato fmt` rewrites gram files to canonical style idempotently; `--check` enables CI enforcement.

**Independent Test**: Run `pato fmt` on `tests/fixtures/invalid/P004.gram` (lowercase label), verify the file is rewritten with uppercase label; run `pato fmt` again, verify no change; run `pato lint` on result, verify zero `auto` diagnostics.

- [X] T028 [US2] Implement `crates/pato/src/commands/fmt.rs` — accept files or `-`; parse each file via `parse_gram_cst`; on CST errors report to stderr, exit 2, skip file; on success apply canonical style transformations through a CST-assisted rewrite pipeline
- [X] T029 [US2] Implement canonical style rules in `commands/fmt.rs`: (1) sort properties alphabetically within each record before rendering; (2) ensure document header is first; (3) ensure single blank line between top-level patterns/comments; (4) consistent spacing around arrow families; preserve arrow family and label separator choices as-is; preserve top-level comments where practical
- [X] T030 [US2] Implement `--check` mode in `commands/fmt.rs` — compare computed canonical string to original file bytes; if different, report filename to stderr and set exit flag; exit 1 if any file would change, 0 if all canonical; no files modified
- [X] T031 [US2] Implement stdin→stdout mode in `commands/fmt.rs` — when `-` is passed, read stdin, apply canonical transform, write to stdout; on parse error write to stderr and exit 2
- [X] T032 [US2] Create before/after fixture pairs in `tests/fixtures/` for fmt tests — `fmt_before_spacing.gram`, `fmt_before_props.gram`, `fmt_before_header.gram`, plus at least one fixture with preserved top-level comments / identified annotations
- [X] T033 [US2] Write integration tests in `crates/pato/tests/fmt_tests.rs` — verify each before/after pair transforms correctly; verify idempotency (`fmt(fmt(x)) == fmt(x)`) for all valid fixtures; verify `pato lint` reports zero `auto`-grade diagnostics on all `pato fmt` output; verify `--check` exits 1 on before-fixtures and 0 on already-formatted files

**Checkpoint**: `pato fmt` fully functional and independently testable.

---

## Phase 5: User Story 3 — Parse and Inspect a Gram File (Priority: P3)

**Goal**: `pato parse` emits parsed pattern structure in gram, sexp, json, or summary format with round-trip stability.

**Independent Test**: Run `pato parse my.gram | pato parse -` and verify the second invocation's output is identical to the first (round-trip stability); verify `--output-format json` produces a parseable JSON array.

- [ ] T034 [US3] Implement `crates/pato/src/commands/parse.rs` — accept files or `-`; parse via `parse_gram_cst`; on CST errors report to stderr and exit 2; on success emit selected output format from either CST or lowered semantic patterns as appropriate
- [ ] T035 [US3] Implement gram output mode in `commands/parse.rs` — lower valid CST trees, serialize each top-level pattern via `to_gram_pattern`, join with newline, emit as flat sequence with no root wrapper; verify round-trip: `parse(gram_out) == lowered_patterns`
- [ ] T036 [P] [US3] Implement sexp output mode in `commands/parse.rs` — render directly from CST structure so the output mirrors tree-sitter/gramref shape without reconstructing it from `Pattern<Subject>`
- [ ] T037 [P] [US3] Implement json output mode in `commands/parse.rs` — convert lowered `Pattern<Subject>` values to `AstPattern`/JSON array via `serde_json`
- [ ] T038 [P] [US3] Implement summary output mode in `commands/parse.rs` — count nodes/relationships/annotations/walks from the CST tree so annotation counts remain accurate even when the semantic tree would drop syntax detail
- [ ] T039 [US3] Write integration tests in `crates/pato/tests/parse_tests.rs` — verify gram round-trip stability using valid fixtures; verify json output is parseable JSON array; verify sexp output matches gramref for at least two corpus fixtures; verify no root-wrapper nesting on repeated round-trips; verify annotation-aware summary counts

**Checkpoint**: `pato parse` fully functional and independently testable.

---

## Phase 6: User Story 4 — Explain a Diagnostic Rule (Priority: P3)

**Goal**: `pato rule` exposes the shared rule/remediation registry as gram/json output; agents can look up any P-code and its reusable fix templates without reading source.

**Independent Test**: Run `pato rule` (no args), verify gram output parses cleanly and lists all P001–P008 plus their remediation templates; run `pato rule P002`, verify output includes code, name, grade, remediation identifiers, and a `TriggerExample` that when linted produces P002.

- [ ] T040 [US4] Extend the shared rule registry in `crates/pato/src/diagnostics.rs` — add `RuleInfo { code, name, severity, grade, description, remediations, trigger_example_gram }` and stable remediation template identifiers/parameter docs for each `DiagnosticCode`
- [ ] T041 [US4] Implement `crates/pato/src/commands/rule.rs` — list mode (no args): emit gram file of kind `"rule"` with one `Rule` pattern per P-code plus its reusable remediation templates; detail mode (with code): emit single `Rule` pattern with `TriggerExample` child and remediation template detail
- [ ] T042 [US4] Add JSON output mode to `commands/rule.rs` for `--output-format json`
- [ ] T043 [US4] Handle unknown code in `commands/rule.rs` — emit error on stderr and exit 3
- [ ] T044 [US4] Write integration tests in `crates/pato/tests/rule_tests.rs` — verify all P-codes and remediation templates have registry entries; verify listing gram output parses cleanly; verify each trigger_example_gram when linted produces exactly the claimed P-code; verify lint output references valid registry identifiers

**Checkpoint**: `pato rule` fully functional; agents can self-serve on unknown diagnostics.

---

## Phase 7: User Story 5 — Check Against a Schema (Priority: P4)

**Goal**: `pato check` is the one-stop CI command — lint + schema discovery + P007 signal.

**Independent Test**: Run `pato check my.gram` (no schema) — verify lint runs and P007 is in output; run `pato check my.gram` with `my.schema.gram` alongside — verify P007 is absent and schema path appears on stderr.

- [ ] T045 [US5] Implement `crates/pato/src/schema.rs` — `discover_schema(data_file: &Path, override_path: Option<&Path>) -> Option<PathBuf>`; if `override_path` is Some, return it; otherwise look for `<stem>.schema.gram` alongside the data file; return None if not found
- [ ] T046 [US5] Implement `crates/pato/src/commands/check.rs` — for each input file: (1) run full lint, (2) call `schema::discover_schema`, (3) if no schema → append P007 info diagnostic; if schema found → log `"using schema: <path>"` to stderr and suppress P007; emit combined diagnostics via `render_diagnostics`
- [ ] T047 [US5] Create `tests/fixtures/schema/sample.schema.gram` and `tests/fixtures/valid/sample.gram` with matching stem for schema discovery tests
- [ ] T048 [US5] Write integration tests in `crates/pato/tests/check_tests.rs` — no schema: P007 present, lint diagnostics present, exit reflects lint severity; with same-stem schema: P007 absent, schema path on stderr; with `--schema` override: specified schema acknowledged; invalid `--schema` path: exit 3 with error on stderr

**Checkpoint**: `pato check` fully functional; CI workflows can use it as single entry point.

---

## Phase 8: User Story 6 — Invoke a Binary Extension (Priority: P4)

**Goal**: `pato xyz` dispatches to `pato-xyz` on PATH; `pato --help` lists discovered extensions.

**Independent Test**: Create a minimal `pato-foo` shell script on PATH that echoes args and exits 0; run `pato foo --test`, verify args forwarded; run `pato --help`, verify `pato-foo` appears.

- [ ] T049 [US6] Implement `crates/pato/src/extensions.rs` — `discover_extensions() -> Vec<(String, Option<String>)>` that scans PATH directories for binaries starting with `pato-`, then queries each with `--pato-describe` (timeout ~1 second); returns `(binary_name, description_or_none)` pairs; deduplicate by name
- [ ] T050 [US6] Implement `exec_extension(subcommand: &str, args: &[String])` in `extensions.rs` — construct binary name `pato-<subcommand>`; if not found in PATH emit error on stderr and exit 3; if found use `std::process::Command::new(bin).args(args).status()` with inherited stdin/stdout/stderr; relay exit code via `std::process::exit(status.code().unwrap_or(3))`
- [ ] T051 [US6] Wire `Commands::External(args)` in `main.rs` to call `extensions::exec_extension` with `args[0]` as subcommand and `&args[1..]` as forwarded args
- [ ] T052 [US6] Update `--help` output in `cli.rs` / `main.rs` to call `extensions::discover_extensions()` and append discovered extensions to the help text under an "Extensions" section with their descriptions
- [ ] T053 [US6] Write integration tests in `crates/pato/tests/extensions_tests.rs` — test unknown subcommand with no PATH match exits 3 with stderr message; test `--pato-describe` protocol by creating a minimal test binary; test help listing includes discovered extensions

**Checkpoint**: Extension ecosystem is open; `pato-apply` and `pato-ingest` can be built independently.

---

## Phase 9: Polish & Cross-Cutting Concerns

**Purpose**: Code quality, CI compliance, and pipeline integration validation.

- [ ] T054 [P] Run `cargo fmt --all` and fix any formatting issues across `crates/pato/src/`
- [ ] T055 [P] Run `cargo clippy --workspace -- -D warnings` and fix all lint warnings in `crates/pato/`
- [ ] T056 Run `cargo test --workspace` and verify all tests pass including pre-existing workspace tests
- [ ] T057 Verify the diagnostic gram pipeline: `cargo run -p relateby-pato -- lint crates/pato/tests/fixtures/invalid/P004.gram | cargo run -p relateby-pato -- parse -` — confirm lint output is valid gram that parse accepts
- [ ] T058 Run `./scripts/ci-local.sh` and fix any CI failures
- [ ] T059 Review and update `specs/041-pato-cli/quickstart.md` with any corrections discovered during implementation

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 — **BLOCKS all user stories**
- **US1 Lint (Phase 3)**: Depends on Phase 2 — first story to implement
- **CST Alignment (Phase 3b)**: Depends on Phase 3 — rebases the completed lint MVP onto merged CST support
- **Diagnostic Contract Realignment (Phase 3c)**: Depends on Phase 3b — updates the unreleased diagnostic/reporting contract before further feature work
- **US2 Fmt (Phase 4)**: Depends on Phase 3c and `editor.rs` from Phase 3
- **US3 Parse (Phase 5)**: Depends on Phase 3c
- **US4 Rule (Phase 6)**: Depends on Phase 3c (shared registry introduced there); otherwise independent of US2–US3
- **US5 Check (Phase 7)**: Depends on US1 (runs lint internally via shared function)
- **US6 Extensions (Phase 8)**: Depends on Phase 1 only (CLI wiring); independent of all other stories
- **Polish (Phase 9)**: Depends on all desired stories being complete

### User Story Dependencies

- **US1 (P1)**: Can start after Phase 2. No dependencies on other stories.
- **US2 (P2)**: Should start after Phase 3c. Uses `editor.rs` built in US1 and depends on CST-backed parsing plus the realigned diagnostic contract.
- **US3 (P3)**: Should start after Phase 3c so sexp/summary outputs can use the merged CST parser directly while gram/json output expectations stay aligned with the adopted report model.
- **US4 (P3)**: Should start after Phase 3c because the rule/remediation registry becomes shared infrastructure for lint and text rendering.
- **US5 (P4)**: Can start after US1 is complete (T016 provides the lint runner it calls).
- **US6 (P4)**: Can start after Phase 1. Fully independent.

### Parallel Opportunities Within Each Story

**US1**: T017, T018, T019, T021, T022 are independent rule checkers — all can run in parallel after T016.

**US3**: T036, T037, T038 (sexp/json/summary output modes) are independent and can run in parallel after T035.

---

## Parallel Example: User Story 1 (Lint)

```
# After T016 (lint.rs scaffold with parse + P001), these can run in parallel:
T017: P002 duplicate identity checker
T018: P003 duplicate annotation key checker
T019: P004 label case checker (auto grade)
T021: P006 empty array checker
T022: P008 unknown document kind checker

# T020 (P005 dangling reference) runs solo — depends on full identity set from all checkers
# T023 (editor.rs) runs solo — needed before T024 (--fix)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL — blocks everything)
3. Complete Phase 3: User Story 1 (pato lint)
4. Complete Phase 3b: CST alignment after the 042 merge
5. Complete Phase 3c: diagnostic contract realignment after the modeling exploration
6. **STOP and VALIDATE**: `pato lint invalid/P002.gram` emits compact parseable diagnostic gram with CST-derived locations and stable rule/remediation identifiers, exits 2
7. At this point, pato has real value for CI and agentic use

### Incremental Delivery

1. Setup + Foundational → scaffolded binary with type system
2. US1 (lint) → MVP: gram file checking with structured diagnostics
3. Phase 3b (CST alignment) → precise, syntax-aware lint foundation
4. Phase 3c (diagnostic realignment) → compact rule-driven reporting contract
5. US2 (fmt) → canonical formatting; `--check` for CI
6. US3 (parse) → structural inspection and round-trip verification
7. US4 (rule) → agent self-service on unknown P-codes and reusable remediations
8. US5 (check) → single CI entry point with schema awareness
9. US6 (extensions) → ecosystem open for `pato-apply`, `pato-ingest`

### Parallel Team Strategy

Once Phase 3c is complete:

- Dev A: US2 (fmt) — canonical rewrites + fixtures
- Dev B: US3 (parse) + US4 (rule) — output formats + registry exposure
- Dev C: US6 (extensions) — can start from Phase 1

---

## Notes

- `[P]` tasks operate on different files with no incomplete dependencies
- `[Story]` label maps each task to its user story for traceability
- Each user story checkpoint describes a self-contained, demonstrable deliverable
- `editor.rs` (T023, built in US1) is shared by US2 (`pato fmt`); build it before starting T028–T031
- The diagnostic gram format in `contracts/diagnostic-gram.md` remains draft until Phase 3c is complete; the adopted compact rule-driven schema is the intended stable v0.1 API
- `std::io::IsTerminal` is available at MSRV 1.70.0 — no additional crate needed for TTY detection
- Total tasks: 70 (T001–T070)
