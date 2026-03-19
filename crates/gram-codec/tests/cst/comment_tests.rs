use gram_codec::cst::{ArrowKind, SyntaxKind};
use gram_codec::parse_gram_cst;

#[test]
fn comments_are_interleaved_in_source_order() {
    let input = "// heading\n(alice)\n(alice)-->(bob)  // edge comment\n// footer";
    let result = parse_gram_cst(input);

    assert!(result.is_valid());
    assert_eq!(result.tree.elements.len(), 5);

    let kinds: Vec<&SyntaxKind> = result
        .tree
        .elements
        .iter()
        .map(|node| &node.value.kind)
        .collect();
    assert!(matches!(kinds[0], SyntaxKind::Comment));
    assert!(matches!(kinds[1], SyntaxKind::Node));
    assert!(matches!(
        kinds[2],
        SyntaxKind::Relationship(ArrowKind::Right)
    ));
    assert!(matches!(kinds[3], SyntaxKind::Comment));
    assert!(matches!(kinds[4], SyntaxKind::Comment));

    let texts: Vec<&str> = result
        .tree
        .elements
        .iter()
        .filter_map(|node| node.value.text.as_deref())
        .collect();
    assert_eq!(texts, vec!["// heading", "// edge comment", "// footer"]);
}

#[test]
fn comment_spans_are_byte_accurate() {
    let input = "(hello)  // trailing comment";
    let result = parse_gram_cst(input);

    assert!(result.is_valid());
    let comment = result
        .tree
        .elements
        .iter()
        .find(|node| matches!(node.value.kind, SyntaxKind::Comment))
        .expect("expected trailing comment node");

    let span = &comment.value.span;
    assert_eq!(comment.value.text.as_deref(), Some("// trailing comment"));
    assert_eq!(&input[span.start..span.end], "// trailing comment");
}
