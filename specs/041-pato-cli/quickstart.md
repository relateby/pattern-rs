# Quickstart: pato Development

**Feature**: `041-pato-cli`
**Crate**: `crates/pato/`
**Binary**: `pato`

## Prerequisites

The pato crate depends on two existing workspace crates:
- `relateby-pattern` (`crates/pattern-core/`) — Pattern and Subject types
- `relateby-gram` (`crates/gram-codec/`) — Gram parsing and serialization

Both build cleanly. Verify:

```bash
cargo build --workspace
```

## Building pato

Once scaffolded:

```bash
# Build the pato binary
cargo build -p relateby-pato

# Run it
cargo run -p relateby-pato -- --version
cargo run -p relateby-pato -- lint my.gram
```

## Running Tests

```bash
# All pato tests
cargo test -p relateby-pato

# Specific test file
cargo test -p relateby-pato --test lint_tests

# With output
cargo test -p relateby-pato -- --nocapture
```

## Test Fixture Structure

```
crates/pato/tests/fixtures/
├── valid/           # Gram files with no diagnostics (expected: clean output, exit 0)
├── invalid/         # One fixture per P-code (P001.gram ... P008.gram)
└── schema/          # *.schema.gram files for check tests
```

Each `invalid/P00N.gram` file should contain exactly the minimal gram that triggers that code and nothing else.

## Development Sequence

Follow the implementation sequence from the proposal:

1. **Scaffold** — `crates/pato/Cargo.toml`, `main.rs`, `cli.rs`, extension dispatch
2. **Diagnostic infrastructure** — `diagnostics.rs`, `diagnostic_gram.rs`, `output.rs`
3. **`pato lint`** — wire P001–P008, `editor.rs`, `--fix`
4. **`pato fmt`** — canonical style, idempotency, `--check`
5. **`pato parse`** — gram/sexp/json/summary output
6. **`pato rule`** — rule registry
7. **`pato check`** — lint + schema discovery

## Verifying Diagnostic Gram Output

Diagnostic gram output must itself be parseable:

```bash
# Lint a file and verify the output parses cleanly
pato lint my.gram | pato parse -
```

Or in tests, use `relateby_gram::parse_gram` to verify the output string round-trips.

## Verifying sexp Output

The sexp output for `pato parse --output-format sexp` should match gramref output. Use shared fixtures from `crates/gram-codec/tests/corpus/` and compare against gramref:

```bash
gramref parse --value-only my.gram
pato parse --output-format sexp my.gram
```

## Code Quality Checks

Before marking any step complete:

```bash
cargo fmt --all -- --check     # or: cargo fmt --all
cargo clippy --workspace -- -D warnings
cargo test --workspace
./scripts/ci-local.sh
```

## Key Reference Files

| What | Where |
|------|-------|
| Gram parsing | `crates/gram-codec/src/lib.rs` — `parse_gram`, `parse_gram_with_header` |
| Gram serialization | `crates/gram-codec/src/serializer.rs` — `to_gram`, `to_gram_pattern` |
| Subject fields | `crates/pattern-core/src/subject.rs` — `identity`, `labels`, `properties` |
| Pattern structure | `crates/pattern-core/src/pattern.rs` — `elements`, `value` |
| sexp format reference | `crates/gram-codec/tests/corpus/validator.rs` |
| Diagnostic gram contract | `specs/041-pato-cli/contracts/diagnostic-gram.md` |
| CLI schema contract | `specs/041-pato-cli/contracts/cli-schema.md` |
