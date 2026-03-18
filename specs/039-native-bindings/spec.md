# Feature Specification: Native TypeScript and Python Bindings

**Feature Branch**: `039-native-bindings`
**Created**: 2026-03-17
**Status**: Draft
**Input**: User description: "Refactor Python & TS to be pure implementations, reducing the use of Rust to the gram-codec parser/serializer, as described in @proposals/migrate-ts-python-proposal.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Work with Pattern as a real native object (Priority: P1)

A TypeScript developer parses gram notation and receives `Pattern<Subject>` objects they can inspect, compare, and traverse using familiar language tools — debugger, type checker, equality operators — without any awareness of an underlying native extension.

**Why this priority**: The most common use of the library is parsing gram and working with the result. If that experience is broken or awkward, nothing else matters. This is also the most direct improvement over the current state, where Pattern objects are opaque handles.

**Independent Test**: Parse a gram string and verify the returned Pattern objects can be compared with structural equality, inspected in a debugger, and traversed without async setup.

**Acceptance Scenarios**:

1. **Given** a gram string `"(a:Person {name: 'Alice'})"`, **When** a developer parses it, **Then** they receive a `Pattern` object whose fields (`identity`, `labels`, `properties`) are directly readable without calling any accessor methods or crossing a native boundary.
2. **Given** two separately parsed `Pattern` objects representing the same gram structure, **When** compared for equality, **Then** they are considered equal without the developer writing a custom comparator.
3. **Given** a `Pattern` object, **When** inspected in a debugger or `console.log`, **Then** its structure is shown as plain data — not as an opaque pointer or native handle.
4. **Given** a `Pattern` with nested elements, **When** `map`, `fold`, or `filter` is called, **Then** the operation completes synchronously without any async steps or native bridge overhead.

---

### User Story 2 - Work with Pattern as a real Python object (Priority: P1)

A Python developer receives `Pattern` objects from parsing that behave like standard Python dataclasses — printable, comparable, introspectable, and traversable with pure Python logic — without PyO3 overhead on every tree operation.

**Why this priority**: Tied with Story 1 in priority; the Python and TypeScript improvements are symmetric and both are core to the feature. Ordering is arbitrary between them.

**Independent Test**: Parse gram in Python and verify the returned objects can be printed, compared with `==`, iterated, and folded using a Python lambda without any native extension calls beyond the initial parse.

**Acceptance Scenarios**:

1. **Given** a gram string, **When** a Python developer parses it, **Then** they receive `Pattern` objects with directly accessible fields that `repr()` and `print()` correctly.
2. **Given** two `Pattern` objects representing the same structure, **When** compared with `==`, **Then** they are equal.
3. **Given** a `Pattern`, **When** `fold` is called with a Python lambda, **Then** the lambda executes purely in Python — no round-trip to a native extension per node.
4. **Given** a large deeply-nested `Pattern`, **When** `fold` or `map` is called, **Then** performance is faster than the equivalent operation through the current native extension binding.

---

### User Story 3 - Gram parsing surfaces errors explicitly (Priority: P2)

A developer calling the gram parser receives a structured error value — not an uncaught exception — when given invalid or unparseable input, and can handle it without a try/catch block.

**Why this priority**: Explicit error handling is a significant quality-of-life improvement for library consumers, but the library is still usable (with try/catch) without it. It is important but not blocking.

**Independent Test**: Pass invalid gram input to the parser and verify the returned value carries error information without throwing, allowing the caller to branch on success or failure.

**Acceptance Scenarios**:

1. **Given** a syntactically invalid gram string, **When** parsed, **Then** the return value carries a structured description of the failure without throwing an exception.
2. **Given** a valid gram string, **When** parsed, **Then** the return value carries the parsed patterns and no error.
3. **Given** a parse error value, **When** inspected, **Then** it includes the original input and a human-readable description of what went wrong.

---

### User Story 4 - Maintainer adds a Pattern operation without touching Rust (Priority: P2)

