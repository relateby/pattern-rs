# Research: Pure Rust Gram Parser

**Feature**: 021-pure-rust-parser  
**Date**: 2026-01-09  
**Purpose**: Resolve technical unknowns for nom-based parser implementation

## Overview

This research document addresses key technical decisions and patterns needed to implement a pure Rust parser for gram notation using nom parser combinators, ensuring 100% conformance with the tree-sitter-gram test corpus.

---

## Reference Implementation Guidance

**Primary References**:
- **Porting Guide**: `../gram-hs/docs/reference/PORTING-GUIDE.md` - Implementation order and testing strategies
- **Gram Serialization**: `../gram-hs/docs/reference/features/gram-serialization.md` - Serialization/parsing specification
- **gram-hs CLI**: Can generate test data and provide canonical outputs for conformance testing

**Key Insight from Porting Guide**: Gram serialization (Phase 3) depends on Pattern (Phase 1) and Subject (Phase 2) being complete. This feature focuses on replacing the parser implementation while maintaining the same Pattern/Subject structures.

---

## 1. nom Parser Combinator Architecture

### Decision: nom 7.x with VerboseError and Custom Error Types

**Rationale**:
- **nom 7.x** is the current stable version (latest 7.1.3) with excellent WASM support
- Zero-copy parsing using `&str` and `&[u8]` slices minimizes allocations
- Combinator composition naturally maps to grammar rules
- Strong type safety prevents parser bugs at compile time
- Mature ecosystem with extensive documentation and examples

**Architecture Pattern**:

```rust
// Core parser structure
pub fn parse_gram(input: &str) -> Result<Vec<Pattern>, ParseError> {
    let (remaining, patterns) = gram_patterns(input)
        .map_err(|e| convert_nom_error(input, e))?;
    
    if !remaining.trim().is_empty() {
        return Err(ParseError::UnexpectedInput { 
            remaining: remaining.to_string() 
        });
    }
    
    Ok(patterns)
}

// Grammar rule mapping
fn gram_patterns(input: &str) -> IResult<&str, Vec<Pattern>> {
    many0(delimited(
        multispace0,
        gram_pattern,
        multispace0
    ))(input)
}

fn gram_pattern(input: &str) -> IResult<&str, Pattern> {
    alt((
        annotated_pattern,   // @key(value) pattern
        subject_pattern,     // [id | elements]
        path_pattern,        // (a)-->(b)-->(c) - flattened relationships
        node_pattern,        // (id:Label {props})
    ))(input)
}
```

**Error Handling Strategy**:
- Use `nom::error::VerboseError` during parsing for detailed error context
- Convert to custom `ParseError` type with line/column information
- Provide context stack for nested parse errors
- Include snippet of problematic input for debugging

**Performance Considerations**:
- Zero-copy parsing: nom operates on input slices without allocation
- Combinator inlining: Most parsers inline to tight machine code
- Backtracking: Use `cut()` combinator after commit points to avoid excessive backtracking
- Benchmarking: Target <120ms for 1000 patterns (within 20% of tree-sitter ~100ms)

**Alternatives Considered**:
- **pest**: Requires external grammar file, less flexible than nom combinators, PEG-based (not LR)
- **lalrpop**: LR parser generator, more complex setup, less dynamic than combinators
- **tree-sitter (keep it)**: Rejected due to C dependencies blocking WASM/Python goals
- **Hand-written recursive descent**: More code, harder to maintain than combinator composition

---

## 2. tree-sitter-gram Test Corpus Integration

### Decision: Custom Test Harness with S-Expression Parser

**Corpus Format**: Tree-sitter test corpus files use a custom format:

```
================================================================================
Test Name
================================================================================

input gram notation here
(hello)

--------------------------------------------------------------------------------

(source_file
  (gram_pattern
    (node
      (identifier))))
```

**Test Harness Design**:

1. **Corpus File Parser**:
   - Parse corpus test files from `../tree-sitter-gram/test/corpus/*.txt`
   - Extract test cases: name, input, expected output (S-expression)
   - Handle multiple test cases per file

2. **S-Expression to Pattern Mapping**:
   - Parse expected S-expression output
   - Map tree-sitter node types to Pattern structure expectations
   - Validate that nom parser produces equivalent Pattern structures

