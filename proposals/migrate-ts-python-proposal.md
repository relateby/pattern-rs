# Proposal: Migrate Pattern/Subject to Native TypeScript and Python

**Date**: 2026-03-17
**Status**: Draft
**Scope**: `crates/pattern-wasm/`, `crates/pattern-core/src/python.rs`, `typescript/@relateby/pattern/`, `python/relateby/`

---

## Summary

The Rust gram codec is the genuine value in this library — a correct, high-performance nom-based parser for gram notation. The `Pattern<Subject>` data structure and its operations are simple recursive algorithms that carry significant FFI overhead when kept in Rust. This proposal recommends making the gram codec the sole component exposed via WASM/PyO3, and reimplementing `Pattern`, `Subject`, `Value`, `StandardGraph`, and all operations as idiomatic native TypeScript and Python.

---

## Evaluation of the Current Architecture

### What each binding exposes today

Both the TypeScript/WASM and Python/PyO3 bindings expose nearly identical surfaces:

| Component | WASM | Python | Notes |
|-----------|------|--------|-------|
| `Pattern<V>` | `WasmPattern` → `NativePattern` | `Pattern` (PyO3) | Thin wrapper; delegates all ops to Rust |
| `Subject` | `WasmSubject` → `NativeSubject` | `Subject` (PyO3) | Thin wrapper |
| `Value` | Opaque `JsValue` | Opaque PyO3 object | Factory + extractor methods only |
| `StandardGraph` | `WasmStandardGraph` → `StandardGraph` | `StandardGraph` (PyO3) | + Python-level `from_gram` bridge |
| `ValidationRules` | Wrapper | Wrapper | All logic in Rust |
| `StructureAnalysis` | Read-only accessors | Read-only accessors | All logic in Rust |
| **Gram codec** | Async, WASM lazy-loaded | Sync, direct PyO3 | The actual hard work |
| Graph algorithms | Exposed (bfs, dfs, etc.) | **Not exposed** | TypeScript-only API gap |

### The FFI tax

**TypeScript/WASM:** The gram parse pipeline is:
```
gram-codec → Vec<Pattern<Subject>> → convert.rs → WasmPattern/WasmSubject → NativePattern wrappers
```
`convert.rs` exists solely to shuttle Rust types across the WASM boundary. Once in TypeScript, any Pattern operation — `fold()`, `map()`, `filter()` — crosses the WASM boundary again. Gram is async not because parsing is async, but because WASM lazy-loads on first access.

**Python/PyO3:** A `Pattern.fold(fn)` call means: PyO3 dispatch → Rust → call back into Python for `fn` → convert return value — repeated for every node in the tree. The GIL is held across every step. This is substantially slower than native Python recursion for the simple tree traversal that fold performs.

### The symptom: `convert.rs`

The existence of `crates/pattern-wasm/src/convert.rs` is the most diagnostic symptom. It is pure translation work — converting Rust types into WASM-compatible representations and back. It handles `WasmSubject` instance detection via `__wbg_ptr`, value subtype detection from JS objects, and bidirectional pattern tree conversion. None of this work produces correctness benefits; it exists because the wrong types are crossing the boundary.

### What genuinely belongs in Rust

The gram codec (`gram-codec` crate) is a non-trivial nom-based PEG parser. It handles:
- Complex gram notation syntax with multiple element types
- Backtracking and error recovery
- Bidirectional serialization (parse and stringify)
- Correct handling of identity, labels, properties, and graph structure

This is hard to implement correctly, benefits from Rust's performance, and changes infrequently. It earns its place in Rust.

### What doesn't belong in Rust (for the bindings)

`Pattern<V>` is approximately:
```rust
enum Pattern<V> {
    Empty,
    Pair { value: V, elements: Vec<Pattern<V>> },
}
```

The operations on it — `map`, `fold`, `filter`, `find_first`, `matches`, `contains` — are basic recursive algorithms over this structure. `Subject` is a struct with three fields. `Value` is a tagged union of primitives. `StandardGraph` is a `HashMap<String, Pattern<Subject>>` partitioned by element class.

None of these benefit from Rust's performance or safety guarantees in a meaningful way at the binding layer. The operations don't do compute-intensive work; they do tree traversal with user-supplied callbacks, which negates any Rust performance advantage the moment a callback crosses the FFI boundary.

