# Public API Contract: CST-Preserving Gram Parser

**Branch**: `042-gram-cst-parser` | **Date**: 2026-03-19
**Crate**: `gram-codec` | **Feature flag**: `cst` (off by default)

---

## Feature Flag

The CST parser is gated behind the `cst` Cargo feature:

```toml
# Cargo.toml (consumer)
gram-codec = { version = "...", features = ["cst"] }
```

When `cst` is not enabled, none of the types or functions below are compiled. The existing
`parse_gram`, `parse_gram_with_header`, and `parse_to_ast` APIs are unaffected by this flag.

---

## Types

### `SourceSpan`

```rust
#[cfg(feature = "cst")]
pub struct SourceSpan {
    pub start: usize,  // inclusive start byte offset into original input
    pub end: usize,    // exclusive end byte offset into original input
}
```

### `ArrowKind`

```rust
#[cfg(feature = "cst")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArrowKind {
    Right,
    Left,
    Bidirectional,
    Undirected,
}
```

### `SyntaxKind`

```rust
#[cfg(feature = "cst")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SyntaxKind {
    Document,
    Node,
    Relationship(ArrowKind),
    Subject,
    Annotated,
    Comment,
}
```

### `Annotation`

```rust
#[cfg(feature = "cst")]
#[derive(Clone, Debug)]
pub enum Annotation {
    /// @key(value) — from tree-sitter node kind "property_annotation"
    Property { key: String, value: Value },
    /// @@id, @@:label, @@id:label — from tree-sitter node kind "identified_annotation" (v0.3.4+)
    Identified { identity: Option<Symbol>, labels: Vec<String> },
}
```

### `SyntaxNode`

```rust
#[cfg(feature = "cst")]
#[derive(Clone, Debug)]
pub struct SyntaxNode {
    pub kind: SyntaxKind,
    pub subject: Option<Subject>,       // identity, labels, properties
    pub span: SourceSpan,
    pub annotations: Vec<Annotation>,  // populated for Annotated nodes; source order preserved
    pub text: Option<String>,           // populated for Comment nodes
}
```

### `CstParseResult`

```rust
#[cfg(feature = "cst")]
pub struct CstParseResult {
    pub tree: Pattern<SyntaxNode>,
    pub errors: Vec<SourceSpan>,
}

impl CstParseResult {
    /// Returns true if the input was fully valid (no parse errors).
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}
```

---

## Functions

### `parse_gram_cst`

Parse a gram notation string into a syntax-preserving tree.

```rust
#[cfg(feature = "cst")]
pub fn parse_gram_cst(input: &str) -> CstParseResult
```

**Behaviour**:
- Always returns a `CstParseResult` — never panics or returns `Err`
- `result.tree` is the best-effort document tree for `input`, including comment nodes
- `result.errors` is empty when input is fully valid; contains error spans otherwise
- The root of `result.tree` has `SyntaxKind::Document`
- Top-level children are interleaved patterns and comment nodes in source order

**Guarantees**:
- Every node in `result.tree` carries an accurate `SourceSpan`
- Arrow kinds are preserved exactly as written in the source
- Annotation key-value pairs are stored in `SyntaxNode.annotations`
- Comment text (including the `//` prefix) is stored in `SyntaxNode.text`

---

### `lower`

Lower a syntax-preserving tree to the semantic pattern form.

```rust
#[cfg(feature = "cst")]
pub fn lower(tree: Pattern<SyntaxNode>) -> Vec<Pattern<Subject>>
```

**Behaviour**:
- Recursively maps `Pattern<SyntaxNode>` to `Pattern<Subject>`
- Comment nodes are silently dropped
- For `Left` arrow relationships, element order is reversed to match nom parser convention
- The document root wrapper is unwrapped; the return value is `Vec<Pattern<Subject>>`
- Output is structurally equivalent to `parse_gram(input)` for the same valid `input`

---

## Existing API (unchanged)

The following functions remain available regardless of whether the `cst` feature is enabled.
Their behaviour is not modified by this feature.

```rust
pub fn parse_gram(input: &str) -> Result<Vec<Pattern<Subject>>, ParseError>
pub fn parse_gram_with_header(input: &str) -> Result<(Option<Record>, Vec<Pattern<Subject>>), ParseError>
pub fn parse_to_ast(input: &str) -> Result<AstPattern, ParseError>
```

---

## Usage Example

```rust
use gram_codec::{parse_gram_cst, lower};
use gram_codec::cst::{SyntaxKind, ArrowKind, Annotation};

let input = r#"
  // a comment
  @type("node") @@alice:Person (alice)
"#;

let result = parse_gram_cst(input);
assert!(result.is_valid());

// Walk top-level children (comment, then annotated pattern)
for child in &result.tree.elements {
    match &child.value.kind {
        SyntaxKind::Comment => {
            println!("comment: {}", child.value.text.as_deref().unwrap_or(""));
        }
        SyntaxKind::Annotated => {
            for ann in &child.value.annotations {
                match ann {
                    Annotation::Property { key, value } => println!("@{key}({value:?})"),
                    Annotation::Identified { identity, labels } => println!("@@{identity:?}:{labels:?}"),
                }
            }
        }
        _ => {}
    }
}

// Lower to semantic form (comments and identified annotations dropped)
let patterns = lower(result.tree);
```
