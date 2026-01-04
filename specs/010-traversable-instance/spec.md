# Feature Specification: Traversable Trait for Pattern

**Feature Branch**: `010-traversable-instance`  
**Created**: 2026-01-04  
**Status**: Draft  
**Input**: User description: "Traversable instance for patterns as described in 010 of @TODO.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Apply Effectful Transformations to Pattern Values (Priority: P1)

Developers need to apply transformations to pattern values where each transformation may produce effects (like validation errors, optional results, asynchronous operations) and have the effects automatically handled and aggregated across the entire pattern structure.

**Why this priority**: This is the fundamental capability of Traversable that distinguishes it from Functor. While Functor's map works with pure functions, Traversable's traverse works with effectful functions (returning Option, Result, Future, etc.), automatically propagating and combining effects. This enables validation pipelines, error-aware transformations, and asynchronous processing without manual effect management.

**Independent Test**: Can be fully tested by creating patterns, applying effectful transformations (validation functions returning Result, lookup functions returning Option), and verifying that effects are properly propagated (e.g., if any value fails validation, the entire traversal returns an error). Delivers immediate value by enabling effect-aware data processing.

**Acceptance Scenarios**:

1. **Given** an atomic pattern with value "42", **When** developer applies traverse with a parse-to-integer function returning `Result<i32, ParseError>`, **Then** the result is `Ok(Pattern<i32>)` with parsed value
2. **Given** a pattern with root value "valid" and element values ["also-valid", "also-valid"], **When** developer applies traverse with a validation function returning `Result<String, Error>`, **Then** all values are validated and the result is `Ok(Pattern<String>)` with all validated values
3. **Given** a pattern with root value "valid" and element values ["valid", "INVALID"], **When** developer applies traverse with a validation function, **Then** the result is `Err(ValidationError)` because one value failed validation
4. **Given** a deeply nested pattern, **When** developer applies traverse with an effectful transformation, **Then** all values at all nesting levels are processed and effects are combined in the correct order (root first, then elements recursively)
5. **Given** a pattern with optional values, **When** developer applies traverse with a function returning `Option<T>`, **Then** if all values produce Some, result is `Some(Pattern<T>)`, but if any produces None, result is `None`

---

### User Story 2 - Sequence Nested Effect Structures (Priority: P1)

Developers need to flip the layers of nested structures (e.g., convert `Pattern<Option<T>>` to `Option<Pattern<T>>` or `Pattern<Result<T, E>>` to `Result<Pattern<T>, E>`) to extract patterns from effects or handle effects uniformly across entire pattern structures.

**Why this priority**: Sequencing is essential when pattern values already contain effects (from previous operations or external sources) and developers need to "pull out" the effects to work with them uniformly. This enables composing multiple effectful operations, handling all-or-nothing semantics (all values must be Some/Ok), and integrating with effect-aware APIs.

**Independent Test**: Can be tested by creating patterns containing effects (Pattern<Option<i32>>), applying sequence, and verifying the structure is flipped correctly (Option<Pattern<i32>>). Delivers value by enabling clean composition of effectful operations.

**Acceptance Scenarios**:

1. **Given** a pattern with all values as `Some(x)` (Pattern<Option<i32>>), **When** developer applies sequence, **Then** the result is `Some(Pattern<i32>)` with all inner values extracted
2. **Given** a pattern with at least one value as `None` (Pattern<Option<i32>>), **When** developer applies sequence, **Then** the result is `None` (all-or-nothing semantics)
3. **Given** a pattern with all values as `Ok(x)` (Pattern<Result<String, Error>>), **When** developer applies sequence, **Then** the result is `Ok(Pattern<String>)` with all success values extracted
4. **Given** a pattern with at least one value as `Err(e)` (Pattern<Result<String, Error>>), **When** developer applies sequence, **Then** the result is `Err(e)` with the first error encountered
5. **Given** a nested pattern structure with effects at all levels, **When** developer applies sequence, **Then** effects are extracted in the correct order and combined properly

