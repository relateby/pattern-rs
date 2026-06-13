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
│  @relateby/pattern (zero-dependency TypeScript)  │
│                                                  │
│  Pattern<V>       ← plain class (structural eq) │
│  Subject          ← plain class (identity eq)   │
│  Value            ← tagged union literals        │
│  StandardGraph    ← pure TypeScript class        │
│  graph algorithms ← pipeable functions           │
│  pipe / Option    ← inline (Effect-shape compat)│
│                                                  │
│  Gram.parse(s)  ──► WASM: gram_codec             │
│                 ◄── plain JSON array             │
│        hand-written validator checks tree        │
│        → Promise<Pattern<Subject>[]>             │
└─────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────┐
│  @relateby/pattern-effect (Effect adapter)       │
│                                                  │
│  Gram.parse / stringify / validate               │
│    → Effect<T, GramParseError>                   │
│    (wraps Promise via Effect.tryPromise)         │
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

### TypeScript implementation sketch

`@relateby/pattern` is zero-dependency. `pipe` and `Option<T>` are provided inline with the same tagged-union shape as Effect's versions, so `@relateby/pattern-effect` can bridge without type conversion.

```typescript
// No effect import — inline fp utilities only
import { Option, pipe } from "./fp.js"

// ─── Value: tagged union literals ────────────────────────────────────────────

export interface StringVal { readonly _tag: "StringVal"; readonly value: string }
export interface IntVal    { readonly _tag: "IntVal";    readonly value: number }
export interface FloatVal  { readonly _tag: "FloatVal";  readonly value: number }
export interface BoolVal   { readonly _tag: "BoolVal";   readonly value: boolean }
export interface NullVal   { readonly _tag: "NullVal" }
export interface SymbolVal { readonly _tag: "SymbolVal"; readonly value: string }

export type Value = StringVal | IntVal | FloatVal | BoolVal | NullVal | SymbolVal

export const Value = {
  String:  (args: Omit<StringVal, "_tag">): StringVal => ({ _tag: "StringVal", ...args }),
  Int:     (args: Omit<IntVal,    "_tag">): IntVal    => ({ _tag: "IntVal",    ...args }),
  Float:   (args: Omit<FloatVal,  "_tag">): FloatVal  => ({ _tag: "FloatVal",  ...args }),
  Bool:    (args: Omit<BoolVal,   "_tag">): BoolVal   => ({ _tag: "BoolVal",   ...args }),
  Null:    (): NullVal                                 => ({ _tag: "NullVal" }),
  Symbol:  (args: Omit<SymbolVal, "_tag">): SymbolVal => ({ _tag: "SymbolVal", ...args }),
} as const

// ─── Subject: identity-based equality ────────────────────────────────────────
//
// Two subjects are equal iff their identity strings match.
// Serial #N identities from gram parsing are deterministic within a file.
// Cross-file subject comparison is a reconciliation concern, not equality.

export class Subject {
  readonly identity: string
  private readonly _labels: ReadonlyArray<string>
  private readonly _properties: Readonly<Record<string, Value>>

  constructor(identity: string, labels: ReadonlyArray<string>, properties: Readonly<Record<string, Value>>) {
    this.identity = identity
    this._labels = labels
    this._properties = properties
  }

  static fromId(identity: string): Subject { return new Subject(identity, [], {}) }
  static from({ identity, labels, properties }: SubjectLike): Subject {
    return new Subject(identity, [...labels], { ...properties })
  }

  get labels(): ReadonlyArray<string> { return this._labels }
  get properties(): Readonly<Record<string, Value>> { return this._properties }

  equals(other: Subject): boolean { return this.identity === other.identity }

  withLabel(label: string): Subject {
    return new Subject(this.identity, [...this._labels, label], this._properties)
  }
  withProperty(name: string, value: Value): Subject {
    return new Subject(this.identity, this._labels, { ...this._properties, [name]: value })
  }
  merge(other: Subject): Subject {
    const merged = new Set([...this._labels, ...other._labels])
    return new Subject(this.identity, [...merged], { ...other._properties, ...this._properties })
  }
}

// ─── Pattern<V>: recursive decorated sequence ────────────────────────────────

export class Pattern<V> {
  readonly value: V
  readonly elements: ReadonlyArray<Pattern<V>>

  constructor({ value, elements }: { value: V; elements: ReadonlyArray<Pattern<V>> }) {
    this.value = value
    this.elements = elements
  }

  static point<V>(value: V): Pattern<V> { return new Pattern({ value, elements: [] }) }
  static of<V>(value: V): Pattern<V> { return Pattern.point(value) }

  get isAtomic(): boolean { return this.elements.length === 0 }
  get length():   number  { return this.elements.length }
  get size():     number  { return 1 + this.elements.reduce((n, e) => n + e.size, 0) }
  get depth():    number  {
    return this.isAtomic ? 0 : 1 + Math.max(...this.elements.map(e => e.depth))
  }
}

// ─── Errors ───────────────────────────────────────────────────────────────────

export class GramParseError extends Error {
  readonly _tag = "GramParseError" as const
  readonly input: string
  readonly cause: unknown
  constructor({ input, cause }: { input: string; cause: unknown }) {
    super(cause instanceof Error ? cause.message : String(cause))
    this.name = "GramParseError"
    this.input = input
    this.cause = cause
  }
}

// ─── Gram: WASM codec, Promise-based ─────────────────────────────────────────
//
// Returns Promise<T>, rejects with GramParseError.
// Effect users get Effect<T, GramParseError> via @relateby/pattern-effect.

export const Gram = {
  async parse(input: string): Promise<ReadonlyArray<Pattern<Subject>>> {
    try {
      const wasm = await loadWasm()
      const raw = wasm.parse(input)
      return validatePayload(raw).map(patternFromRaw)
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input, cause })
    }
  },

  async stringify(patterns: ReadonlyArray<Pattern<Subject>>): Promise<string> {
    try {
      const wasm = await loadWasm()
      return wasm.stringify(patterns.map(patternToRaw))
    } catch (cause) {
      throw new GramParseError({ input: "(stringify)", cause })
    }
  },
}

// Usage: async/await
//
//   const patterns = await Gram.parse("(a)-->(b)")
//   const graph = StandardGraph.fromPatterns(patterns)
//
// Effect users via @relateby/pattern-effect:
//
//   import { Gram } from "@relateby/pattern-effect"
//   const graph = yield* pipe(
//     Gram.parse("(a)-->(b)"),
//     Effect.map(StandardGraph.fromPatterns)
//   )
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
| **TypeScript types** | Real TypeScript generics — `Pattern<Subject>` as a true generic class, not an opaque WASM handle; `instanceof` and `matches()` work naturally |
| **Identity equality** | `Subject` equality is identity-based (same `identity` string = same subject); `Pattern`/`Value` equality is structural via `matches()` |
| **Typed errors** | `Gram.parse` rejects with `GramParseError`; Effect users get `Effect<T, GramParseError>` from `@relateby/pattern-effect` |
| **Pipeable operations** | `map`, `fold`, `filter`, `findFirst` are standalone curried functions; compose with inline `pipe()` for point-free style |
| **Option for partial results** | `findFirst` returns `Option<V>` (same tagged-union shape as Effect's `Option`) — no conversion needed at the Effect adapter boundary |
| **Hand-written validator** | `validatePayload()` checks the WASM JSON payload before any `Pattern` is constructed — bad codec output surfaces as a `GramParseError`, not a runtime crash mid-tree |
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
| Effect adapter split | Effect users import `Gram` from `@relateby/pattern-effect` instead of `@relateby/pattern` | The adapter is a thin `Effect.tryPromise` wrapper; types are identical; the split is explicit rather than hidden |
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

Write `Pattern<V>`, `Subject`, `Value` as plain TypeScript (no Effect dependency): plain classes, tagged-union factory functions, inline `pipe`/`Option` utilities. Validate against the existing test suite by running both the old (WASM) and new (native) implementations against the same inputs. Use `fast-check` to verify operation laws from Phase 3. Effect users get `Effect<T, GramParseError>` wrappers via `@relateby/pattern-effect`.

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
