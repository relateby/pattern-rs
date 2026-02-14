# Phase 7: AST Output Implementation

**Goal**: Add `parse_to_ast()` function to pattern-rs for language-agnostic consumption by gram-js and gram-py

**Status**: Planning  
**Priority**: High  
**Dependencies**: Phase 6 complete (100% parser conformance)

---

## Overview

Currently pattern-rs returns `Vec<Pattern<Subject>>` which is:
- ✅ Correct and complete
- ✅ Works great in Rust
- ❌ Not easily consumable across FFI boundaries
- ❌ Requires full Pattern API in WASM/Python

**Solution**: Add AST output that mirrors Pattern<Subject> structure in JSON-serializable format.

---

## Implementation Tasks

### T083: Define AST Types (Core)

**File**: `crates/gram-codec/src/ast.rs`

```rust
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Abstract Syntax Tree for gram notation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AstPattern {
    pub subject: AstSubject,
    pub elements: Vec<AstPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AstSubject {
    pub identity: String,
    pub labels: Vec<String>,
    pub properties: HashMap<String, serde_json::Value>,
}
```

**Acceptance Criteria**:
- [ ] Types compile without errors
- [ ] Derive Serialize/Deserialize for JSON output
- [ ] Use camelCase for JSON field names (JavaScript convention)
- [ ] Add comprehensive documentation

**Estimated Time**: 30 minutes

---

### T084: Implement Pattern to AST Conversion

**File**: `crates/gram-codec/src/ast.rs`

```rust
use pattern_core::{Pattern, Subject, Value};

impl AstPattern {
    /// Convert from native Pattern<Subject> to AST
    pub fn from_pattern(pattern: &Pattern<Subject>) -> Self {
        let subject = pattern.value();
        
        AstPattern {
            subject: AstSubject {
                identity: subject.identity.0.clone(),
                labels: subject.labels.iter().cloned().collect(),
                properties: subject.properties.iter()
                    .map(|(k, v)| (k.clone(), value_to_json(v)))
                    .collect(),
            },
            elements: pattern.elements().iter()
                .map(|e| AstPattern::from_pattern(e))
                .collect(),
        }
    }
}

fn value_to_json(value: &Value) -> serde_json::Value {
    use pattern_core::Value;
    match value {
        Value::VNull => serde_json::Value::Null,
        Value::VBoolean(b) => serde_json::Value::Bool(*b),
        Value::VInteger(i) => serde_json::Value::Number((*i).into()),
        Value::VDecimal(d) => serde_json::json!(*d),
        Value::VString(s) => serde_json::Value::String(s.clone()),
        Value::VSymbol(sym) => serde_json::json!({
            "type": "Symbol",
            "value": sym.0.clone()
        }),
        Value::VArray(arr) => serde_json::Value::Array(
            arr.iter().map(value_to_json).collect()
        ),
        Value::VMap(map) => serde_json::Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect()
        ),
        Value::VRange(range) => serde_json::json!({
            "type": "Range",
            "lower": range.lower,
            "upper": range.upper
        }),
        Value::VMeasurement { unit, value } => serde_json::json!({
            "type": "Measurement",
            "value": value,
            "unit": unit
        }),
        Value::VTagged { tag, value } => serde_json::json!({
            "type": "Tagged",
            "tag": tag,
            "value": value
        }),
        Value::VRecord(rec) => serde_json::Value::Object(
            rec.0.iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect()
        ),
    }
}
```

**Acceptance Criteria**:
- [ ] All Value types correctly converted to JSON
- [ ] Nested patterns handled recursively
- [ ] No data loss in conversion
- [ ] Handle empty collections correctly

**Estimated Time**: 1 hour

---

### T085: Implement parse_to_ast Function

**File**: `crates/gram-codec/src/lib.rs` and `src/ast.rs`

