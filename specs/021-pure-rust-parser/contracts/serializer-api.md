# API Contract: Serializer

**Feature**: 021-pure-rust-parser  
**Date**: 2026-01-09  
**Purpose**: Define the public API contract for Pattern serialization to gram notation

## Public API Functions

### 1. serialize_patterns

Serialize a collection of Pattern structures into gram notation text.

**Signature**:
```rust
pub fn serialize_patterns(patterns: &[Pattern]) -> Result<String, SerializeError>
```

**Parameters**:
- `patterns: &[Pattern]` - Patterns to serialize

**Returns**:
- `Ok(String)` - Gram notation text
- `Err(SerializeError)` - Serialization error (pattern cannot be represented)

**Behavior**:

1. **Pattern Analysis**:
   - Inspect pattern structure (element count, value type)
   - Select appropriate gram notation form:
     - 0 elements → node notation `(subject)`
     - 2 elements (both atomic) → relationship notation `(a)-->(b)`
     - Other cases → subject pattern notation `[subject | elements]`

2. **Subject Serialization**:
   - Identifier (if present)
   - Labels (`:Label1:Label2`)
   - Properties record (`{key: value}`)

3. **Formatting**:
   - One pattern per line (newline separator)
   - Proper escaping of special characters in identifiers and strings
   - Readable spacing around operators and delimiters

4. **Validation**:
   - All patterns must be representable in gram notation
   - All property values must map to gram value types

**Examples**:

```rust
// Node pattern
let pattern = Pattern::from_subject(Subject {
    identifier: Some("hello".to_string()),
    labels: None,
    record: None,
});
assert_eq!(
    serialize_patterns(&[pattern]).unwrap(),
    "(hello)"
);

// Relationship pattern
let left = Pattern::from_subject(Subject {
    identifier: Some("a".to_string()),
    ..Default::default()
});
let right = Pattern::from_subject(Subject {
    identifier: Some("b".to_string()),
    ..Default::default()
});
let rel = Pattern {
    value: Subject::default().into(),
    elements: vec![left, right],
};
assert_eq!(
    serialize_patterns(&[rel]).unwrap(),
    "(a)-->(b)"
);

// Subject pattern
let pattern = Pattern {
    value: Subject {
        identifier: Some("team".to_string()),
        ..Default::default()
    }.into(),
    elements: vec![
        Pattern::from_subject(Subject {
            identifier: Some("alice".to_string()),
            ..Default::default()
        }),
        Pattern::from_subject(Subject {
            identifier: Some("bob".to_string()),
            ..Default::default()
        }),
    ],
};
assert_eq!(
    serialize_patterns(&[pattern]).unwrap(),
    "[team | (alice), (bob)]"
);
```

**Performance Contract**:
- Simple patterns: <5μs per pattern
- Complex patterns: <50μs per pattern
- Round-trip overhead: <10% (serialize + parse vs parse alone)

**Thread Safety**:
- Function is pure (no internal state)
- Safe to call from multiple threads concurrently

---

### 2. serialize_pattern

Serialize a single Pattern to gram notation.

**Signature**:
```rust
pub fn serialize_pattern(pattern: &Pattern) -> Result<String, SerializeError>
```

**Parameters**:
- `pattern: &Pattern` - Pattern to serialize

**Returns**:
- `Ok(String)` - Gram notation text
- `Err(SerializeError)` - Serialization error

**Behavior**:
- Convenience function for single pattern serialization
- Equivalent to `serialize_patterns(&[pattern])`

**Examples**:

```rust
let pattern = Pattern::from_subject(Subject {
    identifier: Some("hello".to_string()),
    ..Default::default()
});
assert_eq!(
    serialize_pattern(&pattern).unwrap(),
    "(hello)"
);
```

---

## Serialization Rules

### 1. Pattern Form Selection

**Rule**: Pattern serialization form is determined by element count and structure.

| Elements | Structure | Form | Example |
|----------|-----------|------|---------|
| 0 | Any subject | Node | `(id:Label {props})` |
| 2 | Both atomic (0 elements each) | Relationship | `(a)-->(b)` |
| 2 | One or both non-atomic | Subject | `[root \| (a), [nested \| (b)]]` |
| N (N ≠ 0, 2) | Any | Subject | `[root \| e1, e2, e3]` |

**Implementation**:

```rust
fn serialize_pattern_dispatch(pattern: &Pattern) -> Result<String, SerializeError> {
    match pattern.elements.len() {
        0 => serialize_node(pattern),
        2 if pattern.elements.iter().all(|e| e.is_node()) => {
            serialize_relationship(pattern)
        }
        _ => serialize_subject_pattern(pattern),
    }
}
```

### 2. Subject Serialization

**Order**: `identifier:Label1:Label2 {key: value}`

