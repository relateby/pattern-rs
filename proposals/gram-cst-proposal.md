# Proposal: CST-Preserving Gram Parser

**Status:** Draft  
**Scope:** Parser architecture exploration and follow-on implementation planning  
**Relationship to current work:** Pause `041-pato-cli`; pursue this as a separate feature branch, then return

---

## 1. Purpose

This proposal recommends a separate feature effort to add a CST-preserving gram parser,
likely built around `tree-sitter-gram`, while keeping the current `pato` work paused but
intact.

The immediate trigger for this proposal came from implementing `pato lint` on top of the
current parser pipeline. That work was productive, but it also exposed a deeper question:

> Are `Pattern`s not just the semantic result of gram parsing, but the right substrate for
> syntax-aware work as well?

The current answer appears to be: possibly yes, but not with the current parser output
alone.

---

## 2. Motivation

### 2.1 `pato` exposed a parser-layer limitation

The current `pato lint` implementation works on `Vec<Pattern<Subject>>`, which is a strong
shape for semantic analysis. It supports traversal, structure-aware checks, and downstream
transforms well.

However, while experimenting with diagnostics and output formats, several limitations became
clear:

- Source-sensitive diagnostics need better location fidelity than semantic reconstruction can
  reliably provide.
- Some syntax distinctions matter to tooling even when they collapse to the same semantic
  pattern.
- The current parser drops annotations entirely.
- The current output experiments raised broader questions about isomorphism between semantic
  structures, gram, and JSON.

These are parser architecture questions, not just CLI questions.

### 2.2 We want to evaluate Pattern as a working representation

This exploration is not only about `pato lint`. It is also about whether `Pattern` can serve
as a general working representation for:

- semantic analysis
- syntax-aware transforms
- linting and formatting
- downstream machine processing
- possible generic transforms between gram and JSON

The current parser only answers the semantic part of that question.

### 2.3 Tree-sitter S-expressions look unusually compatible with `Pattern`

The `tree-sitter-gram` grammar and `gram-lint -t` output suggest that gram's syntax tree is
already remarkably close to the `Pattern` abstraction.

For example, `gram-lint -t data/hello.gram` produces:

```text
(gram_pattern (relationship_pattern left: (node_pattern identifier: (symbol)) kind: (right_arrow) right: (node_pattern identifier: (symbol))))
```

And `external/tree-sitter-gram/README.md` explicitly describes the pattern correspondence:

- node pattern = 0 elements
- annotated pattern = 1 element
- relationship pattern = 2 elements
- subject pattern = arbitrary elements

This is not a typical CST that obviously wants a completely separate tree abstraction. It
looks much more like a syntax-preserving variant of `Pattern`.

---

## 3. Observations From Current Work

### 3.1 Current parser outputs

Today the active parser provides:

- `parse_gram(input) -> Result<Vec<Pattern<Subject>>, ParseError>`
- `parse_gram_with_header(input) -> Result<(Option<Record>, Vec<Pattern<Subject>>), ParseError>`
- `parse_to_ast(input) -> Result<AstPattern, ParseError>`

This gives us:

- a semantic recursive structure (`Pattern<Subject>`)
- a header-aware convenience wrapper
- a JSON-friendly object representation (`AstPattern`)

But it does **not** give us an exposed CST.

### 3.2 Annotations are not preserved

The active nom parser currently parses annotated syntax but drops the annotation content:

- annotations are recognized
- annotation metadata is not stored in the returned `Pattern<Subject>`

This is a strong signal that the current parser output is semantic, not syntax-preserving.

### 3.3 Span support exists, but only internally

The nom parser has location/span utilities and a `with_span` combinator. That means the code
already has some of the ingredients for syntax-preserving parsing, but those spans are not
part of the public parsed tree.

### 3.4 There is inactive tree-sitter transformation code

The repository still contains older tree-sitter transformation code in
`crates/gram-codec/src/transform.rs`, but it is not wired into the active parser path.

This means the repo already contains:

- the `tree-sitter-gram` grammar as a submodule
- prior work on CST-to-pattern transformation
- tests that compare semantic parser output to tree-sitter S-expressions

So this proposal is not introducing a foreign concept. It is reviving and reframing an
existing direction.

### 3.5 The validator already treats tree-sitter and `Pattern` as structurally related

`crates/gram-codec/tests/corpus/validator.rs` compares nom parser output against tree-sitter
S-expressions by pattern structure, including arity-based pattern typing and file-level
wrapping behavior.

That validator is effectively evidence that the project already treats tree-sitter shape and
`Pattern<Subject>` shape as closely aligned.

---

## 4. Core Question

Should a CST-preserving gram parser produce:

1. a separate traditional CST tree type, later lowered to `Pattern<Subject>`, or
2. a syntax-aware pattern form such as `Pattern<SyntaxNode>`?

This proposal does not fully decide that question, but it argues that option 2 is
particularly worth exploring.

---

## 5. Working Hypothesis

The most promising architecture appears to be a layered model:

- **Syntax-preserving layer:** `Pattern<SyntaxNode>` or an equivalent syntax-aware pattern tree
- **Semantic layer:** `Pattern<Subject>`
- **JSON/object layer:** `AstPattern` or a revised object model derived from one of the above

Under this model:

- tree-sitter parsing would produce or support construction of a syntax-preserving pattern tree
- syntax lowering would map that tree to `Pattern<Subject>`
- semantic tooling such as current `pato lint` could continue to operate on `Pattern<Subject>`
- syntax-sensitive tooling could operate on the richer syntax-preserving form

---

## 6. Why `Pattern<SyntaxNode>` Looks Plausible

### 6.1 Shared recursive shape

The gram grammar already has a pattern-like recursive organization:

- document/root
- node
- relationship
- subject pattern
- annotated pattern

These map cleanly onto a recursive "value plus elements" shape.

### 6.2 Payload can carry syntax-specific data

`SyntaxNode` could carry:

- a semantic `Subject` or subject-like content
- syntax kind (`node`, `relationship`, `annotation`, `document`, etc.)
- source span
- preserved annotations
- exact arrow kind
- quoting/surface-form information
- comments or trivia attachment

This would preserve syntax concerns without abandoning the `Pattern<T>` abstraction.

### 6.3 Mapping between syntax and semantics becomes explicit

A clean architecture would make these transforms explicit:

- `Pattern<SyntaxNode> -> Pattern<Subject>` for semantic lowering
- `Pattern<Subject> -> Pattern<SyntaxNode>` or direct rendering for serialization support

This would let the project test whether `Pattern` is not just the semantic output of gram,
but the general computational substrate for gram tooling.

---

## 7. Concerns and Caveats

### 7.1 A raw CST is not automatically a `Pattern`

If "CST" means every token, delimiter, punctuation mark, and trivia node as distinct tree
elements, that may not fit `Pattern<T>` naturally.

The promising direction is not "raw token tree as `Pattern`". It is:

- syntax-aware structured nodes
- recursive syntax tree projected into `Pattern<SyntaxNode>`

### 7.2 Some tooling still needs file/run-level structures

Even if syntax-aware patterns are adopted, not all diagnostics are node-local:

- duplicate identity is file-scoped
- dangling reference is symbol-table scoped
- formatter behavior may need document-level control

So a syntax-preserving pattern tree does not eliminate the need for file/report-level models.

### 7.3 This is larger than the current `pato` branch

This effort changes parser and representation strategy across the repo. It should not be
folded into the current `041-pato-cli` branch, because doing so would:

- mix a CLI feature branch with a parser architecture experiment
- make pato harder to evaluate independently
- blur implementation scope and acceptance criteria

---

## 8. Recommendation

### 8.1 Do not continue this work inside `041-pato-cli`

The current `pato` branch should be paused. It already proved useful:

- the CLI scaffold exists
- lint infrastructure exists
- the current parser limitations are now concrete rather than hypothetical

That is enough value for the branch to serve as evidence.

### 8.2 Start a new feature branch for CST-preserving parsing

Recommended approach:

1. Create a new feature branch dedicated to CST-preserving parsing.
2. Define the syntax-preserving target representation.
3. Evaluate `Pattern<SyntaxNode>` explicitly against a few representative grammar forms.
4. Determine how lowering to `Pattern<Subject>` should work.
5. Return to `pato` after the parser-layer direction is clear.

This separates the architectural experiment from the CLI deliverable.

---

## 9. Proposed Initial Scope For the New Branch

The new branch should answer a small number of focused questions first.

### 9.1 Representation spike

Design a draft `SyntaxNode` type that can represent:

- node patterns
- relationship patterns
- subject patterns
- annotated patterns
- file/document root

At minimum it should preserve:

- syntax kind
- subject-like payload
- spans
- arrow kind
- annotations

### 9.2 Parsing spike

Use `tree-sitter-gram` to produce one of:

- a direct `Pattern<SyntaxNode>` tree, or
- an intermediate CST that can be deterministically projected into `Pattern<SyntaxNode>`

### 9.3 Lowering spike

Implement a small lowering path:

- `Pattern<SyntaxNode> -> Pattern<Subject>`

Verify this on a few fixtures:

- a node
- a relationship
- an annotated pattern
- a header record plus top-level patterns
- a small path

### 9.4 Preservation spike

Demonstrate that the syntax-aware form preserves information the current parser loses:

- annotations
- exact arrow kind
- source spans
- enough surface detail to support precise diagnostics

---

## 10. Expected Outcomes

If successful, this work would provide:

- a syntax-preserving parser representation
- a principled relationship between syntax and semantic pattern forms
- a clearer foundation for linting, formatting, and transformation tools
- better evidence about whether `Pattern` is the correct general-purpose working model

If unsuccessful, it would still clarify the boundaries of `Pattern` and justify a more
traditional split between CST and semantic pattern structures.

Either result is useful.

---

## 11. Conclusion

The current `pato` work revealed an important parser question that deserves its own track.

The key insight is that `tree-sitter-gram` does not look alien to the `Pattern` model. Its
S-expression structure appears unusually well matched to it. That makes a CST-preserving,
pattern-shaped parser representation worth serious investigation.

The recommended path is therefore:

- pause `041-pato-cli`
- create a separate branch for CST-preserving parser work
- explore `Pattern<SyntaxNode>` or an adjacent syntax-aware pattern representation
- return to `pato` once the parser-layer design is clearer

This keeps the CLI feature branch focused while opening a potentially more foundational
advance for the whole project.