3. **Test Execution**:
   ```rust
   #[test]
   fn test_corpus_conformance() {
       let corpus_dir = PathBuf::from("../tree-sitter-gram/test/corpus");
       let test_cases = parse_corpus_files(&corpus_dir).unwrap();
       
       let mut passed = 0;
       let mut failed = 0;
       let mut failures = Vec::new();
       
       for test in test_cases {
           match parse_gram(&test.input) {
               Ok(patterns) => {
                   if validate_patterns_match_expected(&patterns, &test.expected) {
                       passed += 1;
                   } else {
                       failed += 1;
                       failures.push((test.name, "Pattern mismatch"));
                   }
               }
               Err(e) if test.expect_error => passed += 1,
               Err(e) => {
                   failed += 1;
                   failures.push((test.name, format!("Unexpected error: {}", e)));
               }
           }
       }
       
       if !failures.is_empty() {
           panic!("Corpus tests failed: {}/{}\n{:#?}", 
                  failed, passed + failed, failures);
       }
   }
   ```

4. **Incremental Development**:
   - Start with basic node patterns
   - Add relationship patterns
   - Add subject patterns with nesting
   - Add all value types
   - Add annotations and comments
   - Track conformance percentage during development

**Mapping Strategy**:
- Tree-sitter produces concrete syntax tree (CST) with all tokens
- nom parser produces Pattern abstract semantic structure
- Validation compares semantic equivalence, not syntax tree structure
- Example: `(hello)` → Pattern { value: Subject { identifier: Some("hello") }, elements: vec![] }

**Alternatives Considered**:
- **Direct tree-sitter API calls**: Rejected (defeats purpose of removing C dependency)
- **Manual test case porting**: Rejected (loses authoritative test corpus, high maintenance)
- **Ignore corpus, use only custom tests**: Rejected (insufficient conformance validation)
- **Use gram-hs CLI for conformance**: Possible future enhancement - gram-hs CLI can generate test data and provide canonical JSON outputs for cross-validation

---

## 3. Error Handling and Location Tracking

### Decision: Custom ParseError with Span Information

**Error Type Design**:

```rust
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Syntax error at {location}: expected {expected}, found {found}")]
    SyntaxError {
        location: Location,
        expected: String,
        found: String,
        context: Vec<String>,  // Parser context stack
    },
    
    #[error("Unexpected input at {location}: {remaining}")]
    UnexpectedInput {
        location: Location,
        remaining: String,
    },
    
    #[error("Invalid {kind} at {location}: {reason}")]
    InvalidValue {
        location: Location,
        kind: String,  // "number", "string", "identifier"
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}
```

**Location Tracking**:

```rust
// Wrapper parser that tracks location
fn with_location<'a, O, F>(
    parser: F
) -> impl FnMut(&'a str) -> IResult<&'a str, (O, Location)>
where
    F: Parser<&'a str, O, VerboseError<&'a str>>,
{
    move |input: &'a str| {
        let offset = original_input.len() - input.len();
        let location = calculate_location(original_input, offset);
        
        let (remaining, output) = parser.parse(input)?;
        Ok((remaining, (output, location)))
    }
}

fn calculate_location(input: &str, offset: usize) -> Location {
    let prefix = &input[..offset];
    let line = prefix.matches('\n').count() + 1;
    let column = prefix.rfind('\n')
        .map(|pos| offset - pos)
        .unwrap_or(offset + 1);
    
    Location { line, column, offset }
}
```

**Error Recovery**:
- Use `nom::combinator::cut()` after commit points (e.g., after `(` in node pattern)
- Attempt to parse all patterns, collecting errors, before failing
- Provide "did you mean?" suggestions for common mistakes
- Show context snippet with caret pointing to error location

**Alternatives Considered**:
- **Simple String errors**: Rejected (insufficient debugging information)
- **miette crate for error reporting**: Considered for future enhancement, not MVP
- **Error recovery with partial results**: Deferred to post-MVP (complex implementation)

---

## 4. Grammar Mapping: tree-sitter to nom

### Decision: Direct Rule-to-Combinator Mapping

