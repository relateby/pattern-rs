# Implementation Plan: CST-Preserving Gram Parser

**Branch**: `042-gram-cst-parser` | **Date**: 2026-03-19 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/042-gram-cst-parser/spec.md`

## Summary

Add a syntax-preserving gram parser, parallel to the existing nom-based semantic parser, that
produces `Pattern<SyntaxNode>` — a `Pattern<T>` tree retaining source spans, arrow kinds,
annotation content, and comment nodes. A `lower()` function maps this representation to
`Pattern<Subject>` for compatibility with all existing consumers. The parser is backed by
`tree-sitter-gram` and gated behind a `cst` feature flag to preserve WASM compatibility for the
existing parser surface.

## Technical Context

**Language/Version**: Rust 1.70.0 (workspace MSRV), Edition 2021
**Primary Dependencies**: `tree-sitter` 0.25, `tree-sitter-language` 0.1, `tree-sitter-gram` (via path: `external/tree-sitter-gram`, optional under `cst` feature), `pattern-core` (workspace), `nom` (retained for existing parser)
**Storage**: N/A — in-memory only
**Testing**: `cargo test`, `insta` snapshots for syntax tree shape, equivalence corpus against existing nom parser output
**Target Platform**: Native Rust only; WASM explicitly excluded for this feature (see Complexity Tracking)
**Project Type**: Library — new module in `gram-codec`, behind `cst` feature flag
**Performance Goals**: Not specified; must not regress existing `parse_gram` throughput for callers not using the `cst` feature
**Constraints**: Must not change existing public API; `cst` feature flag must be off by default; tree-sitter C runtime must not appear in non-`cst` builds
**Scale/Scope**: Handles any valid gram file; no document-size limits beyond available memory

## Phase 0: Nom Parser Alignment (Prerequisite)

Before introducing the new CST parser, the existing nom-based parser must be aligned with the
tree-sitter-gram v0.3.4 grammar. The corpus conformance test (`test_corpus_conformance`) currently
reports **90.1% pass rate (128/142 tests)** with 14 failures across 3 distinct root causes:

### Cause A — Corpus runner silently drops `:error` tests (~17 tests)

The corpus test runner's parser does not handle the `:error` flag on test headers. Tests marked
`:error` in the corpus (e.g., all tests in `node_annotations.txt` and
`relationship_annotations.txt`) are being silently skipped with a warning rather than run.

**Fix**: Update `crates/gram-codec/tests/corpus/parser.rs` to recognise the `:error` flag.
`:error` tests should be collected, marked as `ExpectedError`, and skipped during nom-parser
conformance checking (the nom parser is not required to replicate tree-sitter's error-recovery
behaviour). The test count should reflect that these are known-skipped, not silently dropped.

### Cause B — Validator miscounts multi-pattern documents (9 failures)

`count_gram_patterns` in `validator.rs` counts the number of `(gram_pattern` occurrences in the
S-expression string. But the S-expression always has a single outer `(gram_pattern` document
wrapper containing N child patterns. When the nom parser returns N separate `Pattern<Subject>`
values, the validator gets `expected=1, actual=N` and fails.

Affected tests: `empty_nodes.txt` (2 nodes), `empty_relationships.txt` (2 relationships),
`identifiers.txt` (2 nodes), `records.txt` (record + node), `comments.txt` (2 cases),
`graph_global.txt` (3 cases).

**Fix**: Update `count_gram_patterns` to count top-level named pattern children inside the single
`(gram_pattern` root (i.e., count occurrences of `node_pattern`, `relationship_pattern`,
`subject_pattern`, `annotated_pattern`, `comment` at the first indent level), not the number of
`(gram_pattern` wrappers.

### Cause C — `@@` identified annotation syntax not supported (5 failures)

The nom parser cannot parse `@@id`, `@@:label`, or `@@id:label` syntax and returns a parse error.
Affected tests are all in `extended_annotations.txt`.

**Fix**: Add an `identified_annotation` combinator to `crates/gram-codec/src/parser/annotation.rs`:
parse `@@` prefix, then optional identifier, then optional label sequence (`:symbol` one or more
times). Update the `annotations` combinator to accept an optional `identified_annotation` followed
by zero or more `property_annotation` entries (matching the v0.3.4 grammar). The annotation
content is still dropped when constructing `Pattern<Subject>` (the pre-existing TODO); the goal
is only to make the parser accept the syntax without error.

### Target

After Phase 0, `test_corpus_conformance` should report **100% pass rate on all runnable tests**,
with `:error` tests explicitly marked as skipped.

---

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | ✓ Pass | This feature is a new capability with no gram-hs counterpart; tree-sitter-gram grammar is the authoritative reference |
| II. Correctness & Compatibility | ✓ Pass | Lowering equivalence test (SC-002) enforces compatibility; existing `parse_gram` API is unchanged |
| III. Rust Native Idioms | ✓ Pass | `Result<T, E>`, enums, ownership patterns; no Haskell-isms required |
| IV. Multi-Target Library Design | ⚠ Justified Violation | tree-sitter uses a C runtime incompatible with `wasm32-unknown-unknown`; mitigated by `cst` feature flag (off by default); WASM support deferred per spec clarification Q2 |
| V. External Language Bindings & Examples | ✓ Pass | A minimal Rust usage example for `parse_gram_cst` will be included |

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| Principle IV: native-only `cst` feature | tree-sitter's C runtime (`parser.c`) cannot compile to `wasm32-unknown-unknown` without a WASM-specific tree-sitter port | A pure-Rust CST parser would require re-implementing the grammar, which is significantly more work and risks divergence from the authoritative tree-sitter-gram grammar |

## Project Structure

### Documentation (this feature)

```text
specs/042-gram-cst-parser/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── api.md
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
crates/gram-codec/
├── Cargo.toml                        # Add tree-sitter, tree-sitter-gram under [cst] feature
└── src/
    ├── lib.rs                        # Re-export cst module and parse_gram_cst() under #[cfg(feature="cst")]
    ├── transform.rs                  # Existing dormant file; retained as reference, not modified
    └── cst/
        ├── mod.rs                    # Module exports: SyntaxNode, SyntaxKind, SourceSpan, CstParseResult, parse_gram_cst, lower
        ├── syntax_node.rs            # SyntaxNode, SyntaxKind, SourceSpan types
        ├── parser.rs                 # parse_gram_cst() — tree-sitter parsing → Pattern<SyntaxNode>
        └── lowering.rs               # lower() — Pattern<SyntaxNode> → Pattern<Subject> (drops comments, strips syntax)

crates/gram-codec/src/parser/
└── annotation.rs                     # Phase 0: add identified_annotation combinator, fix annotations sequence

crates/gram-codec/tests/
├── corpus/
│   ├── parser.rs                     # Phase 0: add :error flag handling
│   └── validator.rs                  # Phase 0: fix count_gram_patterns for multi-pattern documents
└── cst/
    ├── mod.rs
    ├── parse_tests.rs                # Fixture parse, verify spans/arrow kinds/annotations/comments
    ├── lowering_tests.rs             # Compare lower(parse_gram_cst(s)) to parse_gram(s)
    ├── comment_tests.rs              # Comment node presence, text, span, source-order interleaving
    └── error_recovery_tests.rs       # Partial tree + error spans on malformed input

examples/rust/
└── cst_parse.rs                      # Minimal example: parse gram string, access syntax tree
```

**Structure Decision**: Single-project layout extending the existing `gram-codec` crate. New
functionality lives in `src/cst/` behind the `cst` Cargo feature. No new workspace member is
added. The existing `transform.rs` is left dormant but unreferenced — it serves as reference
material during implementation and may be removed in a follow-on cleanup.
