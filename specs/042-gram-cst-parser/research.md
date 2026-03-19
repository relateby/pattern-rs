# Research: CST-Preserving Gram Parser

**Branch**: `042-gram-cst-parser` | **Date**: 2026-03-19

## Decision 1: tree-sitter Crate Version

**Decision**: Use `tree-sitter = "0.25"` and `tree-sitter-language = "0.1"`.

**Rationale**: The updated `tree-sitter-gram` submodule (v0.3.4) declares `tree-sitter-language = "0.1"`
as a runtime dependency and `tree-sitter = "0.25"` as a dev-dependency. The `LANGUAGE` constant is
now a `LanguageFn` from the `tree_sitter_language` crate, not a bare `tree_sitter::Language`. Usage
pattern: `parser.set_language(&tree_sitter_gram::LANGUAGE.into())`. The tree traversal API
(`node.kind()`, `child_by_field_name`, `node.walk()`, etc.) is unchanged between 0.23 and 0.25.

gram-codec's `cst` feature therefore needs two new dependencies:
```toml
tree-sitter = { version = "0.25", optional = true }
tree-sitter-gram = { path = "../../external/tree-sitter-gram", optional = true }
# tree-sitter-language is a transitive dep via tree-sitter-gram; no explicit declaration needed
```

**Alternatives considered**: 0.23 (previously planned, incompatible with v0.3.4 bindings).

---

## Decision 2: tree-sitter-gram Dependency

**Decision**: Add as a path dependency pointing at the existing submodule's Rust bindings:
`tree-sitter-gram = { path = "../../external/tree-sitter-gram/bindings/rust" }` (relative to
`crates/gram-codec/Cargo.toml`).

**Rationale**: The `external/tree-sitter-gram` submodule is already present in the repository
and is actively maintained as the authoritative grammar. Using it directly avoids a separate
crates.io publish step and keeps the C parser source (grammar.js → parser.c) in sync with what
the Rust bindings expose.

**Alternatives considered**: Publishing tree-sitter-gram to crates.io and pinning a version.
Rejected because: the submodule can be updated in-place, and a crates.io publish adds external
coordination overhead not justified for an architectural spike.

---

## Decision 3: Feature Flag Name

**Decision**: Name the Cargo feature `cst`.

**Rationale**: Short, clear, and consistent with the existing feature naming pattern (`wasm`,
`python`). The flag gates both the tree-sitter dependency and the `cst` module, preventing the
C runtime from appearing in non-CST builds.

**Alternatives considered**: `tree-sitter` (too implementation-specific; callers shouldn't need
to know the backend), `syntax` (too generic). `cst` matches the domain vocabulary used in the
spec and proposal.

---

## Decision 4: Comment Node Behavior in tree-sitter

**Decision**: Comments are emitted by tree-sitter as named nodes with kind `"comment"` at the
document level, interleaved with pattern nodes in source order. They appear because `comment` is
listed in the grammar's `extras` rule — tree-sitter attaches extras as siblings rather than
suppressing them.

**Evidence**: `grammar.js` lines 4–7:
```js
extras: ($) => [/\s/, $.comment],
```
`comment: ($) => token(seq("//", /.*/))` — single-line `//` comments.

**Implication for implementation**: The CST parser's document-level loop iterates all named
children of the root `gram_pattern` node. When `child.kind() == "comment"`, produce a
`Pattern::point(SyntaxNode { kind: SyntaxKind::Comment, text: Some(comment_text), span, .. })`.
This produces correct source-order interleaving with zero heuristics.

**Alternatives considered**: Collecting comments into a separate list attached to the document
root. Rejected per spec clarification Q4 (Option A: source-order interleaving).

---

## Decision 5: Error Recovery API

**Decision**: Expose a `CstParseResult` struct:
```rust
pub struct CstParseResult {
    pub tree: Pattern<SyntaxNode>,   // Best-effort partial tree
    pub errors: Vec<SourceSpan>,     // Spans of all ERROR nodes
}
```

**Rationale**: tree-sitter always produces a tree, even for syntactically invalid input. Error
nodes (kind `"ERROR"`) are reachable via normal tree traversal. A post-parse scan collects all
error node spans into `errors`. Callers that only accept valid input check `errors.is_empty()`.
Tooling like `pato lint` can work with the partial tree while surfacing diagnostics for the error
spans.

**Evidence**: `tree_sitter::Node::is_error()` and `tree_sitter::Node::has_error()` methods exist
in the tree-sitter Rust API. The existing `transform.rs` already traverses nodes by kind.

