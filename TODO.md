# gram-rs TODO

This TODO tracks the incremental porting of features from the gram-hs reference implementation (`../gram-hs`) to gram-rs. Features are organized by development phase and follow the gram-hs feature numbering where applicable.

**Reference**: See `PORTING_GUIDE.md` for porting workflow and `docs/gram-rs-project-plan.md` for overall architecture.

## Phase 1: Foundation & Infrastructure

**Progress**: 3/3 features complete ✅
- ✅ 001: Rust project initialization
- ✅ 002: Multi-crate workspace setup
- ✅ 003: Testing framework infrastructure

### ✅ 001-rust-init: Rust Project Initialization
- [x] Project structure and Cargo.toml setup
- [x] Basic workspace configuration
- [x] Development tooling (rustfmt, clippy)
- [x] WASM target support
- [x] Example structure for external bindings

### 002-workspace-setup: Multi-Crate Workspace
- [x] Convert to Cargo workspace structure
- [x] Create `crates/pattern-core/` crate
- [x] Create `crates/pattern-ops/` crate
- [x] Create `crates/gram-codec/` crate
- [x] Create `crates/pattern-store/` crate (placeholder)
- [x] Create `crates/pattern-wasm/` crate (placeholder)
- [x] Configure workspace dependencies
- [x] Setup CI/CD pipeline (GitHub Actions)
- [x] Add test synchronization infrastructure

### ✅ 003-test-infrastructure: Testing Framework
- [x] Setup property-based testing with `proptest` (infrastructure complete, pattern generators pending feature 004)
- [x] Create test utilities for equivalence checking (infrastructure complete, full implementation pending feature 004)
- [x] Setup snapshot testing with `insta` (infrastructure complete, pattern snapshots pending feature 004)
- [x] Create test data extraction from gram-hs (infrastructure complete, can use gram-hs CLI for test generation)
- [x] Add benchmark suite with `criterion` (infrastructure complete, pattern benchmarks pending feature 004)
- [x] Create test helpers for pattern comparison (infrastructure complete, full implementation pending feature 004)

**Status**: All infrastructure tasks complete (91/91 tasks in `specs/003-test-infrastructure/tasks.md`). Full implementation of pattern-specific functionality (generators, helpers, benchmarks) depends on pattern types being defined in feature 004. See `docs/gram-hs-cli-testing-guide.md` for using gram-hs CLI for testing.

---

## Phase 2: Core Pattern Data Structure

**Progress**: 3/4 features complete (007 deferred)
- ✅ 004: Pattern data structure
- ✅ 005: Pattern construction & access
- ✅ 006: Pattern validation & structure analysis
- ⏸️ 007: Additional pattern builders (deferred - core constructors already available)

### ✅ 004-pattern-data-structure: Core Pattern Type
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/001-pattern-data-structure/` - Historical notes from incremental development (may be outdated)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
  - Pattern: `../gram-hs/libs/pattern/src/Pattern.hs`
  - Subject: `../gram-hs/libs/subject/src/Subject/Core.hs`
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Review gram-hs spec: `../gram-hs/specs/001-pattern-data-structure/spec.md` (historical notes, for context only)
- [x] Create feature spec in `specs/004-pattern-data-structure/`
- [x] Port `Pattern<V>` type definition to Rust (from actual Haskell source, not design docs)
- [x] Verify Subject types in gram-hs: Check actual Haskell source code (`../gram-hs/libs/subject/src/Subject/Core.hs`), not design documents. If Subject is defined in the source, port it. If not, it's a value type that may be defined in other features.
- [x] Implement `Debug` and `Display` traits
- [x] Port test cases from gram-hs (from actual test files)
- [x] Verify behavioral equivalence (against actual Haskell implementation)
- [x] Test WASM compilation

### ✅ 005-basic-pattern-type: Pattern Construction & Access
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/002-basic-pattern-type/` - Historical notes from incremental development (may be outdated)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Review gram-hs spec: `../gram-hs/specs/002-basic-pattern-type/spec.md` (historical notes, for context only)
- [x] Review type signatures: `../gram-hs/specs/002-basic-pattern-type/contracts/type-signatures.md` (historical notes, verify against actual code)
- [x] Port pattern construction functions (from actual Haskell source)
- [x] Port pattern accessors (value, elements) (from actual Haskell source)
- [x] Port pattern inspection utilities (from actual Haskell source)
- [x] Port test cases (from actual test files)
- [x] Verify equivalence (against actual Haskell implementation)