**tree-sitter-gram Grammar Structure** (from `../tree-sitter-gram/src/grammar.json`):

```
gram_pattern → annotated_pattern | subject_pattern | path_pattern | node
node → "(" subject ")"
subject → identifier? labels? record?
subject_pattern → "[" subject "|" pattern_list "]"
path_pattern → node (arrow node)+
relationship → node arrow node
```

**nom Combinator Mapping**:

```rust
// Node: (identifier:Label {props})
fn node(input: &str) -> IResult<&str, Pattern> {
    delimited(
        char('('),
        subject,
        char(')')
    )(input)
    .map(|(i, subj)| (i, Pattern::from_subject(subj)))
}

// Subject: identifier:Label1:Label2 {key: value}
fn subject(input: &str) -> IResult<&str, Subject> {
    let (input, id) = opt(identifier)(input)?;
    let (input, labels) = many0(preceded(char(':'), label))(input)?;
    let (input, record) = opt(record)(input)?;
    
    Ok((input, Subject {
        identifier: id,
        labels: if labels.is_empty() { None } else { Some(labels) },
        record,
    }))
}

// Subject pattern: [team | alice, bob]
fn subject_pattern(input: &str) -> IResult<&str, Pattern> {
    delimited(
        char('['),
        separated_pair(
            subject,
            delimited(ws, char('|'), ws),
            separated_list0(char(','), ws(gram_pattern))
        ),
        char(']')
    )(input)
    .map(|(i, (subj, elements))| (i, Pattern { 
        value: subj.into(), 
        elements 
    }))
}

// Path pattern: (a)-->(b)-->(c) - with arrow direction handling
fn path_pattern(input: &str) -> IResult<&str, Pattern> {
    let (input, first) = node(input)?;
    let (input, segments) = many1(pair(arrow, node))(input)?;
    
    // Flatten into nested pattern structure based on arrow types
    Ok((input, flatten_path(first, segments)))
}

// Arrow types: -->, <--, <-->, ~~, ~>
fn arrow(input: &str) -> IResult<&str, ArrowType> {
    alt((
        value(ArrowType::Right, tag("-->")),
        value(ArrowType::Left, tag("<--")),
        value(ArrowType::Bidirectional, tag("<-->")),
        value(ArrowType::Squiggle, tag("~~")),
        value(ArrowType::SquiggleRight, tag("~>")),
    ))(input)
}
```

**Whitespace and Comment Handling**:

```rust
// Whitespace consumer (spaces, tabs, newlines)
fn ws<'a, F, O>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O>
where
    F: Parser<&'a str, O, VerboseError<&'a str>>,
{
    delimited(multispace0, inner, multispace0)
}

// Comment: // text until end of line
fn comment(input: &str) -> IResult<&str, ()> {
    value(
        (),
        pair(tag("//"), take_until("\n"))
    )(input)
}

// Skip whitespace and comments
fn ws_and_comments(input: &str) -> IResult<&str, ()> {
    value(
        (),
        many0(alt((
            value((), multispace1),
            comment
        )))
    )(input)
}
```

**Precedence and Associativity**:
- Annotations bind tightest: `@key(value) (node)`
- Path flattening is left-associative: `(a)-->(b)-->(c)` → nested from left
- Subject patterns can contain any pattern type recursively

---

## 5. WASM Build Optimization

### Decision: wasm-pack with size optimization flags

**Build Configuration**:

```toml
# Cargo.toml additions
[profile.release]
opt-level = "z"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization (slower build)
strip = true         # Remove debug symbols
panic = "abort"      # Smaller panic handler

[profile.release.package."*"]
opt-level = "z"      # Optimize dependencies for size too
```

**Build Command**:
```bash
wasm-pack build --target web --release \
  crates/gram-codec -- --features wasm
```

**Size Optimization Results** (estimated):
- nom: ~15KB (minimal API surface used)
- pattern-core: ~10KB
- wasm-bindgen glue: ~30KB
- Parser code: ~50KB
- Total (uncompressed): ~150-200KB
- Total (gzipped): ~50-80KB ✅ Well under 500KB target

**Startup Time Optimization**:
- Avoid lazy_static/once_cell for initialization
- Parser combinators are stateless functions (zero init time)
- WASM module loads and is immediately ready

