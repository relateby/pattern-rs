# Implementation Plan: Pure Rust Gram Parser

**Branch**: `021-pure-rust-parser` | **Date**: 2026-01-09 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/021-pure-rust-parser/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Replace the current tree-sitter-gram dependency (C code) with a pure Rust parser implementation based on nom parser combinators. The primary goal is to eliminate C dependencies that block seamless WASM and Python integration, enabling "just works" deployment to browsers, Node.js, and Python environments. The parser must achieve 100% conformance with the tree-sitter-gram test corpus to ensure correctness while providing comparable performance (within 20% of baseline). This migration adopts an immediate hard-switch strategy with no coexistence period.

## Technical Context

**Language/Version**: Rust 1.70.0+ (edition 2021)  
**Primary Dependencies**:
- **nom** (latest stable, ~7.x) - Parser combinator library for pure Rust parsing
- **wasm-bindgen** 0.2 - JavaScript bindings for WASM target (existing)
- **pyo3** 0.23 - Python bindings via Rust extension modules (existing)
- **pattern-core** - Internal crate defining Pattern data structures (existing)
- **thiserror** 2.0 - Error handling (existing)

**Dependencies to REMOVE**:
- **tree-sitter** 0.25 - C library dependency causing build complexity
- **tree-sitter-gram** 0.2 - Grammar binding with C code generation

**Storage**: N/A (parser library, no persistent storage)

**Testing**:
- **cargo test** - Unit and integration tests
- **insta** 1.0 - Snapshot testing for parse results
- **criterion** 0.5 - Performance benchmarking
- **proptest** 1.0 - Property-based testing for parser correctness
- **Custom test harness** - Runs tree-sitter-gram test corpus from `../tree-sitter-gram/test/corpus/`

**Target Platform**:
- **Native Rust**: x86_64, ARM (macOS, Linux, Windows)
- **WASM**: wasm32-unknown-unknown (browsers, Node.js)
- **Python**: PyO3 extension modules (Python 3.8+, macOS/Linux/Windows)

**Project Type**: Library (single crate in workspace)

**Performance Goals**:
- Parse 1000 patterns in <120ms (within 20% of tree-sitter-gram baseline ~100ms)
- Serialize round-trip overhead <10%
- WASM initialization <100ms

**Constraints**:
- **Zero C dependencies** - Must compile with pure Rust toolchain
- **WASM binary size** - <500KB gzipped
- **API compatibility** - Maintain existing gram-codec public API
- **100% test conformance** - Pass all tree-sitter-gram valid/invalid syntax tests
- **Error quality** - Provide location (line, column) and descriptive error messages

**Scale/Scope**:
- Support complete gram grammar (nodes, relationships, subject patterns, annotations, all value types)
- Handle deeply nested structures (100+ levels)
- Process files up to several megabytes
- ~50 tree-sitter-gram corpus test cases to validate

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity

**STATUS**: ⚠️ EXCEPTION - Requires Justification

**Assessment**: This feature intentionally deviates from the standard reference implementation approach. Instead of porting from `../gram-hs`, this feature treats `../tree-sitter-gram` as the authoritative specification.

**Justification**: 
- **Grammar Authority**: tree-sitter-gram defines the authoritative gram notation grammar via Tree-sitter's grammar DSL and comprehensive test corpus
- **Parser Independence**: gram-hs and gram-rs may use different parsing libraries (Parsec vs nom), but both must conform to the same grammar specification
- **Test-Driven Conformance**: Correctness is verified through 100% conformance with tree-sitter-gram test corpus (`../tree-sitter-gram/test/corpus/`), ensuring behavioral equivalence at the grammar level
- **Architectural Goal**: Eliminating the tree-sitter C dependency is the primary objective; the nom parser becomes the implementation, not a port of gram-hs parsing logic

**Verification Plan**:
1. Reference tree-sitter-gram grammar definition in `../tree-sitter-gram/src/grammar.json`
2. Pass 100% of test corpus cases in `../tree-sitter-gram/test/corpus/`
3. Document any intentional parsing behavior differences from gram-hs (if discovered during testing)
4. Maintain semantic equivalence: same Pattern structures produced for same gram notation input
5. Follow guidance from `../gram-hs/docs/reference/PORTING-GUIDE.md` for Phase 3 (Gram Serialization) implementation
6. Use round-trip testing strategy from `../gram-hs/docs/reference/features/gram-serialization.md`: `gram -> pattern -> gram -> pattern` for semantic equivalence

