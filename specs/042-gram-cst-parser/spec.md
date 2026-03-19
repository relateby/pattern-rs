# Feature Specification: CST-Preserving Gram Parser

**Feature Branch**: `042-gram-cst-parser`
**Created**: 2026-03-19
**Status**: Draft
**Input**: User description: "a cst preserving parser based on tree-sitter-gram as described and motivated in proposals/gram-cst-proposal.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Parse Gram into Syntax-Preserving Tree (Priority: P1)

A library consumer parses a gram notation string and receives a syntax-preserving tree that retains
all structural information from the source text: node and relationship patterns, annotations, arrow
kinds, and source spans. The consumer can introspect any part of this tree without losing fidelity
to the original notation.

**Why this priority**: This is the foundational deliverable. Without a syntax-preserving parse
result, none of the downstream tooling goals — linting with precise diagnostics, formatting,
round-trip preservation — can be achieved. It is the core capability this feature exists to provide.

**Independent Test**: Can be tested by parsing a gram fixture file and verifying that the returned
tree contains annotation content, exact arrow kind (left, right, bidirectional, undirected), and
character-accurate source spans for each node and relationship — all of which the current semantic
parser discards.

**Acceptance Scenarios**:

1. **Given** a gram string containing an annotated node pattern, **When** the syntax-preserving
   parser processes it, **Then** the returned tree includes the annotation content attached to the
   node, not discarded.

2. **Given** a gram string with multiple arrow styles (`->`, `<-`, `<->`, `--`), **When** the
   syntax-preserving parser processes it, **Then** each relationship node in the tree preserves the
   exact arrow kind from the source.

3. **Given** any gram string that the current semantic parser accepts, **When** the
   syntax-preserving parser processes it, **Then** the parse succeeds and produces a valid
   syntax-preserving tree.

4. **Given** a gram string, **When** the syntax-preserving parser processes it, **Then** every node
   in the returned tree carries a source span that correctly identifies the start and end positions
   in the original input.

---

### User Story 2 - Lower Syntax Tree to Semantic Pattern (Priority: P2)

A library consumer takes the syntax-preserving parse result and lowers it to the existing
`Pattern<Subject>` semantic form, obtaining a result equivalent to what the current semantic parser
produces. All existing downstream consumers of `Pattern<Subject>` continue to work unchanged.

**Why this priority**: Lowering ensures the new parser layer does not break existing semantic
workflows. It validates that the syntax-preserving tree is a faithful superset of the semantic tree,
and it preserves compatibility with everything already built on `Pattern<Subject>`.

**Independent Test**: Can be tested by taking a set of gram fixtures, parsing each with the new
parser and lowering the result, then comparing the lowered output structurally to the output of the
current semantic parser on the same input.

**Acceptance Scenarios**:

1. **Given** a gram string representing a simple node, **When** the syntax tree is lowered, **Then**
   the result is structurally identical to the current semantic parser's output for that string.

2. **Given** a gram string representing a relationship pattern, **When** the syntax tree is lowered,
   **Then** the result is structurally identical to the current semantic parser's output.

3. **Given** a gram string containing a header record followed by top-level patterns, **When** the
   syntax tree is lowered, **Then** the result matches the current `parse_gram_with_header` output.

4. **Given** a gram string with an annotated pattern, **When** the syntax tree is lowered, **Then**
   the annotation is absent from the semantic result (as expected by current semantic consumers),
   confirming that the lowering step correctly strips syntax-only information.

---

### User Story 3 - Access Preserved Syntax Information for Diagnostics (Priority: P3)

A tooling developer (such as the `pato lint` implementor) accesses the syntax-preserving tree to
produce diagnostics with precise source locations. Instead of reconstructing approximate positions
from semantic output, the developer queries the tree directly for byte-accurate spans and exact
syntax forms.

**Why this priority**: This is the motivating use case from `pato lint`. Precise diagnostics require
source fidelity that the semantic layer cannot reliably provide. This story validates that the
preserved syntax information is actually usable for real tooling, not just theoretically retained.

