# Feature Specification: Python Pattern-Core Bindings

**Feature Branch**: `024-python-pattern-core`  
**Created**: 2026-01-27  
**Status**: Draft  
**Input**: User description: "Make pattern-core available in python, to enable programmatic construction and operations on pattern subjects. Consider best option for type safety"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Construct Patterns Programmatically (Priority: P1)

Python developers need to create Pattern instances programmatically from Python code, including both atomic patterns and nested patterns with Subject values. This enables building pattern-based data structures without requiring gram notation parsing.

**Why this priority**: Construction is foundational - without the ability to create patterns, no other operations are possible. This is the minimum viable product that delivers immediate value.

**Independent Test**: Can be fully tested by creating a simple atomic pattern and a nested pattern with Subject values, verifying structure and values are correct. Delivers immediate value by enabling Python developers to build pattern structures programmatically.

**Acceptance Scenarios**:

1. **Given** a Python developer wants to create an atomic pattern, **When** they call a constructor function with a value, **Then** they receive a Pattern instance with that value and no elements
2. **Given** a Python developer wants to create a nested pattern, **When** they call a constructor function with a value and a list of child patterns, **Then** they receive a Pattern instance with the value and child patterns as elements
3. **Given** a Python developer wants to create a Pattern with Subject value, **When** they construct a Subject with identity, labels, and properties, then create a Pattern with it, **Then** they receive a Pattern<Subject> instance with the correct structure
4. **Given** a Python developer creates patterns, **When** they access pattern attributes (value, elements), **Then** they receive the expected Python values (strings, lists, dictionaries)

---

### User Story 2 - Perform Pattern Operations (Priority: P2)

Python developers need to perform functional programming operations on patterns, including transformations (map), queries (filter, find), structural analysis (depth, size), and combination operations. This enables data processing workflows in Python.

**Why this priority**: Operations are the core value proposition - they enable practical use cases like data transformation, querying, and analysis. However, construction must come first.

**Independent Test**: Can be fully tested by creating a pattern, applying map/filter operations, and verifying results match expected transformations. Delivers value by enabling functional programming workflows on pattern data.

**Acceptance Scenarios**:

1. **Given** a Pattern instance, **When** a developer calls a map operation with a transformation function, **Then** a new Pattern is returned with transformed values while preserving structure
2. **Given** a Pattern instance, **When** a developer calls filter with a predicate function, **Then** a new Pattern is returned containing only matching subpatterns
3. **Given** a Pattern instance, **When** a developer calls structural analysis methods (depth, size, length), **Then** they receive correct numeric results describing the pattern structure
4. **Given** two Pattern instances, **When** a developer calls combine operation, **Then** a new Pattern is returned with combined values and concatenated elements

---

### User Story 3 - Type-Safe Python Development (Priority: P3)

Python developers using type checkers (mypy, pyright) need type hints and annotations that enable static type checking and IDE autocomplete. This improves developer experience and reduces runtime errors.

**Why this priority**: Type safety improves developer experience and code quality, but is not required for basic functionality. It's an enhancement that makes the library production-ready.

**Independent Test**: Can be fully tested by writing Python code with type annotations, running mypy/pyright, and verifying no type errors occur. Delivers value by enabling IDE autocomplete and catching type errors before runtime.

**Acceptance Scenarios**:

1. **Given** a Python developer imports pattern-core types, **When** they use type annotations in their code, **Then** type checkers (mypy/pyright) validate the types correctly
2. **Given** a Python developer uses pattern-core in an IDE, **When** they type a pattern method name, **Then** they see autocomplete suggestions with correct parameter types and return types
3. **Given** a Python developer passes incorrect types to pattern operations, **When** they run type checking, **Then** type errors are reported before runtime
4. **Given** a Python developer accesses pattern attributes, **When** they use type hints, **Then** IDE provides correct type information for nested structures (Pattern<Subject>)

---

### Edge Cases

