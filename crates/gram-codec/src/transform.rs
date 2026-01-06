//! Transformation functions from tree-sitter CST to Pattern AST

use crate::{error::Location, ParseError, Value};
use pattern_core::{Pattern, Subject, Symbol};
use std::collections::{HashMap, HashSet};

/// Transform tree-sitter CST to Pattern AST
pub(crate) fn transform_tree(
    tree: &tree_sitter::Tree,
    input: &str,
) -> Result<Vec<Pattern<Subject>>, ParseError> {
    let root_node = tree.root_node();

    // The root should be a gram_pattern node
    if root_node.kind() != "gram_pattern" {
        return Err(ParseError::new(
            Location::from_node(&root_node),
            format!("Expected gram_pattern root, found {}", root_node.kind()),
        ));
    }

    transform_gram_pattern(&root_node, input)
}

/// Transform a gram_pattern node to Vec<Pattern>
///
/// gram_pattern can contain:
/// - Optional root record
/// - Comments (ignored)
/// - Pattern nodes (node_pattern, relationship_pattern, subject_pattern, annotated_pattern)
fn transform_gram_pattern(
    node: &tree_sitter::Node,
    input: &str,
) -> Result<Vec<Pattern<Subject>>, ParseError> {
    let mut patterns = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if !child.is_named() {
            continue; // Skip punctuation
        }

        match child.kind() {
            "comment" | "record" => {
                // Comments and root records are ignored for now
                // Root records would be handled separately if needed
                continue;
            }
            "node_pattern" => {
                patterns.push(transform_node_pattern(&child, input)?);
            }
            "relationship_pattern" => {
                patterns.push(transform_relationship_pattern(&child, input)?);
            }
            "subject_pattern" => {
                patterns.push(transform_subject_pattern(&child, input)?);
            }
            "annotated_pattern" => {
                patterns.push(transform_annotated_pattern(&child, input)?);
            }
            _ => {
                // Unknown pattern type - could be an extension
                continue;
            }
        }
    }

    Ok(patterns)
}

/// Transform a node_pattern to Pattern (0 elements)
///
/// node_pattern: `(subject)`
fn transform_node_pattern(
    node: &tree_sitter::Node,
    input: &str,
) -> Result<Pattern<Subject>, ParseError> {
    // Transform subject from node_pattern fields
    let subject = transform_subject(node, input)?;
    Ok(Pattern::point(subject))
}

/// Transform a relationship_pattern to Pattern (2 elements)
///
/// relationship_pattern: `(left)-[edge]->(right)`
fn transform_relationship_pattern(
    node: &tree_sitter::Node,
    input: &str,
) -> Result<Pattern<Subject>, ParseError> {
    // Extract left, kind (arrow type), and right nodes
    let left_node = node
        .child_by_field_name("left")
        .ok_or_else(|| ParseError::missing_field(node, "left"))?;

    let right_node = node
        .child_by_field_name("right")
        .ok_or_else(|| ParseError::missing_field(node, "right"))?;

    let kind_node = node
        .child_by_field_name("kind")
        .ok_or_else(|| ParseError::missing_field(node, "kind"))?;

    // Transform left and right patterns
    let left_pattern = transform_pattern_node(&left_node, input)?;
    let right_pattern = transform_pattern_node(&right_node, input)?;

    // Handle arrow type and element ordering
    let (first, second) = handle_arrow_type(&kind_node, left_pattern, right_pattern);

    // Extract edge subject (labels/properties between arrows)
    let edge_subject = extract_edge_subject(node, input)?;

    // Create relationship pattern
    Ok(Pattern {
        value: edge_subject,
        elements: vec![first, second],
    })
}