**Rules**:
- Identifier is optional (may be omitted if None)
- Labels are optional (may be omitted if empty)
- Record is optional (may be omitted if empty)
- At least one component must be present

**Examples**:
```rust
// Identifier only
Subject { identifier: Some("hello"), .. } → "hello"

// Identifier + label
Subject { 
    identifier: Some("a"), 
    labels: Some(vec!["Person".to_string()]),
    ..
} → "a:Person"

// Identifier + labels + properties
Subject {
    identifier: Some("a"),
    labels: Some(vec!["Person".to_string()]),
    record: Some(HashMap::from([
        ("name".to_string(), Value::String("Alice".to_string()))
    ])),
} → "a:Person {name: \"Alice\"}"

// Empty subject (for relationship with no label/props)
Subject::default() → "" (omitted in relationship)
```

### 3. Value Serialization

**Mapping**: Rust `Value` enum → gram notation syntax

| Value Type | Gram Notation | Example |
|------------|---------------|---------|
| String | Quoted string | `"hello"` |
| Integer | Decimal digits | `42` |
| Decimal | Floating-point | `3.14` |
| Boolean | Lowercase | `true`, `false` |
| Symbol | Unquoted identifier | `hello` |
| Array | Bracketed list | `["a", "b", "c"]` |
| Range | Dotted notation | `1..10` |
| TaggedString | Triple-quoted | `"""markdown content"""` |

**Escaping Rules**:

```rust
// String escaping
"hello\"world" → "\"hello\\\"world\""
"line1\nline2" → "\"line1\\nline2\""
"tab\there" → "\"tab\\there\""

// Identifier escaping (if contains special chars)
"my-identifier" → "my-identifier" (no quotes, - is allowed)
"my identifier" → "\"my identifier\"" (quotes required for space)
"hello.world" → "\"hello.world\"" (quotes required for .)
```

### 4. Relationship Arrow Selection

**Rule**: Relationships always use right arrow `-->` in serialization.

**Rationale**: 
- Element ordering captures directionality semantics
- Parser handles all arrow types (`<--`, `<-->`, `~~`, `~>`)
- Serializer normalizes to canonical form for consistency

**Examples**:

```rust
// Left-to-right relationship
(left_node, right_node) → "(left)-->(right)"

// Parser may have reversed elements for <-- arrow
// Serializer always produces --> with correct element order
```

---

## Error Handling

### SerializeError Variants

```rust
#[derive(Debug, Error)]
pub enum SerializeError {
    /// Pattern cannot be represented in gram notation
    #[error("Cannot serialize pattern: {reason}")]
    Unsupported {
        reason: String,
    },
    
    /// Invalid pattern structure
    #[error("Invalid pattern structure: {reason}")]
    InvalidStructure {
        reason: String,
    },
}
```

**Error Scenarios**:

1. **Unsupported Value Type**:
   ```rust
   // If Value enum has variants not in gram notation
   Err(SerializeError::Unsupported {
       reason: "Custom value types not supported in gram notation".to_string()
   })
   ```

2. **Invalid Pattern Structure**:
   ```rust
   // Pattern with 1 element (ambiguous serialization)
   Err(SerializeError::InvalidStructure {
       reason: "Pattern with 1 element has ambiguous serialization".to_string()
   })
   ```

**Recovery**: Serialization errors are typically fatal (pattern cannot be represented). Caller should handle by:
- Validating patterns before serialization
- Transforming unsupported patterns
- Reporting error to user

---

## Round-Trip Contract

### Requirement: Semantic Equivalence

**Strategy** (from `../gram-hs/docs/reference/features/gram-serialization.md`): Test "structural equality after serialization/deserialization cycles" using `gram -> pattern -> gram -> pattern`.

For all patterns produced by `parse_gram`, serialization and re-parsing must produce structurally equivalent patterns:

```rust
let original = "(a:Label {key: \"value\"})";

// First parse
let patterns1 = parse_gram(original).unwrap();

// Serialize
let serialized = serialize_patterns(&patterns1).unwrap();

// Second parse (from serialized output)
let patterns2 = parse_gram(&serialized).unwrap();

// Semantic equivalence: Pattern structures must be identical
assert_eq!(patterns1, patterns2);  // Must be equal
```

**Why This Approach?**
- **Semantic Focus**: Tests Pattern structure preservation, not gram notation string equivalence
- **Robust**: Independent of formatting choices (whitespace, comments, arrow normalization)
- **Canonical**: Verifies that the parse/serialize cycle produces stable, equivalent Pattern structures

### Structural Equivalence

**Definition**: Two patterns are structurally equivalent if:
1. Their `value` (Subject) fields are equal (identifier, labels, properties)
2. Their `elements` vectors have same length
3. Elements are pairwise structurally equivalent (recursive)

