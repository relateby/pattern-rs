# gram-rs TODO

This TODO tracks the incremental porting of features from the gram-hs reference implementation (`../gram-hs`) to gram-rs. Features are organized by development phase and follow the gram-hs feature numbering where applicable.

**Reference**: See `docs/porting-guide.md` for porting workflow and `plan/gram-rs-work-plan.md` for prioritized work plan to achieve feature parity with gram-hs.

**Gap Analysis**: See `plan/gram-rs-implementation-gaps.md` for detailed analysis of missing features (~70% library feature parity currently).

**Scope**: Library modules only (pattern-core, gram-codec). CLI tooling is out of scope for gram-rs.

---

## Current Status Summary

### ✅ Completed Phases
- **Phase 1**: Foundation & Infrastructure (3/3 features)
- **Phase 2**: Core Pattern Data Structure (3/4 features, 1 deferred)
- **Phase 3**: Pattern Typeclass Instances (9/11 features, 1 deferred, 1 complete)
- **Phase 4**: Gram Notation Serialization (1/5 features)

### 🎯 Priority Work (Next Steps)

Based on the gap analysis in `plan/gram-rs-implementation-gaps.md`, the following library features are critical for achieving feature parity:

#### P0 - Critical (Blocking)
1. **Paramorphism** (NEW) - Structure-aware folding operation (1-2 days)

#### P1 - High Priority
1. **Graph Lens** (026-graph-lens) - Graph interpretation layer (1 week)
2. **Comonad Verification** (018-comonad-instance) - Verify/complete implementation (2-3 days)

#### P2 - Medium Priority
1. **Validation Enhancement** - Duplicate/undefined checking (2-3 days)
2. **Documentation** - Feature guides for new features (2-3 days)

See `plan/gram-rs-work-plan.md` for detailed implementation plan (2.5-3.5 weeks to full library parity).

---

## Phase 1: Foundation & Infrastructure ✅ COMPLETE

**Progress**: 3/3 features complete
- ✅ 001: Rust project initialization
- ✅ 002: Multi-crate workspace setup
- ✅ 003: Testing framework infrastructure

---

## Phase 2: Core Pattern Data Structure ✅ MOSTLY COMPLETE

**Progress**: 3/4 features complete (007 deferred)
- ✅ 004: Pattern data structure
- ✅ 005: Pattern construction & access
- ✅ 006: Pattern validation & structure analysis
- ⏸️ 007: Additional pattern builders (deferred - core constructors already available)

---

## Phase 3: Pattern Typeclass Instances ✅ MOSTLY COMPLETE

**Progress**: 9/11 features complete (1 deferred, 1 complete)
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
- ✅ 018: Comonad instance (complete - extract, extend, depth_at, size_at, indices_at)
- ✅ 024: Python bindings for pattern-core (complete)

---

## Phase 4: Gram Notation Serialization 🚧 IN PROGRESS

**Progress**: 1/5 features complete

### ✅ 019-gram-serialization: Basic Gram Codec (COMPLETE)
**Implementation**: `crates/gram-codec/` - Full bidirectional codec with multi-platform support
**Authority**: `external/tree-sitter-gram/` (git submodule)
**Status**: Complete - 162 tests, multi-platform (Rust, WASM, Python)

### 020-gram-parsing-conformance: Parser Conformance
**Primary Reference**: `../gram-hs/libs/gram/src/Gram/Parse.hs`
**Status**: Not started

- [ ] Study Haskell implementation
- [ ] Port grammar conformance tests
- [ ] Verify parser handles all gram syntax
- [ ] Add error recovery and reporting
- [ ] Verify equivalence with gram-hs

### 021-gram-serializer-updates: Serializer Enhancements
**Primary Reference**: `../gram-hs/libs/gram/src/Gram/Serialize.hs`
**Status**: Not started

- [ ] Study Haskell implementation
- [ ] Port serializer improvements
- [ ] Add pretty-printing support
- [ ] Verify equivalence with gram-hs

### 022-subject-serialization: Subject Type Serialization
**Primary Reference**: `../gram-hs/libs/subject/`
**Status**: Not started

- [ ] Study Haskell implementation
- [ ] Port subject serialization
- [ ] Verify equivalence with gram-hs

### 023-gram-serialization-update: Serialization Updates
**Primary Reference**: `../gram-hs/libs/gram/`
**Status**: Not started

- [ ] Study Haskell implementation
- [ ] Port serialization updates
- [ ] Verify equivalence with gram-hs

---

## Phase 5: Advanced Pattern Operations 🎯 HIGH PRIORITY

