# Feature Specification: pato CLI Tool

**Feature Branch**: `041-pato-cli`
**Created**: 2026-03-18
**Status**: Draft
**Input**: User description: "The pato-cli as described in proposals/pato-feature-proposal.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Lint a Gram File (Priority: P1)

A developer or coding agent runs `pato lint` on a gram file to discover syntactic and stylistic problems before committing or processing it further. The tool reports every issue as a structured diagnostic — with location, severity, a human-readable message, and a concrete remediation — in gram format on stdout. The process exits with a code indicating whether errors, warnings, or nothing was found.

**Why this priority**: Lint is the core value proposition of pato. Every other subcommand builds on the diagnostic infrastructure introduced here. Without a working lint command, pato has no purpose.

**Independent Test**: Run `pato lint` on a set of gram files containing known issues (duplicate identities, bad label casing, dangling references) and verify that diagnostic gram output is produced on stdout, is itself parseable gram, and that the exit code matches the severity of the highest-severity finding.

**Acceptance Scenarios**:

1. **Given** a gram file with a parse error, **When** `pato lint my.gram` is run, **Then** stdout contains a P001 diagnostic with the error location, a `guided` remediation, and the process exits with code 2.
2. **Given** a gram file with a duplicate identity, **When** `pato lint my.gram` is run, **Then** stdout contains a P002 diagnostic referencing both line numbers and the process exits with code 2.
3. **Given** a gram file with a lowercase relationship label, **When** `pato lint my.gram` is run, **Then** stdout contains a P004 warning with an `auto` remediation specifying the correct casing, and the process exits with code 1.
4. **Given** a gram file with a reference to an undefined identity, **When** `pato lint my.gram` is run, **Then** stdout contains a P005 warning with an `ambiguous` remediation listing the available options, and the process exits with code 1.
5. **Given** a valid gram file with no issues, **When** `pato lint my.gram` is run, **Then** stdout contains only the diagnostics header and a summary pattern with zero counts, and the process exits with code 0.
6. **Given** a gram file with only `auto`-grade issues, **When** `pato lint --fix my.gram` is run, **Then** the file is rewritten in place, the rewritten file lints clean, and diagnostic gram for any remaining issues is emitted on stdout.
7. **Given** gram text on stdin, **When** `pato lint -` is run, **Then** lint operates on the stdin content with the same behavior as file input.
8. **Given** `--output-format json`, **When** `pato lint` is run, **Then** stdout contains a JSON representation of the same diagnostic information.
9. **Given** `--output-format text` and stdout is a terminal, **When** `pato lint` is run, **Then** stdout contains a human-readable, ANSI-colored rendering of the diagnostics. **Given** stdout is not a terminal (piped or redirected), **When** `--output-format text` is used, **Then** output contains no ANSI escape codes.

---

### User Story 2 - Format a Gram File (Priority: P2)

A developer runs `pato fmt` to apply canonical style to their gram files — consistent spacing, alphabetically sorted properties, blank lines between top-level patterns. The operation is idempotent: formatting an already-formatted file changes nothing.

**Why this priority**: Formatting is the natural counterpart to linting. It resolves all auto-fixable issues non-interactively and enables CI enforcement of style consistency. It directly exercises the same file-editing infrastructure as `lint --fix`.

**Independent Test**: Run `pato fmt` on before/after fixture pairs and verify the output matches expected canonical form; run `pato fmt` twice on the same file and verify the result is identical both times; verify `pato lint` reports zero `auto` diagnostics on all formatted output.

**Acceptance Scenarios**:

1. **Given** a gram file with inconsistent spacing and unsorted properties, **When** `pato fmt my.gram` is run, **Then** the file is rewritten in canonical style and the process exits with code 0.
2. **Given** a gram file that is already canonical, **When** `pato fmt my.gram` is run, **Then** the file is unchanged.
3. **Given** `pato fmt -` with gram text on stdin, **When** run, **Then** canonical gram is emitted on stdout.
4. **Given** `pato fmt --check **/*.gram` in CI, **When** any file would change, **Then** the process exits with code 1 without modifying files.
5. **Given** formatting is applied, **When** `pato lint` is subsequently run on the output, **Then** no `auto`-grade diagnostics are reported.

---

### User Story 3 - Parse and Inspect a Gram File (Priority: P3)

A developer or agent runs `pato parse` to inspect the structure of a gram file — seeing the parsed pattern tree as gram, s-expressions, or JSON. This is useful for debugging gram notation, building integrations, or verifying round-trip correctness.

