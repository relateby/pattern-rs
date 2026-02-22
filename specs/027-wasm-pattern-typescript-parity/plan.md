# Implementation Plan: WASM Feature Parity with Python and Pattern&lt;V&gt; TypeScript Generics

**Branch**: `027-wasm-pattern-typescript-parity` | **Date**: 2026-01-31 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/027-wasm-pattern-typescript-parity/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Deliver WASM bindings and TypeScript type definitions for pattern-core so that JavaScript/TypeScript consumers have feature parity with the Rust pattern-core API. Parity MUST close the feature gap with the Rust API: expose all pattern-core operations (map, filter, combine, para, depth, size, comonad operations, validate, etc.) with behavior equivalent to Rust and, where applicable, to the Python binding (024). Fallible operations MUST return Either-like values at the WASM boundary (trivially convertible to effect-ts Either), not throw. TypeScript definitions MUST include generic `Pattern<V>` and hand-written .d.ts for the public WASM API. Technical approach: wasm-bindgen feature-gated module in pattern-core, JsValue at boundary, hand-written TypeScript declarations.

## Technical Context

**Language/Version**: Rust (existing workspace; same toolchain as pattern-core)  
**Primary Dependencies**: wasm-bindgen (optional / target cfg for wasm32); js-sys, web-sys as needed for callbacks and bindings  
**Storage**: N/A (in-memory pattern structure; no persistence in this feature)  
**Testing**: cargo test (native); wasm-pack test or wasm-bindgen-test / Node for WASM; shared parity corpus with Python where applicable  
**Target Platform**: WASM (web and Node); pattern-core remains multi-target (native + WASM)  
**Project Type**: Library (multi-crate workspace; changes in crates/pattern-core: wasm module, TypeScript types)  
**Performance Goals**: WASM pattern operations within 2x of Python binding for patterns up to 1000 nodes (SC-006)  
**Constraints**: WASM-compatible (no blocking I/O); fallible APIs return Either-like, do not throw; callbacks synchronous only  
**Scale/Scope**: Full pattern-core API surface (constructors, accessors, inspection, query, transformation including para, combination, comonad, validate); TypeScript generics for Pattern&lt;V&gt;, Subject, Value, Symbol

## CRITICAL Design Clarifications

### Pattern<V> is Generic Over ANY Value Type

**MUST understand**: Pattern<V> in Rust is **fully generic** over any value type V. This is fundamental to the entire design.

**Python binding (correct model)**:
```python
fn point(py: Python, value: &Bound<'_, PyAny>) -> PyResult<Self> {
    // Accepts PyAny - literally ANY Python object
    Ok(Self { value: value.clone().unbind(), elements: vec![] })
}
```
- Python stores `PyAny` (any Python object) directly in patterns
- Can be primitives (int, str, bool), dicts, lists, **PySubject**, or even **other Patterns** (nesting)
- Zero type restrictions - user chooses value type per pattern

**WASM binding MUST mirror this**:
```rust
#[wasm_bindgen]
pub struct WasmPattern {
    inner: Pattern<JsValue>,  // ✅ NOT Pattern<Subject>!
}

pub fn point(value: JsValue) -> WasmPattern {
    // Accepts JsValue - literally ANY JavaScript value
    WasmPattern { inner: Pattern::point(value) }
}
```
- Store `JsValue` (any JavaScript value) directly in patterns
- Can be primitives, objects, **WasmSubject**, or even **other WasmPatterns** (nesting)
- Zero type restrictions - user chooses value type per pattern

**What NOT to do**:
- ❌ Hardcode `Pattern<Subject>` - this is ONE USE CASE, not the only one
- ❌ Force conversion of primitives to Subject - this loses generality
- ❌ Restrict `point(value)` to only accept WasmSubject - must accept any JsValue
- ❌ Make `of()` different from `point()` - they MUST be aliases

### Pattern.of() and Pattern.point() MUST Be Identical

**Spec requirement**: "`Pattern.of(value: JsValue) -> Pattern` — alias for point"

**Python implementation**:
```python
fn of(py: Python, value: &Bound<'_, PyAny>) -> PyResult<Self> {
    Self::point(py, value)  // Literally just calls point - zero logic
}
```

**WASM MUST do the same**:
```rust
pub fn of(value: JsValue) -> WasmPattern {
    Self::point(value)  // Just delegate - both accept any JsValue
}
```

