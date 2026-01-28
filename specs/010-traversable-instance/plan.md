# Implementation Plan: Traversable Instance for Pattern

**Branch**: `010-traversable-instance` | **Date**: 2026-01-04 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/010-traversable-instance/spec.md`

## Summary

Implement idiomatic Rust traversable functionality for the `Pattern<V>` type that provides the same behavioral guarantees as Haskell's Traversable typeclass. This enables effectful transformations (with Option, Result, Future types) using concrete methods for each effect type, following Rust's pragmatic approach to effect handling.

**Key Principle**: We are porting the **concept** of a Traversable (structure-preserving effectful transformation) that was tested in Haskell, not the Haskell syntax itself. The implementation will be idiomatic Rust that maintains behavioral equivalence with the Haskell implementation.

**Design Decision**: Use concrete methods for specific effect types (`traverse_option`, `traverse_result`, `traverse_future`, `validate`) rather than a single generic trait. This prioritizes Rust idioms and usability over theoretical elegance.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: 
- Standard library only (for Option, Result support)
- Existing Pattern<V> type from feature 004
- Functor instance from feature 008 (for integration)
- Foldable instance from feature 009 (for integration)
- Potential: `futures` crate for async support (optional feature flag)

**Storage**: N/A (in-memory transformations only)  
**Testing**: 
- `cargo test` - Standard Rust test framework
- `proptest` (workspace) - Property-based testing for traversable laws
- Test utilities in `crates/pattern-core/src/test_utils/` for equivalence checking
- Side-effect counting for short-circuit verification

**Target Platform**: 
- Native Rust targets (x86_64, ARM, etc.)
- WebAssembly (`wasm32-unknown-unknown`)

**Project Type**: Library crate (part of multi-crate workspace)  
**Performance Goals**: 
- Traverse operations should be O(n) where n is total number of nodes
- <50ms for patterns with 1000 nodes (accounting for effect overhead)
- Support 100+ nesting levels without stack overflow
- Minimal memory overhead (constant stack per recursion level)
- Short-circuit on first error (for Result/Option) without processing remaining values

**Constraints**: 
- MUST maintain behavioral equivalence with gram-hs Traversable instance
- MUST compile for `wasm32-unknown-unknown` target
- MUST use idiomatic Rust patterns (concrete methods for each effect type)
- MUST satisfy traversable laws (identity, composition, naturality)
- MUST process values in depth-first order: root value first, then elements left to right
- MUST provide short-circuit semantics for Result and Option
- MUST provide error collection for validation use case
- SHOULD follow Rust naming conventions (`traverse_result` not `traverse`)
- SHOULD use sequential execution for async operations (preserves order)

**Scale/Scope**: 
- Implementation in `pattern-core` crate
- Multiple methods: `traverse_option`, `traverse_result`, `traverse_future`, `validate`
- Sequence operations: `sequence_option`, `sequence_result`
- Property-based tests for traversable laws
- Integration with existing pattern operations (map, fold)
- Handles patterns with 10,000+ nodes efficiently

**Verified from gram-hs Implementation**:
- ✅ Traversable instance exists in `Pattern/Core.hs`
- ✅ Implementation uses Haskell's Traversable typeclass with Applicative constraint
- ✅ Traversable laws tested in property tests
- ✅ Structure preservation guaranteed by typeclass laws
- ✅ Supports any applicative functor (IO, Maybe, Either, etc.)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅
- **Status**: PASS
- **Verification**: Feature spec references the actual Haskell Traversable instance in `../gram-hs/libs/pattern/src/Pattern/Core.hs` as the authoritative source of behavioral requirements
- **Plan**: Implement Rust functionality that maintains the same behavior (depth-first effectful transformation with proper effect sequencing) while using idiomatic Rust syntax
- **Reference Path**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` (Traversable instance) and test file `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` (Traversable law tests)
- **Porting Approach**: Per docs/porting-guide.md, we port **concepts and behavior**, not syntax. The Haskell implementation defines what the code should do; the Rust implementation achieves this idiomatically using concrete methods for each effect type.