/// Transform a subject_pattern to Pattern (N elements)
///
/// subject_pattern: `[subject | elements]`
fn transform_subject_pattern(
    node: &tree_sitter::Node,
    input: &str,
) -> Result<Pattern<Subject>, ParseError> {
    // Extract subject (from identifier, labels, record fields)
    let subject = transform_subject(node, input)?;

    // Extract elements - look for subject_pattern_elements child node
    let mut elements = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "subject_pattern_elements" {
            // Found the elements container, now iterate its children
            let mut elem_cursor = child.walk();
            for elem_child in child.children(&mut elem_cursor) {
                if !elem_child.is_named() {
                    continue; // Skip punctuation like commas
                }

                match elem_child.kind() {
                    "pattern_reference" => {
                        // Pattern reference is just an identifier - create atomic pattern
                        elements.push(transform_pattern_reference(&elem_child, input)?);
                    }
                    "node_pattern"
                    | "relationship_pattern"
                    | "subject_pattern"
                    | "annotated_pattern" => {
                        elements.push(transform_pattern_node(&elem_child, input)?);
                    }
                    _ => {
                        // Unknown child type, skip
                        continue;
                    }
                }
            }
            break; // Found elements, stop looking
        }
    }

    Ok(Pattern {
        value: subject,
        elements,
    })
}

/// Transform an annotated_pattern to Pattern (1 element)
///
/// annotated_pattern: `@key(value) pattern`
fn transform_annotated_pattern(
    node: &tree_sitter::Node,
    input: &str,
) -> Result<Pattern<Subject>, ParseError> {
    // Extract annotation (the @key(value) part)
    let annotation_subject = if let Some(anno_node) = node.child_by_field_name("annotations") {
        extract_annotation_subject(&anno_node, input)?
    } else {
        Subject {
            identity: Symbol(String::new()),
            labels: HashSet::new(),
            properties: HashMap::new(),
        }
    };

    // Extract the pattern being annotated (field name is "elements")
    let pattern_node = node
        .child_by_field_name("elements")
        .ok_or_else(|| ParseError::missing_field(node, "elements"))?;

    let element = transform_pattern_node(&pattern_node, input)?;

    Ok(Pattern {
        value: annotation_subject,
        elements: vec![element],
    })
}

/// Transform any pattern node to Pattern
fn transform_pattern_node(
    node: &tree_sitter::Node,
    input: &str,
) -> Result<Pattern<Subject>, ParseError> {
    match node.kind() {
        "node_pattern" => transform_node_pattern(node, input),
        "relationship_pattern" => transform_relationship_pattern(node, input),
        "subject_pattern" => transform_subject_pattern(node, input),
        "annotated_pattern" => transform_annotated_pattern(node, input),
        _ => Err(ParseError::from_node(
            node,
            format!("Unknown pattern type: {}", node.kind()),
        )),
    }
}

/// Transform a subject (identifier, labels, record) to Subject
fn transform_subject(node: &tree_sitter::Node, input: &str) -> Result<Subject, ParseError> {
    let mut identity = Symbol(String::new());
    let mut labels = HashSet::new();
    let mut properties = HashMap::new();

    // Extract identifier from field
    if let Some(id_node) = node.child_by_field_name("identifier") {
        let id_text = extract_identifier(&id_node, input)?;
        identity = Symbol(id_text);
    }

    // Extract labels from field
    if let Some(labels_node) = node.child_by_field_name("labels") {
        labels = extract_labels(&labels_node, input)?;
    }

    // Extract record from field
    if let Some(record_node) = node.child_by_field_name("record") {
        properties = transform_record(&record_node, input)?;
    }

    Ok(Subject {
        identity,
        labels,
        properties,
    })
}

/// Transform a pattern_reference to Pattern
///
/// pattern_reference is just an identifier reference: `alice` in `[team | alice, bob]`
fn transform_pattern_reference(
    node: &tree_sitter::Node,
    input: &str,
) -> Result<Pattern<Subject>, ParseError> {
    let mut identity = Symbol(String::new());

    // Extract identifier from field
    if let Some(id_node) = node.child_by_field_name("identifier") {
        let id_text = extract_identifier(&id_node, input)?;
        identity = Symbol(id_text);
    }

    Ok(Pattern::point(Subject {
        identity,
        labels: HashSet::new(),
        properties: HashMap::new(),
    }))
}