**Why this priority**: Parse is instrumental for integrations and debugging. It is lower priority than lint/fmt because its primary audience is developers and tooling builders, not the everyday workflow of checking gram files.

**Independent Test**: Run `pato parse` on a gram file and verify that the default gram output can be fed back into `pato parse` to produce identical output (stable round-trip); verify JSON output contains the expected pattern tree.

**Acceptance Scenarios**:

1. **Given** a gram file, **When** `pato parse my.gram` is run, **Then** stdout contains the parsed patterns as gram notation with no wrapping, and the process exits with code 0.
2. **Given** `--output-format json`, **When** `pato parse my.gram` is run, **Then** stdout contains a JSON array of pattern objects.
3. **Given** `--output-format sexp`, **When** `pato parse my.gram` is run, **Then** stdout contains s-expression output matching the reference implementation output for shared fixtures.
4. **Given** gram output from `pato parse` is piped back into `pato parse`, **When** run, **Then** the output is identical (round-trip stability with no added nesting).
5. **Given** a parse error in the file, **When** `pato parse my.gram` is run, **Then** the error is reported on stderr and the process exits with code 2.

---

### User Story 4 - Explain a Diagnostic Rule (Priority: P3)

A coding agent or developer encounters an unfamiliar diagnostic code in `pato lint` output and runs `pato rule P002` to understand what the rule means, what triggers it, and what remediation options exist — without reading source code.

**Why this priority**: Critical for agentic consumers that must act on diagnostics autonomously. Lower priority than the core lint/fmt/parse loop because it is a discovery/explanation feature rather than a data-processing feature.

**Independent Test**: Run `pato rule` with no arguments and verify all defined P-codes are listed; run `pato rule P002` and verify the output includes the rule name, severity, grade, description, and a trigger example that when linted produces P002.

**Acceptance Scenarios**:

1. **Given** no argument, **When** `pato rule` is run, **Then** stdout contains a gram file listing all known rules as `Rule` patterns with `code`, `severity`, `grade`, and `description` properties.
2. **Given** a valid P-code like `P002`, **When** `pato rule P002` is run, **Then** stdout contains a detailed `Rule` pattern with a `trigger_example` child pattern.
3. **Given** `--output-format json`, **When** `pato rule P002` is run, **Then** stdout contains the rule description as JSON.
4. **Given** an unknown code, **When** `pato rule P999` is run, **Then** stderr reports the unknown code and the process exits with code 3.

---

### User Story 5 - Check a Gram File Against a Schema (Priority: P4)

A developer runs `pato check` as a single "is this correct?" command that combines lint with optional schema validation. When a `*.schema.gram` file is present alongside the data file, it is discovered automatically.

**Why this priority**: `check` is the intended entry point for CI and agent workflows. It depends on lint being stable first. Schema validation itself is deferred to v0.2; this story covers the lint-plus-discovery behavior without full semantic validation.

**Independent Test**: Run `pato check` with and without a schema file present; verify that without a schema the P007 informational diagnostic is emitted and lint checks still run; verify that with `--schema` the schema path is accepted.

**Acceptance Scenarios**:

1. **Given** a gram file with no schema alongside it, **When** `pato check my.gram` is run, **Then** stdout includes lint diagnostics and a P007 informational note, and exit code reflects the highest-severity lint finding.
2. **Given** a same-stem `my.schema.gram` alongside `my.gram`, **When** `pato check my.gram` is run, **Then** lint diagnostics are emitted, P007 is suppressed (schema was found), and the schema path is noted on stderr.
3. **Given** `--schema types.schema.gram`, **When** `pato check my.gram` is run, **Then** the specified schema is acknowledged, P007 is suppressed, and the schema path is noted on stderr.

---

### User Story 6 - Invoke a Binary Extension (Priority: P4)

A user runs `pato xyz` where `xyz` is not a built-in subcommand. Pato searches PATH for `pato-xyz` and executes it, forwarding all arguments and streams transparently. The extension's exit code becomes pato's exit code.

**Why this priority**: Extension dispatch is required for the ecosystem to grow beyond built-ins. It enables future commands (`pato-apply`, `pato-ingest`) to be developed and distributed independently.

**Independent Test**: Place a minimal `pato-foo` binary on PATH. Run `pato foo --some-arg` and verify the argument is forwarded; run `pato --help` and verify `pato-foo` is listed with its description.

**Acceptance Scenarios**:

