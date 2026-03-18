# Data Model: Native TypeScript and Python Bindings

**Feature**: 039-native-bindings
**Date**: 2026-03-17
**Source**: gram-hs reference (`../pattern-hs/libs/`) + existing codebase
**Design reference**: `proposals/migrate-ts-python-proposal.md` — TypeScript implementation sketch is normative for the effect-ts approach.

---

## TypeScript Design Philosophy

The TypeScript implementation uses the [Effect](https://effect.website) library throughout. This is not an incidental dependency — it shapes every entity's design:

| concern | effect-ts tool | why |
|---------|---------------|-----|
| Structural equality on `Pattern` and `Subject` | `Data.Class` | `Equal.equals(p1, p2)` works without a custom comparator; opaque WASM handles made this impossible before |
| Tagged variants on `Value` | `Data.Case` interfaces + `Data.tagged()` constructors | `_tag` discriminant enables exhaustive `switch`; structural equality is built in |
| Validating the WASM JSON payload | `Schema.TaggedStruct`, `Schema.Struct`, `Schema.suspend` | Entire tree is validated in one pass before any `Pattern` is constructed; bad codec output surfaces as a typed error, not a mid-tree crash |
| Optional results (`findFirst`) | `Option.Option<V>` | Absence is explicit and composable; no `V \| undefined` ambiguity |
| Async + fallible codec calls | `Effect<A, E>` | Typed errors (`GramParseError`) in the type signature; no thrown exceptions; interops with `Promise` via `Effect.runPromise` |
| Composing operations | `pipe` | Point-free style; standalone curried functions compose without method chaining |

---

## Core Entities

### Pattern\<V\>

A recursive tree structure. The foundational data type for all graph operations.

| Field | Type | Notes |
|-------|------|-------|
| `value` | `V` | The payload at this node. For graph use, `V = Subject`. |
| `elements` | `ReadonlyArray<Pattern<V>>` | Ordered child patterns. Empty list = atomic/leaf node. |

**Constructors**: `point(v)` → atomic; `of(v)` → alias for `point`.

**Invariants**:
- A `Pattern` with zero elements is "atomic".
- `size` = 1 + sum of all descendant sizes.
- `depth` = 0 for atomic; 1 + max(child depths) otherwise.

**Operations** (all pure, no mutation):

| Operation | Signature | Semantics |
|-----------|-----------|-----------|
| `map` | `(V → U) → Pattern<V> → Pattern<U>` | Transform every value; preserve structure. Pre-order. |
| `fold` | `(R, (R, V) → R) → Pattern<V> → R` | Accumulate values. Pre-order traversal (root first). |
| `filter` | `(Pattern<V> → bool) → Pattern<V> → Pattern<V>[]` | Collect matching subtrees. Pre-order. |
| `findFirst` | `(V → bool) → Pattern<V> → Option<V>` | First matching value. Returns `Option.none` if absent. |
| `extend` | `(Pattern<V> → U) → Pattern<V> → Pattern<U>` | Context-aware map: function sees full subtree at each position. |
| `extract` | `Pattern<V> → V` | Root value. Satisfies `extract(extend(f)(p)) == f(p)`. |
| `duplicate` | `Pattern<V> → Pattern<Pattern<V>>` | Each node's value becomes its own subtree. |

**Laws** (verified by property-based tests):
- Functor: `map(id) == id`; `map(f ∘ g) == map(f) ∘ map(g)`
- Foldable: pre-order traversal order is invariant
- Comonad: `extract(extend(f)(p)) == f(p)`; `extend(extract) == id`; `extend(f)(extend(g)(p)) == extend(f ∘ extend(g))(p)`

#### TypeScript implementation — `Data.Class`

`Pattern<V>` extends `Data.Class` so that `Equal.equals` works structurally across the entire tree. Operations are **standalone curried functions** (not methods) so they compose with `pipe`:

```typescript
import { Data, Option, pipe } from "effect"

export class Pattern<V> extends Data.Class<{
  readonly value:    V
  readonly elements: ReadonlyArray<Pattern<V>>
}> {
  static point<V>(value: V): Pattern<V> {
    return new Pattern({ value, elements: [] })
  }
  get isAtomic(): boolean { return this.elements.length === 0 }
  // size, depth as getters
}

// Standalone pipeable operations
export const map = <V, U>(fn: (v: V) => U) =>
  (p: Pattern<V>): Pattern<U> =>
    new Pattern({ value: fn(p.value), elements: p.elements.map(map(fn)) })

export const fold = <V, R>(init: R, fn: (acc: R, v: V) => R) =>
  (p: Pattern<V>): R =>
    p.elements.reduce((acc, e) => pipe(e, fold(acc, fn)), fn(init, p.value))

export const findFirst = <V>(pred: (v: V) => boolean) =>
  (p: Pattern<V>): Option.Option<V> =>
    pred(p.value)
      ? Option.some(p.value)
      : p.elements.reduce(
          (found: Option.Option<V>, e) =>
            Option.orElse(found, () => pipe(e, findFirst(pred))),
          Option.none()
        )

// Comonad
export const extend = <V, U>(fn: (p: Pattern<V>) => U) =>
  (p: Pattern<V>): Pattern<U> =>
    new Pattern({ value: fn(p), elements: p.elements.map(extend(fn)) })

export const extract = <V>(p: Pattern<V>): V => p.value

export const duplicate = <V>(p: Pattern<V>): Pattern<Pattern<V>> =>
  new Pattern({ value: p, elements: p.elements.map(duplicate) })
```

---

### Subject

A self-describing value: a uniquely identified entity with labels and properties.

| Field | Type | Notes |
|-------|------|-------|
| `identity` | `string` | Required unique identifier. |
| `labels` | `ReadonlySet<string>` / `set[str]` | Type labels (e.g., "Person", "KNOWS"). Can be empty. |
| `properties` | `ReadonlyMap<string, Value>` / `dict[str, Value]` | Named property values. Can be empty. |

**Constructors**: `fromId(id)` → minimal Subject; `withLabel(l)` / `withProperty(k, v)` → immutable builders.

**Merge semantics** (Semigroup): `a <> b` = identity from `a` (if non-empty, else `b`), union of labels, union of properties (left-biased on key conflict).

#### TypeScript implementation — `Data.Class`

`Subject` extends `Data.Class` for structural equality. Builder methods return new instances (immutable):

```typescript
export class Subject extends Data.Class<{
  readonly identity:   string
  readonly labels:     ReadonlySet<string>
  readonly properties: ReadonlyMap<string, Value>
}> {
  static fromId(identity: string): Subject {
    return new Subject({ identity, labels: new Set(), properties: new Map() })
  }
  withLabel(label: string): Subject {
    return new Subject({ ...this, labels: new Set([...this.labels, label]) })
  }
  withProperty(name: string, value: Value): Subject {
    return new Subject({
      ...this,
      properties: new Map([...this.properties, [name, value]])
    })
  }
}
```

---

### Value

A tagged union of primitive and structured value types. All 10 variants are required — see `research.md` Decision 1.

| Variant | Fields | Gram notation example |
|---------|--------|-----------------------|
| `StringVal` | `value: string` | `"hello"` |
| `IntVal` | `value: number` | `42` |
| `FloatVal` | `value: number` | `3.14` |
| `BoolVal` | `value: boolean` | `true` |
| `NullVal` | — | `null` |
| `SymbolVal` | `value: string` | `myIdentifier` (unquoted) |
| `TaggedStringVal` | `tag: string; content: string` | `` url`https://example.com` `` |
| `ArrayVal` | `items: Value[]` | `[1, 2, "three"]` |
| `MapVal` | `entries: Map<string, Value>` | `{a: 1, b: "x"}` (as a value, not subject properties) |
| `RangeVal` | `lower?: number; upper?: number` | `1..10`, `1...`, `...10`, `...` |
| `MeasurementVal` | `unit: string; value: number` | `kg 5.0` |

All variants carry structural equality. Variants are distinguishable by the `_tag` discriminant (TypeScript) or class hierarchy (Python).

#### TypeScript implementation — `Data.Case` + `Data.tagged`

Each variant is a `Data.Case` interface. `Data.tagged()` creates a constructor that fills `_tag` automatically and provides structural equality:

```typescript
import { Data, Schema } from "effect"

// Each variant is a Data.Case — Equal.equals works structurally
export interface StringVal extends Data.Case { readonly _tag: "StringVal"; readonly value: string }
export interface IntVal    extends Data.Case { readonly _tag: "IntVal";    readonly value: number }
export interface FloatVal  extends Data.Case { readonly _tag: "FloatVal";  readonly value: number }
export interface BoolVal   extends Data.Case { readonly _tag: "BoolVal";   readonly value: boolean }
export interface NullVal   extends Data.Case { readonly _tag: "NullVal" }
export interface SymbolVal extends Data.Case { readonly _tag: "SymbolVal"; readonly value: string }
export interface TaggedStringVal extends Data.Case { readonly _tag: "TaggedStringVal"; readonly tag: string; readonly content: string }
export interface ArrayVal  extends Data.Case { readonly _tag: "ArrayVal";  readonly items: ReadonlyArray<Value> }
export interface MapVal    extends Data.Case { readonly _tag: "MapVal";    readonly entries: ReadonlyMap<string, Value> }
export interface RangeVal  extends Data.Case { readonly _tag: "RangeVal";  readonly lower?: number; readonly upper?: number }
export interface MeasurementVal extends Data.Case { readonly _tag: "MeasurementVal"; readonly unit: string; readonly value: number }

export type Value =
  StringVal | IntVal | FloatVal | BoolVal | NullVal | SymbolVal |
  TaggedStringVal | ArrayVal | MapVal | RangeVal | MeasurementVal

// Constructors — Data.tagged fills _tag automatically
export const Value = {
  String:       Data.tagged<StringVal>("StringVal"),
  Int:          Data.tagged<IntVal>("IntVal"),
  Float:        Data.tagged<FloatVal>("FloatVal"),
  Bool:         Data.tagged<BoolVal>("BoolVal"),
  Null:         Data.tagged<NullVal>("NullVal"),
  Symbol:       Data.tagged<SymbolVal>("SymbolVal"),
  TaggedString: Data.tagged<TaggedStringVal>("TaggedStringVal"),
  Array:        Data.tagged<ArrayVal>("ArrayVal"),
  Map:          Data.tagged<MapVal>("MapVal"),
  Range:        Data.tagged<RangeVal>("RangeVal"),
  Measurement:  Data.tagged<MeasurementVal>("MeasurementVal"),
} as const
```

---

### StandardGraph

A graph constructed by classifying a collection of `Pattern<Subject>` objects.

| Field | Type | Notes |
|-------|------|-------|
| `nodes` | `Map<string, Pattern<Subject>>` | Atomic patterns (element count = 0). Keyed by identity. |
| `relationships` | `Map<string, {pattern, source, target}>` | Two-element patterns where both elements are nodes. |
| `annotations` | `Map<string, Pattern<Subject>>` | One-element patterns (decorators). |
| `walks` | `Map<string, Pattern<Subject>>` | Identity-chained sequences of relationships. |
| `other` | `Pattern<Subject>[]` | Patterns that don't match any class. |

**Classification rules** (element count determines class):
- `elements.length == 0` → **Node**
- `elements.length == 1` → **Annotation**
- `elements.length == 2` AND both elements are Nodes → **Relationship**
- `elements.length >= 1` AND all elements are Relationships forming a valid identity chain → **Walk**
- Otherwise → **Other**

**Walk validity**: A sequence of relationships is a valid walk if each consecutive pair shares a node identity (end-to-end chaining). See gram-hs `GraphClassifier.hs` for the exact predicate.

**TypeScript**: `fromGram` composes `Gram.parse` and `fromPatterns` entirely via `pipe`:

```typescript
static fromGram(input: string): Effect.Effect<StandardGraph, GramParseError> {
  return pipe(Gram.parse(input), Effect.map(StandardGraph.fromPatterns))
}
```

---

## JSON Interchange Format

The contract crossing the native extension boundary. The Rust gram-codec serializes to this format; the TypeScript/Python layer deserializes it. See `contracts/rust-codec-api.md` for the Rust function signatures.

```
RawPattern:
  subject: RawSubject        ← NOTE: key is "subject", not "value"
  elements: RawPattern[]

RawSubject:
  identity: string
  labels: string[]
  properties: Record<string, RawValue>

RawValue (discriminated):
  string          → native JSON string
  integer         → native JSON number (integer)
  decimal         → native JSON number (float)
  boolean         → native JSON boolean
  null            → native JSON null
  symbol          → { "type": "symbol", "value": string }
  tagged_string   → { "type": "tagged", "tag": string, "content": string }
  array           → native JSON array (recursive RawValue[])
  map             → native JSON object { key: RawValue } (no "type" discriminant)
  range           → { "type": "range", "lower"?: number, "upper"?: number }
  measurement     → { "type": "measurement", "unit": string, "value": number }
```

**Key invariant**: Native JSON primitives do not carry a `"type"` discriminant. Only `symbol`, `tagged_string`, `range`, and `measurement` use the `{ "type": ... }` envelope. `map` is a plain JSON object distinguishable by the absence of a `"type"` key.

### TypeScript Schema decode pipeline — `Schema.suspend`

The JSON payload is validated and decoded in one pass using `Schema`. The recursive `elements` field requires `Schema.suspend` to defer self-reference:

```typescript
import { Schema } from "effect"

// ValueSchema: Schema.Union of Schema.TaggedStruct — one branch per variant.
// Schema.TaggedStruct attaches a _tag literal and validates fields in one step.
export const ValueSchema: Schema.Schema<Value> = Schema.Union(
  Schema.TaggedStruct("StringVal",       { value: Schema.String }),
  Schema.TaggedStruct("IntVal",          { value: Schema.Number }),
  Schema.TaggedStruct("FloatVal",        { value: Schema.Number }),
  Schema.TaggedStruct("BoolVal",         { value: Schema.Boolean }),
  Schema.TaggedStruct("NullVal",         {}),
  Schema.TaggedStruct("SymbolVal",       { value: Schema.String }),
  Schema.TaggedStruct("TaggedStringVal", { tag: Schema.String, content: Schema.String }),
  Schema.TaggedStruct("ArrayVal",        { items: Schema.Array(Schema.suspend(() => ValueSchema)) }),
  Schema.TaggedStruct("MapVal",          { entries: Schema.Record({ key: Schema.String, value: Schema.suspend(() => ValueSchema) }) }),
  Schema.TaggedStruct("RangeVal",        { lower: Schema.optional(Schema.Number), upper: Schema.optional(Schema.Number) }),
  Schema.TaggedStruct("MeasurementVal",  { unit: Schema.String, value: Schema.Number }),
)

const RawSubjectSchema = Schema.Struct({
  identity:   Schema.String,
  labels:     Schema.Array(Schema.String),
  properties: Schema.Record({ key: Schema.String, value: ValueSchema }),
})

// Schema.suspend is required because RawPatternSchema references itself via elements
const RawPatternSchema: Schema.Schema<RawPattern> = Schema.Struct({
  subject:  RawSubjectSchema,
  elements: Schema.Array(Schema.suspend((): Schema.Schema<RawPattern> => RawPatternSchema)),
})

const decodePayload = Schema.decodeUnknownSync(Schema.Array(RawPatternSchema))
```

---

## Parse Pipelines

### TypeScript — `Effect` + `pipe`

Every step is a typed operation; errors are values, not exceptions:

```typescript
import { Effect, pipe } from "effect"

export const Gram = {
  parse: (input: string): Effect.Effect<ReadonlyArray<Pattern<Subject>>, GramParseError> =>
    pipe(
      // Step 1: call WASM codec (async, may fail with WASM load or codec error)
      Effect.tryPromise({
        try:   async () => { const wasm = await loadWasm(); return JSON.parse(wasm.gram_parse_to_json(input)) as unknown },
        catch: (cause) => new GramParseError({ input, cause }),
      }),
      // Step 2: validate entire JSON tree with Schema before constructing any Pattern
      Effect.flatMap((raw) =>
        Effect.try({
          try:   () => decodePayload(raw).map(patternFromRaw),
          catch: (cause) => new GramParseError({ input, cause }),
        })
      )
    ),
}

// Callers: use Effect.runPromise to convert to Promise if needed
const patterns = await Effect.runPromise(Gram.parse("(a)-->(b)"))

// Or compose further with pipe before running
const graph = await pipe(
  Gram.parse("(a)-->(b)"),
  Effect.map(StandardGraph.fromPatterns),
  Effect.runPromise
)
```

### Python — dataclasses + PyO3 codec

```python
# Step 1: call PyO3 codec (may raise GramParseError)
raw = gram_codec.gram_parse_to_json(input)   # returns JSON string
dicts = json.loads(raw)

# Step 2: build native Pattern[Subject] from plain dicts (pure, total)
patterns = [pattern_from_dict(d) for d in dicts]
```

### Classify pipeline (both languages)

```
Pattern<Subject>[]  →  StandardGraph.fromPatterns(patterns)  →  StandardGraph
```

This step is pure and total — it never fails.
