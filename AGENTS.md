# AGENTS.md

## Cursor Cloud specific instructions

### Overview

pattern-rs is a pure Rust library (no servers, databases, or network services). The workspace contains two main crates (`relateby-pattern`, `relateby-gram`), a WASM crate (`pattern-wasm`), and benchmarks.

### Rust toolchain

The pre-installed Rust 1.83.0 is too old for some transitive dependencies (e.g. `getrandom v0.4` requires edition2024 / Rust 1.85+). The update script runs `rustup update stable && rustup default stable` to ensure a compatible version. The WASM target (`wasm32-unknown-unknown`) is also installed by the update script.

### Git submodule

The `external/tree-sitter-gram` submodule must be initialized for the corpus integration tests in `gram-codec` to pass. The update script handles this via `git submodule update --init`.

### gram-lint CLI

The `gram-lint` tool (for validating gram notation syntax) is installed from the submodule by the update script. Usage examples:

- `gram-lint -e '(a)-->(b)'` — lint an expression (exit 0 = valid)
- `gram-lint -t -e '(a)-->(b)'` — show the parse tree
- `gram-lint path/to/file.gram` — lint a file

### Running checks

All CI checks are documented in `CLAUDE.md` and `scripts/ci-local.sh`. The key commands:

- `cargo fmt --all -- --check` — format check
- `cargo clippy --workspace -- -D warnings` — lint
- `cargo build --workspace` — native build
- `cargo test --workspace` — all tests
- `cargo build --workspace --target wasm32-unknown-unknown` — WASM build (optional, non-blocking in CI)
- `./scripts/ci-local.sh` — runs all of the above plus optional Python/TypeScript checks

### Python and TypeScript bindings

Python (PyO3/maturin) and TypeScript (wasm-pack) bindings are optional and marked `continue-on-error` in CI. They are not required for core development. See `CLAUDE.md` for build commands if needed.

### Examples

Two runnable examples demonstrate core functionality:

- `cargo run --example comonad_usage`
- `cargo run --example paramorphism_usage`
