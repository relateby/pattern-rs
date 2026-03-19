//! tree-sitter-backed CST parsing entry points.

use crate::cst::{Annotation, ArrowKind, CstParseResult, SourceSpan, SyntaxKind, SyntaxNode};
use crate::{Pattern, Subject, Value};
use std::collections::HashSet;
use tree_sitter::{Node, Parser};

pub fn parse_gram_cst(input: &str) -> CstParseResult {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_gram::LANGUAGE.into())
        .expect("tree-sitter-gram language should load");

    let Some(tree) = parser.parse(input, None) else {
        return CstParseResult {
            tree: Pattern::point(SyntaxNode {
                kind: SyntaxKind::Document,
                subject: None,
                span: SourceSpan {
                    start: 0,
                    end: input.len(),
                },
                annotations: vec![],
                text: None,
            }),
            errors: vec![],
        };
    };

    let root = tree.root_node();
    assert_eq!(root.kind(), "gram_pattern");

    let mut elements = Vec::new();
    let mut cursor = root.walk();
    for child in root.children(&mut cursor) {
        if !child.is_named() {
            continue;
        }

        match child.kind() {
            "record" => {}
            "node_pattern"
            | "relationship_pattern"
            | "subject_pattern"
            | "annotated_pattern"
            | "comment" => {
                elements.push(convert_named_node(child, input));
            }
            _ => {}
        }
    }

    CstParseResult {
        tree: Pattern::pattern(
            SyntaxNode {
                kind: SyntaxKind::Document,
                subject: root
                    .child_by_field_name("root")
                    .map(|record| extract_record_subject(record, input)),
                span: span_from_node(root),
                annotations: vec![],
                text: None,
            },
            elements,
        ),
        errors: collect_error_spans(root),
    }
}

fn convert_named_node(node: Node<'_>, input: &str) -> Pattern<SyntaxNode> {
    match node.kind() {
        "node_pattern" => convert_node_pattern(node, input),
        "relationship_pattern" => convert_relationship_pattern(node, input),
        "subject_pattern" => convert_subject_pattern(node, input),
        "annotated_pattern" => convert_annotated_pattern(node, input),
        "comment" => convert_comment(node, input),
        "pattern_reference" => convert_pattern_reference(node, input),
        kind => panic!("Unexpected CST node kind: {kind}"),
    }
}

fn convert_node_pattern(node: Node<'_>, input: &str) -> Pattern<SyntaxNode> {
    Pattern::point(SyntaxNode {
        kind: SyntaxKind::Node,
        subject: extract_subject(node, input),
        span: span_from_node(node),
        annotations: vec![],
        text: None,
    })
}

fn convert_relationship_pattern(node: Node<'_>, input: &str) -> Pattern<SyntaxNode> {
    let left = node
        .child_by_field_name("left")
        .map(|child| convert_named_node(child, input))
        .expect("relationship_pattern.left should be present");
    let right = node
        .child_by_field_name("right")
        .map(|child| convert_named_node(child, input))
        .expect("relationship_pattern.right should be present");
    let arrow_node = node
        .child_by_field_name("kind")
        .expect("relationship_pattern.kind should be present");

    Pattern::pattern(
        SyntaxNode {
            kind: SyntaxKind::Relationship(arrow_kind(arrow_node.kind())),
            subject: extract_subject(arrow_node, input),
            span: span_from_node(node),
            annotations: vec![],
            text: None,
        },
        vec![left, right],
    )
}

fn convert_subject_pattern(node: Node<'_>, input: &str) -> Pattern<SyntaxNode> {
    let mut elements = Vec::new();

    let mut node_cursor = node.walk();
    let elements_node = node
        .children(&mut node_cursor)
        .find(|child| child.is_named() && child.kind() == "subject_pattern_elements");

    if let Some(elements_node) = elements_node {
        let mut cursor = elements_node.walk();
        for child in elements_node.children(&mut cursor) {
            if !child.is_named() {
                continue;
            }

            match child.kind() {
                "pattern_reference"
                | "node_pattern"
                | "relationship_pattern"
                | "subject_pattern"
                | "annotated_pattern" => elements.push(convert_named_node(child, input)),
                _ => {}
            }
        }
    }

    Pattern::pattern(
        SyntaxNode {
            kind: SyntaxKind::Subject,
            subject: extract_subject(node, input),
            span: span_from_node(node),
            annotations: vec![],
            text: None,
        },
        elements,
    )
}