A library maintainer adds a new traversal or query operation on `Pattern` entirely in TypeScript or Python, without modifying any Rust code, rebuilding any native extension, or bumping the Rust version.

**Why this priority**: One of the key motivations for the refactor is to reduce the maintenance surface. Demonstrating that new operations can be added purely in the host language validates that motivation.

**Independent Test**: Add a new operation (e.g., `depth`, `values`) to the native TypeScript or Python Pattern without modifying any Rust source, and verify it passes tests.

**Acceptance Scenarios**:

1. **Given** the native TypeScript Pattern implementation, **When** a maintainer adds a new method or standalone function, **Then** they can do so without any Rust changes, rebuild, or wasm-pack invocation.
2. **Given** the native Python Pattern implementation, **When** a maintainer adds a new method, **Then** they can do so without `maturin develop` or any PyO3 changes.

---

### User Story 5 - StandardGraph constructed from parsed patterns (Priority: P3)

A developer parses gram notation and constructs a `StandardGraph` — a graph that classifies patterns as nodes, relationships, walks, or annotations — using purely native code after the initial parse.

**Why this priority**: StandardGraph is built on top of Pattern; it depends on Stories 1 and 2 being complete. It is independently valuable but can be deferred.

**Independent Test**: Parse a gram string containing nodes and relationships, construct a `StandardGraph`, and verify correct classification without any native bridge calls after the parse step.

**Acceptance Scenarios**:

1. **Given** a gram string with node and relationship patterns, **When** a `StandardGraph` is constructed from the parse result, **Then** nodes and relationships are correctly classified and accessible.
2. **Given** a `StandardGraph`, **When** queried for its nodes, **Then** it returns the correct `Pattern<Subject>` objects.
3. **Given** an empty gram string, **When** a `StandardGraph` is constructed, **Then** it contains no nodes or relationships.

---

### Edge Cases

- What happens when a gram string is empty? The parser should return an empty pattern list, not an error.
- What happens when a gram string parses successfully but contains a `Value` type not recognized by the decoder? The decoder should surface a structured error rather than silently dropping the value.
- What happens when `fold` or `map` is called on a deeply-nested pattern that exceeds typical recursion depth? The behavior (stack overflow vs. iterative fallback) should be documented.
- What happens when a developer compares a native `Pattern` object with an object from an older version of the library that used opaque WASM handles? The result should be a clear type mismatch, not a silent false equality.

---

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: The library MUST expose `Pattern`, `Subject`, `Value`, and `StandardGraph` as native objects in both TypeScript and Python — not as wrappers around a native extension handle.
- **FR-002**: The library MUST provide structural equality comparison for `Pattern`, `Subject`, and `Value` without requiring custom comparators.
- **FR-003**: All `Pattern` traversal operations (`map`, `fold`, `filter`, `findFirst`) MUST execute without crossing a native extension boundary after the initial parse.
- **FR-004**: The gram parser MUST return a structured error value (not throw) when given invalid input.
- **FR-005**: The library MUST continue to expose gram parse, stringify, and validate capabilities backed by the existing Rust codec — only the data structure layer moves to native code.
- **FR-006**: The native TypeScript and Python `Pattern` implementations MUST produce behavior that matches the gram-hs Haskell reference for `fold` traversal order, `map`, `filter`, `extend`, and `extract`.
- **FR-007**: The gram-hs Haskell reference (`../pattern-hs/libs/`) MUST be reviewed before implementing operations to confirm traversal semantics, `Value` variant completeness, and `StandardGraph` classification rules.
- **FR-008**: The `Value` type MUST represent all variants defined in the gram-hs reference (at minimum: string, integer, float, boolean, null, symbol).
- **FR-009**: `StandardGraph` MUST correctly classify patterns as nodes, relationships, walks, or annotations using the same rules as the gram-hs reference classifier.
- **FR-010**: The library's published package interface MUST remain backward-compatible for existing consumers — parse/stringify/validate entry points, `Pattern`, `Subject`, `Value`, and `StandardGraph` must all remain accessible under the same import paths.
- **FR-011**: The native Rust extension MUST be reduced to expose only gram codec operations (parse, stringify, validate) with no `Pattern`, `Subject`, `Value`, or `StandardGraph` types crossing the native boundary.

