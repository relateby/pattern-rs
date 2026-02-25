# Research: TypeScript/WASM Graph API

**Branch**: `033-typescript-wasm-graph`  
**Date**: 2026-02-25  
**Status**: Complete — all unknowns resolved

---

## 1. WASM Boundary Strategy

**Decision**: Expose graph construction, querying, and all algorithms via WASM (single crossing per call); implement graph transforms as pure TypeScript.

**Rationale**: The proposal's decision table is confirmed by the existing `wasm.rs` pattern. Each algorithm call is one WASM crossing with all computation Rust-internal. Transform functions (`mapGraph`, `filterGraph`, etc.) call a JS callback on every element — N crossings for N elements — making pure TypeScript the correct choice. `paraGraph` and `paraGraphFixed` use `topoSort()` (one WASM crossing) for ordering, then iterate entirely in TypeScript.

**Alternatives considered**:
- All-WASM transforms: Rejected — N crossings per transform is prohibitively expensive for large graphs and provides no correctness benefit.
- All-TypeScript reimplementation of algorithms: Rejected — violates Reference Implementation Fidelity (Constitution I); Rust implementations already exist and are correct.

---

## 2. Pattern Type at WASM Boundaries

**Decision**: Accept `WasmPattern` (wrapping `Pattern<JsValue>`) at all WASM boundaries; deserialize to `Pattern<Subject>` internally at graph API entry points using the existing `_type: 'Subject'` marker convention.

**Rationale**: `WasmPattern` is the established WASM type for patterns. The `_type: 'Subject'` marker is already implemented in `WasmSubject::from_js_value` and `WasmSubject::to_js_value`. This avoids introducing a second pattern type at the boundary and keeps the JS API surface minimal.

**Alternatives considered**:
- Option A (separate `WasmSubjectPattern` type): Rejected — doubles the type surface; JS developers would need to manage two pattern types.
- Option C (serialize to JSON at boundary): Rejected — higher overhead and loses type information.

---

## 3. `wasm-bindgen` Constraint Handling

**Decision**: Use `js_sys::Array` of `JsValue`-wrapped `WasmPattern` for all collection returns. Use string constants (not numeric enums) for `GraphClass` and `TraversalDirection`. Return `JsValue::null()` for absent optional values; TypeScript declarations use `Pattern | null`.

**Rationale**: `wasm-bindgen` does not support generic types or custom types in arrays. These constraints are already handled in the existing `wasm.rs` (e.g., `elements()` returns `js_sys::Array`). The same pattern is applied consistently to graph types.

**Alternatives considered**:
- Numeric enums for `GraphClass`: Rejected — string constants are more debuggable and match the TypeScript discriminated union tags.
- `undefined` for absent values: Rejected — `null` is the established convention in the existing WASM bindings.

---

## 4. `PatternGraph` Generic Parameters

**Decision**: `WasmPatternGraph` wraps `PatternGraph<(), Subject>`. The `Extra` type parameter is `()` (unit) since the WASM layer has no use for extra per-element metadata.

**Rationale**: The Rust `PatternGraph<Extra, V>` is generic over an `Extra` type stored alongside `pg_other` elements. For WASM, this extra data is not needed. Using `()` is the zero-cost choice and matches the proposal's `WasmPatternGraph wraps PatternGraph<(), Subject>`.

**Alternatives considered**:
- `PatternGraph<JsValue, Subject>`: Rejected — adds unnecessary complexity; `pg_other` elements are not exposed in the initial API.

---

## 5. `GraphQuery` Ownership and Thread Safety

**Decision**: `WasmGraphQuery` wraps `GraphQuery<Subject>` via `Rc`. WASM is single-threaded, so `Rc` (not `Arc`) is correct.

**Rationale**: `GraphQuery<V>` contains `Rc<dyn Fn(...)>` closures internally (see `graph_query.rs`). WASM runs on a single thread; `Rc` is the appropriate smart pointer. This is explicitly noted in the proposal and matches the existing Rust graph query design.

**Alternatives considered**:
- `Arc`: Rejected — unnecessary overhead; WASM is single-threaded.
- Cloning `GraphQuery` into `WasmGraphQuery`: Rejected — `GraphQuery` contains non-Clone closures.