---

### User Story 3 - Validate All Pattern Values with Early Termination (Priority: P2)

Developers need to validate all values in a pattern structure where validation may fail for individual values, with the entire operation failing fast (short-circuiting) on the first error, avoiding unnecessary computation on remaining values.

**Why this priority**: Validation is a common real-world use case where developers need to ensure all pattern values meet certain criteria before proceeding with processing. Early termination on first error improves performance and provides immediate feedback about validation failures.

**Independent Test**: Can be tested by creating patterns with invalid values at different positions, applying traverse with a validation function, and verifying that traversal terminates on first error without processing remaining values. Delivers value by enabling efficient validation pipelines.

**Acceptance Scenarios**:

1. **Given** a pattern where all values are valid, **When** developer applies traverse with a validation function returning Result, **Then** all values are validated and the result is Ok with the validated pattern
2. **Given** a pattern where the root value is invalid, **When** developer applies traverse with a validation function, **Then** the traversal fails immediately with an error for the root value without processing elements
3. **Given** a pattern where an element value (not root) is invalid, **When** developer applies traverse with a validation function, **Then** the traversal processes values in order until reaching the invalid value, then fails without processing remaining values
4. **Given** a pattern with multiple invalid values, **When** developer applies traverse with a validation function, **Then** only the first invalid value (in traversal order) is reported

---

### User Story 4 - Compose Traversable with Other Functional Patterns (Priority: P3)

Developers need to chain Traversable operations with Functor and Foldable operations to build complex data processing pipelines that combine pure transformations, effectful transformations, and aggregations.

**Why this priority**: Real-world applications often require combining multiple functional abstractions. Ensuring Traversable composes cleanly with Functor (map) and Foldable (fold) enables expressive, maintainable pipelines and reduces boilerplate code.

**Independent Test**: Can be tested by building pipelines that combine map (pure transformation), traverse (effectful transformation), and fold (aggregation), verifying the operations compose correctly. Delivers value by enabling expressive functional programming patterns.

**Acceptance Scenarios**:

1. **Given** a pattern with string values, **When** developer applies map (to uppercase) followed by traverse (to parse as integers), **Then** the operations compose correctly and the result is an effectful pattern of integers
2. **Given** a pattern structure, **When** developer applies traverse (validation) followed by map (pure transformation), **Then** the validation runs first and pure transformation only applies to valid values
3. **Given** a pattern, **When** developer combines traverse with fold operations, **Then** effects are handled in traverse and aggregation works on the resulting effectful pattern

---

### User Story 5 - Traverse Patterns with Asynchronous Operations (Priority: P3)

Developers need to apply asynchronous operations (like database lookups, API calls, file I/O) to pattern values and have the async effects properly managed and composed across the pattern structure.

**Why this priority**: Modern applications frequently need to perform async operations on data structures. Traversable provides a natural abstraction for applying async operations to all values in a pattern and collecting the results, which is essential for real-world async programming patterns.

**Independent Test**: Can be tested by creating patterns and applying traverse with functions that return Future/Promise types, verifying that all async operations are initiated and results are collected correctly. Delivers value by enabling async data processing on patterns.

**Acceptance Scenarios**:

1. **Given** a pattern with ID values, **When** developer applies traverse with an async lookup function returning Future<Entity>, **Then** all lookups are initiated and results are collected into a Future<Pattern<Entity>>
2. **Given** a pattern structure, **When** developer applies traverse with async operations, **Then** operations can run concurrently (depending on effect type) and results are combined in the correct order
3. **Given** a pattern with async operations that may fail, **When** developer applies traverse, **Then** the first failure short-circuits and returns the error without waiting for remaining operations

---

### Edge Cases