**Additional Context**:
- Per porting guide, Gram serialization (Phase 3) depends on Pattern (Phase 1) and Subject (Phase 2) being complete
- Pattern and Subject are already implemented in gram-rs; this feature focuses on the parser implementation
- gram-hs CLI tool can be used for conformance testing (generate test data, get canonical outputs)

**GATE DECISION**: ✅ **APPROVED** - Exception is justified and well-documented. Tree-sitter-gram serves as the grammar specification authority, with gram-hs providing implementation guidance.

---

### II. Correctness & Compatibility

**STATUS**: ✅ PASS

**Assessment**:
- **API Compatibility**: Maintains existing gram-codec public API (parse_gram, serialize_patterns, validate_gram functions)
- **Data Format Compatibility**: Produces identical Pattern data structures
- **Behavioral Guarantees**: 100% conformance with tree-sitter-gram test corpus ensures correct parsing behavior
- **Breaking Changes**: None - internal parser implementation change only

**Verification Plan**:
- Existing unit tests must pass without modification
- Snapshot tests verify identical parse results
- Round-trip tests ensure serialization compatibility
- WASM and Python bindings maintain same signatures

**GATE DECISION**: ✅ **PASS** - Correctness is prioritized through comprehensive test coverage.

---

### III. Rust Native Idioms

**STATUS**: ✅ PASS

**Assessment**:
- **nom parser combinators**: Idiomatic Rust parsing library
- **Zero-copy parsing**: Leverages Rust's borrowing for performance
- **Result<T, E> error handling**: nom integrates naturally with Rust error patterns
- **Type safety**: Parser combinators provide strong static type guarantees
- **Memory safety**: No unsafe code required for parsing

**Verification Plan**:
- Code review for idiomatic Rust patterns
- Clippy linting with `-D warnings`
- rustfmt formatting standards

**GATE DECISION**: ✅ **PASS** - nom is a canonical example of idiomatic Rust parsing.

---

### IV. Multi-Target Library Design

**STATUS**: ✅ PASS

**Assessment**:
- **Native Rust**: Primary target, full feature set
- **WASM (wasm32-unknown-unknown)**: Pure Rust enables seamless compilation
- **Python (PyO3)**: Pure Rust enables cross-platform Python extensions
- **No platform-specific code**: Parser logic is platform-agnostic
- **Feature flags**: Existing `wasm` and `python` features remain

**Verification Plan**:
- Build verification: `cargo build --target wasm32-unknown-unknown`
- WASM examples: Browser and Node.js tests
- Python wheel builds: macOS, Linux, Windows
- Conditional compilation: Verify feature flags work correctly

**GATE DECISION**: ✅ **PASS** - Removing C dependencies directly enables multi-target goals.

---

### V. External Language Bindings & Examples

**STATUS**: ✅ PASS (with required updates)

**Assessment**:
- **Existing examples**: `examples/gram-codec-wasm-web/`, `examples/gram-codec-wasm-node/`
- **Required updates**: Remove build complexity documentation, simplify to standard wasm-pack workflow
- **Python examples**: Need to create/update for PyO3 usage
- **Documentation**: Update README with simplified build instructions

**Verification Plan**:
- Update WASM web example to use simplified build
- Update WASM Node.js example to verify ES module imports
- Create Python usage example in `examples/gram-codec-python/`
- Update `examples/gram-codec-README.md` with new workflow
- Verify all examples compile and run

**GATE DECISION**: ✅ **PASS** - Examples are planned and will be updated as part of implementation.

---

### Additional Constraints

**Multi-Target Requirements**: ✅ PASS
- Public APIs already compatible with WASM (no blocking I/O, no filesystem)
- Pure Rust dependency (nom) supports all target platforms
- Build configuration supports conditional compilation (existing features)
- Testing plan includes all target platforms

**Compatibility Requirements**: ✅ PASS
- No API changes that break compatibility
- Semantic versioning: This is a minor version bump (internal implementation change)
- No migration guide needed (internal change, external API unchanged)

---

### Constitution Check Summary

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | ⚠️ EXCEPTION (Approved) | Using tree-sitter-gram as grammar authority instead of gram-hs |
| II. Correctness & Compatibility | ✅ PASS | API and behavior maintained |
| III. Rust Native Idioms | ✅ PASS | nom is idiomatic Rust |
| IV. Multi-Target Library Design | ✅ PASS | Pure Rust enables all targets |
| V. External Language Bindings | ✅ PASS | Examples planned for update |

