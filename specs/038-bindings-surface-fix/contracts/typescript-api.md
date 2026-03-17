# TypeScript API Contract: Public Package Surface

**Feature**: 038-bindings-surface-fix | **Date**: 2026-03-17

## Supported Package Boundary

The supported TypeScript entry point for this feature is:

```typescript
import { ... } from "@relateby/pattern"
```

Consumers must not need to import from generated implementation paths such as `wasm/`, `wasm-node/`, or internal source files to complete documented workflows.

## Public Surface Requirements

### Package-Level Rules

1. The top-level package must expose every documented public symbol needed for common developer workflows.
2. Any package-level alias must map to a real underlying capability and preserve the documented behavior.
3. The package must not document generated-only or internal symbols as stable public API unless they are intentionally re-exposed from the top level.
4. Any asynchronous package helper must be documented and typed consistently with its runtime behavior.

## Workflow Contracts

### Workflow A: Initialize and use the package

```typescript
import { init } from "@relateby/pattern";
```

**Contract**:
- `init()` is available from the package entry point.
- The documented initialization story matches real runtime requirements for both Node and bundler use.
- Failure to initialize, when relevant, surfaces as a documented public error.

### Workflow B: Use core pattern and graph-facing types

```typescript
import {
  NativeSubject,
  NativePattern,
  NativePatternGraph,
  NativeGraphQuery,
  NativeReconciliationPolicy,
  NativeValidationRules,
  NativeValue,
  StandardGraph,
} from "@relateby/pattern";
```

**Contract**:
- Every documented symbol above must be importable from the package entry point if it appears in docs or examples.
- Documented method names, chaining behavior, and return types must match runtime behavior.
- Package-level naming may be curated, but the curated name must be stable and fully documented.

### Workflow C: Use parsing and serialization helpers

```typescript
import { Gram } from "@relateby/pattern";
```

**Contract**:
- The package-level `Gram` surface must expose every documented parsing and serialization workflow intended for end users.
- Runtime behavior, package-level typings, and examples must agree on whether each `Gram` operation is synchronous or asynchronous.
- Public parsing workflows must not require knowledge of generated WASM module paths.

### Workflow D: Use pure TypeScript graph utilities

```typescript
import {
  GraphClass,
  TraversalDirection,
  toGraphView,
  mapGraph,
  filterGraph,
  foldGraph,
  paraGraph,
  unfoldGraph,
} from "@relateby/pattern";
```

**Contract**:
- The package remains self-contained for documented graph-interface and transform utilities.
- These utilities continue to compose with the documented graph-facing workflow from the same package boundary.

## Export Family Expectations

### Exported families that must remain aligned

- Initialization helpers
- Core pattern/value/subject constructors
- Graph-facing classes and helpers
- Parsing and serialization helpers
- Constants and pure TypeScript graph utilities
- Publicly documented analysis, validation, and error-facing helpers

## Error Contract

| Failure Condition | Expected Package-Level Outcome |
|------------------|--------------------------------|
| Documented symbol is unavailable | Fails package verification before release |
| Runtime behavior differs from package typing | Fails type/runtime alignment checks before release |
| Example references unsupported import or call pattern | Fails example validation before release |
| Initialization or parsing flow is incomplete or misleading | Fails public workflow contract checks before release |

## Verification Contract

Release readiness for the TypeScript surface requires all of the following:

1. Top-level export inventory matches the documented public symbol set.
2. Generated binding inventory and package alias map remain in sync.
3. Consumer import and typecheck validation succeeds against the packed npm artifact.
4. Representative runtime smoke tests succeed against the packed npm artifact.
5. Official TypeScript docs and examples execute or typecheck successfully using only public imports.
