# Feature Specification: Pattern Paramorphism

**Feature Branch**: `025-pattern-paramorphism`  
**Created**: 2026-01-30  
**Status**: Draft  
**Input**: User description: "Pattern paramorphism as described in Phase 5 of TODO.md"

## Clarifications

### Session 2026-01-30

- Q: User stories are tree-oriented; primary intent is to analyze patterns of elements (e.g., do they follow an A, B, A pattern). Use "elements" not "children". → A: Revised all user stories and spec to use "elements" consistently and to emphasize pattern-of-elements analysis (e.g., detecting A, B, A or analyzing sequences of elements) as a primary use case.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Pattern-of-Elements Analysis (Priority: P1)

As a library user analyzing patterns of elements, I want to use paramorphism to detect and aggregate over element sequences (e.g., whether elements follow an A, B, A pattern), so I can reason about the structure and ordering of elements in a pattern.

**Why this priority**: The primary intent of the Pattern data structure is to analyze patterns of elements. Paramorphism is essential for structure-aware analysis over element sequences (e.g., repetition, ordering, subsequences).

**Independent Test**: Can be fully tested by creating patterns with ordered elements and verifying paramorphism can aggregate over element sequences (e.g., detecting A, B, A or computing sequence-based statistics). Delivers immediate value for pattern-of-elements use cases.

**Acceptance Scenarios**:

1. **Given** a pattern with value A and elements with values B and A in order, **When** I use paramorphism to analyze the sequence of element results, **Then** I can detect or aggregate over the A, B, A pattern.

2. **Given** a nested pattern with elements at multiple levels, **When** I use paramorphism with access to the current pattern and results from elements, **Then** I can compute structure-aware aggregations over the element sequence.

3. **Given** an atomic pattern (no elements), **When** I apply paramorphism, **Then** the folding function receives the pattern and an empty slice of element results (base case).

---

### User Story 2 - Element-Count-Aware Computation (Priority: P1)

As a library user, I want to perform aggregations that consider how many elements each pattern has, so I can compute statistics that depend on the number and arrangement of elements.

**Why this priority**: Element count awareness is a core structural property for pattern analysis. Many analyses of element sequences require knowing how many elements exist at each position.

**Independent Test**: Can be fully tested by creating patterns with varying numbers of elements and verifying element-count-weighted computations produce correct results.

**Acceptance Scenarios**:

1. **Given** a pattern with 2 elements, **When** I compute a value weighted by element count using paramorphism, **Then** the pattern's value is correctly multiplied by its element count.

2. **Given** a nested pattern where one node has 3 elements and each of those has 2 elements, **When** I aggregate element counts across the structure, **Then** I receive accurate counts at each level.

---

### User Story 3 - Nesting Statistics (Priority: P2)

As a library user, I want to compute multiple statistics about pattern structure in a single traversal (sum, count, max depth), so I can efficiently analyze complex patterns of elements.

**Why this priority**: While individual statistics can be computed separately, computing multiple statistics in one pass is more efficient and demonstrates paramorphism's full power.

**Independent Test**: Can be fully tested by creating patterns and verifying that multi-statistic aggregations produce correct tuples of (sum, count, maxDepth).

**Acceptance Scenarios**:

1. **Given** a pattern structure, **When** I use paramorphism to compute (sum, count, maxDepth), **Then** I receive all three statistics computed in a single traversal.

2. **Given** an atomic pattern (no elements), **When** I compute nesting statistics, **Then** I receive (value, 1, 0) representing the single node.

---

### User Story 4 - Custom Structure-Aware Folding (Priority: P2)

As a library user, I want to define custom folding functions that have access to both the current pattern and the recursively computed results from its elements, so I can implement domain-specific aggregations over patterns of elements.

**Why this priority**: Enables users to implement any structure-aware aggregation (e.g., sequence detection, element-order analysis), making paramorphism a general-purpose tool for pattern analysis.

**Independent Test**: Can be tested by implementing a custom folding function (e.g., building a representation of the element sequence or detecting repetition) and verifying correct output.

**Acceptance Scenarios**:

1. **Given** any valid folding function that accepts (pattern, element_results), **When** I apply paramorphism with this function, **Then** the function is correctly applied at each node with the recursively computed results from its elements.

2. **Given** a folding function that extracts structural information (e.g., element order, path indices), **When** I apply paramorphism, **Then** I can reconstruct structural relationships and analyze patterns of elements from the results.

---

### Edge Cases

- What happens when the pattern is atomic (no elements)? The folding function receives an empty slice for element results.
- What happens with deeply nested patterns? Paramorphism handles arbitrary depth via recursive descent, limited only by stack space.
- What happens if the folding function ignores element results? The function still receives them, but can choose to ignore, effectively simulating simpler folds.
- What happens with empty element results slice? This is the base case for recursion and must be handled correctly.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a `para` method on `Pattern<V>` that accepts a folding function and returns an aggregated result.

- **FR-002**: The folding function MUST receive two arguments: a reference to the current pattern (`&Pattern<V>`) and a slice of recursively computed results from its elements (`&[R]`).

- **FR-003**: System MUST process elements recursively before applying the folding function to the current node (bottom-up evaluation).

- **FR-004**: System MUST preserve element order when computing results (left-to-right over elements).

- **FR-005**: System MUST support any result type `R` for the folding function, allowing flexible aggregations.

- **FR-006**: System MUST handle atomic patterns (no elements) by passing an empty slice to the folding function.

- **FR-007**: System MUST provide access to all pattern structural properties (value, elements, depth, length) within the folding function.

- **FR-008**: System MUST be generic over the pattern's value type `V`, working with any value type.

### Key Entities

- **Pattern<V>**: The recursive data structure being folded over. Has a value of type `V` and zero or more elements (also `Pattern<V>`).

- **Folding Function**: A user-provided function with signature `Fn(&Pattern<V>, &[R]) -> R` that combines the current pattern with results from its elements.

- **Result Type (R)**: The type of the aggregated result, determined by the folding function.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can compute depth-weighted sums for patterns of any depth in a single traversal call.

- **SC-002**: Users can compute element-count-aware aggregations that correctly reflect the branching structure.

- **SC-003**: Users can compute multiple statistics (sum, count, max depth) in a single paramorphism traversal.

- **SC-004**: The paramorphism implementation passes all property-based tests verifying:
  - Structure access property: `para(|p, _| p.clone())` returns the original pattern
  - Relationship to Foldable: `para(|p, rs| p.value + rs.sum())` equals `fold(+, 0)`
  - Order preservation: values are processed in depth-first, left-to-right order

- **SC-005**: All examples from the gram-hs reference documentation produce equivalent results when ported to gram-rs.

- **SC-006**: Documentation clearly explains the relationship between paramorphism, Foldable, and Comonad operations.

## Assumptions

- The existing `Pattern<V>` structure exposes `value()`, `elements()`, and `depth()` methods needed by the folding function.
- Paramorphism is implemented as a method on `Pattern<V>` following Rust idioms (not a standalone function).
- The implementation uses references (`&Pattern<V>`, `&[R]`) to avoid unnecessary cloning.
- Stack depth for recursion is sufficient for typical pattern depths (patterns with thousands of nesting levels are not a common use case).
- The gram-hs reference implementation in `../gram-hs/libs/pattern/src/Pattern/Core.hs` is the authoritative source for behavioral equivalence.

## Out of Scope

- Lazy or streaming paramorphism variants (all results computed eagerly).
- Parallel/concurrent execution of element computations.
- Memoization or caching of intermediate results.
- Iterative (non-recursive) implementation for stack-limited environments.