**Independent Test**: Can be tested by implementing a simple lint check (e.g., flag duplicate node
identities) against the syntax-preserving tree and verifying that reported diagnostic spans point to
the correct character positions in the source input.

**Acceptance Scenarios**:

1. **Given** a gram string with a duplicate node identity appearing at two distinct positions,
   **When** a lint check traverses the syntax-preserving tree, **Then** both occurrences are
   reported with accurate source spans pointing to their respective positions in the original string.

2. **Given** a gram string where a relationship uses an unusual arrow kind, **When** a tool
   inspects the syntax tree, **Then** the exact arrow kind is accessible on the relationship node
   without any re-parsing or source reconstruction.

3. **Given** a gram string with annotation content, **When** a tool accesses the syntax tree,
   **Then** the full annotation text is accessible on the annotated node.

---

### User Story 4 - Demonstrate Structural Alignment with Pattern Abstraction (Priority: P4)

A developer evaluating the architecture confirms that the syntax-preserving tree is shaped as
`Pattern<SyntaxNode>` (or an equivalent syntax-aware pattern form), not as a foreign CST type. The
tree uses the same recursive structure as `Pattern<Subject>` while the payload type carries
syntax-specific data.

**Why this priority**: This story validates the core architectural hypothesis from the proposal —
that tree-sitter's gram S-expression structure is already close enough to `Pattern<T>` to allow a
syntax-preserving representation without a separate tree abstraction. It produces the evidence that
either confirms or refutes `Pattern` as a general computational substrate for gram tooling.

**Independent Test**: Can be tested by inspecting the type of the parse result: if the new parser
returns `Pattern<SyntaxNode>` and `SyntaxNode` can be traversed using the same `Pattern` operations
(`map`, `fold`, `filter`, etc.) that work on `Pattern<Subject>`, the structural alignment is
confirmed.

**Acceptance Scenarios**:

1. **Given** a gram string parsed with the new parser, **When** standard pattern traversal
   operations are applied to the result, **Then** they behave correctly — confirming `Pattern<T>`
   operations are not node-type-specific.

2. **Given** the `Pattern<SyntaxNode>` type, **When** the lowering function maps it to
   `Pattern<Subject>`, **Then** the mapping is expressed as a standard `Pattern::map` or equivalent
   transformation — not a bespoke recursive visitor — demonstrating structural compatibility.

---

### Edge Cases

- When gram input contains syntax errors, the parser returns a partial tree alongside a list of error spans — it does not abort on first error.
- How does the syntax tree represent a completely empty gram document?
- How does span tracking behave when the input contains multi-byte Unicode characters?
- What does the syntax tree look like for a deeply nested subject pattern (many elements)?
- How does lowering handle a gram document with a header record but no body patterns?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The parser MUST accept any gram notation string that the current semantic parser
  accepts and produce a syntax-preserving tree without error.

- **FR-002**: The syntax-preserving tree MUST retain annotation content for every annotated
  pattern in the input, including the annotation body text or structure.

- **FR-003**: The syntax-preserving tree MUST record the exact arrow kind for every relationship
  pattern (`right_arrow`, `left_arrow`, `bidirectional`, `undirected`).

- **FR-004**: Every node in the syntax-preserving tree MUST carry a source span (start byte offset,
  end byte offset) corresponding to its range in the original input string.

- **FR-005**: A lowering function MUST exist that maps the syntax-preserving tree to
  `Pattern<Subject>`, producing output structurally equivalent to the current semantic parser's
  output for the same input.

- **FR-006**: The syntax-preserving tree representation MUST be shaped as `Pattern<SyntaxNode>` or
  a type structurally compatible with the `Pattern<T>` abstraction (same recursive form, usable
  with standard `Pattern` traversal operations).

- **FR-007**: The parser MUST handle all gram construct types: node patterns, relationship patterns,
  subject patterns (arbitrary-element sequences), annotated patterns, and document/root patterns.

- **FR-008**: The lowering function MUST correctly handle header records (producing the same output
  as `parse_gram_with_header`) as well as body-only documents. Comment nodes MUST be silently
  dropped during lowering; the semantic output MUST be identical to the current parser's output.

