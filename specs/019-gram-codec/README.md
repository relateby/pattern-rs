# Feature 019: Basic Gram Codec

**Status**: Specification Complete ✅  
**Branch**: `019-gram-codec`  
**Created**: 2026-01-06  
**Phase**: 4 - Gram Notation Serialization

## Overview

The Basic Gram Codec provides bidirectional transformation between gram notation (human-readable text format) and Pattern data structures (programmatic representation). This feature enables:

- **Parsing**: Transform gram notation text → Pattern structures
- **Serialization**: Transform Pattern structures → gram notation text
- **Validation**: Verify gram notation correctness using `gram-lint` CLI
- **Round-trip**: Parse → Serialize → Parse produces equivalent patterns

## Key Characteristics

### Authoritative Grammar Reference

**IMPORTANT**: Unlike most other features, `../gram-hs` is **NOT** authoritative for this feature. Instead:

- **Grammar Authority**: `../tree-sitter-gram/grammar.js` defines all syntax rules
- **Validation Tool**: `gram-lint` CLI validates all gram notation
- **Test Corpus**: `../tree-sitter-gram/test/corpus/` provides test cases
- **Examples**: `../tree-sitter-gram/examples/data/` shows gram notation examples

### Multi-Platform Requirements

The codec implementation must support:

- **Rust**: Native implementation in `crates/gram-codec/`
- **WASM**: Compile to WebAssembly for browser and Node.js
- **Python**: Python bindings or integration path

### Gram Notation Syntax

Gram notation provides multiple syntax forms:

```gram
// Node patterns (0 elements)
(hello)
(a:Person {name: "Alice"})

// Relationship patterns (2 elements)
(a)-->(b)
(a)-[:KNOWS {since: 2020}]->(b)

// Subject patterns (N elements)
[team:Team | alice, bob, charlie]
[outer | [inner | leaf]]

// Annotated patterns (1 element)
@type(node) (a)

// Comments
// This is a comment
(hello)-->(world)  // End-of-line comment
```

## Documentation Structure

### Core Documents

- **[spec.md](spec.md)**: Complete feature specification
  - User scenarios and acceptance criteria
  - Functional requirements
  - Success criteria
  - Assumptions and constraints

- **[data-model.md](data-model.md)**: Data model and mapping rules
  - Gram notation syntax forms
  - Subject structure (identifier, labels, record)
  - Pattern structure and element count semantics
  - Parsing and serialization mapping rules
  - Round-trip equivalence definition

- **[quickstart.md](quickstart.md)**: Quick reference guide
  - What is the gram codec?
  - Key use cases
  - Supported syntax examples
  - Validation with `gram-lint`
  - Next steps

- **[research.md](research.md)**: Research questions and investigation
  - Parser library selection criteria
  - Tree-sitter grammar direct use
  - Parse tree structure analysis (complete)
  - AST vs direct Pattern construction
  - Error recovery strategy
  - Serializer format strategy
  - Property type mapping
  - Python binding strategy
  - Performance targets

- **[VALIDATION.md](VALIDATION.md)**: Comprehensive gram notation validation
  - All gram snippets validated with `gram-lint`
  - Parse tree output for each example
  - Parse tree structure analysis and mapping
  - Parser implementation guidelines
  - Testing strategy recommendations
  - **All plans and tasks must use validated gram snippets from this document**

### Quality Assurance

- **[checklists/requirements.md](checklists/requirements.md)**: Specification quality checklist
  - ✅ All checklist items passing
  - ✅ No [NEEDS CLARIFICATION] markers
  - ✅ Requirements are testable and unambiguous
  - ✅ Success criteria are measurable and technology-agnostic
  - ✅ Ready for planning phase

### Contracts (To Be Created)

The `contracts/` directory will contain technical contracts created during the planning phase:

- Type signatures for parsing and serialization APIs
- Error type definitions
- Property value representation

## Specification Highlights

### User Stories (Prioritized)