/// Transform a record to HashMap<String, Value>
fn transform_record(
    node: &tree_sitter::Node,
    input: &str,
) -> Result<HashMap<String, pattern_core::Value>, ParseError> {
    let mut properties = HashMap::new();

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.kind() == "record_property" {
            // Extract key
            let key_node = child
                .child_by_field_name("key")
                .ok_or_else(|| ParseError::missing_field(&child, "key"))?;
            let key_text = extract_identifier(&key_node, input)?;

            // Extract value
            let value_node = child
                .child_by_field_name("value")
                .ok_or_else(|| ParseError::missing_field(&child, "value"))?;

            let value = transform_value_to_pattern_value(&value_node, input)?;
            properties.insert(key_text, value);
        }
    }

    Ok(properties)
}

/// Transform a value node to pattern_core::Value
fn transform_value_to_pattern_value(
    node: &tree_sitter::Node,
    input: &str,
) -> Result<pattern_core::Value, ParseError> {
    // First parse to gram_codec::Value
    let codec_value = Value::from_tree_sitter_node(node, input)?;

    // Convert to pattern_core::Value
    match codec_value {
        Value::String(s) => Ok(pattern_core::Value::VString(s)),
        Value::Integer(i) => Ok(pattern_core::Value::VInteger(i)),
        Value::Decimal(f) => Ok(pattern_core::Value::VDecimal(f)),
        Value::Boolean(b) => Ok(pattern_core::Value::VBoolean(b)),
        Value::Array(arr) => {
            let converted: Result<Vec<_>, _> =
                arr.into_iter().map(value_to_pattern_value).collect();
            Ok(pattern_core::Value::VArray(converted?))
        }
        Value::Range { lower, upper } => {
            Ok(pattern_core::Value::VRange(pattern_core::RangeValue {
                lower: Some(lower as f64),
                upper: Some(upper as f64),
            }))
        }
        Value::TaggedString { tag, content } => {
            Ok(pattern_core::Value::VTaggedString { tag, content })
        }
    }
}

/// Helper to convert gram_codec::Value to pattern_core::Value
fn value_to_pattern_value(v: Value) -> Result<pattern_core::Value, ParseError> {
    match v {
        Value::String(s) => Ok(pattern_core::Value::VString(s)),
        Value::Integer(i) => Ok(pattern_core::Value::VInteger(i)),
        Value::Decimal(f) => Ok(pattern_core::Value::VDecimal(f)),
        Value::Boolean(b) => Ok(pattern_core::Value::VBoolean(b)),
        Value::Array(arr) => {
            let converted: Result<Vec<_>, _> =
                arr.into_iter().map(value_to_pattern_value).collect();
            Ok(pattern_core::Value::VArray(converted?))
        }
        Value::Range { lower, upper } => {
            Ok(pattern_core::Value::VRange(pattern_core::RangeValue {
                lower: Some(lower as f64),
                upper: Some(upper as f64),
            }))
        }
        Value::TaggedString { tag, content } => {
            Ok(pattern_core::Value::VTaggedString { tag, content })
        }
    }
}

/// Determine arrow type and element ordering for relationship patterns
fn handle_arrow_type(
    kind_node: &tree_sitter::Node,
    left: Pattern<Subject>,
    right: Pattern<Subject>,
) -> (Pattern<Subject>, Pattern<Subject>) {
    match kind_node.kind() {
        "left_arrow" => (right, left), // Reverse for left arrow
        "right_arrow" | "bidirectional_arrow" | "undirected_arrow" => (left, right), // Preserve order
        _ => (left, right), // Default: preserve order
    }
}

