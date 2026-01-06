# Implementation Plan: Basic Gram Codec

**Branch**: `019-gram-codec` | **Date**: 2026-01-06 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/019-gram-codec/spec.md`

## Summary

Implement bidirectional transformation between gram notation (human-readable text format) and Pattern data structures. The codec provides parsing (gram text → Pattern) and serialization (Pattern → gram text) with full support for all gram syntax forms defined in the tree-sitter-gram grammar. This enables loading patterns from gram files, accepting gram notation as input, and outputting patterns in human-readable form.

**Key Requirements**:
- Parse all gram syntax forms: nodes, relationships, subject patterns, annotations
- Serialize patterns to valid gram notation
- Round-trip correctness (parse → serialize → parse produces equivalent pattern)
- WASM and Python support
- Error recovery (report all syntax errors, not just first)
- Validation using `gram-lint` CLI tool

**Technical Approach**:
- Use tree-sitter-gram bindings directly for parsing (leverages authoritative grammar)
- Transform tree-sitter CST (concrete syntax tree) to Pattern AST
- Implement serializer with format selection logic based on pattern structure
- Define Value enum for heterogeneous property types
- Test against tree-sitter-gram test corpus (27 test files)

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)  
**Primary Dependencies**: 
- `tree-sitter` (parser runtime)
- `tree-sitter-gram` (grammar and bindings from `external/tree-sitter-gram` git submodule)
- `pattern-core` (Pattern and Subject types)
- `gram-lint` CLI for validation

**Storage**: N/A (in-memory transformations)  
**Testing**: `cargo test`, `gram-lint` validation, tree-sitter-gram test corpus  
**Target Platform**: Native Rust (x86_64, ARM), WebAssembly (WASM), Python bindings  
**Project Type**: Library crate (`crates/gram-codec/`)  
**Performance Goals**: 
- Parse/serialize 1000-node patterns in <100ms (native), <200ms (WASM)
- O(n) time complexity for n nodes
- WASM binary <500KB compressed

**Constraints**: 
- Must validate all gram notation with `gram-lint` (exit code 0)
- WASM-compatible (no blocking I/O, no file system access)
- Python integration required (via PyO3 or WASM)
- Round-trip correctness mandatory

**Scale/Scope**: 
- Support all 27 gram syntax forms (from tree-sitter-gram test corpus)
- Handle patterns up to 1000 nodes, 100 nesting levels
- 40+ validated test cases from VALIDATION.md

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Principle I: Reference Implementation Fidelity

**⚠️ DEVIATION - EXPLICITLY JUSTIFIED**

This feature uses **`tree-sitter-gram` as the authoritative reference**, NOT `../gram-hs`.

**Rationale**: The tree-sitter-gram repository contains the canonical grammar definition for gram notation. For parsing and serialization, the grammar is the source of truth, not any specific implementation. The tree-sitter-gram grammar is:
- Maintained separately as a language-agnostic specification
- Used by multiple implementations (Rust, JavaScript, Python, Go, C, Swift)
- Tested with a comprehensive test corpus (27 test files)
- Validated by the `gram-lint` CLI tool

**Reference Locations**:
- **Grammar**: `external/tree-sitter-gram/grammar.js` (authoritative, git submodule)
- **Test Corpus**: `external/tree-sitter-gram/test/corpus/` (27 test files, git submodule)
- **Validator**: `gram-lint` CLI tool (`~/.cargo/bin/gram-lint`)
- **Submodule Setup**: `git submodule update --init --recursive` (see CORPUS_TESTING.md)
- **Validation Documentation**: [VALIDATION.md](VALIDATION.md) (40+ validated examples)
- **Parse Tree Analysis**: [research.md](research.md#11-parse-tree-structure-analysis)

**Verification Strategy**:
1. All gram notation must pass `gram-lint` validation (exit code 0)
2. Parser must handle all test cases from tree-sitter-gram test corpus
3. Parse trees must match tree-sitter-gram parser output
4. Round-trip correctness: parse → serialize → parse produces equivalent Pattern
5. Cross-reference with gram-hs only for non-grammar aspects (if needed)

### Principle II: Correctness & Compatibility (NON-NEGOTIABLE)

✅ **COMPLIANT**

- All implementations prioritize correctness (comprehensive validation with `gram-lint`)
- Compatibility with tree-sitter-gram grammar is mandatory (authoritative for this feature)
- Round-trip correctness verified for all test cases
- No breaking changes from tree-sitter-gram grammar semantics

**Verification**:
- 100% of test cases from tree-sitter-gram corpus must pass
- 100% of serialized output must pass `gram-lint` validation
- Round-trip correctness verified in 100% of cases

### Principle III: Rust Native Idioms

✅ **COMPLIANT**

- Use `Result<T, E>` for error handling (parse errors, serialization errors)
- Rust naming conventions: `parse_gram_notation()`, `serialize_pattern()`
- Value enum for heterogeneous property types (idiomatic Rust pattern)
- Use `HashMap<String, Value>` for properties (standard Rust collections)
- Leverage Rust's type system for pattern structure (0, 1, 2, N elements)

**Design Decisions**:
- Parser returns `Result<Vec<Pattern<Subject>>, ParseError>`
- ParseError includes location information (line, column) and error messages
- Serializer returns `Result<String, SerializeError>`
- Value enum: `String`, `Integer`, `Decimal`, `Boolean`, `Array`, `Range`, `TaggedString`

### Principle IV: Multi-Target Library Design

✅ **COMPLIANT**

- Library compiles to native Rust and WASM
- No platform-specific code paths (in-memory transformations only)
- tree-sitter runtime supports both native and WASM targets
- WASM binary size target: <500KB compressed

**Platform Support**:
- Native Rust: x86_64, ARM (primary target)
- WASM: Browser and Node.js (for JavaScript/TypeScript usage)
- Python: Via PyO3 bindings or WASM runtime

**Feature Flags** (if needed):
- `wasm`: WASM-specific optimizations
- `python`: PyO3 bindings

### Principle V: External Language Bindings & Examples

✅ **COMPLIANT**

- WASM bindings for JavaScript/TypeScript (via `wasm-bindgen`)
- Python bindings (via PyO3 or WASM)
- Examples demonstrating codec usage from external languages
- Integration with existing `examples/wasm-js/` structure

**Examples to Create**:
- Rust: `examples/gram-codec-rust.rs` (parse/serialize demo)
- WASM/JavaScript: Extend `examples/wasm-js/` with codec functions
- Python: `examples/python/gram_codec.py` (parse/serialize demo)

### Multi-Target Requirements

✅ **COMPLIANT**

- No blocking I/O (in-memory transformations)
- No file system access (operates on strings)
- tree-sitter dependency supports WASM
- Testing on native Rust and WASM targets

### Compatibility Requirements

✅ **COMPLIANT**

- API compatibility with tree-sitter-gram grammar (authoritative)
- Semantic versioning for any API changes
- Documentation of any intentional deviations (noted in clarifications)

**Intentional Design Decisions** (from clarifications):
- Error recovery: Report all syntax errors, not just first (better DX)
- Empty input: Return empty collection (valid but no patterns)
- 2-element serialization: Relationship notation when both elements atomic
- Property values: Value enum with all gram types
- Arrow types: Inferred from element ordering (syntactic sugar)

### Constitution Summary

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | ⚠️ **VIOLATION** (Justified) | Uses tree-sitter-gram, not gram-hs (explicit requirement) |
| II. Correctness & Compatibility | ✅ Compliant | Round-trip correctness, comprehensive validation |
| III. Rust Native Idioms | ✅ Compliant | Result types, Value enum, idiomatic patterns |
| IV. Multi-Target Library Design | ✅ Compliant | Native Rust + WASM, Python bindings |
| V. External Language Bindings | ✅ Compliant | WASM/JS, Python examples planned |

**Constitutional Violation - Principle I**: This feature violates the requirement to use `gram-hs` as the authoritative reference implementation.

**Justification for Deviation**:
1. **No authoritative implementation exists**: `gram-hs` does not implement gram notation parsing/serialization
2. **Grammar is more authoritative than implementation**: `tree-sitter-gram` defines the grammar specification itself
3. **Feature specification explicitly requires**: User requirement states "using `../tree-sitter-gram` as the standard grammar reference"
4. **Multi-platform requirement**: tree-sitter provides native Rust, WASM, and Python support
5. **Comprehensive validation**: `gram-lint` CLI uses tree-sitter-gram as validation tool
6. **Test corpus availability**: `../tree-sitter-gram/test/corpus/` provides authoritative test cases

**Overall Assessment**: ✅ **APPROVED** - Justified violation of Principle I. The deviation is necessary, well-justified, and maintains the spirit of the constitution (correctness and fidelity to gram specification).

## Project Structure

### Documentation (this feature)

```text
specs/019-gram-codec/
├── spec.md                    # Feature specification (complete, 177 lines)
├── plan.md                    # This file (implementation plan)
├── research.md                # Phase 0: Technology decisions (372 lines, partially complete)
├── data-model.md              # Phase 1: Data model (156 lines, partially complete)
├── quickstart.md              # Phase 1: Quick reference (203 lines, partially complete)
├── contracts/                 # Phase 1: API contracts (to be created)
├── tasks.md                   # Phase 2: Task breakdown (created by /speckit.tasks)
├── README.md                  # Feature overview (231 lines, complete)
├── VALIDATION.md              # Validation documentation (409 lines, complete)
├── CLARIFICATION_SUMMARY.md   # Clarification session (complete)
└── checklists/
    └── requirements.md        # Quality checklist (complete)
