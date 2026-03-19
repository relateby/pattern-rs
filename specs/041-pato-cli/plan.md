# Implementation Plan: pato CLI Tool

**Branch**: `041-pato-cli` | **Date**: 2026-03-18 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `specs/041-pato-cli/spec.md`

## Summary

Build `pato` — a CLI tool for linting, formatting, parsing, and inspecting gram files. The tool emits structured diagnostic output in gram notation, enabling a fully machine-readable feedback loop for both developers and coding agents. After the diagnostic-format exploration, the v0.1 direction is a compact rule-driven report: canonical gram/json output carries stable rule and remediation identifiers plus source anchors and fix parameters, while optional gram comments and text output provide contextualized explanation as a rendered view. The core v0.1 subcommands are `lint`, `fmt`, `parse`, `rule`, and `check`, plus binary extension dispatch (`pato-foo`).

pato is a net-new CLI binary in the pattern-rs workspace, not a port of an existing gram-hs tool. With `042-gram-cst-parser` now merged, pato should be reoriented around a CST-first parsing pipeline for source-aware work (`lint`, `fmt`, `parse --output-format sexp`), while still lowering to the existing semantic `Pattern<Subject>` form for compatibility with the current serializer and semantic checks.

## Technical Context

**Language/Version**: Rust 1.70.0 (workspace MSRV), Edition 2021
**Primary Dependencies**: `relateby-pattern` (workspace), `relateby-gram` with `cst` feature enabled in `pato`, `clap` v4 with derive, `serde`/`serde_json` (workspace), `thiserror` (workspace), `strsim` v0.11 (new)
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

pato delegates all gram parsing and serialization to `relateby-gram`, maintaining full compatibility with the gram-hs format. With CST available, pato uses `parse_gram_cst` for source-aware analysis, then lowers to the same semantic forms already produced by `parse_gram`; no independent parser is implemented.

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
│   ├── diagnostic-gram.md     # Diagnostic gram format (draft until realignment is complete)
│   ├── schema-gram.md         # Archetypal schema contract and tagged-string DSL direction
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
│   ├── diagnostics.rs      # Diagnostic occurrence model, rule/remediation registry, Severity/Edit types
│   ├── diagnostic_gram.rs  # Serialize diagnostics to compact rule-driven gram/json reports
│   ├── output.rs           # OutputFormat enum; gram/text/json rendering; text/comments derived from registry
│   ├── source_map.rs       # CST SourceSpan → line/column helpers; source slicing utilities
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

## Post-042 Re-evaluation

`042-gram-cst-parser` changes the implementation strategy for the remaining 041 work:

- `pato lint` should stop reconstructing locations from raw text scans and instead use CST spans and preserved annotations as the source of truth.
- `pato fmt` should be planned as a CST-assisted rewrite pipeline so comment nodes and exact syntax-derived locations are not discarded immediately.
- `pato parse --output-format sexp` should be generated from CST structure rather than reverse-engineered from semantic patterns.
- Public CLI contracts remain line/column based, but the unreleased diagnostic gram/json contract should be realigned before v0.1: canonical output becomes compact and rule-driven, with explanatory prose treated as derived presentation rather than primary payload.

## Post-Exploration Diagnostic Realignment

The diagnostics modeling exercise in `data/diagnostics/` changes the plan for pato's reporting layer:

- Canonical diagnostic data should represent problem occurrences, source anchors, stable rule/remediation identifiers, and fix parameters.
- Per-instance prose (`message`, `decision`, `summary`) should no longer be treated as canonical required fields in gram/json output; they are better produced from the rule registry and occurrence parameters.
- `pato rule` is no longer just a convenience command. The rule/remediation registry becomes shared infrastructure used by `lint`, `check`, `text` rendering, and optional explanatory gram comments.
- Gram comments may carry rich contextual explanation, but comments are non-canonical presentation and must be optional to preserve machine-oriented stability.
- JSON mirrors the canonical structured report only; text mode is explicitly a rendering of structured data plus rule templates.

## Schema Exploration Direction

Schema work in this branch now has an explicit exploratory direction, informed by the examples in
`data/schema/`:

- A schema should itself be a gram document with `kind: "schema"`, not a separate sidecar format.
- The schema is archetypal: canonical example structures are part of the contract, not just
  illustrative data.
- Syntax choices in those examples may therefore be normative, including choices such as `:` vs
  `::`, arrow family, and annotation form.
- Property/value constraints should be carried in `::` schema slots using tagged strings.
- When mainstream external languages fit naturally, those slots may use tags such as `ts`, `re`,
  `zod`, `cypher`, or `pydantic`.
- When the constrained concept is native to gram itself (for example ranges, measurements, or
  tagged strings), the preferred direction is a `gram` tagged-string dialect whose content is a
  small gram-shaped vocabulary built from conventional labels and properties.
- Because gram does not allow patterns as direct property values, that explicit constraint
  structure must live inside the tagged-string content rather than as nested outer-schema values.
- Future validation strictness must distinguish at least two axes:
  - vocabulary openness (`open` vs `closed`)
  - composition openness (whether larger structures composed from valid archetypes are allowed)

