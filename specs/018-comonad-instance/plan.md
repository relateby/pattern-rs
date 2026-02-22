# Implementation Plan: Comonad Operations for Pattern

**Branch**: `018-comonad-instance` | **Date**: 2026-01-05 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/018-comonad-instance/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Implement Comonad operations (`extract` and `extend`) for Pattern to enable position-aware decorations. Pattern's "decorated sequence" semantics (where value decorates elements) make Comonad the conceptually correct abstraction for computing new decorative information based on subpattern context. This includes helper functions (`depth_at`, `size_at`, `indices_at`) that demonstrate practical applications of context-aware decoration.

**Technical Approach**: Implement `extract` (trivial accessor) and `extend` (recursive application of context-aware function) as direct methods on Pattern. Use `extend` to implement `depth_at` and `size_at` for conceptual consistency. Implement `indices_at` directly (requires path tracking). Verify correctness through property-based testing of Comonad laws and behavioral equivalence with gram-hs.

## Technical Context

**Language/Version**: Rust 1.75+ (stable features only)
**Primary Dependencies**: 
- `proptest` (property-based testing for Comonad laws)
- Standard library only for implementation (no external dependencies)
**Storage**: N/A (pure in-memory data structure operations)
**Testing**: 
- `cargo test` with property-based tests (`proptest`)
- Behavioral equivalence tests against gram-hs reference implementation
- Unit tests for helper functions
**Target Platform**: Multi-target (native Rust, WASM)
- No platform-specific code required
- Pure functional operations (no I/O, no blocking)
**Project Type**: Library (shared crate)
**Performance Goals**: 
- `extract`: O(1) - direct field access
- `extend`: O(n) where n = node count - single traversal
- Helper functions: O(n) - single traversal each
- Process 1000+ element patterns in <100ms
**Constraints**: 
- No Clone bound on value type V (pass function by reference)
- Must satisfy three Comonad laws (property-tested)
- Behavioral equivalence with gram-hs reference implementation
- WASM-compatible (no blocking operations, pure functions)
**Scale/Scope**: 
- 4 new public methods on Pattern<V>
- 3 Comonad law tests (property-based)
- 3 helper function test suites
- Documentation with conceptual explanation of "decorated sequences"

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### ✅ I. Reference Implementation Fidelity

**Status**: PASS - Faithful port with documented conceptual enhancement

**Verification**:
- ✅ Haskell implementation reviewed: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (lines 720-728, 1104-1138)
- ✅ Tests reviewed: `../pattern-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` (lines 4242-4400), `Properties.hs` (lines 1287-1332)
- ✅ Feature spec reviewed: `../pattern-hs/specs/014-comonad-instance/spec.md`
- ✅ Behavioral equivalence plan: Property tests will verify Comonad laws match gram-hs behavior
- ✅ Output comparison: Helper functions (`depth_at`, `size_at`, `indices_at`) will be tested against gram-hs outputs

**Intentional Enhancement**:
- gram-hs implements `depthAt = extend depth` (uses Comonad)
- gram-hs implements `sizeAt` directly (does NOT use Comonad)
- **pattern-rs will use `extend` for both** for conceptual consistency
- Rationale: Makes "decorative computation" pattern explicit without changing behavior

**Reference Paths**:
- Haskell source: `../pattern-hs/libs/pattern/src/Pattern/Core.hs`
- Helper tests: `../pattern-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs`
- Law tests: `../pattern-hs/libs/pattern/tests/Spec/Pattern/Properties.hs`

### ✅ II. Correctness & Compatibility

**Status**: PASS - Prioritizes correctness with comprehensive law verification

**Verification**:
- ✅ Comonad laws will be property-tested (left identity, right identity, associativity)
- ✅ Helper functions will be unit-tested for correctness
- ✅ Behavioral equivalence tests will compare outputs with gram-hs
- ✅ No breaking changes to existing Pattern API
- ✅ New methods are additive only

**Correctness Guarantees**:
- Three Comonad laws verified through property-based testing
- Helper function outputs match gram-hs reference implementation
- Edge cases (atomic patterns, deeply nested, large patterns) explicitly tested

### ✅ III. Rust Native Idioms

**Status**: PASS - Idiomatic Rust implementation

