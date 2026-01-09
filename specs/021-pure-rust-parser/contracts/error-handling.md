# API Contract: Error Handling

**Feature**: 021-pure-rust-parser  
**Date**: 2026-01-09  
**Purpose**: Define error types, error reporting, and error recovery strategies

## Error Type Hierarchy

### 1. ParseError

Primary error type for parsing failures.

**Definition**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Syntax error at {location}: expected {expected}, found '{found}'")]
    SyntaxError {
        location: Location,
        expected: String,
        found: String,
        context: Vec<String>,
    },
    
    #[error("Unexpected input at {location}: {snippet}")]
    UnexpectedInput {
        location: Location,
        snippet: String,
    },
    
    #[error("Invalid {kind} at {location}: {reason}")]
    InvalidValue {
        location: Location,
        kind: String,
        reason: String,
    },
    
    #[error("Unmatched {delimiter} at {location}")]
    UnmatchedDelimiter {
        location: Location,
        delimiter: char,
    },
    
    #[error("Internal parser error: {message}")]
    Internal {
        message: String,
    },
}
```

**Variant Usage**:

| Variant | When to Use | Example |
|---------|-------------|---------|
| SyntaxError | Expected token not found | Expected ')', found 'abc' |
| UnexpectedInput | Trailing characters after valid parse | Extra text after pattern |
| InvalidValue | Malformed number, string, identifier | Invalid integer: "12a" |
| UnmatchedDelimiter | Bracket/paren/brace mismatch | Unmatched '(' |
| Internal | Logic error (should not occur) | Unexpected nom::Incomplete |

---

### 2. SerializeError

Error type for serialization failures.

**Definition**:
```rust
#[derive(Debug, thiserror::Error)]
pub enum SerializeError {
    #[error("Cannot serialize pattern: {reason}")]
    Unsupported {
        reason: String,
    },
    
    #[error("Invalid pattern structure: {reason}")]
    InvalidStructure {
        reason: String,
    },
}
```

**Variant Usage**:

| Variant | When to Use | Example |
|---------|-------------|---------|
| Unsupported | Value type not in gram notation | Custom enum variant |
| InvalidStructure | Pattern structure ambiguous | 1-element pattern |

---

## Location Information

### Location Type

**Definition**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub line: usize,      // 1-indexed
    pub column: usize,    // 1-indexed
    pub offset: usize,    // 0-indexed byte offset
}
```

**Purpose**: Pinpoint error location in input text for debugging.

**Display Format**:
```rust
impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
```

**Examples**:
```
Input: "(hello)\n(world"
Error at 2:7 (unmatched '(')

Input: "(a {invalid: 12x})"
Error at 1:14 (invalid integer: "12x")
```

---

## Error Messages

### Message Quality Requirements

1. **Specificity**: Clearly state what was expected and what was found
2. **Location**: Include line and column number
3. **Context**: Provide parser context stack (e.g., "in record", "in subject pattern")
4. **Snippet**: Show relevant portion of input near error
5. **Actionability**: Suggest what to fix when possible

### Message Format Template

```
{error_type} at {line}:{column}: {description}

Context: {context_stack}
Found: {snippet}
```

**Example**:

```
Syntax error at 3:15: expected ')', found 'invalid'

Context: subject_pattern > record > string_value
Found: (a {name: "inv...

Hint: Check for missing closing quote or bracket
```

### Error Message Examples

**1. Unmatched Delimiter**:
```rust
ParseError::UnmatchedDelimiter {
    location: Location::new(2, 7, 15),
    delimiter: '(',
}
// Display:
// "Unmatched '(' at 2:7"
```

**2. Syntax Error**:
```rust
ParseError::SyntaxError {
    location: Location::new(1, 10, 9),
    expected: "':'".to_string(),
    found: "{".to_string(),
    context: vec!["node".to_string()],
}
// Display:
// "Syntax error at 1:10: expected ':', found '{'"
// "Context: node"
```

**3. Invalid Value**:
```rust
ParseError::InvalidValue {
    location: Location::new(1, 14, 13),
    kind: "integer".to_string(),
    reason: "invalid digit found in string".to_string(),
}
// Display:
// "Invalid integer at 1:14: invalid digit found in string"
```

---

## Error Recovery Strategies

### Current Approach: Fail Fast

**Strategy**: Return first fatal error encountered, no recovery.

**Rationale**:
- Simpler implementation for MVP
- Clear error reporting (one error at a time)
- Faster parsing (no backtracking for recovery)

**Behavior**:
```rust
// First error stops parsing
let result = parse_gram("(a)\n(b\n(c)");
// Returns error at 2:3 (unmatched '(')
// Does NOT report error on line 3
```

### Future Enhancement: Error Collection

**Strategy** (post-MVP): Continue parsing, collect all errors.

**Benefits**:
- Report multiple syntax errors in one pass
- Better IDE/editor integration
- Improved developer experience

**Implementation Sketch**:
```rust
pub struct ParseErrors {
    pub errors: Vec<ParseError>,
}

pub fn parse_gram_all_errors(input: &str) -> Result<Vec<Pattern>, ParseErrors> {
    // Parse patterns, accumulate errors
    // Return Ok if no errors, Err(ParseErrors) if any
}
```

