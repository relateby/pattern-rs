# Feature Specification: TypeScript/WASM Graph API

**Feature Branch**: `033-typescript-wasm-graph`  
**Created**: 2026-02-25  
**Status**: Draft  
**Input**: User description: "Graph features for Typescript/WASM as described in proposals/typescript-graph-proposal.md"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Build and Query a Graph from Patterns (Priority: P1)

A TypeScript developer working in a browser or Node.js environment wants to construct a graph from a set of `Pattern` values and run structural queries on it — finding nodes, traversing relationships, and computing paths — without writing any Rust or managing low-level serialization.

**Why this priority**: Graph construction and querying are the foundational capabilities. All other graph features depend on this working correctly. Delivering this alone provides a usable MVP for graph-aware TypeScript applications.

**Independent Test**: Can be fully tested by constructing a `NativePatternGraph` from an array of patterns, creating a `NativeGraphQuery` from it, and verifying that node/relationship accessors and traversal functions return correct results.

**Acceptance Scenarios**:

1. **Given** a set of node and relationship patterns, **When** a developer constructs a `NativePatternGraph` from them, **Then** the graph correctly classifies and stores nodes, relationships, walks, and annotations separately.
2. **Given** a `NativePatternGraph`, **When** a developer creates a `NativeGraphQuery` and calls `nodes()` or `relationships()`, **Then** the returned collections match the patterns used to build the graph.
3. **Given** a `NativeGraphQuery` and a starting node, **When** a developer calls a traversal function (breadth-first or depth-first), **Then** the function returns the visited nodes in the correct traversal order.
4. **Given** a `NativeGraphQuery` with two connected nodes, **When** a developer calls `shortestPath`, **Then** the function returns the sequence of patterns connecting them, or indicates no path exists.
5. **Given** two `NativePatternGraph` instances, **When** a developer merges them with a reconciliation policy, **Then** conflicting identities are resolved according to the chosen policy.

---

### User Story 2 - Analyze Graph Structure (Priority: P2)

A TypeScript developer wants to understand the structural properties of a graph — whether it contains cycles, how connected it is, which nodes are most central — to make decisions in their application without implementing these algorithms themselves.

**Why this priority**: Structural analysis builds on graph construction (P1) and delivers high-value insights for graph-aware applications. These are self-contained operations with clear outputs.

**Independent Test**: Can be fully tested by constructing a known graph and verifying that `hasCycle`, `isConnected`, `connectedComponents`, `degreeCentrality`, and `topologicalSort` return expected values.

**Acceptance Scenarios**:

1. **Given** a graph with a cycle, **When** a developer calls `hasCycle`, **Then** it returns `true`; for an acyclic graph it returns `false`.
2. **Given** a disconnected graph, **When** a developer calls `connectedComponents`, **Then** it returns the correct groupings of mutually reachable nodes.
3. **Given** a graph, **When** a developer calls `degreeCentrality` or `betweennessCentrality`, **Then** it returns a mapping of node identities to their centrality scores.
4. **Given** a directed acyclic graph, **When** a developer calls `topologicalSort`, **Then** it returns a valid topological ordering; for a cyclic graph it indicates the sort is not possible.
5. **Given** a weighted graph, **When** a developer calls `minimumSpanningTree`, **Then** it returns the set of relationships forming the minimum spanning tree.

---

### User Story 3 - Transform Graphs with Pure TypeScript Functions (Priority: P3)

A TypeScript developer wants to apply functional transformations to a graph — mapping values, filtering elements, folding to a summary, or performing bottom-up structural computations — using composable, point-free style without crossing into native code for each element.

**Why this priority**: Transforms are the application layer above graph construction and querying. They enable the richest developer experience but depend on P1 and P2 being in place.

**Independent Test**: Can be fully tested by calling `toGraphView` with any object satisfying `PatternGraph<V>` — including a plain TypeScript stub — and verifying that `mapGraph`, `filterGraph`, `foldGraph`, `paraGraph`, and `unfoldGraph` produce correct results on known inputs without requiring WASM initialization.