### ✅ 006-pattern-structure-review: Pattern Structure Validation
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/003-pattern-structure-review/` - Historical notes from incremental development (may be outdated)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Review gram-hs spec: `../gram-hs/specs/003-pattern-structure-review/spec.md` (historical notes, for context only)
- [x] Port pattern validation functions (from actual Haskell source)
- [x] Port structure analysis utilities (from actual Haskell source)
- [x] Port test cases (from actual test files)
- [x] Verify equivalence (against actual Haskell implementation)

**Note**: Validation and structure analysis functionality implemented in feature 005 as part of core Pattern API (`validate()`, `analyze_structure()` methods).

### ⏸️ 007-construction-functions: Pattern Builders (DEFERRED)
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/004-construction-functions/` - Historical notes from incremental development (may be outdated)

**Status**: DEFERRED - Core constructors (`point`, `pattern`) already implemented in feature 005. Additional builder/convenience functions can be added later as needed based on usage patterns.

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/004-construction-functions/spec.md` (historical notes, for context only)
- [ ] Port pattern builder functions (from actual Haskell source)
- [ ] Port convenience constructors (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

---

## Phase 3: Pattern Typeclass Instances (Traits)

**Progress**: 9/11 features complete (1 deferred, 1 recommended)
- ✅ 008: Functor instance (idiomatic `map` method)
- ✅ 009: Foldable instance (fold operations)
- ✅ 010: Traversable instance (effectful transformations)
- ✅ 011: Query functions (any_value, all_values, filter)
- ✅ 012: Ord instance (ordering and comparison)
- ✅ 013: Semigroup instance (Combinable trait, associative combination)
- ✅ 014: Monoid instance (Default trait, identity element)
- ✅ 015: Hash instance (hashing for HashMap/HashSet)
- ✅ 016: Predicate matching (find_first, matches, contains)
- ⏸️ 017: Applicative instance (deferred - no practical use cases)
- 🎯 018: Comonad instance (recommended - conceptually correct for Pattern semantics)

### ✅ 008-functor-instance: Functor Trait
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/005-functor-instance/` - Historical notes from incremental development (may be outdated)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Review gram-hs spec: `../gram-hs/specs/005-functor-instance/spec.md` (historical notes, for context only)
- [x] Design Rust trait equivalent to Functor (based on actual Haskell implementation)
- [x] Implement `map` function for patterns (from actual Haskell source)
- [x] Port test cases (from actual test files)
- [x] Verify equivalence (against actual Haskell implementation)

### 009-foldable-instance: Foldable Trait ✅ COMPLETE
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/006-foldable-instance/` - Historical notes from incremental development (may be outdated)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Review gram-hs spec: `../gram-hs/specs/006-foldable-instance/spec.md` (historical notes, for context only)
- [x] Design Rust trait equivalent to Foldable (based on actual Haskell implementation)
- [x] Implement `fold` functions for patterns (from actual Haskell source)
- [x] Port test cases (from actual test files)
- [x] Verify equivalence (against actual Haskell implementation)

**Implementation**: `crates/pattern-core/src/pattern.rs` - fold(), values() methods
**Tests**: 75 tests in `crates/pattern-core/tests/foldable_*.rs` - all passing

### 010-traversable-instance: Traversable Trait ✅ COMPLETE
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/007-traversable-instance/` - Historical notes from incremental development (may be outdated)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Review gram-hs spec: `../gram-hs/specs/007-traversable-instance/spec.md` (historical notes, for context only)
- [x] Design Rust trait equivalent to Traversable (based on actual Haskell implementation)
- [x] Implement `traverse` functions for patterns (from actual Haskell source)
- [x] Port test cases (from actual test files)
- [x] Verify equivalence (against actual Haskell implementation)

