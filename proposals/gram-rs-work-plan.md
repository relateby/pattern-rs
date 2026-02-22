# pattern-rs Work Plan: Catching Up to gram-hs

**Date**: 2026-01-29  
**Goal**: Achieve library feature parity with gram-hs reference implementation  
**Current Status**: ~70% library feature parity  
**Target**: 100% library feature parity with gram-hs

## Overview

This work plan outlines the prioritized tasks to bring pattern-rs library modules to full feature parity with gram-hs. The plan is organized into 3 phases based on priority and dependencies, with estimated effort and success criteria for each phase.

**Scope**: Library modules only (pattern-core, gram-codec). CLI tooling is out of scope.

**Related Documents**:
- [Implementation Gap Analysis](pattern-rs-implementation-gaps.md) - Detailed gap analysis
- [gram-hs Porting Guide](../gram-hs/docs/reference/PORTING-GUIDE.md) - Reference implementation guide

---

## Phase 1: Core Features (P0) - 1-2 days

**Goal**: Implement critical missing library feature

### Task 1.1: Paramorphism Implementation
**Priority**: P0 - Critical  
**Effort**: 1-2 days  
**Dependencies**: None

**Deliverables**:
1. Implement `para()` function in `pattern-core`
2. Add comprehensive tests (unit + property-based)
3. Add examples demonstrating:
   - Depth-weighted sums
   - Element-count-aware aggregations
   - Nesting-level statistics
4. Document usage patterns and relationship to Foldable/Comonad

**Implementation Steps**:
```rust
// In crates/pattern-core/src/pattern.rs
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

**Success Criteria**:
- ✅ `para()` function implemented and tested
- ✅ Property tests verify structure access
- ✅ Examples demonstrate common use cases
- ✅ Documentation explains relationship to other operations

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` lines 32-34
- gram-hs docs: `docs/reference/features/paramorphism.md`
- gram-hs porting guide: `docs/reference/PORTING-GUIDE.md` lines 386-543

---

## Phase 2: Graph Features (P1) - 1.5-2 weeks

**Goal**: Implement graph interpretation and verify context-aware operations

### Task 2.1: Graph Lens Implementation
**Priority**: P1 - High  
**Effort**: 1 week  
**Dependencies**: None

**Deliverables**:
1. Implement `GraphLens` type
2. Implement node operations (`nodes`, `is_node`)
3. Implement relationship operations (`relationships`, `source`, `target`, `reverse_rel`)
4. Implement walk operations (`walks`, `walk_nodes`, `is_walk`)
5. Implement navigation operations (`neighbors`, `incident_rels`, `degree`)
6. Implement graph analysis operations (`connected_components`, `bfs`, `find_path`)
7. Add comprehensive tests
8. Create examples and documentation

**Implementation Steps**:
1. Create new module: `crates/pattern-core/src/graph.rs`
2. Define `GraphLens` struct with scope pattern and node predicate
3. Implement basic operations (nodes, relationships)
4. Implement navigation operations
5. Implement graph analysis algorithms
6. Add property-based tests
7. Create examples demonstrating:
   - Basic graph interpretation
   - Meta-graphs (relationships as nodes)
   - Graph traversal
8. Document design principles and usage patterns

**Success Criteria**:
- ✅ `GraphLens` type implemented
- ✅ All operations functional and tested
- ✅ Examples demonstrate key use cases
- ✅ Documentation explains design principles
- ✅ Property tests verify correctness

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Graph.hs`
- gram-hs docs: `docs/reference/features/graph-lens.md`

---

### Task 2.2: Comonad Instance Verification
**Priority**: P1 - High  
**Effort**: 2-3 days  
**Dependencies**: None

**Deliverables**:
1. Verify/complete `extract()` implementation
2. Implement explicit `duplicate()` function if missing
3. Verify/complete `extend()` implementation
4. Verify context-aware operations (`depth_at`, `size_at`, `indices_at`)
5. Add comonad law verification tests
6. Document comonad semantics and usage

**Implementation Steps**:
1. Review existing comonad-related code
2. Implement missing `duplicate()` function if needed
3. Verify `extend()` implementation
4. Add property tests for comonad laws:
   - Extract-Extend: `extract(extend(f, p)) == f(p)`
   - Extend-Extract: `extend(extract, p) == p`
   - Extend Composition: `extend(f, extend(g, p)) == extend(f ∘ extend(g), p)`
5. Create examples demonstrating context-aware operations
6. Document relationship to zippers and context

**Success Criteria**:
- ✅ All comonad operations implemented
- ✅ Comonad laws verified with property tests
- ✅ Examples demonstrate context-aware computation
- ✅ Documentation explains comonad semantics

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` (Comonad instance)
- gram-hs docs: `docs/reference/features/typeclass-instances.md` lines 121-131

---