**Acceptance Scenarios**:

1. **Given** a `GraphView`, **When** a developer applies `mapGraph` with per-class mapping functions, **Then** each element is transformed according to its graph class (node, relationship, walk, annotation).
2. **Given** a `GraphView`, **When** a developer applies `filterGraph` with a predicate and a substitution strategy, **Then** removed elements are handled according to the chosen strategy (delete container, splice gap, or replace with surrogate).
3. **Given** a `GraphView`, **When** a developer applies `foldGraph` with a combining function and initial value, **Then** it produces the correct accumulated result across all elements.
4. **Given** a `GraphView`, **When** a developer applies `paraGraph` with a bottom-up accumulation function, **Then** each element receives the pre-computed results of its structural dependencies.
5. **Given** a set of seed values, **When** a developer applies `unfoldGraph` with an expansion function, **Then** it produces a value satisfying `PatternGraph<V>` containing all expanded patterns.
6. **Given** a pipeline of transform functions, **When** composed with `pipe`, **Then** each transform receives the output of the previous one, enabling point-free composition.
7. **Given** plain TypeScript objects satisfying `Subject`, `Pattern<Subject>`, `PatternGraph<Subject>`, and `GraphQuery<Subject>` (no WASM involved), **When** a developer calls `toGraphView` and applies any transform function, **Then** the transforms execute correctly without WASM having been initialized or imported.

---

### User Story 4 - Install and Initialize the Package (Priority: P1)

A TypeScript developer wants to add graph-aware pattern capabilities to their project by installing scoped npm packages and initializing them with minimal boilerplate.

**Why this priority**: Packaging and initialization are prerequisites for any use of the library. Without a working install path, no other capability is accessible.

**Independent Test**: Can be fully tested by installing the packages in a fresh project, calling `init()`, and verifying that `NativePattern`, `Gram`, and graph functions are all accessible from their documented entry points.

**Acceptance Scenarios**:

1. **Given** a JavaScript/TypeScript project, **When** a developer installs `@relateby/pattern`, `@relateby/gram`, and `@relateby/graph`, **Then** all graph, pattern, and gram capabilities are available.
2. **Given** a bundler-based project (Vite, webpack, Rollup), **When** the developer imports from `@relateby/pattern`, **Then** initialization is automatic and no explicit `init()` call is required.
3. **Given** a Node.js project or explicit initialization context, **When** the developer calls `await init()`, **Then** all WASM-backed functions are ready to use.
4. **Given** a project that does not use Effect, **When** the developer imports `@relateby/pattern`, **Then** the package works without requiring Effect to be installed.
5. **Given** a project that does use Effect, **When** the developer imports `@relateby/pattern`, **Then** fallible operations return proper `Either` and `Option` values compatible with the Effect combinator suite.
6. **Given** a project that only needs pure TypeScript graph transforms, **When** the developer installs only `@relateby/graph`, **Then** all transform functions and interfaces (`Subject`, `Pattern<V>`, `PatternGraph<V>`, `GraphQuery<V>`) are available without WASM or `@relateby/pattern` installed.

---

### Edge Cases

