# Research: pato CLI Tool

**Phase**: 0 — Research
**Branch**: `041-pato-cli`
**Date**: 2026-03-18

## Decision 1: TTY Detection for `--output-format text`

**Decision**: Use `std::io::IsTerminal` from the Rust standard library.

**Rationale**: Available since Rust 1.70.0 — exactly the workspace MSRV. Zero external dependencies. Provides `.is_terminal()` on `std::io::Stdout`. The idiomatic, modern approach backed by the Rust stdlib team.

**Alternatives considered**:
- `is-terminal` crate — fine, but unnecessary when stdlib covers it at MSRV
- `atty` crate — legacy, being deprecated in favor of stdlib approach

**Usage**:
```rust
use std::io::IsTerminal;
let use_color = std::io::stdout().is_terminal();
```

---

## Decision 2: Edit Distance for P005 (Dangling Reference Suggestions)

**Decision**: Add `strsim` v0.11 as a workspace dependency.

**Rationale**: `strsim` is the de facto standard for string similarity in Rust. Provides Levenshtein distance (and Jaro-Winkler, Damerau-Levenshtein if needed). Production-proven, minimal footprint, compatible with MSRV 1.70.0.

**Alternatives considered**:
- `edit-distance` crate — single-purpose, less flexible
- Hand-rolled Levenshtein — not worth the maintenance burden for v0.1

**Usage pattern for P005 candidate suggestion**:
```rust
use strsim::levenshtein;

let threshold = (misspelled.len() / 2).max(1);
let mut candidates: Vec<_> = defined_identities.iter()
    .map(|id| (id, levenshtein(misspelled, id)))
    .filter(|(_, dist)| *dist <= threshold)
    .collect();
candidates.sort_by_key(|(_, dist)| *dist);
```

---

## Decision 3: External Subcommand Dispatch (pato-foo)

**Decision**: Use clap v4's `allow_external_subcommands` attribute, combined with manual PATH scanning for help text discovery.

**Rationale**: Clap v4 has built-in support for external subcommand patterns. The External variant captures the subcommand name and all remaining arguments. Uses `std::process::Command` with inherited streams (not a shell) and relays exit code verbatim.

**Alternatives considered**:
- Manual `std::env::args()` parsing — fragile, redundant with clap
- Shell dispatch via `sh -c` — violates the proposal's "exec'd directly, not via shell" requirement

**Key pattern**:
```rust
#[derive(Subcommand)]
enum Commands {
    Lint { /* ... */ },
    // ...
    #[command(external_subcommand)]
    External(Vec<String>),
}

// In dispatch:
Commands::External(args) => {
    let bin = format!("pato-{}", args[0]);
    let status = std::process::Command::new(&bin)
        .args(&args[1..])
        .status()
        .unwrap_or_else(|_| /* emit error, exit 3 */);
    std::process::exit(status.code().unwrap_or(3));
}
```

---

## Decision 4: sexp Output Format for `pato parse`

**Decision**: Use tree-sitter-gram sexp format, matching gramref output. Implement as a new `to_sexp` function in `pato` using the existing `AstPattern` from gram-codec.

**Rationale**: The proposal states sexp output should match `gram-lint` / `gramref` output. The gramref tool uses tree-sitter sexp notation. The gram-codec test corpus validator already parses this format for conformance checks, providing a reference for the expected output shape.

**Format shape** (from gram-codec corpus validator):
```sexp
(gram_pattern
  (node_pattern
    identifier: (symbol)
    labels: (label_name_sequence
      (label_name))
    record: (record
      (property
        name: (property_name)
        value: (string_literal)))))
```

**Alternatives considered**:
- Custom/minimal sexp format — diverges from gramref reference; harder to compare outputs
- Reuse existing `AstPattern` JSON and translate — adds indirection; sexp is simpler to generate directly from `Pattern<Subject>`

**Implementation note**: The `AstPattern` type in gram-codec (`src/ast.rs`) is the right structural basis. The sexp serializer lives in `pato` as it is an output format concern, not a codec concern.

---

## Decision 5: pato Relationship to gram-hs

**Decision**: pato is a net-new tool with no gram-hs equivalent. Reference fidelity applies only to output format compatibility (sexp matches gramref, gram round-trips via relateby-gram).

**Rationale**: gram-hs has `gramref` (a conformance testing tool) but no developer-facing lint/format/check CLI. The diagnostic codes (P001–P008), remediation grades, and canonical formatting rules are original to pattern-rs / pato. This is an intentional design decision, not an accidental omission from the port.

**Documented deviation**: Per constitution Principle I, this is a justified net-new feature. The only areas requiring gram-hs behavioral equivalence are:
- `pato parse --output-format gram` — must round-trip correctly through `relateby-gram`
- `pato parse --output-format sexp` — must match gramref sexp output for shared fixtures

---

## Decision 6: Workspace Dependency Changes

The following additions to `Cargo.toml` workspace dependencies are required:

```toml
# New workspace dependencies
strsim  = "0.11"   # string similarity for P005 edit distance

# New workspace member
# crates/pato — the pato binary crate
```

The following are already in workspace dependencies and can be reused:
- `serde`, `serde_json` — JSON output for diagnostics
- `thiserror` — error type definitions
- `clap` — NOT yet in workspace deps (used in external tree-sitter tool only); add as workspace dep with `derive` feature

```toml
clap = { version = "4", features = ["derive"] }
```

No async runtime. No `walkdir` for v0.1 (glob expansion is shell-delegated; single-file and explicit-file-list mode only).

---

## Key Source Files for Implementation Reference

| Purpose | Path |
|---------|------|
| Gram parsing API | `crates/gram-codec/src/lib.rs` |
| Gram serializer | `crates/gram-codec/src/serializer.rs` |
| AST types | `crates/gram-codec/src/ast.rs` |
| Subject type | `crates/pattern-core/src/subject.rs` |
| Pattern type | `crates/pattern-core/src/pattern.rs` |
| Existing test fixtures | `crates/gram-codec/tests/serializer_tests.rs` |
| Sexp format reference | `crates/gram-codec/tests/corpus/validator.rs` |
| gramref sexp tests | `../pattern-hs/apps/gramref-cli/tests/` |
