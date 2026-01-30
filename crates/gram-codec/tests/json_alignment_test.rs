//! Test to verify JSON output aligns with gram-hs canonical format

use gram_codec::parse_to_ast;
use serde_json::json;

#[test]
fn test_json_format_matches_gram_hs() {
    // Parse a simple pattern
    let input = "(alice:Person {name: \"Alice\", age: 30})";
    let ast = parse_to_ast(input).expect("Failed to parse");

    // Serialize to JSON
    let json_output = serde_json::to_value(&ast).expect("Failed to serialize");

    // Expected format matching gram-hs canonical JSON
    let expected = json!({
        "subject": {
            "identity": "alice",
            "labels": ["Person"],
            "properties": {
                "name": "Alice",
                "age": 30  // Native JSON number, not tagged
            }
        },
        "elements": []
    });

    assert_eq!(
        json_output, expected,
        "JSON format should match gram-hs canonical format"
    );
}

#[test]
fn test_lowercase_type_discriminators() {
    // Test that complex types use lowercase discriminators (not capitalized)
    let input = r#"(node {range: 1..10, body: md`# Heading`})"#;

    let ast = parse_to_ast(input).expect("Failed to parse");
    let json_output = serde_json::to_value(&ast).expect("Failed to serialize");

    let properties = &json_output["subject"]["properties"];

    // Range should have lowercase "range" type (not "Range")
    assert_eq!(properties["range"]["type"], "range");

    // Range bounds should be numbers (JSON doesn't distinguish int/float)
    // The grammar allows _numeric_literal which can be integer or decimal
    assert!(properties["range"]["lower"].is_number());
    assert!(properties["range"]["upper"].is_number());

    // Verify the actual numeric values (JSON may represent as float)
    assert_eq!(properties["range"]["lower"].as_f64().unwrap(), 1.0);
    assert_eq!(properties["range"]["upper"].as_f64().unwrap(), 10.0);

    // Tagged string should have lowercase "tagged" type (not "Tagged")
    assert_eq!(properties["body"]["type"], "tagged");
    assert_eq!(properties["body"]["tag"], "md");
    assert_eq!(properties["body"]["content"], "# Heading");
}

#[test]
fn test_native_json_numbers() {
    // Test that integers and decimals use native JSON, not tagged objects
    let input = "(data {count: 42, temperature: 98.6})";
    let ast = parse_to_ast(input).expect("Failed to parse");
    let json_output = serde_json::to_value(&ast).expect("Failed to serialize");

    let properties = &json_output["subject"]["properties"];

    // Should be native JSON numbers, not objects with "type" field
    assert!(properties["count"].is_number());
    assert_eq!(properties["count"], 42);

    assert!(properties["temperature"].is_number());
    assert_eq!(properties["temperature"], 98.6);

    // Verify they don't have a "type" field (would indicate tagged format)
    assert!(properties["count"].get("type").is_none());
    assert!(properties["temperature"].get("type").is_none());
}

#[test]
fn test_field_names_match_gram_hs() {
    // Verify field names match current gram-hs state
    let input = "(alice:Person)";
    let ast = parse_to_ast(input).expect("Failed to parse");
    let json_output = serde_json::to_value(&ast).expect("Failed to serialize");

    // Pattern should have "subject" field (current gram-hs state)
    assert!(json_output.get("subject").is_some());
    assert!(json_output.get("elements").is_some());

    // Subject should have "identity" field (aligned with gram-hs)
    let subject = &json_output["subject"];
    assert!(subject.get("identity").is_some());
    assert!(subject.get("labels").is_some());
    assert!(subject.get("properties").is_some());

    assert_eq!(subject["identity"], "alice");
}
