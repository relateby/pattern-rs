//! JSON interchange functions for the gram codec.
//!
//! These functions form the stable contract between the Rust gram-codec and
//! native TypeScript/Python implementations. All cross-boundary communication
//! uses the JSON interchange format documented in the data-model spec.
//!
//! # JSON Interchange Format
//!
//! The format is an array of `AstPattern` objects:
//! ```json
//! [
//!   {
//!     "subject": {
//!       "identity": "alice",
//!       "labels": ["Person"],
//!       "properties": { "name": "Alice" }
//!     },
//!     "elements": []
//!   }
//! ]
//! ```
//!
//! Property values use mixed serialization:
//! - Primitives: native JSON (string, number, boolean)
//! - Complex types: tagged objects `{ "type": "symbol"|"range"|"tagged"|"measurement", ... }`

use crate::ast::AstPattern;
use pattern_core::{Pattern, RangeValue, Subject, Symbol, Value};
use std::collections::{HashMap, HashSet};

/// Parse gram notation and return a JSON array string of `AstPattern` objects.
///
/// # Arguments
///
/// * `input` - Gram notation text
///
/// # Returns
///
/// * `Ok(String)` - JSON array of AstPattern objects
/// * `Err(String)` - Parse error message
///
/// # Examples
///
/// ```rust
/// use gram_codec::json::gram_parse_to_json;
///
/// let json = gram_parse_to_json("(alice:Person)").unwrap();
/// assert!(json.contains("alice"));
/// assert!(json.contains("Person"));
/// ```
pub fn gram_parse_to_json(input: &str) -> Result<String, String> {
    if input.trim().is_empty() {
        return Ok("[]".to_string());
    }
    let patterns = crate::parse_gram(input).map_err(|e| e.to_string())?;
    let asts: Vec<AstPattern> = patterns.iter().map(AstPattern::from_pattern).collect();
    serde_json::to_string(&asts).map_err(|e| e.to_string())
}

/// Serialize a JSON array of `AstPattern` objects back to gram notation.
///
/// # Arguments
///
/// * `input` - JSON array string of AstPattern objects
///
/// # Returns
///
/// * `Ok(String)` - Gram notation text
/// * `Err(String)` - Serialization error message
///
/// # Examples
///
/// ```rust
/// use gram_codec::json::gram_stringify_from_json;
///
/// let gram = gram_stringify_from_json(r#"[{"subject":{"identity":"alice","labels":["Person"],"properties":{}},"elements":[]}]"#).unwrap();
/// assert!(gram.contains("alice"));
/// ```
pub fn gram_stringify_from_json(input: &str) -> Result<String, String> {
    let asts: Vec<AstPattern> = serde_json::from_str(input).map_err(|e| e.to_string())?;
    let patterns: Vec<Pattern<Subject>> = asts.iter().map(ast_to_pattern).collect();
    let gram_parts: Result<Vec<String>, String> = patterns
        .iter()
        .map(|p| crate::to_gram_pattern(p).map_err(|e| e.to_string()))
        .collect();
    Ok(gram_parts?.join(" "))
}

/// Validate gram notation and return an empty string on success, or an error message.
///
/// # Arguments
///
/// * `input` - Gram notation text
///
/// # Returns
///
/// A JSON string with an array of error strings (empty array = valid).
pub fn gram_validate_to_json(input: &str) -> String {
    match crate::validate_gram(input) {
        Ok(()) => "[]".to_string(),
        Err(e) => {
            let msg = e.to_string();
            serde_json::to_string(&[msg]).unwrap_or_else(|_| "[]".to_string())
        }
    }
}

/// Convert an `AstPattern` back to a native `Pattern<Subject>`.
fn ast_to_pattern(ast: &AstPattern) -> Pattern<Subject> {
    let subject = Subject {
        identity: Symbol(ast.subject.identity.clone()),
        labels: ast.subject.labels.iter().cloned().collect::<HashSet<_>>(),
        properties: ast
            .subject
            .properties
            .iter()
            .filter_map(|(k, v)| json_to_value(v).map(|val| (k.clone(), val)))
            .collect::<HashMap<_, _>>(),
    };
    let elements: Vec<Pattern<Subject>> = ast.elements.iter().map(ast_to_pattern).collect();
    if elements.is_empty() {
        Pattern::point(subject)
    } else {
        Pattern::pattern(subject, elements)
    }
}