fn convert_annotated_pattern(node: Node<'_>, input: &str) -> Pattern<SyntaxNode> {
    let annotations = node
        .child_by_field_name("annotations")
        .map(|annotations_node| extract_annotations(annotations_node, input))
        .unwrap_or_default();
    let inner = node
        .child_by_field_name("elements")
        .map(|child| convert_named_node(child, input))
        .expect("annotated_pattern.elements should be present");

    Pattern::pattern(
        SyntaxNode {
            kind: SyntaxKind::Annotated,
            subject: None,
            span: span_from_node(node),
            annotations,
            text: None,
        },
        vec![inner],
    )
}

fn convert_comment(node: Node<'_>, input: &str) -> Pattern<SyntaxNode> {
    Pattern::point(SyntaxNode {
        kind: SyntaxKind::Comment,
        subject: None,
        span: span_from_node(node),
        annotations: vec![],
        text: Some(node_text(node, input).to_string()),
    })
}

fn convert_pattern_reference(node: Node<'_>, input: &str) -> Pattern<SyntaxNode> {
    let identifier = node
        .child_by_field_name("identifier")
        .map(|child| extract_identifier(child, input))
        .expect("pattern_reference.identifier should be present");

    Pattern::point(SyntaxNode {
        kind: SyntaxKind::Node,
        subject: Some(Subject {
            identity: pattern_core::Symbol(identifier),
            labels: HashSet::new(),
            properties: Default::default(),
        }),
        span: span_from_node(node),
        annotations: vec![],
        text: None,
    })
}

fn extract_annotations(node: Node<'_>, input: &str) -> Vec<Annotation> {
    let mut annotations = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if !child.is_named() {
            continue;
        }

        match child.kind() {
            "property_annotation" => annotations.push(extract_property_annotation(child, input)),
            "identified_annotation" => {
                annotations.push(extract_identified_annotation(child, input))
            }
            _ => {}
        }
    }

    annotations
}

fn extract_property_annotation(node: Node<'_>, input: &str) -> Annotation {
    let key = node
        .child_by_field_name("key")
        .map(|child| node_text(child, input).to_string())
        .expect("property_annotation.key should be present");
    let value_node = node
        .child_by_field_name("value")
        .expect("property_annotation.value should be present");

    Annotation::Property {
        key,
        value: extract_annotation_value(value_node, input),
    }
}

fn extract_identified_annotation(node: Node<'_>, input: &str) -> Annotation {
    let identity = node
        .child_by_field_name("identifier")
        .map(|child| pattern_core::Symbol(extract_identifier(child, input)));
    let labels = node
        .child_by_field_name("labels")
        .map(|labels| extract_label_list(labels, input))
        .unwrap_or_default();

    Annotation::Identified { identity, labels }
}

fn extract_annotation_value(node: Node<'_>, input: &str) -> Value {
    let raw = node_text(node, input);
    let parsed = crate::parser::value::value_parser(raw)
        .ok()
        .and_then(|(remaining, value)| remaining.trim().is_empty().then_some(value));

    match parsed {
        Some(pattern_core::Value::VString(value)) => Value::String(value),
        Some(pattern_core::Value::VSymbol(value)) => Value::String(value),
        Some(pattern_core::Value::VInteger(value)) => Value::Integer(value),
        Some(pattern_core::Value::VDecimal(value)) => Value::Decimal(value),
        Some(pattern_core::Value::VBoolean(value)) => Value::Boolean(value),
        Some(pattern_core::Value::VArray(values)) => Value::Array(
            values
                .into_iter()
                .map(pattern_value_to_annotation_value)
                .collect(),
        ),
        Some(pattern_core::Value::VRange(range)) => match (range.lower, range.upper) {
            (Some(lower), Some(upper)) if lower.fract() == 0.0 && upper.fract() == 0.0 => {
                Value::Range {
                    lower: lower as i64,
                    upper: upper as i64,
                }
            }
            _ => Value::String(raw.to_string()),
        },
        Some(pattern_core::Value::VTaggedString { tag, content }) => {
            Value::TaggedString { tag, content }
        }
        Some(pattern_core::Value::VMap(_)) | Some(pattern_core::Value::VMeasurement { .. }) => {
            Value::String(raw.to_string())
        }
        None => Value::String(raw.to_string()),
    }
}

