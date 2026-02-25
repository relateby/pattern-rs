# Implementation Plan: TypeScript/WASM Graph API

**Branch**: `033-typescript-wasm-graph` | **Date**: 2026-02-25 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/033-typescript-wasm-graph/spec.md`  
**User note**: Including appropriate additions or changes to `examples/` and `docs/`

## Summary

Expose the existing Rust graph capabilities (`PatternGraph`, `GraphClassifier`, `GraphQuery`, graph algorithms, graph transforms, and `ReconciliationPolicy`) to JavaScript/TypeScript via three scoped npm packages: `@relateby/pattern` (WASM-backed types + algorithms), `@relateby/gram` (Gram codec), and `@relateby/graph` (pure TypeScript interfaces + transforms). WASM concrete classes are prefixed `Native` (`NativePattern`, `NativeSubject`, `NativePatternGraph`, `NativeGraphQuery`, `NativeReconciliationPolicy`). TypeScript interfaces (`Pattern<V>`, `Subject`, `PatternGraph<V>`, `GraphQuery<V>`) are exported from `@relateby/graph` and satisfied structurally by the `Native*` classes. The implementation follows the phased approach in the proposal: Phase 1 (NativePatternGraph + classification), Phase 2 (NativeGraphQuery + algorithms), Phase 3 (package scaffolds + pure-TS transforms), Phase 4 (Effect integration). Examples and documentation are updated in parallel.

## Technical Context

**Language/Version**: Rust 1.70.0 (MSRV, workspace), TypeScript 5.x, wasm-bindgen 0.2, wasm-pack  
**Primary Dependencies**:
- Rust: `wasm-bindgen`, `js-sys` (already in `pattern-wasm/Cargo.toml`); no new Rust crates required
- TypeScript: `typescript ^5.0`, `vitest ^2.0` (dev); `effect >=3.0.0` (optional peer)
- Build: `wasm-pack` (WASM compilation + JS glue generation)

**Storage**: N/A — all in-memory graph operations  
**Testing**: `cargo test --workspace` (Rust); `vitest run` (TypeScript)  
**Target Platform**: `wasm32-unknown-unknown` (browser + bundler), Node.js  
**Project Type**: Library (Rust crate + three scoped npm packages)  
**Performance Goals**: Graph construction and all algorithm functions complete without error for graphs of up to 10,000 nodes / 50,000 relationships on a standard developer machine  
**Constraints**: All WASM-exposed APIs must comply with `wasm-bindgen` constraints (no generic types, `js_sys::Array` for collections, no blocking I/O). Custom weight callbacks are an accepted escape hatch with documented per-edge crossing cost.  
**Scale/Scope**: Three scoped packages (`@relateby/pattern`, `@relateby/gram`, `@relateby/graph`); `@relateby/graph` has no runtime dependency on WASM and is independently installable

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-checked after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | ✅ PASS | All graph algorithms and transforms are ported from `../pattern-hs/libs/pattern/src/Pattern/Graph/`. The Rust implementations in `crates/pattern-core/src/graph/` already exist and are ported from Haskell. WASM bindings expose these without re-implementing logic. TypeScript transforms mirror `Pattern.Graph.Transform` signatures. |
| II. Correctness & Compatibility | ✅ PASS | WASM bindings delegate entirely to existing Rust implementations. No new algorithm logic is introduced. TypeScript transforms are thin wrappers over WASM-provided `topoSort()` for ordering. |
| III. Rust Native Idioms | ✅ PASS | New Rust code is limited to `wasm.rs` additions in `pattern-core` and `pattern-wasm/src/lib.rs`. Follows existing patterns in `crates/pattern-core/src/wasm.rs`. |
| IV. Multi-Target Library Design | ✅ PASS | New WASM bindings are feature-gated behind the existing `wasm` feature. No native-only code paths introduced. Existing `cargo build --target wasm32-unknown-unknown` CI check covers this. |
| V. External Language Bindings & Examples | ✅ PASS | This feature *is* the external language binding update. A new `examples/relateby-graph/` example and updated `docs/wasm-usage.md` are explicitly in scope. |

**Post-design re-check**: Deferred to after Phase 1 artifacts are complete.

## Project Structure

### Documentation (this feature)

```text
specs/033-typescript-wasm-graph/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   ├── wasm-api.md      # WASM binding contracts (NativePatternGraph, NativeGraphQuery, algorithms)
│   └── ts-api.md        # Pure TypeScript interface + transform contracts
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
crates/
└── pattern-core/
    └── src/
        └── wasm.rs          # ADD: WasmPatternGraph (→ NativePatternGraph),
                             #      WasmReconciliationPolicy (→ NativeReconciliationPolicy),
                             #      WasmGraphQuery (→ NativeGraphQuery),
                             #      algorithm free functions,
                             #      WasmGraphClass constant object

crates/
└── pattern-wasm/
    └── src/
        └── lib.rs           # ADD: re-export new WASM graph types as Native* names