**Implementation**: `crates/pattern-core/src/pattern.rs` - traverse_option(), traverse_result(), validate_all(), sequence_option(), sequence_result() methods
**Tests**: 53 tests in `crates/pattern-core/tests/traversable_*.rs` - all passing
**Deferred**: Async support (traverse_future) - Would require tokio/async runtime dependency. Can be added later as feature-gated functionality if needed.

### 011-basic-query-functions: Pattern Query Operations ✅
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/008-basic-query-functions/` - Historical notes from incremental development (may be outdated)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Review gram-hs spec: `../gram-hs/specs/008-basic-query-functions/spec.md` (historical notes, for context only)
- [x] Port pattern query functions (from actual Haskell source) - any_value, all_values, filter
- [x] Port pattern search utilities (from actual Haskell source) - filter operation
- [x] Port test cases (from actual test files) - 66 tests ported and passing
- [x] Verify equivalence (against actual Haskell implementation) - behavioral equivalence verified

**Implementation**: `crates/pattern-core/src/pattern.rs`
**Tests**: `crates/pattern-core/tests/query_*.rs` (66 tests)
**Status**: Complete - all operations implemented with comprehensive test coverage

### 012-ord-instance: Ord Trait ✅
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/009-ord-instance/` - Historical notes from incremental development (may be outdated)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Review gram-hs spec: `../gram-hs/specs/009-ord-instance/spec.md` (historical notes, for context only)
- [x] Implement `PartialOrd` and `Ord` for patterns (from actual Haskell source) - value-first lexicographic ordering
- [x] Port test cases (from actual test files) - 56 tests ported and passing
- [x] Verify equivalence (against actual Haskell implementation) - behavioral equivalence verified

**Implementation**: `crates/pattern-core/src/pattern.rs` - PartialOrd and Ord trait implementations
**Tests**: `crates/pattern-core/tests/ord_*.rs` (56 tests covering comparison, sorting, extrema, collections, property laws)
**Benchmarks**: `crates/pattern-core/benches/ord_benchmarks.rs` (9 benchmark groups for performance validation)
**Status**: Complete - all operations implemented with comprehensive test coverage, property-based Ord law verification, and behavioral equivalence with gram-hs confirmed

### 013-semigroup-instance: Semigroup Trait ✅ COMPLETE
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Spec**: `specs/013-semigroup-instance/spec.md` - Implementation-agnostic specification
**Plan**: `specs/013-semigroup-instance/plan.md` - Implementation plan and design decisions
**Tasks**: `specs/013-semigroup-instance/tasks.md` - 60 detailed implementation tasks

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Design Rust trait equivalent to Semigroup (Combinable trait)
- [x] Implement pattern combination operations (`Pattern::combine`)
- [x] Port test cases (from actual test files)
- [x] Verify equivalence (against actual Haskell implementation)

**Implementation**: 
- `crates/pattern-core/src/lib.rs` - Combinable trait for String, Vec<T>, ()
- `crates/pattern-core/src/pattern.rs` - Pattern::combine method
**Tests**: 
- `crates/pattern-core/tests/semigroup_basic.rs` (12 unit tests)
- `crates/pattern-core/tests/semigroup_property.rs` (11 property tests with proptest for associativity)
- `crates/pattern-core/tests/semigroup_integration.rs` (15 integration tests with iterators)
- `crates/pattern-core/tests/semigroup_equivalence.rs` (14 equivalence tests with gram-hs)
**Benchmarks**: `crates/pattern-core/benches/semigroup_benchmarks.rs` (7 benchmark groups: atomic ~100ns, 1000 elements ~119µs, 100-pattern fold ~17µs)
**Status**: Complete - associative combination operation implemented with comprehensive test coverage, property-based verification, and behavioral equivalence with gram-hs confirmed

