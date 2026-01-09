# API Contract: Parser

**Feature**: 021-pure-rust-parser  
**Date**: 2026-01-09  
**Purpose**: Define the public API contract for gram notation parsing

## Public API Functions

### 1. parse_gram

Parse gram notation text into a collection of Pattern structures.

**Signature**:
```rust
pub fn parse_gram(input: &str) -> Result<Vec<Pattern>, ParseError>
```

**Parameters**:
- `input: &str` - Gram notation text to parse (UTF-8 encoded)

**Returns**:
- `Ok(Vec<Pattern>)` - Successfully parsed patterns
- `Err(ParseError)` - Parse error with location and context

**Behavior**:

1. **Input Validation**:
   - Input must be valid UTF-8 (guaranteed by Rust's `&str` type)
   - Empty input or whitespace-only input returns `Ok(vec![])` (empty collection)
   - Comment-only input returns `Ok(vec![])` (comments are ignored)

2. **Parsing**:
   - Parses zero or more gram patterns separated by whitespace
   - Supports all gram syntax forms: nodes, relationships, subject patterns, annotations
   - Ignores comments (`//` until end of line)
   - Handles nested structures recursively

3. **Error Handling**:
   - Returns first fatal syntax error encountered
   - Error includes location (line, column) and description
   - Error provides context stack (e.g., "in subject pattern > record > value")

4. **Success Criteria**:
   - All input consumed (no trailing unexpected characters)
   - All patterns are well-formed Pattern structures
   - Structural constraints validated (e.g., relationship has 2 elements)

**Examples**:

```rust
// Success cases
assert_eq!(
    parse_gram("(hello)").unwrap(),
    vec![Pattern::from_subject(Subject {
        identifier: Some("hello".to_string()),
        labels: None,
        record: None,
    })]
);

assert_eq!(
    parse_gram("(a)-->(b)").unwrap().len(),
    1  // One relationship pattern
);

assert_eq!(
    parse_gram("").unwrap(),
    vec![]  // Empty input is valid
);

// Error cases
assert!(parse_gram("(unclosed").is_err());  // Unmatched delimiter
assert!(parse_gram("(a)->").is_err());      // Incomplete relationship
assert!(parse_gram("@key").is_err());       // Annotation without pattern
```

**Performance Contract**:
- Typical patterns (simple nodes, relationships): <10μs per pattern
- Complex patterns (deeply nested, many properties): <100μs per pattern
- 1000 simple patterns: <120ms total (within 20% of tree-sitter baseline)
- Memory usage: O(n) where n is total pattern size (input text length + structure)

**Thread Safety**:
- Function is pure (no internal state)
- Safe to call from multiple threads concurrently
- Input and output lifetimes are independent

**Compatibility**:
- Compiles to wasm32-unknown-unknown without modification
- Works in browser and Node.js environments
- No platform-specific code paths

---

### 2. validate_gram

Validate gram notation syntax without constructing Pattern structures.

**Signature**:
```rust
pub fn validate_gram(input: &str) -> Result<(), ParseError>
```

**Parameters**:
- `input: &str` - Gram notation text to validate

**Returns**:
- `Ok(())` - Input is valid gram notation
- `Err(ParseError)` - Syntax error with location and description

**Behavior**:
- Performs full parse but discards results
- Faster than `parse_gram` for validation-only use cases
- Returns same errors as `parse_gram`

**Examples**:

```rust
assert!(validate_gram("(hello)").is_ok());
assert!(validate_gram("(a)-->(b)").is_ok());
assert!(validate_gram("(unclosed").is_err());
```

**Performance Contract**:
- Slightly faster than `parse_gram` (no Pattern allocation)
- ~90% of `parse_gram` time for simple patterns
- ~80% of `parse_gram` time for complex patterns (less allocation overhead)

---

## Internal Parser Functions (Not Public)

These functions are implementation details and not part of the public API:

```rust
// Grammar rule parsers (internal)
fn gram_pattern(input: &str) -> ParseResult<Pattern>;
fn node(input: &str) -> ParseResult<Pattern>;
fn subject_pattern(input: &str) -> ParseResult<Pattern>;
fn path_pattern(input: &str) -> ParseResult<Pattern>;
fn relationship(input: &str) -> ParseResult<Pattern>;
fn annotated_pattern(input: &str) -> ParseResult<Pattern>;

// Component parsers (internal)
fn subject(input: &str) -> ParseResult<Subject>;
fn identifier(input: &str) -> ParseResult<String>;
fn label(input: &str) -> ParseResult<String>;
fn record(input: &str) -> ParseResult<HashMap<String, Value>>;
fn value(input: &str) -> ParseResult<Value>;
fn arrow(input: &str) -> ParseResult<ArrowType>;

// Utility parsers (internal)
fn ws(input: &str) -> ParseResult<()>;
fn comment(input: &str) -> ParseResult<()>;
```

---

## Conformance Requirements

### 1. tree-sitter-gram Test Corpus

The parser MUST pass 100% of valid syntax tests in the tree-sitter-gram test corpus:

- Location: `../tree-sitter-gram/test/corpus/*.txt`
- All test cases marked as valid must parse successfully
- All test cases marked as invalid must return errors
- Semantic equivalence: Pattern structures must match expected parse trees

**Test Execution**:
```rust
#[test]
fn test_corpus_conformance() {
    let corpus = CorpusTestSuite::load("../tree-sitter-gram/test/corpus").unwrap();
    let results = corpus.run();
    
    assert_eq!(results.pass_rate(), 1.0, 
               "Corpus conformance: {}/{} passed",
               results.passed, results.total());
}
```

### 2. Round-Trip Correctness

**Requirement**: Semantic equivalence via `gram -> pattern -> gram -> pattern`

For all valid gram notation, the following must hold:

```rust
let original = "(a:Label {key: \"value\"})";

// First parse
let patterns1 = parse_gram(original).unwrap();

// Serialize
let serialized = serialize_patterns(&patterns1).unwrap();

// Second parse
let patterns2 = parse_gram(&serialized).unwrap();

// Semantic equivalence check
assert_eq!(patterns1, patterns2);  // Must be structurally equivalent
```

**Rationale** (from `../gram-hs/docs/reference/features/gram-serialization.md`):
- Tests **semantic preservation** of Pattern structures, not syntactic preservation of gram text
- **Robust**: Independent of formatting differences (whitespace, comments)
- **Canonical**: Verifies that parse → serialize → parse produces identical Pattern structures

**Note**: Formatting may differ (whitespace, comment placement, arrow types), but semantic structure (Pattern tree) must be preserved.

### 3. API Stability

The public API signatures MUST NOT change:

- `parse_gram(input: &str) -> Result<Vec<Pattern>, ParseError>` (existing)
- `validate_gram(input: &str) -> Result<(), ParseError>` (existing)

Internal implementation changes (tree-sitter → nom) are transparent to API consumers.

---

## Migration Notes

### For Users of gram-codec

**No code changes required** if you use the public API:

```rust
// This code works identically before and after migration
use gram_codec::parse_gram;

let patterns = parse_gram("(hello)").unwrap();
```

**Behavior changes** (improvements):

1. **Build simplicity**: No C compiler or emscripten required for WASM builds
2. **Error messages**: May differ slightly in wording, but provide same location info
3. **Performance**: Within 20% of baseline (may be slightly slower, but acceptable)

**No breaking changes** to:
- Function signatures
- Pattern data structures
- Error types (ParseError enum variants may be reordered, but fields unchanged)
- WASM/Python bindings (same JS/Python API)

---

## Testing Requirements

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty_input() {
        assert_eq!(parse_gram("").unwrap(), vec![]);
        assert_eq!(parse_gram("   ").unwrap(), vec![]);
        assert_eq!(parse_gram("// comment").unwrap(), vec![]);
    }

    #[test]
    fn parse_simple_node() {
        let result = parse_gram("(hello)").unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].is_node());
    }

    #[test]
    fn parse_relationship() {
        let result = parse_gram("(a)-->(b)").unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].is_relationship());
    }

    #[test]
    fn parse_subject_pattern() {
        let result = parse_gram("[team | alice, bob]").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].elements.len(), 2);
    }

    #[test]
    fn parse_error_location() {
        let err = parse_gram("(hello\n(world").unwrap_err();
        let location = err.location().unwrap();
        assert_eq!(location.line, 2);
        assert!(location.column > 0);
    }
}
```

### Property Tests

```rust
#[cfg(test)]
mod proptests {
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn round_trip_preserves_structure(s in "\\([a-z]+\\)") {
            if let Ok(patterns) = parse_gram(&s) {
                let serialized = serialize_patterns(&patterns).unwrap();
                let re_parsed = parse_gram(&serialized).unwrap();
                prop_assert_eq!(patterns, re_parsed);
            }
        }
    }
}
```

---

## Contract Summary

| Aspect | Requirement | Verification |
|--------|-------------|--------------|
| API Stability | No signature changes | Compilation check |
| Corpus Conformance | 100% pass rate | Corpus test suite |
| Round-Trip | Structural equivalence | Round-trip tests |
| Performance | Within 20% of baseline | Criterion benchmarks |
| Error Quality | Location + description | Error message tests |
| Thread Safety | No shared state | Code review |
| WASM Compatibility | Builds and runs | WASM example tests |