/// Convert a `serde_json::Value` back to a `pattern_core::Value`.
///
/// Mirrors `value_to_json` in `ast.rs`. Returns `None` for JSON null (not representable).
fn json_to_value(v: &serde_json::Value) -> Option<Value> {
    match v {
        serde_json::Value::String(s) => Some(Value::VString(s.clone())),
        serde_json::Value::Bool(b) => Some(Value::VBoolean(*b)),
        serde_json::Value::Null => None,
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Some(Value::VInteger(i))
            } else {
                Some(Value::VDecimal(n.as_f64().unwrap_or(0.0)))
            }
        }
        serde_json::Value::Array(arr) => {
            let items: Vec<Value> = arr.iter().filter_map(json_to_value).collect();
            Some(Value::VArray(items))
        }
        serde_json::Value::Object(obj) => {
            // Check for tagged objects (symbol, range, measurement, tagged string)
            if let Some(type_tag) = obj.get("type").and_then(|t| t.as_str()) {
                match type_tag {
                    "symbol" => {
                        let val = obj.get("value")?.as_str()?.to_string();
                        Some(Value::VSymbol(val))
                    }
                    "range" => {
                        let lower = obj.get("lower").and_then(|v| v.as_f64());
                        let upper = obj.get("upper").and_then(|v| v.as_f64());
                        Some(Value::VRange(RangeValue { lower, upper }))
                    }
                    "measurement" => {
                        let unit = obj.get("unit")?.as_str()?.to_string();
                        let value = obj.get("value")?.as_f64()?;
                        Some(Value::VMeasurement { unit, value })
                    }
                    "tagged" => {
                        let tag = obj.get("tag")?.as_str()?.to_string();
                        let content = obj.get("content")?.as_str()?.to_string();
                        Some(Value::VTaggedString { tag, content })
                    }
                    _ => None,
                }
            } else {
                // Plain JSON object → VMap
                let map: HashMap<String, Value> = obj
                    .iter()
                    .filter_map(|(k, v)| json_to_value(v).map(|val| (k.clone(), val)))
                    .collect();
                Some(Value::VMap(map))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_input() {
        assert_eq!(gram_parse_to_json("").unwrap(), "[]");
        assert_eq!(gram_parse_to_json("   ").unwrap(), "[]");
    }

    #[test]
    fn test_parse_simple_node() {
        let json = gram_parse_to_json("(alice:Person)").unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0]["subject"]["identity"], "alice");
        assert_eq!(parsed[0]["subject"]["labels"][0], "Person");
    }

    #[test]
    fn test_parse_node_with_properties() {
        let json = gram_parse_to_json(r#"(a {name: "Alice", age: 30})"#).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed[0]["subject"]["properties"]["name"], "Alice");
        assert_eq!(parsed[0]["subject"]["properties"]["age"], 30);
    }

    #[test]
    fn test_parse_relationship() {
        let json = gram_parse_to_json("(a)-->(b)").unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0]["elements"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_stringify_round_trip() {
        let original = "(alice:Person)";
        let json = gram_parse_to_json(original).unwrap();
        let gram = gram_stringify_from_json(&json).unwrap();
        // Round-trip: re-parse and check identity
        let json2 = gram_parse_to_json(&gram).unwrap();
        let p1: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        let p2: Vec<serde_json::Value> = serde_json::from_str(&json2).unwrap();
        assert_eq!(p1[0]["subject"]["identity"], p2[0]["subject"]["identity"]);
        assert_eq!(p1[0]["subject"]["labels"], p2[0]["subject"]["labels"]);
    }

    #[test]
    fn test_validate_valid_input() {
        let result = gram_validate_to_json("(alice:Person)");
        let errors: Vec<String> = serde_json::from_str(&result).unwrap();
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_invalid_input() {
        let result = gram_validate_to_json("(((invalid");
        let errors: Vec<String> = serde_json::from_str(&result).unwrap();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_json_interchange_format_subject_key() {
        // Verifies the JSON uses "subject" key (not "value")
        let json = gram_parse_to_json("(x)").unwrap();
        assert!(json.contains("\"subject\""));
        assert!(!json.contains("\"value\"") || json.contains("\"value\":"));
    }

    #[test]
    fn test_value_types_in_json() {
        let json = gram_parse_to_json(r#"(a {s: "hello", i: 42, f: 3.14, b: true})"#).unwrap();
        let parsed: Vec<serde_json::Value> = serde_json::from_str(&json).unwrap();
        let props = &parsed[0]["subject"]["properties"];
        assert!(props["s"].is_string());
        assert!(props["i"].is_number());
        assert!(props["f"].is_number());
        assert!(props["b"].is_boolean());
    }

    #[test]
    fn test_json_to_value_tagged_types() {
        // symbol
        let v = json_to_value(&serde_json::json!({"type": "symbol", "value": "foo"})).unwrap();
        assert!(matches!(v, Value::VSymbol(_)));

        // measurement
        let v =
            json_to_value(&serde_json::json!({"type": "measurement", "unit": "kg", "value": 5.0}))
                .unwrap();
        assert!(matches!(v, Value::VMeasurement { .. }));

        // tagged string
        let v = json_to_value(
            &serde_json::json!({"type": "tagged", "tag": "date", "content": "2024-01-01"}),
        )
        .unwrap();
        assert!(matches!(v, Value::VTaggedString { .. }));

        // range
        let v = json_to_value(&serde_json::json!({"type": "range", "lower": 1.0, "upper": 10.0}))
            .unwrap();
        assert!(matches!(v, Value::VRange(_)));
    }
}
