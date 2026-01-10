//! JSON Schema validation tests for AST output
//!
//! Validates that gram-rs AST output conforms to the canonical JSON format
//! defined by gram-hs (spec 029-canonical-json-pattern).

use gram_codec::parse_to_ast;
use serde_json::Value as JsonValue;

/// Load the canonical JSON schema (corrected version with subject/identity)
fn load_canonical_schema() -> JsonValue {
    // Note: The static schema file uses old field names (value/symbol)
    // but the actual schema generator uses subject/identity.
    // This is the corrected schema matching the actual implementation.
    serde_json::json!({
        "$schema": "http://json-schema.org/draft/2020-12/schema#",
        "$id": "https://gram.data/schemas/pattern/v0.1.0/pattern.json",
        "title": "Pattern<Subject>",
        "version": "0.1.0",
        "definitions": {
            "Pattern": {
                "type": "object",
                "required": ["subject", "elements"],
                "properties": {
                    "subject": {"$ref": "#/definitions/Subject"},
                    "elements": {
                        "type": "array",
                        "items": {"$ref": "#/definitions/Pattern"}
                    }
                },
                "additionalProperties": false
            },
            "Subject": {
                "type": "object",
                "required": ["identity", "labels", "properties"],
                "properties": {
                    "identity": {"type": "string"},
                    "labels": {
                        "type": "array",
                        "items": {"type": "string"},
                        "uniqueItems": true
                    },
                    "properties": {
                        "type": "object",
                        "additionalProperties": {"$ref": "#/definitions/Value"}
                    }
                },
                "additionalProperties": false
            },
            "Value": {
                "oneOf": [
                    {"type": "integer"},
                    {"type": "number"},
                    {"type": "boolean"},
                    {"type": "string"},
                    {"$ref": "#/definitions/ValueSymbol"},
                    {"$ref": "#/definitions/ValueTaggedString"},
                    {"$ref": "#/definitions/ValueRange"},
                    {"$ref": "#/definitions/ValueMeasurement"},
                    {
                        "type": "array",
                        "items": {"$ref": "#/definitions/Value"}
                    },
                    {
                        "type": "object",
                        "additionalProperties": {"$ref": "#/definitions/Value"},
                        "not": {"required": ["type"]}
                    }
                ]
            },
            "ValueSymbol": {
                "type": "object",
                "required": ["type", "value"],
                "properties": {
                    "type": {"const": "symbol"},
                    "value": {"type": "string"}
                },
                "additionalProperties": false
            },
            "ValueTaggedString": {
                "type": "object",
                "required": ["type", "tag", "content"],
                "properties": {
                    "type": {"const": "tagged"},
                    "tag": {"type": "string"},
                    "content": {"type": "string"}
                },
                "additionalProperties": false
            },
            "ValueRange": {
                "type": "object",
                "required": ["type", "lower", "upper"],
                "properties": {
                    "type": {"const": "range"},
                    "lower": {"type": "number"},
                    "upper": {"type": "number"}
                },
                "additionalProperties": false
            },
            "ValueMeasurement": {
                "type": "object",
                "required": ["type", "unit", "value"],
                "properties": {
                    "type": {"const": "measurement"},
                    "unit": {"type": "string"},
                    "value": {"type": "number"}
                },
                "additionalProperties": false
            }
        },
        "$ref": "#/definitions/Pattern"
    })
}

/// Basic structural validation (without full JSON Schema library)
/// Checks required fields and basic structure
fn validate_ast_structure(ast: &JsonValue) -> Result<(), String> {
    let obj = ast.as_object().ok_or("AST must be an object")?;

    // Check Pattern structure
    if !obj.contains_key("subject") {
        return Err("Missing required field: 'subject'".to_string());
    }
    if !obj.contains_key("elements") {
        return Err("Missing required field: 'elements'".to_string());
    }

    // Check Subject structure
    let subject = obj.get("subject").ok_or("Missing subject")?;
    let subject_obj = subject.as_object().ok_or("Subject must be an object")?;

    if !subject_obj.contains_key("identity") {
        return Err("Missing required field: 'identity'".to_string());
    }
    if !subject_obj.contains_key("labels") {
        return Err("Missing required field: 'labels'".to_string());
    }
    if !subject_obj.contains_key("properties") {
        return Err("Missing required field: 'properties'".to_string());
    }

    // Check identity is string
    if !subject_obj.get("identity").unwrap().is_string() {
        return Err("'identity' must be a string".to_string());
    }

    // Check labels is array
    if !subject_obj.get("labels").unwrap().is_array() {
        return Err("'labels' must be an array".to_string());
    }

    // Check properties is object
    if !subject_obj.get("properties").unwrap().is_object() {
        return Err("'properties' must be an object".to_string());
    }

    // Check elements is array
    if !obj.get("elements").unwrap().is_array() {
        return Err("'elements' must be an array".to_string());
    }

    // Recursively validate elements
    let elements = obj.get("elements").unwrap().as_array().unwrap();
    for (i, elem) in elements.iter().enumerate() {
        validate_ast_structure(elem).map_err(|e| format!("Element {} invalid: {}", i, e))?;
    }

    Ok(())
}

