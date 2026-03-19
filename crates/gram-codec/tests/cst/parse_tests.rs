use gram_codec::cst::{Annotation, ArrowKind, SyntaxKind, SyntaxNode};
use gram_codec::{parse_gram_cst, Pattern};

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

#[test]
fn parse_spans_cover_named_nodes_for_diagnostics() {
    let input =
        "// note\n@@meta:Doc @desc(\"route\") [team | (alice), (bob)-->(carol)]\n(commentary)";
    let result = parse_gram_cst(input);

    assert!(result.is_valid());

    let mut nodes = Vec::new();
    collect_nodes(&result.tree, &mut nodes);
    assert!(!nodes.is_empty());

    for node in &nodes {
        let span = &node.value.span;
        assert!(span.start <= span.end, "invalid span ordering: {span:?}");
        assert!(span.end <= input.len(), "span past end of input: {span:?}");

        let fragment = &input[span.start..span.end];
        assert!(
            !fragment.is_empty(),
            "expected non-empty fragment for {:?}",
            node.value.kind
        );

        if let Some(subject) = &node.value.subject {
            if !subject.identity.0.is_empty() {
                assert!(
                    fragment.contains(&subject.identity.0),
                    "fragment '{fragment}' should contain identifier '{}'",
                    subject.identity.0
                );
            }
        }

        match &node.value.kind {
            SyntaxKind::Relationship(ArrowKind::Right) => assert!(fragment.contains("-->")),
            SyntaxKind::Relationship(ArrowKind::Left) => assert!(fragment.contains("<--")),
            SyntaxKind::Relationship(ArrowKind::Bidirectional) => {
                assert!(fragment.contains("<-->"))
            }
            SyntaxKind::Relationship(ArrowKind::Undirected) => assert!(fragment.contains("--")),
            SyntaxKind::Annotated => {
                assert!(fragment.contains("@@meta:Doc"));
                assert!(fragment.contains("@desc(\"route\")"));
            }
            SyntaxKind::Comment => {
                assert_eq!(node.value.text.as_deref(), Some(fragment));
                assert!(fragment.contains("// note"));
            }
            _ => {}
        }
    }
}

#[test]
fn parse_duplicate_identity_demo_returns_two_non_overlapping_spans() {
    let input = "(alice) (alice)";
    let result = parse_gram_cst(input);

    assert!(result.is_valid());

    let mut matches = Vec::new();
    collect_matching_nodes(&result.tree, "alice", &mut matches);
    assert_eq!(matches.len(), 2);

    matches.sort_by_key(|span| span.start);
    assert_eq!(&input[matches[0].start..matches[0].end], "(alice)");
    assert_eq!(&input[matches[1].start..matches[1].end], "(alice)");
    assert!(
        matches[0].end <= matches[1].start,
        "duplicate identity spans should not overlap: {:?}",
        matches
    );
}

fn collect_nodes<'a>(node: &'a Pattern<SyntaxNode>, out: &mut Vec<&'a Pattern<SyntaxNode>>) {
    out.push(node);
    for child in &node.elements {
        collect_nodes(child, out);
    }
}

fn collect_matching_nodes(
    node: &Pattern<SyntaxNode>,
    identity: &str,
    out: &mut Vec<gram_codec::cst::SourceSpan>,
) {
    if matches!(node.value.kind, SyntaxKind::Node)
        && node
            .value
            .subject
            .as_ref()
            .is_some_and(|subject| subject.identity.0 == identity)
    {
        out.push(node.value.span.clone());
    }

    for child in &node.elements {
        collect_matching_nodes(child, identity, out);
    }
}