### 🆕 PARAMORPHISM: Structure-Aware Folding (P0 - CRITICAL)
**Primary Reference**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` lines 32-34
**Documentation**: `../gram-hs/docs/reference/features/paramorphism.md`
**Porting Guide**: `../gram-hs/docs/reference/PORTING-GUIDE.md` lines 386-543
**Status**: **NOT STARTED** - Critical gap identified in feature parity analysis

**Priority**: P0 - Critical for structure-aware operations
**Effort**: 1-2 days
**Impact**: HIGH - Enables depth-weighted sums, element-count aggregations, nesting statistics

**Implementation**:
```rust
impl<V> Pattern<V> {
    pub fn para<R, F>(&self, f: F) -> R
    where
        F: Fn(&Pattern<V>, &[R]) -> R,
    {
        let child_results: Vec<R> = self.elements.iter()
            .map(|child| child.para(&f))
            .collect();
        f(self, &child_results)
    }
}
```

**Tasks**:
- [x] Study Haskell implementation
- [x] Implement `para()` function
- [x] Add comprehensive tests (unit + property-based)
- [x] Add examples (depth-weighted sums, element-count aggregations)
- [x] Document usage patterns and relationship to Foldable/Comonad
- [x] Verify equivalence with gram-hs

### 026-graph-lens: Graph Lens Operations (P1 - HIGH PRIORITY)
**Primary Reference**: `../gram-hs/libs/pattern/src/Pattern/Graph.hs`
**Documentation**: `../gram-hs/docs/reference/features/graph-lens.md`
**Status**: **NOT STARTED** - Critical gap identified in feature parity analysis

**Priority**: P1 - High priority for graph interpretation
**Effort**: 1 week
**Impact**: HIGH - Enables graph interpretation of patterns

**Tasks**:
- [ ] Study Haskell implementation
- [ ] Implement `GraphLens` type
- [ ] Implement node operations (nodes, is_node)
- [ ] Implement relationship operations (relationships, source, target, reverse_rel)
- [ ] Implement walk operations (walks, walk_nodes, is_walk)
- [ ] Implement navigation operations (neighbors, incident_rels, degree)
- [ ] Implement graph analysis (connected_components, bfs, find_path)
- [ ] Add comprehensive tests
- [ ] Create examples and documentation
- [ ] Verify equivalence with gram-hs

### 🆕 018-comonad-verification: Comonad Instance Verification (P1 - HIGH PRIORITY)
**Primary Reference**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (Comonad instance)
**Documentation**: `../gram-hs/docs/reference/features/typeclass-instances.md` lines 121-131
**Status**: **NEEDS VERIFICATION** - Implementation exists but needs verification

**Priority**: P1 - High priority for context-aware operations
**Effort**: 2-3 days
**Impact**: MEDIUM - Ensures correctness of context-aware operations

**Tasks**:
- [ ] Review existing comonad-related code
- [ ] Implement missing `duplicate()` function if needed
- [ ] Verify `extend()` implementation
- [ ] Add property tests for comonad laws
- [ ] Create examples demonstrating context-aware operations
- [ ] Document relationship to zippers and context
- [ ] Verify equivalence with gram-hs

### 025-pattern-path-semantics: Pattern Path Operations
**Primary Reference**: `../gram-hs/libs/`
**Status**: Not started

- [ ] Study Haskell implementation
- [ ] Port pattern path operations
- [ ] Port path traversal functions
- [ ] Verify equivalence with gram-hs

### 027-decouple-identity-assignment: Identity Management
**Primary Reference**: `../gram-hs/libs/`
**Status**: Not started

- [ ] Study Haskell implementation
- [ ] Port identity assignment logic
- [ ] Verify equivalence with gram-hs

### 028-integration-polish: Integration & Polish
**Primary Reference**: `../gram-hs/libs/`
**Status**: Not started

- [ ] Study Haskell implementation
- [ ] Port integration improvements
- [ ] Port polish features
- [ ] Verify equivalence with gram-hs

---

## Phase 6: Validation & Documentation (P2 - MEDIUM PRIORITY)

### 🆕 VALIDATION: Enhanced Validation (P2 - MEDIUM PRIORITY)
**Primary Reference**: `../gram-hs/libs/gram/src/Gram/Validate.hs`
**Status**: **PARTIAL** - Basic validation exists, needs enhancement

**Priority**: P2 - Medium priority for correctness
**Effort**: 2-3 days
**Impact**: MEDIUM - Important for correctness

**Tasks**:
- [ ] Study Haskell implementation
- [ ] Implement duplicate definition checking
- [ ] Implement undefined reference checking
- [ ] Implement arity consistency checking
- [ ] Add validation tests
- [ ] Document validation rules

### 🆕 DOCUMENTATION: Feature Documentation (P2 - MEDIUM PRIORITY)
**Status**: **PARTIAL** - Some documentation exists

**Priority**: P2 - Medium priority for users
**Effort**: 2-3 days
**Impact**: MEDIUM - Important for users

**Tasks**:
- [ ] Write paramorphism guide
- [ ] Write Graph Lens guide
- [ ] Write comonad operations guide
- [ ] Add usage examples for all features
- [ ] Update README with feature status
- [ ] Create tutorial documentation

---

## Phase 7: Optimized Pattern Store (FUTURE)

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

## Phase 8: WASM Integration (FUTURE)

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

## Phase 9: Production Features (FUTURE)

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

- **Scope**: Library modules only (pattern-core, gram-codec). CLI tooling is out of scope for gram-rs.

- **Feature Parity Status**: Currently at ~70% library feature parity with gram-hs. See `plan/gram-rs-implementation-gaps.md` for detailed analysis.

- **Priority Work**: Focus on P0 (Paramorphism) and P1 (Graph Lens, Comonad Verification) features to achieve library feature parity. See `plan/gram-rs-work-plan.md` for detailed implementation plan (2.5-3.5 weeks estimated).

- **Reference Implementation**: Always verify against the actual Haskell source code in `../gram-hs/libs/` before marking features complete. The Haskell implementation is the authoritative source of truth. Documentation in `../gram-hs/docs/` provides up-to-date information about the implementation. Historical notes in `../gram-hs/specs/` guided incremental development but may be outdated. See `docs/porting-guide.md` for detailed workflow.

- **Deferred Features**: Some features may be deferred if their core functionality is already covered by other features. For example, feature 007 (additional pattern builders) is deferred because the essential constructors (`point`, `pattern`) are already available from feature 005. Feature 017 (Applicative instance) is deferred due to no practical use cases. Deferred features can be revisited later based on user needs.

- **Breaking Changes**: Any intentional deviations from gram-hs behavior must be documented with rationale.

- **New Features**: Features marked with 🆕 are newly identified gaps from the feature parity analysis and should be prioritized according to their P0/P1/P2 priority levels.