---

## 6. Weight Function Bridge

**Decision**: Accept `JsValue` as the weight parameter. Map string constants (`"undirected"`, `"directed"`, `"directed_reverse"`) to the corresponding Rust `TraversalWeight` constructors. Wrap a JS `Function` in an `Rc<dyn Fn(...)>` closure that calls back into JS with a `WasmPattern` and direction string.

**Rationale**: This matches the proposal's "Weight bridge" implementation note. The string-constant path is zero-overhead. The callback path is the documented escape hatch with per-edge crossing cost.

**Performance note for docs**: A custom weight function is called once per traversed edge. For a dense graph with 50,000 edges, this is 50,000 WASM crossings per traversal. Document prominently.

---

## 7. Effect Integration Strategy

**Decision**: Effect (`pipe`, `Either`, `Option`, `Match`) is an optional peer dependency. The WASM layer returns raw `{ _tag: 'Right'/'Left' }` objects. The TypeScript wrapper layer converts these to proper `Either.Either` and `Option.Option` values when Effect is available. `Match.tag` + `Match.exhaustive` is used internally in `mapGraph` and `filterGraph` for exhaustive `GraphClass` dispatch.

**Rationale**: Making Effect optional preserves compatibility for projects that don't use it. The `{ _tag }` shape is already established in the existing WASM bindings. The TypeScript wrapper layer is the right place for the conversion — it keeps the Rust layer clean and gives TypeScript users the full Effect combinator suite.

**Alternatives considered**:
- Hard dependency on Effect: Rejected — forces all users to install Effect even if they don't use it.
- No Effect integration: Rejected — the proposal explicitly calls for `Either`/`Option` wrappers; they significantly improve the developer experience for functional-style TypeScript.

---

## 8. Package Layout and Entry Points

**Decision**: `typescript/relateby/` with exports map: `"."` (root), `"./pattern"`, `"./gram"`, `"./graph"`. Mirrors `python/relateby/` layout exactly.

**Rationale**: The Python layout is already established and working. Mirroring it provides consistency across language targets and aligns with the proposal's "Guiding Principle: Mirror the Python Layout".

**Build process**: `wasm-pack build ../../crates/pattern-wasm --target bundler --out-dir ../../typescript/relateby/wasm` → `tsc` → `dist/`. The `--target bundler` flag produces ES module output compatible with Vite, webpack, Rollup, and esbuild.

---

## 9. Examples and Documentation Scope

**Decision**: Add `examples/relateby-graph/` (new) with Node.js and browser variants. Add `docs/typescript-graph.md` (new). Update `docs/wasm-usage.md` to reference the new graph API and `relateby` package name.

**Rationale**: Constitution V requires working examples for external language targets. The existing `examples/wasm-js/` is a minimal placeholder; a dedicated graph example demonstrates the new capabilities. `docs/typescript-graph.md` provides the reference documentation. The user input explicitly requested "appropriate additions or changes to examples/ and docs/".

**`examples/wasm-js/` is not modified**: Per spec assumption, this example builds its own WASM separately and is not affected by the `relateby` package.

---

## 10. Haskell Reference Alignment

**Verified**: The Rust implementations in `crates/pattern-core/src/graph/` are already ported from:
- `../pattern-hs/libs/pattern/src/Pattern/Graph/Algorithms.hs`
- `../pattern-hs/libs/pattern/src/Pattern/Graph/GraphClassifier.hs`
- `../pattern-hs/libs/pattern/src/Pattern/Graph/GraphQuery.hs`
- `../pattern-hs/libs/pattern/src/Pattern/Graph/Transform.hs`
- `../pattern-hs/libs/pattern/src/Pattern/PatternGraph.hs`
- `../pattern-hs/libs/pattern/src/Pattern/Reconcile.hs`

The TypeScript transform signatures in `src/graph/index.ts` mirror `Pattern.Graph.Transform` (curried, `GraphView`-consuming, `Substitution` ADT for `filterGraph`, explicit `(empty, combine)` for `foldGraph`, `ReadonlyMap<string, R>` for `paraGraph`).

No behavioral deviations from the reference implementation are required.