**Idiomatic Rust Patterns**:
- ✅ Direct methods on `impl<V> Pattern<V>` (not a trait - no need for abstraction)
- ✅ Pass function by reference `&F` (no Clone bound required)
- ✅ Use `&self` for `extract` (returns reference, no ownership transfer)
- ✅ Consume `self` for `extend` if value transform needed, or use `&self` with Clone if appropriate
- ✅ Standard Rust naming: `snake_case` methods (`extract`, `extend`, `depth_at`, `size_at`, `indices_at`)
- ✅ Return references where possible (`extract` returns `&V`)
- ✅ Use `Vec` for indices (not Haskell list)

**Type Signatures**:
```rust
impl<V> Pattern<V> {
    pub fn extract(&self) -> &V { ... }
    pub fn extend<W, F>(&self, f: &F) -> Pattern<W> 
    where F: Fn(&Pattern<V>) -> W { ... }
    pub fn depth_at(&self) -> Pattern<usize> { ... }
    pub fn size_at(&self) -> Pattern<usize> { ... }
    pub fn indices_at(&self) -> Pattern<Vec<usize>> { ... }
}
```

### ✅ IV. Multi-Target Library Design

**Status**: PASS - Pure functions, no platform-specific code

**Multi-Target Compatibility**:
- ✅ No I/O operations
- ✅ No blocking operations
- ✅ No file system access
- ✅ Pure functional operations only
- ✅ WASM-compatible (all operations are pure)
- ✅ No platform-specific conditional compilation needed

**Testing Strategy**:
- Unit tests run on native Rust target
- Property tests run on native Rust target
- Integration tests can run on WASM (if needed)

### ✅ V. External Language Bindings & Examples

**Status**: PASS - Examples will demonstrate comonad operations

**Example Requirements**:
- ✅ Demonstrate `extract` usage
- ✅ Demonstrate `extend` with custom function
- ✅ Demonstrate helper functions (`depth_at`, `size_at`, `indices_at`)
- ✅ Show practical use case (e.g., annotating pattern for visualization)
- ✅ Document in module-level documentation with examples

**Note**: External language bindings (WASM, FFI) can expose these operations. No special binding considerations (pure functions, no callbacks).

### Summary

**All gates PASS** ✅

- Reference implementation fidelity: Faithful port with documented enhancement
- Correctness: Comprehensive law verification + behavioral equivalence testing
- Rust idioms: Direct methods, idiomatic naming, efficient patterns
- Multi-target: Pure functions, WASM-compatible
- Examples: Will demonstrate all operations with practical use cases