**Alternatives considered**:
- Return `Result<Pattern<SyntaxNode>, ParseError>` (fail fast). Rejected per spec clarification Q3.
- Silent partial tree (no error list). Rejected per spec clarification Q3.

---

## Decision 6: Pattern<SyntaxNode> Feasibility

**Decision**: `Pattern<SyntaxNode>` is fully feasible. The existing `Pattern<T>` type in
`pattern-core` is generic over `T` with no constraints beyond `Clone`. `SyntaxNode` can be any
struct that implements `Clone`. No changes to `pattern-core` are needed.

**Evidence**: `transform.rs` already demonstrates that tree-sitter node traversal maps cleanly
onto `Pattern { value, elements }` — the CST variant simply replaces `Subject` with `SyntaxNode`
as the value type and retains the same structural mapping.

**Implication**: The lowering function `lower: Pattern<SyntaxNode> -> Pattern<Subject>` is
expressible as a `Pattern::map` call (plus comment-filtering at the document level). This confirms
SC-005.

---

## Decision 7: Disposition of Existing transform.rs

**Decision**: Retain `transform.rs` unchanged but unreferenced during this feature. It serves as
implementation reference material. Removal is deferred to a follow-on cleanup.

**Rationale**: The file is already commented out of `lib.rs` (lines 47–50). Removing it in this
branch would mix cleanup with the architectural spike, obscuring the scope of changes. Its
presence does not affect compilation.

---

## Decision 8: Arrow Kind Representation in SyntaxNode

**Decision**: `SyntaxKind::Relationship` carries an `arrow: ArrowKind` field. `ArrowKind` is an
enum:
```rust
pub enum ArrowKind {
    Right,           // ->, =>, ~>
    Left,            // <--, <=, <~
    Bidirectional,   // <->, <=>, <~>
    Undirected,      // --, ==, ~~
}
```

**Rationale**: The grammar defines four distinct `_relationship_kind` variants
(`right_arrow`, `left_arrow`, `bidirectional_arrow`, `undirected_arrow`). The tree-sitter node
kind string for the arrow child gives this directly (`child.kind()`). The existing `transform.rs`
already handles these four cases in `handle_arrow_type()`.

**Implication for lowering**: Left arrows reverse element order in `Pattern<Subject>` (first=right,
second=left). This behavior from `transform.rs` is preserved in the lowering path.

---

## Decision 9: Annotation Grammar Split (v0.3.4 Breaking Change)

**Decision**: Represent the two annotation kinds as distinct variants in a new `Annotation` enum
stored on `SyntaxNode`, replacing the previous `HashMap<String, Value>` field.

**Context**: The v0.3.4 grammar split the old single `annotation` node kind into two:

| Node kind | Syntax | Semantics |
|---|---|---|
| `property_annotation` | `@key(value)` | Key/value property metadata |
| `identified_annotation` | `@@id`, `@@:label`, `@@id:label` | Identity and/or label metadata |

The old `annotation` node kind **no longer exists**. The existing `transform.rs` line 438
(`child.kind() == "annotation"`) will silently match nothing against v0.3.4.

**New type**:
```rust
pub enum Annotation {
    Property { key: String, value: Value },
    Identified { identity: Option<Symbol>, labels: Vec<String> },
}
```

`SyntaxNode.annotations` becomes `Vec<Annotation>` (ordered, preserving source order).

**Rationale**: A `HashMap<String, Value>` can only represent `property_annotation` entries. The
new `identified_annotation` carries identity and labels — a structurally different payload that
requires a typed enum variant. An ordered `Vec` also preserves the original annotation sequence,
which a map does not.

**Lowering behaviour for `identified_annotation`**: The nom parser has no `@@` syntax support yet,
so `identified_annotation` nodes have no semantic counterpart in `Pattern<Subject>`. During
lowering, `identified_annotation` entries are dropped (same as comment nodes). The lowering
equivalence guarantee (SC-002) is scoped to gram input that the current nom parser accepts; `@@`
syntax is outside that scope until the nom parser is extended.

**Implication for `transform.rs`**: The dormant `extract_annotation_subject` function must be
rewritten for v0.3.4. It currently iterates children looking for `"annotation"` — this must be
updated to match `"property_annotation"` and `"identified_annotation"` separately.

**Alternatives considered**: Keeping `HashMap<String, Value>` and silently discarding
`identified_annotation`. Rejected because it would lose information that gramdoc tooling
specifically needs, and the data model should be accurate to what the grammar actually produces.
