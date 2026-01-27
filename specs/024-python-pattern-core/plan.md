# Implementation Plan: Python Pattern-Core Bindings

**Branch**: `024-python-pattern-core` | **Date**: 2026-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/024-python-pattern-core/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Create Python bindings for the pattern-core Rust crate using PyO3, enabling Python developers to programmatically construct and operate on Pattern and Subject instances. The implementation will provide a Pythonic API with full type safety support via .pyi type stubs, comprehensive tests, Python usage documentation, and updated examples. The bindings will expose all pattern-core functionality including construction, transformation, query, combination, and comonad operations while maintaining performance within 2x of native Rust operations.

## Technical Context

**Language/Version**: Python 3.8+ (type hints support), Rust 1.75+ (edition 2021)  
**Primary Dependencies**: PyO3 0.23+ (Rust-Python bindings), maturin (Python packaging), pattern-core crate (existing Rust library)  
**Storage**: N/A (in-memory data structures only)  
**Testing**: pytest (Python tests), cargo test (Rust unit tests), mypy/pyright (type checking)  
**Target Platform**: Python 3.8+ on macOS (x86_64, arm64), Linux (x86_64, aarch64), Windows (x86_64)  
**Project Type**: Rust library with Python extension module (cdylib)  
**Performance Goals**: Python bindings maintain performance within 2x of native Rust operations for patterns with up to 1000 nodes  
**Constraints**: 
- Must preserve pattern structure invariants across Python-Rust boundary
- Python callbacks (map/filter functions) must be type-safe
- Error messages must be Python-friendly (not Rust-specific terminology)
- Type stubs (.pyi files) must support mypy and pyright
- Python API must be Pythonic (snake_case, dict/list/set types)  
**Scale/Scope**: 
- Support all pattern-core public API methods (~30+ methods)
- Handle patterns with up to 1000 nodes efficiently
- Support deeply nested patterns (100+ levels) with stack overflow protection
- Type-safe conversion for all Subject Value types (VString, VInt, VDecimal, VBoolean, VSymbol, VArray, VMap, VRange, VMeasurement)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅
- **Status**: PASS
- **Rationale**: This feature creates Python bindings for existing pattern-core Rust crate, which already faithfully replicates gram-hs reference implementation. The Python bindings expose the same API semantics, ensuring behavioral equivalence. Python examples will demonstrate usage patterns consistent with Rust examples.

### II. Correctness & Compatibility ✅
- **Status**: PASS
- **Rationale**: Python bindings maintain compatibility with pattern-core Rust API contracts. All operations preserve pattern structure invariants. Error handling converts Rust errors to appropriate Python exceptions while maintaining semantic correctness.

### III. Rust Native Idioms ✅
- **Status**: PASS
- **Rationale**: Implementation uses PyO3 (standard Rust-Python binding library) following idiomatic Rust patterns. Error handling uses Result types converted to Python exceptions. Python API uses Pythonic naming (snake_case) while Rust internals use Rust conventions.

### IV. Multi-Target Library Design ✅
- **Status**: PASS
- **Rationale**: Python bindings are feature-gated (`python` feature flag), allowing pattern-core to remain WASM-compatible. Python-specific code is isolated in `src/python.rs` module. The core pattern-core crate remains platform-agnostic.

### V. External Language Bindings & Examples ✅
- **Status**: PASS
- **Rationale**: This feature explicitly provides Python bindings with minimal working examples, comprehensive documentation, and type hints. Examples demonstrate core functionality (construction, operations, type safety). Build instructions included for maturin-based packaging.

**Constitution Check Result**: ✅ **ALL GATES PASS** - No violations detected. Feature aligns with all constitution principles.

**Note**: When porting features from gram-hs, reference the local implementation at `../gram-hs` and corresponding feature specifications in `../gram-hs/specs/`. See [PORTING_GUIDE.md](../../../PORTING_GUIDE.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/024-python-pattern-core/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
│   └── python-api.md    # Python API contract definitions
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── Cargo.toml           # Add pyo3 dependency with python feature flag
├── src/
│   ├── lib.rs           # Re-export python module when feature enabled
│   ├── pattern.rs       # Existing Pattern implementation
│   ├── subject.rs       # Existing Subject implementation
│   └── python.rs         # NEW: PyO3 bindings (feature-gated)
├── pyproject.toml       # NEW: maturin configuration for Python packaging
├── tests/
│   └── python/          # NEW: Python integration tests
│       ├── test_pattern.py
│       ├── test_subject.py
│       ├── test_operations.py
│       └── test_type_safety.py
└── pattern_core/
    └── __init__.pyi     # NEW: Type stubs for Python type checkers

examples/pattern-core-python/
├── README.md            # NEW: Python usage documentation
├── basic_usage.py       # NEW: Basic construction examples
├── operations.py        # NEW: Pattern operations examples
├── type_safety.py       # NEW: Type hints and mypy examples
└── advanced.py          # NEW: Advanced use cases (comonad, complex subjects)

docs/
└── python-usage.md      # NEW: Comprehensive Python API documentation
```

**Structure Decision**: 
- Python bindings added to existing `pattern-core` crate via feature-gated module (`src/python.rs`)
- Python tests organized in `tests/python/` directory
- Type stubs in `pattern_core/__init__.pyi` for IDE and type checker support
- Examples in `examples/pattern-core-python/` following existing example structure
- Comprehensive documentation in `docs/python-usage.md` and example READMEs

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No violations detected - all constitution gates pass.
