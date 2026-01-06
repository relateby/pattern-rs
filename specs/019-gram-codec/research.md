# Research: Basic Gram Codec

**Feature**: 019-gram-codec  
**Created**: 2026-01-06  
**Status**: Draft

## Research Questions

This document tracks research questions that need to be answered during the planning phase to inform implementation decisions.

### 1. Parser Library Selection

**Question**: Which parser library should be used for the gram codec, considering requirements for tree-sitter-gram compatibility, WASM compilation, and Python bindings?

**Requirements**:
- MUST compile to WASM target (for browser and Node.js)
- MUST support Python bindings or integration
- SHOULD leverage existing tree-sitter-gram grammar
- SHOULD provide good error reporting with location information
- SHOULD have reasonable performance for typical patterns

**Validation Tool Available**: `gram-lint` CLI provides reference validation
- Usage: `gram-lint -e "<gram>" [--tree]`
- Exit code 0 = valid, 1 = parse error
- `--tree` flag outputs S-expression parse tree
- See [VALIDATION.md](VALIDATION.md) for comprehensive examples with parse trees

**Options to Evaluate**:

1. **tree-sitter-gram bindings (Rust)**
   - Pros: Authoritative grammar, Rust bindings exist, WASM output available
   - Cons: Need to evaluate if tree-sitter runtime works well in Rust for parsing
   - Investigation needed: Can we use tree-sitter-gram Rust bindings directly?

2. **winnow** (Rust parser combinator library)
   - Pros: Pure Rust, good error reporting, WASM compatible, flexible
   - Cons: Need to implement grammar manually
   - Investigation needed: Effort to port tree-sitter-gram grammar?

3. **nom** (Rust parser combinator library)
   - Pros: Mature, pure Rust, WASM compatible
   - Cons: Need to implement grammar manually, less modern than winnow
   - Investigation needed: Effort to port tree-sitter-gram grammar?

4. **pest** (Rust PEG parser)
   - Pros: Grammar file-driven, WASM compatible, good for DSLs
   - Cons: Need to translate tree-sitter grammar to PEG format
   - Investigation needed: How different is PEG from tree-sitter grammar?

5. **chumsky** (Rust parser combinator library)
   - Pros: Modern, good error reporting, WASM compatible
   - Cons: Need to implement grammar manually
   - Investigation needed: Error recovery capabilities?

**Decision Criteria**:
1. tree-sitter-gram grammar compatibility (can we reuse it directly?)
2. WASM compilation support (required)
3. Python binding support (required - either direct or via separate Python bindings)
4. Error reporting quality (line/column, expected vs found)
5. Implementation effort (grammar porting vs direct use)
6. Performance (parse time for typical patterns)
7. Maintenance burden (keeping grammar in sync)

**Action**: Research each option during planning phase, create comparison matrix.

### 2. Tree-Sitter Grammar Direct Use

**Question**: Can we directly use the tree-sitter-gram parser from Rust to parse gram notation, or do we need to implement a separate parser?

**Investigation Needed**:
- How do tree-sitter Rust bindings work?
- Can we use `tree-sitter-gram` crate in `gram-codec`?
- What's the API for accessing parse trees?
- How do we transform tree-sitter CST into Pattern AST?
- What's the WASM story for tree-sitter in Rust?
- What's the Python story for tree-sitter Rust bindings?

**Pros of Direct Use**:
- Grammar is authoritative (no porting)
- No need to keep grammar in sync
- Mature, well-tested parser
- Multi-language bindings already exist

**Cons of Direct Use**:
- Additional dependency (tree-sitter runtime)
- Need to transform concrete syntax tree to Pattern AST
- May be overkill for serialization use case
- Need to verify WASM and Python compatibility

**Action**: Test tree-sitter-gram Rust bindings, build prototype parser.

### 3. AST vs Direct Pattern Construction

**Question**: Should we build an intermediate AST from gram notation, or directly construct Pattern structures during parsing?

**Options**:

1. **Direct Pattern Construction**:
   - Parser directly builds `Pattern<Subject>` structures
   - Pros: Simpler, fewer allocations, immediate validation
   - Cons: Parser is tightly coupled to Pattern type

2. **Intermediate AST**:
   - Parser builds gram-specific AST, then transform to Pattern
   - Pros: Separation of concerns, easier testing, format-agnostic Pattern type
   - Cons: Extra allocation, additional transformation step

