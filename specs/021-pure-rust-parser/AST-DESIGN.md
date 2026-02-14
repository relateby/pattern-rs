# AST Design for pattern-rs Parser Output

**Date**: January 9, 2026  
**Status**: Design Document  
**Purpose**: Define the minimal AST output from pattern-rs for consumption by gram-js and gram-py

---

## Overview

The pattern-rs parser should output a **minimal Abstract Syntax Tree (AST)** that represents the parsed Pattern<Subject> structure in a language-agnostic, JSON-serializable format. This AST serves as the interchange format between:

- **pattern-rs** (Rust parser)
- **gram-js** (JavaScript/TypeScript native Pattern implementation)
- **gram-py** (Python native Pattern implementation)

---

## Core Principle

**The AST mirrors Pattern<Subject> structure exactly - nothing more, nothing less.**

Key insights:
1. ✅ Gram notation always produces a **single Pattern<Subject>** (the file-level pattern)
2. ✅ The AST represents this structure **without graph-specific concepts**
3. ✅ Path notation (e.g., `(a)-->(b)`) is **syntactic sugar** - already desugared by parser
4. ✅ No special types for "nodes", "edges", "relationships" - just Pattern and Subject

---

## AST Structure

### Core Types

```rust
/// Abstract Syntax Tree for gram notation
/// 
/// Gram always produces a single Pattern<Subject> (the file-level pattern).
/// This AST mirrors that structure exactly.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AstPattern {
    /// The subject (value) of this pattern
    pub subject: AstSubject,
    
    /// Child patterns (elements)
    pub elements: Vec<AstPattern>,
}

/// Subject data - identity, labels, and properties
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AstSubject {
    /// Identity (empty string if not specified)
    pub identity: String,
    
    /// Labels
    pub labels: Vec<String>,
    
    /// Properties (arbitrary JSON values)
    pub properties: HashMap<String, serde_json::Value>,
}
```

### API

```rust
/// Parse gram notation to AST
/// 
/// Always returns a single AstPattern representing the file-level pattern.
pub fn parse_to_ast(input: &str) -> Result<AstPattern, ParseError>
```

---

## Examples

### Example 1: Simple Node

**Input**:
```gram
(alice:Person {name: "Alice", age: 30})
```

**AST Output**:
```json
{
  "subject": {
    "identity": "alice",
    "labels": ["Person"],
    "properties": {
      "name": "Alice",
      "age": 30
    }
  },
  "elements": []
}
```

### Example 2: Path Notation (Desugared)

**Input**:
```gram
(alice)-->(bob)
```

**AST Output** (file-level pattern with elements):
```json
{
  "subject": {
    "identity": "",
    "labels": [],
    "properties": {}
  },
  "elements": [
    {
      "subject": {
        "identity": "alice",
        "labels": [],
        "properties": {}
      },
      "elements": []
    },
    {
      "subject": {
        "identity": "bob",
        "labels": [],
        "properties": {}
      },
      "elements": []
    }
  ]
}
```

**Note**: The parser has already desugared the path notation. The AST doesn't know or care that this was written as `(alice)-->(bob)` - it just represents the resulting pattern structure.

### Example 3: Subject Pattern

**Input**:
```gram
[team:Team {name: "DevRel"} | (alice), (bob), (charlie)]
```

**AST Output**:
```json
{
  "subject": {
    "identity": "team",
    "labels": ["Team"],
    "properties": {
      "name": "DevRel"
    }
  },
  "elements": [
    {
      "subject": { "identity": "alice", "labels": [], "properties": {} },
      "elements": []
    },
    {
      "subject": { "identity": "bob", "labels": [], "properties": {} },
      "elements": []
    },
    {
      "subject": { "identity": "charlie", "labels": [], "properties": {} },
      "elements": []
    }
  ]
}
```

### Example 4: File-Level Pattern with Properties

**Input**:
```gram
{version: "1.0", schema: "social"}
(alice:Person)
(bob:Person)
```

**AST Output**:
```json
{
  "subject": {
    "identity": "",
    "labels": [],
    "properties": {
      "version": "1.0",
      "schema": "social"
    }
  },
  "elements": [
    {
      "subject": { "identity": "alice", "labels": ["Person"], "properties": {} },
      "elements": []
    },
    {
      "subject": { "identity": "bob", "labels": ["Person"], "properties": {} },
      "elements": []
    }
  ]
}
```

### Example 5: Empty File

**Input**:
```gram

```

**AST Output**:
```json
{
  "subject": {
    "identity": "",
    "labels": [],
    "properties": {}
  },
  "elements": []
}
```

---

## What the AST Is NOT

❌ **Not a Concrete Syntax Tree (CST)** - Doesn't preserve whitespace, comments, exact token positions

❌ **Not graph-specific** - No "node", "edge", "relationship" types

❌ **Not a query language** - No pattern matching, filtering, or query operations

❌ **Not an execution plan** - No optimization, no execution semantics

✅ **Just a faithful representation of the Pattern<Subject> tree**

---

## Language-Specific Consumption

### gram-js (JavaScript/TypeScript)

