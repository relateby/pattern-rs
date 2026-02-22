# Implementation Plan: Predicate-Based Pattern Matching

**Branch**: `016-predicate-matching` | **Date**: 2025-01-05 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/016-predicate-matching/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Complete the predicate-based pattern matching implementation for the Pattern type by adding the remaining missing functions. Most functionality is already implemented: `any_value`, `all_values`, and `filter` are complete and working. This feature adds three new functions: (1) `find_first` to find the first matching subpattern returning Option<&Pattern<V>>, (2) `matches` for structural equality checking beyond Eq, and (3) `contains` for subpattern containment checking. All new functions will follow Rust idioms using Option for optional results, borrowed references for zero-cost abstraction, Fn trait bounds for reusable predicates, and depth-first pre-order traversal consistent with existing operations. The implementation will maintain behavioral equivalence with gram-hs reference implementation while using idiomatic Rust patterns.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)  
**Primary Dependencies**: None (std library only for core functionality)  
**Storage**: N/A (pure functional data structure)  
**Testing**: cargo test (built-in), proptest ^1.0 (property-based testing), insta ^1.34 (snapshot testing), criterion ^0.5 (benchmarks)  
**Target Platform**: Cross-platform (native Rust + WASM32-unknown-unknown)  
**Project Type**: Multi-crate workspace library (pattern-core crate)  
**Performance Goals**: find_first operations complete within 10ms for patterns with 1000 nodes when match found in first 10 nodes. Structural matching (matches/contains) complete within 100ms for patterns with 1000 nodes and nesting depth up to 100 levels.  
**Constraints**: Must maintain behavioral equivalence with gram-hs reference implementation. Must compile for WASM target. Must use idiomatic Rust (Option not Result, borrowed references, Fn trait, depth-first pre-order traversal). Must handle deeply nested structures (100+ levels) without stack overflow.  
**Scale/Scope**: 3 new functions to add to existing Pattern<V> implementation in crates/pattern-core/src/pattern.rs

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**Reference Implementation Fidelity**: ✅ Feature ports predicate matching from gram-hs located at `../pattern-hs/specs/012-predicate-matching/`. Reference implementation provides authoritative behavior for all predicate and matching operations. Behavioral equivalence will be verified through test cases comparing results with gram-hs output. The Rust implementation will maintain functional equivalence while using idiomatic Rust patterns (Option instead of Maybe, borrowed references instead of immutable values, Fn trait instead of function types).

**Correctness & Compatibility**: ✅ Implementation prioritizes correctness over optimization. All functions will have comprehensive unit tests, property-based tests, and edge case coverage matching gram-hs behavior. API contracts will be documented and verified through integration tests. Breaking changes from reference behavior require explicit justification.

**Rust Native Idioms**: ✅ Implementation uses idiomatic Rust patterns:
- Option<&Pattern<V>> for optional results (not Result, not panic)
- Iterator return types for lazy evaluation (existing filter already returns Vec, new find_first returns Option)
- Borrowed references (&Pattern<V>) to avoid unnecessary cloning
- Fn trait bounds for reusable predicates (not FnMut or FnOnce)
- Depth-first pre-order traversal consistent with existing fold/map
- snake_case naming (find_first, not findFirst)

**Multi-Target Library Design**: ✅ All functions compatible with WASM compilation. No platform-specific code paths. No blocking I/O or file system access. Uses only std library functionality available in WASM (no threads, no async required). Will verify WASM compilation as part of test suite.

**External Language Bindings & Examples**: N/A for this feature. Pattern matching functions are Rust-native and don't require changes to existing WASM bindings or examples. Future work may expose these functions through WASM interface if needed.

**Note**: When porting features from gram-hs, reference the local implementation at `../pattern-hs` and corresponding feature specifications in `../pattern-hs/specs/`. See [porting guide](../../../docs/porting-guide.md) for detailed porting instructions.

### Post-Phase 1 Design Review

**Reference Implementation Fidelity**: ✅ Design artifacts complete. Type signatures in contracts/type-signatures.md match gram-hs reference semantics. Data model in data-model.md documents behavioral equivalence requirements. Quickstart examples demonstrate expected behavior matching gram-hs output. All functions maintain functional equivalence while using idiomatic Rust patterns.

**Correctness & Compatibility**: ✅ Contracts define comprehensive test requirements including unit tests, property tests, equivalence tests, and performance tests. All edge cases documented (atomic patterns, empty elements, deep nesting, no matches). API contracts specify preconditions, postconditions, and behavioral properties. Breaking changes: none (all additions, backward compatible).

**Rust Native Idioms**: ✅ Design uses idiomatic Rust patterns throughout:
- Option<&Pattern<V>> for optional results (not Result, not panic)
- Borrowed references for zero-cost abstraction
- Fn trait bounds for reusable predicates (documented in data-model.md)
- Depth-first pre-order traversal consistent with existing operations
- snake_case naming (find_first, not findFirst)
- PartialEq bounds where needed (matches, contains)

**Multi-Target Library Design**: ✅ All functions use only std library features available in WASM. No platform-specific code. No blocking I/O or file system access. Stack usage within WASM limits (documented in contracts). Compatibility verified in contracts/type-signatures.md.

**External Language Bindings & Examples**: N/A - No changes to WASM bindings required. Functions are Rust-native and integrate with existing Pattern API. Future WASM exposure can be added if needed without API changes.

*All constitution checks pass after Phase 1 design. Ready to proceed to Phase 2 (task generation via /speckit.tasks command).*

## Project Structure

### Documentation (this feature)

```text
specs/016-predicate-matching/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── type-signatures.md
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── src/
│   ├── lib.rs           # No changes (Pattern already exported)
│   └── pattern.rs       # Add 3 new methods to Pattern<V> impl:
│                        #   - find_first(&self, F) -> Option<&Pattern<V>>
│                        #   - matches(&self, &Pattern<V>) -> bool (requires V: PartialEq)
│                        #   - contains(&self, &Pattern<V>) -> bool (requires V: PartialEq)
│
├── tests/
│   ├── query_any_value.rs      # Already exists (66 tests passing)
│   ├── query_all_values.rs     # Already exists (66 tests passing)
│   ├── query_filter.rs          # Already exists (66 tests passing)
│   ├── query_find_first.rs      # NEW: Unit tests for find_first
│   ├── predicate_matches.rs     # NEW: Unit tests for matches
│   ├── predicate_contains.rs    # NEW: Unit tests for contains
│   └── predicate_properties.rs  # NEW: Property-based tests
│
└── benches/
    └── predicate_benchmarks.rs  # NEW: Performance benchmarks
```

**Structure Decision**: Single crate (pattern-core) in existing multi-crate workspace. New functions added as methods on Pattern<V> in src/pattern.rs, alongside existing any_value, all_values, and filter methods. Tests follow existing pattern of one test file per function plus a properties file for cross-function properties. This maintains consistency with existing codebase structure where query functions (any_value, all_values, filter, length, size, depth, values) are all methods on Pattern<V>.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

*No violations. All constitution checks pass without justification needed.*