**Decision Factors**:
- Is Pattern type stable enough for direct construction?
- Do we need gram-specific AST for other use cases (formatting, linting)?
- What's the performance impact of intermediate AST?

**Action**: Evaluate during planning based on Pattern type stability.

### 4. Error Recovery Strategy

**Question**: How should the parser handle syntax errors? Should it attempt error recovery to continue parsing, or fail fast?

**Options**:

1. **Fail Fast**: Stop at first error, report it
   - Pros: Simple, clear error location
   - Cons: User sees only one error at a time

2. **Error Recovery**: Try to recover and find multiple errors
   - Pros: Better developer experience (see all errors)
   - Cons: Complex, may report spurious errors

**Considerations**:
- What does `gram-lint` do?
- What's standard practice for data notation parsers (JSON, YAML, TOML)?
- Implementation complexity vs user benefit

**Action**: Check gram-lint behavior, decide based on effort vs benefit.

### 5. Serializer Format Strategy

**Question**: Should the serializer produce canonical (normalized) gram notation, or should it attempt to use the most concise syntax for each pattern?

**Options**:

1. **Canonical Format**: Always use subject pattern notation
   - Example: `[a | ]` even for atomic patterns
   - Pros: Simple, consistent, predictable
   - Cons: Verbose, not idiomatic

2. **Concise Format**: Choose syntax based on element count
   - 0 elements → `(a)` (node notation)
   - 2 elements → `(a)-->(b)` (relationship notation)
   - Other → `[a | ...]` (subject pattern notation)
   - Pros: Readable, idiomatic, matches common usage
   - Cons: More complex serializer logic

3. **Hybrid**: Canonical with options for concise
   - Default canonical, optional concise mode
   - Pros: Flexibility for different use cases
   - Cons: More API surface, more testing

**Considerations**:
- What do users expect to see?
- What does gram-lint prefer?
- Round-trip correctness is not affected (both parse to same Pattern)

**Action**: Review gram-hs serializer, check tree-sitter-gram examples.

### 6. Property Type Mapping

**Question**: How do gram notation property types (strings, numbers, booleans, arrays, ranges) map to Rust types in Subject records?

**Current Subject Definition** (from TODO.md):
```rust
// Check actual definition in crates/pattern-core/src/subject.rs
```

**Investigation Needed**:
- What is the current Subject type definition?
- Does it support all gram property types?
- Do we need a `Value` enum for heterogeneous property values?
- How do we represent ranges? Tagged strings?

**Options**:

1. **Strongly Typed**: Subject has typed property fields
   - Pros: Type safety, compile-time checks
   - Cons: Less flexible, need schema

2. **Dynamic Typing**: Subject properties are `HashMap<String, Value>`
   - Pros: Flexible, matches gram notation
   - Cons: Runtime type checking needed

**Action**: Review Subject implementation, define Value enum if needed.

### 7. Python Binding Strategy

**Question**: How should Python access the gram codec?

**Options**:

1. **PyO3 Bindings**: Direct Rust-Python bindings
   - Pros: Direct access to Rust codec, good performance
   - Cons: Need to write Python bindings, maintenance

2. **WASM via Python**: Use WASM codec from Python
   - Pros: Reuse WASM build, one implementation
   - Cons: WASM runtime overhead in Python

3. **Separate Python Parser**: Use tree-sitter-gram Python bindings
   - Pros: Native Python implementation
   - Cons: Need separate parser implementation, sync issues

**Considerations**:
- Who are the Python users?
- What performance is needed?
- Maintenance burden vs user experience

**Action**: Evaluate PyO3 effort, check if WASM from Python is viable.

### 8. Streaming vs Batch Parsing

**Question**: Should the parser support streaming (incremental) parsing, or only batch (whole-file) parsing?

**Use Cases**:
- Batch: Parse complete gram file into patterns
- Streaming: Parse gram notation as it arrives (network, large files)

**Considerations**:
- Are there use cases for streaming?
- What's the implementation complexity?
- Does tree-sitter support streaming?

**Action**: Focus on batch parsing initially, defer streaming unless needed.

### 9. Comment Preservation

**Question**: Should comments from gram notation be preserved (e.g., attached to patterns as metadata), or discarded during parsing?