- **FR-009**: The syntax-preserving tree MUST represent comments as first-class nodes, preserving
  their full text content and source span. Comment nodes MUST appear as top-level siblings in the
  document tree, interleaved with pattern nodes in source order. Association with adjacent patterns
  is not required.

- **FR-010**: When parsing input that contains syntax errors, the parser MUST return a result
  containing both the best-effort partial syntax tree and a list of error spans identifying the
  locations of all parse errors. The parser MUST NOT silently succeed on invalid input.

### Key Entities

- **SyntaxNode**: The payload type for the syntax-preserving pattern tree. Carries a syntax kind
  (node, relationship, annotation, document, etc.), an optional subject-like identity/label/property
  payload, a source span, the exact arrow kind (for relationship nodes), and preserved annotation
  content.

- **Pattern\<SyntaxNode\>**: The syntax-preserving parse result. A recursive `Pattern` tree whose
  payload is `SyntaxNode` rather than `Subject`. Shares the same recursive structure as
  `Pattern<Subject>`.

- **SyntaxKind**: An enumeration of the structural roles a syntax node can play: document root,
  node pattern, relationship pattern, annotated pattern, subject pattern, annotation body, comment.

- **SourceSpan**: A byte-range value (start, end) locating a node within the original input string.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: 100% of gram fixture files accepted by the current semantic parser are also accepted
  by the new syntax-preserving parser without error.

- **SC-002**: The lowered output of the new parser matches the current semantic parser's output on
  all existing test fixtures, with zero structural discrepancies.

- **SC-003**: Annotation content is preserved and accessible for every annotated pattern in a
  representative set of gram fixtures, compared to zero accessible annotation content through the
  current parser.

- **SC-004**: Source spans on syntax tree nodes are byte-accurate for all nodes across a
  representative fixture set (verified by extracting the span range from the input and comparing to
  the source text of the node).

- **SC-005**: The structural relationship between `Pattern<SyntaxNode>` and `Pattern<Subject>` is
  expressible as a standard `Pattern::map`-style transformation, demonstrating that no bespoke
  tree-walking visitor is required for the lowering path.

## Clarifications

### Session 2026-03-19

- Q: Should comments be in scope for this feature, and if so how should they be preserved? → A: Comments preserved as first-class nodes in the syntax tree; attachment/association to adjacent patterns deferred to gramdoc.
- Q: Must the CST parser compile and work on the WASM target? → A: Native-only for this feature; WASM support is a documented follow-on concern.
- Q: What should the parser return when tree-sitter encounters a parse error? → A: Return a result carrying both the partial tree and a list of error spans (best-effort recovery).
- Q: Where do comment nodes appear in the Pattern<SyntaxNode> tree? → A: As top-level siblings in the document tree, interleaved with patterns in source order.
- Q: What happens to comment nodes when lowering to Pattern<Subject>? → A: Silently dropped; lowered output is identical to current semantic parser output.

## Assumptions

- The `tree-sitter-gram` grammar submodule already present in the repository is the correct and
  authoritative grammar to use as the parsing backend.
- The existing `tree-sitter` transformation code in `crates/gram-codec/src/transform.rs` can serve
  as a starting point or reference, even if it requires significant revision.
- The `SyntaxNode` payload type will be a new type in this feature branch; it does not need to
  reuse or extend `Subject` internally, though it may hold subject-like data.
- Arrow kind preservation requires distinguishing at minimum: `->` (right), `<-` (left), `<->`
  (bidirectional), `--` (undirected). The grammar's existing arrow kind tokens are the source of
  truth.
- Whitespace trivia is out of scope. Comments are in scope as first-class syntax tree nodes.
  Comment-to-pattern association (e.g., "this comment belongs to the next node") is deferred to
  gramdoc tooling, not resolved in this feature.
- The new parser is an addition to the existing API surface, not a replacement; the current
  `parse_gram` and `parse_gram_with_header` functions remain available.
- The CST parser targets native Rust only. WASM compatibility (blocked by tree-sitter's C runtime
  dependency) is a documented follow-on concern, not a requirement of this feature.
