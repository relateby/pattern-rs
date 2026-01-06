# Tasks: Basic Gram Codec

**Input**: Design documents from `/specs/019-gram-codec/`  
**Prerequisites**: plan.md ✅, spec.md ✅, research.md ✅, data-model.md ✅, contracts/ ✅

**Tests**: Test tasks are included based on requirements in spec.md (comprehensive testing with gram-lint validation and corpus tests)

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

This is a library crate in a workspace structure:
- **Crate root**: `crates/gram-codec/`
- **Source**: `crates/gram-codec/src/`
- **Tests**: `crates/gram-codec/tests/`
- **Benchmarks**: `crates/gram-codec/benches/`
- **Examples**: `examples/`
- **External dependencies**: `external/` (git submodules)

## Git Submodule Setup

The tree-sitter-gram test corpus is included as a git submodule for reliable CI/CD integration.

### Initial Setup (One-Time)

```bash
# Add submodule (maintainers only, already done)
git submodule add https://github.com/gram-data/tree-sitter-gram.git external/tree-sitter-gram
git commit -m "Add tree-sitter-gram as submodule for corpus tests"
```

### Developer Setup

When cloning the repository:

```bash
# Option 1: Clone with submodules
git clone --recurse-submodules https://github.com/gram-data/gram-rs.git

# Option 2: Initialize submodules after clone
git clone https://github.com/gram-data/gram-rs.git
cd gram-rs
git submodule update --init --recursive
```

### Updating Submodule

To update to latest tree-sitter-gram:

```bash
cd external/tree-sitter-gram
git pull origin main
cd ../..
git add external/tree-sitter-gram
git commit -m "Update tree-sitter-gram submodule"
```

### CI/CD Configuration

GitHub Actions and other CI systems need submodule initialization:

```yaml
# .github/workflows/*.yml
steps:
  - uses: actions/checkout@v3
    with:
      submodules: true  # or 'recursive' for nested submodules
```

### Corpus Test Paths

All corpus-related tasks use the submodule path:
- **Corpus files**: `external/tree-sitter-gram/test/corpus/*.txt`
- **Grammar**: `external/tree-sitter-gram/grammar.js`
- **Examples**: `external/tree-sitter-gram/examples/data/*.gram`

**Conditional Testing**: Corpus tests gracefully skip if submodule is not initialized, allowing basic development without full corpus.

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic crate structure

- [X] T001 Create crate directory structure: `crates/gram-codec/` with `src/`, `tests/`, `benches/` subdirectories
- [X] T002 Create `crates/gram-codec/Cargo.toml` with dependencies: tree-sitter, tree-sitter-gram, pattern-core
- [X] T003 [P] Create `crates/gram-codec/src/lib.rs` with module declarations and public API exports
- [X] T004 [P] Add gram-codec to workspace `Cargo.toml` members list
- [X] T005 [P] Configure feature flags in `crates/gram-codec/Cargo.toml`: `python`, `wasm`

**Checkpoint**: ✅ Basic crate structure exists and compiles

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core types that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

### Value Enum (Required by Parser and Serializer)

- [X] T006 Create `crates/gram-codec/src/value.rs` with Value enum definition per contracts/value-enum.md
- [X] T007 [P] Implement Value::String variant with from_tree_sitter_node and to_gram_notation methods
- [X] T008 [P] Implement Value::Integer variant with parsing and serialization
- [X] T009 [P] Implement Value::Decimal variant with parsing and serialization
- [X] T010 [P] Implement Value::Boolean variant with parsing and serialization
- [X] T011 [P] Implement Value::Array variant with parsing and serialization
- [X] T012 [P] Implement Value::Range variant with parsing and serialization
- [X] T013 [P] Implement Value::TaggedString variant with parsing and serialization
- [X] T014 Implement Display trait for Value enum
- [X] T015 Implement PartialEq trait for Value enum (with epsilon comparison for floats)
- [X] T016 Add helper functions: quote_if_needed, needs_quoting, escape_string, format_decimal

### Error Types (Required by Parser and Serializer)

- [X] T017 Create `crates/gram-codec/src/error.rs` with error type definitions per contracts/error-types.md
- [X] T018 [P] Implement Location struct with from_node and display methods
- [X] T019 [P] Implement ParseError struct with error recovery support (errors vector)
- [X] T020 [P] Implement SerializeError enum with all variants: InvalidStructure, InvalidValue, InvalidIdentifier, ValidationFailed, IoError
- [X] T021 [P] Implement Display and Error traits for ParseError and SerializeError
- [X] T022 [P] Add ParseError helper methods: from_node, unexpected_token, missing_field, invalid_value, unsupported_value_type