- What happens when `NativePatternGraph.fromPatterns` receives an empty array? The graph should be valid and empty, with all collection accessors returning empty arrays.
- What happens when `shortestPath` is called for two nodes with no connecting path? The function should indicate absence of a path rather than throwing.
- What happens when `topologicalSort` is called on a graph with a cycle? The function should indicate the sort is not possible rather than looping indefinitely.
- What happens when `filterGraph` removes a node that is referenced by a relationship inside a walk? The substitution strategy governs the outcome: the walk is deleted, the gap is spliced, or a surrogate replaces the removed element.
- What happens when a custom weight function is provided to a traversal algorithm? The function is called once per traversed edge; the developer is responsible for the performance cost of many such calls on large graphs.
- What happens when two patterns with the same identity are added to a `NativePatternGraph` under the `strict` reconciliation policy? The conflict is recorded and accessible via the `conflicts` accessor rather than silently overwriting.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Developers MUST be able to construct a graph from a collection of patterns, with the graph automatically classifying each pattern as a node, relationship, walk, annotation, or other element.
- **FR-002**: Developers MUST be able to specify a reconciliation policy when constructing a graph, controlling how identity conflicts between patterns are resolved.
- **FR-003**: Developers MUST be able to merge two graphs into a new graph, with conflicts resolved by the active reconciliation policy.
- **FR-004**: Developers MUST be able to create a query handle from a graph and use it to access nodes, relationships, source/target of a relationship, incident relationships of a node, and look up elements by identity.
- **FR-005**: Developers MUST be able to traverse a graph from a starting node using breadth-first and depth-first strategies, receiving the visited nodes in traversal order.
- **FR-006**: Developers MUST be able to find the shortest path between two nodes, with the result indicating absence when no path exists.
- **FR-007**: Developers MUST be able to find all paths between two nodes.
- **FR-008**: Developers MUST be able to compute connected components of a graph.
- **FR-009**: Developers MUST be able to test whether a graph contains a cycle and whether it is fully connected.
- **FR-010**: Developers MUST be able to compute a topological sort of a directed acyclic graph, with the result indicating impossibility for cyclic graphs.
- **FR-011**: Developers MUST be able to compute degree centrality and betweenness centrality for all nodes in a graph.
- **FR-012**: Developers MUST be able to compute the minimum spanning tree of a graph.
- **FR-013**: Developers MUST be able to query walks containing a specific node, co-members of a node within walks, and annotations targeting a specific element.
- **FR-014**: Traversal and algorithm functions MUST accept an optional weight specification; the default behavior MUST be undirected traversal.
- **FR-015**: Developers MUST be able to supply a custom weight function as an escape hatch; the library MUST document that this incurs one call per traversed edge.
- **FR-016**: Developers MUST be able to apply `mapGraph` to transform each element of a graph view using separate mapping functions per graph class.
- **FR-032**: Developers MUST be able to apply `mapAllGraph` to transform every element of a graph view with a single uniform function, regardless of graph class. `mapAllGraph` is a convenience specialization of `mapGraph` and MUST be curried.
- **FR-017**: Developers MUST be able to apply `filterGraph` to remove elements from a graph view, with a substitution strategy controlling how container integrity is maintained.
- **FR-018**: Developers MUST be able to apply `foldGraph` to reduce a graph view to a single value using an explicit empty value and combining function.
- **FR-019**: Developers MUST be able to apply `mapWithContext` to transform elements while receiving a snapshot query (typed as `GraphQuery<V>`) reflecting the graph state at the start of the transformation.
- **FR-020**: Developers MUST be able to apply `paraGraph` to perform a bottom-up structural fold, where each element receives the pre-computed results of its structural dependencies.
- **FR-021**: Developers MUST be able to apply `paraGraphFixed` to iterate a bottom-up fold until a convergence predicate is satisfied.
- **FR-022**: Developers MUST be able to apply `unfoldGraph` to expand a set of seed values into an object satisfying `PatternGraph<V>`.
- **FR-023**: All transform functions MUST be curried so they can be composed in a point-free pipeline using `pipe`.
- **FR-024**: The library MUST be installable as three scoped npm packages: `@relateby/pattern` (WASM-backed pattern and gram types), `@relateby/gram` (Gram codec), and `@relateby/graph` (pure TypeScript interfaces and transforms). Each package MUST be independently installable.
- **FR-025**: `@relateby/graph` MUST be usable without installing `@relateby/pattern` or `@relateby/gram`; it has no runtime dependency on WASM.
- **FR-026**: All packages MUST work without Effect installed; Effect MUST be an optional peer dependency.
- **FR-027**: When Effect is available, fallible operations MUST return `Either` and nullable returns MUST return `Option` values compatible with the Effect combinator suite.
- **FR-028**: `@relateby/pattern` MUST initialize automatically in bundler environments; explicit `init()` MUST be available for Node.js and other environments requiring manual initialization.
- **FR-029**: `@relateby/graph` MUST export `Subject`, `Pattern<V>`, `GraphQuery<V>`, and `PatternGraph<V>` as TypeScript interfaces (formerly `ISubject`, `IPattern<V>`, `IGraphQuery<V>`, `IPatternGraph<V>`). `Subject` describes the read-facing contract of a subject value (`identity`, `labels`, `properties`). The canonical typed form for graph transforms is `Pattern<Subject>`, `GraphQuery<Subject>`, and `PatternGraph<Subject>` — all fully expressible in pure TypeScript without WASM. The WASM concrete classes `NativeSubject`, `NativePattern`, `NativeGraphQuery`, and `NativePatternGraph` (exported from `@relateby/pattern`) MUST satisfy these interfaces structurally. The dependency direction MUST flow from `@relateby/pattern` → `@relateby/graph`; `@relateby/graph` MUST NOT import from `@relateby/pattern`.
- **FR-030**: `@relateby/graph` MUST export a generic free function `toGraphView<V>(graph: PatternGraph<V>): GraphView<V>` that constructs the initial `GraphView` for the transform pipeline. This function MUST work with any object satisfying `PatternGraph<V>`, including plain TypeScript objects, without requiring WASM initialization.
- **FR-031**: All WASM construction paths in `@relateby/pattern` MUST be declared with concrete generic return types using `Subject` (the interface) as the value type. Specifically: `NativePatternGraph.fromPatterns()` and `NativePatternGraph.empty()` MUST return a type satisfying `PatternGraph<Subject>`; `NativeGraphQuery.fromPatternGraph()` MUST return a type satisfying `GraphQuery<Subject>`; `NativePatternGraph` accessors (`nodes`, `relationships`, etc.) MUST return `readonly Pattern<Subject>[]`. This ensures the `V = Subject` assertion is made once at construction and propagates through the type system without requiring downstream casts.