```

### Source Code (repository root)

**Structure Decision**: Single library crate in workspace

```text
crates/gram-codec/
├── Cargo.toml                 # Crate configuration
├── src/
│   ├── lib.rs                 # Public API exports
│   ├── parser.rs              # Parser: gram notation → Pattern
│   ├── serializer.rs          # Serializer: Pattern → gram notation
│   ├── value.rs               # Value enum for property types
│   ├── error.rs               # Error types (ParseError, SerializeError)
│   └── transform.rs           # CST → Pattern transformation
├── tests/
│   ├── parser_tests.rs        # Parser unit tests
│   ├── serializer_tests.rs    # Serializer unit tests
│   ├── roundtrip_tests.rs     # Round-trip correctness tests
│   └── corpus_tests.rs        # Tree-sitter-gram corpus integration
└── benches/
    └── codec_benchmarks.rs    # Performance benchmarks

tests/
├── integration/
│   └── gram_codec_integration.rs  # End-to-end integration tests
└── corpus/                    # Tree-sitter-gram test corpus (to be incorporated)

examples/
├── gram_codec_usage.rs        # Rust usage example
├── wasm-js/
│   └── [extend with codec]    # WASM/JavaScript example
└── python/
    └── gram_codec.py          # Python usage example (if PyO3)