### 014-monoid-instance: Monoid Trait ✅ COMPLETE
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Spec**: `specs/014-monoid-instance/spec.md` - Implementation-agnostic specification
**Plan**: `specs/014-monoid-instance/plan.md` - Implementation plan and design decisions
**Tasks**: `specs/014-monoid-instance/tasks.md` - 56 detailed implementation tasks (all complete)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Design Rust trait equivalent to Monoid (idiomatic Default trait approach)
- [x] Implement pattern identity element using std::default::Default
- [x] Port test cases and verify monoid laws
- [x] Verify equivalence (against actual Haskell implementation)

**Implementation**: 
- `crates/pattern-core/src/pattern.rs` - Default trait implementation for Pattern<V> where V: Default
**Tests**: 
- `crates/pattern-core/tests/monoid_default.rs` (25 unit tests)
- `crates/pattern-core/tests/monoid_identity.rs` (18 property tests - 4,608+ patterns verified)
- `crates/pattern-core/tests/monoid_integration.rs` (24 integration tests with iterators)
**Status**: Complete - identity element implemented with comprehensive test coverage, property-based verification of monoid laws (left/right identity), and behavioral equivalence with gram-hs confirmed. Uses idiomatic Rust Default trait instead of custom Monoid trait.

### 015-hashable-instance: Hash Trait ✅
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Spec**: `specs/015-hashable-instance/spec.md` - Implementation-agnostic specification
**Plan**: `specs/015-hashable-instance/plan.md` - Implementation plan and design decisions
**Tasks**: `specs/015-hashable-instance/tasks.md` - 42 detailed implementation tasks (all complete)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Review gram-hs spec: `../gram-hs/specs/012-hashable-instance/spec.md` (historical notes, for context only)
- [x] Implement `Hash` trait for patterns (from actual Haskell source)
- [x] Add Hash to Symbol type
- [x] Port test cases (from actual test files)
- [x] Verify equivalence (against actual Haskell implementation)

**Implementation**: 
- `crates/pattern-core/src/pattern.rs` - Hash trait implementation for Pattern<V> where V: Hash
- `crates/pattern-core/src/subject.rs` - Hash derive added to Symbol
**Tests**: 
- `crates/pattern-core/tests/hash_basic.rs` (14 unit tests)
- `crates/pattern-core/tests/hash_consistency.rs` (8 property tests with proptest)
- `crates/pattern-core/tests/hash_integration.rs` (13 integration tests)
**Status**: Complete - hashing support implemented with comprehensive test coverage, property-based verification of hash/eq consistency, and behavioral equivalence with gram-hs confirmed. Enables O(1) HashMap/HashSet usage for patterns.

### 016-predicate-matching: Pattern Matching ✅ COMPLETE
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Spec**: `specs/016-predicate-matching/spec.md` - Implementation-agnostic specification
**Plan**: `specs/016-predicate-matching/plan.md` - Implementation plan and design decisions
**Tasks**: `specs/016-predicate-matching/tasks.md` - 88 detailed implementation tasks (all complete)

- [x] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [x] Review gram-hs spec: `../gram-hs/specs/012-predicate-matching/spec.md` (historical notes, for context only)
- [x] Port pattern matching algorithm (from actual Haskell source)
- [x] Port predicate matching functions (from actual Haskell source)
- [x] Port test cases (from actual test files)
- [x] Verify equivalence (against actual Haskell implementation)

**Implementation**: 
- `crates/pattern-core/src/pattern.rs` - find_first(), matches(), contains() methods
**Tests**: 
- `crates/pattern-core/tests/query_find_first.rs` (26 unit tests)
- `crates/pattern-core/tests/predicate_matches.rs` (31 unit tests)
- `crates/pattern-core/tests/predicate_contains.rs` (29 unit tests)
- `crates/pattern-core/tests/predicate_properties.rs` (19 property tests with proptest)
**Benchmarks**: `crates/pattern-core/benches/predicate_benchmarks.rs` (13 benchmark groups for find_first, matches, contains)
**Status**: Complete - predicate-based pattern matching implemented with comprehensive test coverage, property-based verification of mathematical properties (reflexivity, symmetry, transitivity), and behavioral equivalence with gram-hs confirmed. All operations meet performance targets (<10ms for typical patterns).

