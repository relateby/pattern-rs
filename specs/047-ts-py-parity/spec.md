# Feature Specification: TypeScript and Python Pattern API Parity

**Feature Branch**: `047-ts-py-parity`
**Created**: 2026-04-16
**Status**: Draft
**Input**: User description: "ts-py-parity as detailed in @proposals/language-parity-proposal.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Structural Pattern Comparison (Priority: P1)

A developer using TypeScript or Python wants to check whether two patterns have the same shape and values, or whether one pattern contains another as a sub-structure. Today they must implement these checks themselves using `fold` or `filter`, which is error-prone and repetitive.

**Why this priority**: These are the most frequently needed operations in pattern-matching and graph query code. The Haskell reference and Rust implementation both expose them directly. Their absence forces workarounds in every consumer.

**Independent Test**: Can be fully tested by calling `matches(patternA, patternB)` and `contains(outer, inner)` on known patterns and verifying correct boolean results. Delivers value independently as a standalone query toolkit.

**Acceptance Scenarios**:

1. **Given** two patterns with identical structure and equal values, **When** `matches` is called, **Then** it returns `true`.
2. **Given** two patterns that differ in depth, element count, or value, **When** `matches` is called, **Then** it returns `false`.
3. **Given** an outer pattern that recursively contains an inner pattern anywhere in its tree, **When** `contains` is called, **Then** it returns `true`.
4. **Given** an outer pattern that does not contain the inner pattern anywhere, **When** `contains` is called, **Then** it returns `false`.
5. **Given** a large pattern tree, **When** `anyValue(pred)` is called, **Then** it returns `true` on the first matching value without traversing the remainder.
6. **Given** all values in a pattern satisfy a predicate, **When** `allValues(pred)` is called, **Then** it returns `true`.
7. **Given** any value in a pattern fails a predicate, **When** `allValues(pred)` is called, **Then** it returns `false`.

---

### User Story 2 — Structure-Aware Folding (Paramorphism) (Priority: P1)

A developer wants to compute a result from a Pattern where the fold function needs to see both the current sub-pattern and the already-computed results from its child elements — for example, rendering a pattern to a string that reflects nesting, or computing per-node metrics that depend on children.

Today this requires a manual recursive function because `fold` only passes values, not sub-patterns. The developer ends up reimplementing the recursion each time.

**Why this priority**: `para` (paramorphism) is the foundation for display, diffing, and structural analysis algorithms. It is present in Haskell and Rust. Many graph transform operations at the `paraGraph` level already depend on this concept — making it available at the base `Pattern<V>` level completes the abstraction.

**Independent Test**: Can be tested by calling `para(f)` on a known tree and verifying that the fold function receives the correct sub-patterns and child results at each node.

**Acceptance Scenarios**:

1. **Given** a pattern tree, **When** `para(f)` is applied, **Then** `f` is called bottom-up with each sub-pattern and the pre-computed results of its direct children.
2. **Given** an atomic (leaf) pattern, **When** `para(f)` is applied, **Then** `f` receives the pattern and an empty child-results list.
3. **Given** a nested pattern, **When** `para(f)` builds a depth annotation, **Then** the root receives the correct maximum depth computed from children.

---

### User Story 3 — Programmatic Pattern Construction (Priority: P2)

A developer wants to build `Pattern<V>` trees programmatically — creating non-atomic patterns with specific child elements — without resorting to gram string parsing. They also want a shorthand to create a pattern whose children are all atomic patterns over a list of values.

Currently the only documented constructor is `Pattern.point(value)` which creates only leaf nodes. Creating composite patterns requires direct class/dataclass construction which is undiscoverable from the public API.

**Why this priority**: Programmatic construction is essential for testing, data transformation, and generating patterns from external sources. Gram parsing is a runtime dependency; programmatic constructors are not.

**Independent Test**: Can be tested by constructing a multi-level pattern using `Pattern.pattern(value, [child1, child2])` and verifying the resulting structure.

**Acceptance Scenarios**:

1. **Given** a value and a list of child patterns, **When** `Pattern.pattern(value, children)` is called, **Then** the resulting pattern has that value and exactly those children as its elements.
2. **Given** a value and a list of plain values, **When** `Pattern.fromList(value, [v1, v2, v3])` is called, **Then** the result is a pattern with that value and each list entry as an atomic child.
3. **Given** a seed value and an expand function that returns `(value, [seeds])`, **When** `unfold(expand, seed)` is called, **Then** the resulting tree matches the recursive expansion of the seed.
4. **Given** `unfold` with an expand function that returns empty children for a seed, **Then** the result is an atomic pattern at that node.