```rust
/// Parse gram notation to AST
/// 
/// Returns a single AstPattern representing the file-level pattern.
/// This is the recommended output format for cross-language consumption.
///
/// # Arguments
///
/// * `input` - Gram notation text to parse
///
/// # Returns
///
/// * `Ok(AstPattern)` - The parsed pattern as AST
/// * `Err(ParseError)` - If parsing fails
///
/// # Example
///
/// ```rust
/// use gram_codec::parse_to_ast;
///
/// let ast = parse_to_ast("(alice:Person {name: \"Alice\"})")?;
/// println!("Identity: {}", ast.subject.identity);
/// println!("Labels: {:?}", ast.subject.labels);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn parse_to_ast(input: &str) -> Result<AstPattern, ParseError> {
    let patterns = parse_gram(input)?;
    
    // Parser always returns a single file-level pattern
    match patterns.into_iter().next() {
        Some(pattern) => Ok(AstPattern::from_pattern(&pattern)),
        None => {
            // Empty file - return empty pattern
            Ok(AstPattern {
                subject: AstSubject {
                    identity: String::new(),
                    labels: vec![],
                    properties: HashMap::new(),
                },
                elements: vec![],
            })
        }
    }
}
```

**Acceptance Criteria**:
- [ ] Function compiles and works correctly
- [ ] Comprehensive documentation
- [ ] Handles empty files
- [ ] Returns single pattern (file-level)
- [ ] Error handling matches parse_gram

**Estimated Time**: 30 minutes

---

### T086: Add WASM Bindings for AST

**File**: `crates/gram-codec/src/wasm.rs`

```rust
use wasm_bindgen::prelude::*;
use crate::ast::AstPattern;

/// Parse gram notation to AST (JavaScript-friendly)
///
/// Returns a single pattern as a JavaScript object.
/// This is the recommended way to parse gram in JavaScript/TypeScript.
///
/// # Arguments
///
/// * `input` - Gram notation text
///
/// # Returns
///
/// JavaScript object with structure:
/// ```javascript
/// {
///   subject: {
///     identity: string,
///     labels: string[],
///     properties: object
///   },
///   elements: AstPattern[]
/// }
/// ```
///
/// # Example
///
/// ```javascript
/// import init, { parse_to_ast } from './gram_codec.js';
/// await init();
///
/// const ast = parse_to_ast("(alice:Person {name: 'Alice'})");
/// console.log(ast.subject.identity);  // "alice"
/// console.log(ast.subject.labels);    // ["Person"]
/// ```
#[wasm_bindgen]
pub fn parse_to_ast(input: &str) -> Result<JsValue, JsValue> {
    let ast = crate::parse_to_ast(input)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;
    
    // Serialize to JsValue
    serde_wasm_bindgen::to_value(&ast)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
}
```

**Dependencies**: Add `serde-wasm-bindgen` to Cargo.toml

**Acceptance Criteria**:
- [ ] Function exposed to JavaScript
- [ ] Returns proper JavaScript object (not opaque type)
- [ ] Documentation includes JavaScript examples
- [ ] Error handling works correctly

**Estimated Time**: 45 minutes

---

### T087: Add Python Bindings for AST

**File**: `crates/gram-codec/src/python.rs`

```rust
use pyo3::prelude::*;
use crate::ast::AstPattern;

/// Parse gram notation to AST (Python dict)
///
/// Returns a single pattern as a Python dictionary.
/// This is the recommended way to parse gram in Python.
///
/// # Arguments
///
/// * `input` (str) - Gram notation text
///
/// # Returns
///
/// Dictionary with structure:
/// ```python
/// {
///   'subject': {
///     'identity': str,
///     'labels': list[str],
///     'properties': dict
///   },
///   'elements': list[dict]
/// }
/// ```
///
/// # Example
///
/// ```python
/// import gram_codec
///
/// ast = gram_codec.parse_to_ast("(alice:Person {name: 'Alice'})")
/// print(ast['subject']['identity'])  # "alice"
/// print(ast['subject']['labels'])    # ["Person"]
/// ```
#[pyfunction]
fn parse_to_ast(input: &str) -> PyResult<PyObject> {
    let ast = crate::parse_to_ast(input)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Parse error: {}", e)
        ))?;
    
    // Convert to Python dict
    Python::with_gil(|py| {
        pythonize::pythonize(py, &ast)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Serialization error: {}", e)
            ))
    })
}