**Alternatives Considered**:
- **wasm-opt post-processing**: May add 5-10% additional size reduction, deferred to polish phase
- **Custom allocator**: Minimal benefit for parser workload, added complexity
- **Stripping unused features**: Already minimal feature set

---

## 6. Unicode and International Character Support

### Decision: Use nom's built-in UTF-8 support

**Strategy**:
- Input is `&str` (guaranteed valid UTF-8 by Rust)
- nom operates on UTF-8 codepoints naturally
- Use `nom::character::complete::*` for Unicode-aware parsing
- Identifiers can contain any Unicode letter/digit/symbol
- Property values preserve Unicode characters exactly

**Implementation**:

```rust
use nom::character::complete::{alpha1, alphanumeric1, char, multispace0};

// Unicode-aware identifier: starts with letter, contains letters/digits
fn identifier(input: &str) -> IResult<&str, String> {
    recognize(pair(
        satisfy(|c| c.is_alphabetic() || c == '_'),
        many0_count(satisfy(|c| c.is_alphanumeric() || c == '_' || c == '-'))
    ))(input)
    .map(|(i, s)| (i, s.to_string()))
}

// Quoted string with Unicode escapes
fn quoted_string(input: &str) -> IResult<&str, String> {
    delimited(
        char('"'),
        escaped_transform(
            none_of("\\\""),
            '\\',
            alt((
                value('\\', char('\\')),
                value('"', char('"')),
                value('\n', char('n')),
                // Unicode escape: \u{1F600}
                preceded(char('u'), unicode_escape),
            ))
        ),
        char('"')
    )(input)
}
```

**Testing Strategy**:
- Test identifiers with emoji: `(😀:Emoji)`
- Test properties with international characters: `{name: "北京"}`
- Test strings with Unicode escapes: `{emoji: "\u{1F600}"}`
- Verify round-trip preserves Unicode exactly

---

## 7. Performance Benchmarking Strategy

### Decision: Criterion benchmarks with comparison to baseline

**Benchmark Suite**:

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_parser_simple_node(c: &mut Criterion) {
    let input = "(hello)";
    c.bench_function("parse_simple_node", |b| {
        b.iter(|| parse_gram(black_box(input)))
    });
}

fn bench_parser_1000_patterns(c: &mut Criterion) {
    // Generate 1000 pattern input
    let input = (0..1000)
        .map(|i| format!("(node_{}:Label {{id: {}}})", i, i))
        .collect::<Vec<_>>()
        .join("\n");
    
    c.bench_function("parse_1000_patterns", |b| {
        b.iter(|| parse_gram(black_box(&input)))
    });
}

fn bench_round_trip(c: &mut Criterion) {
    let input = "[team | (alice:Person), (bob:Person)]";
    c.bench_function("round_trip", |b| {
        b.iter(|| {
            let patterns = parse_gram(black_box(input)).unwrap();
            let serialized = serialize_patterns(&patterns).unwrap();
            parse_gram(&serialized).unwrap()
        })
    });
}

criterion_group!(benches, 
    bench_parser_simple_node,
    bench_parser_1000_patterns,
    bench_round_trip
);
criterion_main!(benches);
```

**Performance Targets** (from spec):
- Parse 1000 patterns: <120ms (target), ~100ms (tree-sitter baseline)
- Simple pattern: <10μs
- Round-trip overhead: <10%

**Comparison Methodology**:
1. Establish baseline with current tree-sitter parser before removal
2. Run benchmarks on identical hardware/input
3. Track performance throughout development
4. Identify bottlenecks if target not met

---

## 8. Round-Trip Testing Strategy

### Decision: Semantic Equivalence via Pattern Comparison

**Correct Round-Trip Test** (from gram-hs guidance):

```rust
// CORRECT: gram -> pattern -> gram -> pattern
// Tests semantic equivalence of Pattern structures

let original_gram = "(a:Label {key: \"value\"})";

// First parse
let patterns1 = parse_gram(original_gram).unwrap();

// Serialize
let serialized = serialize_patterns(&patterns1).unwrap();

// Second parse (from serialized output)
let patterns2 = parse_gram(&serialized).unwrap();

