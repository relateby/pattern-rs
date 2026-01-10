//! Abstract Syntax Tree (AST) types for gram notation
//!
//! The AST provides a language-agnostic, JSON-serializable representation
//! of the Pattern<Subject> structure that gram notation describes.
//!
//! # Design Philosophy
//!
//! The AST mirrors the Pattern<Subject> structure exactly:
//! - No graph-specific concepts (no "nodes", "edges", "relationships")
//! - Path notation is already desugared by the parser
//! - Just patterns and subjects - clean and conceptual
//!
//! # Usage
//!
//! ```rust
//! use gram_codec::{parse_to_ast, AstPattern};
//!
//! let ast = parse_to_ast("(alice:Person {name: \"Alice\"})")?;
//! println!("Identity: {}", ast.subject.identity);
//! println!("Labels: {:?}", ast.subject.labels);
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # JSON Serialization
//!
//! The AST is designed to be JSON-serializable for cross-language use:
//!
//! ```rust
//! use gram_codec::parse_to_ast;
//! use serde_json;
//!
//! let ast = parse_to_ast("(alice:Person)")?;
//! let json = serde_json::to_string(&ast)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Abstract Syntax Tree representation of a Pattern
///
/// This is the minimal, language-agnostic output format from gram-rs.
/// It mirrors the Pattern<Subject> structure exactly.
///
/// # Structure
///
/// - `subject`: The value of this pattern (identity, labels, properties)
/// - `elements`: Child patterns (recursive structure)
///
/// # Examples
///
/// Simple node:
/// ```json
/// {
///   "subject": {
///     "identity": "alice",
///     "labels": ["Person"],
///     "properties": {"name": "Alice"}
///   },
///   "elements": []
/// }
/// ```
///
/// Subject pattern with elements:
/// ```json
/// {
///   "subject": {
///     "identity": "team",
///     "labels": ["Team"],
///     "properties": {}
///   },
///   "elements": [
///     {"subject": {"identity": "alice", ...}, "elements": []},
///     {"subject": {"identity": "bob", ...}, "elements": []}
///   ]
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AstPattern {
    /// The subject (value) of this pattern
    pub subject: AstSubject,

    /// Child patterns (elements)
    pub elements: Vec<AstPattern>,
}

/// Subject data - identity, labels, and properties
///
/// The subject provides "information about the elements" in a pattern.
///
/// # Examples
///
/// Node with identity and label:
/// ```json
/// {
///   "identity": "alice",
///   "labels": ["Person"],
///   "properties": {}
/// }
/// ```
///
/// Anonymous node with properties:
/// ```json
/// {
///   "identity": "",
///   "labels": [],
///   "properties": {"name": "Alice", "age": 30}
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AstSubject {
    /// Identity (empty string if not specified)
    pub identity: String,

    /// Labels (type tags)
    pub labels: Vec<String>,

    /// Properties (arbitrary JSON values)
    ///
    /// Values use mixed serialization:
    /// - Simple types: native JSON (string, number, boolean, array, object)
    /// - Complex types: tagged objects (Symbol, Integer, Decimal, Range, etc.)
    pub properties: HashMap<String, serde_json::Value>,
}

impl AstPattern {
    /// Create an empty pattern
    ///
    /// Useful for representing empty gram files or as a placeholder.
    pub fn empty() -> Self {
        AstPattern {
            subject: AstSubject {
                identity: String::new(),
                labels: Vec::new(),
                properties: HashMap::new(),
            },
            elements: Vec::new(),
        }
    }
}

// Conversion from Pattern<Subject> to AST
use pattern_core::{Pattern, Subject, Value};

impl AstPattern {
    /// Convert from native Pattern<Subject> to AST
    ///
    /// This is the core conversion function that transforms the Rust
    /// Pattern structure into a JSON-serializable AST.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use gram_codec::AstPattern;
    /// use pattern_core::{Pattern, Subject, Symbol};
    /// use std::collections::{HashSet, HashMap};
    ///
    /// let subject = Subject {
    ///     identity: Symbol("alice".to_string()),
    ///     labels: HashSet::new(),
    ///     properties: HashMap::new(),
    /// };
    /// let pattern = Pattern::point(subject);
    /// let ast = AstPattern::from_pattern(&pattern);
    /// assert_eq!(ast.subject.identity, "alice");
    /// ```
    pub fn from_pattern(pattern: &Pattern<Subject>) -> Self {
        let subject = pattern.value();

        AstPattern {
            subject: AstSubject {
                identity: subject.identity.0.clone(),
                labels: subject.labels.iter().cloned().collect(),
                properties: subject
                    .properties
                    .iter()
                    .map(|(k, v)| (k.clone(), value_to_json(v)))
                    .collect(),
            },
            elements: pattern
                .elements()
                .iter()
                .map(AstPattern::from_pattern)
                .collect(),
        }
    }
}

