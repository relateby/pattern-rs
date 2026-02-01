# Implementation Plan: Unified Gram WASM Package

**Branch**: `028-unified-gram-wasm` | **Date**: 2026-01-31 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/028-unified-gram-wasm/spec.md`

## Summary

Deliver a single consumable WASM package that unifies pattern-core and gram-codec so JavaScript/TypeScript users get one import for Pattern types and gram serialization (stringify/parse). The package composes existing Rust crates; conversion between Pattern&lt;Subject&gt; and gram-codec AST stays internal. Users work only with Pattern, Subject, Value, and a Gram namespace (stringify, parse, parseOne, from). Round-trip equivalence and conventional conversion from Pattern&lt;V&gt; to Pattern&lt;Subject&gt; are required. The crate is named **pattern-wasm** to reflect that the pattern data structure is the dominant feature.

**Conventional conversion (design)**: **Subject.fromValue(value, options?)** implements the convention for turning arbitrary JS values into Subjects (string, number, boolean, object, Subject passthrough with defaults for identity, label, value property). **Gram.from(pattern, options?)** is implemented as `pattern.map(v => Subject.fromValue(v, options))`, so the convention lives on Subject and Gram.from is convenience sugar. Users can also do `pattern.map(Subject.fromValue)` then `Gram.stringify(...)` explicitly.

## Technical Context

**Language/Version**: Rust 1.70+ (workspace MSRV), edition 2021  
**Primary Dependencies**: pattern-core (path), gram-codec (path), wasm-bindgen 0.2, js-sys 0.3  
**Storage**: N/A (in-memory patterns; no persistence in scope)  
**Testing**: cargo test (native), wasm-pack test / Node/browser for WASM; equivalence with gram-codec round-trip  
**Target Platform**: WebAssembly (wasm32-unknown-unknown); consumed from JavaScript/TypeScript (browser and Node)  
**Project Type**: Single library crate (pattern-wasm) in existing workspace  
**Performance Goals**: Parse/stringify latency acceptable for typical document sizes; no explicit target in spec  
**Constraints**: WASM-compatible APIs only (no blocking I/O, no file system); single entry point for JS consumers  
**Scale/Scope**: One new crate; re-use pattern-core and gram-codec; unified TypeScript definitions

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|--------|
| **I. Reference Implementation Fidelity** | Pass | gram-codec and pattern-core already align with gram-hs; unified crate composes them without changing semantics. No new gram semantics. |
| **II. Correctness & Compatibility** | Pass | Round-trip and API contracts preserve existing behavior. |
| **III. Rust Native Idioms** | Pass | pattern-wasm is Rust; WASM bindings follow existing pattern-core/gram-codec wasm-bindgen style. |
| **IV. Multi-Target Library Design** | Pass | pattern-wasm targets WASM only; pattern-core and gram-codec remain multi-target. |
| **V. External Language Bindings & Examples** | Pass | Single JS/TS entry point and TypeScript definitions; examples updated per workflow. |

**Re-check after Phase 1 design**: All gates still pass. data-model.md and contracts/ define the JS/WASM surface; quickstart aligns with spec SC-001.

**Note**: When porting features from gram-hs, reference the local implementation at `../gram-hs` and corresponding feature specifications in `../gram-hs/specs/`. See [porting guide](../../docs/porting-guide.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/028-unified-gram-wasm/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (unified WASM API, TypeScript surface)
└── tasks.md             # Phase 2 output (/speckit.tasks - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/
├── pattern-core/        # Existing: Pattern<V>, Subject, Value; wasm feature
├── gram-codec/          # Existing: parse_gram_notation, to_gram_pattern; wasm feature
└── pattern-wasm/        # NEW: unified WASM package (pattern-first naming)
    ├── Cargo.toml       # Depends on pattern-core, gram-codec, wasm-bindgen, js-sys
    ├── src/
    │   ├── lib.rs       # WASM entry; re-export or wrap Pattern, Subject, Value; Gram namespace
    │   ├── gram.rs      # Gram::stringify, parse, parseOne, from (from = pattern.map(Subject.fromValue))
    │   └── convert.rs   # Internal: Rust Pattern<Subject> ↔ JS bindings; Subject.fromValue logic
    └── typescript/
        └── gram.d.ts    # Unified TypeScript definitions (single import surface)

examples/
└── wasm-js/             # Update to consume pattern-wasm when available (optional follow-up)
```

**Structure Decision**: One new crate **pattern-wasm** under `crates/`. It depends on existing `pattern-core` and `gram-codec` and exposes a single WASM surface (Pattern, Subject, Value, Gram). Subject.fromValue implements conventional value→Subject conversion; Gram.from delegates to it. No new REST/HTTP surface; contracts describe the JS/WASM API and TypeScript types.

## Complexity Tracking

> No constitution violations. This section is empty.
