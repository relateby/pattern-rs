# Changelog

All notable changes to the `pattern-core` crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Python Bindings (Feature: 024-python-pattern-core) - 2026-01-27

Complete Python bindings for pattern-core using PyO3, enabling Python developers to programmatically construct and operate on Pattern and Subject instances.

**User Story 1 - Pattern Construction:**
- `Pattern.point(value)` - Create atomic patterns
- `Pattern.of(value)` - Functor/applicative alias for `point()`
- `Pattern.pattern(value, elements)` - Create decorated patterns
- `Pattern.from_values(values)` - Convert list of values to list of atomic patterns
- `Subject(identity, labels, properties)` - Create subjects with identity, labels, and properties
- `PatternSubject` - Pattern specialized for Subject values
- Full Subject API: `add_label()`, `remove_label()`, `has_label()`, `get_property()`, `set_property()`, `remove_property()`

**User Story 2 - Pattern Operations:**
- Inspection methods: `length()`, `size()`, `depth()`, `is_atomic()`, `values()`
- Query methods: `any_value()`, `all_values()`, `filter()`, `find_first()`, `matches()`, `contains()`
- Transformation methods: `map()`, `fold()`, `combine()`
- Comonad operations: `extract()`, `extend()`, `depth_at()`, `size_at()`, `indices_at()`
- Validation: `validate(rules)`, `analyze_structure()`

**User Story 3 - Type Safety:**
- Complete type stubs in `pattern_core/__init__.pyi`
- Full type hints for all classes, methods, and parameters
- IDE autocomplete support (VS Code, PyCharm, etc.)
- Type checker support (mypy, pyright)

**Documentation & Examples:**
- Comprehensive API documentation in `docs/python-usage.md`
- Quickstart guide in `examples/pattern-core-python/README.md`
- Example files demonstrating all features: `basic_usage.py`, `operations.py`, `type_safety.py`, `advanced.py`, `zip_relationships.py`

**Testing:**
- 94 Python integration tests (100% pass rate)
- Performance tests verify <2x overhead vs native Rust for patterns with 1000 nodes

**Build & Packaging:**
- Feature-gated with `python` feature flag
- Built with maturin for Python wheel distribution
- Supports Python 3.8+

**API Changes:**
- **Breaking**: Removed `Pattern.from_list(value, values)` (confusing signature)
- Added `Pattern.from_values(values) -> List[Pattern]` (clear, single-purpose)
- Added `Pattern.of(value)` as alias for `Pattern.point()` (FP convention)
- See `crates/pattern-core/API-CHANGES.md` for migration guide

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

[Unreleased]: https://github.com/relateby/pattern-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/relateby/pattern-rs/releases/tag/v0.1.0