**Preserved in Round-Trip**:
- Pattern tree structure (nesting, element count)
- Subject values (identifiers, labels, property keys and values)
- Element ordering
- Value types (String, Integer, Boolean, Array, etc.)

**Not Required to Match** (acceptable differences):
- Gram notation formatting (whitespace may vary)
- Comment placement (comments are not preserved during parsing)
- Arrow types in original input (all relationships serialize as `-->`)

### Validation

```rust
#[test]
fn test_round_trip_semantic_equivalence() {
    let test_cases = [
        "(hello)",
        "(a:Person {name: \"Alice\"})",
        "(a)-->(b)",
        "(a)<--(b)",  // Left arrow - element order reversed during parse
        "[team | (alice), (bob)]",
        "[nested | [inner | (leaf)]]",
        "// comment\n(a)\n(b)",  // Comments not preserved
    ];
    
    for input in test_cases {
        // First parse
        let patterns1 = parse_gram(input).unwrap();
        
        // Serialize
        let serialized = serialize_patterns(&patterns1).unwrap();
        
        // Second parse (from serialized output)
        let patterns2 = parse_gram(&serialized).unwrap();
        
        // Semantic equivalence: Pattern structures must be identical
        assert_eq!(
            patterns1, patterns2,
            "Round-trip semantic equivalence failed\n\
             Original input: {}\n\
             Serialized as: {}\n\
             Pattern structures don't match",
            input, serialized
        );
    }
}

#[test]
fn test_round_trip_idempotent() {
    // After first round-trip, subsequent round-trips should be identical
    let input = "(a:Label {key: \"value\"})";
    
    let p1 = parse_gram(input).unwrap();
    let g1 = serialize_patterns(&p1).unwrap();
    let p2 = parse_gram(&g1).unwrap();
    let g2 = serialize_patterns(&p2).unwrap();
    let p3 = parse_gram(&g2).unwrap();
    
    // After first round-trip, structure and serialization stabilize
    assert_eq!(p2, p3);
    assert_eq!(g1, g2);  // Serialized form is stable
}
```

---

## Testing Requirements

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_empty_collection() {
        assert_eq!(serialize_patterns(&[]).unwrap(), "");
    }

    #[test]
    fn serialize_simple_node() {
        let pattern = Pattern::from_subject(Subject {
            identifier: Some("hello".to_string()),
            ..Default::default()
        });
        assert_eq!(serialize_patterns(&[pattern]).unwrap(), "(hello)");
    }

    #[test]
    fn serialize_with_labels() {
        let pattern = Pattern::from_subject(Subject {
            identifier: Some("a".to_string()),
            labels: Some(vec!["Person".to_string(), "User".to_string()]),
            ..Default::default()
        });
        assert_eq!(
            serialize_patterns(&[pattern]).unwrap(),
            "(a:Person:User)"
        );
    }

    #[test]
    fn serialize_with_properties() {
        let mut record = HashMap::new();
        record.insert("name".to_string(), Value::String("Alice".to_string()));
        record.insert("age".to_string(), Value::Integer(30));
        
        let pattern = Pattern::from_subject(Subject {
            identifier: Some("a".to_string()),
            record: Some(record),
            ..Default::default()
        });
        
        let result = serialize_patterns(&[pattern]).unwrap();
        assert!(result.contains("name: \"Alice\""));
        assert!(result.contains("age: 30"));
    }

    #[test]
    fn serialize_relationship() {
        let left = Pattern::from_subject(Subject {
            identifier: Some("a".to_string()),
            ..Default::default()
        });
        let right = Pattern::from_subject(Subject {
            identifier: Some("b".to_string()),
            ..Default::default()
        });
        let rel = Pattern {
            value: Subject::default().into(),
            elements: vec![left, right],
        };
        
        assert_eq!(serialize_patterns(&[rel]).unwrap(), "(a)-->(b)");
    }

    #[test]
    fn serialize_escapes_strings() {
        let mut record = HashMap::new();
        record.insert(
            "quote".to_string(),
            Value::String("say \"hello\"".to_string())
        );
        
        let pattern = Pattern::from_subject(Subject {
            identifier: Some("a".to_string()),
            record: Some(record),
            ..Default::default()
        });
        
        let result = serialize_patterns(&[pattern]).unwrap();
        assert!(result.contains("\\\""));
    }
}
```

---

## Contract Summary

| Aspect | Requirement | Verification |
|--------|-------------|--------------|
| Form Selection | Based on element count | Unit tests |
| Subject Order | identifier:labels {record} | Unit tests |
| Value Mapping | All gram value types | Unit tests |
| Escaping | Special chars escaped | Unit tests |
| Round-Trip | Structural equivalence | Round-trip tests |
| Performance | <10% round-trip overhead | Benchmarks |
| Thread Safety | No shared state | Code review |

**Key Invariant**: `parse_gram(serialize_patterns(patterns)) == patterns` for all valid patterns
