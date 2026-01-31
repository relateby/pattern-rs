# Research: WASM Feature Parity with Python and Pattern&lt;V&gt; TypeScript Generics

**Feature**: 027-wasm-pattern-typescript-parity  
**Date**: 2026-01-31  
**Purpose**: Research technical decisions for WASM bindings and TypeScript generics for pattern-core

## Decision: wasm-bindgen Binding Strategy

**Date**: 2026-01-31  
**Question**: How should we expose Pattern and Subject to JavaScript/TypeScript via WASM?  
**Decision**: Use wasm-bindgen with a feature-gated `wasm` module in pattern-core, exposing JS classes or exported functions that wrap Rust Pattern/Subject. Use `#[wasm_bindgen]` on types and methods; use `JsValue` for generic value conversion at the boundary.

**Rationale**:
- wasm-bindgen is the standard Rust-WASM bridge; gram-codec already uses it in `gram-codec/src/wasm.rs`
- pattern-core already compiles to wasm32-unknown-unknown; adding wasm-bindgen is additive
- Feature-gating keeps native and Python builds unchanged and avoids pulling in WASM deps for non-WASM users
- Single crate keeps the codebase simple and mirrors the Python binding approach (feature-gated module)

**Alternatives Considered**:
- **Separate crate (e.g. pattern-core-wasm)**: Extra crate to maintain; duplication of re-exports. Rejected in favor of same-crate feature gate.
- **Emscripten / raw FFI**: More manual, less ergonomic. wasm-bindgen provides idiomatic Rust and automatic JS glue.
- **Non-Rust WASM (e.g. AssemblyScript)**: Would reimplement logic; defeats parity with Rust core. Rejected.

**Trade-offs**:
- ✅ Pro: Same crate, single source of truth; feature flag allows clean separation
- ✅ Pro: wasm-bindgen handles ABI, serialization of primitives and supported types
- ⚠️ Con: Generic Pattern&lt;V&gt; in Rust cannot be directly exposed as one JS class; we expose a single “Pattern” at the boundary and use JsValue for V, with TypeScript generics layered on top for typing

**Implementation Impact**:
- Add `wasm-bindgen` dependency under `[target.'cfg(target_arch = "wasm32")].dependencies` or optional feature
- Create `src/wasm.rs` with `#[wasm_bindgen]` structs and functions; convert Rust Pattern/Subject to/from JsValue at boundary
- Export constructors (point, pattern), accessors (value, elements), and all methods matching Python API

---

## Decision: Generic Pattern&lt;V&gt; at the WASM Boundary

**Date**: 2026-01-31  
**Question**: Rust has Pattern&lt;V&gt;; WASM/JS has no generics. How do we expose this and still support Pattern&lt;Subject&gt; and TypeScript generics?  
**Decision**: Expose a single opaque WASM type “Pattern” (and “Subject”) at the JS boundary. Value type V is carried as JsValue in Rust (for generic Pattern) or as a dedicated Subject wrapper. TypeScript declarations define `Pattern<V>` as a generic interface/class so that in TS, `Pattern<Subject>` and `Pattern<unknown>` are expressed for type checking and IDE support; at runtime the JS object is the same.

**Rationale**:
- JS has no static generics; runtime representation is one “Pattern” type
- TypeScript generics are erased at runtime; we only need .d.ts to declare `interface Pattern<V> { value: V; elements: Pattern<V>[]; ... }` and method signatures with generic return types (e.g. `map<W>(fn: (v: V) => W): Pattern<W>`)
- Python binding uses separate Pattern vs PatternSubject where useful; for TS, a single generic `Pattern<V>` with V = Subject or unknown is sufficient and matches spec FR-011

**Alternatives Considered**:
- **Multiple WASM classes (PatternGeneric, PatternSubject)**: Possible but duplicates API surface; TypeScript can express both with one generic. Chosen to keep one Pattern at boundary, TypeScript for generics.
- **No generics in TypeScript**: Would fail spec FR-010/FR-011 (Pattern&lt;V&gt; type definitions). Rejected.