```typescript
import { parse_to_ast } from '@gram-data/codec';
import { Pattern, Subject, Symbol } from '@gram-data/pattern';

// Parse gram text to AST
const ast = parse_to_ast("(alice:Person {name: 'Alice'})");

// Convert to native Pattern<Subject>
const pattern = Pattern.fromAst(ast);

// Use native JS Pattern API (no FFI overhead)
const identities = pattern.fold([], (acc, subject) => {
  return [...acc, subject.identity.value];
});
```

### gram-py (Python)

```python
from gram_codec import parse_to_ast
from gram import Pattern, Subject, Symbol

# Parse gram text to AST
ast = parse_to_ast("(alice:Person {name: 'Alice'})")

# Convert to native Pattern<Subject>
pattern = Pattern.from_ast(ast)

# Use native Python Pattern API (no FFI overhead)
identities = pattern.fold([], lambda acc, subject: acc + [subject.identity.value])
```

---

## Project Boundaries

### pattern-rs Responsibility

✅ **Parse gram notation** - Text → AST
✅ **Validate syntax** - Ensure well-formed gram
✅ **Serialize to gram** - Pattern → Text (for round-trips)
✅ **Provide WASM/Python bindings** - Expose `parse_to_ast()`

❌ **NOT responsible for**:
- Pattern operations (map, fold, filter, etc.)
- Graph operations (queries, traversals, etc.)
- Persistence or storage
- Type system or validation beyond syntax

### gram-js / gram-py Responsibility

✅ **Implement Pattern<V> and Subject** - Native language implementations
✅ **Full FP API** - map, fold, traverse, comonad operations, etc.
✅ **Pattern operations** - Combine, validate, query, transform
✅ **Idiomatic APIs** - TypeScript/Python best practices

❌ **NOT responsible for**:
- Parsing gram notation (use pattern-rs via WASM/bindings)

### pattern-frame Responsibility

✅ **DataFrame-like operations** - Bulk operations on pattern collections
✅ **Columnar storage** - Efficient representation
✅ **Query DSL** - High-level query language
✅ **Analytics** - Aggregations, grouping, windowing

### pattern-store / pattern-db Responsibility

✅ **Persistence** - ACID transactions
✅ **Indexes** - Query optimization
✅ **Multi-process access** - Concurrency control
✅ **SQL interop** - Integration with existing databases

---

## Benefits of This Design

### 1. Clear Separation of Concerns

```
pattern-rs:     Text → AST       (parsing only)
gram-js:     AST → Pattern    (FP operations in JS)
gram-py:     AST → Pattern    (FP operations in Python)
frame:       Pattern → Frame  (bulk operations)
store:       Frame → DB       (persistence)
```

### 2. No FFI Tax for Operations

All Pattern operations run in native language - no WASM/FFI overhead:

```javascript
// Pure JavaScript - no WASM calls after initial parse
pattern.map(s => s.identity.toUpperCase())
       .fold(0, (count, s) => count + 1)
       .filter(p => p.value.labels.has("Person"))
```

### 3. Language-Idiomatic APIs

Each language implements Pattern following its own best practices:

- **gram-js**: Immutable, functional, TypeScript-friendly
- **gram-py**: Pythonic, with dataclasses, type hints
- **gram-jvm**: Java/Kotlin/Scala idioms (future)

### 4. Small Parser Binary

pattern-rs stays focused:
- ✅ Parser + AST serialization: ~50-100KB WASM
- ✅ No FP runtime or graph engine
- ✅ Fast compilation

### 5. Independent Evolution

- pattern-rs can optimize parsing without affecting consumers
- gram-js/gram-py can add features without changing parser
- AST format can version independently

### 6. Easy to Port to New Languages

To add gram support to a new language:
1. Use pattern-rs parser (via WASM or native bindings)
2. Implement Pattern<V> in target language
3. Write `Pattern.fromAst(ast)` converter
4. Done!

---

## Migration Path

### Current State (Phase 6)

pattern-rs exposes:
```rust
pub fn parse_gram(input: &str) -> Result<Vec<Pattern<Subject>>, ParseError>
pub fn validate_gram(input: &str) -> Result<(), ParseError>
pub fn round_trip(input: &str) -> Result<String, ParseError>
```

WASM/Python expose minimal metadata:
```javascript
const result = parse_gram("(hello)");
// { pattern_count: 1, identifiers: [] }
```

### Phase 7: Add AST Output

Add new function:
```rust
pub fn parse_to_ast(input: &str) -> Result<AstPattern, ParseError>
```

WASM/Python expose AST:
```javascript
const ast = parse_to_ast("(hello)");
// { subject: {...}, elements: [] }
```

### Phase 8: gram-js / gram-py Projects

Create separate projects:
```
gram-js/
  src/
    pattern.ts      // Pattern<V> implementation
    subject.ts      // Subject implementation
    index.ts        // Public API
  examples/
  tests/

gram-py/
  gram/
    pattern.py      // Pattern implementation
    subject.py      // Subject implementation
  examples/
  tests/
```

Both depend on pattern-rs only for parsing:
```json
// package.json (gram-js)
{
  "dependencies": {
    "@gram-data/codec": "^0.1.0"  // WASM parser
  }
}
```

