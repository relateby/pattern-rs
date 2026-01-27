# Research: Python Pattern-Core Bindings

**Feature**: 024-python-pattern-core  
**Date**: 2026-01-27  
**Purpose**: Research technical decisions for implementing Python bindings for pattern-core

## Decision: PyO3 Binding Strategy

**Date**: 2026-01-27  
**Question**: How should we expose Pattern<V> and Subject types to Python?  
**Decision**: Use PyO3 with `#[pyclass]` for Pattern and Subject, with generic type handling via trait objects or type-specific implementations

**Rationale**:
- PyO3 is the standard Rust-Python binding library, already used in gram-codec
- Provides excellent type conversion between Rust and Python
- Supports Python callbacks via `PyCFunction` and `PyAny`
- Feature-gating allows pattern-core to remain WASM-compatible
- Maturin integration provides seamless Python packaging

**Alternatives Considered**:
- **CFFI**: Lower-level, more manual work, less type safety
- **WASM + Python**: Higher overhead, more complex deployment
- **Pure Python reimplementation**: Loses performance benefits, duplicates code

**Trade-offs**:
- ✅ Pro: Native performance (no WASM overhead)
- ✅ Pro: Standard approach in Rust ecosystem
- ✅ Pro: Excellent PyO3 documentation and community support
- ⚠️ Con: Requires compilation for Python (maturin handles this)
- ⚠️ Con: Platform-specific wheels needed (maturin generates these)

**Implementation Impact**:
- Add `pyo3` dependency with `python` feature flag
- Create `src/python.rs` module with PyO3 bindings
- Use `#[pyclass]` for Pattern and Subject types
- Implement `#[pymethods]` for all public operations

## Decision: Generic Type Handling

**Date**: 2026-01-27  
**Question**: How to handle Pattern<V> generic type in Python (where V can be any type or Subject)?  
**Decision**: Create separate Python classes: `Pattern` (generic, accepts Any) and `PatternSubject` (specialized for Pattern<Subject>)

**Rationale**:
- Python doesn't have Rust-style generics
- Pattern<Subject> is the primary use case (per spec)
- Separate classes allow type-safe Subject operations
- Generic Pattern can still accept Python values (str, int, etc.)

**Alternatives Considered**:
- **Single Pattern class with runtime type checking**: Less type-safe, harder for type checkers
- **Only Pattern<Subject>**: Too restrictive, loses generic Pattern utility
- **Trait objects**: Complex, loses type information

**Trade-offs**:
- ✅ Pro: Type-safe Pattern<Subject> operations
- ✅ Pro: Flexible generic Pattern for simple values
- ✅ Pro: Clear API for Python developers
- ⚠️ Con: Two classes to maintain (but they share implementation)

**Implementation Impact**:
- `Pattern` class for generic Pattern<V> where V is Python-native
- `PatternSubject` class for Pattern<Subject> with Subject-specific methods
- Both share underlying Rust implementation via trait bounds

## Decision: Type Stub Generation

**Date**: 2026-01-27  
**Question**: How to generate .pyi type stub files for type checking?  
**Decision**: Manually create `pattern_core/__init__.pyi` with comprehensive type hints, validated with mypy and pyright

**Rationale**:
- PyO3 doesn't auto-generate type stubs
- Manual stubs allow precise control over type information
- Can include generic type parameters using `TypeVar` and `Generic`
- Validated against actual PyO3 bindings to ensure accuracy

**Alternatives Considered**:
- **pyo3-stubgen**: Experimental, may not support all PyO3 features
- **No type stubs**: Poor developer experience, no IDE autocomplete
- **Auto-generation scripts**: Complex, may miss edge cases

**Trade-offs**:
- ✅ Pro: Full control over type information
- ✅ Pro: Supports advanced Python typing (TypeVar, Generic, Union)
- ✅ Pro: Can be validated with mypy/pyright
- ⚠️ Con: Manual maintenance required (but API is stable)
- ⚠️ Con: Must keep in sync with Rust implementation

**Implementation Impact**:
- Create `pattern_core/__init__.pyi` with all public APIs
- Use `TypeVar` for generic Pattern types
- Include docstrings in type stubs for IDE tooltips
- Validate stubs with `mypy pattern_core` and `pyright pattern_core`

## Decision: Python Callback Handling

**Date**: 2026-01-27  
**Question**: How to handle Python callbacks (functions passed to map/filter) with type safety?  
**Decision**: Use `PyCFunction` with type hints in .pyi files, runtime validation via PyO3's `PyAny::call1`

**Rationale**:
- PyO3's `PyCFunction` provides safe Python callback invocation
- Type hints in .pyi files enable static type checking
- Runtime validation catches type mismatches early
- Supports both lambda functions and regular Python functions

**Alternatives Considered**:
- **No type checking**: Poor developer experience, runtime errors
- **Compile-time validation**: Not possible with dynamic Python
- **Custom type checking decorator**: Overly complex

**Trade-offs**:
- ✅ Pro: Type-safe callbacks with static checking
- ✅ Pro: Runtime validation for safety
- ✅ Pro: Works with standard Python functions
- ⚠️ Con: Type hints are advisory (Python is dynamic)

**Implementation Impact**:
- Use `PyCFunction` type in PyO3 bindings
- Define callback signatures in .pyi files: `Callable[[T], U]`
- Runtime validation via PyO3's type conversion
- Clear error messages for type mismatches

## Decision: Error Conversion Strategy

**Date**: 2026-01-27  
**Question**: How to convert Rust errors to Python-friendly exceptions?  
**Decision**: Map Rust error types to appropriate Python exceptions (ValueError, TypeError, RuntimeError) with clear, actionable messages

**Rationale**:
- Python developers expect standard Python exceptions
- Clear error messages improve developer experience
- Avoid Rust-specific terminology in error messages
- Follow gram-codec pattern (already established)