- What happens when traversing an empty pattern structure (atomic pattern with no elements) with an effectful function?
- How does traverse handle very deeply nested patterns (thousands of levels) without stack overflow?
- What happens when traversing patterns with many elements at a single level (thousands of siblings) - does it process sequentially or can it parallelize?
- How does traverse handle transformation functions that are computationally expensive or may panic?
- What happens when the effect type requires specific ordering of operations (non-commutative effects)?
- How does traverse handle patterns where the accumulation of effects creates large intermediate structures?
- What happens when traversing patterns with different effect types (Option vs Result vs Future)?
- How does sequence handle patterns with nested effects (Pattern<Option<Result<T, E>>>)?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a `traverse` operation that applies an effectful function to all values in a pattern (root value and all element values recursively)
- **FR-002**: System MUST process values in depth-first order: root value first, then element values in order from left to right, recursing into each element
- **FR-003**: System MUST properly sequence and combine effects produced by the effectful function according to the effect type's semantics (e.g., short-circuit on first Error for Result, aggregate all effects for applicative functors)
- **FR-004**: System MUST provide a `sequence` operation that flips the layers of nested structures (e.g., `Pattern<Option<T>>` to `Option<Pattern<T>>`)
- **FR-005**: Traversable implementation MUST satisfy the identity law: `traverse(Identity::new) == Identity::new`
- **FR-006**: Traversable implementation MUST satisfy the composition law: for composable effects F and G, `traverse(Compose::new . fmap(g) . f) == Compose::new . fmap(traverse(g)) . traverse(f)`
- **FR-007**: Traversable implementation MUST satisfy the naturality law: for natural transformation t, `t . traverse(f) == traverse(t . f)`
- **FR-008**: System MUST provide `traverse_result()` method for Result types, providing short-circuit semantics (first error terminates traversal)
- **FR-009**: System MUST provide `traverse_option()` method for Option types, providing all-or-nothing semantics (first None terminates traversal)
- **FR-010**: System MUST provide `traverse_future()` method for async effect types, processing values sequentially (one at a time in order) [DEFERRED - requires async runtime]
- **FR-011**: System MUST provide `validate_all()` method that collects all errors encountered during traversal instead of short-circuiting (named `validate_all` to avoid conflict with existing structural `validate()` method)
- **FR-012**: System MUST process atomic patterns (patterns with no elements) by applying the effectful function only to the root value
- **FR-013**: System MUST preserve the order guarantee: for pattern with root value V and elements [E1, E2, E3], traverse must process V first, then values from E1, then values from E2, then values from E3
- **FR-014**: System MUST integrate cleanly with Functor (map) and Foldable (fold) operations to enable composition of functional patterns
- **FR-015**: Sequence operations (e.g., `sequence_option()`, `sequence_result()`) MUST be provided as convenience methods for flipping structure layers

### Key Entities *(include if feature involves data)*

- **Pattern<V>**: The pattern data structure with value type V that implements the Traversable trait
- **Pattern<F<W>>**: A pattern containing effect-wrapped values (e.g., Pattern<Option<i32>>, Pattern<Result<String, Error>>)
- **F<Pattern<W>>**: The result of sequencing - effect wrapping an entire pattern (e.g., Option<Pattern<i32>>, Result<Pattern<String>, Error>)
- **Effectful Function**: A function that maps values of type V to effectful values of type F<W> (e.g., `fn(V) -> Result<W, E>`)
- **Effect Type F**: The effect container (Option, Result, Future, etc.) that wraps computation results

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All traversable law property tests pass with at least 100 randomly generated test cases each (identity, composition, naturality laws)
- **SC-002**: Traverse operations complete on patterns with 1000 nodes in under 50 milliseconds (accounting for effect overhead)
- **SC-003**: Traverse operations complete on patterns with 100 nesting levels without stack overflow
- **SC-004**: Sequence operations correctly flip structure layers for patterns containing Option, Result, and other effect types
- **SC-005**: 100% of existing gram-hs traversable tests are ported and pass in Rust implementation
- **SC-006**: Traversable implementation compiles for WASM target without errors
- **SC-007**: `traverse_result()` method properly short-circuits on first error without processing remaining values (verifiable through side-effect counting)
- **SC-008**: `traverse_option()` method properly terminates on first None without processing remaining values
- **SC-012**: `validate_all()` method collects all errors from a pattern with multiple invalid values and reports them all
- **SC-009**: Pattern structures with 10,000 elements can be traversed without exceeding 100MB memory overhead
- **SC-010**: Traverse operations compose cleanly with map (Functor) and fold (Foldable) operations in test pipelines
- **SC-011**: Async traverse operations (with Future types) correctly initiate and collect results for all values in a pattern

