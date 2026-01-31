# Feature Specification: Paramorphism in Python Bindings

**Feature Branch**: `026-paramorphism-python-bindings`  
**Created**: 2026-01-31  
**Status**: Draft  
**Input**: User description: "Python bindings should include the new paramorphism"

## Clarifications

### Session 2026-01-31

- Q: Do we need a separate `PatternSubject` class in Python, or can we use the generic `Pattern` with `Subject` as the value and simplify the design? → A: Simplify. Expose paramorphism on the generic `Pattern` class only. When values are Subjects, use `Pattern` with Subject as the value type. No separate `PatternSubject` class is required for this feature.
- Q: What must hold when migrating from PatternSubject to Pattern with Subject as value? → A: All existing PatternSubject tests must still pass when changed to use `Pattern[Subject]` (same behavior, same assertions).

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Structure-Aware Aggregation from Python (Priority: P1)

As a Python developer analyzing pattern structures, I want to run paramorphism (structure-aware folding) on patterns from Python, so I can aggregate over element sequences (e.g., detect A–B–A patterns, compute depth-weighted or element-count-aware statistics) without dropping to Rust.

**Why this priority**: Paramorphism is the main way to do structure-aware analysis over patterns. Exposing it in Python makes pattern-of-elements analysis available to Python users and keeps parity with the Rust API.

**Independent Test**: Can be tested by building a pattern in Python, calling the paramorphism operation with a function that uses both the current pattern and element results, and asserting the result matches the expected aggregation (e.g., depth-weighted sum or multi-statistics in one pass).

**Acceptance Scenarios**:

1. **Given** a Pattern instance created in Python with nested elements, **When** the developer calls the paramorphism operation with a callable that receives the current pattern and a sequence of element results, **Then** the result is the same as the equivalent operation in Rust (e.g., depth-weighted sum, or sum/count/max-depth in one pass).
2. **Given** an atomic pattern (no elements), **When** the developer calls paramorphism, **Then** the callable receives that pattern and an empty sequence of element results, and the result is the callable’s return value for that base case.
3. **Given** a pattern and a paramorphism callable that combines the current value with element results (e.g., value + sum(element_results)), **When** the developer runs paramorphism, **Then** the result equals the result of the equivalent fold over values (parity with fold for value-only aggregation).

---

### User Story 2 - Element-Count and Depth in One Pass (Priority: P2)

As a Python developer, I want to compute statistics that depend on structure (e.g., element count per node, depth) in a single traversal via paramorphism, so I can analyze patterns efficiently without multiple passes.

**Why this priority**: Single-pass structure-aware aggregation is a core benefit of paramorphism; Python users need the same capability.

**Independent Test**: Can be tested by building patterns with known structure, running paramorphism to compute (e.g.) sum, node count, and max depth in one call, and asserting the tuple matches expected values.

**Acceptance Scenarios**:

1. **Given** a pattern with known sum, node count, and max depth, **When** the developer uses paramorphism to compute (sum, count, max_depth) in one traversal, **Then** the returned tuple matches the expected values.
2. **Given** a paramorphism callable that uses the current pattern’s element count or depth, **When** the developer runs it on a nested pattern, **Then** the result correctly reflects the structure at each node.

---

### User Story 3 - Structure-Preserving Transformation from Python (Priority: P2)

As a Python developer, I want to use paramorphism to build a new pattern from the current pattern and transformed element results (structure-preserving transformation), so I can express “map over structure” style operations in Python.

**Why this priority**: Structure-preserving transformation (e.g., scaling values by depth) is a natural use of paramorphism and should be available in Python.

**Independent Test**: Can be tested by applying a paramorphism that returns a new pattern (same structure, transformed values), then checking that the resulting pattern has the expected shape and values.

**Acceptance Scenarios**:

1. **Given** a pattern and a paramorphism callable that returns a new pattern node from the current pattern and a list of result patterns, **When** the developer runs paramorphism, **Then** the result is a pattern with the same structure and values determined by the callable (e.g., value * (depth + 1)).
2. **Given** an atomic pattern, **When** the developer uses paramorphism for structure-preserving transformation, **Then** the callable receives the pattern and an empty list of element results and returns the transformed leaf pattern.

---

### User Story 4 - Type-Safe Paramorphism in Python (Priority: P3)

As a Python developer using type checkers, I want paramorphism to be exposed with clear type hints (callable signature, return type) so that static checking and IDE support work for paramorphism calls.

**Why this priority**: Type safety improves correctness and usability but is not required for basic behavior.

**Independent Test**: Can be tested by writing paramorphism calls with type annotations and running mypy/pyright without type errors.

**Acceptance Scenarios**:

1. **Given** a Pattern instance and a paramorphism callable with typed parameters and return, **When** the developer runs a type checker, **Then** no type errors are reported for the paramorphism API.
2. **Given** the paramorphism method in an IDE, **When** the developer inspects its signature, **Then** they see the expected parameter types (pattern view, sequence of element results) and return type.

---

### Edge Cases

