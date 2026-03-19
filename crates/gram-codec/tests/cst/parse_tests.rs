use gram_codec::cst::{Annotation, ArrowKind, SyntaxKind};
use gram_codec::parse_gram_cst;

#[test]
fn parse_reproduces_spans_for_top_level_constructs() {
    let input = "(alice)\n[a_team | bob, (carol)]\n@desc(\"historic route\") (dan)";
    let result = parse_gram_cst(input);

    assert!(result.is_valid());
    assert!(matches!(result.tree.value.kind, SyntaxKind::Document));
    assert_eq!(result.tree.elements.len(), 3);

    let spans: Vec<&str> = result
        .tree
        .elements
        .iter()
        .map(|element| &input[element.value.span.start..element.value.span.end])
        .collect();
    assert_eq!(
        spans,
        vec![
            "(alice)",
            "[a_team | bob, (carol)]",
            "@desc(\"historic route\") (dan)"
        ]
    );

    assert!(matches!(
        result.tree.elements[0].value.kind,
        SyntaxKind::Node
    ));
    assert!(matches!(
        result.tree.elements[1].value.kind,
        SyntaxKind::Subject
    ));
    assert!(matches!(
        result.tree.elements[2].value.kind,
        SyntaxKind::Annotated
    ));

    let annotations = &result.tree.elements[2].value.annotations;
    assert_eq!(annotations.len(), 1);
    match &annotations[0] {
        Annotation::Property { key, value } => {
            assert_eq!(key, "desc");
            assert_eq!(
                value,
                &gram_codec::Value::String("historic route".to_string())
            );
        }
        other => panic!("expected property annotation, got {other:?}"),
    }
}

#[test]
fn parse_maps_all_arrow_kinds() {
    let inputs = [
        ("(a)-->(b)", ArrowKind::Right),
        ("(a)<--(b)", ArrowKind::Left),
        ("(a)<-->(b)", ArrowKind::Bidirectional),
        ("(a)--(b)", ArrowKind::Undirected),
    ];

    for (input, expected_kind) in inputs {
        let result = parse_gram_cst(input);
        assert!(result.is_valid(), "expected valid CST parse for {input}");
        assert_eq!(result.tree.elements.len(), 1);

        match &result.tree.elements[0].value.kind {
            SyntaxKind::Relationship(actual_kind) => assert_eq!(actual_kind, &expected_kind),
            other => panic!("expected relationship node, got {other:?}"),
        }

        let span = &result.tree.elements[0].value.span;
        assert_eq!(&input[span.start..span.end], input);
    }
}

#[test]
fn parse_preserves_identified_and_property_annotations() {
    let input = "@@p:Person @desc(a) (alice)";
    let result = parse_gram_cst(input);

    assert!(result.is_valid());
    let annotated = &result.tree.elements[0];
    assert!(matches!(annotated.value.kind, SyntaxKind::Annotated));
    assert_eq!(annotated.value.annotations.len(), 2);

    match &annotated.value.annotations[0] {
        Annotation::Identified { identity, labels } => {
            assert_eq!(identity.as_ref().map(|id| id.0.as_str()), Some("p"));
            assert_eq!(labels, &vec!["Person".to_string()]);
        }
        other => panic!("expected identified annotation, got {other:?}"),
    }

    match &annotated.value.annotations[1] {
        Annotation::Property { key, value } => {
            assert_eq!(key, "desc");
            assert_eq!(value, &gram_codec::Value::String("a".to_string()));
        }
        other => panic!("expected property annotation, got {other:?}"),
    }
}