### Key Entities

- **Pattern**: A recursive tree structure with a typed value at each node and an ordered list of child patterns. The foundational data type; all graph operations are defined in terms of it.
- **Subject**: A self-describing value with a unique identity, a set of labels, and a map of named properties. Used as the value type in `Pattern<Subject>`.
- **Value**: A tagged union representing the primitive value types that can appear as properties on a `Subject`: string, integer, float, boolean, null, and symbol.
- **StandardGraph**: A graph constructed from a collection of `Pattern<Subject>` objects, classified into nodes, relationships, walks, and annotations based on structural rules.
- **Gram codec**: The Rust parser and serializer for gram notation. Remains in Rust; all other entities move to native host-language implementations.

---

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All existing tests in the TypeScript and Python test suites pass against the new native implementations with no behavior regressions.
- **SC-002**: `Pattern` operations (`map`, `fold`, `filter`) on a tree of 10,000 nodes complete at least 5× faster in the native implementation than in the current WASM-bridge implementation.
- **SC-003**: A Python `fold` over a 1,000-node tree completes at least 10× faster than the current PyO3 round-trip implementation (eliminating per-node GIL-held callbacks).
- **SC-004**: The compiled native Rust extension (WASM binary) is at least 40% smaller than the current build, measured by compressed binary size, because `pattern-core` types are no longer compiled in.
- **SC-005**: A new `Pattern` operation can be added to the TypeScript or Python implementation without modifying any Rust source file.
- **SC-006**: The native TypeScript and Python implementations pass a property-based test suite that verifies functor laws for `map`, catamorphism laws for `fold`, and comonad laws for `extend`/`extract` against the gram-hs reference behavior.
- **SC-007**: `Pattern` and `Subject` objects are directly introspectable in standard debuggers (browser DevTools for TypeScript, pdb/debugpy for Python) without opaque handles or accessor indirection.

---

## Assumptions

- The gram-hs Haskell reference implementation at `../pattern-hs/libs/` is the authoritative source for operation semantics, `Value` variants, and `StandardGraph` classification rules. Any ambiguity in the Rust implementation is resolved by consulting gram-hs.
- The existing TypeScript package (`@relateby/pattern`) and Python package (`relateby-pattern`) import paths are preserved. No major-version bump is required because the observable behavior is unchanged; only the internal implementation layer changes.
- The effect-ts library (`effect`) is an acceptable peer dependency for the TypeScript implementation. It provides structural equality, typed error handling, schema-based validation, and functional composition utilities.
- Python `dataclasses` and standard library types (`set`, `dict`, `list`) are sufficient for the Python implementation; no third-party Python libraries are required for the core data structures.
- The gram stringify operation (native Pattern → gram string) remains acceptable with a JSON round-trip through the Rust codec, as stringify is significantly less frequent than parse in typical usage.
- `findFirst` and other partial-result operations returning `Option` (TypeScript) or `Optional` (Python) instead of nullable types is a non-breaking API improvement, as the previous `undefined`/`None` return is a subset of the new behavior.

---

## Dependencies

- **gram-hs reference** (`../pattern-hs/libs/`): Must be reviewed in Phase 3 before implementing Pattern operations — see proposal for specific items to cross-check (fold order, comonad ops, Value variants, StandardGraph classification).
- **Existing test suites**: TypeScript (`typescript/@relateby/pattern/`) and Python (`python/relateby/`) test suites serve as the correctness baseline throughout migration.
- **Rust gram-codec crate** (`crates/gram-codec`): Unchanged; remains the authoritative parser/serializer backing both WASM and PyO3 surfaces.
- **Migration proposal**: `proposals/migrate-ts-python-proposal.md` — detailed rationale, architecture diagrams, implementation sketches, and phased migration path. Planning should treat this document as the primary design reference.