- What happens when a Python developer passes None/null values to constructors?
- How does the system handle very deeply nested patterns (stack overflow protection)?
- What happens when combining patterns with incompatible value types?
- How are Python exceptions converted from Rust errors (ValueError, TypeError)?
- What happens when accessing elements on an atomic pattern (should return empty list)?
- How are Python callbacks (map/filter functions) handled for type safety?
- What happens with Subject properties containing complex Value types (arrays, maps, ranges)?
- How are Python sets/lists converted to Rust HashSet/Vec and vice versa?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide Python constructors for creating Pattern instances (atomic and nested)
- **FR-002**: System MUST provide Python constructors for creating Subject instances with identity, labels, and properties
- **FR-003**: System MUST expose Pattern accessor methods (value, elements) as Python attributes or methods
- **FR-004**: System MUST expose Pattern inspection methods (length, size, depth, is_atomic) as Python methods
- **FR-005**: System MUST expose Pattern query methods (filter, find_first, any_value, all_values, matches, contains) as Python methods
- **FR-006**: System MUST expose Pattern transformation methods (map, fold) as Python methods
- **FR-007**: System MUST expose Pattern combination method (combine) as Python method
- **FR-008**: System MUST expose Pattern comonad operations (extract, extend) as Python methods
- **FR-009**: System MUST convert Rust errors to appropriate Python exceptions (ValueError, TypeError)
- **FR-010**: System MUST provide type hints/stubs (.pyi files) for all public Python APIs
- **FR-011**: System MUST support Python type checkers (mypy, pyright) for static type validation
- **FR-012**: System MUST convert between Python and Rust types correctly (str↔String, list↔Vec, dict↔HashMap, set↔HashSet)
- **FR-013**: System MUST handle Subject Value types (VString, VInt, VDecimal, VBoolean, VSymbol, VArray, VMap, VRange, VMeasurement) in Python
- **FR-014**: System MUST preserve pattern structure invariants (recursive nesting, value-element pairing) across Python-Rust boundary
- **FR-015**: System MUST support Python callbacks (functions passed to map/filter) with proper type checking

### Key Entities

- **Pattern**: A recursive, nested structure (s-expression-like) generic over value type V. In Python, this is exposed as a class that can hold any value type or specifically Pattern<Subject>.
- **Subject**: A self-descriptive value type with identity (Symbol), labels (set of strings), and properties (map of string to Value). In Python, this is exposed as a class with typed attributes.
- **Value**: An enum representing property value types (string, int, decimal, boolean, symbol, array, map, range, measurement). In Python, this is exposed as a union type or enum.
- **Symbol**: A wrapper around string representing an identifier. In Python, this is exposed as a simple string or Symbol class.
- **Python Type Stubs**: Type hint files (.pyi) that describe the Python API for type checkers and IDEs.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Python developers can create a Pattern<Subject> with 3 levels of nesting in under 5 lines of Python code
- **SC-002**: Python developers can perform 10 common pattern operations (map, filter, combine, depth, etc.) without consulting documentation
- **SC-003**: Type checkers (mypy/pyright) report zero type errors when used with pattern-core Python bindings on a sample program with 20+ operations
- **SC-004**: Python developers can complete a data transformation workflow (create pattern, map values, filter subpatterns, combine results) in under 2 minutes
- **SC-005**: IDE autocomplete provides correct suggestions for 95% of pattern-core method calls and attributes
- **SC-006**: Python bindings maintain performance within 2x of native Rust operations for patterns with up to 1000 nodes
- **SC-007**: Python developers can successfully convert between Python native types (dict, list, set) and pattern-core types without manual conversion code
- **SC-008**: Error messages from Rust are converted to Python exceptions that are clear and actionable for Python developers (not Rust-specific terminology)

## Assumptions

- Python developers are familiar with functional programming concepts (map, filter, fold)
- Python developers use Python 3.8 or later (type hints support)
- Type safety is achieved through Python type hints (.pyi files) and runtime validation, not compile-time checking
- PyO3 is the preferred binding approach (consistent with gram-codec implementation)
- Python developers may use type checkers (mypy, pyright) but it's not required for basic usage
- Pattern operations that accept callbacks (map, filter) will use Python callable types with appropriate type hints
- Subject properties can contain nested Value types (arrays, maps) that need recursive conversion
- Python developers expect Pythonic APIs (snake_case methods, dict/list/set types) rather than Rust-style APIs

## Dependencies

- Existing pattern-core Rust crate must be stable and complete
- PyO3 library for Rust-Python bindings
- Python packaging tools (maturin) for building Python wheels
- Type stubs generation or manual creation for .pyi files
- Python type checking tools (mypy, pyright) for validation

## Out of Scope

- Pure Python implementation of Pattern (this is about bindings to Rust)
- WASM-based Python bindings (PyO3 is the chosen approach)
- Python 2.x support (Python 3.8+ only)
- Runtime type checking beyond what PyO3 provides (static type checking via .pyi files)
- Automatic conversion of all Rust types (only pattern-core public API types)
- Python async/await support (synchronous operations only)
- Custom Python metaclasses or advanced Python features