### ⏸️ 017-applicative-instance: Applicative Trait (DEFERRED - Not Recommended)
**Primary Reference (Authoritative)**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` - Lines 670-676
**Tests**: `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` - Lines 1075-1189 (law verification only)
**Analysis**: `specs/017-applicative-instance/ANALYSIS.md` - Detailed evaluation and recommendation
**Recommendation**: `specs/017-applicative-instance/RECOMMENDATION.md` - Port recommendation summary

**Status**: DEFERRED - Analysis completed 2026-01-05. Not recommended for port at this time.

**Rationale**: 
- ❌ Zero practical usage in gram-hs (only law verification tests, no production use)
- ❌ All use cases better served by existing features:
  - `Pattern::point` for `pure` (already exists)
  - `Pattern::map` for function application (Functor, already implemented)
  - `Pattern::traverse_result` for validation workflows (Traversable, already implemented)
  - `Pattern::fold` for aggregations (Foldable, already implemented)
  - `Pattern::combine` for merging patterns (Semigroup, already implemented)
- ❌ Complex Cartesian product semantics (creates exponential element growth)
  - Spec claims "zip-like" behavior, but implementation does Cartesian product
  - Pattern `f [f1, f2]` applied to `x [x1, x2]` creates 4 elements, not 2
- ❌ Awkward in Rust (requires storing functions in patterns, extensive cloning)
- ❌ High testing burden (5 applicative laws + edge cases) for no clear benefit

**Alternative Approaches for Users**:
Users can achieve applicative-like operations using existing methods:
- `Pattern::point(value)` provides `pure` functionality
- `pattern.map(f)` provides single function application
- `pattern.traverse_result(f)` provides validation workflows with short-circuiting
- `pattern.fold(init, combine)` provides aggregation
- Custom `zip_with` or similar methods can be added later if concrete use cases emerge

**Reconsider if**: Concrete use cases emerge that cannot be solved with existing methods.

- [x] Study Haskell implementation: `../gram-hs/libs/` - **Analysis complete**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Analysis complete**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Only law tests, no practical usage**
- [x] Review gram-hs spec: `../gram-hs/specs/013-applicative-instance/spec.md` - **Spec semantics don't match implementation**
- [ ] ~~Design Rust trait equivalent to Applicative~~ - **DEFERRED - Not recommended**
- [ ] ~~Implement applicative operations for patterns~~ - **DEFERRED - No compelling use case**
- [ ] ~~Port test cases~~ - **DEFERRED**
- [ ] ~~Verify equivalence~~ - **DEFERRED**

### 🎯 018-comonad-instance: Comonad Operations (RECOMMENDED - Conceptually Correct)
**Primary Reference (Authoritative)**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` - Lines 720-728, 1104-1138
**Tests**: `../gram-hs/libs/pattern/tests/Spec/Pattern/CoreSpec.hs` - Lines 4242-4400 (helper tests)
**Property Tests**: `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` - Lines 1287-1332 (law tests)
**Analysis**: `specs/018-comonad-instance/ANALYSIS.md` - Detailed evaluation and recommendation
**Recommendation**: `specs/018-comonad-instance/RECOMMENDATION.md` - Port recommendation summary (updated)

**Status**: ✅ COMPLETE - Implementation finished 2026-01-05. Comonad operations are available for Pattern.

**Conceptual Rationale**:
- ✅ **Pattern semantics**: Elements ARE the pattern, value DECORATES the elements
  - Example: `Pattern { value: "sonata", elements: ["A", "B", "A"] }`
  - The elements `["A", "B", "A"]` form the actual pattern
  - The value `"sonata"` provides information ABOUT that pattern
- ✅ **Comonad is the only typeclass that treats both as information**:
  - `extract`: Access the decorative information (the value)
  - `extend`: Compute new decorative information based on context (the subpattern)
