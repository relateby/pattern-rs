# Feature Specification: Foldable Trait for Pattern

**Feature Branch**: `009-foldable-instance`  
**Created**: 2026-01-04  
**Status**: Draft  
**Input**: User description: "Add an idiomatic Rust foldable instance as described in 009-foldable-instance of TODO.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Aggregate Pattern Values into Single Result (Priority: P1)

Developers need to combine all values in a pattern structure into a single result (sum, product, concatenation, etc.) without manually traversing the pattern tree, processing values in a predictable order (root first, then elements).

**Why this priority**: This is the fundamental capability for data aggregation and reduction operations on patterns. Without this, developers must manually traverse pattern structures, which is error-prone and verbose. This enables essential operations like counting values, summing numeric data, collecting all strings, and building derived data structures.

**Independent Test**: Can be fully tested by creating patterns with various structures, applying fold operations (sum, concatenation, counting), and verifying that all values are processed in the correct order (root value first, then element values recursively). Delivers immediate value by enabling data aggregation on patterns.

**Acceptance Scenarios**:

1. **Given** an atomic pattern with integer value 42, **When** developer applies a fold operation to sum values, **Then** the result is 42 (the single value)
2. **Given** a pattern with root value 100 and three element values [10, 20, 30], **When** developer applies a fold operation to sum values, **Then** the result is 160 (processing root first: 100 + 10 + 20 + 30)
3. **Given** a deeply nested pattern with values at multiple levels, **When** developer applies a fold operation, **Then** all values at all nesting levels are processed in depth-first order (root first, then elements recursively)
4. **Given** a pattern with string values, **When** developer applies a fold operation to concatenate strings, **Then** all strings are concatenated in order (root value first, then element values)

---

### User Story 2 - Convert Pattern Values to Collections (Priority: P1)

Developers need to extract all values from a pattern into a standard collection (list, vector, array) to enable further processing with collection-based operations, maintaining the order of values (root first, then elements in order).

**Why this priority**: Converting patterns to collections is a critical operation for interoperability with standard library functions and external APIs. This enables developers to use familiar collection operations (filter, map, sort) on pattern values and integrate patterns with existing codebases.

**Independent Test**: Can be fully tested by creating patterns, converting them to collections (like `Vec`), and verifying that the collection contains all values in the correct order. Delivers value by enabling seamless integration with standard library operations.

**Acceptance Scenarios**:

1. **Given** an atomic pattern with value "test", **When** developer converts pattern to a collection, **Then** the result is a single-element collection containing "test"
2. **Given** a pattern with root value "first" and elements ["second", "third"], **When** developer converts pattern to a collection, **Then** the result is ["first", "second", "third"] in that exact order
3. **Given** a nested pattern structure, **When** developer converts pattern to a collection, **Then** all values appear in the collection in depth-first order (root first, then elements recursively)
4. **Given** a pattern with multiple levels of nesting and multiple elements per level, **When** developer converts pattern to a collection, **Then** the collection preserves the depth-first traversal order

---

### User Story 3 - Build Custom Aggregations with Folding Functions (Priority: P2)

Developers need to apply custom aggregation logic (counting, finding max/min, building custom data structures) by providing their own folding functions, with the fold handling pattern traversal automatically.

**Why this priority**: While sum and concatenation are common, real-world applications require custom aggregation logic. This enables developers to count values, find extrema, build maps/sets, validate all values meet criteria, and implement domain-specific aggregations without implementing tree traversal.

**Independent Test**: Can be tested by defining custom folding functions (counting, max-finding, custom data structure building) and verifying they process all pattern values correctly. Delivers value by enabling flexible, custom aggregations.

**Acceptance Scenarios**:

1. **Given** a pattern structure, **When** developer applies a counting fold function, **Then** the result is the total count of values in the pattern (including root and all elements)
2. **Given** a pattern with numeric values, **When** developer applies a fold function to find the maximum value, **Then** the result is the maximum across the root and all element values
3. **Given** a pattern with custom type values, **When** developer applies a fold function to build a derived data structure (e.g., histogram, index), **Then** all values contribute to the result structure
4. **Given** a pattern structure, **When** developer applies a fold function with initial accumulator state, **Then** the state is threaded through all values in the correct order

---

### User Story 4 - Chain Foldable Operations with Other Functional Patterns (Priority: P3)

Developers need to combine foldable operations with other functional patterns (functor, applicative, traversable) to build complex data pipelines, with all operations working together seamlessly.

**Why this priority**: Real-world applications often require chaining multiple operations (transform values, filter, then aggregate). Ensuring foldable works seamlessly with other abstractions enables idiomatic functional programming patterns and code reuse.

**Independent Test**: Can be tested by chaining map (functor) and fold operations, verifying the pipeline produces expected results. Delivers value by enabling expressive, composable data processing pipelines.

