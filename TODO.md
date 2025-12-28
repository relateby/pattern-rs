# gram-rs TODO

This TODO tracks the incremental porting of features from the gram-hs reference implementation (`../gram-hs`) to gram-rs. Features are organized by development phase and follow the gram-hs feature numbering where applicable.

**Reference**: See `PORTING_GUIDE.md` for porting workflow and `docs/gram-rs-project-plan.md` for overall architecture.

## Phase 1: Foundation & Infrastructure

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

### 004-pattern-data-structure: Core Pattern Type
**Reference**: `../gram-hs/specs/001-pattern-data-structure/`

- [ ] Review gram-hs spec: `../gram-hs/specs/001-pattern-data-structure/spec.md`
- [ ] Review type signatures: `../gram-hs/specs/001-pattern-data-structure/contracts/type-signatures.md`
- [ ] Study Haskell implementation: `../gram-hs/libs/`
- [ ] Create feature spec in `specs/004-pattern-data-structure/`
- [ ] Port `Pattern<V>` type definition to Rust
- [ ] Port `Subject` types (Node, Edge, etc.)
- [ ] Implement `Debug` and `Display` traits
- [ ] Port test cases from gram-hs
- [ ] Verify behavioral equivalence
- [ ] Test WASM compilation

### 005-basic-pattern-type: Pattern Construction & Access
**Reference**: `../gram-hs/specs/002-basic-pattern-type/`

- [ ] Review gram-hs spec: `../gram-hs/specs/002-basic-pattern-type/spec.md`
- [ ] Review type signatures: `../gram-hs/specs/002-basic-pattern-type/contracts/type-signatures.md`
- [ ] Port pattern construction functions
- [ ] Port pattern accessors (value, elements)
- [ ] Port pattern inspection utilities
- [ ] Port test cases
- [ ] Verify equivalence

### 006-pattern-structure-review: Pattern Structure Validation
**Reference**: `../gram-hs/specs/003-pattern-structure-review/`

- [ ] Review gram-hs spec
- [ ] Port pattern validation functions
- [ ] Port structure analysis utilities
- [ ] Port test cases
- [ ] Verify equivalence

### 007-construction-functions: Pattern Builders
**Reference**: `../gram-hs/specs/004-construction-functions/`

- [ ] Review gram-hs spec
- [ ] Port pattern builder functions
- [ ] Port convenience constructors
- [ ] Port test cases
- [ ] Verify equivalence

---

## Phase 3: Pattern Typeclass Instances (Traits)

### 008-functor-instance: Functor Trait
**Reference**: `../gram-hs/specs/005-functor-instance/`

- [ ] Review gram-hs spec
- [ ] Design Rust trait equivalent to Functor
- [ ] Implement `map` function for patterns
- [ ] Port test cases
- [ ] Verify equivalence

### 009-foldable-instance: Foldable Trait
**Reference**: `../gram-hs/specs/006-foldable-instance/`

- [ ] Review gram-hs spec
- [ ] Design Rust trait equivalent to Foldable
- [ ] Implement `fold` functions for patterns
- [ ] Port test cases
- [ ] Verify equivalence

### 010-traversable-instance: Traversable Trait
**Reference**: `../gram-hs/specs/007-traversable-instance/`

- [ ] Review gram-hs spec
- [ ] Design Rust trait equivalent to Traversable
- [ ] Implement `traverse` functions for patterns
- [ ] Port test cases
- [ ] Verify equivalence

### 011-basic-query-functions: Pattern Query Operations
**Reference**: `../gram-hs/specs/008-basic-query-functions/`

- [ ] Review gram-hs spec
- [ ] Port pattern query functions
- [ ] Port pattern search utilities
- [ ] Port test cases
- [ ] Verify equivalence

### 012-ord-instance: Ord Trait
**Reference**: `../gram-hs/specs/009-ord-instance/`

- [ ] Review gram-hs spec
- [ ] Implement `PartialOrd` and `Ord` for patterns
- [ ] Port test cases
- [ ] Verify equivalence

### 013-semigroup-instance: Semigroup Trait
**Reference**: `../gram-hs/specs/010-semigroup-instance/`

- [ ] Review gram-hs spec
- [ ] Design Rust trait equivalent to Semigroup
- [ ] Implement pattern combination operations
- [ ] Port test cases
- [ ] Verify equivalence

### 014-monoid-instance: Monoid Trait
**Reference**: `../gram-hs/specs/011-monoid-instance/`

- [ ] Review gram-hs spec
- [ ] Design Rust trait equivalent to Monoid
- [ ] Implement pattern identity element
- [ ] Port test cases
- [ ] Verify equivalence

### 015-hashable-instance: Hash Trait
**Reference**: `../gram-hs/specs/012-hashable-instance/`

