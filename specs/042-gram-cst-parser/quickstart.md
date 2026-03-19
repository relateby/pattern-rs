# Quickstart: CST-Preserving Gram Parser

**Branch**: `042-gram-cst-parser`

## Enable the Feature

Add the `cst` feature to your `gram-codec` dependency:

```toml
[dependencies]
gram-codec = { path = "crates/gram-codec", features = ["cst"] }
```

Or when running tests locally:

```bash
cargo test -p gram-codec --features cst
```

## Parse Gram to Syntax Tree

```rust
use gram_codec::parse_gram_cst;

let result = parse_gram_cst("(alice)->(bob)");

if result.is_valid() {
    println!("parse succeeded, {} top-level nodes", result.tree.elements.len());
} else {
    for span in &result.errors {
        println!("error at bytes {}..{}", span.start, span.end);
    }
}
```

## Inspect Preserved Information

```rust
use gram_codec::cst::{SyntaxKind, ArrowKind};
use gram_codec::parse_gram_cst;

let input = r#"(a)-[e:KNOWS]->(b)"#;
let result = parse_gram_cst(input);

// Walk the document's top-level children
for child in &result.tree.elements {
    match &child.value.kind {
        SyntaxKind::Relationship(ArrowKind::Right) => {
            let span = &child.value.span;
            println!("right-directed relationship at bytes {}..{}", span.start, span.end);
            // elements[0] = left node, elements[1] = right node
        }
        SyntaxKind::Comment => {
            println!("comment: {}", child.value.text.as_deref().unwrap_or(""));
        }
        _ => {}
    }
}
```

## Lower to Semantic Pattern

```rust
use gram_codec::{parse_gram_cst, lower};

let result = parse_gram_cst("(a)->(b)");
let patterns = lower(result.tree);
// patterns is Vec<Pattern<Subject>>, identical to parse_gram("(a)->(b)")
```

## Running Tests

```bash
# All CST tests
cargo test -p gram-codec --features cst cst::

# Lowering equivalence tests
cargo test -p gram-codec --features cst lowering

# With snapshot review (insta)
cargo insta review
```
