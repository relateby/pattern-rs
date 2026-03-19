# Implementation Plan: pato CLI Tool

**Branch**: `041-pato-cli` | **Date**: 2026-03-18 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/041-pato-cli/spec.md`

## Summary

Build `pato` — a CLI tool for linting, formatting, parsing, and inspecting gram files. The tool emits structured diagnostic output in gram notation, enabling a fully machine-readable feedback loop for both developers and coding agents. The core v0.1 subcommands are `lint`, `fmt`, `parse`, `rule`, and `check`, plus binary extension dispatch (`pato-foo`).

pato is a net-new CLI binary in the pattern-rs workspace, not a port of an existing gram-hs tool. It depends entirely on the existing `relateby-pattern` and `relateby-gram` workspace crates for parsing and serialization.

## Technical Context

**Language/Version**: Rust 1.70.0 (workspace MSRV), Edition 2021
**Primary Dependencies**: `relateby-pattern` (workspace), `relateby-gram` (workspace), `clap` v4 with derive, `serde`/`serde_json` (workspace), `thiserror` (workspace), `strsim` v0.11 (new)
**Storage**: Local filesystem — gram files read/written in-place. Atomic writes (temp-file + rename). No database.
**Testing**: `cargo test`; fixture-based integration tests; property tests for idempotency
**Target Platform**: Native CLI binary — Linux, macOS, Windows. Not WASM (CLI tool).
**Project Type**: CLI binary (`relateby-pato`, binary `pato`)
**Performance Goals**: Process 10,000-line gram files in under 1 second (SC-001)
**Constraints**: No async runtime; no network I/O; atomic file writes; no shell dispatch for extensions
**Scale/Scope**: Single-user local tool; files up to ~100k lines; PATH scan bounded by typical PATH length

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle I — Reference Implementation Fidelity

**Status**: PARTIAL — DOCUMENTED EXCEPTION

pato is a net-new CLI tool. gram-hs has `gramref` (a conformance testing tool) but no developer-facing lint/format/check CLI. The diagnostic codes (P001–P008), remediation grades, and canonical formatting rules are original to pattern-rs.

**Fidelity maintained where applicable**:
- `pato parse --output-format gram` — round-trips through `relateby-gram` (gram-hs compatible)
- `pato parse --output-format sexp` — matches gramref tree-sitter sexp output for shared fixtures
- `pato parse --output-format json` — matches `AstPattern` JSON structure from gram-codec

**Documented deviation**: pato lint rules and diagnostic gram format are original to pattern-rs. No gram-hs equivalent exists. This is an intentional design decision, not an accidental omission.

### Principle II — Correctness & Compatibility

**Status**: PASS

pato delegates all gram parsing and serialization to `relateby-gram`, maintaining full compatibility with the gram-hs format. No independent parser is implemented.

### Principle III — Rust Native Idioms

**Status**: PASS

New crate; idiomatic Rust throughout: `Result<T, E>` for errors (`thiserror`), clap v4 derive API, `std::io::IsTerminal` for TTY detection, `std::process::Command` for extension exec.

### Principle IV — Multi-Target Library Design

**Status**: JUSTIFIED EXCEPTION

pato is a CLI binary, not a library. It requires filesystem access and process spawning. WASM compilation is not applicable. The underlying `relateby-gram` and `relateby-pattern` libraries maintain WASM compatibility independently.

### Principle V — External Language Bindings

**Status**: JUSTIFIED EXCEPTION

CLI tools do not have language bindings. Shell usage examples are documented in `quickstart.md` and the contracts.

## Project Structure

### Documentation (this feature)

```text
specs/041-pato-cli/
├── plan.md              # This file
├── spec.md              # Feature specification
├── research.md          # Phase 0 research findings
├── data-model.md        # Phase 1 data model
├── quickstart.md        # Phase 1 developer quickstart
├── contracts/
│   ├── cli-schema.md          # Subcommand interface contract
│   ├── diagnostic-gram.md     # Diagnostic gram format (stable API)
│   └── extension-protocol.md # pato-foo extension protocol
└── tasks.md             # Phase 2 output (/speckit.tasks — NOT created here)
```

### Source Code

```text
crates/pato/
├── Cargo.toml              # [package] name = "relateby-pato"; [[bin]] name = "pato"
├── src/
│   ├── main.rs             # Entry point; clap parse; subcommand dispatch
│   ├── cli.rs              # Clap definitions: Commands enum + per-subcommand Args structs
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── lint.rs         # pato lint: parse → check rules → emit diagnostics
│   │   ├── fmt.rs          # pato fmt: parse → canonical rewrite → write/check
│   │   ├── parse.rs        # pato parse: parse → emit gram/sexp/json/summary
│   │   ├── rule.rs         # pato rule: rule registry lookup and emit
│   │   └── check.rs        # pato check: lint + schema discovery
│   ├── diagnostics.rs      # Diagnostic, DiagnosticCode, Severity, Remediation, Edit types
│   ├── diagnostic_gram.rs  # Serialize Vec<Diagnostic> → gram per contracts/diagnostic-gram.md
│   ├── output.rs           # OutputFormat enum; gram/text/json rendering; TTY detection
│   ├── editor.rs           # Atomic in-place file editing (reverse-order edits, temp+rename)
│   ├── schema.rs           # Same-stem *.schema.gram discovery; --schema override
│   └── extensions.rs       # PATH scan for pato-* binaries; --pato-describe; exec dispatch
└── tests/
    ├── fixtures/
    │   ├── valid/           # Gram files with no diagnostics
    │   │   └── simple.gram
    │   ├── invalid/         # One gram file per P-code
    │   │   ├── P001.gram    # Parse failure
    │   │   ├── P002.gram    # Duplicate identity
    │   │   ├── P003.gram    # Duplicate annotation key
    │   │   ├── P004.gram    # Label case
    │   │   ├── P005.gram    # Dangling reference
    │   │   ├── P006.gram    # Empty array
    │   │   └── P008.gram    # Unknown document kind
    │   └── schema/
    │       └── sample.schema.gram
    ├── lint_tests.rs        # One test per P-code; --fix tests; gram output parsability
    ├── fmt_tests.rs         # Before/after pairs; idempotency; lint-clean after fmt
    ├── parse_tests.rs       # Round-trip stability; sexp vs gramref; json array
    ├── rule_tests.rs        # All P-codes in registry; gram output valid
    └── check_tests.rs       # Schema discovery; P007 behavior
