# Copilot Instructions for pattern-rs

## Project Overview

pattern-rs is a multi-language library workspace providing a Pattern data structure and Gram notation codec. It compiles for native Rust, WebAssembly, TypeScript, and Python (via PyO3).

## Workspace Structure

The workspace has these Rust crates and adapter packages. Use the **package name** (from `Cargo.toml [package].name`), not the directory name:

| Directory | Package Name | Description |
|-----------|-------------|-------------|
| `crates/pattern-core/` | `relateby-pattern` | Core Pattern, Subject, StandardGraph types |
| `crates/gram-codec/` | `relateby-gram` | Gram notation parser/serializer |
| `adapters/wasm/pattern-wasm/` | `pattern-wasm` | WASM bindings |

When writing cargo commands, use the package name: `cargo test -p relateby-pattern`, `cargo run --package relateby-pattern --example standard_graph_usage`.

The public non-Rust package roots live in:

- `typescript/packages/pattern/`
- `typescript/packages/graph/`
- `typescript/packages/gram/`
- `python/packages/relateby/`

For Python work, prefer `uv` with a local `.venv` inside `python/packages/relateby/`.

## Key Types

- **`StandardGraph`** (`crates/pattern-core/src/graph/standard.rs`): Ergonomic zero-config graph wrapping `PatternGraph<(), Subject>`. Recommended entry point for graph construction.
- **`SubjectBuilder`** (`crates/pattern-core/src/subject.rs`): Fluent builder created via `Subject::build("id")`. Lives alongside `Subject`, `Symbol`, and `Value` types.
- **`Pattern<V>`** (`crates/pattern-core/src/pattern.rs`): Core recursive data structure, generic over value type.
- **`PatternGraph<Extra, V>`** (`crates/pattern-core/src/pattern_graph.rs`): Abstract classified graph container. StandardGraph wraps `PatternGraph<(), Subject>`.
- **`FromGram` trait** (`crates/gram-codec/src/standard_graph.rs`): Extension trait in gram-codec (not pattern-core) due to dependency direction. Provides `StandardGraph::from_gram()`.

## Architecture Constraints

- **Dependency direction**: `gram-codec` depends on `pattern-core`, not vice versa. Anything requiring both must live in gram-codec or a higher-level crate.
- **WASM compatibility**: All public APIs must avoid blocking I/O and file system access. Verify with `cargo build --target wasm32-unknown-unknown`.
- **No new external crates** without discussion. The workspace uses only `std` collections (`HashMap`, `HashSet`, `Vec`, `BTreeMap`, `Rc`, `Arc`).
- **MSRV**: Rust 1.70.0, Edition 2021.

## Coding Conventions

- Write idiomatic Rust, not translated Haskell (the project ports from a Haskell reference implementation).
- `From`/`Into` trait implementations belong next to the type they convert *to* or *from* (e.g., `From<&str> for Symbol` lives in `subject.rs` near `Symbol`).
- StandardGraph's atomic methods (`add_node`, `add_relationship`, etc.) are infallible and return `&mut Self` for chaining. Only `from_gram` is fallible (`Result`).
- Placeholder nodes created by `add_relationship` or `add_annotation` are inserted into `pg_nodes` for discoverability.
- `is_empty()` checks all six buckets including `pg_conflicts` and `pg_other`.

## Testing

- Run all tests: `cargo test --workspace`
- Run StandardGraph tests: `cargo test -p relateby-pattern --test standard_graph_tests`
- Full CI check: `./scripts/ci-local.sh` (format, clippy, build, WASM, TypeScript, Python, tests)
- Validate gram notation: `gram-lint -e "expression"` or `gram-lint path/to/file.gram`

## Code Quality

Before suggesting changes, run:
```bash
cargo fmt --all
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

## What Not To Do

- Do not add features, refactor code, or make improvements beyond what was asked.
- Do not add docstrings, comments, or type annotations to code you didn't change.
- Do not create documentation files unless explicitly requested.
- Do not add error handling for scenarios that can't happen.