The name `of` follows functional programming convention (Functor's "lift"), while `point` is explicit about creating an atomic pattern. They are **semantically identical** - just naming choices for different programming styles.

### Subject is a Value Type, Not THE Value Type

**Correct understanding**:
- Subject is **one specific value type** you can put in patterns
- Users can also create `Pattern.point(42)`, `Pattern.point("hello")`, `Pattern.point(anotherPattern)`
- `Pattern<Subject>` is the **most common use case**, but not the only one
- Python allows `Pattern.point(subject)` alongside `Pattern.point(42)` - WASM MUST match

**Implementation strategy**:
1. WasmPattern wraps `Pattern<JsValue>` (generic storage)
2. WasmSubject is a separate WASM type that can be **converted to/from JsValue**
3. Users can: `Pattern.point(subject.toJsValue())` or provide implicit conversions
4. The pattern itself doesn't care - it just holds the JsValue

### Parity Requirement: Match Python Exactly

From the spec: "For the same logical input, the result of any operation MUST match the result of the equivalent Python binding operation."

This means:
- If Python accepts `PyAny`, WASM must accept `JsValue` (equivalent: any value)
- If Python allows `Pattern.point(pattern)` (nesting), WASM must too
- If Python has `of()` as an alias for `point()`, WASM must too (not different logic)
- Behavior must be **identical**, not "similar" or "inspired by"

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|--------|
| **I. Reference Implementation Fidelity** | PASS | Behavior MUST match gram-hs reference at `../pattern-hs` and Rust pattern-core. Where Python (024) exposes an operation, behavior MUST match Python; where Rust has an operation (e.g. para from 025) not yet in Python, WASM MUST expose it. Parity tests and shared corpus verify equivalence. |
| **II. Correctness & Compatibility** | PASS | API contracts (contracts/wasm-api.md, typescript-types.md) and data-model.md define the boundary; fallible operations preserve Rust Result as Either-like at boundary. No breaking changes to existing Rust API. |
| **III. Rust Native Idioms** | PASS | Rust side uses Result, references, idiomatic pattern-core; WASM layer converts to/from JsValue and Either-like return shape only at the boundary. |
| **IV. Multi-Target Library Design** | PASS | WASM code isolated behind feature flag; native and Python builds unchanged. Public APIs compatible with WASM constraints (no blocking I/O). |
| **V. External Language Bindings & Examples** | PASS | Minimal working examples for WASM (browser and Node) and TypeScript usage; quickstart.md and package README document build, load, and effect-ts usage. |

**Note**: When porting features from gram-hs, reference the local implementation at `../pattern-hs` and corresponding feature specifications in `../pattern-hs/specs/`. See [porting guide](../../docs/porting-guide.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/027-wasm-pattern-typescript-parity/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   ├── wasm-api.md
│   └── typescript-types.md
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── Cargo.toml            # Optional [target.'cfg(target_arch = "wasm32")'].dependencies (wasm-bindgen, js-sys)
├── src/
│   ├── lib.rs            # Conditional re-export of wasm module when feature "wasm" enabled
│   ├── pattern.rs        # Existing; para and other methods already present (025)
│   ├── pattern/
│   │   ├── comonad.rs
│   │   └── comonad_helpers.rs
│   ├── subject.rs
│   ├── python.rs         # Unchanged for this feature
│   ├── wasm.rs           # New (or existing): #[wasm_bindgen] types and exports for Pattern, Subject, Value, etc.
│   └── test_utils/
├── typescript/           # Or pkg/ after wasm-pack: hand-written .d.ts (pattern_core.d.ts)
│   └── pattern_core.d.ts
└── tests/                # Unit + parity tests; WASM tests via wasm-bindgen-test or Node

examples/
├── pattern-core/         # Existing Rust examples
├── wasm-js/               # Existing or new: minimal WASM + JS/TS example (load, construct, map, para, validate)
```

**Structure Decision**: Single-crate extension within the existing pattern-rs workspace. WASM bindings live in `crates/pattern-core/src/wasm.rs` behind a feature (e.g. `wasm`). TypeScript declarations live in `crates/pattern-core/typescript/` (or equivalent) and are shipped with the WASM package. Examples in `examples/wasm-js` or under `examples/pattern-core` demonstrate browser/Node and TypeScript usage, including effect-ts for Either-like returns.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations. Table left empty.