typescript/
├── @relateby/
│   ├── pattern/             # NEW — @relateby/pattern (WASM-backed)
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── src/
│   │   │   └── index.ts     # init() + NativePattern, NativeSubject, NativePatternGraph,
│   │   │                    #          NativeGraphQuery, NativeReconciliationPolicy,
│   │   │                    #          algorithm functions, Either/Option wrappers
│   │   ├── wasm/            # generated by wasm-pack (gitignored)
│   │   ├── dist/            # generated by tsc (gitignored)
│   │   └── tests/
│   │       └── pattern.test.ts
│   │
│   ├── gram/                # NEW — @relateby/gram (WASM-backed Gram codec)
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── src/
│   │   │   └── index.ts     # Gram parse/stringify
│   │   └── tests/
│   │       └── gram.test.ts
│   │
│   └── graph/               # NEW — @relateby/graph (pure TypeScript, no WASM dep)
│       ├── package.json
│       ├── tsconfig.json
│       ├── src/
│       │   └── index.ts     # Subject, Pattern<V>, PatternGraph<V>, GraphQuery<V>,
│       │                    # GraphView<V>, toGraphView, GraphClass, Substitution,
│       │                    # mapGraph, mapAllGraph, filterGraph, foldGraph,
│       │                    # mapWithContext, paraGraph, paraGraphFixed, unfoldGraph
│       └── tests/
│           └── graph.test.ts  # WASM-free tests using plain TS stubs

examples/
└── relateby-graph/          # NEW — graph API example (Node.js + browser)
    ├── README.md
    ├── package.json
    ├── node.mjs             # Node.js usage: graph construction, BFS, centrality
    └── browser.html         # Browser usage: same operations via CDN/bundler

docs/
├── wasm-usage.md            # UPDATE: add graph API section, @relateby/* packages
└── typescript-graph.md      # NEW: TypeScript graph API reference (Native* + interfaces)
```

**Structure Decision**: Single Rust crate extension (`pattern-core/src/wasm.rs`) plus three new TypeScript package directories under `typescript/@relateby/`. `@relateby/graph` has no dependency on `@relateby/pattern` — the dependency flows the other way. The `examples/wasm-js/` directory is untouched per spec assumption.

## Complexity Tracking

No constitution violations. No complexity justification required.

---

## Implementation Phases

### Phase 1: NativePatternGraph + NativeReconciliationPolicy + GraphClass (WASM)

**Rust additions to `crates/pattern-core/src/wasm.rs`**:

- `WasmPatternGraph` (exported to JS as `NativePatternGraph`) wrapping `PatternGraph<(), Subject>`
  - `from_patterns(patterns: js_sys::Array, policy?: WasmReconciliationPolicy) → WasmPatternGraph`
  - `empty() → WasmPatternGraph`
  - getters: `nodes`, `relationships`, `walks`, `annotations`, `conflicts`, `size`
  - `merge(other: WasmPatternGraph) → WasmPatternGraph`
  - `topo_sort() → js_sys::Array` (bottom-up shape-class order)

- `WasmReconciliationPolicy` (exported to JS as `NativeReconciliationPolicy`) wrapping `ReconciliationPolicy`
  - static constructors: `last_write_wins()`, `first_write_wins()`, `strict()`, `merge(options?)`

- `WasmGraphClass` constant object (string constants, not a class)
  - `NODE`, `RELATIONSHIP`, `ANNOTATION`, `WALK`, `OTHER`

**Pattern deserialization**: `WasmPattern` values entering `from_patterns` are deserialized to `Pattern<Subject>` using the existing `_type: 'Subject'` marker convention (via `WasmSubject::from_js_value`).

**TypeScript declarations**: `NativePatternGraph` return types are declared as satisfying `PatternGraph<Subject>` (the interface from `@relateby/graph`). `@relateby/pattern` lists `@relateby/graph` as a dependency for interface types.

### Phase 2: NativeGraphQuery + Algorithms (WASM)

**Rust additions to `crates/pattern-core/src/wasm.rs`**:

- `WasmGraphQuery` (exported to JS as `NativeGraphQuery`) wrapping `GraphQuery<Subject>` via `Rc`
  - `from_pattern_graph(graph: WasmPatternGraph) → WasmGraphQuery`
  - `nodes() → js_sys::Array`, `relationships() → js_sys::Array`
  - `source(rel: WasmPattern) → JsValue` (null if absent)
  - `target(rel: WasmPattern) → JsValue` (null if absent)
  - `incident_rels(node: WasmPattern) → js_sys::Array`
  - `degree(node: WasmPattern) → usize`
  - `node_by_id(identity: &str) → JsValue` (null if absent)
  - `relationship_by_id(identity: &str) → JsValue` (null if absent)

- Free algorithm functions (all `#[wasm_bindgen]`):
  - `bfs`, `dfs`, `shortest_path`, `all_paths`, `connected_components`
  - `has_cycle`, `is_connected`, `topological_sort`
  - `degree_centrality`, `betweenness_centrality`, `minimum_spanning_tree`
  - `query_walks_containing`, `query_co_members`, `query_annotations_of`