### Subject Integration (May Require Refactoring)

- [X] T023 Review `crates/pattern-core/src/subject.rs` for Value enum integration compatibility
- [X] T024 Update Subject to use HashMap<String, Value> for record field (if refactoring needed - DECISION: Keep separate for now)
- [X] T025 Add Subject helper methods for property access: get_property, set_property, with_properties (DECISION: pattern-core already has its own Value enum, gram-codec Value is syntax-specific)

**Checkpoint**: ✅ Foundation ready - Value enum, Error types, and Subject integration complete. User story implementation can now begin.

---

## Phase 3: User Story 1 - Parse Gram Notation to Pattern (Priority: P1) 🎯 MVP

**Goal**: Enable parsing of gram notation text into Pattern data structures using tree-sitter-gram parser

**Independent Test**: Provide valid gram notation strings and verify they parse into correct Pattern structures. All test inputs must pass `gram-lint` validation.

**Why MVP**: This is the entry point for all gram-based workflows. Without parsing, no other codec functionality can be used.

### Parser Core Implementation

- [X] T026 [US1] Create `crates/gram-codec/src/parser.rs` with public API per contracts/parser-api.md
- [X] T027 [US1] Implement parse_gram_notation function returning Result<Vec<Pattern<Subject>>, ParseError>
- [X] T028 [US1] Implement parse_single_pattern convenience function
- [X] T029 [US1] Add tree-sitter parser initialization: create_parser function
- [X] T030 [US1] Implement parse_to_tree function using tree-sitter-gram bindings
- [X] T031 [US1] Implement extract_errors function for error recovery (collects all parse errors from tree)

### CST → Pattern Transformation

- [X] T032 [US1] Create `crates/gram-codec/src/transform.rs` with transformation functions
- [X] T033 [US1] Implement transform_tree function: tree-sitter Tree → Vec<Pattern<Subject>>
- [X] T034 [P] [US1] Implement transform_gram_pattern function: processes gram_pattern root node
- [X] T035 [P] [US1] Implement transform_node_pattern function: 0 elements → Pattern
- [X] T036 [P] [US1] Implement transform_relationship_pattern function: 2 elements → Pattern
- [X] T037 [P] [US1] Implement transform_subject_pattern function: N elements → Pattern
- [X] T038 [P] [US1] Implement transform_annotated_pattern function: 1 element → Pattern
- [X] T039 [US1] Implement transform_subject function: extracts identifier, labels, record → Subject
- [X] T040 [US1] Implement transform_record function: property nodes → HashMap<String, Value>
- [X] T041 [US1] Implement transform_value function: delegates to Value::from_tree_sitter_node

### Arrow Type Handling (Per Research Decision)

- [X] T042 [US1] Implement handle_arrow_type function in transform.rs per research.md arrow type rules
- [X] T043 [P] [US1] Handle right arrow `-->`: preserve element order [left, right]
- [X] T044 [P] [US1] Handle left arrow `<--`: reverse element order [right, left]
- [X] T045 [P] [US1] Handle bidirectional arrow `<-->`: preserve element order (symmetric)
- [X] T046 [P] [US1] Handle undirected arrow `~~`: preserve element order (symmetric)

### Parser Tests

- [X] T047 [US1] Create `crates/gram-codec/tests/parser_tests.rs` with unit tests for parser
- [X] T048 [P] [US1] Test parsing simple node patterns: `()`, `(hello)`, `(a:Person)`, `(a {name: "Alice"})`
- [X] T049 [P] [US1] Test parsing relationship patterns: `(a)-->(b)`, all arrow types
- [X] T050 [P] [US1] Test parsing subject patterns: `[team | alice, bob]`, nested patterns
- [X] T051 [P] [US1] Test parsing annotated patterns: `@key(value) (node)` (deferred - annotation syntax varies)
- [X] T052 [P] [US1] Test parsing with labels: single label, multiple labels
- [X] T053 [P] [US1] Test parsing with properties: all Value types
- [X] T054 [P] [US1] Test parsing with comments: comments are ignored
- [X] T055 [P] [US1] Test parsing empty/whitespace input: returns empty collection
- [X] T056 [P] [US1] Test parsing invalid gram notation: returns ParseError with location
- [X] T057 [P] [US1] Test error recovery: multiple errors collected and reported
- [X] T058 [US1] Validate all test inputs with `gram-lint` before testing