---

### User Story 4 — Pattern Combination (Semigroup / Combine) (Priority: P2)

A developer wants to merge two patterns into one — combining their root values and concatenating their element lists — to build up patterns incrementally or merge partial patterns from different sources.

**Why this priority**: The Semigroup/combine operation is the algebraic primitive for merging patterns. It underpins the reconciliation system in Rust and Haskell. Without it, TypeScript and Python consumers must build merge logic manually.

**Independent Test**: Can be tested by combining two patterns and verifying the root value is the combination of both root values and the elements are the concatenation of both element lists.

**Acceptance Scenarios**:

1. **Given** two patterns with combinable values, **When** `combine(a, b)` is called, **Then** the result has a combined root value (per the value type's combine semantics) and elements `[...a.elements, ...b.elements]`.
2. **Given** a pattern combined with an identity/empty pattern, **When** the combination is computed, **Then** the result is structurally equal to the original pattern.
3. **Given** three patterns combined in sequence, **When** `combine(combine(a, b), c)` equals `combine(a, combine(b, c))`, **Then** the operation is associative.

---

### User Story 5 — Comonad Position Helpers (Priority: P3)

A developer wants to annotate every position in a pattern tree with context-derived information: the nesting depth, the subtree size, or the index path from the root. These are common operations for layout, display, and debugging.

**Why this priority**: These are named `extend` specializations that Rust exposes as helpers. Naming them makes the comonad pattern discoverable and reduces boilerplate. Lower priority because `extend` already makes them expressible by any caller.

**Independent Test**: Can be tested by calling `depthAt(pattern)` and verifying each node in the resulting `Pattern<number>` carries the correct depth integer.

**Acceptance Scenarios**:

1. **Given** a pattern tree, **When** `depthAt(p)` is called, **Then** it returns a `Pattern<number>` where each node's value is its depth from the root (root = 0).
2. **Given** a pattern tree, **When** `sizeAt(p)` is called, **Then** each node's value is the total number of nodes in its subtree (including itself).
3. **Given** a pattern tree, **When** `indicesAt(p)` is called, **Then** each node's value is the list of child indices forming the path from root to that node.

---

### User Story 6 — Python Graph Transforms (Priority: P3)

A Python developer wants to transform a graph by mapping, filtering, or folding over its classified elements (nodes, relationships, walks, annotations). Today Python's `StandardGraph` only provides query and lookup operations; it has no transform functions equivalent to those available in TypeScript.

**Why this priority**: Graph transforms are frequently needed for processing and projecting graph data. TypeScript already has a complete transform layer. Python consumers are blocked from doing the same without reimplementing the logic.

**Independent Test**: Can be tested by applying a node-mapping transform to a `StandardGraph` and verifying that all node patterns are transformed while other element types are unchanged.

**Acceptance Scenarios**:

1. **Given** a `StandardGraph`, **When** `map_graph(mappers)` is called with a node mapper, **Then** all node patterns are transformed and relationship/walk/annotation patterns are unchanged.
2. **Given** a `StandardGraph`, **When** `filter_graph(pred, substitution)` removes a node, **Then** relationships referencing that node are handled according to the substitution strategy.
3. **Given** a `StandardGraph`, **When** `fold_graph(f, empty, combine)` is applied, **Then** it reduces all classified elements to a single value.
4. **Given** a `StandardGraph`, **When** `map_with_context(f)` is applied, **Then** each element's transformation function receives a snapshot `GraphQuery` reflecting the graph at transformation start.
5. **Given** a `StandardGraph`, **When** `para_graph(f)` is applied, **Then** elements are processed bottom-up with pre-computed child results passed to each element's fold function.

---

### Edge Cases

- What happens when `contains` is called with a pattern equal to the outer pattern? It returns `true`.
- What happens when `unfold` encounters a non-terminating expand function? The specification does not require cycle detection; callers are responsible for terminating expand functions.
- What happens when `combine` is called with value types that do not define a combine operation? The behavior is constrained at call time by the type system; no runtime error should be possible for well-typed inputs.
- What happens when `para` is applied to an atomic pattern? The fold function receives the pattern and an empty results list.
- What happens when `filter_graph` removes an element that is the only member of a container? The substitution strategy determines the outcome (delete container, splice gap, or replace with surrogate).

---

## Requirements *(mandatory)*

### Functional Requirements

**TypeScript and Python — Core Pattern Operations:**

- **FR-001**: The TypeScript and Python libraries MUST expose `anyValue(predicate)` returning `true` if any value in the pattern tree satisfies the predicate, short-circuiting on the first match.
- **FR-002**: The TypeScript and Python libraries MUST expose `allValues(predicate)` returning `true` only if every value satisfies the predicate, short-circuiting on the first failure.
- **FR-003**: The TypeScript and Python libraries MUST expose `matches(a, b)` returning `true` when two patterns have identical structure and pairwise-equal values.
- **FR-004**: The TypeScript and Python libraries MUST expose `contains(outer, inner)` returning `true` when the inner pattern appears anywhere in the outer pattern tree.
- **FR-005**: The TypeScript and Python libraries MUST expose `para(f)` (paramorphism) that folds a Pattern bottom-up, passing each sub-pattern and the pre-computed results of its direct children to `f`.
- **FR-006**: The TypeScript and Python libraries MUST expose `Pattern.pattern(value, elements)` (or equivalent named factory) for constructing non-atomic patterns without using gram parsing.
- **FR-007**: The TypeScript and Python libraries MUST expose `Pattern.fromList(value, values)` that creates a pattern whose children are atomic patterns over the provided list of values.
- **FR-008**: The TypeScript and Python libraries MUST expose `unfold(expand, seed)` that builds a `Pattern<V>` tree by repeatedly applying an expand function to a seed value.
- **FR-009**: The TypeScript and Python libraries MUST expose `combine(a, b)` that merges two patterns by combining root values (per the value type's combine semantics) and concatenating element lists.
- **FR-010**: The TypeScript and Python libraries MUST expose `depthAt(p)`, `sizeAt(p)`, and `indicesAt(p)` as named comonad helpers annotating every position with depth, subtree size, and root-path index list respectively.

**Python — Graph Transforms:**

- **FR-011**: The Python library MUST expose `map_graph(mappers)` that transforms elements by class using per-class mapper functions.
- **FR-012**: The Python library MUST expose `map_all_graph(f)` that applies a uniform transformation to all elements regardless of class.
- **FR-013**: The Python library MUST expose `filter_graph(pred, substitution)` that removes non-matching elements and applies the specified substitution strategy to affected containers.
- **FR-014**: The Python library MUST expose `fold_graph(f, empty, combine)` that reduces all classified elements to a single value.
- **FR-015**: The Python library MUST expose `map_with_context(f)` providing each element's transform function with an immutable snapshot of the graph at transform start.
- **FR-016**: The Python library MUST expose `para_graph(f)` processing elements bottom-up in topological order with pre-computed child results.

### Key Entities

- **Pattern\<V\>**: A recursive tree of nodes, each with a value of type `V` and an ordered list of child patterns. All new operations preserve this structure contract.
- **Combinable value**: A value type that defines an associative binary combine operation, used as a constraint for `combine`.
- **GraphQuery snapshot**: An immutable view of the graph captured at the start of a `map_with_context` transform, preventing transform functions from observing mid-transform state.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All ten core Pattern operations (FR-001 through FR-010) are callable from TypeScript and Python with correct results on all acceptance scenarios defined in this spec.
- **SC-002**: All six Python graph transform operations (FR-011 through FR-016) produce results equivalent to their TypeScript counterparts when applied to the same input graph.
- **SC-003**: New operations produce results consistent with the Rust reference implementation on the same logical inputs, as verified by cross-language equivalence tests.
- **SC-004**: No existing TypeScript or Python tests regress as a result of adding the new operations.
- **SC-005**: Each new operation is reachable through the primary public import path (`@relateby/pattern` for TypeScript, `relateby.pattern` for Python) without internal import path workarounds.
- **SC-006**: TypeScript type signatures for new operations are fully generic and do not require `any` casts at call sites for well-typed inputs.

---

## Assumptions

- TypeScript implementations follow the existing curried-function style in `ops.ts` for composability with `pipe()`.
- Python implementations follow the existing class-method and standalone-function style in `relateby.pattern`.
- `combine` requires the value type to provide a combine operation; no default combine behavior is assumed for arbitrary `V`.
- Python graph transforms may be implemented as standalone functions operating on `StandardGraph` or `GraphView`, matching the TypeScript pattern of separating data from transforms.
- `unfold` is not required to handle infinite expansion; callers supply terminating expand functions.

---

## Out of Scope

- Rust-side changes (`duplicate`, Subject property helpers) tracked in `proposals/language-parity-proposal.md` items 9 and 10.
- Gram parsing, serialization, or codec changes.
- `Applicative` instance, `paraWithScope`, `ScopeQuery`, `PatternKind`, `RepresentationMap` — deferred per proposal recommendation.
- Changes to `@relateby/gram` or `relateby.gram` modules.
