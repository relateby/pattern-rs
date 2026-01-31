# WASM / JavaScript API Contract

**Feature**: 027-wasm-pattern-typescript-parity  
**Date**: 2026-01-31

## Module: pattern_core (WASM exports)

JavaScript/TypeScript consumers load the WASM module (e.g. via wasm-pack output or init function). The following API MUST be exposed with behavior equivalent to the Python binding (024-python-pattern-core). Naming MAY follow JS/TS conventions (camelCase); semantics MUST match Python.

### Pattern

**Construction**:
- `Pattern.point(value: JsValue) -> Pattern` — atomic pattern (no elements)
- `Pattern.of(value: JsValue) -> Pattern` — alias for point
- `Pattern.pattern(value: JsValue, elements: Pattern[]) -> Pattern` — pattern with value and children
- `Pattern.fromValues(values: JsValue[]) -> Pattern[]` — lift each value to a Pattern (point)

**Accessors** (getters or methods):
- `pattern.value` — value (JsValue)
- `pattern.elements` — array of Pattern

**Inspection**:
- `pattern.length() -> number`
- `pattern.size() -> number`
- `pattern.depth() -> number`
- `pattern.isAtomic() -> boolean`
- `pattern.values() -> JsValue[]` (pre-order)

**Query**:
- `pattern.anyValue(predicate: (v: JsValue) => boolean) -> boolean`
- `pattern.allValues(predicate: (v: JsValue) => boolean) -> boolean`
- `pattern.filter(predicate: (p: Pattern) => boolean) -> Pattern`
- `pattern.findFirst(predicate: (p: Pattern) => boolean) -> Pattern | null`
- `pattern.matches(other: Pattern) -> boolean`
- `pattern.contains(other: Pattern) -> boolean`

**Transformation**:
- `pattern.map(fn: (v: JsValue) => JsValue) -> Pattern`
- `pattern.fold(init: JsValue, fn: (acc: JsValue, v: JsValue) => JsValue) -> JsValue`
- `pattern.para(fn: (value: JsValue, elementResults: JsValue[]) => JsValue) -> JsValue` — paramorphism: bottom-up fold with (node value, array of child results). Behavior equivalent to Rust pattern-core para (025-pattern-paramorphism).

**Combination**:
- `pattern.combine(other: Pattern) -> Pattern` (when value type supports combination; e.g. Subject)

**Comonad**:
- `pattern.extract() -> JsValue`
- `pattern.extend(fn: (p: Pattern) => JsValue) -> Pattern`
- `pattern.depthAt() -> Pattern` (value type number)
- `pattern.sizeAt() -> Pattern` (value type number)
- `pattern.indicesAt() -> Pattern` (value type array of numbers)

**Validation / Analysis** (fallible operations return Either-like; see Result/Either contract below):
- `pattern.validate(rules: ValidationRules) -> Result<void, ValidationError>` — returns an Either-like value (e.g. `{ _tag: 'Right', right: undefined }` | `{ _tag: 'Left', left: ValidationError }`), trivially convertible to effect-ts Either. Does NOT throw.
- `pattern.analyzeStructure() -> StructureAnalysis` — infallible

### Subject

**Construction**:
- `Subject.new(identity: string, labels?: string[] | Iterable<string>, properties?: Record<string, Value>) -> Subject` (or equivalent)

**Accessors**:
- `subject.identity` — string or Symbol
- `subject.labels` — array or set of strings
- `subject.properties` — record of string to Value

**Methods** (if exposed; otherwise readonly attributes):
- `subject.addLabel(label: string) -> void`
- `subject.removeLabel(label: string) -> void`
- `subject.hasLabel(label: string) -> boolean`
- `subject.getProperty(name: string) -> Value | undefined`
- `subject.setProperty(name: string, value: Value) -> void`
- `subject.removeProperty(name: string) -> void`

### Value

**Factories** (static methods or module functions):
- `Value.string(s: string) -> Value`
- `Value.int(i: number) -> Value`
- `Value.decimal(n: number) -> Value`
- `Value.boolean(b: boolean) -> Value`
- `Value.symbol(s: string) -> Value`
- `Value.array(items: Value[]) -> Value`
- `Value.map(entries: Record<string, Value>) -> Value`
- `Value.range(lower?: number, upper?: number) -> Value`
- `Value.measurement(value: number, unit: string) -> Value`

**Extractors** (or type discriminators):
- `value.asString() -> string` (throws if not string)
- `value.asInt() -> number`
- `value.asDecimal() -> number`
- `value.asBoolean() -> boolean`
- `value.asArray() -> Value[]`
- `value.asMap() -> Record<string, Value>`
- (and analogous for symbol, range, measurement as needed)

### ValidationRules

- Constructor or factory: `ValidationRules.new({ maxDepth?: number, maxElements?: number })`
- Fields: `maxDepth`, `maxElements` (optional numbers)

### StructureAnalysis

- Attributes: `summary` (string), `depthDistribution` (number[]), `elementCounts` (number[]), `nestingPatterns` (string[]) or equivalent as per Python.

### Result / Either contract (fallible operations)

- Fallible operations (e.g. `validate`, and any WASM-exposed `traverse_result`/`sequence_result`/`validate_all` if exposed) MUST return a value that is trivially convertible to effect-ts Either: either the same shape as `Either.right(value)` / `Either.left(error)` (e.g. `{ _id: 'Either', _tag: 'Right', right: T }` | `{ _id: 'Either', _tag: 'Left', left: E }`) or a documented one-line conversion. They MUST NOT throw.
- Error payloads (e.g. `ValidationError`) MUST be represented as plain objects (e.g. `{ message: string, ruleViolated?: string }`) so that `Either.left(error)` can hold them and consumers can use effect-ts `Either.match`, `Either.map`, etc.
- Usage with effect-ts MUST be documented in the package README or quickstart (e.g. “Return value is compatible with Effect’s Either; use `Either.fromNullable` or pass directly into Effect pipelines”).

## Parity Requirements

- For the same logical input (same structure and values), the result of any operation (construct, map, filter, combine, depth, size, etc.) MUST match the result of the equivalent Python binding operation. This contract is the source of truth for the JS surface; TypeScript types are defined in typescript-types.md.
