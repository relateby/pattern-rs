# pattern-rs Implementation Gap Analysis

**Date**: 2026-01-29  
**Purpose**: Identify library features from gram-hs that need to be ported to pattern-rs  
**Reference**: gram-hs at ../gram-hs (Haskell reference implementation)

## Executive Summary

pattern-rs is a Rust port of gram-hs with solid foundations in place. The Pattern library core is well-implemented with most essential features. However, several advanced library features are missing or incomplete.

**Overall Status**: ~70% feature parity with gram-hs (library modules only)

### Critical Library Gaps
1. **Paramorphism** - Structure-aware folding (missing)
2. **Graph Lens** - Graph interpretation layer (missing)
3. **Applicative Instance** - Zip-like application (missing, but deferred - no practical use cases)
4. **Comonad Instance** - Context-aware operations (complete, needs verification)

---

## 1. Pattern Library Gaps

### 1.1 Core Type & Construction ✅ COMPLETE
**Status**: Fully implemented

- ✅ `Pattern<V>` recursive data structure
- ✅ `point()` - atomic pattern constructor
- ✅ `pattern()` - pattern with elements constructor
- ✅ `from_list()` - construct from value list

**Reference**: 
- gram-hs: `libs/pattern/src/Pattern/Core.hs` lines 267-271
- pattern-rs: `crates/pattern-core/src/pattern.rs` lines 122-134

---

### 1.2 Query Functions ✅ COMPLETE
**Status**: Fully implemented

- ✅ `length()` - number of direct elements
- ✅ `size()` - total nodes in structure
- ✅ `depth()` - maximum nesting depth
- ✅ `values()` - extract all values as flat list
- ✅ `any_value()` - check if any value satisfies predicate
- ✅ `all_values()` - check if all values satisfy predicate
- ✅ `filter()` - filter patterns by predicate
- ✅ `find_first()` - find first matching pattern
- ✅ `matches()` - structural equality check
- ✅ `contains()` - subpattern containment check

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` lines 218-231
- pattern-rs: `crates/pattern-core/src/pattern.rs` lines 332-2189

---

### 1.3 Functor Instance ✅ COMPLETE
**Status**: Fully implemented

- ✅ `map()` - transform values while preserving structure
- ✅ Functor laws verified

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` (Functor instance)
- pattern-rs: `crates/pattern-core/src/pattern.rs` lines 1473-1494

---

### 1.4 Foldable Instance ✅ COMPLETE
**Status**: Fully implemented

- ✅ `fold()` - fold over values
- ✅ `values()` - extract all values (equivalent to `toList`)
- ✅ Right and left folding supported

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` (Foldable instance)
- pattern-rs: `crates/pattern-core/src/pattern.rs` lines 1620-1738

---

### 1.5 Traversable Instance ✅ COMPLETE
**Status**: Fully implemented

- ✅ `traverse_option()` - effectful traversal with Option
- ✅ `traverse_result()` - effectful traversal with Result
- ✅ `sequence_option()` - sequence Option effects
- ✅ `sequence_result()` - sequence Result effects
- ✅ `validate_all()` - validation with error collection

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` (Traversable instance)
- pattern-rs: `crates/pattern-core/src/pattern.rs` lines 2055-2459

---

### 1.6 Ord Instance ✅ COMPLETE
**Status**: Fully implemented

- ✅ `PartialOrd` trait - partial ordering
- ✅ `Ord` trait - total ordering
- ✅ Lexicographic ordering (value first, then elements)

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` lines 274-300
- pattern-rs: `crates/pattern-core/src/pattern.rs` lines 2512-2635

---

### 1.7 Semigroup/Monoid Instance ✅ COMPLETE
**Status**: Fully implemented via Combinable trait

- ✅ `combine()` - associative combination
- ✅ `Combinable` trait for value types
- ✅ `Default` trait for identity element
- ✅ Associativity verified

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` (Semigroup/Monoid instances)
- pattern-rs: `crates/pattern-core/src/pattern.rs` lines 2641-2860

---

### 1.8 Hashable Instance ✅ COMPLETE
**Status**: Fully implemented

- ✅ `Hash` trait implementation
- ✅ Structure-preserving hashing

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` (Hashable instance)
- pattern-rs: `crates/pattern-core/src/pattern.rs` lines 3154-3185

---

### 1.9 Applicative Instance ⏸️ DEFERRED
**Status**: Not implemented (intentionally deferred)

**Missing Features**:
- ❌ `pure()` - lift value into pattern (note: `point()` exists but not as Applicative)
- ❌ `ap()` or `<*>` - zip-like application of pattern of functions to pattern of values
- ❌ Applicative laws verification

**Impact**: Low - No practical use cases identified in gram-hs

**Rationale**: Analysis shows zero production usage in gram-hs (only law verification tests). All use cases better served by existing features (map, traverse, fold, combine).

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` (Applicative instance)
- gram-hs docs: `docs/reference/features/typeclass-instances.md` lines 109-119
- pattern-rs analysis: `specs/017-applicative-instance/ANALYSIS.md`