1. **P1 - Parse Gram Notation to Pattern**: Enable loading patterns from gram files and accepting gram notation as input
2. **P2 - Serialize Pattern to Gram Notation**: Enable saving patterns to gram files and outputting human-readable form
3. **P3 - Handle All Gram Syntax Forms**: Support complete grammar including all value types, relationship types, and identifier formats

### Functional Requirements Summary

- 23 functional requirements covering parsing, serialization, and validation
- Complete grammar support (nodes, relationships, subject patterns, annotations)
- All value types (strings, numbers, booleans, arrays, ranges, tagged strings)
- All relationship arrow types (→, ←, ↔, ~~, ~>)
- Descriptive error messages with location information
- Round-trip correctness guarantee
- WASM and Python support requirements
- Validation using `gram-lint` CLI

### Success Criteria Summary

- 100% of parsed patterns validated by `gram-lint`
- 100% of serialized output validated by `gram-lint`
- 100% round-trip correctness for test cases
- Complete grammar support from tree-sitter-gram
- Performance: < 100ms for patterns with 1000 nodes
- WASM binary size < 500KB (compressed)
- Python integration path established

## Next Steps

1. **Planning Phase** (`/speckit.plan`):
   - Research parser library options (tree-sitter, winnow, pest, chumsky)
   - Evaluate tree-sitter-gram direct use vs manual implementation
   - Design codec architecture and API
   - Create implementation plan and task breakdown

2. **Implementation**:
   - Build parser (gram notation → Pattern)
   - Build serializer (Pattern → gram notation)
   - Implement error reporting
   - Add WASM compilation support
   - Create Python bindings

3. **Testing**:
   - Port tree-sitter-gram test corpus
   - Validate all output with `gram-lint`
   - Round-trip testing
   - WASM and Python integration testing

4. **Integration**:
   - Add to `crates/gram-codec/` module
   - Update workspace documentation
   - Create usage examples

## References

### External References

- **tree-sitter-gram**: `external/tree-sitter-gram/` (git submodule - authoritative grammar)
  - Grammar: `grammar.js`
  - Test corpus: `test/corpus/`
  - Examples: `examples/data/`
  - Rust bindings: `bindings/rust/`
  - Python bindings: `bindings/python/`
  - **Setup**: Run `git submodule update --init --recursive` after clone

- **gram-lint**: CLI validator using tree-sitter-gram
  - Usage: `gram-lint [OPTIONS] [FILES]...`
  - Validate expression: `gram-lint -e "(hello)-->(world)"`
  - Show parse tree: `gram-lint -t file.gram`

### Internal References

- **Pattern Core**: `crates/pattern-core/` (Pattern and Subject types)
- **TODO**: TODO.md entry for 019-gram-serialization
- **Project Plan**: `docs/gram-rs-project-plan.md`

## Validation

The specification has been validated using `gram-lint`:

```bash
# Test simple relationship
$ gram-lint -e "(hello)-->(world)"
$ echo $?
0  # ✓ Valid

# Test subject pattern with properties
$ gram-lint -e "[team:Team {name: \"DevRel\"} | alice, bob, charlie]"
$ echo $?
0  # ✓ Valid

# View parse tree
$ gram-lint -e "(a:Person {name: \"Alice\"})" --tree
(gram_pattern (node_pattern identifier: (symbol) labels: (labels (symbol)) record: (record (record_property key: (symbol) value: (string_literal content: (string_content))))))
```

**Validation Status**: ✅ All gram notation examples in the specification are valid according to tree-sitter-gram grammar.

**Comprehensive Validation**: See [VALIDATION.md](VALIDATION.md) for:
- 40+ validated gram snippets with parse trees
- All syntax forms (nodes, relationships, subject patterns, annotations)
- All value types (strings, numbers, booleans, arrays, ranges)
- All arrow types (→, ←, ↔, ~~)
- Parse tree structure analysis
- Parser implementation guidelines

---

**Specification Complete**: Ready for planning phase (`/speckit.plan`)

