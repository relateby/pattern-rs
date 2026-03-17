# Python API Contract: Unified Public Package Surface

**Feature**: 038-bindings-surface-fix | **Date**: 2026-03-17

## Supported Package Boundaries

The supported Python imports for this feature are:

```python
import relateby.pattern
import relateby.gram
```

Consumers must not need to import `pattern_core`, `gram_codec`, or other internal/native modules to complete documented workflows.

## Public Surface Requirements

### Package-Level Rules

1. The unified wrapper package is the authoritative public API boundary.
2. Wrapper-only public behavior is allowed when required by package composition, but it must be documented, typed, and tested from the wrapper boundary.
3. Public stubs must describe the wrapper package that users import, not a different internal layer.
4. Official examples and guides must use only `relateby.pattern` and `relateby.gram` imports.

## Workflow Contracts

### Workflow A: Import the supported package surface

```python
import relateby.pattern as pattern
import relateby.gram as gram
```

**Contract**:
- These imports work from the built distribution artifact.
- Public guidance does not require direct use of internal native modules.

### Workflow B: Create and inspect pattern-facing objects

```python
from relateby.pattern import Pattern, Subject, Value, ValidationRules, StructureAnalysis
```

**Contract**:
- Every documented public symbol must exist from `relateby.pattern`.
- Documented constructor signatures, helper names, and return shapes must match runtime behavior and shipped stubs.
- Publicly documented aliases or helper classes must exist at runtime if they are described as importable names.

### Workflow C: Use graph-oriented helpers from the public package

```python
from relateby.pattern import StandardGraph, Subject
```

**Contract**:
- `StandardGraph` workflows documented for the unified package, including parsing-oriented helpers owned by the wrapper layer, must work from the public package boundary.
- Chainable graph-building workflows, element access, and query helpers must behave as documented.
- Public graph workflows must not require callers to know whether behavior comes from a native module or a wrapper layer.

### Workflow D: Use gram helpers from the public package

```python
from relateby.gram import parse_gram, validate_gram, round_trip
```

**Contract**:
- Public gram helpers and their documented return shapes must match the built distribution behavior.
- Official examples must use the actual public return model rather than stale or internal assumptions.

## Error Contract

| Failure Condition | Expected Public Outcome |
|------------------|-------------------------|
| Public wrapper symbol is missing | Fails wheel-based public import verification before release |
| Wrapper behavior differs from stubs or docs | Fails public contract or stub alignment verification before release |
| Invalid public workflow raises an undocumented exception class or shape | Fails public error contract verification before release |
| Example depends on internal imports or stale return shapes | Fails example validation before release |

## Typing Contract

1. Shipped stubs must describe the public `relateby.pattern` and `relateby.gram` surfaces that external developers install.
2. If the wrapper package adds or curates public behavior, that behavior must be reflected in the shipped stubs.
3. Public symbols described in stubs must exist at runtime in the installed wheel.

## Verification Contract

Release readiness for the Python surface requires all of the following:

1. Public import checks succeed against the built combined wheel.
2. Wrapper-owned public workflows succeed from the installed `relateby` package.
3. Stub validation succeeds against the supported public imports.
4. Representative runtime smoke tests and examples succeed from the installed wheel.
5. Official Python docs and examples use only supported public imports and match real return types and error behavior.