- ✅ **Natural fit for "decorated sequences"**: Value + Elements are peers, not hierarchy
- ✅ **Enables position-aware decorations**: depthAt, sizeAt, indicesAt compute new decorations

**Implementation Approach**:
1. **Implement `extract` and `extend`** (skip `duplicate` for now)
2. **Use `extend` for helpers** to maintain conceptual consistency:
   - `depth_at()` = `extend(|p| p.depth())`
   - `size_at()` = `extend(|p| p.size())`
   - `indices_at()` = custom implementation (needs path tracking)
3. **Keep it simple**: Pass function by reference (no Clone bound needed)
4. **Document semantics**: Explain Pattern's "decorated sequence" model

**Practical Considerations**:
- ⚠️ Limited production usage in gram-hs (only test helpers)
- ✅ But conceptually correct for Pattern's semantics
- ✅ Enables natural expression of position-aware operations
- ✅ Not complex to implement (extract trivial, extend straightforward)

**Implementation Plan**:
```rust
impl<V> Pattern<V> {
    /// Extracts the decorative value (the information about the pattern).
    pub fn extract(&self) -> &V {
        &self.value
    }
    
    /// Computes new decorative information at each position.
    /// Takes a function that computes information about a subpattern.
    pub fn extend<W, F>(&self, f: &F) -> Pattern<W>
    where
        F: Fn(&Pattern<V>) -> W,
    {
        Pattern {
            value: f(self),
            elements: self.elements.iter().map(|e| e.extend(f)).collect(),
        }
    }
    
    /// Decorates each position with its depth.
    pub fn depth_at(&self) -> Pattern<usize> {
        self.extend(&|p| p.depth())
    }
    
    /// Decorates each position with its subtree size.
    pub fn size_at(&self) -> Pattern<usize> {
        self.extend(&|p| p.size())
    }
}
```

- [x] Study Haskell implementation: `../gram-hs/libs/` - **Analysis complete**
- [x] Review gram-hs documentation: `../gram-hs/docs/` - **Analysis complete**
- [x] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Helper tests + law tests**
- [x] Review gram-hs spec: `../gram-hs/specs/014-comonad-instance/spec.md` - **Reviewed**
- [x] Implement `extract` and `extend` methods - **Complete**
- [x] Implement `depth_at`, `size_at`, `indices_at` helpers using `extend` - **Complete**
- [x] Port test cases (comonad laws + helper tests) - **Complete**
- [x] Verify equivalence (against actual Haskell implementation) - **Complete**
- [x] Document Pattern's "decorated sequence" semantics - **Complete**
- [x] Add examples showing position-aware decorations - **Complete**

---

## Phase 4: Gram Notation Serialization

For all gram notation work, use the `gram-lint` CLI tool to validate
snippets. 

### 019-gram-serialization: Basic Gram Codec ✅ **COMPLETE**
**Implementation**: `crates/gram-codec/` - Full bidirectional codec with multi-platform support
**Authority**: `external/tree-sitter-gram/` (git submodule) - **Deviation from gram-hs justified by requirement**
**Documentation**: `specs/019-gram-codec/` - Comprehensive specification and implementation docs
**Examples**: `examples/gram-codec/`, `examples/gram-codec-python/`, `examples/gram-codec-wasm-*/` - Multi-platform examples