---

## Proposal A: Thin Codec Boundary, Native Pattern

Make the WASM/PyO3 surface the gram codec only. Implement `Pattern`, `Subject`, `Value`, `StandardGraph`, and all operations natively in TypeScript and Python.

### Principle

> The WASM/PyO3 boundary is for things that are hard to write correctly or need native performance. The gram parser is both. A recursive data structure and tree operations are neither.

### New architecture

```
┌─────────────────────────────────────────────────┐
│  @relateby/pattern (TypeScript + effect)         │
│                                                  │
│  Pattern<V>       ← Data.Class (structural eq)  │
│  Subject          ← Data.Class (structural eq)  │
│  Value            ← Data.Case tagged union       │
│  StandardGraph    ← pure TypeScript class        │
│  graph algorithms ← pipeable functions           │
│                                                  │
│  Gram.parse(s)  ──► WASM: gram_codec             │
│                 ◄── plain JSON array             │
│        Schema.decodeUnknownSync validates tree   │
│        → Effect<Pattern<Subject>[], GramParseError>│
└─────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────┐
│  relateby (Python)                               │
│                                                  │
│  Pattern[V]       ← Python dataclass             │
│  Subject          ← Python dataclass             │
│  Value            ← Python dataclass (tagged)    │
│  StandardGraph    ← Python class                 │
│                                                  │
│  parse_gram(s)  ──► PyO3: gram_codec             │
│                 ◄── list of dicts                │
│                  → constructs native Pattern[]   │
└─────────────────────────────────────────────────┘
```

### WASM surface after migration

The WASM module shrinks to:

```typescript
// wasm surface: gram codec only
export function gram_parse(input: string): string;    // returns JSON
export function gram_stringify(data: string): string; // takes JSON
export function gram_validate(input: string): string; // returns errors JSON
```

`convert.rs` is deleted. `crates/pattern-wasm` no longer depends on `pattern-core` for types — only for `gram-codec`.

### PyO3 surface after migration

```python
# PyO3 surface: gram codec only
def gram_parse(input: str) -> list[dict]: ...      # returns plain dicts
def gram_stringify(data: list[dict]) -> str: ...   # takes plain dicts
def gram_validate(input: str) -> list[str]: ...    # returns error strings
```

`python.rs` shrinks from ~1800 lines to ~50.

### TypeScript implementation sketch (effect-ts style)