## Assumptions

- Developers are familiar with functional programming concepts including Functor, Foldable, and effect types (Option, Result, Future)
- The Pattern type already exists with Functor (map) and Foldable (fold) implementations
- Rust's trait system can adequately represent Haskell's Traversable typeclass, potentially using multiple traits for different effect types
- Effect types (Option, Result, Future) provide appropriate sequencing/combining operations (e.g., Option's and_then, Result's and_then, Future's join)
- The implementation will follow Rust's idiomatic patterns for handling effects rather than direct Haskell translation
- Developers understand the difference between pure transformations (map) and effectful transformations (traverse)
- The implementation will use concrete methods for different effect types (traverse_option, traverse_result, traverse_future, validate) rather than a single generic trait, prioritizing Rust idioms and usability

## Dependencies

- Feature 004: Pattern Data Structure (must be complete - defines Pattern<V> type)
- Feature 005: Basic Pattern Type (must be complete - defines construction and access functions)
- Feature 008: Functor Instance (must be complete - provides map operation that traversable extends)
- Feature 009: Foldable Instance (must be complete - provides fold operation that traversable extends)
- Rust standard library support for Option, Result, and effect handling
- Potential external dependency for Future/async support (e.g., tokio, async-std) or use of std::future

## References

### Primary Reference (Authoritative)
- `../gram-hs/libs/pattern/src/Pattern/Core.hs` - Traversable instance implementation
- Haskell Traversable typeclass documentation and laws

### Test References
- `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` - Traversable law tests
- `../gram-hs/libs/pattern/tests/` - Traversable behavior tests

### Documentation Reference
- `../gram-hs/docs/` - Up-to-date documentation about the implementation
- Traversable instance documentation in Pattern/Core.hs provides detailed examples and semantics

### Historical Reference (Context Only)
- `../gram-hs/specs/007-traversable-instance/` - Historical notes from incremental development (may be outdated, verify against actual code)

## Design Decisions

### Effect Type Support Strategy

**Decision**: Use concrete methods for specific effect types (traverse_option, traverse_result, traverse_future)

**Rationale**: This approach is more Rust-idiomatic, easier to implement and use, and provides clear, type-safe APIs for each effect type. While less generic than a single trait with Applicative constraints, it offers better developer ergonomics and avoids complex higher-kinded type simulations. Users get dedicated methods like `traverse_result()`, `traverse_option()`, and `traverse_future()` that are discoverable and have clear type signatures.

### Async Execution Strategy

**Decision**: Sequential execution (process one value at a time in order)

**Rationale**: Sequential execution preserves strict ordering guarantees specified in FR-002, makes behavior easier to reason about, and matches synchronous semantics. While this sacrifices some performance compared to concurrent execution, it ensures predictable behavior and simpler error handling. The depth-first traversal order is maintained consistently across sync and async operations.

### Error Handling Strategy

**Decision**: Both approaches - separate methods for different use cases

**Rationale**: Provide two methods: `traverse_result()` for short-circuit behavior (fails fast on first error, matching typical Result semantics) and `validate_all()` for collecting all errors (provides comprehensive feedback for validation scenarios). This covers both performance-critical use cases (short-circuit) and user-feedback use cases (collect all errors) without forcing a single compromise. The `validate_all()` method is named with the `_all` suffix to avoid conflict with the existing `Pattern::validate(&ValidationRules)` method used for structural validation. The method will collect multiple errors using `Vec<E>` as the error type.