**Options**:

1. **Discard**: Comments are ignored (semantic content only)
   - Pros: Simple, matches most parsers
   - Cons: Can't round-trip with comments

2. **Preserve**: Comments stored as pattern metadata
   - Pros: Can round-trip with comments, useful for tooling
   - Cons: Complex, need metadata mechanism in Pattern

**Considerations**:
- Do users need to preserve comments?
- Is comment preservation a requirement for round-trip?
- How would comments attach to patterns?

**Action**: Start with discarding, add preservation if requested.

### 10. Performance Targets

**Question**: What are acceptable performance targets for parsing and serialization?

**Considerations**:
- Typical pattern size: 10-1000 nodes
- Typical nesting depth: 1-10 levels
- Large patterns: 10,000+ nodes (rare, but possible)
- WASM performance vs native Rust

**Proposed Targets**:
- Parse 1000-node pattern in < 100ms (native), < 200ms (WASM)
- Serialize 1000-node pattern in < 100ms (native), < 200ms (WASM)
- Linear time complexity O(n) for n nodes

**Action**: Benchmark after implementation, optimize if needed.

### 11. Parse Tree Structure Analysis

**Question**: How does the tree-sitter-gram parse tree map to Pattern structures?

**Investigation Complete**: Parse tree structure fully documented in [VALIDATION.md](VALIDATION.md)

**Key Findings**:

1. **Top-Level Structure**: All valid gram notation produces `gram_pattern` root
   ```
   (gram_pattern [root: (record)]? [comment]* pattern*)
   ```

2. **Pattern Type Identification**:
   - `node_pattern`: Has no `kind` or `annotations` fields → 0 elements
   - `relationship_pattern`: Has `kind` field (arrow type) → 2 elements
   - `subject_pattern`: Has `elements` field with `|` syntax → N elements
   - `annotated_pattern`: Has `annotations` field → 1 element

3. **Arrow Types** (in `relationship_pattern` kind field):
   - `right_arrow`: `-->`
   - `left_arrow`: `<--`
   - `bidirectional_arrow`: `<-->`
   - `undirected_arrow`: `~~`

4. **Subject Components** (optional named fields):
   - `identifier`: symbol | string_literal | integer
   - `labels`: One or more symbols (can repeat)
   - `record`: Key-value properties

5. **Value Types** (in record properties):
   - `symbol`: Unquoted identifier
   - `string_literal`: Quoted string
   - `integer`: Whole number
   - `decimal`: Floating-point number
   - `boolean_literal`: `true` | `false`
   - `array`: List of scalar values
   - `range`: Numeric range with `lower` and `upper`

6. **Path Pattern Handling**:
   - Chained relationships like `(a)-[r1]->(b)-[r2]->(c)` are right-associative
   - Parse tree: `relationship_pattern(left: node_a, right: relationship_pattern(left: node_b, right: node_c))`
   - Need to flatten during parsing or handle nested relationship patterns

7. **Comment Handling**:
   - Comments appear as `(comment)` nodes in parse tree
   - No semantic content preserved (just presence)
   - Comments are siblings to patterns, not attached to specific patterns

**Implications for Parser Design**:
- Tree-sitter parse tree → Pattern AST transformation is straightforward
- Pattern type can be determined by checking field names in parse tree node
- Subject components map directly to named fields
- Path flattening requires recursive handling of nested relationship patterns
- Comments should be discarded during parsing (semantic content only)

**Action**: Use this structure analysis when designing parser transformation logic.

## References

- **tree-sitter-gram**: `../tree-sitter-gram/` (authoritative grammar)
  - Grammar: `grammar.js`
  - Test corpus: `test/corpus/`
  - Examples: `examples/data/`
  - Rust bindings: `bindings/rust/`
  - Python bindings: `bindings/python/`

- **gram-lint**: CLI validator using tree-sitter-gram
  - Location: `../tree-sitter-gram/tools/gram-lint/`
  - Binary: `~/.cargo/bin/gram-lint` (installed)
  - Usage: `gram-lint [OPTIONS] [FILES]...`
  - Options:
    - `-e <expr>`: Validate expression directly
    - `--tree`: Output S-expression parse tree
    - Exit code 0 = valid, 1 = parse error
  - **All gram snippets in plans/tasks MUST be validated with gram-lint**