### II. Correctness & Compatibility (NON-NEGOTIABLE) ✅
- **Status**: PASS
- **Verification**: Spec requires traversable laws (identity, composition, naturality) to pass property tests
- **Plan**: Port traversable law tests from gram-hs and verify behavior matches
- **Behavioral Equivalence**: Same structure preservation, same effect sequencing, same traversal order, same short-circuit semantics

### III. Rust Native Idioms ✅
- **Status**: PASS  
- **Verification**: Plan uses concrete methods for each effect type (`traverse_option`, `traverse_result`, `traverse_future`) rather than generic trait with Applicative constraints
- **Plan**: Follow Rust conventions for effect handling (Option, Result, Future are first-class types, not generic applicatives)
- **Rationale**: Rust lacks Higher-Kinded Types (HKTs) making generic Traversable traits with Applicative constraints extremely complex. Concrete methods are more idiomatic, discoverable, and provide better error messages. This matches Rust's pragmatic approach to abstracting over effects.

### IV. Multi-Target Library Design ✅
- **Status**: PASS
- **Verification**: Spec requires WASM compilation
- **Plan**: Pure transformation logic with optional async support behind feature flag
- **Note**: Async support (`traverse_future`) will be feature-gated to avoid pulling in async runtime dependencies for WASM targets that don't need it

### V. External Language Bindings & Examples ✅
- **Status**: DEFERRED
- **Verification**: WASM bindings are out of scope for this feature
- **Plan**: Methods must compile for WASM but bindings deferred to later features

**Note**: When porting features from gram-hs, **always use the Haskell implementation in `../gram-hs/libs/` as the behavioral specification**. Per docs/porting-guide.md, we port concepts and behavior (what the code does), not syntax (how it looks in Haskell). The Rust implementation should be idiomatic while maintaining behavioral equivalence. See [docs/porting-guide.md](../../../docs/porting-guide.md) section on "Idiomatic Rust vs Literal Translation" for detailed guidance.

## Project Structure

### Documentation (this feature)

```text
specs/010-traversable-instance/
├── plan.md                  # This file
├── spec.md                  # Feature specification
├── research.md              # Phase 0 output (to be generated)
├── data-model.md            # Phase 1 output (to be generated)
├── quickstart.md            # Phase 1 output (to be generated)
├── contracts/               # Phase 1 output (to be generated)
│   └── type-signatures.md   # Method signatures
├── checklists/
│   └── requirements.md      # Specification quality checklist (complete)
└── tasks.md                 # Phase 2 output (/speckit.tasks command)
```

### Implementation

```text
crates/pattern-core/
├── src/
│   ├── pattern.rs       # Add traverse methods to impl<V> Pattern<V> block
│   └── lib.rs           # Export traverse functionality
└── tests/
    ├── traversable_laws.rs       # New: Property-based tests for traversable laws
    ├── traversable_option.rs     # New: Tests for traverse_option and sequence_option
    ├── traversable_result.rs     # New: Tests for traverse_result and sequence_result
    ├── traversable_validate.rs   # New: Tests for validate method
    └── traversable_async.rs      # New: Tests for traverse_future (if async feature enabled)
```

**Structure Decision**: Single-project library structure. All traversable functionality implemented in `pattern-core` crate with comprehensive test coverage. Async support will be feature-gated to avoid unnecessary dependencies.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

N/A - No constitutional violations. The decision to use concrete methods instead of a generic trait is an idiomatic Rust design choice, not a violation. It aligns with Principle III (Rust Native Idioms) and avoids complex HKT simulations that would harm usability.

## Phase Breakdown

### Phase 0: Research & Validation

**Research Questions to Resolve**:

1. **Q: How to represent Traversable without HKTs?**
   - **Investigation needed**: Research Rust patterns for abstracting over effects
   - **Options**: Generic trait with Applicative constraint (complex), concrete methods per effect type (pragmatic), trait objects (runtime overhead)
   - **Decision criteria**: Usability, type inference, error messages, maintainability

2. **Q: What effect types to support initially?**
   - **Investigation needed**: Review gram-hs Traversable usage, analyze common Rust effect patterns
   - **Options**: Option and Result only (minimal), add Future support (async), add custom effect types via trait
   - **Decision criteria**: Spec requirements, common use cases, implementation complexity