// Compare Pattern structures (semantic equivalence)
assert_eq!(patterns1, patterns2);  // ✅ Tests semantic preservation
```

**Why NOT just `gram -> pattern -> gram`?**

```rust
// INCORRECT: Only tests syntactic round-trip
let original = "(a:Label {key: \"value\"})";
let patterns = parse_gram(original).unwrap();
let serialized = serialize_patterns(&patterns).unwrap();

// This would fail due to formatting differences:
// Original:  "(a:Label {key: \"value\"})"
// Serialized: "(a:Label {key: \"value\"})"  // Might have different spacing
assert_eq!(original, serialized);  // ❌ Brittle, fails on formatting changes
```

**Rationale** (from `../gram-hs/docs/reference/features/gram-serialization.md`):

- **Semantic Equivalence**: Round-trip tests verify "structural equality after serialization/deserialization cycles"
- **Formatting Independence**: Gram notation formatting (whitespace, comment placement) may vary, but Pattern semantics must be preserved
- **Robust Testing**: Comparing Pattern structures is more reliable than string comparison

**Implementation**:

```rust
#[test]
fn test_round_trip_semantic_equivalence() {
    let test_cases = [
        "(hello)",
        "(a:Person {name: \"Alice\", age: 30})",
        "(a)-->(b)",
        "[team | (alice), (bob)]",
        "[nested | [inner | (leaf)]]",
        "// comment\n(a) // inline comment\n(b)",  // Comments stripped
    ];
    
    for input in test_cases {
        // Parse original
        let patterns1 = parse_gram(input).unwrap();
        
        // Serialize
        let serialized = serialize_patterns(&patterns1).unwrap();
        
        // Parse serialized output
        let patterns2 = parse_gram(&serialized).unwrap();
        
        // Semantic equivalence check
        assert_eq!(
            patterns1, patterns2,
            "Round-trip semantic equivalence failed for input: {}\n\
             Serialized as: {}",
            input, serialized
        );
    }
}
```

**Property-Based Testing**:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn round_trip_preserves_semantics(pattern in arbitrary_pattern()) {
        // Serialize
        let gram1 = serialize_pattern(&pattern).unwrap();
        
        // Parse
        let parsed = parse_gram(&gram1).unwrap();
        assert_eq!(parsed.len(), 1);
        
        // Serialize again
        let gram2 = serialize_pattern(&parsed[0]).unwrap();
        
        // Parse again
        let re_parsed = parse_gram(&gram2).unwrap();
        
        // Both parsed results should be identical
        prop_assert_eq!(parsed[0], re_parsed[0]);
    }
}
```

**Expected Invariants**:

1. **Structure Preservation**: Pattern tree structure (elements, nesting) is identical
2. **Value Preservation**: All Subject values (identifiers, labels, properties) are identical
3. **Ordering Preservation**: Element order is maintained
4. **Type Preservation**: All Value types (String, Integer, Boolean, etc.) are preserved

**Not Preserved** (acceptable differences):

1. **Whitespace**: Spacing may be normalized
2. **Comments**: Comments are not preserved during parsing (intentional)
3. **Arrow Types**: All relationship arrows normalize to `-->` in serialization
4. **Quoted vs Unquoted**: Identifiers may be quoted in serialized form if needed for escaping

---

## Research Summary

| Topic | Decision | Key Benefits | Risks Mitigated |
|-------|----------|--------------|-----------------|
| Parser Library | nom 7.x with combinators | Zero-copy, composable, type-safe | Build complexity eliminated |
| Test Corpus | Custom harness + S-expr parser | Authoritative conformance validation | Behavior divergence from spec |
| Error Handling | Custom types with location tracking | Helpful error messages | Poor debugging experience |
| Grammar Mapping | Direct rule-to-combinator | Clear correspondence to grammar | Implementation inconsistency |
| WASM Optimization | Profile flags + wasm-pack | <100KB gzipped | Binary size bloat |
| Unicode Support | Native UTF-8 in nom | International character support | Encoding edge cases |
| Performance | Criterion benchmarks | Objective comparison to baseline | Performance regression |

**Next Steps**: Proceed to Phase 1 (Design) to define data models and API contracts based on this research.