**OVERALL GATE**: ✅ **APPROVED TO PROCEED** - All gates pass with one documented exception.

## Project Structure

### Documentation (this feature)

```text
specs/021-pure-rust-parser/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output: nom API patterns, error handling, corpus test parsing
├── data-model.md        # Phase 1 output: Parser state, AST nodes, error types
├── quickstart.md        # Phase 1 output: Migration guide and quick reference
├── contracts/           # Phase 1 output: Public API contracts
│   ├── parser-api.md    # parse_gram function contract
│   ├── serializer-api.md # serialize_patterns function contract
│   └── error-handling.md # Error types and recovery strategies
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/gram-codec/
├── src/
│   ├── lib.rs                    # Public API (parse_gram, serialize_patterns, validate_gram)
│   ├── parser/
│   │   ├── mod.rs               # Parser module exports
│   │   ├── combinators.rs       # Core nom combinators for gram grammar
│   │   ├── node.rs              # Node pattern parsing: (identifier:Label {props})
│   │   ├── relationship.rs      # Relationship parsing: (a)-->(b), arrow types
│   │   ├── subject.rs           # Subject pattern parsing: [id | elements]
│   │   ├── annotation.rs        # Annotation parsing: @key(value)
│   │   ├── value.rs             # Value type parsing: strings, numbers, booleans, arrays, ranges
│   │   ├── comment.rs           # Comment handling and whitespace
│   │   └── error.rs             # Parser error types, location tracking
│   ├── serializer/
│   │   ├── mod.rs               # Serializer module exports
│   │   ├── pattern.rs           # Pattern to gram notation (dispatch by element count)
│   │   ├── subject.rs           # Subject serialization (identifier, labels, properties)
│   │   ├── escape.rs            # Identifier and string escaping
│   │   └── format.rs            # Formatting utilities
│   ├── wasm.rs                  # WASM bindings (existing, no changes expected)
│   └── python.rs                # Python bindings (if separate from wasm, may need creation)
├── tests/
│   ├── parser_tests.rs          # Unit tests for parser combinators
│   ├── serializer_tests.rs      # Unit tests for serializer
│   ├── round_trip_tests.rs      # Round-trip correctness tests
│   └── corpus/
│       ├── mod.rs               # Test harness for tree-sitter-gram corpus
│       └── runner.rs            # Corpus test execution and reporting
├── benches/
│   └── codec_benchmarks.rs      # Performance benchmarks (existing, may need updates)
└── Cargo.toml                   # Update dependencies: add nom, remove tree-sitter*

tests/                           # Workspace-level integration tests
├── equivalence/                 # Existing cross-validation tests
└── corpus_integration/          # Integration with tree-sitter-gram corpus
    └── corpus_validation.rs     # Full corpus test suite integration

examples/
├── gram-codec/
│   ├── basic_usage.rs           # Update to show nom parser usage (no API changes)
│   └── advanced_usage.rs        # Update examples
├── gram-codec-wasm-web/
│   ├── index.html               # Update documentation for simplified build
│   ├── README.md                # Remove emscripten/LLVM complexity notes
│   └── package.json             # May be simplified or removed
├── gram-codec-wasm-node/
│   ├── index.js                 # Verify ES module imports work
│   ├── README.md                # Update with simplified workflow
│   └── package.json             # Verify configuration
└── gram-codec-python/           # NEW: Python usage example
    ├── example.py               # Basic parse/serialize usage
    ├── README.md                # Python installation and usage guide
    └── pyproject.toml           # Python project configuration (if needed)

scripts/
└── build-wasm/                  # Existing WASM build scripts
    ├── build-wasm.sh            # REMOVE or SIMPLIFY to basic wasm-pack
    └── check-prerequisites.sh   # REMOVE (no longer needed, only wasm-pack required)
```

**Structure Decision**: This is a library crate within the existing workspace structure. The gram-codec crate will be refactored with a clear separation between parser (nom-based) and serializer modules. The parser module is organized by gram grammar constructs (nodes, relationships, subjects, annotations, values), making it easy to map to the tree-sitter-gram grammar rules. Testing includes both unit tests within the crate and integration tests at the workspace level, with special focus on the tree-sitter-gram corpus validation.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**Status**: No violations requiring justification. The Constitution Check identified one approved exception (using tree-sitter-gram as grammar authority instead of gram-hs), which is documented in the Constitution Check section above. This exception is intentional and well-justified by the architectural goals of this feature.