3. **Q: How to implement short-circuit semantics?**
   - **Investigation needed**: Study Result's `?` operator behavior, early return patterns
   - **Options**: Use `?` operator (idiomatic), manual matching (explicit), Try trait (experimental)
   - **Decision criteria**: Clarity, performance, stability

4. **Q: How to collect all errors for validation?**
   - **Investigation needed**: Research error aggregation patterns in Rust
   - **Options**: Vec<E> (simple), custom ValidatedNec type (typed), Result<T, Vec<E>> (pragmatic)
   - **Decision criteria**: Ergonomics, type safety, integration with Result

5. **Q: How to handle async traverse execution?**
   - **Investigation needed**: Study async patterns for sequential vs concurrent execution
   - **Options**: Sequential with .await (simple, predictable), concurrent with join_all (fast), configurable (flexible)
   - **Decision criteria**: Spec decision (sequential), complexity, predictability

6. **Q: How to implement traversable laws in Rust?**
   - **Investigation needed**: Study Haskell law definitions, translate to Rust property tests
   - **Options**: Direct translation (may not type-check), adapt for Rust effect system, test observable properties only
   - **Decision criteria**: Testability, type system compatibility, behavioral guarantees

**Outputs**:
- research.md with all decisions documented
- Rationale for concrete methods approach
- Error aggregation strategy
- Async execution strategy
- Law testing strategy

**Acceptance Criteria**:
- All research questions resolved with documented decisions
- Implementation approach validated
- Test strategy defined
- All NEEDS CLARIFICATION items resolved

### Phase 1: Design & Contracts

**Prerequisites:** `research.md` complete

**Tasks**:
1. Extract effect transformation patterns from spec → `data-model.md`
2. Generate API contracts for each traverse method
3. Design error aggregation type for `validate` method
4. Plan traversable law testing approach
5. Create quickstart guide with effect-handling examples
6. Update agent context

**Outputs**:
- [data-model.md](./data-model.md) - Effect type relationships and transformation flows
- [contracts/type-signatures.md](./contracts/type-signatures.md) - Method signatures for all traverse operations
- [quickstart.md](./quickstart.md) - Usage examples for each effect type
- Updated agent-specific context file

**Acceptance Criteria**:
- Method signatures documented and approved for all effect types
- Error aggregation type defined
- Traversal strategy validated
- Test structure planned
- All Phase 1 artifacts generated
- Agent context updated successfully

### Phase 2: Core Implementation - traverse_option

**Tasks**:
1. Implement `Pattern::traverse_option` method
2. Implement `Pattern::sequence_option` method
3. Add comprehensive documentation with examples
4. Ensure WASM compatibility
5. Add unit tests for basic cases
6. Add property tests for correctness

**Acceptance Criteria**:
- Methods compile without errors
- All-or-nothing semantics work (Some → Some, any None → None)
- Structure preservation verified
- Documentation includes clear examples
- WASM target compiles successfully
- Basic tests pass

### Phase 3: Core Implementation - traverse_result

**Tasks**:
1. Implement `Pattern::traverse_result` method (short-circuit)
2. Implement `Pattern::sequence_result` method
3. Implement short-circuit using `?` operator
4. Add tests for error propagation
5. Verify first-error-only behavior

**Acceptance Criteria**:
- Methods compile without errors
- Short-circuit semantics work (first error terminates)
- Error type preserved and propagated
- Tests verify early termination (side-effect counting)
- Integration with `?` operator works

### Phase 4: Validation Method - validate

**Tasks**:
1. Design error aggregation type (e.g., Vec<(Path, Error)>)
2. Implement `Pattern::validate` method (collect all errors)
3. Add tests for multi-error scenarios
4. Document difference from `traverse_result`

**Acceptance Criteria**:
- Method collects all errors, not just first
- Error locations tracked (if feasible)
- Clear documentation on when to use validate vs traverse_result
- Tests verify all errors collected

### Phase 5: Async Support - traverse_future (Optional)

**Tasks**:
1. Add `async` feature flag
2. Implement `Pattern::traverse_future` with sequential execution
3. Use async/await for sequential processing
4. Add async tests
5. Document async behavior and ordering guarantees

