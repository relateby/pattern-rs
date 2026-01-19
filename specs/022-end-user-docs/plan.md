# Implementation Plan: End-user documentation

**Feature Branch**: `022-end-user-docs`  
**Created**: 2026-01-19  
**Status**: Draft  
**Spec**: [spec.md](spec.md)

## Technical Context

### Existing Infrastructure
- **Crate `pattern-core`**: Defines the fundamental `Pattern<V>` data structure. Key constructors: `Pattern::point(v)` and `Pattern::pattern(v, elements)`.
- **Crate `gram-codec`**: Handles parsing and serialization between Gram notation and `Pattern` structures.
- **Reference Documentation**: `../gram-hs/docs/guide/` contains a comprehensive guide for the Haskell implementation.
- **CLI Tools**: `gramref` (formerly `gram-hs`) is used for validation and test generation.

### Target Documentation Structure
- `docs/introduction.md`: High-level concepts, "decorated sequence" model.
- `docs/gram-notation.md`: Detailed reference for Gram syntax and its mapping to `Pattern`.
- `docs/rust-usage.md`: Practical guide for using `gram-rs` crates in Rust projects.
- `README.md`: Update to link to these new guides.

### Unknowns & Research
- [NEEDS CLARIFICATION: Should the Rust usage guide focus on `pattern-core` or `gram-codec` as the primary entry point for new users?]
- [NEEDS CLARIFICATION: Are there any planned API changes in `pattern-core` that should be documented as "experimental" or "upcoming"?]

## Constitution Check

| Principle | Check | Status |
|-----------|-------|--------|
| **I. Reference Fidelity** | Documentation must align with `gram-hs` conceptual model. | [ ] |
| **II. Correctness** | Examples provided must be tested and correct. | [ ] |
| **III. Rust Native Idioms** | Documentation style should follow Rust conventions (e.g., using `cargo` commands). | [ ] |
| **IV. Multi-Target** | Documentation should mention WASM and other targets. | [ ] |
| **V. Examples** | Include minimal examples for Rust and mention external bindings. | [ ] |

## Gates

- [ ] **Research Gate**: All [NEEDS CLARIFICATION] items resolved in `research.md`.
- [ ] **Design Gate**: `data-model.md` and `contracts/` (if applicable) defined.
- [ ] **Quality Gate**: Plan reviewed for consistency and completeness.

---

## Phase 0: Outline & Research

1. **Research Task 1**: Compare `gram-hs` guide structure with current `gram-rs` implementation to identify gaps.
2. **Research Task 2**: Verify which `gram-rs` features (like specific relationships) are fully supported in `gram-codec` to avoid documenting unsupported features.
3. **Research Task 3**: Determine the best way to present WASM/Python examples in the documentation.

**Output**: `research.md`

## Phase 1: Design & Documentation Plan

1. **Draft `introduction.md`**: Adapt "What is a Pattern?" and "Why Patterns?" from `gram-hs`.
2. **Draft `gram-notation.md`**: Create a concise reference for nodes, relationships, and annotations.
3. **Draft `rust-usage.md`**: Provide code snippets for:
   - Adding dependencies
   - Creating patterns programmatically
   - Parsing from gram notation
   - Basic queries (any_value, all_values)
4. **Update `README.md`**: Add "Documentation" section.
5. **Update Agent Context**: Run script to update `cursor-agent` context with documentation knowledge.

**Output**: `introduction.md`, `gram-notation.md`, `rust-usage.md`, updated `README.md`

---

## Phase 2: Implementation (Reserved for Task Agent)

*This section will be detailed by the Task Agent based on the finalized plan.*
