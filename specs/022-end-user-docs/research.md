# Research: End-user documentation

## Research Task 1: Reference vs Implementation Gaps

**Decision**: The documentation will focus on the core `Pattern` data structure and `gram-codec` parser/serializer, as these are the most mature parts of `gram-rs`.

**Rationale**: `gram-rs` is a faithful port of `gram-hs`. The conceptual model is identical. Some advanced morphisms (e.g., paramorphisms) are still being ported, so the documentation will stick to basic operations and typeclass instances (Functor, Foldable, Traversable) that are already implemented.

**Alternatives Considered**: Including advanced category theory sections. Rejected because they are not yet fully implemented or may overwhelm new users.

---

## Research Task 2: Primary Entry Point

**Decision**: Present `pattern-core` as the foundational data structure and `gram-codec` as the interface for working with Gram notation.

**Rationale**: Most users will start by parsing Gram notation (`gram-codec`) but will quickly need to manipulate the results (`pattern-core`). Showing them how these interact provides the clearest path to value.

**Findings**:
- `pattern-core` provides `Pattern<V>`.
- `gram-codec` provides `parse_gram(input)` and `to_gram(pattern)`.

---

## Research Task 3: Polyglot & Multi-Target Presentation

**Decision**: Mention WASM, Python, and other targets in the introduction to highlight the library's design philosophy, but focus implementation details on Rust in `rust-usage.md`.

**Rationale**: The user query specifically asked for "basic usage in rust". Mentioning other targets establishes the library as a multi-environment solution without cluttering the Rust-specific guide.

---

## Resolution of NEEDS CLARIFICATION

1. **Focus for Rust usage guide**: It will cover both `pattern-core` (data structure) and `gram-codec` (parsing/serialization) as an integrated workflow.
2. **Planned API changes**: None that affect core `point` or `pattern` constructors. Some internal changes in `gram-codec` parser structure are ongoing but don't affect public API.