**Acceptance Criteria**:
- Feature-gated behind `async` flag
- Sequential execution preserves order
- Async tests pass
- No async dependencies when feature disabled
- Works with tokio, async-std, etc.

### Phase 6: Sequence Operations

**Tasks**:
1. Verify `sequence_option` implemented (Phase 2)
2. Verify `sequence_result` implemented (Phase 3)
3. Add tests showing sequence as special case of traverse
4. Document relationship between traverse and sequence

**Acceptance Criteria**:
- Sequence operations work for nested effects
- Relationship to traverse documented
- Tests show equivalence where applicable

### Phase 7: Traversable Laws Testing

**Tasks**:
1. Port identity law tests from gram-hs
2. Port composition law tests from gram-hs
3. Port naturality law tests from gram-hs
4. Implement property-based tests using `proptest`
5. Adapt laws for concrete method approach
6. Verify tests pass with 100+ random cases

**Acceptance Criteria**:
- Identity law: 100+ property test cases pass
- Composition law: 100+ property test cases pass (where applicable to Rust's effect system)
- Naturality law: 100+ property test cases pass (where applicable)
- Tests cover edge cases (atomic patterns, deep nesting)
- All ported gram-hs tests pass (adapted for Rust)

### Phase 8: Integration Testing

**Tasks**:
1. Test composition with map (Functor)
2. Test composition with fold (Foldable)
3. Verify traverse + map + fold pipelines
4. Test complex effect scenarios
5. Performance testing with large patterns (1000+ nodes)
6. Stack safety testing (100+ nesting levels)

**Acceptance Criteria**:
- Composes cleanly with map and fold
- Complex pipelines work correctly
- Performance targets met (<50ms for 1000 nodes)
- Stack overflow doesn't occur (100+ levels)
- Memory overhead acceptable (10K elements under 100MB)

### Phase 9: Integration & Documentation

**Tasks**:
1. Update crate documentation
2. Add examples to README
3. Verify WASM compilation
4. Update TODO.md feature checklist
5. Create migration notes if needed
6. Document differences from Haskell approach

**Acceptance Criteria**:
- Feature marked complete in TODO.md
- Documentation examples compile and run
- WASM target compiles successfully
- All success criteria from spec.md verified
- Differences from Haskell documented with rationale

## Implementation Approach: Idiomatic Rust

### Core Implementation Strategy

**Implement concrete methods for each effect type** following Rust effect-handling patterns:

```rust
impl<V> Pattern<V> {
    /// Traverses the pattern with a function returning Option, short-circuiting on None.
    ///
    /// Processes values in depth-first order (root first, then elements).
    /// If any transformation returns None, the entire operation returns None.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("1", vec![Pattern::point("2")]);
    /// 
    /// // Successful parse
    /// let result = pattern.traverse_option(|s| s.parse::<i32>().ok());
    /// assert!(result.is_some());
    /// 
    /// // Failed parse
    /// let pattern = Pattern::pattern("1", vec![Pattern::point("invalid")]);
    /// let result = pattern.traverse_option(|s| s.parse::<i32>().ok());
    /// assert!(result.is_none());
    /// ```
    pub fn traverse_option<W, F>(&self, f: F) -> Option<Pattern<W>>
    where
        F: Fn(&V) -> Option<W>,
    {
        self.traverse_option_with(&f)
    }
    
    /// Internal helper for recursive traverse with Option.
    fn traverse_option_with<W, F>(&self, f: &F) -> Option<Pattern<W>>
    where
        F: Fn(&V) -> Option<W>,
    {
        // Transform root value
        let new_value = f(&self.value)?;
        
        // Transform elements recursively
        let new_elements: Option<Vec<Pattern<W>>> = self
            .elements
            .iter()
            .map(|elem| elem.traverse_option_with(f))
            .collect();
        
        Some(Pattern {
            value: new_value,
            elements: new_elements?,
        })
    }
    
    /// Traverses the pattern with a function returning Result, short-circuiting on error.
    ///
    /// Processes values in depth-first order (root first, then elements).
    /// If any transformation returns Err, the entire operation returns Err
    /// with that error (first error encountered).
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("1", vec![Pattern::point("2")]);
    /// 
    /// // Successful parse
    /// let result: Result<Pattern<i32>, _> = 
    ///     pattern.traverse_result(|s| s.parse::<i32>());
    /// assert!(result.is_ok());
    /// 
    /// // Failed parse (first error)
    /// let pattern = Pattern::pattern("1", vec![Pattern::point("invalid")]);
    /// let result: Result<Pattern<i32>, _> = 
    ///     pattern.traverse_result(|s| s.parse::<i32>());
    /// assert!(result.is_err());
    /// ```
    pub fn traverse_result<W, E, F>(&self, f: F) -> Result<Pattern<W>, E>
    where
        F: Fn(&V) -> Result<W, E>,
    {
        self.traverse_result_with(&f)
    }
    
    /// Internal helper for recursive traverse with Result.
    fn traverse_result_with<W, E, F>(&self, f: &F) -> Result<Pattern<W>, E>
    where
        F: Fn(&V) -> Result<W, E>,
    {
        // Transform root value (short-circuits on error via ?)
        let new_value = f(&self.value)?;
        
        // Transform elements recursively (short-circuits on first error)
        let new_elements: Result<Vec<Pattern<W>>, E> = self
            .elements
            .iter()
            .map(|elem| elem.traverse_result_with(f))
            .collect();
        
        Ok(Pattern {
            value: new_value,
            elements: new_elements?,
        })
    }
    
    /// Validates all values in the pattern, collecting all errors instead of short-circuiting.
    ///
    /// Unlike traverse_result which returns on first error, validate processes all values
    /// and collects all errors encountered.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern = Pattern::pattern("1", vec![
    ///     Pattern::point("invalid1"),
    ///     Pattern::point("2"),
    ///     Pattern::point("invalid2"),
    /// ]);
    /// 
    /// let result: Result<Pattern<i32>, Vec<_>> = 
    ///     pattern.validate(|s| s.parse::<i32>().map_err(|e| e.to_string()));
    /// 
    /// // Returns all errors, not just the first
    /// assert!(result.is_err());
    /// let errors = result.unwrap_err();
    /// assert_eq!(errors.len(), 2); // Both invalid values reported
    /// ```
    pub fn validate<W, E, F>(&self, f: F) -> Result<Pattern<W>, Vec<E>>
    where
        F: Fn(&V) -> Result<W, E>,
    {
        self.validate_with(&f)
    }
    
    /// Internal helper for recursive validate.
    fn validate_with<W, E, F>(&self, f: &F) -> Result<Pattern<W>, Vec<E>>
    where
        F: Fn(&V) -> Result<W, E>,
    {
        let mut errors = Vec::new();
        
        // Transform root value, collecting error if any
        let new_value_result = f(&self.value);
        
        // Transform elements, collecting all errors
        let element_results: Vec<Result<Pattern<W>, Vec<E>>> = self
            .elements
            .iter()
            .map(|elem| elem.validate_with(f))
            .collect();
        
        // Separate successful elements from errors
        let mut new_elements = Vec::new();
        for result in element_results {
            match result {
                Ok(elem) => new_elements.push(elem),
                Err(mut errs) => errors.append(&mut errs),
            }
        }
        
        // Handle root value result
        match new_value_result {
            Ok(new_value) => {
                if errors.is_empty() {
                    Ok(Pattern {
                        value: new_value,
                        elements: new_elements,
                    })
                } else {
                    Err(errors)
                }
            }
            Err(e) => {
                errors.insert(0, e); // Root error at front
                Err(errors)
            }
        }
    }
    
    /// Sequences a pattern of Options into an Option of pattern.
    ///
    /// Flips the layers: Pattern<Option<T>> → Option<Pattern<T>>.
    /// Returns Some only if all values are Some.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern: Pattern<Option<i32>> = Pattern::pattern(
    ///     Some(1),
    ///     vec![Pattern::point(Some(2))],
    /// );
    /// 
    /// let result: Option<Pattern<i32>> = pattern.sequence_option();
    /// assert!(result.is_some());
    /// ```
    pub fn sequence_option<W>(&self) -> Option<Pattern<W>>
    where
        V: AsRef<Option<W>>,
        W: Clone,
    {
        self.traverse_option(|opt| opt.as_ref().as_ref().cloned())
    }
    
    /// Sequences a pattern of Results into a Result of pattern.
    ///
    /// Flips the layers: Pattern<Result<T, E>> → Result<Pattern<T>, E>.
    /// Returns Ok only if all values are Ok.
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let pattern: Pattern<Result<i32, String>> = Pattern::pattern(
    ///     Ok(1),
    ///     vec![Pattern::point(Ok(2))],
    /// );
    /// 
    /// let result: Result<Pattern<i32>, String> = pattern.sequence_result();
    /// assert!(result.is_ok());
    /// ```
    pub fn sequence_result<W, E>(&self) -> Result<Pattern<W>, E>
    where
        V: AsRef<Result<W, E>>,
        W: Clone,
        E: Clone,
    {
        self.traverse_result(|res| res.as_ref().as_ref().map(Clone::clone).map_err(Clone::clone))
    }
}
```

### Why This Is Idiomatic Rust

- ✅ **Concrete methods**: Clear, discoverable APIs for each effect type
- ✅ **Standard patterns**: Leverages `?` operator, Iterator::collect for effect sequencing
- ✅ **Type inference**: Works seamlessly with Rust's type system
- ✅ **No complex traits**: Avoids HKT simulation, trait hierarchies
- ✅ **Clear semantics**: Each method documents its behavior explicitly
- ✅ **Practical**: Covers common use cases without abstract complexity
- ✅ **Composable**: Works with map, fold, and other pattern operations

### Behavioral Equivalence with Haskell

The Rust implementation maintains the same behavior as Haskell's `traverse`:

**Haskell**:
```haskell
traverse :: Applicative f => (a -> f b) -> Pattern a -> f (Pattern b)
traverse f (Pattern v es) = Pattern <$> f v <*> traverse (traverse f) es
```

**Rust equivalent (for Option)**:
```rust
let new_value = f(&self.value)?;
let new_elements: Option<Vec<_>> = self.elements.iter()
    .map(|e| e.traverse_option_with(f))
    .collect();
Some(Pattern { value: new_value, elements: new_elements? })
```

**Traversable Laws** (must be satisfied):
1. **Identity**: `traverse(Id::new) == Id::new(pattern)`
2. **Composition**: `traverse(Compose::new . fmap(g) . f) == Compose::new . fmap(traverse(g)) . traverse(f)`
3. **Naturality**: `t . traverse(f) == traverse(t . f)` (for natural transformation t)

These laws will be tested via property-based tests, adapted for Rust's concrete effect types.

## Testing Strategy

### Property-Based Tests (Traversable Laws)

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn identity_law_option(pattern in arbitrary_pattern::<i32>()) {
        let original = pattern.clone();
        let traversed = pattern.traverse_option(|v| Some(v.clone()));
        prop_assert_eq!(traversed, Some(original));
    }

    #[test]
    fn structure_preservation_option(pattern in arbitrary_pattern::<i32>()) {
        let original_size = pattern.size();
        let traversed = pattern.traverse_option(|v| Some(v * 2));
        
        if let Some(new_pattern) = traversed {
            prop_assert_eq!(new_pattern.size(), original_size);
        }
    }
}
```

### Unit Tests (Effect-Specific Behavior)

```rust
#[test]
fn traverse_option_all_some() {
    let p = Pattern::pattern("1", vec![Pattern::point("2")]);
    let result = p.traverse_option(|s| s.parse::<i32>().ok());
    assert!(result.is_some());
    assert_eq!(result.unwrap().value, 1);
}

#[test]
fn traverse_option_any_none() {
    let p = Pattern::pattern("1", vec![Pattern::point("invalid")]);
    let result = p.traverse_option(|s| s.parse::<i32>().ok());
    assert!(result.is_none());
}

#[test]
fn traverse_result_short_circuit() {
    use std::sync::atomic::{AtomicUsize, Ordering};
    
    let counter = AtomicUsize::new(0);
    let p = Pattern::pattern("1", vec![
        Pattern::point("invalid"),
        Pattern::point("2"),
    ]);
    
    let result: Result<Pattern<i32>, _> = p.traverse_result(|s| {
        counter.fetch_add(1, Ordering::SeqCst);
        s.parse::<i32>()
    });
    
    assert!(result.is_err());
    // Should only process root + first element before error
    assert_eq!(counter.load(Ordering::SeqCst), 2);
}

#[test]
fn validate_collects_all_errors() {
    let p = Pattern::pattern("1", vec![
        Pattern::point("invalid1"),
        Pattern::point("2"),
        Pattern::point("invalid2"),
    ]);
    
    let result: Result<Pattern<i32>, Vec<_>> = p.validate(|s| {
        s.parse::<i32>().map_err(|e| e.to_string())
    });
    
    assert!(result.is_err());
    let errors = result.unwrap_err();
    assert_eq!(errors.len(), 2); // Both invalids reported
}
```

## Success Criteria

From spec.md (all must pass):

- ✅ **SC-001**: All traversable law property tests pass with at least 100 randomly generated test cases each
- ✅ **SC-002**: Traverse operations complete on patterns with 1000 nodes in under 50 milliseconds
- ✅ **SC-003**: Traverse operations complete on patterns with 100 nesting levels without stack overflow
- ✅ **SC-004**: Sequence operations correctly flip structure layers for patterns containing Option, Result
- ✅ **SC-005**: 100% of existing gram-hs traversable tests are ported and pass in Rust implementation
- ✅ **SC-006**: Traversable implementation compiles for WASM target without errors
- ✅ **SC-007**: `traverse_result()` method properly short-circuits on first error without processing remaining values
- ✅ **SC-008**: `traverse_option()` method properly terminates on first None without processing remaining values
- ✅ **SC-012**: `validate()` method collects all errors from a pattern with multiple invalid values
- ✅ **SC-009**: Pattern structures with 10,000 elements can be traversed without exceeding 100MB memory overhead
- ✅ **SC-010**: Traverse operations compose cleanly with map (Functor) and fold (Foldable) operations
- ✅ **SC-011**: Async traverse operations (with Future types) correctly initiate and collect results for all values

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Stack overflow on deep nesting | Low | High | Test with 100+ levels early; Rust has good stack management; use iterative approach if needed |
| Performance overhead from effect sequencing | Medium | Medium | Benchmark early; effects naturally have overhead; optimize if specific bottleneck found |
| Complex type inference issues | Medium | Medium | Provide clear type annotations in examples; use concrete methods instead of generic trait |
| Traversable law violations | Low | High | Extensive property-based testing with 100+ cases; adapt laws for Rust effect system |
| Error aggregation implementation complexity | Medium | Low | Start simple with Vec<E>; enhance if needed; clear documentation on trade-offs |
| Async support complexity | Medium | Medium | Feature-gate async; keep sequential execution simple; extensive async testing |

## Dependencies

- Feature 004: Pattern Data Structure (complete) - Defines `Pattern<V>` type
- Feature 005: Basic Pattern Type (complete) - Defines construction and access functions
- Feature 008: Functor Instance (complete) - Provides `map` for integration testing
- Feature 009: Foldable Instance (complete) - Provides `fold` for integration testing
- Rust standard library `Fn` trait, Option, Result types
- Optional: `futures` crate for async support (feature-gated)

## Next Steps (Phase 0)

1. Generate research.md with all design decisions documented
2. Resolve effect type representation approach
3. Define error aggregation strategy
4. Plan traversable law testing for Rust
5. Document rationale for concrete methods over generic trait

## References

- **Haskell Implementation**: `../gram-hs/libs/pattern/src/Pattern/Core.hs` - Traversable instance
- **Haskell Tests**: `../gram-hs/libs/pattern/tests/Spec/Pattern/Properties.hs` - Traversable law tests
- **Porting Guide**: `docs/porting-guide.md` - Section on "Idiomatic Rust vs Literal Translation"
- **Rust Effect Handling**: Option, Result standard library documentation
- **Feature 008**: Functor instance plan for consistency
- **Feature 009**: Foldable instance plan for consistency