## Phase 3: Polish & Validation (P2) - 1 week

**Goal**: Complete validation and documentation

### Task 3.1: Validation Enhancement
**Priority**: P2 - Medium  
**Effort**: 2-3 days  
**Dependencies**: None

**Deliverables**:
1. Implement duplicate definition checking
2. Implement undefined reference checking
3. Implement arity consistency checking
4. Add validation tests
5. Document validation rules

**Implementation Steps**:
1. Implement duplicate definition checker:
   - Track defined identities
   - Report duplicates with locations
2. Implement undefined reference checker:
   - Track references
   - Report undefined references
3. Implement arity consistency checker:
   - Verify relationship arity
   - Report inconsistencies
4. Add comprehensive tests
5. Document validation rules and error messages

**Success Criteria**:
- ✅ All validation checks implemented
- ✅ Tests verify correctness
- ✅ Error messages are clear and helpful
- ✅ Documentation explains validation rules

**Reference**:
- gram-hs: `libs/gram/src/Gram/Validate.hs`

---

### Task 3.2: Documentation & Examples
**Priority**: P2 - Medium  
**Effort**: 2-3 days  
**Dependencies**: All previous tasks

**Deliverables**:
1. Create feature guides for new features
2. Add usage examples for all features
3. Update README with feature status
4. Create tutorial documentation

**Implementation Steps**:
1. Write feature guides:
   - Paramorphism guide
   - Graph Lens guide
   - Comonad operations guide
2. Add examples to documentation
3. Update README with current feature status
4. Create tutorial showing common workflows

**Success Criteria**:
- ✅ Feature guides complete
- ✅ Examples demonstrate key use cases
- ✅ README accurately reflects status
- ✅ Tutorial covers common workflows

---

## Timeline Summary

| Phase | Duration | Key Deliverables |
|-------|----------|------------------|
| Phase 1 (P0) | 1-2 days | Paramorphism |
| Phase 2 (P1) | 1.5-2 weeks | Graph Lens, Comonad Verification |
| Phase 3 (P2) | 1 week | Validation, Documentation |
| **Total** | **2.5-3.5 weeks** | **Full library feature parity** |

---

## Success Metrics

### Phase 1 Success
- ✅ Paramorphism functional with examples
- ✅ Property tests verify structure access
- ✅ Documentation complete

### Phase 2 Success
- ✅ Graph Lens operational with all operations
- ✅ Comonad laws verified
- ✅ Examples demonstrate graph interpretation
- ✅ Context-aware operations working

### Phase 3 Success
- ✅ Validation comprehensive
- ✅ Documentation complete
- ✅ 100% library feature parity with gram-hs

---

## Risk Management

### High Risk Items
1. **Graph Lens Complexity** - Complex algorithms
   - Mitigation: Port carefully from gram-hs, add comprehensive tests

### Medium Risk Items
1. **Comonad Laws** - May be tricky to verify
   - Mitigation: Use property-based testing, study gram-hs implementation

### Low Risk Items
1. **Paramorphism** - Well-defined, straightforward implementation
2. **Documentation** - Time-consuming but low risk

---

## Dependencies & Blockers

### External Dependencies
- None - all features can be implemented independently

### Internal Dependencies
- None - all tasks can proceed in parallel if needed

### Potential Blockers
- Graph Lens algorithms may need optimization for performance
- Comonad verification may reveal implementation issues

---

## Resource Requirements

### Development Resources
- 1 developer, full-time
- Access to gram-hs reference implementation
- Testing infrastructure (CI/CD)

### Tools & Infrastructure
- Rust toolchain (stable)
- Testing frameworks (proptest, criterion)

### Documentation Resources
- Rustdoc for API documentation
- Markdown for guides and tutorials
- Examples directory for code samples

---

## Monitoring & Reporting

### Weekly Progress Reports
- Tasks completed
- Tasks in progress
- Blockers encountered
- Next week's plan

### Milestone Reviews
- End of each phase
- Review deliverables
- Assess quality
- Adjust plan if needed

### Success Criteria Reviews
- Verify success criteria met
- Run property tests
- Compare with gram-hs outputs
- Document any deviations

---

## Post-Implementation

### Maintenance Plan
- Monitor for bugs and issues
- Keep aligned with gram-hs updates
- Respond to user feedback
- Maintain documentation

### Future Enhancements
- Performance optimizations
- Additional platform support
- Extended validation rules
- Enhanced error messages

---

## Conclusion

This work plan provides a clear path to achieving full library feature parity with gram-hs. By following the phased approach and prioritizing core features first, pattern-rs will become a complete and reliable port of the reference implementation.

The plan is designed to be flexible and can be adjusted based on:
- Actual implementation complexity
- Discovered issues or blockers
- Changing priorities
- Resource availability

Regular monitoring and milestone reviews will ensure the project stays on track and delivers high-quality results.