**Error Mapping**:
- `ValidationError` → `ValueError` (invalid input)
- `Type conversion errors` → `TypeError` (wrong Python type)
- `Stack overflow` → `RecursionError` (deep nesting)
- `Other Rust errors` → `RuntimeError` (unexpected errors)

**Alternatives Considered**:
- **Custom exception classes**: More Pythonic but adds complexity
- **Raw Rust error messages**: Confusing for Python developers
- **Generic Exception**: Too vague, poor developer experience

**Trade-offs**:
- ✅ Pro: Familiar Python exception types
- ✅ Pro: Clear, actionable error messages
- ✅ Pro: Consistent with gram-codec approach
- ⚠️ Con: Must maintain error mapping logic

**Implementation Impact**:
- Create error conversion helper functions
- Map each Rust error type to appropriate Python exception
- Include context in error messages (what operation failed, why)
- Test error messages for clarity

## Decision: Testing Strategy

**Date**: 2026-01-27  
**Question**: How to test Python bindings comprehensively?  
**Decision**: Multi-layered testing: Rust unit tests (PyO3 bindings), Python pytest tests (integration), type checking tests (mypy/pyright validation)

**Rationale**:
- Rust unit tests verify PyO3 bindings work correctly
- Python pytest tests verify Python API usability
- Type checking tests ensure type stubs are accurate
- Follows existing pattern-core testing patterns

**Test Structure**:
- `tests/python/test_pattern.py`: Pattern construction and operations
- `tests/python/test_subject.py`: Subject creation and Value types
- `tests/python/test_operations.py`: Map, filter, combine, comonad operations
- `tests/python/test_type_safety.py`: Type checking validation

**Alternatives Considered**:
- **Only Python tests**: Misses Rust-side issues
- **Only Rust tests**: Doesn't verify Python API usability
- **No type checking tests**: Type stubs may drift from implementation

**Trade-offs**:
- ✅ Pro: Comprehensive coverage (Rust + Python + types)
- ✅ Pro: Catches issues at multiple levels
- ✅ Pro: Validates developer experience
- ⚠️ Con: More test code to maintain (but worth it)

**Implementation Impact**:
- Add pytest as dev dependency
- Create Python test files in `tests/python/`
- Add type checking validation in CI
- Test all user stories from spec (construction, operations, type safety)

## Decision: Documentation Strategy

**Date**: 2026-01-27  
**Question**: How to document Python bindings for developers?  
**Decision**: Multi-format documentation: API reference (docs/python-usage.md), examples (examples/pattern-core-python/), README with quickstart, inline docstrings in Python code

**Rationale**:
- Different developers need different documentation formats
- Examples are crucial for Python developers (show, don't tell)
- API reference provides comprehensive coverage
- Inline docstrings enable IDE tooltips

**Documentation Structure**:
- `docs/python-usage.md`: Complete API reference with examples
- `examples/pattern-core-python/README.md`: Quickstart and common patterns
- `examples/pattern-core-python/*.py`: Working code examples
- Inline docstrings: All public Python methods and classes

**Alternatives Considered**:
- **Only README**: Insufficient for complex API
- **Only API reference**: Too dry, lacks practical examples
- **No inline docstrings**: Poor IDE experience

**Trade-offs**:
- ✅ Pro: Multiple entry points for different needs
- ✅ Pro: Examples demonstrate real-world usage
- ✅ Pro: IDE tooltips improve developer experience
- ⚠️ Con: More documentation to maintain (but improves adoption)

**Implementation Impact**:
- Write comprehensive API documentation
- Create 4+ example files covering all user stories
- Add docstrings to all PyO3 bindings
- Include type hints in docstrings for IDE support

## Decision: Value Type Conversion

**Date**: 2026-01-27  
**Question**: How to convert Subject Value enum (VString, VInt, VArray, etc.) to/from Python types?  
**Decision**: Create Python Value class with methods for each variant, automatic conversion from Python native types (str, int, float, bool, list, dict)

**Rationale**:
- Python developers expect native types (str, int, list, dict)
- Value class provides explicit construction when needed
- Automatic conversion improves ergonomics
- Matches Python's duck typing philosophy

**Conversion Rules**:
- `str` → `VString`
- `int` → `VInt`
- `float` → `VDecimal`
- `bool` → `VBoolean`
- `list` → `VArray` (recursive conversion)
- `dict` → `VMap` (recursive conversion)
- `Symbol` → `VSymbol` (via Symbol class)

**Alternatives Considered**:
- **Only Value class**: Too verbose, poor ergonomics
- **Only automatic conversion**: Loses explicit control
- **Separate functions**: Inconsistent API

**Trade-offs**:
- ✅ Pro: Pythonic API (native types work)
- ✅ Pro: Explicit control when needed (Value class)
- ✅ Pro: Recursive conversion for nested types
- ⚠️ Con: Must handle conversion edge cases

**Implementation Impact**:
- Implement `FromPyObject` for Value types
- Create Value Python class with variant constructors
- Handle recursive conversion for VArray and VMap
- Test conversion edge cases (None, empty collections, etc.)

## Summary

All technical decisions resolved. Key choices:
1. **PyO3** for Rust-Python bindings (standard, proven approach)
2. **Separate Pattern and PatternSubject classes** for type safety
3. **Manual type stubs** (.pyi files) for comprehensive type checking
4. **PyCFunction with type hints** for Python callbacks
5. **Python exception mapping** for user-friendly errors
6. **Multi-layered testing** (Rust + Python + type checking)
7. **Comprehensive documentation** (API reference + examples + docstrings)
8. **Automatic Value conversion** with explicit Value class option

All decisions align with constitution principles and existing gram-codec patterns.