fn pattern_value_to_annotation_value(value: pattern_core::Value) -> Value {
    match value {
        pattern_core::Value::VString(value) => Value::String(value),
        pattern_core::Value::VSymbol(value) => Value::String(value),
        pattern_core::Value::VInteger(value) => Value::Integer(value),
        pattern_core::Value::VDecimal(value) => Value::Decimal(value),
        pattern_core::Value::VBoolean(value) => Value::Boolean(value),
        pattern_core::Value::VArray(values) => Value::Array(
            values
                .into_iter()
                .map(pattern_value_to_annotation_value)
                .collect(),
        ),
        pattern_core::Value::VRange(range) => match (range.lower, range.upper) {
            (Some(lower), Some(upper)) if lower.fract() == 0.0 && upper.fract() == 0.0 => {
                Value::Range {
                    lower: lower as i64,
                    upper: upper as i64,
                }
            }
            _ => Value::String(format!("{range}")),
        },
        pattern_core::Value::VTaggedString { tag, content } => Value::TaggedString { tag, content },
        pattern_core::Value::VMap(map) => Value::String(pattern_core::Value::VMap(map).to_string()),
        pattern_core::Value::VMeasurement { unit, value } => {
            Value::String(format!("{value}{unit}"))
        }
    }
}

fn extract_subject(node: Node<'_>, input: &str) -> Option<Subject> {
    let has_identifier = node.child_by_field_name("identifier").is_some();
    let has_labels = node.child_by_field_name("labels").is_some();
    let has_record = node.child_by_field_name("record").is_some();
    let has_subject = node.child_by_field_name("subject").is_some();

    if !(has_identifier || has_labels || has_record || has_subject) {
        return None;
    }

    let identity = node
        .child_by_field_name("identifier")
        .map(|child| pattern_core::Symbol(extract_identifier(child, input)))
        .unwrap_or_else(|| pattern_core::Symbol(String::new()));
    let labels = node
        .child_by_field_name("labels")
        .map(|labels_node| extract_labels(labels_node, input))
        .unwrap_or_default();
    let properties = node
        .child_by_field_name("record")
        .map(|record| extract_record(record, input))
        .unwrap_or_default();

    Some(Subject {
        identity,
        labels,
        properties,
    })
}

fn extract_record_subject(node: Node<'_>, input: &str) -> Subject {
    Subject {
        identity: pattern_core::Symbol(String::new()),
        labels: HashSet::new(),
        properties: extract_record(node, input),
    }
}

fn extract_record(node: Node<'_>, input: &str) -> pattern_core::PropertyRecord {
    let raw = node_text(node, input);
    crate::parser::subject::record(raw)
        .ok()
        .and_then(|(remaining, record)| remaining.trim().is_empty().then_some(record))
        .unwrap_or_default()
}

fn extract_labels(node: Node<'_>, input: &str) -> HashSet<String> {
    extract_label_list(node, input).into_iter().collect()
}

fn extract_label_list(node: Node<'_>, input: &str) -> Vec<String> {
    let mut labels = Vec::new();
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.is_named() && child.kind() == "symbol" {
            labels.push(node_text(child, input).to_string());
        }
    }

    labels
}

fn extract_identifier(node: Node<'_>, input: &str) -> String {
    let raw = node_text(node, input);
    crate::parser::value::identifier(raw)
        .ok()
        .and_then(|(remaining, identifier)| remaining.trim().is_empty().then_some(identifier))
        .unwrap_or_else(|| raw.to_string())
}

fn collect_error_spans(node: Node<'_>) -> Vec<SourceSpan> {
    let mut spans = Vec::new();
    collect_error_spans_inner(node, &mut spans);
    spans
}

fn collect_error_spans_inner(node: Node<'_>, spans: &mut Vec<SourceSpan>) {
    if node.is_error() {
        spans.push(span_from_node(node));
    }

    if !(node.is_error() || node.has_error()) {
        return;
    }

    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        if child.is_error() || child.has_error() {
            collect_error_spans_inner(child, spans);
        }
    }
}

fn arrow_kind(kind: &str) -> ArrowKind {
    match kind {
        "right_arrow" => ArrowKind::Right,
        "left_arrow" => ArrowKind::Left,
        "bidirectional_arrow" => ArrowKind::Bidirectional,
        "undirected_arrow" => ArrowKind::Undirected,
        other => panic!("Unexpected relationship kind: {other}"),
    }
}

fn span_from_node(node: Node<'_>) -> SourceSpan {
    SourceSpan {
        start: node.start_byte(),
        end: node.end_byte(),
    }
}

fn node_text<'a>(node: Node<'_>, input: &'a str) -> &'a str {
    node.utf8_text(input.as_bytes())
        .expect("tree-sitter node text should be valid UTF-8")
}