/// Extract edge subject from relationship pattern (labels/properties between arrows)
fn extract_edge_subject(node: &tree_sitter::Node, input: &str) -> Result<Subject, ParseError> {
    // Extract labels and properties from the "kind" (arrow) node
    // Parse tree: (relationship_pattern ... kind: (right_arrow labels: ... record: ...) ...)
    let kind_node = node
        .child_by_field_name("kind")
        .ok_or_else(|| ParseError::missing_field(node, "kind"))?;

    // Extract labels from the arrow node
    let labels = if let Some(labels_node) = kind_node.child_by_field_name("labels") {
        extract_labels(&labels_node, input)?
    } else {
        HashSet::new()
    };

    // Extract properties from the arrow node
    let properties = if let Some(record_node) = kind_node.child_by_field_name("record") {
        transform_record(&record_node, input)?
    } else {
        HashMap::new()
    };

    // Edge subject has no identifier, only labels and properties
    Ok(Subject {
        identity: Symbol(String::new()),
        labels,
        properties,
    })
}

/// Extract annotation subject from annotations node
///
/// # Annotation Representation
///
/// Annotations are key/value pairs that form a property record for an anonymous,
/// unlabeled pattern with a single element (the annotated target).
///
/// For example: `@type(node) @depth(2) (a)` becomes:
/// ```text
/// Pattern {
///   value: Subject {
///     identity: Symbol(""),  // Anonymous
///     labels: {},            // Unlabeled
///     properties: {
///       "type": String("node"),
///       "depth": Integer(2)
///     }
///   },
///   elements: [Pattern(a)]
/// }
/// ```
///
/// This representation:
/// - Naturally supports multiple annotations
/// - Makes annotations semantically consistent as metadata properties
/// - Enables round-trip correctness (serializer can detect anonymous + properties = annotations)
fn extract_annotation_subject(
    node: &tree_sitter::Node,
    input: &str,
) -> Result<Subject, ParseError> {
    // Parse tree structure: (annotations (annotation key: (symbol) value: (type))*)

    let mut properties = HashMap::new();
    let mut cursor = node.walk();

    // Iterate all annotation children and collect them as properties
    for child in node.children(&mut cursor) {
        if child.kind() == "annotation" {
            // Extract key field
            let key_node = child.child_by_field_name("key").ok_or_else(|| {
                ParseError::from_node(&child, "Annotation missing key field".to_string())
            })?;

            let key = key_node
                .utf8_text(input.as_bytes())
                .map_err(|e| ParseError::from_node(&key_node, format!("UTF-8 error: {}", e)))?
                .to_string();

            // Extract value field if present
            if let Some(value_node) = child.child_by_field_name("value") {
                let value = transform_value_to_pattern_value(&value_node, input)?;
                properties.insert(key, value);
            }
        }
    }

    Ok(Subject {
        identity: Symbol(String::new()), // Anonymous
        labels: HashSet::new(),          // Unlabeled
        properties,                      // Annotations as properties
    })
}

/// Extract identifier text from node
fn extract_identifier(node: &tree_sitter::Node, input: &str) -> Result<String, ParseError> {
    let text = node
        .utf8_text(input.as_bytes())
        .map_err(|e| ParseError::from_node(node, format!("UTF-8 error: {}", e)))?;

    // Handle quoted strings
    if text.starts_with('"') && text.ends_with('"') && text.len() >= 2 {
        Ok(text[1..text.len() - 1].to_string())
    } else {
        Ok(text.to_string())
    }
}

/// Extract labels from labels node
fn extract_labels(node: &tree_sitter::Node, input: &str) -> Result<HashSet<String>, ParseError> {
    let mut labels = HashSet::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "symbol" {
            let label = child
                .utf8_text(input.as_bytes())
                .map_err(|e| ParseError::from_node(&child, format!("UTF-8 error: {}", e)))?
                .to_string();
            labels.insert(label);
        }
    }

    Ok(labels)
}
