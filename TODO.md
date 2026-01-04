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

**Progress**: 1/11 features complete
- ✅ 008: Functor instance (idiomatic `map` method)
- ⏸️ 009-018: Remaining typeclass instances (pending)

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

### 010-traversable-instance: Traversable Trait
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/007-traversable-instance/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/007-traversable-instance/spec.md` (historical notes, for context only)
- [ ] Design Rust trait equivalent to Traversable (based on actual Haskell implementation)
- [ ] Implement `traverse` functions for patterns (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 011-basic-query-functions: Pattern Query Operations
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/008-basic-query-functions/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/008-basic-query-functions/spec.md` (historical notes, for context only)
- [ ] Port pattern query functions (from actual Haskell source)
- [ ] Port pattern search utilities (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 012-ord-instance: Ord Trait
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/009-ord-instance/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/009-ord-instance/spec.md` (historical notes, for context only)
- [ ] Implement `PartialOrd` and `Ord` for patterns (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 013-semigroup-instance: Semigroup Trait
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/010-semigroup-instance/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/010-semigroup-instance/spec.md` (historical notes, for context only)
- [ ] Design Rust trait equivalent to Semigroup (based on actual Haskell implementation)
- [ ] Implement pattern combination operations (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 014-monoid-instance: Monoid Trait
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/011-monoid-instance/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/011-monoid-instance/spec.md` (historical notes, for context only)
- [ ] Design Rust trait equivalent to Monoid (based on actual Haskell implementation)
- [ ] Implement pattern identity element (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 015-hashable-instance: Hash Trait
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/012-hashable-instance/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/012-hashable-instance/spec.md` (historical notes, for context only)
- [ ] Implement `Hash` trait for patterns (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 016-predicate-matching: Pattern Matching
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/012-predicate-matching/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/012-predicate-matching/spec.md` (historical notes, for context only)
- [ ] Port pattern matching algorithm (from actual Haskell source)
- [ ] Port predicate matching functions (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 017-applicative-instance: Applicative Trait
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/013-applicative-instance/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/013-applicative-instance/spec.md` (historical notes, for context only)
- [ ] Design Rust trait equivalent to Applicative (based on actual Haskell implementation)
- [ ] Implement applicative operations for patterns (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

### 018-comonad-instance: Comonad Trait
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/014-comonad-instance/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/014-comonad-instance/spec.md` (historical notes, for context only)
- [ ] Design Rust trait equivalent to Comonad (based on actual Haskell implementation)
- [ ] Implement comonad operations for patterns (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Verify equivalence (against actual Haskell implementation)

---

## Phase 4: Gram Notation Serialization

For all gram notation work, use the `gram-lint` CLI tool to validate
snippets. 

### 019-gram-serialization: Basic Gram Codec
**Primary Reference (Authoritative)**: `../gram-hs/libs/` - Haskell implementation source code
**Documentation Reference**: `../gram-hs/docs/` - Up-to-date documentation about the implementation
**Historical Reference (Context Only)**: `../gram-hs/specs/014-gram-serialization/` - Historical notes from incremental development (may be outdated)

- [ ] Study Haskell implementation: `../gram-hs/libs/` - **This is the source of truth**
- [ ] Review gram-hs documentation: `../gram-hs/docs/` - **Up-to-date information about the implementation**
- [ ] Review gram-hs tests: `../gram-hs/libs/*/tests/` - **Shows expected behavior**
- [ ] Review gram-hs spec: `../gram-hs/specs/014-gram-serialization/spec.md` (historical notes, for context only)
- [ ] Review gram grammar definition (from actual Haskell source)
- [ ] Choose parser library (recommended: winnow)
- [ ] Implement gram → pattern decoder (from actual Haskell source)
- [ ] Implement pattern → gram encoder (from actual Haskell source)
- [ ] Port test cases (from actual test files)
- [ ] Round-trip testing
- [ ] Verify equivalence (against actual Haskell implementation)

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

### 039-documentation: Documentation & Tutorials
- [ ] Write comprehensive API documentation
- [ ] Create usage tutorials
- [ ] Add code examples
- [ ] Create migration guides
- [ ] Add architecture documentation

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