**Note**: When porting features from gram-hs, reference the local implementation at `../pattern-hs` and corresponding feature specifications in `../pattern-hs/specs/`. See [porting guide](../../../docs/porting-guide.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/018-comonad-instance/
├── spec.md             # Feature specification (user scenarios, requirements, success criteria)
├── plan.md             # This file (/speckit.plan command output)
├── research.md         # Phase 0 output: technical decisions and rationale
├── data-model.md       # Phase 1 output: Pattern structure and operations
├── quickstart.md       # Phase 1 output: quick start guide for using comonad operations
├── contracts/          # Phase 1 output: API contracts (type signatures, laws)
│   └── comonad.md      # Comonad operation signatures and law specifications
├── checklists/         # Quality checklists
│   └── requirements.md # Spec quality checklist (✅ complete)
├── ANALYSIS.md         # Original analysis document
├── RECOMMENDATION.md   # Updated recommendation document
└── README.md           # Overview document
```

### Source Code (repository root)

```text
crates/pattern-core/
├── src/
│   ├── pattern/
│   │   ├── mod.rs              # Pattern type definition (existing)
│   │   ├── comonad.rs          # NEW: Comonad operations (extract, extend)
│   │   └── comonad_helpers.rs  # NEW: Helper functions (depth_at, size_at, indices_at)
│   └── lib.rs                  # Re-export comonad operations (existing, updated)
│
└── tests/
    ├── comonad_laws.rs         # NEW: Property-based tests for Comonad laws
    ├── comonad_helpers.rs      # NEW: Unit tests for helper functions
    └── equivalence/            # NEW: Behavioral equivalence with gram-hs
        ├── mod.rs
        ├── depth_at.rs         # Compare depth_at outputs
        ├── size_at.rs          # Compare size_at outputs
        └── indices_at.rs       # Compare indices_at outputs
```

**Structure Decision**: Single library project (pattern-core crate). Comonad operations are added as new modules within the existing Pattern implementation. Tests are organized by concern: law verification (property-based), helper functionality (unit), and equivalence (integration with gram-hs outputs).

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

**Status**: N/A - No constitution violations. All gates pass.

## Phase 0: Research & Decisions

**Status**: COMPLETE (see [research.md](./research.md))

Key decisions documented:
1. Implementation strategy: Direct methods vs. trait abstraction
2. Function passing: By reference vs. by value
3. Helper implementation: Using `extend` vs. direct recursion
4. Testing approach: Property-based laws + behavioral equivalence

## Phase 1: Design Artifacts

**Status**: COMPLETE

Generated artifacts:
- ✅ [data-model.md](./data-model.md) - Pattern structure and operation semantics
- ✅ [contracts/comonad.md](./contracts/comonad.md) - API contracts and law specifications
- ✅ [quickstart.md](./quickstart.md) - Quick start guide with examples

## Phase 2: Implementation Tasks

**Status**: READY - Use `/speckit.tasks` to generate task breakdown

Tasks will be generated by `/speckit.tasks` command based on this plan.

## Implementation Notes

### Critical Considerations

1. **Conceptual Clarity**: Documentation must explain Pattern's "decorated sequence" semantics
   - Value decorates elements (not just another component)
   - Comonad operations work with decoration-as-information
   - `extend` enables context-aware decoration computation

2. **Performance**: All operations must be O(n) single-pass
   - `extract`: O(1) field access
   - `extend`: O(n) single traversal
   - Helpers: O(n) using extend

3. **Law Verification**: Property-based tests must cover:
   - Left identity: `extract(extend(f, p)) == f(p)`
   - Right identity: `extend(extract, p) == p`
   - Associativity: `extend(f, extend(g, p)) == extend(f ∘ extend(g), p)`

4. **Behavioral Equivalence**: Compare outputs with gram-hs for:
   - `depth_at` on various pattern structures
   - `size_at` on various pattern structures
   - `indices_at` on various pattern structures

### Integration Points

- **Existing Pattern API**: No changes, purely additive
- **Testing Infrastructure**: Leverage existing property test setup
- **Documentation**: Add module-level examples and conceptual explanation
- **Benchmarks**: Optional, but should verify O(n) performance

### Success Metrics

From spec.md Success Criteria:
- SC-001: ✅ Extract decorative values in single method call
- SC-002: ✅ Compute position-aware decorations with `extend`
- SC-003: ✅ 100% Comonad law verification (property tests)
- SC-004: ✅ 100% correct helper function outputs
- SC-005: ✅ Handle 100+ nesting levels
- SC-006: ✅ Process 1000+ elements in <100ms
- SC-007: ✅ O(n) time complexity
- SC-008: ✅ Compose with existing operations
- SC-009: ✅ Document "decorated sequence" semantics
- SC-010: ✅ Provide practical examples

## Next Steps

1. **Generate tasks**: Run `/speckit.tasks` to break down implementation into concrete tasks
2. **Implement core operations**: `extract` and `extend` in `comonad.rs`
3. **Implement helpers**: `depth_at`, `size_at`, `indices_at` in `comonad_helpers.rs`
4. **Write property tests**: Comonad laws in `tests/comonad_laws.rs`
5. **Write unit tests**: Helper functions in `tests/comonad_helpers.rs`
6. **Verify equivalence**: Compare outputs with gram-hs in `tests/equivalence/`
7. **Document**: Module-level docs with conceptual explanation and examples
8. **Review**: Ensure all constitution checks still pass
9. **Benchmark** (optional): Verify performance meets targets

## References

- **Feature Spec**: [spec.md](./spec.md)
- **Haskell Reference**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs` (lines 720-728, 1104-1138)
- **Haskell Tests**: `../pattern-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` (lines 4242-4400)
- **Haskell Law Tests**: `../pattern-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (lines 1287-1332)
- **Analysis**: [ANALYSIS.md](./ANALYSIS.md)
- **Recommendation**: [RECOMMENDATION.md](./RECOMMENDATION.md)
- **Constitution**: `../../.specify/memory/constitution.md`
- **Porting Guide**: `../../docs/porting-guide.md`