```toml
# pyproject.toml (gram-py)
[dependencies]
gram-codec = "^0.1.0"  # Python wheel with parser
```

---

## Implementation Tasks

See `tasks.md` for detailed task breakdown:
- Phase 7: AST Output Implementation
- Phase 8: gram-js Foundation
- Phase 9: gram-py Foundation

---

## Design Decisions

### Value Serialization Strategy

**Decision**: Mixed approach - native JSON for simple types, tagged objects for complex types

#### Rationale

- **Native JSON** for common types keeps AST clean and easy to work with
- **Tagged objects** for complex types preserve semantic information
- This balances simplicity with correctness

#### Implementation

```rust
fn value_to_json(value: &Value) -> serde_json::Value {
    use pattern_core::Value;
    match value {
        // Native JSON types (simple, common)
        Value::VNull => serde_json::Value::Null,
        Value::VBoolean(b) => serde_json::Value::Bool(*b),
        Value::VString(s) => serde_json::Value::String(s.clone()),
        Value::VArray(arr) => serde_json::Value::Array(
            arr.iter().map(value_to_json).collect()
        ),
        Value::VMap(map) => serde_json::Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect()
        ),
        Value::VRecord(rec) => serde_json::Value::Object(
            rec.0.iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect()
        ),
        
        // Tagged types (need disambiguation or structure)
        Value::VSymbol(sym) => serde_json::json!({
            "type": "Symbol",
            "value": sym.0.clone()
        }),
        Value::VInteger(i) => serde_json::json!({
            "type": "Integer",
            "value": i
        }),
        Value::VDecimal(d) => serde_json::json!({
            "type": "Decimal",
            "value": d
        }),
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
    }
}
```

#### Examples

**Simple values** (native JSON):
```json
{
  "name": "Alice",           // VString
  "active": true,            // VBoolean
  "tags": ["dev", "admin"],  // VArray
  "metadata": {              // VMap
    "created": "2024-01-09"
  }
}
```

**Complex values** (tagged):
```json
{
  "id": {                    // VSymbol
    "type": "Symbol",
    "value": "user123"
  },
  "age": {                   // VInteger
    "type": "Integer",
    "value": 30
  },
  "balance": {               // VDecimal
    "type": "Decimal",
    "value": 123.45
  },
  "range": {                 // VRange
    "type": "Range",
    "lower": 1,
    "upper": 10
  },
  "height": {                // VMeasurement
    "type": "Measurement",
    "value": 168,
    "unit": "cm"
  },
  "birthDate": {             // VTagged
    "type": "Tagged",
    "tag": "date",
    "value": "1990-05-15"
  }
}
```

#### Benefits

1. **Simple cases are simple**: Most properties are strings, booleans, or numbers
2. **No ambiguity**: Symbol vs String, Integer vs Decimal are distinguishable
3. **Easy to consume**: JavaScript/Python can use native types directly for common cases
4. **Preserves semantics**: Complex types retain their meaning

#### Trade-offs

- **Inconsistent**: Some values are primitives, some are objects
- **Type checking needed**: Consumers must check for `{ type: ... }` pattern
- **Slightly verbose**: Tagged types take more space than raw values

**Why this is acceptable**: The common case (strings, booleans, arrays, maps) is optimized for simplicity, and the complexity is only added where semantic distinction matters.

### Error Representation

**Decision**: Keep simple for now, enhance later

**Current approach**: Simple error message string
```rust
Err(ParseError) => JsValue::from_str(&format!("Parse error: {}", e))
```

**Future**: Structured error with location, context, suggestions (Phase 8+)

**Rationale**: AST output (Phase 7) is additive - doesn't change error handling. Can improve errors in a later phase without breaking changes.

### Streaming Support

**Decision**: Deferred to future phase

**Current**: Parse entire file to AST (sufficient for most use cases)
**Future**: Event-based parsing for large files (SAX-like) if needed

**Rationale**: 
- Most gram files are small (< 1MB)
- Adding streaming would complicate the API
- Can add later as `parse_to_ast_stream()` without breaking changes

### Schema/Validation

**Decision**: Not in pattern-rs scope

**Current**: No structural validation beyond syntax
**Future**: JSON Schema validation could be separate tool

**Rationale**: 
- Parsing and validation are separate concerns
- Structural validation depends on use case (graph schemas vary)
- Better handled by gram-js/gram-py or separate validation library

---

## References

- [gram-hs Pattern implementation](../../../external/tree-sitter-gram/)
- [pattern-core Rust implementation](../../../crates/pattern-core/)
- [Apache Arrow](https://arrow.apache.org/) - Inspiration for columnar interchange format
- [JSON Schema](https://json-schema.org/) - Potential validation approach
- [DECISIONS.md](./DECISIONS.md) - Complete design decision rationale

---

**Status**: ✅ **DESIGN APPROVED**  
**Decisions**: ✅ **ALL APPROVED** (see DECISIONS.md)  
**Next Step**: Implement `parse_to_ast()` in pattern-rs (Phase 7)