1. **Given** `pato-xyz` exists on PATH, **When** `pato xyz --arg val` is run, **Then** `pato-xyz` is invoked with `--arg val` forwarded verbatim.
2. **Given** `pato-xyz` does not exist on PATH, **When** `pato xyz` is run, **Then** an error is emitted on stderr and the process exits with code 3.
3. **Given** `pato-foo --pato-describe` returns a one-line description, **When** `pato --help` is run, **Then** `pato-foo` appears in the help listing with that description.
4. **Given** `pato-xyz` exits with code 2, **When** `pato xyz` is run, **Then** pato also exits with code 2.

---

### Edge Cases

- What happens when a file cannot be read (permissions, not found)? → stderr reports the error; the file is skipped; exit code 3.
- What happens when `lint --fix` encounters a file with an ambiguous diagnostic? → The file is skipped entirely; the ambiguous diagnostic is reported on stdout; no partial edits are applied.
- What happens when `fmt` is run on a file with a parse error? → The file is left unchanged; the parse error is reported on stderr; exit code 2.
- What happens when multiple files are passed and some have errors and some do not? → Each file is processed independently; a `Run` wrapper in the diagnostic gram groups per-file results; exit code reflects the highest severity across all files.
- What happens when the same identity appears in two different files? → P002 is file-scoped; no cross-file duplicate detection in v0.1.
- What happens when `pato fmt -` receives input with a parse error? → The error is reported on stderr; pato exits with code 2 without writing to stdout.

## Requirements *(mandatory)*

### Functional Requirements

**Subcommand: lint**

- **FR-001**: `pato lint <files>...` MUST parse each gram file and emit all diagnostics as a valid gram file on stdout.
- **FR-002**: `pato lint -` MUST read gram from stdin and emit diagnostics on stdout.
- **FR-003**: Diagnostic gram output MUST include a document header `{ kind: "diagnostics", pato_version: "...", file: "..." }`.
- **FR-004**: Each diagnostic MUST carry: severity, P-code, rule name, human-readable message, source location (line and column), and a remediation with a grade.
- **FR-005**: Remediations MUST be classified into exactly one grade: `auto`, `guided`, or `ambiguous`. Grade is a property of the rule, not the instance.
- **FR-006**: `pato lint` MUST check for: parse failure (P001), duplicate identity (P002), duplicate annotation key (P003), label case convention (P004), dangling reference (P005), empty array value (P006), and unknown document kind (P008).
- **FR-007**: `pato lint --fix` MUST apply all `auto`-grade remediations in-place using atomic writes and reverse-order line editing to prevent line number drift.
- **FR-008**: `pato lint --fix` MUST skip a file entirely if any ambiguous diagnostic is in scope for that file, reporting the ambiguous diagnostic instead of modifying the file.
- **FR-009**: `pato lint` MUST support `--output-format gram|text|json`; default is `gram`. In `text` mode, ANSI color codes MUST be auto-detected: enabled when stdout is a TTY, disabled when stdout is piped or redirected.
- **FR-010**: Diagnostic gram output MUST itself be parseable by the gram library.

**Subcommand: fmt**

- **FR-011**: `pato fmt <files>...` MUST rewrite each file in canonical style in-place.
- **FR-012**: `pato fmt -` MUST read from stdin and emit canonical gram on stdout.
- **FR-013**: `pato fmt --check <files>...` MUST exit with code 1 (warning-level) without modifying files if any file would change; exit code 0 if all files are already canonical.
- **FR-014**: `pato fmt` MUST be idempotent: formatting an already-formatted file MUST produce no change.
- **FR-015**: `pato fmt` output MUST pass `pato lint` with zero `auto`-grade diagnostics.
- **FR-016**: Canonical style MUST enforce: consistent spacing around arrow families, single blank line between top-level patterns, properties sorted alphabetically within a record, document header placed at top of file. Arrow family and label separator MUST be preserved as-is.

**Subcommand: parse**

- **FR-017**: `pato parse <files>...` MUST emit the parsed pattern structure on stdout.
- **FR-018**: Default output MUST be a flat sequence of top-level patterns with no implicit root wrapper.
- **FR-019**: `pato parse` MUST support `--output-format gram|sexp|json|summary`.
- **FR-020**: Round-trip stability MUST hold: `pato parse` output re-parsed by `pato parse` MUST produce identical output.

**Subcommand: rule**

- **FR-021**: `pato rule` with no argument MUST list all known P-codes as a gram file of kind `"rule"`.
- **FR-022**: `pato rule <code>` MUST emit a full rule description including: name, severity, grade, description, and a minimal gram snippet that would trigger the rule.
- **FR-023**: `pato rule` output MUST be a valid gram file.

