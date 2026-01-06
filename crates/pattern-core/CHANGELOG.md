# Changelog

All notable changes to the `pattern-core` crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Comonad Operations (2026-01-05)

- **Core Operations**:
  - `Pattern::extract()` - Extract decorative value at current position (O(1))
  - `Pattern::extend()` - Compute new decorations based on subpattern context (O(n))

- **Helper Functions**:
  - `Pattern::depth_at()` - Decorate each position with its depth (maximum nesting level)
  - `Pattern::size_at()` - Decorate each position with subtree size (total node count)
  - `Pattern::indices_at()` - Decorate each position with path from root (index sequence)

- **Documentation**:
  - Module-level documentation explaining "decorated sequence" semantics
  - Comprehensive doc examples for all operations
  - Example program demonstrating practical use cases (`examples/comonad_usage.rs`)

- **Testing**:
  - Property-based tests verifying all three Comonad laws (left identity, right identity, associativity)
  - Unit tests for extract and extend operations
  - Unit tests for all helper functions
  - Edge case tests for atomic patterns, deeply nested structures, and composition

**Conceptual Foundation**: Comonad is the natural abstraction for Pattern's "decorated sequence" 
semantics, where elements ARE the pattern and the value DECORATES those elements with information.

## [0.1.0] - (Date TBD)

### Added

- Initial release of pattern-core crate
- Pattern<V> type - recursive, nested structure (s-expression-like)
- Subject type - self-descriptive value with identity, labels, and properties
- Functor operations (map, map_result, fold)
- Traversable operations (traverse_option, traverse_result)
- Semigroup operations (combine)
- Foldable operations (fold)
- Query operations (filter, find_first, any_value, all_values)
- Predicate operations (matches, contains)
- Validation with configurable rules
- Structural analysis (depth, size, length)
- Equality, ordering, and hashing traits
- Comprehensive property-based testing suite

[Unreleased]: https://github.com/gram-data/gram-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/gram-data/gram-rs/releases/tag/v0.1.0

