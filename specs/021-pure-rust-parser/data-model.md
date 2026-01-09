# Data Model: Pure Rust Gram Parser

**Feature**: 021-pure-rust-parser  
**Date**: 2026-01-09  
**Purpose**: Define parser architecture, types, and state management

## Overview

This document defines the data structures and types for the nom-based pure Rust parser implementation. The parser transforms gram notation (text) into Pattern data structures using composable parser combinators.

---

## 1. Parser Module Structure

### Module Hierarchy

```rust
crates/gram-codec/src/
├── lib.rs                  // Public API
├── parser/
│   ├── mod.rs             // Parser module exports
│   ├── types.rs           // Core parser types (Location, Span, ArrowType)
│   ├── error.rs           // Error types and conversion
│   ├── combinators.rs     // Utility combinators (ws, ws_and_comments)
│   ├── node.rs            // Node pattern parsing
│   ├── relationship.rs    // Relationship and path pattern parsing
│   ├── subject.rs         // Subject and subject pattern parsing
│   ├── annotation.rs      // Annotation parsing
│   ├── value.rs           // Value type parsing (strings, numbers, arrays, etc.)
│   └── comment.rs         // Comment and whitespace handling
└── serializer/
    ├── mod.rs             // Serializer module exports
    ├── pattern.rs         // Pattern serialization dispatch
    ├── subject.rs         // Subject serialization
    ├── escape.rs          // Escaping utilities
    └── format.rs          // Formatting utilities
```

---

## 2. Core Parser Types

### 2.1 Location and Span Tracking

```rust
/// Represents a location in the input text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)  
    pub column: usize,
    /// Byte offset from start (0-indexed)
    pub offset: usize,
}

impl Location {
    pub fn new(line: usize, column: usize, offset: usize) -> Self {
        Self { line, column, offset }
    }
    
    pub fn from_offset(input: &str, offset: usize) -> Self {
        let prefix = &input[..offset.min(input.len())];
        let line = prefix.matches('\n').count() + 1;
        let column = prefix.rfind('\n')
            .map(|pos| offset - pos)
            .unwrap_or(offset + 1);
        
        Self { line, column, offset }
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Represents a span of text in the input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Span {
    pub fn new(start: Location, end: Location) -> Self {
        Self { start, end }
    }
    
    pub fn single(location: Location) -> Self {
        Self { start: location, end: location }
    }
}
```

### 2.2 Arrow Types for Relationships

```rust
/// Relationship arrow types from gram notation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrowType {
    /// Right arrow: -->
    Right,
    /// Left arrow: <--
    Left,
    /// Bidirectional arrow: <-->
    Bidirectional,
    /// Undirected/squiggle: ~~
    Squiggle,
    /// Squiggle right: ~>
    SquiggleRight,
}

impl ArrowType {
    /// Returns true if arrow implies left-to-right directionality
    pub fn is_forward(&self) -> bool {
        matches!(self, ArrowType::Right | ArrowType::SquiggleRight)
    }
    
    /// Returns true if arrow implies right-to-left directionality
    pub fn is_backward(&self) -> bool {
        matches!(self, ArrowType::Left)
    }
    
    /// Returns true if arrow is bidirectional
    pub fn is_bidirectional(&self) -> bool {
        matches!(self, ArrowType::Bidirectional)
    }
    
    /// Returns true if arrow is undirected (squiggle)
    pub fn is_undirected(&self) -> bool {
        matches!(self, ArrowType::Squiggle)
    }
}

impl std::fmt::Display for ArrowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ArrowType::Right => "-->",
            ArrowType::Left => "<--",
            ArrowType::Bidirectional => "<-->",
            ArrowType::Squiggle => "~~",
            ArrowType::SquiggleRight => "~>",
        };
        write!(f, "{}", s)
    }
}
```

### 2.3 Intermediate Parsing Types

```rust
/// Intermediate type for annotations during parsing
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Annotation {
    pub key: String,
    pub value: Option<String>,
    pub span: Span,
}

/// Intermediate type for path segments during parsing
#[derive(Debug, Clone)]
pub(crate) struct PathSegment {
    pub arrow: ArrowType,
    pub node: Pattern,
}

/// Parsing context for error reporting
#[derive(Debug, Clone)]
pub(crate) struct ParseContext {
    /// Original input for location calculation
    pub input: String,
    /// Stack of parsing contexts (e.g., "node", "subject_pattern", "record")
    pub context_stack: Vec<String>,
}

impl ParseContext {
    pub fn new(input: String) -> Self {
        Self {
            input,
            context_stack: Vec::new(),
        }
    }
    
    pub fn push(&mut self, context: impl Into<String>) {
        self.context_stack.push(context.into());
    }
    
    pub fn pop(&mut self) {
        self.context_stack.pop();
    }
    
    pub fn current(&self) -> Option<&str> {
        self.context_stack.last().map(|s| s.as_str())
    }
}
```