- [ ] Review gram-hs spec
- [ ] Implement `Hash` trait for patterns
- [ ] Port test cases
- [ ] Verify equivalence

### 016-predicate-matching: Pattern Matching
**Reference**: `../gram-hs/specs/012-predicate-matching/`

- [ ] Review gram-hs spec
- [ ] Port pattern matching algorithm
- [ ] Port predicate matching functions
- [ ] Port test cases
- [ ] Verify equivalence

### 017-applicative-instance: Applicative Trait
**Reference**: `../gram-hs/specs/013-applicative-instance/`

- [ ] Review gram-hs spec
- [ ] Design Rust trait equivalent to Applicative
- [ ] Implement applicative operations for patterns
- [ ] Port test cases
- [ ] Verify equivalence

### 018-comonad-instance: Comonad Trait
**Reference**: `../gram-hs/specs/014-comonad-instance/`

- [ ] Review gram-hs spec
- [ ] Design Rust trait equivalent to Comonad
- [ ] Implement comonad operations for patterns
- [ ] Port test cases
- [ ] Verify equivalence

---

## Phase 4: Gram Notation Serialization

### 019-gram-serialization: Basic Gram Codec
**Reference**: `../gram-hs/specs/014-gram-serialization/`

- [ ] Review gram-hs spec
- [ ] Review gram grammar definition
- [ ] Choose parser library (recommended: winnow)
- [ ] Implement gram → pattern decoder
- [ ] Implement pattern → gram encoder
- [ ] Port test cases
- [ ] Round-trip testing
- [ ] Verify equivalence with gram-hs

### 020-gram-parsing-conformance: Parser Conformance
**Reference**: `../gram-hs/specs/016-gram-parsing-conformance/`

- [ ] Review gram-hs spec
- [ ] Port grammar conformance tests
- [ ] Verify parser handles all gram syntax
- [ ] Add error recovery and reporting
- [ ] Port test cases
- [ ] Verify equivalence

### 021-gram-serializer-updates: Serializer Enhancements
**Reference**: `../gram-hs/specs/017-gram-serializer-updates/`

- [ ] Review gram-hs spec
- [ ] Port serializer improvements
- [ ] Add pretty-printing support
- [ ] Port test cases
- [ ] Verify equivalence

### 022-subject-serialization: Subject Type Serialization
**Reference**: `../gram-hs/specs/020-subject-serialization/`

- [ ] Review gram-hs spec
- [ ] Port subject serialization
- [ ] Port test cases
- [ ] Verify equivalence

### 023-gram-serialization-update: Serialization Updates
**Reference**: `../gram-hs/specs/021-gram-serialization-update/`

- [ ] Review gram-hs spec
- [ ] Port serialization updates
- [ ] Port test cases
- [ ] Verify equivalence

### 024-codefence-strings: Code Fence String Support
**Reference**: `../gram-hs/specs/024-codefence-strings/`

- [ ] Review gram-hs spec
- [ ] Port code fence string parsing
- [ ] Port test cases
- [ ] Verify equivalence

---

## Phase 5: Advanced Pattern Operations

### 025-pattern-path-semantics: Pattern Path Operations
**Reference**: `../gram-hs/specs/018-pattern-path-semantics/`

- [ ] Review gram-hs spec
- [ ] Port pattern path operations
- [ ] Port path traversal functions
- [ ] Port test cases
- [ ] Verify equivalence

### 026-graph-lens: Graph Lens Operations
**Reference**: `../gram-hs/specs/023-graph-lens/`

- [ ] Review gram-hs spec
- [ ] Review graph lens analysis: `../gram-hs/specs/022-graph-lens-review/`
- [ ] Port graph lens operations
- [ ] Port lens composition
- [ ] Port test cases
- [ ] Verify equivalence

### 027-decouple-identity-assignment: Identity Management
**Reference**: `../gram-hs/specs/025-decouple-identity-assignment/`

- [ ] Review gram-hs spec
- [ ] Port identity assignment logic
- [ ] Port test cases
- [ ] Verify equivalence

### 028-integration-polish: Integration & Polish
**Reference**: `../gram-hs/specs/015-integration-polish/` and `../gram-hs/specs/019-integration-polish/`

- [ ] Review gram-hs specs
- [ ] Port integration improvements
- [ ] Port polish features
- [ ] Port test cases
- [ ] Verify equivalence

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

- **Reference Implementation**: Always verify against `../gram-hs` before marking features complete. See `PORTING_GUIDE.md` for detailed workflow.

- **Priority**: Focus on Phase 1-4 first to establish core functionality. Phases 5-8 can proceed in parallel once core is stable.

- **Breaking Changes**: Any intentional deviations from gram-hs behavior must be documented with rationale.

