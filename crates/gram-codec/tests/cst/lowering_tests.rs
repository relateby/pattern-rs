use crate::corpus::CorpusTestSuite;
use gram_codec::cst::{Annotation, SyntaxKind, SyntaxNode};
use gram_codec::{lower, parse_gram, parse_gram_cst, Pattern, Subject};
use pattern_core::{Symbol, Value};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[test]
fn lowering_matches_nom_parser_for_all_non_error_corpus_fixtures() {
    let corpus_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/tree-sitter-gram/test/corpus");
    let suite = CorpusTestSuite::load(&corpus_path).expect("failed to load corpus fixtures");

    for test in suite.tests {
        if test.is_error {
            continue;
        }

        let lowered = lower(parse_gram_cst(&test.input).tree);
        let parsed = parse_gram(&test.input).unwrap_or_else(|err| {
            panic!(
                "nom parser failed for {} ({}): {err}",
                test.name, test.input
            )
        });

        assert_eq!(lowered, parsed, "fixture mismatch for {}", test.name);
    }
}

#[test]
fn lowering_covers_all_relationship_arrow_kinds() {
    let inputs = [
        ("(a)-->(b)", vec!["a", "b"]),
        ("(a)<--(b)", vec!["b", "a"]),
        ("(a)<-->(b)", vec!["a", "b"]),
        ("(a)--(b)", vec!["a", "b"]),
    ];

    for (input, expected_order) in inputs {
        let lowered = lower(parse_gram_cst(input).tree);
        assert_eq!(lowered.len(), 1);
        let pattern = &lowered[0];
        let actual: Vec<&str> = pattern
            .elements
            .iter()
            .map(|element| element.value.identity.0.as_str())
            .collect();
        assert_eq!(actual, expected_order, "unexpected order for {input}");
    }
}

#[test]
fn lowering_preserves_header_record_and_subject_patterns() {
    let input = "{ name: \"Graph\" }\n[group | (alice), (bob)]";
    let lowered = lower(parse_gram_cst(input).tree);

    assert_eq!(lowered.len(), 2);
    assert_eq!(
        lowered[0].value.properties["name"],
        Value::VString("Graph".to_string())
    );
    assert_eq!(lowered[1].value.identity.0, "group");
    assert_eq!(lowered[1].elements.len(), 2);
    assert_eq!(lowered[1].elements[0].value.identity.0, "alice");
    assert_eq!(lowered[1].elements[1].value.identity.0, "bob");
}

#[test]
fn lowering_builds_annotation_wrapper_from_property_annotations_only() {
    let tree = Pattern::pattern(
        SyntaxNode {
            kind: SyntaxKind::Document,
            subject: None,
            span: span(),
            annotations: vec![],
            text: None,
        },
        vec![Pattern::pattern(
            SyntaxNode {
                kind: SyntaxKind::Annotated,
                subject: None,
                span: span(),
                annotations: vec![
                    Annotation::Identified {
                        identity: Some(Symbol("meta".to_string())),
                        labels: vec!["Label".to_string()],
                    },
                    Annotation::Property {
                        key: "desc".to_string(),
                        value: gram_codec::Value::String("historic".to_string()),
                    },
                ],
                text: None,
            },
            vec![Pattern::point(SyntaxNode {
                kind: SyntaxKind::Node,
                subject: Some(subject("alice")),
                span: span(),
                annotations: vec![],
                text: None,
            })],
        )],
    );

    let lowered = lower(tree);
    assert_eq!(lowered.len(), 1);
    let annotated = &lowered[0];
    assert_eq!(
        annotated.value.properties.get("desc"),
        Some(&Value::VString("historic".to_string()))
    );
    assert_eq!(annotated.value.identity.0, "meta");
    assert!(annotated.value.labels.contains("Label"));
    assert_eq!(annotated.elements.len(), 1);
    assert_eq!(annotated.elements[0].value.identity.0, "alice");
}

#[test]
fn pattern_traversal_operations_work_on_cst_trees() {
    let input = "@@meta:Doc @desc(\"route\") [team | (alice), (bob)-->(carol)]";
    let tree = parse_gram_cst(input).tree;

    let count = tree.fold(0, |acc, _| acc + 1);
    assert_eq!(count, 7);

    let kinds = tree.clone().map(|node| node.kind.clone());
    assert!(matches!(kinds.value, SyntaxKind::Document));
    assert_eq!(kinds.elements.len(), 1);
    assert!(matches!(kinds.elements[0].value, SyntaxKind::Annotated));
    assert_eq!(kinds.elements[0].elements.len(), 1);
    assert!(matches!(
        kinds.elements[0].elements[0].value,
        SyntaxKind::Subject
    ));
    assert_eq!(kinds.elements[0].elements[0].elements.len(), 2);
    assert!(matches!(
        kinds.elements[0].elements[0].elements[0].value,
        SyntaxKind::Node
    ));
    assert!(matches!(
        kinds.elements[0].elements[0].elements[1].value,
        SyntaxKind::Relationship(_)
    ));
}

#[test]
fn map_style_lowering_matches_lower_tree_structure() {
    let input = "[team | (alice), (bob)-->(carol)]";
    let tree = parse_gram_cst(input).tree;
    let mapped = tree.clone().map(lower_value);

    assert_eq!(mapped.value.identity.0, "");
    assert_eq!(mapped.elements.len(), 1);

    let expected_lowered = lower(tree);
    assert_eq!(expected_lowered.len(), 1);

    let expected = Pattern::pattern(empty_subject(), expected_lowered);
    assert_eq!(mapped, expected);
}

fn subject(identity: &str) -> Subject {
    Subject {
        identity: Symbol(identity.to_string()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    }
}

fn span() -> gram_codec::cst::SourceSpan {
    gram_codec::cst::SourceSpan { start: 0, end: 0 }
}

fn lower_value(node: &SyntaxNode) -> Subject {
    node.subject.clone().unwrap_or_else(empty_subject)
}

fn empty_subject() -> Subject {
    Subject {
        identity: Symbol(String::new()),
        labels: HashSet::new(),
        properties: HashMap::new(),
    }
}