---

## 3. Error Types

### 3.1 Main Error Type

```rust
use thiserror::Error;

/// Errors that can occur during parsing
#[derive(Debug, Error)]
pub enum ParseError {
    /// Syntax error with location and context
    #[error("Syntax error at {location}: expected {expected}, found '{found}'")]
    SyntaxError {
        location: Location,
        expected: String,
        found: String,
        context: Vec<String>,
    },
    
    /// Unexpected input after successful parse
    #[error("Unexpected input at {location}: {snippet}")]
    UnexpectedInput {
        location: Location,
        snippet: String,
    },
    
    /// Invalid value (number, string, identifier)
    #[error("Invalid {kind} at {location}: {reason}")]
    InvalidValue {
        location: Location,
        kind: String,
        reason: String,
    },
    
    /// Unmatched delimiter (bracket, paren, brace)
    #[error("Unmatched {delimiter} at {location}")]
    UnmatchedDelimiter {
        location: Location,
        delimiter: char,
    },
    
    /// Internal parser error (should not occur in production)
    #[error("Internal parser error: {message}")]
    Internal {
        message: String,
    },
}

impl ParseError {
    /// Create a syntax error from nom's VerboseError
    pub fn from_nom_error(input: &str, err: nom::Err<nom::error::VerboseError<&str>>) -> Self {
        match err {
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                let (error_input, kind) = e.errors.first()
                    .map(|(i, k)| (*i, k))
                    .unwrap_or((input, &nom::error::VerboseErrorKind::Context("unknown")));
                
                let offset = input.len() - error_input.len();
                let location = Location::from_offset(input, offset);
                
                let found = error_input.chars().take(20).collect::<String>();
                let expected = format!("{:?}", kind);
                
                ParseError::SyntaxError {
                    location,
                    expected,
                    found,
                    context: Vec::new(),
                }
            }
            nom::Err::Incomplete(_) => {
                ParseError::Internal {
                    message: "Unexpected incomplete parse (streaming not supported)".to_string(),
                }
            }
        }
    }
    
    /// Get the location of this error
    pub fn location(&self) -> Option<Location> {
        match self {
            ParseError::SyntaxError { location, .. }
            | ParseError::UnexpectedInput { location, .. }
            | ParseError::InvalidValue { location, .. }
            | ParseError::UnmatchedDelimiter { location, .. } => Some(*location),
            ParseError::Internal { .. } => None,
        }
    }
    
    /// Add context to this error
    pub fn with_context(mut self, context: String) -> Self {
        if let ParseError::SyntaxError { context: ref mut ctx, .. } = self {
            ctx.push(context);
        }
        self
    }
}
```

### 3.2 Serialization Error Types

```rust
/// Errors that can occur during serialization
#[derive(Debug, Error)]
pub enum SerializeError {
    /// Pattern cannot be represented in gram notation
    #[error("Cannot serialize pattern: {reason}")]
    Unsupported {
        reason: String,
    },
    
    /// Invalid pattern structure for serialization
    #[error("Invalid pattern structure: {reason}")]
    InvalidStructure {
        reason: String,
    },
    
    /// IO error during serialization
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

---

## 4. Parser State and Result Types

### 4.1 nom IResult Type Alias

```rust
use nom::{IResult, error::VerboseError};

/// Type alias for nom parser results with verbose errors
pub(crate) type ParseResult<'a, O> = IResult<&'a str, O, VerboseError<&'a str>>;
```

### 4.2 Parser Function Signatures

```rust
// Top-level parsing function
pub fn parse_gram(input: &str) -> Result<Vec<Pattern>, ParseError>;

// Individual parser combinators (internal)
pub(crate) fn gram_pattern(input: &str) -> ParseResult<Pattern>;
pub(crate) fn node(input: &str) -> ParseResult<Pattern>;
pub(crate) fn subject_pattern(input: &str) -> ParseResult<Pattern>;
pub(crate) fn path_pattern(input: &str) -> ParseResult<Pattern>;
pub(crate) fn annotated_pattern(input: &str) -> ParseResult<Pattern>;
pub(crate) fn subject(input: &str) -> ParseResult<Subject>;
pub(crate) fn value(input: &str) -> ParseResult<Value>;
pub(crate) fn identifier(input: &str) -> ParseResult<String>;
pub(crate) fn label(input: &str) -> ParseResult<String>;
pub(crate) fn record(input: &str) -> ParseResult<HashMap<String, Value>>;
```

---

## 5. Value Type Enumeration

### 5.1 Value Enum

```rust
use std::collections::HashMap;

