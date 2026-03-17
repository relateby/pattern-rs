# Contract: TypeScript Public API

**Feature**: 039-native-bindings
**Package**: `@relateby/pattern`
**Date**: 2026-03-17
**Design reference**: `proposals/migrate-ts-python-proposal.md` — TypeScript implementation sketch is normative.

The public API surface remains at the same import path. The internal implementation changes from WASM-backed opaque handles to native TypeScript built on the [Effect](https://effect.website) library.

---

## Effect-ts Architecture

The effect-ts library provides four things that directly shape this API:

1. **`Data.Class`** — `Pattern` and `Subject` extend `Data.Class`. This gives `Equal.equals(p1, p2)` structural equality for free, with no custom comparator. The previous WASM-handle approach made structural equality impossible.

2. **`Data.Case` + `Data.tagged`** — Each `Value` variant is a `Data.Case` interface with a `_tag` discriminant. `Data.tagged("StringVal")` creates a constructor that fills `_tag` automatically. TypeScript's `switch (v._tag)` exhaustiveness checking works across all 11 variants.

3. **`Schema`** — The JSON payload from the WASM codec is validated with `Schema.decodeUnknownSync` before any `Pattern` is constructed. `Schema.TaggedStruct` validates `Value` variants. `Schema.suspend` handles the self-referential `elements` array. Invalid codec output surfaces as a `GramParseError`, not a crash mid-tree.

4. **`Effect` + `Option`** — `Gram.parse` returns `Effect<..., GramParseError>` instead of `Promise`. Errors are in the type signature. `findFirst` returns `Option.Option<V>` instead of `V | undefined`. Both compose with `pipe`.

---

## Imports

```typescript
// Public package import — unchanged
import { Pattern, Subject, Value, StandardGraph, Gram, GramParseError } from "@relateby/pattern"

// Pipeable operations — importable separately for tree-shaking
import { map, fold, filter, findFirst, extend, extract, duplicate } from "@relateby/pattern"

// effect-ts types used in signatures (consumers must have effect installed)
import { Data, Effect, Equal, Option, Schema, pipe } from "effect"
```

---

## Value

Defined in `src/value.ts`. Each variant is a `Data.Case` interface; `Data.tagged()` creates the constructor.

```typescript
// Each variant interface — _tag is the discriminant
export interface StringVal       extends Data.Case { readonly _tag: "StringVal";       readonly value: string }
export interface IntVal          extends Data.Case { readonly _tag: "IntVal";          readonly value: number }
export interface FloatVal        extends Data.Case { readonly _tag: "FloatVal";        readonly value: number }
export interface BoolVal         extends Data.Case { readonly _tag: "BoolVal";         readonly value: boolean }
export interface NullVal         extends Data.Case { readonly _tag: "NullVal" }
export interface SymbolVal       extends Data.Case { readonly _tag: "SymbolVal";       readonly value: string }
export interface TaggedStringVal extends Data.Case { readonly _tag: "TaggedStringVal"; readonly tag: string; readonly content: string }
export interface ArrayVal        extends Data.Case { readonly _tag: "ArrayVal";        readonly items: ReadonlyArray<Value> }
export interface MapVal          extends Data.Case { readonly _tag: "MapVal";          readonly entries: ReadonlyMap<string, Value> }
export interface RangeVal        extends Data.Case { readonly _tag: "RangeVal";        readonly lower?: number; readonly upper?: number }
export interface MeasurementVal  extends Data.Case { readonly _tag: "MeasurementVal";  readonly unit: string; readonly value: number }

export type Value =
  StringVal | IntVal | FloatVal | BoolVal | NullVal | SymbolVal |
  TaggedStringVal | ArrayVal | MapVal | RangeVal | MeasurementVal

// Constructor namespace — Data.tagged fills _tag automatically
export const Value: {
  String:       (args: { value: string })                          => StringVal
  Int:          (args: { value: number })                          => IntVal
  Float:        (args: { value: number })                          => FloatVal
  Bool:         (args: { value: boolean })                         => BoolVal
  Null:         (args?: {})                                        => NullVal
  Symbol:       (args: { value: string })                          => SymbolVal
  TaggedString: (args: { tag: string; content: string })           => TaggedStringVal
  Array:        (args: { items: ReadonlyArray<Value> })            => ArrayVal
  Map:          (args: { entries: ReadonlyMap<string, Value> })    => MapVal
  Range:        (args: { lower?: number; upper?: number })         => RangeVal
  Measurement:  (args: { unit: string; value: number })            => MeasurementVal
}

// Schema for decoding the WASM JSON payload (used internally in schema.ts)
// Shown here so implementers understand what Schema.TaggedStruct produces:
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
```

---

## Subject

Defined in `src/subject.ts`. Extends `Data.Class` for structural equality.

```typescript
export class Subject extends Data.Class<{
  readonly identity:   string
  readonly labels:     ReadonlySet<string>
  readonly properties: ReadonlyMap<string, Value>
}> {
  static fromId(identity: string): Subject
  withLabel(label: string): Subject         // returns new instance
  withProperty(name: string, value: Value): Subject  // returns new instance
}

// Equal.equals works on Subject:
// Equal.equals(Subject.fromId("a").withLabel("Person"), Subject.fromId("a").withLabel("Person")) → true
```

---

## Pattern\<V\>

Defined in `src/pattern.ts`. Extends `Data.Class` for structural equality.

```typescript
export class Pattern<V> extends Data.Class<{
  readonly value:    V
  readonly elements: ReadonlyArray<Pattern<V>>
}> {
  static point<V>(value: V): Pattern<V>    // atomic (no elements)
  static of<V>(value: V): Pattern<V>       // alias for point

  readonly isAtomic: boolean
  readonly length:   number    // elements.length
  readonly size:     number    // 1 + sum of all descendant sizes
  readonly depth:    number    // 0 if atomic; 1 + max(child depths) otherwise
}

// Equal.equals works on Pattern<V> when V also implements Equal:
// Equal.equals(Pattern.point(Subject.fromId("a")), Pattern.point(Subject.fromId("a"))) → true
```

---

## Pipeable Operations

Defined in `src/ops.ts`. All operations are **standalone curried functions**, not methods. This enables point-free composition with `pipe` and allows individual operations to be imported and tree-shaken independently.

```typescript
import { pipe } from "effect"
import { map, fold, filter, findFirst, extend, extract, duplicate } from "@relateby/pattern"

// map — transforms every value, preserves structure (functor)
export declare const map: <V, U>(fn: (v: V) => U) => (p: Pattern<V>) => Pattern<U>

// fold — accumulates values, pre-order traversal (root first)
export declare const fold: <V, R>(init: R, fn: (acc: R, v: V) => R) => (p: Pattern<V>) => R

// filter — collects matching subtrees (pre-order)
export declare const filter: <V>(pred: (p: Pattern<V>) => boolean) => (p: Pattern<V>) => ReadonlyArray<Pattern<V>>

// findFirst — first matching value; Option.none if absent
export declare const findFirst: <V>(pred: (v: V) => boolean) => (p: Pattern<V>) => Option.Option<V>

// extend — context-aware map; fn sees the full subtree at each position (comonad)
export declare const extend: <V, U>(fn: (p: Pattern<V>) => U) => (p: Pattern<V>) => Pattern<U>

// extract — root value (comonad)
export declare const extract: <V>(p: Pattern<V>) => V

// duplicate — pattern of subtrees (comonad; enables extend composition)
export declare const duplicate: <V>(p: Pattern<V>) => Pattern<Pattern<V>>

// --- Usage examples ---

// Collect all identities in a pattern tree:
const ids = pipe(somePattern, fold([] as string[], (acc, s) => [...acc, s.identity]))

// Find the first Person node:
const maybePerson = pipe(somePattern, findFirst(s => s.labels.has("Person")))
// → Option.Option<Subject> — use Option.getOrUndefined or Option.match to extract

// Annotate every node with its depth (using extend):
const withDepth = pipe(somePattern, extend(p => ({ subject: p.value, depth: p.depth })))
```

---

## Schema Decode Pipeline

Defined in `src/schema.ts`. Internal to the library — not part of the public API — but shown here because it is the boundary where effect-ts `Schema` is central to correctness:

```typescript
// Schema.suspend is required for the self-referential elements array.
// The entire tree is decoded and validated in one pass before any Pattern is constructed.
const RawPatternSchema: Schema.Schema<RawPattern> = Schema.Struct({
  subject:  RawSubjectSchema,
  elements: Schema.Array(Schema.suspend((): Schema.Schema<RawPattern> => RawPatternSchema)),
})

const decodePayload = Schema.decodeUnknownSync(Schema.Array(RawPatternSchema))

// If the WASM codec returns malformed JSON, Schema throws here — caught by Effect.try
// in gram.ts and surfaced as a GramParseError with the original input attached.
```

---

## Gram

Defined in `src/gram.ts`. Returns `Effect` — not `Promise`. The entire pipeline (WASM call + JSON parse + Schema decode + tree construction) is expressed as a `pipe` of `Effect` operations.

```typescript
// Error type — Data.TaggedError gives it a _tag and structured fields
export class GramParseError extends Data.TaggedError("GramParseError")<{
  readonly input: string
  readonly cause: unknown
}> {}

export const Gram: {
  // Returns Effect<..., GramParseError> — errors are typed, not thrown
  parse(input: string):     Effect.Effect<ReadonlyArray<Pattern<Subject>>, GramParseError>
  stringify(patterns: ReadonlyArray<Pattern<Subject>>): Effect.Effect<string, GramParseError>
  validate(input: string):  Effect.Effect<void, GramParseError>
}

// --- Implementation shape (in gram.ts) ---
const parse = (input: string): Effect.Effect<ReadonlyArray<Pattern<Subject>>, GramParseError> =>
  pipe(
    Effect.tryPromise({
      try:   async () => { const wasm = await loadWasm(); return JSON.parse(wasm.gram_parse_to_json(input)) as unknown },
      catch: (cause) => new GramParseError({ input, cause }),
    }),
    Effect.flatMap((raw) =>
      Effect.try({
        try:   () => decodePayload(raw).map(patternFromRaw),
        catch: (cause) => new GramParseError({ input, cause }),
      })
    )
  )

// --- Consumer usage patterns ---

// Convert to Promise (interop with async/await code):
const patterns = await Effect.runPromise(Gram.parse("(a)-->(b)"))

// Compose with more Effect operations before running:
const graph = await pipe(
  Gram.parse("(a:Person {name:'Alice'})-->(b:Person {name:'Bob'})"),
  Effect.map(StandardGraph.fromPatterns),
  Effect.runPromise
)

// Handle errors explicitly (without try/catch):
const result = await pipe(
  Gram.parse(userInput),
  Effect.match({
    onFailure: (err) => ({ ok: false, error: String(err.cause) }),
    onSuccess: (patterns) => ({ ok: true, patterns }),
  }),
  Effect.runPromise
)
```

---

## StandardGraph

Defined in `src/standard-graph.ts`. A pure native TypeScript class; no WASM involvement after the initial parse.

```typescript
export class StandardGraph {
  // Pure synchronous construction from already-parsed patterns
  static fromPatterns(patterns: ReadonlyArray<Pattern<Subject>>): StandardGraph

  // Convenience: parse + classify in one Effect
  static fromGram(input: string): Effect.Effect<StandardGraph, GramParseError>
  // Implementation: pipe(Gram.parse(input), Effect.map(StandardGraph.fromPatterns))

  readonly nodeCount:         number
  readonly relationshipCount: number
  readonly annotationCount:   number
  readonly walkCount:         number

  nodes():         IterableIterator<[string, Pattern<Subject>]>
  relationships(): IterableIterator<[string, { pattern: Pattern<Subject>; source: string; target: string }]>
  annotations():   IterableIterator<[string, Pattern<Subject>]>
  walks():         IterableIterator<[string, Pattern<Subject>]>
  other():         ReadonlyArray<Pattern<Subject>>

  // Option instead of T | undefined — absence is explicit
  node(id: string):         Option.Option<Pattern<Subject>>
  relationship(id: string): Option.Option<{ pattern: Pattern<Subject>; source: string; target: string }>
}
```

---

## Breaking Changes from Current API

| Current | New | Migration |
|---------|-----|-----------|
| `Gram.parse()` returns `Promise<NativePattern[]>` | returns `Effect<Pattern<Subject>[], GramParseError>` | Wrap with `Effect.runPromise()` to get a `Promise` |
| `Pattern` is an opaque WASM handle | `Pattern` extends `Data.Class` — fields directly readable | `instanceof Pattern` now works; no `.getValue()` accessor needed |
| `Equal.equals(p1, p2)` not available | Works structurally via `Data.Class` | No change needed for `===` users; `Equal.equals` is additive |
| `Gram.parseOne()` exists | Removed | Use `pipe(Gram.parse(s), Effect.map(ps => ps[0]))` |
| `findFirst()` returns `V \| undefined` | Returns `Option.Option<V>` | Use `Option.getOrUndefined(result)` for backward compat |
| Pattern operations (fold, map) cross WASM boundary | Run entirely in TypeScript | No API change; significant performance improvement |
| Parse errors thrown as exceptions | `GramParseError` returned as `Effect` failure | Replace `try/catch` with `Effect.match` or `Effect.catchAll` |
