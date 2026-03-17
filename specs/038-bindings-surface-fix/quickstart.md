# Quickstart: TypeScript and Python Surface Validation

**Feature**: 038-bindings-surface-fix | **Date**: 2026-03-17

## Goal

Verify that the documented TypeScript and Python public package surfaces are complete, consistent, and usable without internal imports.

## TypeScript

### 1. Build and package the public artifact

```bash
cd /Users/akollegger/Developer/gram-data/pattern-rs/typescript/@relateby/pattern
npm run build
npm pack
```

### 2. Validate public imports only

Use only:

```typescript
import {
  init,
  NativeSubject,
  NativePattern,
  NativePatternGraph,
  NativeGraphQuery,
  NativeReconciliationPolicy,
  NativeValidationRules,
  NativeValue,
  StandardGraph,
  Gram,
  GraphClass,
  TraversalDirection,
  toGraphView,
} from "@relateby/pattern";
```

### 3. Confirm representative workflows

1. Initialize the package from the public entry point.
2. Create values, subjects, and patterns from the public entry point.
3. Build or parse a graph using only the public entry point.
4. Use one pure TypeScript graph utility from the same package.
5. Confirm invalid input or missing setup fails through the documented public path.

## Python

### 1. Build the combined wheel

```bash
cd /Users/akollegger/Developer/gram-data/pattern-rs/python/relateby
pip wheel . -w dist
```

### 2. Install and validate the public imports

Use only:

```python
import relateby.pattern
import relateby.gram

from relateby.pattern import Pattern, Subject, Value, ValidationRules, StandardGraph
from relateby.gram import parse_gram, validate_gram, round_trip
```

### 3. Confirm representative workflows

1. Create pattern-facing objects from `relateby.pattern`.
2. Run one graph-oriented workflow from `relateby.pattern`.
3. Run one gram-oriented workflow from `relateby.gram`.
4. Confirm any wrapper-owned helper behaves from the public package boundary.
5. Confirm invalid input fails with the documented public behavior.

## Documentation and Example Validation

For both languages:

1. Run or validate official quick-start snippets using only public imports.
2. Confirm every referenced symbol exists from the documented package boundary.
3. Confirm example return shapes and error handling match runtime behavior.
4. Reject any guide that depends on internal modules, generated files, or stale symbol names.

## Completion Criteria

The quickstart passes when:

- Public imports work for both languages.
- Representative workflows succeed without internal imports.
- Docs and examples match the packaged artifact behavior.
- TypeScript type declarations and Python stubs match the public runtime surface.
