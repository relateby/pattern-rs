# Research: Traversable Instance for Pattern

**Feature**: 010-traversable-instance  
**Date**: 2026-01-04

## Research Tasks

### 1. Representing Traversable Without Higher-Kinded Types (HKTs)

**Task**: Determine how to implement Traversable functionality in Rust without HKTs

**Findings**:
- **Decision**: Use concrete methods for each effect type (`traverse_option`, `traverse_result`, `traverse_future`)
- **Rationale**: 
  - Rust lacks Higher-Kinded Types, making generic Functor/Applicative/Traversable traits extremely complex
  - Attempting to simulate HKTs requires trait hierarchies with associated types and complex bounds
  - Concrete methods provide:
    - Clear, discoverable APIs (users know exactly what methods are available)
    - Better error messages (no complex trait resolution failures)
    - Type inference that actually works
    - Idiomatic Rust (matches how std library handles effects)
  - This aligns with Rust's pragmatic approach: Option, Result, and Future are first-class types, not generic applicatives
- **Alternatives considered**: 
  - Generic Traversable trait with Applicative constraint (rejected - requires HKT simulation, poor ergonomics, confusing error messages)
  - Trait objects for effects (rejected - runtime overhead, doesn't work well with generic return types)
  - Macro-based approach (rejected - harder to understand, worse IDE support)
- **Precedent**: Rust standard library uses concrete methods (`Option::map`, `Result::map`) instead of generic Functor trait

### 2. Effect Types to Support

**Task**: Determine which effect types to support initially

**Findings**:
- **Decision**: Support Option, Result, and Future (feature-gated)
- **Rationale**:
  - **Option**: Essential for nullable/optional value transformations, all-or-nothing semantics
  - **Result**: Essential for error-prone transformations, validation pipelines
  - **Future**: Important for async operations (database lookups, API calls, I/O)
  - These three cover the vast majority of effectful computation in Rust
  - Per spec requirements: FR-008 (Result), FR-009 (Option), FR-010 (Future)
- **Alternatives considered**:
  - Only Option and Result (rejected - async support is increasingly important)
  - Add custom effect trait for extension (deferred - wait for user demand)
  - Support Iterator as effect (rejected - Pattern is not a sequence, Foldable already provides iteration)
- **Implementation notes**:
  - Future support will be feature-gated (`async` feature) to avoid pulling in async runtime dependencies
  - Each effect type gets its own method pair: `traverse_X` and `sequence_X`

### 3. Short-Circuit Semantics Implementation

**Task**: Determine how to implement short-circuit behavior for Result and Option

**Findings**:
- **Decision**: Use Rust's `?` operator for natural short-circuiting
- **Rationale**:
  - The `?` operator is idiomatic Rust for early return on error
  - Automatically handles short-circuiting: returns immediately on Err/None
  - Clean, readable code that expresses intent clearly
  - No manual matching or complex control flow needed
  - Iterator::collect() also handles short-circuiting for collections of Results/Options
- **Alternatives considered**:
  - Manual matching on each Result/Option (rejected - verbose, error-prone)
  - Try trait (rejected - still experimental, not stable)
  - Custom short-circuit mechanism (rejected - reinventing the wheel)
- **Implementation pattern**:
  ```rust
  let new_value = f(&self.value)?;  // Short-circuit here if error/none
  let new_elements: Result<Vec<_>, E> = self.elements
      .iter()
      .map(|e| e.traverse_result_with(f))
      .collect();  // collect() propagates first error
  Ok(Pattern { value: new_value, elements: new_elements? })
  ```

### 4. Error Aggregation for Validation

**Task**: Research error aggregation patterns for collecting all errors

**Findings**:
- **Decision**: Use `Result<Pattern<W>, Vec<E>>` for `validate` method
- **Rationale**:
  - Simple, straightforward type signature
  - Vec<E> accumulates all errors encountered during traversal
  - Return Ok only if no errors encountered
  - Compatible with Result type, easy to work with
  - Keeps success type (Pattern<W>) clean - no partial results
- **Alternatives considered**:
  - Custom `Validated<T, E>` type similar to Haskell's ValidatedNec (rejected - adds complexity, unfamiliar to Rust developers)
  - Return partial pattern with errors (rejected - unclear semantics, how to represent missing values?)
  - Use Result<Pattern<Result<W, E>>, ()> (rejected - nested Results are confusing)
  - Collect errors as Vec<(Path, E)> with position tracking (deferred - simple Vec<E> first, add tracking if needed)
- **Implementation approach**:
  - Process all values (don't short-circuit)
  - Accumulate errors in Vec as we go
  - Build successful pattern if no errors, return Err(errors) otherwise
  - Root value error inserted at front of error vec for clarity
- **Usage pattern**:
  ```rust
  match pattern.validate(|v| validate_value(v)) {
      Ok(valid_pattern) => // all values valid,
      Err(errors) => // show all errors to user
  }
  ```

### 5. Async Traverse Execution Strategy

**Task**: Determine sequential vs concurrent execution for async operations

**Findings**:
- **Decision**: Sequential execution (process one value at a time with .await)
- **Rationale** (per spec design decision):
  - Preserves strict ordering guarantees (FR-002: depth-first, root-first)
  - Predictable behavior matching synchronous traverse semantics
  - Simpler error handling (errors occur in predictable order)
  - Easier to reason about and debug
  - Matches synchronous traverse behavior
  - While slower than concurrent execution, it ensures correctness and consistency
- **Alternatives considered**:
  - Concurrent execution with join_all (rejected - breaks ordering guarantees, complicates error handling)
  - Configurable execution mode (rejected - adds API complexity, most users want predictability)
  - Parallel with ordered result collection (rejected - complex implementation, minimal benefit)
- **Implementation approach**:
  ```rust
  pub async fn traverse_future<W, F, Fut>(&self, f: F) -> Result<Pattern<W>, E>
  where
      F: Fn(&V) -> Fut,
      Fut: Future<Output = Result<W, E>>,
  {
      // Process root value
      let new_value = f(&self.value).await?;
      
      // Process elements sequentially
      let mut new_elements = Vec::new();
      for elem in &self.elements {
          new_elements.push(elem.traverse_future(&f).await?);
      }
      
      Ok(Pattern { value: new_value, elements: new_elements })
  }
  ```
- **Note**: This is feature-gated behind `async` feature flag

### 6. Traversable Laws in Rust

**Task**: Determine how to test traversable laws in Rust's type system

**Findings**:
- **Decision**: Adapt laws for concrete effect types, test observable properties
- **Rationale**:
  - Haskell's traversable laws are stated generically for any Applicative functor
  - Rust doesn't have generic applicatives, so we test laws for specific effect types
  - Focus on observable behavior rather than type-level proofs
  - Use property-based testing (proptest) to verify laws hold across many inputs
- **Laws to test** (adapted for Rust):

  **Identity Law**: `traverse(identity) ≡ identity`
  - For Option: `pattern.traverse_option(|v| Some(v.clone())) == Some(pattern.clone())`
  - For Result: `pattern.traverse_result(|v| Ok(v.clone())) == Ok(pattern.clone())`
  - Verifies traverse with identity effect returns wrapped pattern

  **Composition Law**: `traverse(Compose . fmap g . f) ≡ Compose . fmap (traverse g) . traverse f`
  - Harder to express in Rust without HKTs
  - Test observable property: composing effects outside traverse equals composing inside
  - May need to adapt or test weakened version for concrete effect types

  **Naturality Law**: `t . traverse f ≡ traverse (t . f)` for natural transformation t
  - Test with concrete natural transformations (e.g., Option to Result via ok_or)
  - Verifies transformation order doesn't matter

- **Alternatives considered**:
  - Skip law testing (rejected - laws ensure correctness)
  - Only test identity law (rejected - composition and naturality also important)
  - Attempt full generic law testing (rejected - doesn't type-check without HKTs)
- **Implementation approach**:
  ```rust
  proptest! {
      #[test]
      fn identity_law_option(pattern in arbitrary_pattern::<i32>()) {
          let original = pattern.clone();
          let traversed = pattern.traverse_option(|v| Some(*v));
          prop_assert_eq!(traversed, Some(original));
      }
      
      #[test]
      fn structure_preservation(pattern in arbitrary_pattern::<String>()) {
          let original_size = pattern.size();
          let traversed = pattern.traverse_option(|s| Some(s.to_uppercase()));
          prop_assert_eq!(traversed.map(|p| p.size()), Some(original_size));
      }
  }
  ```

### 7. Method Naming Conventions

**Task**: Determine naming scheme for traverse methods

**Findings**:
- **Decision**: Use `traverse_<effect>` pattern (e.g., `traverse_option`, `traverse_result`)
- **Rationale**:
  - Clear what effect type the method works with
  - Consistent with Rust naming conventions (descriptive, explicit)
  - Easily discoverable in IDE autocomplete
  - No ambiguity about which version to use
- **Alternatives considered**:
  - Generic `traverse` name (rejected - doesn't work without generic effect trait)
  - `try_traverse` for Result (rejected - inconsistent with other methods)
  - `traverse_with_option` (rejected - verbose)
- **Pattern applies to sequence methods**: `sequence_option`, `sequence_result`, etc.
- **Special case**: `validate` for error-collecting variant (clearer than `traverse_result_all_errors`)

### 8. Helper Method Pattern for Recursion

**Task**: Determine how to handle closure capture for recursive calls

**Findings**:
- **Decision**: Use helper method pattern - public API takes `F` by value, internal helper takes `&F` by reference
- **Rationale**:
  - Same pattern successfully used in Functor (map/map_with) and Foldable (fold/fold_with)
  - Public API is ergonomic (users pass closure by value, natural Rust style)
  - Internal helper efficient (shares closure reference across recursive calls, no cloning)
  - Avoids `Clone` bound on closure type
  - Prevents complex nested reference types
- **Alternatives considered**:
  - Require `F: Clone` and clone on each recursion (rejected - unnecessary constraint, performance overhead)
  - Pass `&F` in public API (rejected - unergonomic, users would need `&|x| ...` syntax)
  - Use impl trait in recursive calls (rejected - doesn't work, trait solver issues)
- **Implementation pattern**:
  ```rust
  pub fn traverse_option<W, F>(&self, f: F) -> Option<Pattern<W>>
  where F: Fn(&V) -> Option<W>
  {
      self.traverse_option_with(&f)  // Public delegates to internal
  }
  
  fn traverse_option_with<W, F>(&self, f: &F) -> Option<Pattern<W>>
  where F: Fn(&V) -> Option<W>
  {
      let new_value = f(&self.value)?;
      // Recursive calls use f: &F
      let new_elements: Option<Vec<_>> = self.elements
          .iter()
          .map(|elem| elem.traverse_option_with(f))  // Pass &F to recursion
          .collect();
      Some(Pattern { value: new_value, elements: new_elements? })
  }
  ```

### 9. Integration with Existing Pattern Operations

**Task**: Ensure traverse composes with map (Functor) and fold (Foldable)

**Findings**:
- **Decision**: Traverse methods are independent but composable with existing operations
- **Rationale**:
  - Traverse operates on `&self` (borrows pattern), doesn't consume it
  - Can chain: `pattern.map(f).traverse_result(g)` or `pattern.traverse_option(f)?.fold(acc, g)`
  - Methods follow same design principles as map and fold
  - Type signatures are consistent and predictable
- **Composition patterns**:
  - **Map then traverse**: `pattern.map(|v| preprocess(v)).traverse_result(|v| validate(v))`
  - **Traverse then map**: `pattern.traverse_option(|v| parse(v))?.map(|v| transform(v))`
  - **Traverse then fold**: `pattern.traverse_result(|v| fetch(v))?.fold(init, combine)`
- **Testing approach**: Explicitly test common composition patterns to ensure they work smoothly

### 10. WASM Compatibility

**Task**: Ensure traverse implementation works in WASM environment

**Findings**:
- **Decision**: Core traverse operations (Option, Result) are WASM-compatible; async support is feature-gated
- **Rationale**:
  - Option and Result traverse are pure computation, no platform dependencies
  - Future/async support requires async runtime, which may not be available in all WASM contexts
  - Feature-gating async keeps core functionality universally compatible
- **Implementation approach**:
  - Core methods: Always available, no feature flags
  - Async methods: Behind `async` feature flag
  - Testing: Include WASM compilation check in CI

## Summary

All research questions resolved. Key decisions:

1. **Concrete methods** per effect type (not generic trait) for Rust idioms
2. **Support Option, Result, Future** (Future feature-gated)
3. **Use `?` operator** for natural short-circuiting
4. **Use `Result<T, Vec<E>>`** for error aggregation in `validate`
5. **Sequential execution** for async operations (preserves ordering)
6. **Adapt traversable laws** for concrete effect types, test with proptest
7. **Helper method pattern** for efficient closure capture in recursion
8. **Full integration** with existing map and fold operations

Implementation approach validated. Ready for Phase 1 (Design & Contracts).