**Trade-offs**:
- ✅ Pro: One implementation at boundary; TypeScript adds type safety without runtime cost
- ✅ Pro: Matches spec: “TypeScript definitions MUST include a generic Pattern&lt;V&gt; type”
- ⚠️ Con: Rust side may need to box or use enum for “value” (JsValue vs Subject) to support both generic and Subject-specific flows; acceptable

**Implementation Impact**:
- WASM: Export `Pattern` with `value: JsValue`, `elements: Vec<Pattern>` (or equivalent that crosses boundary)
- TypeScript: `pattern_core.d.ts` declares `export interface Pattern<V> { readonly value: V; readonly elements: Pattern<V>[]; ... }` and methods with appropriate generics

---

## Decision: TypeScript Definition Authoring

**Date**: 2026-01-31  
**Question**: Hand-written .d.ts vs generated from Rust/wasm-bindgen?  
**Decision**: Hand-written TypeScript declaration file(s) for the public WASM API, with full generic Pattern&lt;V&gt;, Subject, Value, Symbol, and all method signatures. Keep in repo (e.g. `crates/pattern-core/typescript/` or `pkg/` after wasm-pack) and ship with the WASM package so consumers get types without extra steps.

**Rationale**:
- wasm-bindgen does not emit TypeScript; wasm-pack can emit a minimal .d.ts but often incomplete for complex types and generics
- Hand-written allows precise Pattern&lt;V&gt;, map&lt;W&gt; return types, and JSDoc
- Single source of truth for “what the JS API looks like”; can be validated by type-checking a small TS test file

**Alternatives Considered**:
- **Generated from Rust**: No mature tool that outputs TypeScript generics from Rust generics; would require custom codegen. Rejected for now.
- **No .d.ts**: Fails FR-010/FR-011. Rejected.

**Trade-offs**:
- ✅ Pro: Full control over generics and documentation
- ⚠️ Con: Must be kept in sync with wasm.rs API manually (mitigated by contracts and tests)

**Implementation Impact**:
- Add `pattern_core.d.ts` (or similar) under crate or pkg; document in contracts/typescript-types.md
- CI or pre-publish step: run `tsc --noEmit` on a small TS file that uses Pattern&lt;Subject&gt; and key methods

---

## Decision: Error Handling Across WASM Boundary (Result / Either at Boundary)

**Date**: 2026-01-31 (updated to align with spec clarifications)  
**Question**: How should Rust Result/errors be surfaced to JavaScript?  
**Decision**: Preserve Rust’s Result at the WASM boundary. Fallible operations (e.g. `validate`, `traverse_result`) MUST return an Either-like value (same shape as effect-ts Either.right/Either.left or a documented one-line conversion), not throw. The Rust implementation already returns `Result` for these operations; wasm-bindgen bindings MUST NOT convert `Result::Err` to a JS throw for fallible APIs—instead return a value that is trivially convertible to effect-ts Either. Document the return shape and usage with effect-ts in the API contract, TypeScript types, and quickstart.

**Rationale**:
- Rust pattern-core already uses `Result` for fallible operations (e.g. `validate` → `Result<(), ValidationError>`; `traverse_result`, `sequence_result`, `validate_all`)
- Spec FR-016 / SC-009: fallible results must be trivially convertible for effect-ts Either; user requirement for functional programming and Either compatibility
- Matching Rust Result at the boundary avoids a separate “throw + helper” layer and keeps one mental model (Result/Either) across Rust and JS/TS
- effect-ts Either uses `{ _id: 'Either', _tag: 'Right', right: T }` | `{ _id: 'Either', _tag: 'Left', left: E }`; we return a shape compatible with that or document a one-line conversion

**Alternatives Considered**:
- **Throw on Err (wasm-bindgen default)**: Would require a helper to convert try/catch to Either; spec requires “trivially convertible” without a separate helper for the primary API. Rejected for fallible operations.
- **Dual API (throw + return Either variant)**: Adds surface area; single return-Either API is sufficient and matches Rust. Rejected.

