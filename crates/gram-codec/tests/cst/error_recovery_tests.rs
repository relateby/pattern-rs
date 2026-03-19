use gram_codec::cst::SyntaxKind;
use gram_codec::parse_gram_cst;

#[test]
fn malformed_input_returns_partial_tree_and_errors() {
    let input = "@@ (alice)";
    let result = parse_gram_cst(input);

    assert!(!result.is_valid());
    assert!(!result.errors.is_empty());
    assert!(matches!(result.tree.value.kind, SyntaxKind::Document));
    assert!(
        result
            .tree
            .elements
            .iter()
            .any(|node| matches!(node.value.kind, SyntaxKind::Node)),
        "expected partial CST tree to retain the recovered node",
    );
}