**Acceptance Scenarios**:

1. **Given** a pattern with integer values, **When** developer applies a transformation (map) followed by a fold (sum), **Then** the operations compose correctly (e.g., map doubling followed by sum produces double the original sum)
2. **Given** a pattern structure, **When** developer applies a fold to count values followed by a comparison, **Then** the fold integrates naturally with standard control flow
3. **Given** a pattern with mixed operations needed, **When** developer combines multiple functional operations in a pipeline, **Then** each operation receives the correct input from the previous stage

---

### Edge Cases

- What happens when folding an empty pattern structure (atomic pattern with no elements) - should process the single root value?
- How does the fold handle very deeply nested patterns (thousands of levels) without stack overflow?
- What happens when folding patterns with many elements at a single level (thousands of siblings)?
- How does the fold handle patterns where values have different types (requires type-level handling)?
- What happens when the folding function is computationally expensive or may panic - does the fold short-circuit or continue?
- How does the fold handle patterns where the accumulator state becomes very large?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a fold operation that processes all values in a pattern (root value and all element values recursively)
- **FR-002**: System MUST process values in depth-first order: root value first, then element values in order from left to right, recursing into each element
- **FR-003**: System MUST support right fold (foldr) operation that combines values from right to left with an accumulator
- **FR-004**: System MUST support left fold (foldl) operation that combines values from left to right with an accumulator
- **FR-005**: System MUST provide a way to convert pattern values into a collection (e.g., `to_vec()`, `collect()`, or similar) maintaining value order
- **FR-006**: System MUST allow developers to provide custom folding functions with an initial accumulator value
- **FR-007**: System MUST support monoid-based fold operations (fold without initial value, combining using a monoid operation like string concatenation or addition)
- **FR-008**: System MUST process atomic patterns (patterns with no elements) by processing only the root value
- **FR-009**: System MUST preserve the order guarantee: for pattern with root value V and elements [E1, E2, E3], folding must process V first, then values from E1, then values from E2, then values from E3
- **FR-010**: System MUST support integration with other functional operations (composing fold with map, filter, and other functional patterns)

### Key Entities *(include if feature involves data)*

- **Pattern<V>**: The pattern data structure with value type V that implements the Foldable trait
- **Accumulator**: The intermediate result value that is threaded through the fold operation
- **Folding Function**: A function that combines a value of type V with an accumulator to produce a new accumulator
- **Result**: The final value produced by the fold operation after processing all pattern values

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Fold operations correctly process all values in patterns with 1000 nodes, maintaining correct order and producing correct aggregations
- **SC-002**: Fold operations complete on patterns with 1000 nodes in under 10 milliseconds
- **SC-003**: Fold operations complete on patterns with 100 nesting levels without stack overflow
- **SC-004**: Converting patterns to collections preserves exact order of values (verifiable by comparing with hand-crafted traversal)
- **SC-005**: 100% of existing gram-hs foldable tests are ported and pass in Rust implementation
- **SC-006**: Foldable implementation compiles for WASM target without errors
- **SC-007**: Custom folding functions can be provided and work correctly across all pattern structures
- **SC-008**: Fold operations use constant stack space for iterative implementation or gracefully handle deep recursion
- **SC-009**: Pattern structures with 10,000 elements can be folded without exceeding 100MB memory overhead

## Assumptions

- Developers are familiar with basic functional programming concepts (fold, reduce, aggregation)
- The Pattern type already exists with its core structure (value and elements fields) and Functor implementation
- Rust's trait system can adequately represent Haskell's Foldable typeclass
- Folding functions provided by developers are pure (no side effects beyond accumulation)
- The implementation will use Rust's idiomatic patterns (iterators, trait implementations) rather than direct Haskell translation
- The implementation will follow Rust's Iterator trait patterns where applicable

## Dependencies

- Feature 004: Pattern Data Structure (must be complete - defines Pattern<V> type)
- Feature 005: Basic Pattern Type (must be complete - defines construction and access functions)
- Feature 008: Functor Instance (must be complete - provides map operation that foldable may compose with)
- Rust standard library support for Iterator trait and fold operations

## References

### Primary Reference (Authoritative)
- `../pattern-hs/libs/pattern/src/Pattern/Core.hs` - Foldable instance implementation (lines 750-751)

### Test References
- `../pattern-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` - Foldable tests (lines 1054-1499, comprehensive fold operation tests)

### Documentation Reference
- `../pattern-hs/docs/` - Up-to-date documentation about the implementation
- Foldable instance documentation in Pattern/Core.hs provides detailed examples and semantics

### Historical Reference (Context Only)
- `../pattern-hs/specs/006-foldable-instance/` - Historical notes from incremental development (may be outdated, verify against actual code)