- `WasmTraversalDirection` constant object: `FORWARD`, `BACKWARD`

- **Weight bridge**: accept `JsValue`; map string constants to `undirected()`/`directed()`/`directed_reverse()`; wrap JS `Function` in `Rc<dyn Fn(...)>` closure.

**TypeScript declarations**: `NativeGraphQuery` return types declared as satisfying `GraphQuery<Subject>`. Algorithm functions typed against `GraphQuery<Subject>` and `Pattern<Subject>`.

### Phase 3: TypeScript Package Scaffolds + Pure-TS Transforms

**`@relateby/graph` package** (`typescript/@relateby/graph/`):
- `package.json`: no runtime dependencies (pure TypeScript); `effect >=3.0.0` optional peer
- `src/index.ts`: exports all interfaces and transforms:
  - Interfaces: `Subject`, `Pattern<V>`, `PatternGraph<V>`, `GraphQuery<V>`, `GraphView<V>`
  - Free function: `toGraphView<V>(graph: PatternGraph<V>): GraphView<V>`
  - ADTs: `GraphClass` discriminated union + smart constructors, `Substitution` discriminated union
  - Transforms: `mapGraph`, `mapAllGraph`, `filterGraph`, `foldGraph`, `mapWithContext`, `paraGraph`, `paraGraphFixed`, `unfoldGraph`
  - All transforms curried; `paraGraph`/`paraGraphFixed` call `graph.topoSort()` once for ordering

**`@relateby/pattern` package** (`typescript/@relateby/pattern/`):
- `package.json`: depends on `@relateby/graph` for interface types; `effect >=3.0.0` optional peer
- `src/index.ts`: exports `init()`, `NativePattern`, `NativeSubject`, `NativePatternGraph`, `NativeGraphQuery`, `NativeReconciliationPolicy`, `NativeValue`, `NativeValidationRules`, `NativeStructureAnalysis`, algorithm functions
- Return types declared against `@relateby/graph` interfaces (e.g., `NativePatternGraph.fromPatterns()` returns `PatternGraph<Subject>`)

**`@relateby/gram` package** (`typescript/@relateby/gram/`):
- `package.json`: depends on `@relateby/pattern` for WASM init
- `src/index.ts`: exports `Gram.parse` / `Gram.stringify`

### Phase 4: Effect Integration

- `@relateby/pattern/src/index.ts`: Convert `{ _tag: 'Right'/'Left' }` WASM returns to `Either.Either<T,E>` and `Option.Option<T>` when `effect` is present
- `@relateby/graph/src/index.ts`: Use `Match.tag` + `Match.exhaustive` internally for `GraphClass` dispatch in `mapGraph` and `filterGraph`
- All `package.json` files: `peerDependencies: { "effect": ">=3.0.0" }` with `optional: true`

### Examples and Docs

**`examples/relateby-graph/`** (new):
- `node.mjs`: Build a graph from `NativePattern`/`NativeSubject`, run BFS, compute centrality, apply `mapGraph` + `filterGraph` pipeline via `@relateby/graph`
- `browser.html`: Same operations in a browser context
- `README.md`: Prerequisites (`@relateby/pattern`, `@relateby/graph`), build steps, run instructions

**`docs/typescript-graph.md`** (new):
- Package installation (`@relateby/pattern`, `@relateby/gram`, `@relateby/graph`)
- Initialization (`init()` from `@relateby/pattern`)
- Graph construction with `NativePatternGraph` and `NativeReconciliationPolicy`
- Querying with `NativeGraphQuery`
- Running algorithms (BFS, shortest path, centrality)
- Pure TypeScript transforms via `@relateby/graph` (map, filter, fold, para)
- WASM-free usage: implementing `Subject`, `Pattern<Subject>`, `PatternGraph<Subject>` as plain TS stubs
- Effect integration (Either/Option)
- Performance notes on custom weight callbacks

**`docs/wasm-usage.md`** (update):
- Add section: "Graph API" pointing to `docs/typescript-graph.md`
- Update package name references to `@relateby/pattern`, `@relateby/gram`, `@relateby/graph`

### Verification Steps (per constitution)

1. Review `../pattern-hs/libs/pattern/src/Pattern/Graph/` for behavioral equivalence
2. Port relevant tests from `../pattern-hs/libs/pattern/tests/` to TypeScript vitest suite
3. `cargo build --workspace --target wasm32-unknown-unknown` — must pass
4. `cargo test --workspace` — must pass
5. `cargo clippy --workspace -- -D warnings` — must pass
6. `cargo fmt --all -- --check` — must pass
7. `npm run build` in each `typescript/@relateby/*/` package — must produce `dist/` and `wasm/`
8. `npm test` in each package — vitest suite must pass
9. `@relateby/graph` tests MUST pass without WASM initialization (pure TS stubs only)
10. `node examples/relateby-graph/node.mjs` — must run without error
11. `./scripts/ci-local.sh` — full CI validation
