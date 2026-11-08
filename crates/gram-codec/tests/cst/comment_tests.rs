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

#[test]
fn doc_comment_is_kept_as_doc_comment_node() {
    let input = "/**\n * A human individual.\n *\n * @property name The full name.\n */\n(Person {name: \"STRING!\"})";
    let result = parse_gram_cst(input);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let doc = result
        .tree
        .elements
        .iter()
        .find(|n| matches!(n.value.kind, SyntaxKind::DocComment))
        .expect("doc comment node");
    assert!(doc.value.text.as_deref().unwrap().starts_with("/**"));
}

fn nested_docs(input: &str) -> Vec<Vec<String>> {
    let result = parse_gram_cst(input);
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    fn walk(p: &gram_codec::Pattern<gram_codec::cst::SyntaxNode>, out: &mut Vec<Vec<String>>) {
        if !p.value.doc_comments.is_empty() {
            out.push(
                p.value
                    .doc_comments
                    .iter()
                    .map(|d| d.text.clone())
                    .collect(),
            );
        }
        p.elements.iter().for_each(|e| walk(e, out));
    }
    let mut out = Vec::new();
    walk(&result.tree, &mut out);
    out
}

#[test]
fn doc_comment_inside_record_is_captured_on_node() {
    let docs = nested_docs("(Person {\n  /** The full name. */\n  name: \"x\"\n})");
    assert_eq!(docs, vec![vec!["/** The full name. */".to_string()]]);
}

#[test]
fn doc_comments_inside_subject_pattern_are_captured() {
    let docs = nested_docs(
        "[:French {\n  /** nom */\n  name: \"nom\"\n} |\n  /** the element */\n  Person ]",
    );
    assert_eq!(
        docs,
        vec![vec![
            "/** nom */".to_string(),
            "/** the element */".to_string()
        ]]
    );
}

#[test]
fn doc_comment_inside_relationship_record_is_captured() {
    let docs = nested_docs("(a)-[:KNOWS {\n  /** since when */\n  since: 2020\n}]->(b)");
    assert_eq!(docs, vec![vec!["/** since when */".to_string()]]);
}

#[test]
fn doc_comment_between_annotation_and_element_is_captured() {
    let docs = nested_docs("@key(id)\n/** the node */\n(a)");
    assert_eq!(docs, vec![vec!["/** the node */".to_string()]]);
}

#[test]
fn nested_doc_comments_do_not_leak_to_parent_patterns() {
    let docs = nested_docs("[outer | (a {\n  /** on a */\n  x: 1\n}), (b)]");
    assert_eq!(docs, vec![vec!["/** on a */".to_string()]]);
}

#[test]
fn doc_comment_inside_header_record_is_captured_on_document() {
    let result = parse_gram_cst("{ /** documents version */ version: 1 }\n(a)");
    assert!(result.errors.is_empty(), "{:?}", result.errors);
    let docs: Vec<_> = result
        .tree
        .value
        .doc_comments
        .iter()
        .map(|d| d.text.as_str())
        .collect();
    assert_eq!(docs, ["/** documents version */"]);
}
