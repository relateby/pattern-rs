# Data Model: CST-Preserving Gram Parser

**Branch**: `042-gram-cst-parser` | **Date**: 2026-03-19

## Core Types

### SourceSpan

Locates a syntax node within the original input string using byte offsets.

```
SourceSpan {
    start: usize   // inclusive start byte offset
    end: usize     // exclusive end byte offset
}
```

- Derived from `tree_sitter::Node::start_byte()` and `end_byte()`
- Byte offsets (not char offsets) — correct for multi-byte Unicode
- The slice `&input[span.start..span.end]` reproduces the exact source text of the node

---

### ArrowKind

Discriminates the four relationship directions defined in the gram grammar.

```
ArrowKind {
    Right          // ->, =>, ~>
    Left           // <--, <=, <~
    Bidirectional  // <->, <=>, <~>
    Undirected     // --, ==, ~~
}
```

- Mapped from tree-sitter node kinds: `"right_arrow"`, `"left_arrow"`,
  `"bidirectional_arrow"`, `"undirected_arrow"`
- Only meaningful when `SyntaxKind == Relationship`

---

### SyntaxKind

Discriminates the structural role of a syntax node in the parse tree.

```
SyntaxKind {
    Document                  // Root gram_pattern node
    Node                      // node_pattern: (subject)
    Relationship(ArrowKind)   // relationship_pattern: left -[edge]-> right
    Subject                   // subject_pattern: [subject | elements]
    Annotated                 // annotated_pattern: @key(value) pattern
    Comment                   // comment: // text
}
```

- `Relationship` carries `ArrowKind` inline (single enum variant with data)
- `Comment` nodes carry their full text in `SyntaxNode.text`
- `Annotated` nodes carry annotation properties in `SyntaxNode.annotations`

---

### Annotation

Represents a single annotation entry. The v0.3.4 grammar defines two distinct annotation kinds;
this enum distinguishes them.

```
Annotation {
    Property {
        key:   String
        value: Value
    }
    // @key(value) — property metadata

    Identified {
        identity: Option<Symbol>
        labels:   Vec<String>
    }
    // @@id, @@:label, @@id:label — identity/label metadata (new in v0.3.4)
}
```

- `Property` corresponds to tree-sitter node kind `"property_annotation"` (`@key(value)`)
- `Identified` corresponds to tree-sitter node kind `"identified_annotation"` (`@@...`)
- The old `"annotation"` node kind no longer exists in v0.3.4; `transform.rs` line 438 is stale

---

### SyntaxNode

The payload type for `Pattern<SyntaxNode>`. Carries all syntax-layer information
that the semantic `Subject` type discards.

```
SyntaxNode {
    kind:        SyntaxKind
    subject:     Option<Subject>   // semantic payload (identity, labels, properties)
    span:        SourceSpan        // byte range in original input
    annotations: Vec<Annotation>   // populated for SyntaxKind::Annotated; source order preserved
    text:        Option<String>    // populated for SyntaxKind::Comment
}
```

**Field notes**:
- `subject` is `None` for `Document`, `Comment`, and anonymous nodes
- `annotations` is empty for all kinds except `Annotated`
- `text` is `None` for all kinds except `Comment`
- For `Relationship`, arrow direction is encoded in `SyntaxKind::Relationship(arrow)`,
  not in a separate field
- `annotations` uses `Vec` (not `HashMap`) to preserve source order and accommodate
  `Identified` variants that carry no string key

---

### Pattern\<SyntaxNode\>

The syntax-preserving parse result. Uses the existing generic `Pattern<T>` type
from `pattern-core` with `T = SyntaxNode`.

```
Pattern<SyntaxNode> {
    value:    SyntaxNode
    elements: Vec<Pattern<SyntaxNode>>
}
```

**Structural mapping** (matches pattern-core's arity conventions):
- `Document`: elements = top-level patterns and comment nodes in source order
- `Node` (node_pattern): elements = `[]` (zero elements)
- `Relationship` (relationship_pattern): elements = `[left, right]` (two elements; left_arrow reverses order)
- `Annotated` (annotated_pattern): elements = `[pattern]` (one element)
- `Subject` (subject_pattern): elements = `[e1, e2, ...]` (arbitrary elements)
- `Comment`: elements = `[]` (zero elements; leaf node)

---

### CstParseResult

The return type of `parse_gram_cst()`. Separates the best-effort syntax tree from
error location information.

```
CstParseResult {
    tree:   Pattern<SyntaxNode>  // best-effort full document tree
    errors: Vec<SourceSpan>      // spans of all ERROR nodes in the tree
}
```

- `errors` is empty when the input is fully valid
- `errors` is non-empty when tree-sitter produced one or more ERROR nodes
- Callers that require fully valid input check `result.errors.is_empty()`
- The `tree` is always present, even when `errors` is non-empty

---

## Lowering: Pattern\<SyntaxNode\> → Pattern\<Subject\>

The `lower()` function maps the syntax tree to the semantic form.

**Transformation rules**:
- `Document` root → produces `Vec<Pattern<Subject>>` (drops the document wrapper)
- `Node` → `Pattern::point(syntax_node.subject.unwrap_or_default())`
- `Relationship(Right | Bidirectional | Undirected)` → `Pattern { value: edge_subject, elements: [lower(left), lower(right)] }`
- `Relationship(Left)` → `Pattern { value: edge_subject, elements: [lower(right), lower(left)] }` (reversed)
- `Annotated` → `Pattern { value: annotation_subject, elements: [lower(inner)] }` where `annotation_subject` is built from `Property` annotations only; `Identified` annotations are dropped during lowering (no nom parser counterpart)
- `Subject` → `Pattern { value: subject, elements: elements.map(lower) }`
- `Comment` → **dropped** (not present in lowered output)

**Guarantee**: `lower(parse_gram_cst(s).tree)` equals `parse_gram(s)` for all valid `s`,
modulo source-order comment removal. This is the equivalence invariant tested by `lowering_tests.rs`.

---

## Entity Relationships

```
CstParseResult
  └── tree: Pattern<SyntaxNode>
              ├── value: SyntaxNode
              │     ├── kind: SyntaxKind
              │     │     └── Relationship(ArrowKind)
              │     ├── subject: Option<Subject>      ← from pattern-core
              │     ├── span: SourceSpan
              │     ├── annotations: Vec<Annotation>
              │     │     ├── Property { key, value }
              │     │     └── Identified { identity, labels }
              │     └── text: Option<String>
              └── elements: Vec<Pattern<SyntaxNode>>  (recursive)

  └── errors: Vec<SourceSpan>
```