```

**Integration Points**:
- `crates/pattern-core/`: Uses Pattern<V> and Subject types
- `external/tree-sitter-gram/`: Grammar, test corpus, Rust bindings (git submodule)
- `gram-lint`: Validation tool for all codec output
- See [CORPUS_TESTING.md](CORPUS_TESTING.md) for submodule setup

## Complexity Tracking

> **Constitutional Deviation**: This feature VIOLATES Principle I (Reference Implementation Fidelity) by using `tree-sitter-gram` instead of `gram-hs` as the authoritative grammar reference.
> 
> **Justification**: The feature specification explicitly requires this deviation. Unlike other features, gram-hs does not have an authoritative gram notation parser/serializer implementation. The tree-sitter-gram repository is the definitive grammar specification, maintained by the same team, with comprehensive test corpus and multi-language bindings. Using tree-sitter-gram ensures:
> - Correctness: Grammar is the source of truth, not an implementation
> - Validation: `gram-lint` CLI provides authoritative validation
> - Test coverage: Comprehensive test corpus in `../tree-sitter-gram/test/corpus/`
> - Multi-platform: Native tree-sitter support for Rust, WASM, Python
> 
> **Approval**: See Constitution Summary (lines 170-180) for APPROVED justified deviation.

| Aspect | Complexity | Justification |
|--------|------------|---------------|
| Parser library choice | Medium | Using tree-sitter-gram directly (authoritative grammar) vs manual implementation |
| Value enum design | Low | Standard Rust pattern for heterogeneous data (like serde_json::Value) |
| Arrow type handling | Medium | Syntactic sugar - needs clear rules for each arrow variation |
| Subject refactoring | Medium | May need to refactor existing Subject to integrate Value enum without redundancy |
| Test corpus integration | Low | **RESOLVED**: Git submodule approach for CI/CD compatibility (see CORPUS_TESTING.md) |

**Mitigation Strategies**:
- **Parser library**: Research phase will evaluate tree-sitter-gram direct use vs manual port
- **Value enum**: Standard pattern, well-documented in Rust ecosystem
- **Arrow types**: Clarification phase identified need for explicit rules (planning phase will specify)
- **Subject refactoring**: Review existing implementation, design unified Value enum
- **Test corpus**: **RESOLVED** - Git submodule at `external/tree-sitter-gram/` for CI/CD compatibility

---

## Phase 0: Research & Technology Decisions

**Status**: Partially complete (research.md exists, needs updates)

See [research.md](research.md) for detailed research findings.

**Research Questions** (from research.md):
1. ✅ Parser library selection (tree-sitter vs winnow vs pest vs chumsky)
2. ✅ Tree-sitter grammar direct use (feasibility, API, transformation)
3. ⚠️ **Arrow type handling** - NEEDS DETAILED SPECIFICATION
4. ⏸️ AST vs direct Pattern construction (to be decided)
5. ⏸️ Error recovery strategy (clarified: report all errors)
6. ⏸️ Serializer format strategy (clarified: relationship notation when both atomic)
7. ⚠️ **Property type mapping** - NEEDS Value enum design
8. ⏸️ Python binding strategy (PyO3 vs WASM)
9. ⏸️ Streaming vs batch parsing (batch initially)
10. ⏸️ Comment preservation (discard initially)
11. ✅ Parse tree structure analysis (complete in VALIDATION.md)

**Key Decisions Needed**:
1. **Arrow Type Representation** (HIGH PRIORITY)
   - How to map each arrow type to element ordering
   - Right arrow `(a)-->(b)`: Preserve order (left → right) ✅
   - Left arrow `(a)<--(b)`: Reverse elements? Keep order with metadata?
   - Bidirectional `(a)<-->(b)`: Symmetric - arbitrary order? Metadata?
   - Undirected `(a)~~(b)`: Symmetric - arbitrary order? Metadata?

2. **Value Enum Design** (HIGH PRIORITY)
   - Complete enum definition with all gram types
   - Integration with existing Subject implementation
   - Avoid redundant value representation

3. **Tree-sitter Integration** (MEDIUM PRIORITY)
   - Confirm tree-sitter-gram Rust bindings work in WASM
   - Design CST → Pattern transformation approach
   - Error message extraction from tree-sitter errors

4. **Test Corpus Integration** ✅ **RESOLVED**
   - **Decision**: Git submodule at `external/tree-sitter-gram/`
   - **Benefits**: CI/CD compatibility, version pinning, standard workflow
   - **Documentation**: See [CORPUS_TESTING.md](CORPUS_TESTING.md)
   - **Remaining**: Automate test generation from corpus format (Phase 5)

---

## Phase 1: Design & API Contracts

**Status**: Partially complete (data-model.md, quickstart.md exist, contracts/ needs creation)

### Data Model

See [data-model.md](data-model.md) for complete data model.

**Key Types**:
- `Pattern<V>`: Core pattern structure (value + elements)
- `Subject`: Pattern value type (identifier, labels, record)
- `Value`: Enum for heterogeneous property values ⚠️ **NEEDS DESIGN**
- `ParseError`: Parser error with location and message
- `SerializeError`: Serialization error

**Design Tasks**:
1. ✅ Pattern structure mapping (complete in data-model.md)
2. ⚠️ **Value enum definition** - NEEDS COMPLETION
3. ⚠️ **Subject refactoring** - NEEDS ASSESSMENT
4. ⚠️ **Arrow type semantics** - NEEDS SPECIFICATION

### API Contracts

**Contracts to Create** (contracts/ directory):

1. **`contracts/parser-api.md`**: Parser function signatures
   ```rust
   pub fn parse_gram_notation(input: &str) -> Result<Vec<Pattern<Subject>>, ParseError>;
   pub fn parse_single_pattern(input: &str) -> Result<Pattern<Subject>, ParseError>;
   ```

2. **`contracts/serializer-api.md`**: Serializer function signatures
   ```rust
   pub fn serialize_pattern(pattern: &Pattern<Subject>) -> Result<String, SerializeError>;
   pub fn serialize_patterns(patterns: &[Pattern<Subject>]) -> Result<String, SerializeError>;
   ```

3. **`contracts/value-enum.md`**: Value enum definition
   ```rust
   pub enum Value {
       String(String),
       Integer(i64),
       Decimal(f64),
       Boolean(bool),
       Array(Vec<Value>),  // Scalar values only
       Range { lower: i64, upper: i64 },
       TaggedString { tag: String, content: String },
   }
   ```

4. **`contracts/error-types.md`**: Error type definitions
   ```rust
   pub struct ParseError {
       pub location: Location,
       pub message: String,
       pub errors: Vec<ParseError>,  // For error recovery
   }
   
   pub struct Location {
       pub line: usize,
       pub column: usize,
   }
   ```

### Quickstart

See [quickstart.md](quickstart.md) for usage guide.

**Updates Needed**:
- Add concrete API examples once contracts finalized
- Add error handling examples
- Add WASM/Python usage examples

---

## Next Steps

1. **Complete Phase 0 Research**:
   - ✅ Parse tree structure analysis (complete)
   - ⚠️ **Arrow type handling** - Specify element ordering rules for each arrow
   - ⚠️ **Value enum design** - Complete enum definition
   - ⚠️ **Subject refactoring** - Assess existing implementation
   - ✅ Test corpus integration - Git submodule approach (CORPUS_TESTING.md)

2. **Complete Phase 1 Design**:
   - ⚠️ Create `contracts/` directory with API specifications
   - ⚠️ Finalize Value enum design
   - ⚠️ Specify arrow type semantics
   - ✅ Update quickstart.md with concrete examples

3. **Phase 2: Task Breakdown** (via `/speckit.tasks`):
   - Break implementation into concrete tasks
   - Prioritize by dependency and risk
   - Estimate effort for each task

4. **Implementation**:
   - Follow constitution verification workflow
   - Validate all gram notation with `gram-lint`
   - Test against tree-sitter-gram corpus
   - Verify round-trip correctness

---

**Plan Status**: ⚠️ **IN PROGRESS** - Needs completion of arrow type specification and Value enum design

**Blockers**:
1. Arrow type handling rules (element ordering for each arrow variation)
2. Value enum complete design (integration with Subject)
3. API contracts creation (parser, serializer, error types)

**Ready for**:
- Detailed research on arrow type semantics
- Value enum design and Subject refactoring assessment
- API contract specification