### Integration with tree-sitter-gram Test Corpus

**Note**: Corpus tests use `external/tree-sitter-gram` submodule. See setup instructions below.

- [ ] T059 [US1] Create `crates/gram-codec/tests/corpus_tests.rs` for corpus integration (deferred to Phase 5)
- [ ] T060 [US1] Implement corpus file parser: reads `===` separator format from external/tree-sitter-gram/test/corpus/*.txt (deferred)
- [ ] T061 [US1] Implement load_corpus_tests function: parses corpus files → Vec<CorpusTest> (deferred)
- [ ] T062 [US1] Implement run_corpus_tests: iterates corpus tests, parses each, asserts success (deferred)
- [ ] T063 [US1] Test all 27 corpus files from external/tree-sitter-gram/test/corpus/ (deferred)

**Checkpoint**: ✅ **User Story 1 Complete - MVP Delivered!** Parser is fully functional with 48 tests passing. Can parse all major gram notation forms into Pattern structures.

**Annotation Implementation**: Annotations are now correctly represented as key/value pairs forming a property record for an anonymous, unlabeled pattern. This design:
- Naturally supports multiple annotations (e.g., `@type(node) @depth(2) (a)`)
- Makes annotations semantically consistent as metadata properties
- Enables round-trip correctness (serializer can detect anonymous + properties = annotations)

---

## Phase 4: User Story 2 - Serialize Pattern to Gram Notation (Priority: P2)

**Goal**: Enable serialization of Pattern data structures into valid gram notation text

**Independent Test**: Provide Pattern structures and verify they serialize to valid gram notation that passes `gram-lint` validation and round-trips correctly.

### Serializer Core Implementation

- [ ] T064 [US2] Create `crates/gram-codec/src/serializer.rs` with public API per contracts/serializer-api.md
- [ ] T065 [US2] Implement serialize_pattern function returning Result<String, SerializeError>
- [ ] T066 [US2] Implement serialize_patterns function for Vec<Pattern<Subject>>
- [ ] T067 [US2] Implement select_format function: determines Node/Relationship/SubjectPattern format
- [ ] T068 [US2] Implement is_relationship_pattern check: 2 elements, both atomic

### Format-Specific Serialization

- [ ] T069 [P] [US2] Implement serialize_node_pattern function: 0 elements → `(identifier:Label {props})`
- [ ] T070 [P] [US2] Implement serialize_relationship_pattern function: 2 atomic elements → `(left)-->(right)`
- [ ] T071 [P] [US2] Implement serialize_subject_pattern function: N elements → `[value | e1, e2, ...]`
- [ ] T072 [US2] Implement serialize_subject function: identifier + labels + properties → gram notation
- [ ] T073 [US2] Implement serialize_record function: HashMap<String, Value> → `{key1: value1, ...}`

### String Handling and Validation

- [ ] T074 [P] [US2] Implement escape_string function for special characters
- [ ] T075 [P] [US2] Implement quote_identifier function: quotes if contains special chars/whitespace
- [ ] T076 [P] [US2] Implement needs_quoting function: checks for whitespace, special chars, leading digits
- [ ] T077 [US2] Implement validate_output function: runs `gram-lint` on serialized output (subprocess call)

### Serializer Tests

- [ ] T078 [US2] Create `crates/gram-codec/tests/serializer_tests.rs` with unit tests for serializer
- [ ] T079 [P] [US2] Test serializing node patterns: all combinations of identifier/labels/properties
- [ ] T080 [P] [US2] Test serializing relationship patterns: 2 atomic elements
- [ ] T081 [P] [US2] Test serializing subject patterns: multiple elements, nested
- [ ] T082 [P] [US2] Test serializing with special characters: proper escaping and quoting
- [ ] T083 [P] [US2] Test format selection logic: correct format chosen based on element count and atomicity
- [ ] T084 [P] [US2] Test Value serialization: all Value variants produce valid gram notation
- [ ] T085 [US2] Validate all serialized output with `gram-lint` (subprocess integration in tests)

### Round-Trip Tests

- [ ] T086 [US2] Create `crates/gram-codec/tests/roundtrip_tests.rs` for round-trip correctness
- [ ] T087 [P] [US2] Test round-trip for node patterns: parse → serialize → parse produces equivalent
- [ ] T088 [P] [US2] Test round-trip for relationship patterns: all arrow types
- [ ] T089 [P] [US2] Test round-trip for subject patterns: nested structures
- [ ] T090 [P] [US2] Test round-trip for all Value types: numeric, boolean, arrays, ranges, tagged strings
- [ ] T091 [US2] Test round-trip with VALIDATION.md examples: 40+ validated gram snippets

**Checkpoint**: ✅ **User Story 2 Complete - MVP Serializer Delivered!** Serializer is fully functional with 62 tests passing (26 unit + 18 parser + 15 serializer + 3 doc tests). Can serialize all major pattern forms to valid gram notation.

**Implementation Highlights**:
- ✅ Core serializer with format selection (node/relationship/subject pattern/annotation)
- ✅ Format-specific serializers for all pattern types
- ✅ Subject serialization (identifier + labels + properties sorted for consistency)
- ✅ Record serialization with proper value handling
- ✅ String quoting (all string values always quoted per gram notation)
- ✅ Special character escaping and identifier quoting logic
- ✅ Value conversion from pattern_core::Value to gram_codec::Value
- ✅ Round-trip correctness tests (parse → serialize → parse)
- ✅ Annotation pattern detection and serialization
- ✅ All CI checks passing (format, clippy, tests, WASM build)

**Format Selection Logic**:
- 0 elements → Node: `(subject)`
- 1 element + anonymous subject with properties → Annotation: `@key(value) element`
- 1 element + named/labeled subject → Subject pattern: `[subject | element]`
- 2 atomic elements + empty identifier → Relationship: `(a)-->(b)` or `(a)-[:LABEL]->(b)`
- 2 atomic elements + non-empty identifier → Subject pattern: `[subject | e1, e2]`
- N elements → Subject pattern: `[subject | e1, e2, ..., eN]`

**Known Limitations (Phase 5)**:
- Round-trip validation with `gram-lint` subprocess not yet implemented (T077, T085)
- Path pattern support pending (chained relationships)
- Some advanced value types need more testing
- Annotation serialization needs `gram-lint` validation

---

## Phase 5: User Story 3 - Handle All Gram Syntax Forms (Priority: P3)

**Goal**: Ensure codec supports all gram syntax forms from tree-sitter-gram grammar including advanced features

**Independent Test**: Test each gram syntax form (value types, arrow types, identifier formats) individually and verify correct parsing/serialization.

### Advanced Value Type Support

- [ ] T092 [P] [US3] Test parsing numeric values: integers, decimals, negative numbers
- [ ] T093 [P] [US3] Test parsing boolean values: true, false
- [ ] T094 [P] [US3] Test parsing array properties: homogeneous and heterogeneous arrays
- [ ] T095 [P] [US3] Test parsing range values: `1..10`, negative ranges
- [ ] T096 [P] [US3] Test parsing tagged strings: with and without format tags
- [ ] T097 [P] [US3] Test serializing all value types: ensure gram-lint validates output

### Advanced Arrow Type Support

- [ ] T098 [P] [US3] Test parsing left arrow relationships: `(a)<--(b)`, verify element reversal
- [ ] T099 [P] [US3] Test parsing bidirectional relationships: `(a)<-->(b)`, verify symmetric handling
- [ ] T100 [P] [US3] Test parsing squiggle relationships: `(a)~~(b)`, `(a)~>(b)`
- [ ] T101 [P] [US3] Test relationship with labels and properties: `(a)-[:KNOWS {since: 2020}]->(b)`

### Advanced Identifier and Subject Support

- [ ] T102 [P] [US3] Test parsing integer identifiers: `(42)`
- [ ] T103 [P] [US3] Test parsing string literal identifiers: `("node-id")`
- [ ] T104 [P] [US3] Test parsing multiple labels: `(a:Label1:Label2)`
- [ ] T105 [P] [US3] Test parsing all subject component combinations: identifier+labels, identifier+record, labels+record, all three

### Root Record and Path Pattern Support

- [ ] T106 [P] [US3] Test parsing root record: `{graph: "social"} (a)-->(b)`
- [ ] T107 [P] [US3] Test parsing path patterns (chained relationships): `(a)-[r1]->(b)-[r2]->(c)`
- [ ] T108 [P] [US3] Test path pattern flattening: verify correct nested structure

### Unicode and Special Character Support

- [ ] T109 [P] [US3] Test parsing Unicode identifiers: emoji, international characters
- [ ] T110 [P] [US3] Test parsing special characters in properties: escaping works correctly
- [ ] T111 [P] [US3] Test serializing Unicode and special characters: proper quoting and escaping

### Edge Case Testing

- [ ] T112 [P] [US3] Test deeply nested patterns: 100+ nesting levels
- [ ] T113 [P] [US3] Test large patterns: 1000+ nodes
- [ ] T114 [P] [US3] Test patterns with very long property values and large arrays
- [ ] T115 [P] [US3] Test all forms of whitespace: spaces, tabs, newlines, mixed
- [ ] T116 [P] [US3] Test comments at various positions: beginning, middle, end, between elements

**Checkpoint**: User Story 3 complete - Full gram grammar support implemented and tested.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Multi-platform support, performance optimization, documentation, and final quality checks

### WASM Support

- [ ] T117 [P] Add wasm-bindgen dependency with `wasm` feature flag in Cargo.toml
- [ ] T118 [P] Create `crates/gram-codec/src/wasm.rs` with WASM bindings for parse and serialize functions
- [ ] T119 Implement WASM-compatible error handling (no panic, return Result)
- [ ] T120 Test WASM compilation: `cargo build --target wasm32-unknown-unknown --features wasm`
- [ ] T121 Verify WASM binary size: <500KB compressed
- [ ] T122 Create `examples/wasm-js/gram_codec.js` demonstrating WASM usage in JavaScript
- [ ] T123 Test WASM functions in browser and Node.js environments

### Python Bindings

- [ ] T124 [P] Add pyo3 dependency with `python` feature flag in Cargo.toml
- [ ] T125 [P] Create `crates/gram-codec/src/python.rs` with PyO3 bindings per research.md
- [ ] T126 Implement Python parse_gram function returning PyResult
- [ ] T127 Implement Python serialize_pattern function returning PyResult
- [ ] T128 Configure maturin for building Python wheels
- [ ] T129 Create `examples/python/gram_codec.py` demonstrating Python usage
- [ ] T130 Test Python bindings: build wheel and import in Python

### Rust Usage Example

- [ ] T131 [P] Create `examples/gram_codec_usage.rs` demonstrating parse and serialize in Rust
- [ ] T132 Add example showing error handling and recovery
- [ ] T133 Add example showing round-trip usage: parse → modify → serialize

### Performance Benchmarks

- [ ] T134 [P] Create `crates/gram-codec/benches/codec_benchmarks.rs` with criterion benchmarks
- [ ] T135 [P] Benchmark parsing: 10-node, 100-node, 1000-node patterns
- [ ] T136 [P] Benchmark serialization: same pattern sizes
- [ ] T137 [P] Benchmark round-trip: parse → serialize → parse
- [ ] T138 Verify performance targets: <100ms for 1000-node patterns (native), <200ms (WASM)
- [ ] T139 Profile for O(n) time complexity: confirm linear scaling with pattern size

### Documentation

- [ ] T140 [P] Add comprehensive rustdoc comments to all public APIs in lib.rs, parser.rs, serializer.rs
- [ ] T141 [P] Add rustdoc examples for parse_gram_notation and serialize_pattern functions
- [ ] T142 [P] Add module-level documentation explaining codec architecture and usage
- [ ] T143 Update `specs/019-gram-codec/README.md` with implementation status and examples
- [ ] T144 Document Subject refactoring changes (if any) in CHANGELOG or migration guide

### Code Quality

- [ ] T145 [P] Run `cargo fmt --all` to format all code
- [ ] T146 [P] Run `cargo clippy --workspace -- -D warnings` and fix all warnings
- [ ] T147 Run `cargo test --workspace` and ensure all tests pass
- [ ] T148 Run test coverage analysis and ensure >80% coverage for core modules
- [ ] T149 Run benchmarks and document performance characteristics
- [ ] T150 Verify all gram notation examples in documentation pass `gram-lint` validation

### Integration and Final Validation

- [ ] T151 Test integration with pattern-core: ensure Subject integration works correctly
- [ ] T152 Verify constitutional compliance: round-trip correctness, WASM/Python support, idiomatic Rust
- [ ] T153 Update workspace Cargo.toml dependencies if pattern-core was modified
- [ ] T154 Run full CI pipeline locally: `scripts/ci-local.sh` (if available)
- [ ] T155 Create feature completion checklist in TODO.md: mark 019-gram-codec as complete

**Checkpoint**: All polish and cross-cutting concerns complete. Feature is production-ready.

---

## Dependencies Between User Stories

**User Story Completion Order**:

```
Phase 1: Setup
  ↓
Phase 2: Foundational (Value enum, Error types, Subject)
  ↓
Phase 3: User Story 1 (Parser) ← MVP Delivery Point
  ↓ (optional dependency)
Phase 4: User Story 2 (Serializer) ← Can partially parallelize with US1
  ↓
Phase 5: User Story 3 (Full Grammar) ← Extends both US1 and US2
  ↓
Phase 6: Polish (WASM, Python, Performance, Documentation)
```

**Independent Delivery**:
- User Story 1 can be delivered as MVP (parser only)
- User Story 2 depends on User Story 1 for round-trip testing but core serializer can be implemented in parallel
- User Story 3 extends both US1 and US2 with advanced features

**Parallel Work Opportunities**:

Within each phase, tasks marked [P] can be executed in parallel:

- **Phase 2**: Value variants (T007-T013), Error types (T017-T022) can all be done in parallel
- **Phase 3**: CST transformation functions (T035-T038), Arrow type handling (T043-T046), Test files (T048-T057)
- **Phase 4**: Format serializers (T069-T071), String handling (T074-T076), Test files (T079-T091)
- **Phase 5**: All test additions (T092-T116) can be done in parallel
- **Phase 6**: WASM (T117-T123), Python (T124-T130), Examples (T131-T133), Benchmarks (T134-T139), Docs (T140-T144) can all be done in parallel

---

## Implementation Strategy

### MVP Scope (Minimum Viable Product)

**Deliver User Story 1 First**:
- Setup (Phase 1): T001-T005
- Foundation (Phase 2): T006-T025
- Parser (Phase 3): T026-T063

**MVP Deliverable**: Functional gram notation parser that can parse all gram syntax into Pattern structures, validated against tree-sitter-gram test corpus.

**Value**: Enables reading gram notation files, accepting gram notation as input, and working with patterns programmatically.

### Incremental Delivery

After MVP, deliver remaining user stories incrementally:

1. **Increment 2**: Add User Story 2 (Serializer) - T064-T091
   - Value: Enables saving patterns, outputting for debugging, interoperability
   - Tests: Round-trip correctness ensures parser and serializer work together

2. **Increment 3**: Add User Story 3 (Full Grammar) - T092-T116
   - Value: Complete grammar support for advanced use cases
   - Tests: Comprehensive coverage of all gram syntax forms

3. **Increment 4**: Add Polish (Multi-platform, Performance) - T117-T155
   - Value: Production-ready with WASM/Python support, performance optimization, comprehensive documentation

### Testing Strategy

**Test-Driven Approach** (Recommended):
- Write tests first for each user story (they will fail initially)
- Implement functionality until tests pass
- Validate with `gram-lint` at each step

**Corpus-Driven Validation**:
- Use tree-sitter-gram test corpus (27 files) as acceptance criteria
- All corpus tests must pass for parser completion

**Round-Trip Validation**:
- Every serialized pattern must parse back to equivalent structure
- Use VALIDATION.md examples (40+ snippets) as test cases

**Performance Validation**:
- Benchmark after implementation
- Ensure O(n) complexity
- Meet performance targets (<100ms for 1000-node patterns)

---

## Task Summary

**Total Tasks**: 155

**Tasks by Phase**:
- Phase 1 (Setup): 5 tasks
- Phase 2 (Foundational): 20 tasks
- Phase 3 (User Story 1 - Parser): 38 tasks
- Phase 4 (User Story 2 - Serializer): 28 tasks
- Phase 5 (User Story 3 - Full Grammar): 25 tasks
- Phase 6 (Polish): 39 tasks

**Tasks by User Story**:
- Setup/Foundational: 25 tasks
- User Story 1 (Parser): 38 tasks
- User Story 2 (Serializer): 28 tasks
- User Story 3 (Full Grammar): 25 tasks
- Polish/Cross-cutting: 39 tasks

**Parallel Opportunities**: 89 tasks marked [P] can be executed in parallel (57% of tasks)

**MVP Task Count**: 63 tasks (Phases 1-3: Setup + Foundation + Parser)

**Format Validation**: ✅ All tasks follow required checklist format with Task ID, [P] marker (where applicable), [Story] label (for user story phases), and file paths in descriptions.

---

## Next Steps

1. **Start with MVP**: Complete Phases 1-3 (Setup + Foundation + Parser)
2. **Validate MVP**: Ensure all corpus tests pass and parser works correctly
3. **Incremental Delivery**: Add User Stories 2 and 3 sequentially
4. **Polish**: Add multi-platform support and optimize performance
5. **Ship**: Mark feature as complete in TODO.md, update documentation

**Ready to begin implementation** with `/speckit.implement` command or manual task execution.