- **Validation Documentation**: [VALIDATION.md](VALIDATION.md)
  - Comprehensive validated examples with parse trees
  - Parse tree structure analysis
  - Testing strategy guidelines
  - Parser implementation guidelines based on parse tree structure

- **Pattern Core**: `crates/pattern-core/` (Pattern and Subject types)
- **gram-hs**: `../gram-hs/` (reference for non-grammar aspects)

## Decision Log

This section is updated during the planning phase as research questions are answered and decisions are made.

### Decision: Parser Library Selection
**Date**: 2026-01-06  
**Question**: Which parser library should be used for the gram codec?  
**Decision**: Use tree-sitter-gram Rust bindings directly  
**Rationale**: 
- tree-sitter-gram is the authoritative grammar (per feature requirements)
- Rust bindings already exist in `../tree-sitter-gram/bindings/rust/`
- No need to manually port or maintain grammar
- WASM support confirmed (tree-sitter-gram has WASM output)
- Multi-language bindings already exist (Python, JavaScript, etc.)
- Parse tree structure already documented in VALIDATION.md

**Alternatives Considered**:
- **winnow/nom/chumsky**: Would require manual grammar implementation and ongoing sync
- **pest**: Would require PEG translation and grammar maintenance
- All manual approaches rejected due to maintenance burden and deviation from authoritative grammar

**Trade-offs**:
- ✅ Pro: Grammar authority and zero maintenance
- ✅ Pro: Error reporting built-in (parse tree errors)
- ✅ Pro: Multi-platform support confirmed
- ⚠️ Con: Additional dependency (tree-sitter runtime ~small)
- ⚠️ Con: Need CST → Pattern transformation layer
- ⚠️ Con: May be slight overkill for simple use cases

**Implementation Impact**:
- Add `tree-sitter` and `tree-sitter-gram` dependencies to Cargo.toml
- Implement CST (Concrete Syntax Tree) → Pattern AST transformation
- Extract error messages from tree-sitter error nodes
- Leverage existing bindings for Python/WASM

### Decision: Arrow Type Representation and Handling
**Date**: 2026-01-06  
**Question**: How should arrow types (→, ←, ↔, ~~) be represented and handled in parsing/serialization?  
**Decision**: Arrow types are syntactic sugar - element ordering captures semantics, with the following rules:

**Parsing Rules**:
1. **Right arrow `(a)-->(b)`**: 
   - Element order: [left_node, right_node] (preserve order)
   - Directionality: left → right (explicit)
   
2. **Left arrow `(a)<--(b)`**: 
   - Element order: [right_node, left_node] (reverse order to maintain semantic left→right)
   - Logical flow: a ← b means b → a
   - Transform: parse `(a)<--(b)` as Pattern with elements [b, a]
   
3. **Bidirectional `(a)<-->(b)`**: 
   - Element order: [node_a, node_b] (preserve written order)
   - Semantic: symmetric relationship (order arbitrary but preserved)
   - No metadata needed - symmetric property inferred from context if needed
   
4. **Undirected `(a)~~(b)`**: 
   - Element order: [node_a, node_b] (preserve written order)
   - Semantic: symmetric relationship (order arbitrary but preserved)
   - No metadata needed - undirected property inferred from context if needed

**Serialization Rules**:
1. **2-element pattern with both elements atomic (0 elements each)**:
   - Use relationship notation: `(elem1)-->(elem2)` (always right arrow)
   - Order: elements[0] → elements[1]
   
2. **2-element pattern with non-atomic elements**:
   - Use subject pattern notation: `[value | elem1, elem2]`
   
3. **Other element counts** (0, 1, N where N ≠ 2):
   - Use appropriate notation (node, annotation, subject pattern)

**Rationale**:
- Arrow types are syntactic sugar for expressing relationship direction (clarification from user)
- Element ordering in Pattern structure captures the semantic relationship
- No additional metadata needed in Pattern structure (keeps it clean)
- Serialization always uses right arrow `-->` for simplicity (canonical form)
- Left arrow `<--` is parsed by reversing element order (maintains semantic direction)
- Bidirectional and undirected are treated as symmetric (preserve written order)

**Alternatives Considered**:
- **Store arrow type in Subject**: Would require Subject to have arrow_type field (adds complexity)
- **Metadata field in Pattern**: Would break Pattern's clean structure
- **Always serialize as written**: Would require storing original arrow type (not semantic)