**Deferred**: Not required for MVP, evaluate based on user feedback.

---

## Error Conversion

### From nom Errors

**Strategy**: Convert nom's `VerboseError` to `ParseError`.

**Implementation**:
```rust
impl ParseError {
    pub fn from_nom_error(
        input: &str, 
        err: nom::Err<nom::error::VerboseError<&str>>
    ) -> Self {
        match err {
            nom::Err::Error(e) | nom::Err::Failure(e) => {
                // Extract first error from error stack
                let (error_input, kind) = e.errors.first()
                    .map(|(i, k)| (*i, k))
                    .unwrap_or((input, &nom::error::VerboseErrorKind::Context("unknown")));
                
                // Calculate location
                let offset = input.len() - error_input.len();
                let location = Location::from_offset(input, offset);
                
                // Extract found text (up to 20 chars)
                let found = error_input.chars().take(20).collect::<String>();
                
                // Format expected description
                let expected = match kind {
                    nom::error::VerboseErrorKind::Char(c) => format!("'{}'", c),
                    nom::error::VerboseErrorKind::Context(s) => s.to_string(),
                    nom::error::VerboseErrorKind::Nom(e) => format!("{:?}", e),
                };
                
                ParseError::SyntaxError {
                    location,
                    expected,
                    found,
                    context: Vec::new(),
                }
            }
            nom::Err::Incomplete(_) => {
                // Should not occur (we use complete parsers)
                ParseError::Internal {
                    message: "Unexpected incomplete parse".to_string(),
                }
            }
        }
    }
}
```

---

## Error Testing

### Test Categories

**1. Syntax Error Detection**:
```rust
#[test]
fn parse_error_unmatched_paren() {
    let err = parse_gram("(hello").unwrap_err();
    match err {
        ParseError::UnmatchedDelimiter { delimiter, .. } => {
            assert_eq!(delimiter, '(');
        }
        _ => panic!("Expected UnmatchedDelimiter error"),
    }
}
```

**2. Error Location Accuracy**:
```rust
#[test]
fn parse_error_location() {
    let err = parse_gram("(hello)\n(world").unwrap_err();
    let location = err.location().unwrap();
    assert_eq!(location.line, 2);
    assert!(location.column > 0);
}
```

**3. Error Message Quality**:
```rust
#[test]
fn parse_error_message() {
    let err = parse_gram("(a {key: 12x})").unwrap_err();
    let message = err.to_string();
    assert!(message.contains("Invalid"));
    assert!(message.contains("1:"));  // Line number
}
```

**4. Error Context**:
```rust
#[test]
fn parse_error_context() {
    let err = parse_gram("[a | (b {key:})]").unwrap_err();
    match err {
        ParseError::SyntaxError { context, .. } => {
            assert!(!context.is_empty());
        }
        _ => panic!("Expected SyntaxError with context"),
    }
}
```

---

## WASM Error Handling

### JavaScript Error Mapping

**Strategy**: Convert Rust errors to JS exceptions with same information.

**Implementation**:
```rust
#[wasm_bindgen]
pub fn parse_gram(input: &str) -> Result<JsValue, JsValue> {
    crate::parse_gram(input)
        .map(|patterns| serde_wasm_bindgen::to_value(&patterns).unwrap())
        .map_err(|e| {
            // Convert ParseError to JS Error object
            let error = js_sys::Error::new(&e.to_string());
            
            // Add location as property if available
            if let Some(loc) = e.location() {
                js_sys::Reflect::set(
                    &error,
                    &"line".into(),
                    &JsValue::from(loc.line as u32),
                ).ok();
                js_sys::Reflect::set(
                    &error,
                    &"column".into(),
                    &JsValue::from(loc.column as u32),
                ).ok();
            }
            
            error.into()
        })
}
```

**JavaScript Usage**:
```javascript
try {
    const patterns = parse_gram("(hello)");
} catch (error) {
    console.error(`Parse error at ${error.line}:${error.column}`);
    console.error(error.message);
}
```

---

## Python Error Handling

### Python Exception Mapping

**Strategy**: Convert Rust errors to Python exceptions with traceback.

**Implementation**:
```rust
#[pyfunction]
pub fn parse_gram(input: &str) -> PyResult<Vec<Pattern>> {
    crate::parse_gram(input)
        .map_err(|e| {
            // Convert ParseError to Python ValueError
            PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("{}", e)
            )
        })
}
```

**Python Usage**:
```python
try:
    patterns = gram_codec.parse_gram("(hello)")
except ValueError as e:
    print(f"Parse error: {e}")
```

---

## Error Contract Summary

| Aspect | Requirement | Verification |
|--------|-------------|--------------|
| Error Types | ParseError, SerializeError | Type definitions |
| Location Info | Line, column, offset | Unit tests |
| Message Quality | Specific, actionable | Error message tests |
| Error Recovery | Fail fast (MVP) | Behavior tests |
| nom Conversion | VerboseError → ParseError | Conversion tests |
| WASM Errors | JS Error with properties | WASM integration tests |
| Python Errors | ValueError exception | Python integration tests |

**Key Invariant**: All parse errors include location information (except Internal errors)