### Key Entities

- **Subject** *(interface, from `@relateby/graph`)*: TypeScript interface describing the read-facing contract of a subject value: `identity: string`, `labels: readonly string[]`, `properties: Record<string, unknown>`. `NativeSubject` (from `@relateby/pattern`) satisfies this interface structurally. Enables fully WASM-free construction of `Pattern<Subject>` stubs.
- **Pattern\<V\>** *(interface, from `@relateby/graph`)*: Generic TypeScript interface describing the read-facing contract of a pattern node: `value: V` and `elements: readonly Pattern<V>[]`. Generic over the value type `V`, mirroring Rust's `Pattern<V>`. `NativePattern` (from `@relateby/pattern`) satisfies `Pattern<unknown>` structurally. The canonical typed form in the transform layer is `Pattern<Subject>` — fully expressible in pure TypeScript without WASM. All transform function signatures, `GraphQuery<V>` method returns, and `GraphView<V>` element lists are typed against `Pattern<V>`.
- **PatternGraph\<V\>** *(interface, from `@relateby/graph`)*: Generic TypeScript interface describing the read-facing contract of a classified graph: `nodes: readonly Pattern<V>[]`, `relationships: readonly Pattern<V>[]`, `walks: readonly Pattern<V>[]`, `annotations: readonly Pattern<V>[]`, `conflicts: Record<string, readonly Pattern<V>[]>`, `size: number`, and `topoSort(): readonly Pattern<V>[]`. `NativePatternGraph` (from `@relateby/pattern`) satisfies `PatternGraph<Subject>` structurally. The canonical typed form is `PatternGraph<Subject>`.
- **GraphQuery\<V\>** *(interface, from `@relateby/graph`)*: Generic TypeScript interface describing the structural navigation contract: `nodes(): readonly Pattern<V>[]`, `relationships(): readonly Pattern<V>[]`, `source(rel: Pattern<V>): Pattern<V> | null`, `target(rel: Pattern<V>): Pattern<V> | null`, `incidentRels(node: Pattern<V>): readonly Pattern<V>[]`, `degree(node: Pattern<V>): number`, `nodeById(id: string): Pattern<V> | null`, `relationshipById(id: string): Pattern<V> | null`. `NativeGraphQuery` (from `@relateby/pattern`) satisfies `GraphQuery<Subject>` structurally. All pure TypeScript transform functions are typed against `GraphQuery<V>`. `GraphView<V>` is typed against `GraphQuery<V>`. The canonical typed form is `GraphQuery<Subject>`.
- **NativePattern** *(WASM class, from `@relateby/pattern`)*: The WASM-backed concrete class satisfying `Pattern<unknown>` structurally — `value` is typed as `unknown` at the WASM boundary. Constructed via `NativePattern.point(value)` or `NativePattern.pattern(value)`. When used in graph contexts, callers work through `Pattern<Subject>` (from `@relateby/graph`) for type-safe access.
- **NativeSubject** *(WASM class, from `@relateby/pattern`)*: The WASM-backed concrete class satisfying `Subject` structurally — `identity: string`, `labels: readonly string[]`, `properties: Record<string, unknown>`. Constructed via `new NativeSubject(identity, labels, properties)`. Used as the concrete value type in the primary graph use case: `Pattern<Subject>`.
- **NativePatternGraph** *(WASM class, from `@relateby/pattern`)*: The WASM-backed concrete class satisfying `PatternGraph<Subject>`. Constructed via `NativePatternGraph.fromPatterns()` or `NativePatternGraph.empty()`. Immutable after construction; `merge` returns a new instance.
- **NativeReconciliationPolicy** *(WASM class, from `@relateby/pattern`)*: A rule governing how identity conflicts are resolved when patterns with the same identity are combined into a graph. Variants include last-write-wins, first-write-wins, strict (conflict recording), and merge (with configurable element, label, and property strategies).
- **NativeGraphQuery** *(WASM class, from `@relateby/pattern`)*: The WASM-backed concrete class satisfying `GraphQuery<Subject>`. Constructed via `NativeGraphQuery.fromPatternGraph()`.
- **GraphView\<V\>** *(pure TypeScript, from `@relateby/graph`)*: A generic paired snapshot of a `GraphQuery<V>` and a classified list of `[GraphClass, Pattern<V>]` pairs. Typed entirely against `Pattern<V>`, `GraphQuery<V>`, and `PatternGraph<V>` — no WASM concrete types. Constructed via the free function `toGraphView<V>(graph: PatternGraph<V>): GraphView<V>` exported from `@relateby/graph`. Consumed by transform functions; transforms produce new `GraphView<V>` instances.
- **GraphClass** *(pure TypeScript, from `@relateby/graph`)*: A classification of a pattern's role in a graph (node, relationship, walk, annotation, other). Used as a discriminant in transform callbacks.
- **Substitution** *(pure TypeScript, from `@relateby/graph`)*: A strategy for repairing container integrity when `filterGraph` removes an element from inside a walk or annotation (delete the container, splice the gap, or replace with a surrogate pattern).
- **Weight** *(pure TypeScript, from `@relateby/graph`)*: A specification for how edges are weighted during traversal. Can be a string constant (undirected, directed, directed-reverse) or a custom function called per traversed edge.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer can install `@relateby/pattern`, initialize it, construct a graph from patterns, and run a traversal in under 10 lines of application code.
- **SC-002**: All graph algorithm functions (traversal, path-finding, component analysis, centrality, spanning tree) produce results consistent with the Haskell reference implementation on equivalent inputs.
- **SC-003**: All pure TypeScript transform functions (`mapGraph`, `filterGraph`, `foldGraph`, `mapWithContext`, `paraGraph`, `paraGraphFixed`, `unfoldGraph`) produce results consistent with the Haskell reference implementation on equivalent inputs. These functions MUST be testable using plain TypeScript objects satisfying `Pattern<V>`, `GraphQuery<V>`, and `PatternGraph<V>` without WASM initialization.
- **SC-004**: Graph construction, querying, and all algorithm functions complete without error for graphs of up to 10,000 nodes and 50,000 relationships on a standard developer machine.
- **SC-005**: `@relateby/pattern` installs and operates correctly in bundler-based browser environments (Vite, webpack, Rollup) without additional configuration.
- **SC-006**: `@relateby/pattern` installs and operates correctly in Node.js environments with explicit initialization.
- **SC-007**: Projects that do not install Effect can still use all graph, pattern, and gram capabilities without errors.
- **SC-008**: All fallible operations surface errors in a composable form; no operation throws an unhandled exception for valid inputs. Without Effect (FR-026), fallible operations return `T | null` rather than throwing. With Effect (FR-027), they return `Either.Either<T, E>` or `Option.Option<T>`. Edge cases (empty graph, no path, cyclic sort, walk integrity violations) MUST be handled gracefully per this contract.
- **SC-009**: `@relateby/graph` exports all transform functions and interfaces (`Subject`, `Pattern<V>`, `GraphQuery<V>`, `PatternGraph<V>`, `GraphView<V>`, `toGraphView`); `@relateby/pattern` exports all WASM-backed types (`NativePattern`, `NativeSubject`, `NativePatternGraph`, `NativeGraphQuery`, `NativeReconciliationPolicy`) and algorithm functions.
- **SC-010**: A point-free pipeline composing three or more transform functions produces the same result as applying them sequentially.
- **SC-011**: `@relateby/graph` (`toGraphView`, `mapGraph`, `filterGraph`, `foldGraph`, `mapWithContext`, `paraGraph`, `paraGraphFixed`, `unfoldGraph`, `Subject`, `Pattern`, `GraphQuery`, `PatternGraph`) can be imported and used in a TypeScript project that never imports or initializes `@relateby/pattern`, using only plain TypeScript objects satisfying `Subject`, `Pattern<Subject>`, `GraphQuery<Subject>`, and `PatternGraph<Subject>`.