- What happens when the Python callable raises an exception? The system should propagate it (or convert from Rust errors) in a way that is consistent with other Python bindings (e.g., fold/map).
- What happens with very deep patterns? Behavior should be consistent with Rust (e.g., stack limits); documentation should note any practical depth limits.
- What happens when the callable returns a type that does not match other nodes (e.g., mixed types in structure-preserving para)? Behavior and types should be well-defined and documented (e.g., homogeneous result type or explicit rules).
- What happens when the callable is None or not callable? The system should reject invalid input with a clear error (e.g., TypeError), consistent with other Python operations.
- What happens when element results are used in a way that assumes order? Element results should be provided in a well-defined order (e.g., depth-first, left-to-right) and documented so that order-sensitive aggregations are reliable.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST expose a paramorphism operation on the generic Pattern class in Python that accepts a callable and returns an aggregated result. When values are Subjects, Pattern is used with Subject as the value type; no separate PatternSubject class is required.
- **FR-002**: The callable MUST receive two arguments: a representation of the current pattern (value and structure information as needed) and a sequence of results from applying the operation recursively to its elements.
- **FR-003**: System MUST compute element results recursively before invoking the callable for the current node (bottom-up evaluation).
- **FR-004**: System MUST provide element results in a consistent, documented order (e.g., depth-first, left-to-right) so that order-sensitive aggregations are reliable.
- **FR-005**: System MUST support paramorphism that returns a single aggregated value (e.g., number, tuple) and paramorphism that returns a new pattern (structure-preserving transformation), with behavior and types documented.
- **FR-006**: System MUST handle atomic patterns (no elements) by passing an empty sequence of element results to the callable.
- **FR-007**: System MUST allow the callable to access structural information (e.g., depth, element count) for the current pattern where the core API supports it.
- **FR-008**: System MUST convert any Rust errors arising from paramorphism into appropriate Python exceptions, consistent with other Python bindings.
- **FR-009**: System MUST document the relationship between paramorphism and fold (e.g., when para can replicate fold) so users can choose the right operation.
- **FR-010**: System MUST provide type hints or stubs for the paramorphism API so type checkers and IDEs can validate usage.
- **FR-011**: When PatternSubject is removed or callers migrate to Pattern with Subject as value, all existing tests that previously used PatternSubject MUST still pass when rewritten to use `Pattern[Subject]` (same behavior and assertions; migration is a refactor, not a behavior change).

### Key Entities

- **Pattern (Python)**: The generic pattern type exposed in Python bindings; it holds a value (any type, including Subject) and zero or more element patterns. Paramorphism is an operation on this single class. No separate PatternSubject class is required—use Pattern with Subject as the value type when needed.
- **Paramorphism callable**: A user-provided Python callable that takes (current pattern view, sequence of element results) and returns either an aggregated value or a new pattern node, depending on use case.
- **Element results**: The ordered sequence of results from recursively applying the paramorphism operation to each element of the current pattern; passed into the callable for the current node.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Python developers can compute depth-weighted sums (or equivalent structure-aware aggregations) for patterns of any supported depth using a single paramorphism call, with results matching the Rust implementation for the same inputs.
- **SC-002**: Python developers can compute multiple statistics (e.g., sum, node count, max depth) in one paramorphism traversal and get results consistent with the Rust implementation.
- **SC-003**: Python developers can perform structure-preserving transformations via paramorphism (same shape, transformed values) and obtain a pattern that matches the expected structure and values.
- **SC-004**: Paramorphism in Python preserves the relationship with fold: for a callable that only combines value and sum of element results, the result equals the result of the equivalent fold over values.
- **SC-005**: Documentation and examples allow a Python developer to complete a structure-aware aggregation (e.g., from the Rust paramorphism examples) in under 10 minutes without prior knowledge of the Rust API.
- **SC-006**: Type checkers (mypy or pyright) validate correctly typed paramorphism usage without errors; incorrect argument types are reported.
- **SC-007**: All existing PatternSubject tests pass when changed to use `Pattern[Subject]` (Pattern with Subject as the value type); no test behavior or assertions are relaxed.

## Assumptions

- The Rust pattern-core already implements paramorphism (see spec 025-pattern-paramorphism); this feature is limited to exposing it in the existing Python bindings.
- The Python API is simplified to a single generic Pattern class; Subject is used as the value type when needed. A separate PatternSubject class is not required for paramorphism (see Clarifications).
- The Python bindings follow the same design as other pattern-core operations (e.g., fold, map): a method on the Pattern class that accepts a Python callable and may cross the Python–Rust boundary.
- Structural properties (depth, element count) needed by paramorphism are already exposed or can be exposed on the pattern view passed to the callable, consistent with the Rust API.
- Stack or recursion limits for deep patterns in Python are acceptable if they align with Rust behavior; no separate iterative implementation is required for this feature.
- Parity with the Rust paramorphism semantics (order, base case, type of folding function) is the target; any intentional differences must be documented.

## Out of Scope

- Implementing paramorphism in Rust (already in scope of 025).
- Introducing or depending on a separate PatternSubject class for paramorphism; the design uses the generic Pattern class only.
- Adding new Rust APIs solely for Python; exposure uses the existing paramorphism API.
- Lazy or streaming paramorphism; behavior is eager and consistent with Rust.
- Parallel or concurrent execution of element computations.
- Memoization or caching of intermediate results.