// Add to module initialization
#[pymodule]
fn gram_codec(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // ... existing functions ...
    m.add_function(wrap_pyfunction!(parse_to_ast, m)?)?;
    Ok(())
}
```

**Dependencies**: Add `pythonize` to Cargo.toml

**Acceptance Criteria**:
- [ ] Function exposed to Python
- [ ] Returns proper Python dict (not opaque type)
- [ ] Documentation includes Python examples
- [ ] Error handling works correctly

**Estimated Time**: 45 minutes

---

### T088: Add Unit Tests for AST Conversion

**File**: `crates/gram-codec/src/ast.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_node() {
        let ast = crate::parse_to_ast("(alice:Person {name: \"Alice\", age: 30})").unwrap();
        assert_eq!(ast.subject.identity, "alice");
        assert_eq!(ast.subject.labels, vec!["Person"]);
        assert_eq!(ast.subject.properties.len(), 2);
        assert_eq!(ast.elements.len(), 0);
    }
    
    #[test]
    fn test_path_pattern() {
        let ast = crate::parse_to_ast("(alice)-->(bob)").unwrap();
        // File-level pattern
        assert_eq!(ast.subject.identity, "");
        assert!(ast.elements.len() >= 2); // Depends on parser structure
    }
    
    #[test]
    fn test_subject_pattern() {
        let ast = crate::parse_to_ast("[team | (alice), (bob)]").unwrap();
        assert_eq!(ast.subject.identity, "team");
        assert_eq!(ast.elements.len(), 2);
    }
    
    #[test]
    fn test_file_with_properties() {
        let ast = crate::parse_to_ast("{version: \"1.0\"}\n(alice)").unwrap();
        assert!(ast.subject.properties.contains_key("version"));
        assert_eq!(ast.elements.len(), 1);
    }
    
    #[test]
    fn test_empty_file() {
        let ast = crate::parse_to_ast("").unwrap();
        assert_eq!(ast.subject.identity, "");
        assert_eq!(ast.elements.len(), 0);
    }
    
    #[test]
    fn test_all_value_types() {
        let gram = r#"(node {
            null: null,
            bool: true,
            int: 42,
            decimal: 3.14,
            string: "hello",
            array: [1, 2, 3],
            map: {key: "value"},
            range: 1..10,
            measurement: 168cm,
            tagged: date`2024-01-09`
        })"#;
        
        let ast = crate::parse_to_ast(gram).unwrap();
        let props = &ast.subject.properties;
        assert!(props.contains_key("null"));
        assert!(props.contains_key("bool"));
        assert!(props.contains_key("int"));
        // ... test all types
    }
    
    #[test]
    fn test_deeply_nested() {
        let ast = crate::parse_to_ast("[a | [b | [c | (d)]]]").unwrap();
        assert_eq!(ast.subject.identity, "a");
        assert_eq!(ast.elements.len(), 1);
        assert_eq!(ast.elements[0].subject.identity, "b");
    }
    
    #[test]
    fn test_json_serialization() {
        let ast = crate::parse_to_ast("(alice:Person {name: \"Alice\"})").unwrap();
        let json = serde_json::to_string(&ast).unwrap();
        assert!(json.contains("alice"));
        assert!(json.contains("Person"));
        
        // Test round-trip
        let deserialized: AstPattern = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.subject.identity, "alice");
    }
}
```

**Acceptance Criteria**:
- [ ] All tests pass
- [ ] Cover all Value types
- [ ] Cover nested patterns
- [ ] Cover edge cases (empty, file-level properties)
- [ ] Test JSON serialization round-trip

**Estimated Time**: 1 hour

---

### T089: Update Examples to Use AST

**Files**: 
- `examples/gram-codec-wasm-node/index.js`
- `examples/gram-codec-python/example.py`

Add new sections demonstrating AST usage:

**WASM Example**:
```javascript
console.log("\n=== AST Output ===");

// Parse to AST (language-agnostic format)
const ast = parse_to_ast("(alice:Person {name: 'Alice', age: 30})-[:KNOWS]->(bob)");

console.log("File-level pattern:");
console.log("  Identity:", ast.subject.identity);
console.log("  Labels:", ast.subject.labels);
console.log("  Properties:", ast.subject.properties);
console.log("  Elements:", ast.elements.length);

if (ast.elements.length > 0) {
    console.log("\nFirst element:");
    console.log("  Identity:", ast.elements[0].subject.identity);
    console.log("  Labels:", ast.elements[0].subject.labels);
}

// AST is just JSON - can be serialized, stored, transmitted
const json = JSON.stringify(ast, null, 2);
console.log("\nAs JSON:");
console.log(json);
```

**Python Example**:
```python
print("\n=== AST Output ===")

# Parse to AST (language-agnostic format)
ast = parse_to_ast("(alice:Person {name: 'Alice', age: 30})-[:KNOWS]->(bob)")

print("File-level pattern:")
print(f"  Identity: {ast['subject']['identity']}")
print(f"  Labels: {ast['subject']['labels']}")
print(f"  Properties: {ast['subject']['properties']}")
print(f"  Elements: {len(ast['elements'])}")