## Clarifications

### Session 2026-02-25 (continued)

- Q: Should the TypeScript transforms operate against TypeScript interfaces rather than the WASM concrete classes directly? → A: Yes — define `GraphQuery` and `PatternGraph` interfaces; type transforms against interfaces; WASM classes satisfy them structurally.
- Q: Where should `GraphQuery` and `PatternGraph` interfaces be exported from? → A: From `@relateby/graph`; WASM concrete classes in `@relateby/pattern` satisfy them; dependency flows from `@relateby/pattern` → `@relateby/graph`.
- Q: How should the initial `GraphView` be constructed? → A: Free function `toGraphView<V>(graph: PatternGraph<V>): GraphView<V>` exported from `@relateby/graph`; works with any `PatternGraph<V>` implementation.
- Q: Should `@relateby/graph` export test helper utilities for constructing stubs? → A: No — document a minimal stub pattern in developer docs instead; no test utilities shipped.
- Q: Should WASM-free use of `@relateby/graph` be a first-class scenario? → A: Yes — acceptance scenario 7 in User Story 3 and SC-011 verify that `@relateby/graph` transforms work without WASM initialization.
- Q: Should `Pattern` (as used in the transform layer) be a plain TypeScript interface rather than the WASM concrete class? → A: Yes — define `Pattern<V>` interface in `@relateby/graph`; type all transforms against it; `NativePattern` satisfies it structurally.
- Q: Should `Pattern<V>` be generic over its value type? → A: Yes — `Pattern<V>` with `value: V`; `GraphQuery<V>`, `PatternGraph<V>`, and all transform signatures become generic accordingly, mirroring Rust's `Pattern<V>`.
- Q: Should the spec ship a cast helper for `Pattern<Subject>` or document it as a typed assertion? → A: Document assertion in Assumptions and docs only; no cast helper. All construction paths (e.g., `NativePatternGraph.fromPatterns`) MUST be verified to correctly type their output as `PatternGraph<Subject>`.
- Q: Should `@relateby/pattern` export a single `NativePattern` type or also a typed alias? → A: Single `NativePattern` type (satisfying `Pattern<unknown>`); typed usage goes through `Pattern<Subject>` from `@relateby/graph`; no additional aliases.
- Q: Should `@relateby/graph` export `Subject` interface to complete the WASM-free chain for `Pattern<Subject>`? → A: Yes — export `Subject` interface from `@relateby/graph`; `Pattern<Subject>` is the canonical typed pattern in the transform layer; `NativeSubject` satisfies `Subject` structurally.
- Q: What are the package names for the graph transforms and gram codec packages? → A: Three separate scoped packages: `@relateby/pattern`, `@relateby/graph`, `@relateby/gram`.
- Q: Should WASM concrete classes be prefixed `Native` and TypeScript interfaces drop the `I` prefix? → A: Yes — `NativePattern`, `NativeSubject`, `NativePatternGraph`, `NativeGraphQuery`, `NativeReconciliationPolicy`; interfaces are `Pattern<V>`, `Subject`, `PatternGraph<V>`, `GraphQuery<V>`.