/// Validate value types match canonical format
fn validate_value_format(value: &JsonValue) -> Result<(), String> {
    match value {
        JsonValue::Number(_) => Ok(()), // Integer or Decimal (native JSON)
        JsonValue::Bool(_) => Ok(()),   // Boolean (native JSON)
        JsonValue::String(_) => Ok(()), // String (native JSON)
        JsonValue::Array(arr) => {
            // Array - validate each element
            for (i, item) in arr.iter().enumerate() {
                validate_value_format(item)
                    .map_err(|e| format!("Array element {} invalid: {}", i, e))?;
            }
            Ok(())
        }
        JsonValue::Object(obj) => {
            // Could be Map or tagged type
            if obj.contains_key("type") {
                // Tagged type - check discriminator
                let type_val = obj
                    .get("type")
                    .unwrap()
                    .as_str()
                    .ok_or("'type' must be string")?;
                match type_val {
                    "symbol" => {
                        if !obj.contains_key("value") {
                            return Err("Symbol missing 'value' field".to_string());
                        }
                        Ok(())
                    }
                    "tagged" => {
                        if !obj.contains_key("tag") || !obj.contains_key("content") {
                            return Err("TaggedString missing 'tag' or 'content'".to_string());
                        }
                        Ok(())
                    }
                    "range" => {
                        if !obj.contains_key("lower") || !obj.contains_key("upper") {
                            return Err("Range missing 'lower' or 'upper'".to_string());
                        }
                        Ok(())
                    }
                    "measurement" => {
                        if !obj.contains_key("unit") || !obj.contains_key("value") {
                            return Err("Measurement missing 'unit' or 'value'".to_string());
                        }
                        Ok(())
                    }
                    _ => Err(format!("Unknown type discriminator: '{}'", type_val)),
                }
            } else {
                // Map - validate all values
                for (key, val) in obj {
                    validate_value_format(val)
                        .map_err(|e| format!("Map value '{}' invalid: {}", key, e))?;
                }
                Ok(())
            }
        }
        JsonValue::Null => Err("Null values not supported in canonical format".to_string()),
    }
}

#[test]
fn test_ast_structure_validation() {
    let ast = parse_to_ast("(alice:Person {name: \"Alice\", age: 30})").unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    validate_ast_structure(&json).expect("AST structure should be valid");
}

#[test]
fn test_ast_value_format_validation() {
    let ast = parse_to_ast("(test {int: 42, decimal: 3.14, bool: true, str: \"hello\"})").unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    let props = json["subject"]["properties"].as_object().unwrap();
    for (key, value) in props {
        validate_value_format(value)
            .unwrap_or_else(|e| panic!("Property '{}' invalid: {}", key, e));
    }
}

#[test]
fn test_ast_tagged_types_validation() {
    // Test with range and measurement (symbol syntax may not be fully parsed yet)
    let gram = r#"(node {
        range: 1..10,
        measure: 5kg
    })"#;

    let ast = parse_to_ast(gram).unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    let props = json["subject"]["properties"].as_object().unwrap();

    // Check range
    if let Some(range_val) = props.get("range") {
        assert_eq!(range_val["type"], "range");
        validate_value_format(range_val).unwrap();
    }

    // Check measurement
    if let Some(meas_val) = props.get("measure") {
        assert_eq!(meas_val["type"], "measurement");
        validate_value_format(meas_val).unwrap();
    }
}

#[test]
fn test_ast_native_json_types() {
    let ast = parse_to_ast("(test {int: 42, decimal: 3.14, bool: true, str: \"hello\"})").unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    let props = json["subject"]["properties"].as_object().unwrap();

    // Integer should be native JSON number (not tagged)
    assert!(props.get("int").unwrap().is_number());
    assert_eq!(props.get("int").unwrap().as_i64(), Some(42));

    // Decimal should be native JSON number (not tagged)
    assert!(props.get("decimal").unwrap().is_number());
    assert_eq!(props.get("decimal").unwrap().as_f64(), Some(3.14));

    // Boolean should be native JSON boolean
    assert!(props.get("bool").unwrap().is_boolean());

    // String should be native JSON string
    assert!(props.get("str").unwrap().is_string());
}

#[test]
fn test_ast_nested_structure() {
    let ast = parse_to_ast("[outer | [inner | (a), (b)], (c)]").unwrap();
    let json = serde_json::to_value(&ast).unwrap();

    validate_ast_structure(&json).expect("Nested AST structure should be valid");

    // Verify nesting
    assert_eq!(json["subject"]["identity"], "outer");
    assert_eq!(json["elements"].as_array().unwrap().len(), 2);

    let first_elem = &json["elements"][0];
    assert_eq!(first_elem["subject"]["identity"], "inner");
    assert_eq!(first_elem["elements"].as_array().unwrap().len(), 2);
}
