# Feature Specification: WASM Feature Parity with Python and Pattern&lt;V&gt; TypeScript Generics

**Feature Branch**: `027-wasm-pattern-typescript-parity`  
**Created**: 2026-01-31  
**Status**: Draft  
**Input**: User description: "wasm feature parity with Python, including Pattern<V> generic type definitions for typescript"

## Clarifications

### Session 2026-01-31

- Q: How should fallible results be exposed for functional programming and effect-ts? A: Results that may fail must be trivially convertible for use in libraries like effect-ts that use an Either (see https://effect.website/docs/data-types/either/). The library should use functional programming techniques and expose fallible operations so they can be consumed as Either without extra glue.
- Q: Should fallible operations return Either-like values by default, or throw with a helper? A: Return Either-like by default. The Rust implementation already returns Result for fallible operations (e.g. validate, traverse_result). WASM bindings MUST preserve that at the boundary: fallible operations return a value that is trivially convertible to effect-ts Either (e.g. same shape as Either.right/Either.left or a documented one-line conversion), not throw. This matches Rust and avoids a separate helper.
- Q: Does this plan close the feature gap with the Python bindings, including paramorphism (para)? A: Yes. Parity MUST close the feature gap with the Rust pattern-core API. WASM/TypeScript MUST expose all pattern-core operations that exist in Rust, including paramorphism (para), so that the feature set matches the core library. Where Python (024) already exposes an operation, behavior MUST match Python; where an operation exists in Rust but not yet in Python (e.g. para), WASM MUST expose it so that the gap is closed and Python can be brought to parity later.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Construct Patterns in Browser/Node (Priority: P1)

Developers using the library from JavaScript or TypeScript (browser or Node) need to create Pattern instances programmatically, including atomic patterns and nested patterns with Subject values, matching what Python bindings offer. This enables building pattern-based data structures in web and Node applications without requiring a separate backend.

**Why this priority**: Construction is foundational. Without the ability to create patterns from JavaScript/TypeScript, no other operations are usable. This is the minimum viable product for WASM consumers.

**Independent Test**: Can be fully tested by loading the WASM module, creating a simple atomic pattern and a nested Pattern with Subject values, and verifying structure and values. Delivers immediate value by enabling web/Node developers to build pattern structures without Python.

**Acceptance Scenarios**:

1. **Given** a developer in a browser or Node environment, **When** they load the WASM module and call a constructor with a value, **Then** they receive a Pattern instance with that value and no elements
2. **Given** a developer wants to create a nested pattern, **When** they call a constructor with a value and a list of child patterns, **Then** they receive a Pattern instance with the value and child patterns as elements
3. **Given** a developer wants to create a Pattern with Subject value, **When** they construct a Subject (identity, labels, properties) and create a Pattern with it, **Then** they receive a correctly structured Pattern holding that Subject
4. **Given** a developer creates patterns, **When** they access pattern attributes (value, elements), **Then** they receive the expected JavaScript values (primitives, arrays, plain objects)

---

### User Story 2 - Perform Pattern Operations from WASM (Priority: P2)

Developers need to perform the same functional operations on patterns from JavaScript/TypeScript as are available in Python: transformations (map), queries (filter, find), structural analysis (depth, size), and combination operations. Behavior and outcomes must match the Python binding so that logic can be shared or ported across runtimes.

**Why this priority**: Operations are the core value. Parity with Python ensures developers can use one mental model and one set of docs regardless of runtime.

**Independent Test**: Can be fully tested by creating a pattern, applying map/filter/combine and structural queries, and comparing results to equivalent Python operations on the same logical data. Delivers value by enabling identical workflows in browser/Node and Python.

**Acceptance Scenarios**:

1. **Given** a Pattern instance from WASM, **When** a developer calls a map operation with a callback, **Then** a new Pattern is returned with transformed values and unchanged structure
2. **Given** a Pattern instance, **When** a developer calls filter (or equivalent) with a predicate callback, **Then** a new Pattern is returned containing only matching subpatterns
3. **Given** a Pattern instance, **When** a developer calls structural methods (depth, size, length), **Then** they receive correct numeric results consistent with Python
4. **Given** two Pattern instances, **When** a developer calls a combine operation, **Then** a new Pattern is returned with combined values and concatenated elements, matching Python behavior
5. **Given** a Pattern instance, **When** a developer calls paramorphism (para) with a function (pattern, element results), **Then** they receive an aggregated result computed bottom-up with behavior equivalent to Rust pattern-core para

---

### User Story 3 - TypeScript Pattern&lt;V&gt; Generics and Type Safety (Priority: P3)

TypeScript developers need generic type definitions (e.g. Pattern&lt;V&gt;) so that they can express pattern types over specific value types (e.g. Pattern&lt;Subject&gt;) and get correct inference and IDE support. Type definitions must cover the public surface exposed to JavaScript/TypeScript so that type checkers and editors provide accurate feedback.

**Why this priority**: Type safety improves correctness and developer experience. Generic Pattern&lt;V&gt; aligns with the core type model and enables type-safe usage without leaking implementation details.

**Independent Test**: Can be fully tested by writing TypeScript that uses Pattern&lt;Subject&gt; and other generic forms, running the type checker, and verifying no spurious errors and correct autocomplete/parameter types. Delivers value by enabling refactors and catching misuse at compile time.

**Acceptance Scenarios**:

1. **Given** a TypeScript developer imports pattern types, **When** they declare a variable as Pattern&lt;Subject&gt;, **Then** the type checker accepts it and infers correct types for value and elements
2. **Given** a TypeScript developer uses pattern operations (map, filter, combine), **When** they use generics, **Then** return types and callback parameter types are correctly inferred (e.g. map returns Pattern&lt;W&gt; when mapping V→W)
3. **Given** a TypeScript developer accesses pattern attributes, **When** they use typed APIs, **Then** IDE provides correct autocomplete and documentation for nested structures (e.g. Pattern&lt;Subject&gt; elements)
4. **Given** a TypeScript developer passes incorrect types to an operation, **When** they run the type checker, **Then** errors are reported before runtime, consistent with the declared generics

---

### Edge Cases

- What happens when a developer passes null/undefined or invalid types to constructors or callbacks?
- How does the system handle very deeply nested patterns (stack or recursion limits)?
- What happens when combining patterns with incompatible or mixed value types?
- Fallible operations (e.g. validate) return Either-like values at the boundary; how are error payloads (e.g. ValidationError) represented so they remain trivially convertible to Effect/Either (see Clarifications)?
- What happens when accessing elements on an atomic pattern (empty collection)?
- How are JavaScript callbacks (map/filter) handled across the WASM boundary (e.g. re-entrancy, exceptions)?
- How are Subject properties with complex value types (arrays, maps, ranges) represented and validated in JS/TS?
- How are JavaScript objects/arrays converted to and from the internal representation without losing type information for generics?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST expose constructors for creating Pattern instances (atomic and nested) to JavaScript/TypeScript consumers of the WASM module, with behavior equivalent to Python bindings
- **FR-002**: System MUST expose constructors for creating Subject instances (identity, labels, properties) to JavaScript/TypeScript, equivalent to Python
- **FR-003**: System MUST expose Pattern accessors (value, elements) to JavaScript/TypeScript with the same semantics as Python (read-only where applicable, same structure)
- **FR-004**: System MUST expose Pattern inspection methods (length, size, depth, is_atomic or equivalent) to JavaScript/TypeScript with results matching Python for the same inputs
- **FR-005**: System MUST expose Pattern query methods (filter, find_first, any_value, all_values, matches, contains or equivalents) to JavaScript/TypeScript with behavior equivalent to Python
- **FR-006**: System MUST expose Pattern transformation methods (map, fold or equivalent) to JavaScript/TypeScript with behavior equivalent to Python
- **FR-007**: System MUST expose Pattern combination method (combine or equivalent) to JavaScript/TypeScript with behavior equivalent to Python
- **FR-008**: System MUST expose Pattern comonad operations (extract, extend or equivalents) to JavaScript/TypeScript where they exist in Python, with equivalent behavior
- **FR-017**: System MUST expose Pattern paramorphism (para or equivalent) to JavaScript/TypeScript with behavior equivalent to Rust pattern-core (025-pattern-paramorphism): a method that accepts a function (pattern, array of element results) and returns an aggregated result, bottom-up order, so that the feature gap with the Rust API is closed (see Clarifications)
- **FR-009**: For fallible operations, system MUST surface results as Either-like return values (see FR-016). For any remaining error paths that throw (e.g. programmer errors), system MUST use clear, actionable messages
- **FR-016**: System MUST expose fallible operations by returning an Either-like value at the WASM/JS boundary (not by throwing). The Rust implementation already returns Result for fallible operations (e.g. validate, traverse_result); WASM bindings MUST preserve that: return a value that is trivially convertible to effect-ts Either (e.g. same shape as Either.right/Either.left or a documented one-line conversion) so that integration with effect-ts requires no helper
- **FR-010**: System MUST provide TypeScript type definitions for all public APIs exposed to JavaScript/TypeScript, including generic types
- **FR-011**: TypeScript definitions MUST include a generic Pattern&lt;V&gt; type (or equivalent naming) so that Pattern&lt;Subject&gt; and other value types can be expressed
- **FR-012**: System MUST convert between JavaScript/TypeScript and core representation correctly for value types (primitives, arrays, plain objects, Subject, Value enum variants) so that round-trips preserve meaning
- **FR-013**: System MUST support the same Subject Value types (string, int, decimal, boolean, symbol, array, map, range, measurement) in JavaScript/TypeScript as in Python
- **FR-014**: System MUST preserve pattern structure invariants (recursive nesting, value-element pairing) across the WASM boundary so that results match Python for the same logical input
- **FR-015**: System MUST support JavaScript/TypeScript callbacks (functions passed to map, filter, etc.) with behavior and type signatures consistent with the generic Pattern&lt;V&gt; model

### Key Entities *(include if feature involves data)*

- **Pattern&lt;V&gt;** (in spec, “Pattern over V”): A recursive, nested structure generic over value type V. In TypeScript, this is expressed as a generic type (e.g. Pattern&lt;V&gt;) so that Pattern&lt;Subject&gt; and other concrete value types can be used with correct typing.
- **Subject**: A self-descriptive value type with identity (Symbol), labels (set of strings), and properties (map of string to Value). Exposed to JavaScript/TypeScript with the same structure as in Python.
- **Value**: An enum representing property value types (string, int, decimal, boolean, symbol, array, map, range, measurement). Exposed to TypeScript with types that allow discrimination and type-safe access.
- **Symbol**: A wrapper around string for identifiers. Exposed to JavaScript/TypeScript in a way that aligns with Python (string or small wrapper).
- **TypeScript type definitions**: Declarations (.d.ts or equivalent) that describe the public WASM API, including generic Pattern&lt;V&gt;, so that type checkers and IDEs provide accurate support.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can create a Pattern holding Subject with 3 levels of nesting from JavaScript/TypeScript in under 5 lines of code, matching Python ergonomics
- **SC-002**: Developers can perform the same set of pattern operations (map, filter, combine, para, depth, size, comonad operations, etc.) from JavaScript/TypeScript as from the Rust pattern-core API, with equivalent results for the same logical input; where Python exposes an operation, behavior matches Python; where Rust has an operation not yet in Python (e.g. para), WASM exposes it so the feature gap is closed
- **SC-003**: Type checker reports zero false positives when using the provided TypeScript definitions with Pattern&lt;Subject&gt; and Pattern&lt;V&gt; on a sample program with 20+ operations
- **SC-004**: Developers can complete a data transformation workflow (create pattern, map values, filter subpatterns, combine results) from JavaScript/TypeScript in under 2 minutes, with no need to consult Python-only docs for behavior
- **SC-005**: IDE autocomplete and parameter hints are correct for Pattern&lt;V&gt; and related types for at least 95% of public API surface
- **SC-006**: WASM-based pattern operations stay within 2x of Python binding performance for patterns with up to 1000 nodes, so that parity includes practical performance
- **SC-007**: Round-trip of a Pattern from core representation to JavaScript/TypeScript and back (or equivalent cross-boundary usage) preserves structure and value types without data loss
- **SC-008**: Error messages surfaced to JavaScript/TypeScript are clear and actionable for web/Node developers (no requirement to understand the underlying implementation)
- **SC-009**: Fallible operations return a value that is trivially convertible to effect-ts Either (return shape compatible with Either.right/Either.left or a documented one-line conversion); no throw-by-default for those operations

## Assumptions

- The core pattern implementation is stable and already targets or can target WASM; this feature is about API surface and type definitions, not reimplementing core logic
- Feature parity means closing the feature gap with the Rust pattern-core API: WASM/TypeScript MUST expose the same set of operations as the Rust core (including para), with equivalent behavior. Where Python (024) exposes an operation, behavior matches Python; where an operation exists in Rust but not yet in Python (e.g. para), WASM exposes it (see Clarifications).
- TypeScript consumers expect generic types (Pattern&lt;V&gt;) and type definitions that work with standard type checkers and editors
- JavaScript-only consumers can use the same WASM API; TypeScript definitions are additive and do not require TypeScript at runtime
- Callbacks passed from JavaScript/TypeScript into WASM (e.g. for map/filter) are synchronous; async behavior is out of scope unless already part of Python parity
- The existing Python binding (024-python-pattern-core) defines the reference behavior for operations it exposes; Rust pattern-core (including 025-pattern-paramorphism) defines the full set of operations that MUST be exposed for parity
- WASM bundle size and load time are important but secondary to correctness and parity; optimization is in scope only as a non-breaking follow-on

## Dependencies

- Core pattern implementation (pattern-core or equivalent) must be available and WASM-compilable
- Python bindings (024-python-pattern-core) exist as the parity reference
- Build tooling can produce a WASM artifact and TypeScript declaration files for the public API
- TypeScript type definitions can be maintained by hand or generated from a single source of truth, as long as Pattern&lt;V&gt; and related generics are correctly expressed

## Out of Scope

- Implementing new core pattern operations that do not exist in Rust pattern-core; this feature is parity with the Rust API (including para) and TypeScript generics only. Operations that exist in Rust (e.g. para) MUST be exposed in WASM even if not yet in Python
- Python API changes; parity is one-way (WASM/TS matches Python)
- Async/await or Promise-based APIs unless they already exist in Python
- Framework-specific bindings (React, Vue, etc.); the spec is for the core WASM + TypeScript API
- Non-browser, non-Node WASM targets (e.g. other embedders) unless they fall under “WASM” as used in the project
- Changing the core Rust type system or generic bounds; only the JavaScript/TypeScript-facing types and definitions are in scope
