//! Integration tests for AST output

use gram_codec::{parse_to_ast, AstPattern};

#[test]
fn test_parse_to_ast_simple_node() {
    let ast = parse_to_ast("(alice:Person {name: \"Alice\"})").unwrap();

    assert_eq!(ast.subject.identity, "alice");
    assert_eq!(ast.subject.labels, vec!["Person"]);
    assert_eq!(ast.subject.properties.len(), 1);
    assert_eq!(ast.subject.properties.get("name").unwrap(), "Alice");
    assert_eq!(ast.elements.len(), 0);
}

#[test]
fn test_parse_to_ast_empty() {
    let ast = parse_to_ast("").unwrap();

    assert_eq!(ast.subject.identity, "");
    assert_eq!(ast.subject.labels.len(), 0);
    assert_eq!(ast.subject.properties.len(), 0);
    assert_eq!(ast.elements.len(), 0);
}

#[test]
fn test_parse_to_ast_with_elements() {
    let ast = parse_to_ast("[team | (alice), (bob)]").unwrap();

    assert_eq!(ast.subject.identity, "team");
    assert_eq!(ast.elements.len(), 2);
    assert_eq!(ast.elements[0].subject.identity, "alice");
    assert_eq!(ast.elements[1].subject.identity, "bob");
}

#[test]
fn test_parse_to_ast_json_serialization() {
    let ast = parse_to_ast("(alice:Person)").unwrap();

    // Serialize to JSON
    let json = serde_json::to_string(&ast).unwrap();
    assert!(json.contains("alice"));
    assert!(json.contains("Person"));

    // Deserialize back
    let deserialized: AstPattern = serde_json::from_str(&json).unwrap();
    assert_eq!(deserialized.subject.identity, "alice");
    assert_eq!(deserialized.subject.labels, vec!["Person"]);
}

#[test]
fn test_parse_to_ast_path_notation() {
    let ast = parse_to_ast("(alice)-->(bob)").unwrap();

    // Path notation creates a file-level pattern with elements
    // The exact structure depends on parser implementation
    // but we should get at least alice and bob somewhere
    let json = serde_json::to_string(&ast).unwrap();
    assert!(json.contains("alice"));
    assert!(json.contains("bob"));
}

#[test]
fn test_parse_to_ast_invalid_input() {
    let result = parse_to_ast("(unclosed");
    assert!(result.is_err());
}

#[test]
fn test_parse_to_ast_empty_node() {
    let ast = parse_to_ast("() (a)").unwrap();
    // Both nodes should be present as elements of the document
    assert_eq!(
        ast.elements.len(),
        2,
        "Expected 2 elements for '() (a)', but found {}",
        ast.elements.len()
    );
    assert_eq!(ast.elements[1].subject.identity, "a");
}