This does **not** change the current v0.1 implementation target for `pato check`: in this branch,
`check` remains lint plus schema discovery/P007 suppression only. The exploration clarifies the
intended contract for future semantic validation so later work does not have to invent "schema"
from scratch.

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

- Implement `diagnostics.rs`: `Diagnostic`, `DiagnosticCode`, `Severity`, `Edit`, rule/remediation identifiers, occurrence parameters, and the shared rule registry
- Implement `diagnostic_gram.rs`: serialize diagnostics to gram/json per `contracts/diagnostic-gram.md` using a compact rule-driven report shape
- Implement `output.rs`: `OutputFormat` enum; gram (default), text (TTY-aware via `std::io::IsTerminal`), json rendering; text/comments derived from rule templates plus occurrence data
- **Tests**: Serialize one diagnostic of each grade (auto/guided/ambiguous/none) to gram; verify output parses with `relateby_gram::parse_gram`; verify JSON round-trips; verify optional comments do not affect canonical data

### Step 2b — Diagnostic Contract Realignment

- Update `contracts/diagnostic-gram.md` from the earlier nested/container-oriented examples to the adopted compact rule-driven schema
- Refactor completed diagnostic infrastructure work to match the compact occurrence model before adding further user stories
- Ensure `pato lint` emits stable rule/remediation identifiers and occurrence parameters rather than storing all human prose per instance
- Keep line/column locations as the stable public location contract; continue deriving them from CST spans
- **Tests**: confirm gram/json carry the same canonical facts, comments remain optional, and text output is rendered from structured data rather than stored strings

### Step 3 — `pato lint`

- Enable `relateby-gram`'s `cst` feature in `crates/pato/Cargo.toml`
- Add `source_map.rs` to convert CST byte spans into stable line/column locations and source slices
- Wire `parse_gram_cst` → P001 (guided; locations derived from `CstParseResult.errors`)
- Lower valid CST trees to semantic patterns only where semantic checks or serialization still need `Pattern<Subject>`
- Implement duplicate identity detection → P002 (guided; CST definition-site spans)
- Implement duplicate annotation key detection → P003 (guided; inspect `SyntaxNode.annotations`, not raw text)
- Implement label case warnings → P004 (auto; CST label spans plus relationship/node kind)
- Implement dangling reference warnings → P005 (ambiguous; CST reference sites + `strsim::levenshtein` for nearest candidate)
- Implement empty array detection → P006 (info, guided)
- Implement document kind validation → P008 (warning, guided; inspect CST document/header subject)
- Implement `editor.rs`: reverse-order edits, atomic writes (temp file + rename)
- Wire `--fix`: apply `auto` remediations via `editor.rs`; skip file entirely if any `ambiguous` in scope
- **Tests**: One fixture per code; add coverage for precise duplicate-identity spans, identified annotations, and comment-bearing files; verify rule/remediation identifiers and occurrence parameters; verify `--fix` produces clean-linting file; verify gram output is parseable

### Step 4 — `pato fmt`

- Implement canonical formatting rules as a CST-assisted rewrite pipeline (exhaustive `auto` remediations):
  - Consistent spacing around arrow families
  - Single blank line between top-level patterns
  - Properties sorted alphabetically within records
  - Document header at top of file
  - Arrow family and label separator preserved as-is
- Preserve top-level comments and source-order interleaving where practical; full trivia-preserving pretty-printing remains out of scope
- Implement `-` (stdin → stdout) and `--check` modes
- Idempotency: `fmt(fmt(x)) == fmt(x)` for all fixtures
- **Tests**: Before/after fixture pairs, including comments and identified annotations; `pato lint` reports zero `auto` diagnostics on all `pato fmt` output; idempotency property test

### Step 5 — `pato parse`

- Implement `gram` output (flat top-level sequence, no root wrapper) from lowered semantic patterns
- Implement `sexp` output directly from CST shape (tree-sitter sexp, matching gramref for shared fixtures)
- Implement `json` output (JSON array of `Pattern<Subject>` via lowered patterns / `AstPattern`)
- Implement `summary` output from CST-aware counts (nodes, rels, annotations, walks; comments may be reported additionally if useful)
- **Tests**: gram round-trip stability; sexp matches gramref for corpus fixtures; no root-wrapper nesting on repeated round-trips; CST-backed annotation counts are accurate

### Step 5b — `pato rule`

- Expose the shared rule/remediation registry: `DiagnosticCode` → name, description, grade, remediation templates, minimal trigger example
- Implement `pato rule` (no arg): emit gram file listing all rules and remediation templates as reusable knowledge
- Implement `pato rule <code>`: emit single `Rule` pattern with `TriggerExample` child and remediation template detail
- **Tests**: All P-codes have registry entries; gram output parses cleanly; lint output references valid registry identifiers

### Step 6 — `pato check`

- Compose lint + schema discovery
- Same-stem `*.schema.gram` discovery; `--schema` override
- P007 when no schema; suppress P007 + log schema path when schema found
- Keep semantic schema validation deferred until the archetypal schema contract and validation
  strictness modes are specified
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
relateby-gram    = { path = "../gram-codec",    version = "0.2", features = ["cst"] }
clap             = { workspace = true }
serde            = { workspace = true }
serde_json       = { workspace = true }
thiserror        = { workspace = true }
strsim           = { workspace = true }
```
