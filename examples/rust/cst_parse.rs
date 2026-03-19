//! Minimal CST parsing example for relateby-gram.
//!
//! Run with:
//! `cargo run -p relateby-gram --features cst --example cst_parse`

use gram_codec::cst::{ArrowKind, SyntaxKind, SyntaxNode};
use gram_codec::{lower, parse_gram_cst, Pattern};

fn main() {
    let input = "// greeting\n(alice)->(bob)";
    let result = parse_gram_cst(input);

    println!("Input:\n{input}\n");
    println!("Valid: {}", result.is_valid());
    if !result.errors.is_empty() {
        println!("Errors: {:?}", result.errors);
    }

    println!("\nCST:");
    print_tree(&result.tree, 0);

    let lowered = lower(result.tree);
    println!("\nLowered patterns ({}):", lowered.len());
    for (index, pattern) in lowered.iter().enumerate() {
        println!("  {index}: {pattern:#?}");
    }
}

fn print_tree(tree: &Pattern<SyntaxNode>, indent: usize) {
    let padding = "  ".repeat(indent);
    let kind = format_kind(&tree.value.kind);
    let span = &tree.value.span;

    print!("{padding}{kind} [{}..{}]", span.start, span.end);

    if let Some(subject) = &tree.value.subject {
        if !subject.identity.0.is_empty() {
            print!(" id={}", subject.identity.0);
        }
    }

    if let Some(text) = &tree.value.text {
        print!(" text={text:?}");
    }

    println!();

    for element in &tree.elements {
        print_tree(element, indent + 1);
    }
}

fn format_kind(kind: &SyntaxKind) -> &'static str {
    match kind {
        SyntaxKind::Document => "Document",
        SyntaxKind::Node => "Node",
        SyntaxKind::Relationship(ArrowKind::Right) => "Relationship(Right)",
        SyntaxKind::Relationship(ArrowKind::Left) => "Relationship(Left)",
        SyntaxKind::Relationship(ArrowKind::Bidirectional) => "Relationship(Bidirectional)",
        SyntaxKind::Relationship(ArrowKind::Undirected) => "Relationship(Undirected)",
        SyntaxKind::Subject => "Subject",
        SyntaxKind::Annotated => "Annotated",
        SyntaxKind::Comment => "Comment",
    }
}