/// Convert a Value to JSON using canonical format (aligned with gram-hs)
///
/// # Strategy
///
/// **Native JSON** (simple, common types):
/// - VInteger, VDecimal, VBoolean, VString, VArray, VMap
///
/// **Tagged objects** (complex, need disambiguation):
/// - VSymbol, VRange, VMeasurement, VTaggedString
///
/// This matches the gram-hs canonical format:
/// - Numbers (integer/decimal) use native JSON
/// - Complex types use lowercase type discriminators
fn value_to_json(value: &Value) -> serde_json::Value {
    match value {
        // Native JSON types (simple, common)
        Value::VInteger(i) => serde_json::Value::Number((*i).into()),

        Value::VDecimal(d) => {
            // Convert f64 to serde_json::Number
            serde_json::Number::from_f64(*d)
                .map(serde_json::Value::Number)
                .unwrap_or_else(|| serde_json::Value::Null)
        }

        Value::VBoolean(b) => serde_json::Value::Bool(*b),

        Value::VString(s) => serde_json::Value::String(s.clone()),

        Value::VArray(arr) => serde_json::Value::Array(arr.iter().map(value_to_json).collect()),

        Value::VMap(map) => serde_json::Value::Object(
            map.iter()
                .map(|(k, v)| (k.clone(), value_to_json(v)))
                .collect(),
        ),

        // Tagged types (need disambiguation or structure)
        // Use lowercase type discriminators to match gram-hs canonical format
        Value::VSymbol(sym) => serde_json::json!({
            "type": "symbol",
            "value": sym.clone()
        }),

        Value::VRange(range) => serde_json::json!({
            "type": "range",
            "lower": range.lower,
            "upper": range.upper
        }),

        Value::VMeasurement { unit, value } => serde_json::json!({
            "type": "measurement",
            "unit": unit,
            "value": value
        }),

        Value::VTaggedString { tag, content } => serde_json::json!({
            "type": "tagged",
            "tag": tag,
            "content": content
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pattern_core::{Pattern, Subject, Symbol};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_empty_pattern() {
        let pattern = AstPattern::empty();
        assert_eq!(pattern.subject.identity, "");
        assert_eq!(pattern.subject.labels.len(), 0);
        assert_eq!(pattern.subject.properties.len(), 0);
        assert_eq!(pattern.elements.len(), 0);
    }

    #[test]
    fn test_json_serialization() {
        let pattern = AstPattern {
            subject: AstSubject {
                identity: "alice".to_string(),
                labels: vec!["Person".to_string()],
                properties: {
                    let mut props = HashMap::new();
                    props.insert("name".to_string(), serde_json::json!("Alice"));
                    props
                },
            },
            elements: vec![],
        };

        // Serialize to JSON
        let json = serde_json::to_string(&pattern).unwrap();
        assert!(json.contains("alice"));
        assert!(json.contains("Person"));

        // Deserialize back
        let deserialized: AstPattern = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, pattern);
    }

    #[test]
    fn test_nested_patterns() {
        let child1 = AstPattern {
            subject: AstSubject {
                identity: "child1".to_string(),
                labels: vec![],
                properties: HashMap::new(),
            },
            elements: vec![],
        };

        let child2 = AstPattern {
            subject: AstSubject {
                identity: "child2".to_string(),
                labels: vec![],
                properties: HashMap::new(),
            },
            elements: vec![],
        };

        let parent = AstPattern {
            subject: AstSubject {
                identity: "parent".to_string(),
                labels: vec![],
                properties: HashMap::new(),
            },
            elements: vec![child1, child2],
        };

        assert_eq!(parent.elements.len(), 2);
        assert_eq!(parent.elements[0].subject.identity, "child1");
        assert_eq!(parent.elements[1].subject.identity, "child2");

        // Serialize and check it works
        let json = serde_json::to_string(&parent).unwrap();
        let deserialized: AstPattern = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.elements.len(), 2);
    }

    #[test]
    fn test_from_pattern_simple() {
        let subject = Subject {
            identity: Symbol("alice".to_string()),
            labels: {
                let mut labels = HashSet::new();
                labels.insert("Person".to_string());
                labels
            },
            properties: HashMap::new(),
        };
        let pattern = Pattern::point(subject);

        let ast = AstPattern::from_pattern(&pattern);

        assert_eq!(ast.subject.identity, "alice");
        assert_eq!(ast.subject.labels, vec!["Person"]);
        assert_eq!(ast.elements.len(), 0);
    }

    #[test]
    fn test_from_pattern_with_properties() {
        let subject = Subject {
            identity: Symbol("alice".to_string()),
            labels: HashSet::new(),
            properties: {
                let mut props = HashMap::new();
                props.insert("name".to_string(), Value::VString("Alice".to_string()));
                props.insert("age".to_string(), Value::VInteger(30));
                props
            },
        };
        let pattern = Pattern::point(subject);

        let ast = AstPattern::from_pattern(&pattern);

        assert_eq!(ast.subject.identity, "alice");
        assert_eq!(ast.subject.properties.len(), 2);

        // Check native JSON for string
        assert_eq!(ast.subject.properties.get("name").unwrap(), "Alice");

        // Check native JSON for integer (canonical format)
        let age_value = ast.subject.properties.get("age").unwrap();
        assert_eq!(age_value, 30); // Native JSON number, not tagged
    }

    #[test]
    fn test_value_serialization_simple_types() {
        // Integer (native JSON)
        let v = value_to_json(&Value::VInteger(42));
        assert_eq!(v, serde_json::json!(42));
        assert!(v.is_number());

        // Decimal (native JSON)
        let v = value_to_json(&Value::VDecimal(3.14));
        assert_eq!(v, serde_json::json!(3.14));
        assert!(v.is_number());

        // Boolean
        let v = value_to_json(&Value::VBoolean(true));
        assert_eq!(v, serde_json::Value::Bool(true));

        // String
        let v = value_to_json(&Value::VString("hello".to_string()));
        assert_eq!(v, serde_json::Value::String("hello".to_string()));

        // Array (with native JSON integers)
        let v = value_to_json(&Value::VArray(vec![Value::VInteger(1), Value::VInteger(2)]));
        assert!(v.is_array());
        assert_eq!(v.as_array().unwrap().len(), 2);
        assert_eq!(v.as_array().unwrap()[0], serde_json::json!(1));
        assert_eq!(v.as_array().unwrap()[1], serde_json::json!(2));
    }

    #[test]
    fn test_value_serialization_tagged_types() {
        // Symbol (lowercase type discriminator)
        let v = value_to_json(&Value::VSymbol("user123".to_string()));
        assert_eq!(v["type"], "symbol");
        assert_eq!(v["value"], "user123");

        // Range (lowercase type discriminator)
        let v = value_to_json(&Value::VRange(pattern_core::RangeValue {
            lower: Some(1.0),
            upper: Some(10.0),
        }));
        assert_eq!(v["type"], "range");
        assert_eq!(v["lower"], 1.0);
        assert_eq!(v["upper"], 10.0);

        // Measurement (lowercase type discriminator)
        let v = value_to_json(&Value::VMeasurement {
            unit: "cm".to_string(),
            value: 168.0,
        });
        assert_eq!(v["type"], "measurement");
        assert_eq!(v["value"], 168.0);
        assert_eq!(v["unit"], "cm");

        // Tagged string (lowercase type discriminator)
        let v = value_to_json(&Value::VTaggedString {
            tag: "date".to_string(),
            content: "2024-01-09".to_string(),
        });
        assert_eq!(v["type"], "tagged");
        assert_eq!(v["tag"], "date");
        assert_eq!(v["content"], "2024-01-09");
    }

    #[test]
    fn test_value_serialization_map() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), Value::VString("value1".to_string()));
        map.insert("key2".to_string(), Value::VInteger(42));

        let v = value_to_json(&Value::VMap(map));

        assert!(v.is_object());
        assert_eq!(v["key1"], "value1");
        // Integer is now native JSON, not tagged
        assert_eq!(v["key2"], 42);
    }
}