```

**Structure decision**: Single-crate CLI binary within the existing workspace. No new library crate; all pato logic is internal to the binary. If reusable lint APIs emerge (e.g., for IDE integration), they can be extracted to a `relateby-pato-core` library later.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Principle I exception (net-new tool) | pato is the first developer-facing CLI in the ecosystem; gram-hs provides no equivalent | Waiting for gram-hs equivalent would block the entire developer tooling story |
| Principle IV exception (no WASM) | CLI tools require filesystem + process I/O by definition | A WASM build of pato has no meaningful runtime environment |
| `strsim` new dependency | P005 requires edit-distance candidate suggestions — a correctness feature, not an optimization | Rolling our own Levenshtein is maintenance burden with no advantage |

## Implementation Sequence

### Step 1 — Scaffold

- Create `crates/pato/Cargo.toml` with `[package]`, `[[bin]]`, and dependencies
- Add `crates/pato` to workspace `members` in root `Cargo.toml`
- Add `clap = { version = "4", features = ["derive"] }` and `strsim = "0.11"` to workspace deps
- Implement `main.rs`: clap parse, built-in subcommand dispatch, extension dispatch (`pato-foo`)
- Implement `cli.rs`: `Commands` enum with External variant (`allow_external_subcommands`)
- Implement `extensions.rs`: PATH scan, `--pato-describe` query, exec with inherited streams
- **Tests**: `--version` smoke test; unknown subcommand dispatches to `pato-foo`

### Step 2 — Diagnostic Infrastructure

- Implement `diagnostics.rs`: `Diagnostic`, `DiagnosticCode`, `Severity`, `Remediation`, `RemediationSteps`, `Edit`, `RemediationOption` types
- Implement `diagnostic_gram.rs`: serialize `Vec<Diagnostic>` to gram per `contracts/diagnostic-gram.md`; scalar `remediations` for Inline steps; child `Remediation` patterns for Structured steps
- Implement `output.rs`: `OutputFormat` enum; gram (default), text (TTY-aware via `std::io::IsTerminal`), json rendering
- **Tests**: Serialize one diagnostic of each grade (auto/guided/ambiguous/none) to gram; verify output parses with `relateby_gram::parse_gram`; verify JSON round-trips

### Step 3 — `pato lint`

- Wire `parse_gram` → P001 (guided; location from ParseError)
- Implement duplicate identity detection → P002 (guided)
- Implement duplicate annotation key detection → P003 (guided)
- Implement label case warnings → P004 (auto; check arity to distinguish node/rel labels)
- Implement dangling reference warnings → P005 (ambiguous; `strsim::levenshtein` for nearest candidate)
- Implement empty array detection → P006 (info, guided)
- Implement document kind validation → P008 (warning, guided; check against `DocumentKind` registry)
- Implement `editor.rs`: reverse-order edits, atomic writes (temp file + rename)
- Wire `--fix`: apply `auto` remediations via `editor.rs`; skip file entirely if any `ambiguous` in scope
- **Tests**: One fixture per code; verify remediation grade and gram output structure; verify `--fix` produces clean-linting file; verify gram output is parseable

### Step 4 — `pato fmt`

- Implement canonical formatting rules (exhaustive `auto` remediations):
  - Consistent spacing around arrow families
  - Single blank line between top-level patterns
  - Properties sorted alphabetically within records
  - Document header at top of file
  - Arrow family and label separator preserved as-is
- Implement `-` (stdin → stdout) and `--check` modes
- Idempotency: `fmt(fmt(x)) == fmt(x)` for all fixtures
- **Tests**: Before/after fixture pairs; `pato lint` reports zero `auto` diagnostics on all `pato fmt` output; idempotency property test

### Step 5 — `pato parse`

- Implement `gram` output (flat top-level sequence, no root wrapper)
- Implement `sexp` output (tree-sitter sexp, matching gramref for shared fixtures)
- Implement `json` output (JSON array of `Pattern<Subject>` via `AstPattern`)
- Implement `summary` output (plain text counts: nodes, rels, annotations, walks)
- **Tests**: gram round-trip stability; sexp matches gramref for corpus fixtures; no root-wrapper nesting on repeated round-trips

### Step 5b — `pato rule`

- Implement rule registry: `DiagnosticCode` → name, description, grade, minimal trigger example
- Implement `pato rule` (no arg): emit gram file listing all rules as `Rule` patterns
- Implement `pato rule <code>`: emit single `Rule` pattern with `TriggerExample` child
- **Tests**: All P-codes have registry entries; gram output parses cleanly

### Step 6 — `pato check`

- Compose lint + schema discovery
- Same-stem `*.schema.gram` discovery; `--schema` override
- P007 when no schema; suppress P007 + log schema path when schema found
- **Tests**: With/without schema; explicit `--schema` path

## Dependency Changes

### Root `Cargo.toml` workspace additions

```toml
[workspace]
members = [
  # existing...
  "crates/pato",   # ADD
]

[workspace.dependencies]
# ADD:
clap     = { version = "4", features = ["derive"] }
strsim   = "0.11"
```

### `crates/pato/Cargo.toml`

```toml
[package]
name        = "relateby-pato"
version.workspace = true
edition.workspace = true
rust-version.workspace = true

[[bin]]
name = "pato"
path = "src/main.rs"

[dependencies]
relateby-pattern = { path = "../pattern-core",  version = "0.2" }
relateby-gram    = { path = "../gram-codec",    version = "0.2" }
clap             = { workspace = true }
serde            = { workspace = true }
serde_json       = { workspace = true }
thiserror        = { workspace = true }
strsim           = { workspace = true }
```