**Trade-offs**:
- ✅ Pro: Clean Pattern structure (no arrow metadata)
- ✅ Pro: Semantic consistency (element order = relationship direction)
- ✅ Pro: Simple serialization (always use right arrow)
- ⚠️ Con: Loss of original arrow type in round-trip (left arrow becomes right arrow)
- ⚠️ Con: Cannot distinguish bidirectional from undirected in serialized form
- ✅ Pro: This is acceptable - semantic meaning preserved, syntax normalized

**Implementation Impact**:
- Parser must reverse elements for left arrow `<--`
- Parser preserves written order for bidirectional `<-->` and undirected `~~`
- Serializer always uses right arrow `-->` for 2-element atomic patterns
- No additional fields needed in Pattern or Subject

### Decision: Value Enum Design for Property Types
**Date**: 2026-01-06  
**Question**: How should heterogeneous property values be represented in Subject's record?  
**Decision**: Define a `Value` enum with variants for all gram notation types

**Value Enum Definition**:
```rust
/// Represents a value in gram notation property records.
/// Supports all value types defined in the tree-sitter-gram grammar.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Unquoted or quoted string
    String(String),
    
    /// Integer value (i64 for full range support)
    Integer(i64),
    
    /// Decimal/floating-point value
    Decimal(f64),
    
    /// Boolean true/false
    Boolean(bool),
    
    /// Array of scalar values (homogeneous or heterogeneous)
    /// Note: Grammar allows ["string", 42, true] mixed types
    Array(Vec<Value>),
    
    /// Numeric range with inclusive bounds
    Range { lower: i64, upper: i64 },
    
    /// Tagged string with format identifier
    /// Example: """markdown content"""
    TaggedString { tag: String, content: String },
}

impl Value {
    /// Parse value from tree-sitter node
    pub fn from_tree_sitter_node(node: &Node) -> Result<Self, ParseError> {
        // Implementation based on node type
    }
    
    /// Serialize value to gram notation
    pub fn to_gram_notation(&self) -> String {
        // Implementation for each variant
    }
}
```

**Subject Integration**:
```rust
/// Subject represents the value/decoration of a pattern
#[derive(Debug, Clone, PartialEq)]
pub struct Subject {
    /// Optional identifier (symbol, string, or integer)
    pub identifier: Option<String>,
    
    /// Zero or more labels
    pub labels: Vec<String>,
    
    /// Property record (key-value pairs)
    pub record: HashMap<String, Value>,
}
```

**Rationale**:
- Standard Rust pattern for heterogeneous data (like `serde_json::Value`)
- Type-safe representation of all gram property types
- Enables compile-time correctness checks
- No runtime type confusion (unlike string storage or Any)
- Clear mapping to tree-sitter grammar value types

**Alternatives Considered**:
- **String storage**: Loses type information, requires parsing on access
- **Strongly typed**: Requires schema, less flexible
- **trait objects/Any**: Less idiomatic Rust, runtime overhead
- All rejected in favor of enum (best practice for this use case)

**Trade-offs**:
- ✅ Pro: Type safety at compile time
- ✅ Pro: Standard Rust pattern (familiar to developers)
- ✅ Pro: Efficient (no boxing, no vtables)
- ✅ Pro: Easy serialization (pattern matching)
- ⚠️ Con: Larger memory footprint than string storage
- ⚠️ Con: May require Subject refactoring if value already exists elsewhere

**Subject Refactoring Assessment**:
- **Action**: Review `crates/pattern-core/src/subject.rs` before implementation
- **Goal**: Check if Subject already has value representation that conflicts
- **Risk**: May need to unify value representation to avoid redundancy
- **Mitigation**: If redundancy found, refactor Subject to use shared Value enum

**Implementation Impact**:
- Define Value enum in `crates/gram-codec/src/value.rs`
- Update Subject to use `HashMap<String, Value>` for properties
- Implement parsing from tree-sitter nodes for each Value variant
- Implement serialization to gram notation for each Value variant
- Add comprehensive tests for each value type

### Decision: Test Corpus Integration Approach
**Date**: 2026-01-06  
**Updated**: 2026-01-06 (Submodule approach)  
**Question**: How should the tree-sitter-gram test corpus be incorporated?  
**Decision**: Use git submodule for CI/CD reliability with programmatic test generation