**Subcommand: check**

- **FR-024**: `pato check <files>...` MUST run lint on each file and emit combined diagnostic gram.
- **FR-025**: `pato check` MUST automatically discover a same-stem `*.schema.gram` file alongside each input file.
- **FR-026**: `pato check` MUST accept `--schema <path>` to override schema discovery for all inputs.
- **FR-027**: When no schema is found or provided, `pato check` MUST emit a P007 informational diagnostic. When a schema IS found or provided, P007 MUST be suppressed and the schema path MUST be noted on stderr; full semantic validation is deferred to v0.2.

**Extension dispatch**

- **FR-028**: When an unknown subcommand is invoked, pato MUST search PATH for `pato-<subcommand>` and exec it with remaining arguments forwarded verbatim.
- **FR-029**: The extension binary's exit code MUST be relayed as pato's exit code without modification.
- **FR-030**: `pato --help` MUST discover and list all `pato-*` binaries on PATH, showing their one-line description if they respond to `--pato-describe`.

**I/O contract**

- **FR-031**: Exit code contract MUST be: 0 = no issues, 1 = warnings only, 2 = one or more errors, 3 = tool invocation error.
- **FR-032**: Stdout MUST always carry data only (diagnostic gram, transformed gram, or structured output). Stderr MUST carry only progress and log messages — never data.
- **FR-033**: All subcommands MUST accept `--output-format gram|text|json`.

### Key Entities

- **Diagnostic**: A finding about a gram file. Has a severity (error/warning/info), a P-code, a rule name, a human-readable message, a source location, and exactly one remediation.
- **Remediation**: The structured fix attached to a diagnostic. Classified as `auto`, `guided`, or `ambiguous`.
- **Edit**: A structured description of a file change: replace text at a location, delete a line, or append content.
- **DiagnosticCode (P-code)**: A stable, never-reused identifier for a rule (P001–P008 in v0.1).
- **Location**: A line and column reference into a gram file.
- **DocumentKind**: A recognized value for the `kind` property in a gram document header.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer can run `pato lint my.gram` and receive a complete, actionable diagnostic report for all known rule violations in under 1 second for files up to 10,000 lines.
- **SC-002**: Every diagnostic report emitted by pato is itself valid gram that can be re-parsed without error.
- **SC-003**: All `auto`-grade issues in a gram file are fully resolved by a single `pato fmt` invocation; subsequent lint produces zero `auto` diagnostics.
- **SC-004**: A coding agent receiving an unknown P-code can run `pato rule <code>` and obtain a self-contained explanation sufficient to understand and act on the diagnostic without consulting source code.
- **SC-005**: `pato fmt` is idempotent: running it twice on any file produces identical output both times.
- **SC-006**: The exit code contract is machine-reliable: scripts and CI pipelines can branch on exit codes 0/1/2/3 without parsing output.
- **SC-007**: The diagnostic gram format produced in v0.1 is stable enough to support a future `pato apply` command without breaking changes.
- **SC-008**: All diagnostic gram output for multiple files includes per-file result grouping, enabling consumers to process results file-by-file without parsing the entire output first.

## Clarifications

### Session 2026-03-18

- Q: Should `--output-format text` emit ANSI colors when output is piped or redirected? → A: Auto-detect TTY — colors on when stdout is a terminal, off when piped or redirected.
- Q: What exit code should `pato fmt --check` use when files need formatting? → A: Exit code 1 (warning-level; unformatted files are not data errors).
- Q: What does `pato check` v0.1 do when a schema is found but validation is deferred? → A: Run lint only; suppress P007 (schema acknowledged); log schema path to stderr.

## Assumptions

- The gram library is the authoritative gram parser; pato delegates all parsing to it and does not implement its own.
- Comment preservation is out of scope for v0.1 (the parser currently drops comments).
- Glob expansion is delegated to the shell in v0.1; pato does not expand globs internally.
- `pato validate` (semantic validation against a schema) is deferred to v0.2.
- Style settings (arrow family, label separator) are not configurable in v0.1; `pato fmt` preserves the author's choices for these.
- `pato apply` is deferred to v0.2+; the diagnostic gram format is treated as stable API from v0.1.
- Duplicate identity detection (P002) is file-scoped; cross-file detection is out of scope.
- Extension dispatch passes streams through transparently; pato does not inspect extension output.