/// Represents all possible value types in gram notation
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// String value (quoted or unquoted symbol)
    String(String),
    
    /// Integer value
    Integer(i64),
    
    /// Decimal/floating-point value
    Decimal(f64),
    
    /// Boolean value
    Boolean(bool),
    
    /// Array of homogeneous scalar values
    Array(Vec<Value>),
    
    /// Range with lower and upper bounds
    Range {
        lower: Box<Value>,
        upper: Box<Value>,
        inclusive: bool,  // true for 1..10, false for 1...10
    },
    
    /// Tagged string with format tag (e.g., """markdown""")
    TaggedString {
        tag: String,
        content: String,
    },
    
    /// Symbol (unquoted identifier-like value)
    Symbol(String),
}

impl Value {
    /// Returns true if this value is a scalar (not array or complex type)
    pub fn is_scalar(&self) -> bool {
        !matches!(self, Value::Array(_) | Value::Range { .. })
    }
    
    /// Attempt to convert to string
    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) | Value::Symbol(s) => Some(s),
            _ => None,
        }
    }
    
    /// Attempt to convert to integer
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }
}
```

---

## 6. Parser Combinator Utilities

### 6.1 Whitespace Handling

```rust
use nom::character::complete::{multispace0, multispace1};
use nom::combinator::value;
use nom::sequence::{delimited, pair};
use nom::branch::alt;
use nom::multi::many0;
use nom::bytes::complete::{tag, take_until};

/// Consume whitespace and comments
pub(crate) fn ws(input: &str) -> ParseResult<()> {
    value(
        (),
        many0(alt((
            value((), multispace1),
            comment,
        )))
    )(input)
}

/// Parse a comment: // text until newline
pub(crate) fn comment(input: &str) -> ParseResult<()> {
    value(
        (),
        pair(
            tag("//"),
            alt((
                take_until("\n"),
                nom::combinator::rest,  // Comment at end of file
            ))
        )
    )(input)
}

/// Wrap a parser with optional leading/trailing whitespace
pub(crate) fn padded<'a, O, F>(parser: F) -> impl FnMut(&'a str) -> ParseResult<O>
where
    F: nom::Parser<&'a str, O, VerboseError<&'a str>>,
{
    delimited(ws, parser, ws)
}
```

### 6.2 Location Tracking Combinator

```rust
use nom::Parser;

/// Wrap a parser to track its location in the input
pub(crate) fn with_span<'a, O, F>(
    mut parser: F,
) -> impl FnMut(&'a str) -> ParseResult<(O, Span)>
where
    F: Parser<&'a str, O, VerboseError<&'a str>>,
{
    move |original_input: &'a str| {
        let start_offset = original_input.len();
        let (remaining, output) = parser.parse(original_input)?;
        let end_offset = remaining.len();
        
        let span = Span {
            start: Location::from_offset(original_input, start_offset),
            end: Location::from_offset(original_input, end_offset),
        };
        
        Ok((remaining, (output, span)))
    }
}
```

---

## 7. Pattern Construction Helpers

### 7.1 Pattern Builder Functions

```rust
impl Pattern {
    /// Create a node pattern (0 elements) from a subject
    pub fn from_subject(subject: Subject) -> Self {
        Self {
            value: subject.into(),
            elements: Vec::new(),
        }
    }
    
    /// Create a relationship pattern (2 elements)
    pub fn relationship(left: Pattern, right: Pattern, arrow: ArrowType) -> Self {
        // Handle arrow directionality
        let (first, second) = match arrow {
            ArrowType::Left => (right, left),  // Reverse for left arrow
            _ => (left, right),
        };
        
        Self {
            value: Subject::default().into(),
            elements: vec![first, second],
        }
    }
    
    /// Create a subject pattern with elements
    pub fn with_elements(subject: Subject, elements: Vec<Pattern>) -> Self {
        Self {
            value: subject.into(),
            elements,
        }
    }
    
    /// Check if this pattern is a simple node (0 elements)
    pub fn is_node(&self) -> bool {
        self.elements.is_empty()
    }
    
    /// Check if this pattern is a relationship (2 atomic elements)
    pub fn is_relationship(&self) -> bool {
        self.elements.len() == 2 
            && self.elements.iter().all(|e| e.is_node())
    }
}
```

---

## 8. Testing Data Structures

### 8.1 Corpus Test Case

```rust
/// Represents a single test case from the tree-sitter-gram corpus
#[derive(Debug, Clone)]
pub struct CorpusTest {
    /// Test name
    pub name: String,
    
    /// Input gram notation
    pub input: String,
    
    /// Expected S-expression output (from tree-sitter)
    pub expected_sexp: String,
    
    /// Whether this test expects an error
    pub expect_error: bool,
    