- [x] Study grammar authority: `tree-sitter-gram` (explicit requirement, validated with `gram-lint`)
- [x] Review tree-sitter-gram tests: `external/tree-sitter-gram/test/corpus/`
- [x] Created specification: `specs/019-gram-codec/spec.md`
- [x] Chose parser library: tree-sitter-gram v0.2 with tree-sitter v0.25 (WASM + Python support)
- [x] Implemented gram → pattern parser (with error recovery and edge identifier extraction)
- [x] Implemented pattern → gram serializer (with format selection, handles relationship edge identifiers)
- [x] Comprehensive test suite: 162 tests (161 passing + 1 ignored)
- [x] Round-trip correctness validation
- [x] Created Rust examples (basic_usage.rs, advanced_usage.rs) in `examples/gram-codec/`
- [x] Created Python examples (demo.py, quickstart.py) in `examples/gram-codec-python/`
- [x] Created WASM examples (web UI, Node.js CLI) in `examples/gram-codec-wasm-*/`
- [x] Created comprehensive README and API documentation
- [x] WASM bindings (`--features wasm`) for browser/Node.js usage
- [x] Python bindings (`--features python`) with PyO3 and maturin
- [x] Performance benchmarks (8 comprehensive benchmark suites)
- [x] Fixed relationship serialization with edge identifiers (Bug #1)
- [x] Updated spec documentation to match implementation (tree-sitter 0.25)

**Note**: Uses tree-sitter-gram as authoritative grammar (not gram-hs) per feature requirement.

**Multi-Platform**: Native Rust, WebAssembly (browsers/Node.js), and Python (via PyO3)

**Stats**: 162 tests, ~3,500 LOC, O(n) performance, production-ready

### 020-gram-parsing-conformance: Parser Conformance
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/016-gram-parsing-conformance/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/016-gram-parsing-conformance/spec.md` (historical notes, for context only)
- [ ] Port grammar conformance tests (from actual test files)
- [ ] Verify parser handles all gram syntax (from actual Haskell implementation)
- [ ] Add error recovery and reporting (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 021-gram-serializer-updates: Serializer Enhancements
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/017-gram-serializer-updates/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/017-gram-serializer-updates/spec.md` (historical notes, for context only)
- [ ] Port serializer improvements (from actual Haskell source)
- [ ] Add pretty-printing support (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 022-subject-serialization: Subject Type Serialization
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/020-subject-serialization/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/020-subject-serialization/spec.md` (historical notes, for context only)
- [ ] Port subject serialization (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 023-gram-serialization-update: Serialization Updates
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/021-gram-serialization-update/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/021-gram-serialization-update/spec.md` (historical notes, for context only)
- [ ] Port serialization updates (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 024-codefence-strings: Code Fence String Support
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/024-codefence-strings/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/024-codefence-strings/spec.md` (historical notes, for context only)
- [ ] Port code fence string parsing (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

---

## Phase 5: Advanced Pattern Operations

### 025-pattern-path-semantics: Pattern Path Operations
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/018-pattern-path-semantics/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/018-pattern-path-semantics/spec.md` (historical notes, for context only)
- [ ] Port pattern path operations (from actual Haskell source)
- [ ] Port path traversal functions (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 026-graph-lens: Graph Lens Operations
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/023-graph-lens/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/023-graph-lens/spec.md` (historical notes, for context only)
- [ ] Review graph lens analysis: `../gram-hs/specs/022-graph-lens-review/` (for context only)
- [ ] Port graph lens operations (from actual Haskell source)
- [ ] Port lens composition (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 027-decouple-identity-assignment: Identity Management
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/025-decouple-identity-assignment/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/025-decouple-identity-assignment/spec.md` (historical notes, for context only)
- [ ] Port identity assignment logic (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 028-integration-polish: Integration & Polish
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/015-integration-polish/` and `../gram-hs/specs/019-integration-polish/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs specs: `../gram-hs/specs/015-integration-polish/` and `../gram-hs/specs/019-integration-polish/` (for context only)
- [ ] Port integration improvements (from actual Haskell source)
- [ ] Port polish features (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

---

## Phase 6: Optimized Pattern Store

### 029-pattern-store-design: Storage Architecture
- [ ] Design columnar storage for patterns
- [ ] Design string interning system
- [ ] Design spatial indices for UI queries
- [ ] Create storage API contracts
- [ ] Write design documentation

### 030-pattern-store-implementation: Storage Implementation
- [ ] Implement columnar storage backend
- [ ] Implement string interning
- [ ] Implement pattern indices
- [ ] Implement incremental update system
- [ ] Add performance benchmarks
- [ ] Verify 10x performance improvement target

### 031-pattern-query-optimizer: Query Optimization
- [ ] Design query optimizer
- [ ] Implement query planning
- [ ] Implement query execution engine
- [ ] Add query benchmarks
- [ ] Verify optimization effectiveness

---

## Phase 7: WASM Integration

### 032-wasm-bindings: WASM Bindings
- [ ] Create wasm-bindgen interfaces
- [ ] Expose core pattern API to JavaScript
- [ ] Expose gram codec API to JavaScript
- [ ] Add TypeScript definitions
- [ ] Create JavaScript test suite
- [ ] Verify WASM size <100KB (compressed)

### 033-wasm-optimization: WASM Size Optimization
- [ ] Optimize for size (wee_alloc, compression)
- [ ] Remove unused code paths
- [ ] Optimize dependencies
- [ ] Verify size targets
- [ ] Benchmark parse time (<10ms for typical graphs)

### 034-wasm-examples: WASM Examples
- [ ] Create example web application
- [ ] Create Node.js usage example
- [ ] Create browser usage example
- [ ] Add example documentation
- [ ] Test examples in target environments

---

## Phase 8: Production Features

### 035-logging-telemetry: Observability
- [ ] Add comprehensive logging
- [ ] Add telemetry hooks
- [ ] Add performance metrics
- [ ] Add error tracking
- [ ] Document observability features

### 036-error-recovery: Error Handling
- [ ] Implement error recovery strategies
- [ ] Add graceful degradation
- [ ] Improve error messages
- [ ] Add error documentation
- [ ] Test error scenarios

### 037-migration-tools: Format Migration
- [ ] Create migration tools from other formats
- [ ] Add format conversion utilities
- [ ] Add migration documentation
- [ ] Test migration paths

### 038-debugging-tools: Developer Tools
- [ ] Create debugging utilities
- [ ] Add profiling tools
- [ ] Add pattern visualization
- [ ] Add debugging documentation
- [ ] Test debugging tools

### ✅ 039-documentation: Documentation & Tutorials
- [x] Write comprehensive API documentation (Introduction, Gram Notation, Rust Usage)
- [x] Create usage tutorials (Introduction, Rust Usage)
- [x] Add code examples (Rust Usage)
- [ ] Create migration guides
- [x] Add architecture documentation (Introduction)

---

## Ongoing Tasks

### Test Synchronization
- [ ] Setup automated test extraction from gram-hs
- [ ] Create test comparison utilities
- [ ] Add CI checks for test parity
- [ ] Document test synchronization process

### Performance Monitoring
- [ ] Setup performance regression testing
- [ ] Add benchmark tracking
- [ ] Monitor WASM size over time
- [ ] Track performance metrics

### Code Quality
- [ ] Maintain 100% test parity with gram-hs
- [ ] Ensure all code compiles for WASM
- [ ] Keep documentation in sync with gram-hs
- [ ] Review and update as gram-hs evolves

---

## Notes

- **Feature Numbering**: Features 001-039 in gram-rs correspond to incremental development phases. Some gram-hs features (e.g., 001-pattern-data-structure) map to later gram-rs features (004-pattern-data-structure) because infrastructure setup comes first.

- **Reference Implementation**: Always verify against the actual Haskell source code in `../gram-hs/libs/` before marking features complete. The Haskell implementation is the authoritative source of truth. Documentation in `../gram-hs/docs/` provides up-to-date information about the implementation. Historical notes in `../gram-hs/specs/` guided incremental development but may be outdated. See `PORTING_GUIDE.md` for detailed workflow.

- **Priority**: Focus on Phase 1-4 first to establish core functionality. Phases 5-8 can proceed in parallel once core is stable.

- **Deferred Features**: Some features may be deferred if their core functionality is already covered by other features. For example, feature 007 (additional pattern builders) is deferred because the essential constructors (`point`, `pattern`) are already available from feature 005. Deferred features can be revisited later based on user needs.

- **Breaking Changes**: Any intentional deviations from gram-hs behavior must be documented with rationale.