## Assumptions

- The existing `crates/pattern-wasm` Rust crate is the target for new WASM bindings; no new Rust crate is required.
- The `typescript/` directory will contain three scoped packages: `typescript/@relateby/pattern/`, `typescript/@relateby/gram/`, and `typescript/@relateby/graph/`. None of these directories exist yet.
- The existing `WasmPattern` type (wrapping `Pattern<JsValue>`) is the canonical pattern type at WASM boundaries; graph operations deserialize to `Pattern<Subject>` internally using the existing `_type: 'Subject'` marker convention.
- `wasm-bindgen` constraints (no generic types, no custom types in arrays, `js_sys::Array` for collections) are accepted as given; the TypeScript API is designed around these constraints.
- The `effect` library version 3.x is the target for optional Effect integration.
- All graph operations are synchronous; no async/streaming variants are required in this feature.
- The `@relateby/io` module (Effect Schema validation) is explicitly out of scope and deferred to a future feature.
- Custom weight callbacks are an accepted escape hatch with documented performance implications; no attempt is made to optimize the per-edge crossing cost.
- The `examples/wasm-js/` directory is not modified by this feature.
- No test utility helpers for `Pattern<V>`/`PatternGraph<V>`/`GraphQuery<V>` stubs are shipped as part of the package API. A minimal stub pattern is documented in `docs/typescript-graph.md` to guide consumers writing their own stubs for unit testing.
- `NativeSubject` and `NativeValue` (the concrete value types carried inside patterns) are WASM-backed classes. The transform layer reads pattern structure via `Pattern<V>` (`value: V`, `elements: readonly Pattern<V>[]`) without needing to construct or inspect `NativeSubject`/`NativeValue` instances. Construction of `NativePattern` graphs requires WASM initialization. `NativePattern` satisfies `Pattern<unknown>` structurally; typed usage in the transform layer uses `Pattern<Subject>` when working with subject-bearing graphs.
- `Pattern<Subject>` is a typed assertion at the TypeScript level, not a runtime guarantee enforced by the WASM boundary. The `_type: 'Subject'` marker convention on pattern values provides runtime discrimination when needed. No cast helper is shipped; instead, all WASM construction paths (`NativePatternGraph.fromPatterns`, `NativePatternGraph.empty`, `NativeGraphQuery.fromPatternGraph`, and related accessors) MUST be declared to return the appropriate concrete generic type (e.g., `PatternGraph<Subject>`) so the assertion is made once at construction and propagates through the type system.
- The `@relateby/pattern` package exports a single `NativePattern` WASM class (satisfying `Pattern<unknown>`). No typed alias is exported from `@relateby/pattern`. Callers who need typed access use `Pattern<Subject>` from `@relateby/graph` directly.