    /// Source file and line number
    pub source: TestSource,
}

#[derive(Debug, Clone)]
pub struct TestSource {
    pub file: String,
    pub line: usize,
}

impl CorpusTest {
    /// Validate that parsed patterns match expected structure
    pub fn validate(&self, patterns: &[Pattern]) -> Result<(), String> {
        // Parse expected S-expression
        // Compare semantic structure
        // Return Ok(()) if match, Err(message) if mismatch
        todo!("Implement S-expression validation")
    }
    
    /// Validate round-trip semantic equivalence
    /// Per gram-hs guidance: gram -> pattern -> gram -> pattern
    pub fn validate_round_trip(&self) -> Result<(), String> {
        // First parse
        let patterns1 = parse_gram(&self.input)
            .map_err(|e| format!("First parse failed: {}", e))?;
        
        // Serialize
        let serialized = serialize_patterns(&patterns1)
            .map_err(|e| format!("Serialization failed: {}", e))?;
        
        // Second parse
        let patterns2 = parse_gram(&serialized)
            .map_err(|e| format!("Second parse failed: {}", e))?;
        
        // Check semantic equivalence
        if patterns1 == patterns2 {
            Ok(())
        } else {
            Err(format!(
                "Round-trip semantic equivalence failed\n\
                 Original: {}\n\
                 Serialized: {}\n\
                 Pattern structures differ",
                self.input, serialized
            ))
        }
    }
}
```

### 8.2 Corpus Test Collection

```rust
/// Collection of all corpus tests
#[derive(Debug)]
pub struct CorpusTestSuite {
    pub tests: Vec<CorpusTest>,
}

impl CorpusTestSuite {
    /// Load all corpus tests from directory
    pub fn load(corpus_dir: &Path) -> Result<Self, std::io::Error> {
        let mut tests = Vec::new();
        
        for entry in std::fs::read_dir(corpus_dir)? {
            let path = entry?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("txt") {
                let file_tests = parse_corpus_file(&path)?;
                tests.extend(file_tests);
            }
        }
        
        Ok(Self { tests })
    }
    
    /// Run all tests and return results
    pub fn run(&self) -> CorpusTestResults {
        let mut results = CorpusTestResults::new();
        
        for test in &self.tests {
            let result = match parse_gram(&test.input) {
                Ok(patterns) if test.expect_error => {
                    TestResult::Failure {
                        reason: "Expected error but parsing succeeded".to_string(),
                    }
                }
                Ok(patterns) => {
                    match test.validate(&patterns) {
                        Ok(()) => TestResult::Pass,
                        Err(msg) => TestResult::Failure { reason: msg },
                    }
                }
                Err(e) if test.expect_error => TestResult::Pass,
                Err(e) => TestResult::Failure {
                    reason: format!("Parse error: {}", e),
                },
            };
            
            results.add(test.name.clone(), result);
        }
        
        results
    }
}

#[derive(Debug)]
pub enum TestResult {
    Pass,
    Failure { reason: String },
}

#[derive(Debug)]
pub struct CorpusTestResults {
    pub results: HashMap<String, TestResult>,
    pub passed: usize,
    pub failed: usize,
}

impl CorpusTestResults {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
            passed: 0,
            failed: 0,
        }
    }
    
    pub fn add(&mut self, name: String, result: TestResult) {
        match result {
            TestResult::Pass => self.passed += 1,
            TestResult::Failure { .. } => self.failed += 1,
        }
        self.results.insert(name, result);
    }
    
    pub fn total(&self) -> usize {
        self.passed + self.failed
    }
    
    pub fn pass_rate(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            (self.passed as f64) / (self.total() as f64)
        }
    }
}
```

---

## Data Model Summary

| Component | Purpose | Key Types |
|-----------|---------|-----------|
| Location/Span | Error reporting and debugging | Location, Span |
| ArrowType | Relationship directionality | ArrowType enum |
| ParseError | Error handling and messages | ParseError enum |
| Value | Property value types | Value enum (8 variants) |
| Parser Combinators | Parsing logic | nom IResult functions |
| Pattern Builders | Pattern construction | Pattern helper methods |
| Corpus Testing | Conformance validation | CorpusTest, CorpusTestSuite |

**Relationships**:
- Parser combinators consume input (`&str`) and produce `Pattern` structures
- `ParseError` provides detailed location and context information
- `Value` enum represents all gram notation value types in pattern properties
- `CorpusTest` validates parser output against tree-sitter-gram expectations

**Invariants**:
- All `Location` offsets must be valid UTF-8 boundaries
- `Pattern.elements` length determines serialization strategy (0→node, 2→relationship, other→subject)
- `ArrowType` affects element ordering in relationships (Left arrow reverses elements)