**Approach**:
1. **Corpus location**: Git submodule at `external/tree-sitter-gram/` (not peer directory `../`)
2. **CI/CD compatibility**: Submodule ensures tests work in GitHub Actions and other CI systems
3. **Version pinning**: Lock to specific tree-sitter-gram commit for reproducibility
4. **Test generation**: Write Rust code to parse corpus format and generate tests
5. **Conditional testing**: Tests gracefully skip if submodule not initialized (allows basic dev)

**Why Submodule vs Peer Directory**:
- ✅ **CI/CD**: Works automatically in GitHub Actions (just add `submodules: true`)
- ✅ **Consistency**: All developers and CI use same corpus version
- ✅ **Standard practice**: Git submodules are well-understood workflow
- ✅ **Easy setup**: Single command (`git submodule update --init`)
- ✅ **Updates**: Standard git commands to pull latest corpus

See [CORPUS_TESTING.md](CORPUS_TESTING.md) for complete setup documentation.

**Corpus File Format** (example from `nodes.txt`):
```
==================
Test name
==================

gram_input

---

(expected_parse_tree)
```

**Implementation**:
```rust
// tests/corpus_tests.rs
fn load_corpus_tests(corpus_path: &Path) -> Vec<CorpusTest> {
    // Parse corpus files (.txt format)
    // Each test has: name, input, expected parse tree
}

#[test]
fn run_corpus_tests() {
    let corpus_path = Path::new("../../tree-sitter-gram/test/corpus");
    let tests = load_corpus_tests(corpus_path);
    
    for test in tests {
        let result = parse_gram_notation(&test.input);
        assert!(result.is_ok(), "Test '{}' failed to parse", test.name);
        
        // Optional: Compare parse tree structure
        // (tree-sitter parse tree vs our Pattern structure)
    }
}
```

**Rationale**:
- No duplication (single source of truth in tree-sitter-gram repo)
- Automatic sync (changes to corpus reflected immediately)
- No git submodule complexity (just relative path reference)
- Works in CI if both repos checked out in expected structure

**Alternatives Considered**:
- **Copy files**: Manual sync required, duplication
- **Git submodule**: Adds complexity, harder to manage
- **Symbolic links**: Works locally, may fail in CI or Windows

**Trade-offs**:
- ✅ Pro: Single source of truth (no duplication)
- ✅ Pro: Automatic sync with corpus changes
- ✅ Pro: Simple implementation (just relative path)
- ⚠️ Con: Requires both repos checked out in expected structure
- ⚠️ Con: Test failures if corpus path not found (document in README)

**Implementation Impact**:
- Create corpus test module: `tests/corpus_tests.rs`
- Implement corpus file parser (handles `===` separator format)
- Add documentation about expected repo structure
- CI configuration must check out both repos in correct layout

### Decision: Python Bindings Strategy
**Date**: 2026-01-06  
**Question**: How should Python access the gram codec?  
**Decision**: Use PyO3 for direct Rust-Python bindings (primary), with WASM as fallback

**Primary Approach: PyO3**
```rust
// python bindings in crates/gram-codec/src/python.rs (feature-gated)
#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyfunction]
fn parse_gram(input: &str) -> PyResult<Vec<Pattern>> {
    parse_gram_notation(input)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
}

#[cfg(feature = "python")]
#[pymodule]
fn gram_codec(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(parse_gram, m)?)?;
    Ok(())
}
```

**Rationale**:
- Native performance (no WASM overhead)
- Direct Rust-Python integration
- Type conversion handled by PyO3
- Standard approach in Rust ecosystem

**Fallback: WASM via Python**
- If PyO3 doesn't work for specific use case
- Use WASM module from Python (via wasmtime or similar)
- Lower performance but more portable

**Trade-offs**:
- ✅ Pro: Native performance
- ✅ Pro: Clean Python API
- ✅ Pro: Standard Rust Python binding approach
- ⚠️ Con: Requires compilation for Python (maturin)
- ⚠️ Con: Platform-specific wheels needed

**Implementation Impact**:
- Add `pyo3` dependency with `python` feature flag
- Create Python module bindings in feature-gated code
- Use `maturin` for building Python wheels
- Add Python example in `examples/python/`