This implementation uses [Effect](https://effect.website) for typed errors and async operations, `Data.Class` for structural equality on immutable data, `Schema` for decoding the JSON payload from the WASM codec boundary, and `Option` for partial lookups.

```typescript
import { Data, Effect, Equal, Option, Schema, pipe } from "effect"

// ─── Value: tagged variants with structural equality ─────────────────────────
//
// Each variant is a Data.Case so Equal.equals() works structurally.
// Data.tagged() fills _tag automatically; no manual bookkeeping.

export interface StringVal extends Data.Case { readonly _tag: "StringVal"; readonly value: string }
export interface IntVal    extends Data.Case { readonly _tag: "IntVal";    readonly value: number }
export interface FloatVal  extends Data.Case { readonly _tag: "FloatVal";  readonly value: number }
export interface BoolVal   extends Data.Case { readonly _tag: "BoolVal";   readonly value: boolean }
export interface NullVal   extends Data.Case { readonly _tag: "NullVal" }
export interface SymbolVal extends Data.Case { readonly _tag: "SymbolVal"; readonly value: string }

export type Value = StringVal | IntVal | FloatVal | BoolVal | NullVal | SymbolVal

// Constructors mirror the Rust Value enum variants.
export const Value = {
  String:  Data.tagged<StringVal>("StringVal"),
  Int:     Data.tagged<IntVal>("IntVal"),
  Float:   Data.tagged<FloatVal>("FloatVal"),
  Bool:    Data.tagged<BoolVal>("BoolVal"),
  Null:    Data.tagged<NullVal>("NullVal"),
  Symbol:  Data.tagged<SymbolVal>("SymbolVal"),
} as const

// Schema for decoding the WASM JSON output → typed Value.
// Schema.TaggedStruct attaches a _tag literal and validates fields.
export const ValueSchema: Schema.Schema<Value> = Schema.Union(
  Schema.TaggedStruct("StringVal", { value: Schema.String }),
  Schema.TaggedStruct("IntVal",    { value: Schema.Number }),
  Schema.TaggedStruct("FloatVal",  { value: Schema.Number }),
  Schema.TaggedStruct("BoolVal",   { value: Schema.Boolean }),
  Schema.TaggedStruct("NullVal",   {}),
  Schema.TaggedStruct("SymbolVal", { value: Schema.String }),
)

// ─── Subject ─────────────────────────────────────────────────────────────────
//
// Data.Class provides structural equality (Equal/Hash) automatically.
// Immutable builder methods return new instances.

export class Subject extends Data.Class<{
  readonly identity: string
  readonly labels:    ReadonlySet<string>
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

// ─── Pattern<V>: recursive tree with structural equality ─────────────────────
//
// Data.Class gives deep structural equality via Equal.equals(p1, p2).
// Operations are standalone pipeable functions rather than methods,
// enabling point-free composition with pipe().

export class Pattern<V> extends Data.Class<{
  readonly value:    V
  readonly elements: ReadonlyArray<Pattern<V>>
}> {
  static point<V>(value: V): Pattern<V> {
    return new Pattern({ value, elements: [] })
  }

  static of<V>(value: V): Pattern<V> {
    return Pattern.point(value)
  }

  get isAtomic(): boolean { return this.elements.length === 0 }
  get length():   number  { return this.elements.length }
  get size():     number  { return 1 + this.elements.reduce((n, e) => n + e.size, 0) }
  get depth():    number  {
    return this.isAtomic ? 0 : 1 + Math.max(...this.elements.map(e => e.depth))
  }
}

// Pipeable operations — compose with pipe(pattern, map(fn), ...).
// Returning Option rather than undefined makes absence explicit.

export const map = <V, U>(fn: (v: V) => U) =>
  (p: Pattern<V>): Pattern<U> =>
    new Pattern({
      value:    fn(p.value),
      elements: p.elements.map(map(fn))
    })

export const fold = <V, R>(init: R, fn: (acc: R, v: V) => R) =>
  (p: Pattern<V>): R =>
    p.elements.reduce(
      (acc, e) => pipe(e, fold(acc, fn)),
      fn(init, p.value)
    )

export const filter = <V>(predicate: (p: Pattern<V>) => boolean) =>
  (p: Pattern<V>): ReadonlyArray<Pattern<V>> => [
    ...(predicate(p) ? [p] : []),
    ...p.elements.flatMap(e => pipe(e, filter(predicate)))
  ]

export const findFirst = <V>(predicate: (v: V) => boolean) =>
  (p: Pattern<V>): Option.Option<V> =>
    predicate(p.value)
      ? Option.some(p.value)
      : p.elements.reduce(
          (found: Option.Option<V>, e) =>
            Option.orElse(found, () => pipe(e, findFirst(predicate))),
          Option.none()
        )

// ─── Schema for Pattern<Subject> (decodes WASM JSON payload) ─────────────────
//
// Schema.suspend() handles the self-referential elements array.
// Decoding validates the entire tree in one pass before any Pattern
// objects are constructed.

interface RawSubject {
  readonly identity:   string
  readonly labels:     ReadonlyArray<string>
  readonly properties: Readonly<Record<string, Value>>
}

interface RawPattern {
  readonly value:    RawSubject
  readonly elements: ReadonlyArray<RawPattern>
}

const RawSubjectSchema = Schema.Struct({
  identity:   Schema.String,
  labels:     Schema.Array(Schema.String),
  properties: Schema.Record({ key: Schema.String, value: ValueSchema }),
})

const RawPatternSchema: Schema.Schema<RawPattern> = Schema.Struct({
  value:    RawSubjectSchema,
  elements: Schema.Array(Schema.suspend((): Schema.Schema<RawPattern> => RawPatternSchema)),
})

const decodePayload = Schema.decodeUnknownSync(Schema.Array(RawPatternSchema))

function patternFromRaw(raw: RawPattern): Pattern<Subject> {
  return new Pattern({
    value: new Subject({
      identity:   raw.value.identity,
      labels:     new Set(raw.value.labels),
      properties: new Map(Object.entries(raw.value.properties)),
    }),
    elements: raw.elements.map(patternFromRaw),
  })
}

// ─── Errors ───────────────────────────────────────────────────────────────────

export class GramParseError extends Data.TaggedError("GramParseError")<{
  readonly input: string
  readonly cause: unknown
}> {}

// ─── Gram: WASM codec wrapped in Effect ──────────────────────────────────────
//
// Callers get typed errors (GramParseError) instead of thrown exceptions.
// Effect.runPromise() converts to Promise when interoperating with async code.

export const Gram = {
  parse: (input: string): Effect.Effect<ReadonlyArray<Pattern<Subject>>, GramParseError> =>
    pipe(
      Effect.tryPromise({
        try:   async () => { const wasm = await loadWasm(); return JSON.parse(wasm.gram_parse(input)) as unknown },
        catch: (cause) => new GramParseError({ input, cause }),
      }),
      Effect.flatMap((raw) =>
        Effect.try({
          try:   () => decodePayload(raw).map(patternFromRaw),
          catch: (cause) => new GramParseError({ input, cause }),
        })
      )
    ),

  stringify: (patterns: ReadonlyArray<Pattern<Subject>>): Effect.Effect<string, GramParseError> =>
    Effect.tryPromise({
      try:   async () => { const wasm = await loadWasm(); return wasm.gram_stringify(JSON.stringify(patterns.map(patternToRaw))) },
      catch: (cause) => new GramParseError({ input: "(stringify)", cause }),
    }),
}

// Usage: pipe/Effect compose naturally
//
//   const graph = await pipe(
//     Gram.parse("(a)-->(b)"),
//     Effect.map(patterns => StandardGraph.fromPatterns(patterns)),
//     Effect.runPromise
//   )
//
//   const ids = pipe(
//     somePattern,
//     fold([] as string[], (acc, s) => [...acc, s.identity])
//   )
//
//   const maybeNode = pipe(somePattern, findFirst(s => s.identity === "a"))
//   // → Option.some(Subject) | Option.none()
```

### Python implementation sketch

```python
from __future__ import annotations
from dataclasses import dataclass, field
from typing import Callable, Generic, TypeVar

V = TypeVar('V')
U = TypeVar('U')
R = TypeVar('R')

@dataclass
class Pattern(Generic[V]):
    value: V
    elements: list['Pattern[V]'] = field(default_factory=list)

    @classmethod
    def point(cls, value: V) -> 'Pattern[V]':
        return cls(value=value)

    @classmethod
    def of(cls, value: V) -> 'Pattern[V]':
        return cls.point(value)

    @property
    def is_atomic(self) -> bool:
        return len(self.elements) == 0

    @property
    def depth(self) -> int:
        if self.is_atomic:
            return 0
        return 1 + max(e.depth for e in self.elements)

    @property
    def size(self) -> int:
        return 1 + sum(e.size for e in self.elements)

    def map(self, fn: Callable[[V], U]) -> 'Pattern[U]':
        return Pattern(fn(self.value), [e.map(fn) for e in self.elements])

    def fold(self, init: R, fn: Callable[[R, V], R]) -> R:
        acc = fn(init, self.value)
        for e in self.elements:
            acc = e.fold(acc, fn)
        return acc

    def filter(self, predicate: Callable[['Pattern[V]'], bool]) -> list['Pattern[V]']:
        results = [self] if predicate(self) else []
        for e in self.elements:
            results.extend(e.filter(predicate))
        return results

    # ... find_first(), values(), matches(), contains(), extend(), extract()

@dataclass
class Subject:
    identity: str
    labels: set[str] = field(default_factory=set)
    properties: dict[str, 'Value'] = field(default_factory=dict)

    @classmethod
    def from_id(cls, identity: str) -> 'Subject':
        return cls(identity=identity)

# Value as a dataclass hierarchy
@dataclass
class Value:
    pass

@dataclass
class StringValue(Value):
    value: str

@dataclass
class IntValue(Value):
    value: int

# ... or as a simple tagged dict for maximum interoperability

# Gram: PyO3 returns plain dicts, Python builds native Pattern[Subject]
def parse(input: str) -> list[Pattern[Subject]]:
    from relateby._native import gram_codec
    raw = gram_codec.parse_to_dicts(input)   # returns list[dict]
    return [_pattern_from_dict(r) for r in raw]
```

### StandardGraph as native TypeScript/Python

`StandardGraph` becomes a straightforward native class that classifies `Pattern<Subject>` by the element type detected in the gram codec output. No WASM calls needed for graph construction or traversal.

```typescript
export class StandardGraph {
  private _nodes = new Map<string, Pattern<Subject>>();
  private _relationships = new Map<string, { pattern: Pattern<Subject>; source: string; target: string }>();
  private _walks = new Map<string, Pattern<Subject>>();
  private _annotations = new Map<string, Pattern<Subject>>();

  static fromGram(input: string): Promise<StandardGraph> {
    return Gram.parse(input).then(patterns => {
      const g = new StandardGraph();
      patterns.forEach(p => g.addPattern(p));
      return g;
    });
  }

  addPattern(pattern: Pattern<Subject>): this {
    // classification logic: inspect pattern structure
    // same rules as current Rust GraphClassifier
    return this;
  }

  get nodeCount(): number { return this._nodes.size; }
  nodes(): IterableIterator<[string, Pattern<Subject>]> { return this._nodes.entries(); }
  // ...
}
```

---

## Benefits

| Benefit | Detail |
|---------|--------|
| **WASM binary size** | Significant reduction: `pattern-core` excluded from WASM build; only `gram-codec` compiled to WASM |
| **Sync Pattern ops** | `fold`, `map`, `filter` no longer cross the WASM boundary; run at native JS/Python speed |
| **Python performance** | Eliminates GIL-held PyO3 round-trips for each fold/map step; native Python recursion is faster for this workload |
| **TypeScript types** | Real TypeScript generics — `Pattern<Subject>` as a true generic class, not an opaque WASM handle; `instanceof` and `Equal.equals()` work naturally |
| **Structural equality** | `Data.Class`/`Data.Case` gives deep structural equality on `Pattern`, `Subject`, and `Value` via `Equal.equals()` — no hand-written comparators |
| **Typed errors** | `Gram.parse` returns `Effect<..., GramParseError>` — callers handle failures in the type system, not at runtime with try/catch |
| **Pipeable operations** | `map`, `fold`, `filter`, `findFirst` are standalone curried functions; compose with `pipe()` for point-free style |
| **Option for partial results** | `findFirst` returns `Option<V>` instead of `V \| undefined` — absence is explicit and composable |
| **Schema validation** | `Schema.decodeUnknownSync` validates the entire WASM JSON payload before any `Pattern` is constructed — bad codec output surfaces as a typed error, not a runtime crash mid-tree |
| **`convert.rs` deleted** | The most complex part of the WASM binding layer disappears; no bidirectional type conversion needed |
| **`python.rs` shrinks ~97%** | From ~1800 lines to ~50 lines of gram codec exposure |
| **Graph algorithms in Python** | With Pattern as native Python, graph algorithms can be written directly in Python (or use networkx) — closing the TypeScript/Python API gap |
| **Type stubs generated** | Python stubs derived from actual type annotations, not hand-maintained `.pyi` files |
| **Testability** | Pattern and Subject are plain values — no mocking, no WASM init, no async setup in unit tests |
| **Debuggability** | Native TypeScript objects show up correctly in DevTools; native Python objects in debuggers; no opaque WASM handles |

---

## Tradeoffs

| Tradeoff | Impact | Mitigation |
|----------|--------|------------|
| Two implementations of Pattern | Rust (authoritative) + TypeScript + Python; behavior must stay in sync | Pattern semantics are simple and stable; property-based tests against gram-hs reference cover correctness |
| JSON serialization at WASM boundary | Gram parse output crosses boundary as JSON string | Cost is one JSON parse + one `Schema.decodeUnknownSync` per `Gram.parse()` call, not per operation; acceptable for a codec |
| effect dependency | `effect` is a 70 kB (minified+gzip) peer dependency; its runtime is non-trivial | Only the core `Data`/`Schema`/`Effect`/`Option` modules are used; tree-shaking keeps bundle impact modest for browser targets |
| Migration effort | Large diff: TypeScript and Python binding layers substantially rewritten | Can be staged: gram codec boundary first, then remove Rust types from bindings |
| Gram stringify round-trip | To stringify a native Pattern, must serialize to JSON, pass to WASM, deserialize in Rust | Same cost as parse; stringify is less common than parse |

---

## What stays in Rust

After this migration, the WASM and PyO3 bindings expose only:

- `gram_parse(input: string) → JSON`: Parse gram notation, return plain JSON
- `gram_stringify(data: JSON) → string`: Serialize plain data to gram notation
- `gram_validate(input: string) → JSON`: Validate gram notation, return errors

The `gram-codec` crate remains unchanged. `pattern-core` is still a Rust library used for native Rust applications, but is no longer compiled into the WASM or Python extension targets.

---

## Migration Path

### Phase 1: Define the JSON interchange format

Specify the JSON schema that the gram codec returns. This schema becomes the contract between Rust and host languages. Both TypeScript and Python reconstruct `Pattern<Subject>` from this format.

### Phase 2: Add `gram_parse_to_json` in Rust

Add a thin Rust function that parses gram and returns a JSON string (using `serde_json`). Keep existing WASM bindings working in parallel.

### Phase 3: Cross-check semantics against pattern-hs

Before writing TypeScript or Python code, review the authoritative Haskell source in `../pattern-hs/libs/` to confirm:

- **`fold` traversal order** — the sketched implementation is pre-order (value before elements); verify this matches the Haskell `fold`/`foldMap` behavior. Wrong order breaks every accumulator built on `fold`.
- **`extend` and `extract`** (comonad ops) — `extend`'s "annotate every subtree" semantics are non-obvious; take them from the Haskell source directly.
- **`matches` and `contains`** — confirm whether matching is structural or identity-based, and whether it is full-subtree or partial.
- **`Value` variant completeness** — verify the six proposed variants (`StringVal`, `IntVal`, `FloatVal`, `BoolVal`, `NullVal`, `SymbolVal`) cover everything gram-hs uses. Locking the `Schema.Union` prematurely would require a breaking change to add a variant later.
- **StandardGraph classification rules** — the exact rules for node / relationship / walk / annotation detection should come from `../pattern-hs/libs/` rather than being inferred from the Rust port.

The Haskell tests in `../pattern-hs/libs/*/tests/` also define the operation *laws* (functor laws for `map`, catamorphism laws for `fold`, comonad laws for `extend`/`extract`). These should be ported as property-based tests using `fast-check` alongside the implementation.

### Phase 4: Implement native TypeScript Pattern/Subject/Value

Write `Pattern<V>`, `Subject`, `Value` as TypeScript using the effect-ts style described above (`Data.Class`, `Data.Case`, `Schema`, `Effect`, `Option`). Validate against the existing test suite by running both the old (WASM) and new (native) implementations against the same inputs. Use `fast-check` to verify operation laws from Phase 3.

### Phase 5: Implement native Python Pattern/Subject/Value

Same as Phase 4 for Python. Use `dataclasses` and standard library types. Validate against existing pytest suite.

### Phase 6: Implement native StandardGraph

Port `StandardGraph` to TypeScript and Python using the classification rules confirmed in Phase 3. The logic (node vs. relationship vs. walk vs. annotation detection) is the core of `GraphClassifier`.

### Phase 7: Cut over and remove Rust types from bindings

Switch `Gram.parse()` to use the new JSON path and construct native Pattern. Remove `WasmPattern`, `WasmSubject`, `WasmValue`, `WasmStandardGraph` from the WASM surface. Delete `convert.rs`. Shrink `python.rs` to codec-only.

---

## Decision Points

Before committing to this migration, the following questions should be answered:

1. **Is the JSON boundary acceptable for stringify?** If round-trip performance matters (frequent parse-modify-stringify cycles), the JSON boundary adds cost. Profile before deciding.

2. **Is Pattern expected to grow significantly in complexity?** If future plans include complex unification, constraint solving, or pattern matching algorithms that benefit from Rust, keeping Pattern in Rust has more value. If Pattern stays a simple recursive container, native is better.

3. **Is graph algorithm parity (TypeScript vs Python) a priority?** If yes, native Python Pattern enables porting algorithms without PyO3; if not, the motivation is reduced.

4. **Is WASM binary size a concern?** If the library targets browser environments where initial load matters, reducing WASM size has real user impact. If it's server-only, the argument weakens.