**Implementation Impact**:
- In wasm.rs, fallible functions (validate, traverse_result, etc.) return a JS object representing Ok/Err (e.g. `{ _tag: 'Right', right: value }` | `{ _tag: 'Left', left: error }` or equivalent) so that they do not throw; use wasm_bindgen in a way that preserves the return value (e.g. custom serialization or a wrapper type that does not trigger throw-on-Err)
- TypeScript: fallible methods are typed as returning `Either<T, E>` or a compatible type; document in JSDoc and in quickstart how to use with effect-ts (e.g. `import { Either } from 'effect'` and pass return value as Either or one-line conversion)
- API contract (wasm-api.md) and quickstart MUST document the return shape and effect-ts usage

---

## Decision: Callbacks (map, filter, etc.) Across WASM

**Date**: 2026-01-31  
**Question**: How are JS functions passed to Rust (e.g. for map, filter) and invoked from WASM?  
**Decision**: Use wasm-bindgen’s `js_sys::Function` (or `Closure`) to accept JS callbacks. When Rust calls the callback, it passes JsValue arguments and receives JsValue return; convert to/from Rust types inside the WASM layer. Use synchronous callbacks only (spec: callbacks are synchronous).

**Rationale**:
- wasm-bindgen supports `Fn(JsValue) -> JsValue`-style closures and passing them as JS functions
- Python binding accepts callables and calls them; parity means we support the same operations with JS functions
- Synchronous only keeps implementation and spec aligned; no async/await in this feature

**Alternatives Considered**:
- **No callbacks in WASM (only pre-defined ops)**: Would severely limit parity (no map/filter with user logic). Rejected.
- **Async callbacks**: Spec out of scope; adds complexity. Deferred.

**Trade-offs**:
- ✅ Pro: Full parity with Python for map, filter, fold, extend
- ⚠️ Con: Callback overhead (JS ↔ WASM) per element; acceptable for correctness and spec SC-006 (within 2x of Python)

**Implementation Impact**:
- In wasm.rs, for each method that takes a callback (map, filter, fold, extend), accept a `js_sys::Function` or `&Closure<...>`, and invoke it from Rust with JsValue args; convert return value back to Rust types as needed

---

## Decision: Paramorphism (para) at WASM Boundary

**Date**: 2026-01-31  
**Question**: Should WASM expose paramorphism (para) to close the feature gap with Rust pattern-core (025)?  
**Decision**: Yes. Expose `pattern.para(fn: (value, elementResults[]) => result)` at the WASM/JS boundary with behavior equivalent to Rust pattern-core para. Parity MUST close the gap with the Rust API; para exists in Rust (025-pattern-paramorphism) and MUST be exposed in WASM even if not yet in Python (spec FR-017, SC-002).

**Rationale**:
- Spec clarification: parity means closing the feature gap with the Rust pattern-core API; WASM MUST expose all pattern-core operations that exist in Rust, including para.
- Rust already implements para in pattern-core; WASM bindings wrap the same implementation and expose it with a JS callback (value, array of child results) → result, bottom-up.

**Implementation Impact**:
- wasm-api.md and typescript-types.md: add `para` to Pattern (Transformation section). TypeScript: `para<R>(fn: (value: V, elementResults: R[]) => R): R`.
- In wasm.rs, expose a method that calls Rust’s para, converting (value, child results) to/from JsValue at the boundary; callback receives (JsValue, JsValue[]), returns JsValue.

---

## Decision: Parity Verification Approach

**Date**: 2026-01-31  
**Question**: How do we verify WASM behavior matches Python?  
**Decision**: Reuse the same logical test cases as Python: build a small corpus of patterns (atomic, nested, Pattern&lt;Subject&gt;), apply the same operations (construct, map, filter, combine, depth, size, etc.), and compare outputs. Prefer shared test data (e.g. JSON or canonical representation) that both Python and WASM tests consume, with Rust tests driving WASM via wasm-bindgen-test or Node.

**Rationale**:
- Spec SC-002: “equivalent results for the same logical input”
- Reduces drift between Python and WASM implementations
- Contract tests (contracts/wasm-api.md) define the API; parity tests define behavior

**Alternatives Considered**:
- **No cross-runtime tests**: Risk of subtle behavioral drift. Rejected.
- **Only unit tests per runtime**: Good but not sufficient for parity; shared corpus preferred.

**Implementation Impact**:
- Add tests that load WASM, construct patterns, run operations, and assert on structure/values (and optionally compare to Python output for same inputs)
- Document parity corpus location and format in research or contracts