---

### 1.10 Comonad Instance ✅ COMPLETE
**Status**: Implemented (needs verification)

**Implemented**:
- ✅ `extract()` - get current value (via `value` field accessor)
- ✅ `extend()` - context-aware transformation
- ✅ `depth_at()` - decorate with depth at each position
- ✅ `size_at()` - decorate with size at each position
- ✅ `indices_at()` - decorate with indices at each position

**Needs Verification**:
- ⚠️ Explicit `duplicate()` function (may not be implemented)
- ⚠️ Comonad laws verification
- ⚠️ Full documentation of comonad semantics

**Impact**: Medium - Enables context-aware transformations

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` (Comonad instance)
- gram-hs docs: `docs/reference/features/typeclass-instances.md` lines 121-131
- pattern-rs: `specs/018-comonad-instance/`

---

### 1.11 Paramorphism ❌ MISSING
**Status**: Not implemented

**Missing Features**:
- ❌ `para()` - structure-aware folding function
- ❌ Access to full pattern structure during folding
- ❌ Examples and documentation

**Impact**: HIGH - Critical for structure-aware aggregations

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Core.hs` lines 32-34
- gram-hs docs: `docs/reference/features/paramorphism.md`
- gram-hs porting guide: `docs/reference/PORTING-GUIDE.md` lines 386-543

**Implementation Notes**:
```rust
impl<V> Pattern<V> {
    /// Paramorphism: structure-aware folding
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

**Use Cases**:
- Depth-weighted sums
- Element-count-aware aggregations
- Nesting-level statistics
- Structure-aware transformations

---

### 1.12 Graph Lens ❌ MISSING
**Status**: Not implemented

**Missing Features**:
- ❌ `GraphLens` type
- ❌ Node operations (`nodes`, `isNode`)
- ❌ Relationship operations (`relationships`, `source`, `target`, `reverseRel`)
- ❌ Walk operations (`walks`, `walkNodes`)
- ❌ Navigation operations (`neighbors`, `incidentRels`, `degree`)
- ❌ Graph analysis (`connectedComponents`, `bfs`, `findPath`)

**Impact**: HIGH - Critical for graph interpretation

**Reference**:
- gram-hs: `libs/pattern/src/Pattern/Graph.hs`
- gram-hs docs: `docs/reference/features/graph-lens.md`

**Implementation Notes**:
```rust
pub struct GraphLens<V> {
    scope_pattern: Pattern<V>,
    test_node: Box<dyn Fn(&Pattern<V>) -> bool>,
}

impl<V> GraphLens<V> {
    pub fn new<F>(scope_pattern: Pattern<V>, test_node: F) -> Self
    where
        F: Fn(&Pattern<V>) -> bool + 'static,
    {
        Self {
            scope_pattern,
            test_node: Box::new(test_node),
        }
    }
    
    pub fn nodes(&self) -> Vec<&Pattern<V>> {
        self.scope_pattern.elements.iter()
            .filter(|p| (self.test_node)(p))
            .collect()
    }
    
