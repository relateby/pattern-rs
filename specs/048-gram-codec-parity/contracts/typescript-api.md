# Contract: TypeScript API — Gram namespace in @relateby/pattern (048)

Namespace: `Gram` exported from `@relateby/pattern` (and re-exported by `@relateby/gram`)

All functions return `Effect<T, GramParseError>`. Use `Effect.runPromise` to convert to a Promise.

---

## Existing (unchanged)

### `Gram.parse(input: string): Effect<ReadonlyArray<Pattern<Subject>>, GramParseError>`

Parse gram notation into an array of Pattern objects. Unchanged. If a leading bare record is present, it appears as the first element with empty identity, no labels, and non-empty properties.

### `Gram.stringify(patterns: ReadonlyArray<Pattern<Subject>>): Effect<string, GramParseError>`

Serialize Pattern objects to gram notation. Unchanged.

### `Gram.validate(input: string): Effect<void, GramParseError>`

Validate gram notation syntax. Unchanged.

---

## New

### `Gram.parseWithHeader(input: string): Effect<{ header: Record<string, unknown> | undefined, patterns: ReadonlyArray<Pattern<Subject>> }, GramParseError>`

Parse gram notation, separating an optional leading header record from the patterns.

```typescript
import { Gram } from "@relateby/pattern"
import { Effect } from "effect"

// With header
const result = await Effect.runPromise(
  Gram.parseWithHeader("{version: 1} (alice)-[:KNOWS]->(bob)")
)
result.header   // { version: 1 }
result.patterns // [Pattern<Subject>]

// Without header
const result2 = await Effect.runPromise(
  Gram.parseWithHeader("(alice)-[:KNOWS]->(bob)")
)
result2.header   // undefined
result2.patterns // [Pattern<Subject>]
```

**Input**: gram notation string
**Output**: `Effect` resolving to `{ header: Record<string, unknown> | undefined, patterns: ReadonlyArray<Pattern<Subject>> }`
**Errors**: `Effect` failing with `GramParseError` if input is syntactically invalid
**Guarantee**: `patterns` never contains the header record — it is always separated into `header`

---

### `Gram.stringifyWithHeader(header: Record<string, unknown> | undefined, patterns: ReadonlyArray<Pattern<Subject>>): Effect<string, GramParseError>`

Serialize a header record and Pattern objects to gram notation.

```typescript
import { Gram } from "@relateby/pattern"
import { Effect } from "effect"

// With header
const gram = await Effect.runPromise(
  Gram.stringifyWithHeader({ version: 1 }, patterns)
)
// → "{version: 1}\n(alice)-[:KNOWS]->(bob)"

// Without header
const gram2 = await Effect.runPromise(
  Gram.stringifyWithHeader(undefined, patterns)
)
// → "(alice)-[:KNOWS]->(bob)"

// Header only, no patterns
const gram3 = await Effect.runPromise(
  Gram.stringifyWithHeader({ version: 1 }, [])
)
// → "{version: 1}"
```

**Input**: `header` (`Record<string, unknown> | undefined`), `patterns` (`ReadonlyArray<Pattern<Subject>>`)
**Output**: `Effect` resolving to gram notation string
**Errors**: `Effect` failing with `GramParseError` if serialization fails (e.g. unsupported value types in header)

---

## WASM layer changes (adapters/wasm/pattern-wasm/src/gram.rs)

The existing `parseToJson` / `stringifyFromJson` methods (which returned/accepted JSON strings) are replaced by `parse` / `stringify` that return/accept `JsValue` directly via `serde-wasm-bindgen json_compatible()`. Two new methods are added:

```rust
// Returns JsValue: { header: Record<string, unknown> | null, patterns: AstPattern[] }
pub fn parse_with_header(gram: &str) -> Result<JsValue, JsValue>

// Accepts JsValue: { header: Record<string, unknown> | undefined, patterns: AstPattern[] }
pub fn stringify_with_header(input: JsValue) -> Result<String, JsValue>
```

These are internal implementation details — not part of the public TypeScript `Gram` API contract. No `JSON.parse` or `JSON.stringify` is involved anywhere in the call path.

---

## Round-trip guarantee

```typescript
const { header, patterns } = await Effect.runPromise(
  Gram.parseWithHeader(gramText)
)
const recovered = await Effect.runPromise(
  Gram.stringifyWithHeader(header, patterns)
)
const { header: h2, patterns: p2 } = await Effect.runPromise(
  Gram.parseWithHeader(recovered)
)
// h2 deep-equals header, p2 deep-equals patterns
```
