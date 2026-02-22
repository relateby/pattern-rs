# Implementation Plan: Paramorphism in Python Bindings

**Branch**: `026-paramorphism-python-bindings` | **Date**: 2026-01-31 | **Spec**: [spec.md](./spec.md)  
**Input**: Feature specification from `/specs/026-paramorphism-python-bindings/spec.md`

## Summary

Expose the existing Rust paramorphism API (`para`) in the pattern-core Python bindings on the **generic Pattern class only**; **review PatternSubject and remove it if possible** so that Subject is used as the value type of `Pattern` (no separate class); and **define Pattern as a generic class** in Python type stubs using `from typing import TypeVar, Generic` (e.g. `class Pattern(Generic[V])`) for correct type inference. Implementation adds a `para` method on Pattern (PyO3 callback into Rust `para`), removes or deprecates PatternSubject after review, and updates `__init__.pyi` so that Pattern is declared as `Generic[V]` with typed constructors and `para`.

## Technical Context

**Language/Version**: Python 3.8+ (type hints, Generic), Rust 1.75+ (edition 2021)  
**Primary Dependencies**: PyO3 (existing, pattern-core `python` feature), maturin, pattern-core crate with existing `para` (spec 025), `typing.TypeVar` and `typing.Generic` for Pattern[V]  
**Storage**: N/A (in-memory only)  
**Testing**: pytest (Python), cargo test (Rust), mypy/pyright (type checking)  
**Target Platform**: Same as 024 — Python 3.8+ on macOS (x86_64, arm64), Linux (x86_64, aarch64), Windows (x86_64)  
**Project Type**: Extension to existing Rust library with Python extension module (cdylib)  
**Performance Goals**: Python `para` within 2x of native Rust `para` for patterns up to ~1000 nodes  
**Constraints**: Pattern as single generic class; Subject as value type when needed; paramorphism on Pattern only; type stubs use Generic[V]  
**Scale/Scope**: Add `para` on Pattern; review and remove PatternSubject if possible; define Pattern as Generic[V] in .pyi; update tests/examples/docs

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Reference Implementation Fidelity ✅
- **Status**: PASS
- **Rationale**: Rust `para` already replicates gram-hs (spec 025). This feature exposes it in Python on Pattern only; semantics match Rust. Simplifying to generic Pattern (no PatternSubject) is a Python API design choice that preserves Rust behavior.

### II. Correctness & Compatibility ✅
- **Status**: PASS
- **Rationale**: Python `para` must produce the same results as Rust `para` for the same inputs. Removing PatternSubject (if done) requires a migration path for existing callers (Pattern.point(subject), Pattern.pattern(subject, elements)); compatibility documented.

### III. Rust Native Idioms ✅
- **Status**: PASS
- **Rationale**: New code in existing `src/python.rs`; PyO3 patterns unchanged. Python-side typing (Generic[V]) is standard Python typing, not Rust.

### IV. Multi-Target Library Design ✅
- **Status**: PASS
- **Rationale**: Python code remains behind existing `python` feature; core crate unchanged.

### V. External Language Bindings & Examples ✅
- **Status**: PASS
- **Rationale**: Adds `para` to Python bindings; simplifies API (generic Pattern); examples and docs updated.

**Constitution Check Result**: ✅ **ALL GATES PASS**

**Note**: When porting features from gram-hs, reference the local implementation at `../pattern-hs` and corresponding feature specifications in `../pattern-hs/specs/`. See [porting guide](../../docs/porting-guide.md) for detailed porting instructions.

## Project Structure

### Documentation (this feature)

```text
specs/026-paramorphism-python-bindings/
├── plan.md              # This file
├── research.md          # Phase 0 output (includes PatternSubject removal, Pattern Generic[V])
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── python-api-para.md
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
crates/pattern-core/
├── src/
│   ├── python.rs        # Add para on PyPattern; review PyPatternSubject — remove or deprecate
│   └── pattern.rs       # No change (para already implemented)
├── pattern_core/
│   ├── __init__.pyi     # Define Pattern as Generic[V]: class Pattern(Generic[V]); add para; remove/deprecate PatternSubject
│   └── __init__.py      # Re-exports; align with .pyi (drop PatternSubject if removed)
├── tests/               # Add Python tests for para; migrate PatternSubject tests to Pattern with Subject value
└── pyproject.toml       # No change

examples/pattern-core-python/
├── operations.py        # Add paramorphism examples; use Pattern only (Subject as value if needed)
└── README.md            # Mention para; document Pattern[V] and Subject as value
```

**Structure Decision**: Additions and changes are in `crates/pattern-core` (python.rs, __init__.pyi, __init__.py, tests) and `examples/pattern-core-python`. Plan includes (1) add para on Pattern, (2) review PatternSubject and remove if possible, (3) define Pattern as Generic[V] in .pyi.

## Complexity Tracking

*No constitution violations. Table left empty.*