    // ... other operations
}
```

---

## 2. Subject Library Gaps

### 2.1 Core Subject Type ✅ COMPLETE
**Status**: Fully implemented

- ✅ `Subject` struct with identity, labels, properties
- ✅ `Symbol` type for identity
- ✅ `Value` enum for property values
- ✅ `PropertyRecord` (HashMap) for properties

**Reference**:
- gram-hs: `libs/subject/src/Subject/Core.hs`
- pattern-rs: `crates/pattern-core/src/subject.rs`

---

### 2.2 Value Types ✅ COMPLETE
**Status**: Fully implemented

- ✅ Standard types (String, Integer, Decimal, Boolean)
- ✅ Extended types (Symbol, Array, Map, Range, Measurement)
- ✅ Tagged strings

**Reference**:
- gram-hs: `libs/subject/src/Subject/Value.hs`
- pattern-rs: `crates/pattern-core/src/subject.rs` lines 126-157

---

### 2.3 Subject Combination ✅ COMPLETE
**Status**: Fully implemented

- ✅ `Combinable` trait for Subject
- ✅ Label merging (union)
- ✅ Property merging (right-biased)
- ✅ Identity handling

**Reference**:
- gram-hs: `libs/subject/src/Subject/Core.hs`
- pattern-rs: `crates/pattern-core/src/lib.rs` lines 347-365

---

## 3. Gram Codec Library Gaps

### 3.1 Parsing ✅ MOSTLY COMPLETE
**Status**: Implemented with nom parser

- ✅ Parse gram notation to Pattern<Subject>
- ✅ Handle nodes, relationships, paths
- ✅ Property parsing
- ✅ Label parsing
- ✅ Error recovery and reporting

**Reference**:
- gram-hs: `libs/gram/src/Gram/Parse.hs`
- pattern-rs: `crates/gram-codec/src/parser/`

---

### 3.2 Serialization ✅ MOSTLY COMPLETE
**Status**: Implemented

- ✅ Serialize Pattern<Subject> to gram notation
- ✅ Handle all value types
- ✅ Format nodes, relationships, paths

**Reference**:
- gram-hs: `libs/gram/src/Gram/Serialize.hs`
- pattern-rs: `crates/gram-codec/src/serializer.rs`

---

### 3.3 Validation ⚠️ PARTIAL
**Status**: Basic validation exists

**Implemented**:
- ✅ Basic structure validation

**Missing**:
- ❌ Duplicate definition checking
- ❌ Undefined reference checking
- ❌ Arity consistency checking

**Impact**: Medium - Important for correctness

**Reference**:
- gram-hs: `libs/gram/src/Gram/Validate.hs`
- pattern-rs: Needs implementation

---

### 3.4 JSON Schema Generation ⏸️ DEFERRED
**Status**: Not implemented (lower priority for library)

**Missing Features**:
- ❌ Generate JSON Schema (Draft 2020-12)
- ❌ Generate TypeScript type definitions
- ❌ Generate Rust type definitions

**Impact**: Low - Useful for interoperability but not core library functionality

**Reference**:
- gram-hs: `libs/gram/src/Gram/Schema/`

---

## 4. Documentation Gaps

### 4.1 API Documentation ✅ GOOD
**Status**: Well documented

- ✅ Rustdoc comments on public APIs
- ✅ Examples in documentation
- ✅ Module-level documentation

---

### 4.2 Feature Documentation ⚠️ PARTIAL
**Status**: Some documentation exists

**Needs**:
- ⚠️ Paramorphism guide
- ⚠️ Graph Lens guide
- ⚠️ Comonad operations guide (verification)

**Impact**: Medium - Important for users

---

### 4.3 Porting Guide ✅ EXISTS
**Status**: Porting guide exists in gram-hs

**Reference**:
- gram-hs: `docs/reference/PORTING-GUIDE.md`

---

## 5. Platform Support Gaps

### 5.1 WebAssembly ✅ SUPPORTED
**Status**: WASM compilation supported

- ✅ WASM target builds
- ✅ JavaScript bindings

**Reference**:
- pattern-rs: `crates/gram-codec/src/wasm.rs`

---

### 5.2 Python Bindings ✅ SUPPORTED
**Status**: Python bindings implemented

- ✅ PyO3 bindings for Pattern
- ✅ Python package structure

**Reference**:
- pattern-rs: `crates/pattern-core/src/python.rs`

---

## Priority Matrix

### P0 - Critical (Blocking)
1. **Paramorphism** - Core feature for structure-aware operations

### P1 - High Priority
1. **Graph Lens** - Core feature for graph interpretation
2. **Comonad Verification** - Complete implementation and verify laws

### P2 - Medium Priority
1. **Validation** - Duplicate/undefined checking
2. **Documentation** - Feature guides for new features

### P3 - Low Priority
1. **JSON Schema Generation** - Useful for interoperability
2. **Additional Documentation** - Expand coverage

---

## Recommended Implementation Order

### Phase 1: Core Features (P0)
1. **Paramorphism** (1-2 days)
   - Implement `para()` function
   - Add tests and examples
   - Document usage patterns

### Phase 2: Graph Features (P1)
2. **Graph Lens** (1 week)
   - Implement GraphLens type
   - Add node/relationship operations
   - Add navigation operations
   - Add graph analysis operations

3. **Comonad Verification** (2-3 days)
   - Verify/complete implementation
   - Add `duplicate()` function if missing
   - Verify comonad laws
   - Add documentation

### Phase 3: Polish (P2)
4. **Validation** (2-3 days)
   - Duplicate definition checking
   - Undefined reference checking
   - Arity consistency checking

5. **Documentation** (ongoing)
   - Feature guides
   - Usage examples
   - Migration guides

---

## Estimated Total Effort

- **Phase 1 (P0)**: 1-2 days
- **Phase 2 (P1)**: 1.5-2 weeks
- **Phase 3 (P2)**: 1 week

**Total**: 2.5-3.5 weeks for core library feature parity

---

## Success Criteria

1. ✅ All P0 features implemented and tested
2. ✅ Paramorphism working with examples
3. ✅ Graph Lens operational
4. ✅ Comonad laws verified
5. ✅ Validation comprehensive
6. ✅ Documentation complete for new features

---

## Notes

- pattern-rs has a solid foundation with ~70% library feature parity
- Core Pattern operations are well-implemented
- Main gaps are in advanced features (Paramorphism, Graph Lens)
- Python and WASM bindings are already in place, which is excellent
- Focus is on library modules only; CLI tooling is out of scope