if ast['elements']:
    print("\nFirst element:")
    print(f"  Identity: {ast['elements'][0]['subject']['identity']}")
    print(f"  Labels: {ast['elements'][0]['subject']['labels']}")

# AST is just a dict - can be serialized, stored, transmitted
import json
json_str = json.dumps(ast, indent=2)
print("\nAs JSON:")
print(json_str)
```

**Acceptance Criteria**:
- [ ] Examples compile and run
- [ ] Output demonstrates AST structure clearly
- [ ] Show JSON serialization
- [ ] Include comments explaining purpose

**Estimated Time**: 30 minutes

---

### T090: Update Documentation

**Files to Update**:
1. `examples/gram-codec-wasm-web/README.md`
2. `examples/gram-codec-wasm-node/README.md`
3. `examples/gram-codec-python/README.md`
4. `crates/gram-codec/README.md` (if exists, otherwise create)

Add sections:

#### Section: AST Output (Recommended for Cross-Language Use)

```markdown
## AST Output (Recommended)

For building applications in JavaScript or Python, we recommend using the **AST output** instead of the metadata-only functions. The AST provides the complete pattern structure in a language-agnostic, JSON-friendly format.

### What is the AST?

The AST (Abstract Syntax Tree) is a direct representation of the `Pattern<Subject>` structure that gram notation describes. It contains:
- **subject**: The pattern's value (identity, labels, properties)
- **elements**: Child patterns (recursive structure)

### Usage

```javascript
import init, { parse_to_ast } from './gram_codec.js';
await init();

const ast = parse_to_ast("(alice:Person {name: 'Alice'})");
console.log(ast.subject.identity);  // "alice"
console.log(ast.subject.labels);    // ["Person"]
console.log(ast.subject.properties); // {name: "Alice"}
```

### Why Use AST?

1. **Complete Data Access** - Get all pattern information, not just metadata
2. **Language Agnostic** - Pure JSON, works in any language
3. **Ready for gram-js/gram-py** - These libraries will consume AST directly
4. **Serializable** - Can store, transmit, or cache AST as JSON

### What's NOT in the AST?

The AST represents patterns only - it doesn't include:
- ❌ Graph-specific concepts (nodes, edges, relationships)
- ❌ Pattern operations (map, fold, query)
- ❌ Execution or optimization logic

For full Pattern operations, use:
- **gram-js** - JavaScript/TypeScript native Pattern implementation (coming soon)
- **gram-py** - Python native Pattern implementation (coming soon)

These libraries will use gram-codec for parsing, then provide full FP APIs.
```

**Acceptance Criteria**:
- [ ] All READMEs updated with AST section
- [ ] Clear examples for each platform
- [ ] Explain relationship to gram-js/gram-py
- [ ] Link to AST design document

**Estimated Time**: 1 hour

---

### T091: Add Dependencies

**File**: `crates/gram-codec/Cargo.toml`

```toml
[dependencies]
# ... existing dependencies ...
serde_json = "1.0"

# WASM-specific
[target.'cfg(target_arch = "wasm32")'.dependencies]
serde-wasm-bindgen = "0.6"

# Python-specific  
[target.'cfg(feature = "python")'.dependencies]
pythonize = "0.21"
```

**Acceptance Criteria**:
- [ ] Dependencies added
- [ ] Cargo builds without errors
- [ ] WASM build works
- [ ] Python build works

**Estimated Time**: 15 minutes

---

## Summary

**Total Estimated Time**: 6-7 hours

**Tasks**:
- [x] T083: Define AST types (30 min)
- [ ] T084: Pattern to AST conversion (1 hour)
- [ ] T085: parse_to_ast function (30 min)
- [ ] T086: WASM bindings (45 min)
- [ ] T087: Python bindings (45 min)
- [ ] T088: Unit tests (1 hour)
- [ ] T089: Update examples (30 min)
- [ ] T090: Update documentation (1 hour)
- [ ] T091: Add dependencies (15 min)

**Testing Plan**:
1. Unit tests for all AST conversion
2. WASM example runs and shows correct output
3. Python example runs and shows correct output
4. JSON round-trip (serialize → deserialize)
5. Verify against corpus tests

**Success Criteria**:
- ✅ `parse_to_ast()` works in Rust, WASM, Python
- ✅ Returns proper JSON-like structures (not opaque types)
- ✅ All existing tests still pass
- ✅ New AST tests pass
- ✅ Examples updated and working
- ✅ Documentation complete

---

**Next Phase**: Phase 8 - gram-js Foundation (separate project)
