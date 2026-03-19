//! Lowering from syntax-preserving CST nodes to semantic patterns.

use crate::cst::{Annotation, ArrowKind, SyntaxKind, SyntaxNode};
use crate::{Pattern, Subject};
use pattern_core::{RangeValue, Symbol, Value};
use std::collections::{HashMap, HashSet};

pub fn lower(tree: Pattern<SyntaxNode>) -> Vec<Pattern<Subject>> {
    assert!(
        matches!(tree.value.kind, SyntaxKind::Document),
        "lower expects a document root"
    );

    let mut lowered = Vec::new();

    if let Some(subject) = tree.value.subject {
        lowered.push(Pattern::point(subject));
    }

    for element in tree.elements {
        if let Some(pattern) = lower_node(element) {
            lowered.push(pattern);
        }
    }

    lowered
}

fn lower_node(node: Pattern<SyntaxNode>) -> Option<Pattern<Subject>> {
    match node.value.kind {
        SyntaxKind::Document => unreachable!("document nodes are only handled at the root"),
        SyntaxKind::Node => Some(Pattern::point(
            node.value.subject.unwrap_or_else(empty_subject),
        )),
        SyntaxKind::Subject => Some(Pattern::pattern(
            node.value.subject.unwrap_or_else(empty_subject),
            node.elements
                .into_iter()
                .filter_map(lower_node)
                .collect::<Vec<_>>(),
        )),
        SyntaxKind::Relationship(_) => Some(lower_relationship(node)),
        SyntaxKind::Annotated => {
            let mut elements = node.elements.into_iter().filter_map(lower_node);
            let inner = elements.next()?;
            Some(Pattern::pattern(
                annotation_subject(&node.value.annotations),
                vec![inner],
            ))
        }
        SyntaxKind::Comment => None,
    }
}

fn lower_relationship(node: Pattern<SyntaxNode>) -> Pattern<Subject> {
    let (operands, relationships) = flatten_relationship_chain(node);
    let mut operands = operands.into_iter();
    let mut acc = lower_node(
        operands
            .next()
            .expect("relationship chain should have a first operand"),
    )
    .expect("relationship operands should lower to patterns");

    for ((arrow_kind, subject), operand) in relationships.into_iter().zip(operands) {
        let next =
            lower_node(operand).expect("relationship chain operands should lower to patterns");
        let elements = if matches!(arrow_kind, ArrowKind::Left) {
            vec![next, acc]
        } else {
            vec![acc, next]
        };
        acc = Pattern::pattern(subject, elements);
    }

    acc
}

fn flatten_relationship_chain(
    node: Pattern<SyntaxNode>,
) -> (Vec<Pattern<SyntaxNode>>, Vec<(ArrowKind, Subject)>) {
    let arrow_kind = match node.value.kind {
        SyntaxKind::Relationship(arrow_kind) => arrow_kind,
        _ => unreachable!("flatten_relationship_chain only accepts relationships"),
    };

    let mut elements = node.elements.into_iter();
    let left = elements
        .next()
        .expect("relationship nodes should have a left operand");
    let right = elements
        .next()
        .expect("relationship nodes should have a right operand");

    assert!(
        elements.next().is_none(),
        "relationship nodes should have exactly two operands"
    );

    let mut operands = vec![left];
    let mut relationships = vec![(arrow_kind, node.value.subject.unwrap_or_else(empty_subject))];

    match right.value.kind {
        SyntaxKind::Relationship(_) => {
            let (mut child_operands, mut child_relationships) = flatten_relationship_chain(right);
            operands.append(&mut child_operands);
            relationships.append(&mut child_relationships);
        }
        _ => operands.push(right),
    }

    (operands, relationships)
}

fn annotation_subject(annotations: &[Annotation]) -> Subject {
    let mut identity = Symbol(String::new());
    let mut labels = HashSet::new();
    let mut properties = HashMap::new();

    for annotation in annotations {
        match annotation {
            Annotation::Property { key, value } => {
                properties.insert(key.clone(), lower_annotation_value(value));
            }
            Annotation::Identified {
                identity: annotation_identity,
                labels: annotation_labels,
            } => {
                if let Some(annotation_identity) = annotation_identity {
                    identity = annotation_identity.clone();
                }

                for label in annotation_labels {
                    labels.insert(label.clone());
                }
            }
        }
    }

    Subject {
        identity,
        labels,
        properties,
    }
}

fn lower_annotation_value(value: &crate::Value) -> Value {
    match value {
        crate::Value::String(value) => Value::VString(value.clone()),
        crate::Value::Integer(value) => Value::VInteger(*value),
        crate::Value::Decimal(value) => Value::VDecimal(*value),
        crate::Value::Boolean(value) => Value::VBoolean(*value),
        crate::Value::Array(values) => Value::VArray(
            values
                .iter()
                .map(lower_annotation_value)
                .collect::<Vec<_>>(),
        ),
        crate::Value::Range { lower, upper } => Value::VRange(RangeValue {
            lower: Some(*lower as f64),
            upper: Some(*upper as f64),
        }),
        crate::Value::TaggedString { tag, content } => Value::VTaggedString {
            tag: tag.clone(),
            content: content.clone(),
        },
    }
}

fn empty_subject() -> Subject {
    Subject {
        identity: Symbol(String::new()),
        labels: Default::default(),
        properties: Default::default(),
    }
}
