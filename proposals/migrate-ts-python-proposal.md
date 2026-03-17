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
│  @relateby/pattern (TypeScript)                  │
│                                                  │
│  Pattern<V>       ← pure TypeScript class        │
│  Subject          ← pure TypeScript class        │
│  Value            ← TypeScript discriminated union│
│  StandardGraph    ← pure TypeScript class        │
│  graph algorithms ← pure TypeScript functions    │
│                                                  │
│  Gram.parse(s)  ──► WASM: gram_codec             │
│                 ◄── plain JSON array             │
│                  → constructs native Pattern[]   │
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

```typescript
// Pattern<V>: ~80 lines, pure TypeScript
export class Pattern<V> {
  constructor(
    readonly value: V,
    readonly elements: readonly Pattern<V>[] = []
  ) {}

  static point<V>(value: V): Pattern<V> {
    return new Pattern(value);
  }

  static of<V>(value: V): Pattern<V> {
    return Pattern.point(value);
  }

  get isAtomic(): boolean { return this.elements.length === 0; }
  get length(): number { return this.elements.length; }
  get size(): number { return 1 + this.elements.reduce((n, e) => n + e.size, 0); }
  get depth(): number {
    if (this.isAtomic) return 0;
    return 1 + Math.max(...this.elements.map(e => e.depth));
  }

  map<U>(fn: (v: V) => U): Pattern<U> {
    return new Pattern(fn(this.value), this.elements.map(e => e.map(fn)));
  }

  fold<R>(init: R, fn: (acc: R, v: V) => R): R {
    return this.elements.reduce(
      (acc, e) => e.fold(acc, fn),
      fn(init, this.value)
    );
  }

  filter(predicate: (p: Pattern<V>) => boolean): Pattern<V>[] {
    const results: Pattern<V>[] = predicate(this) ? [this] : [];
    return results.concat(this.elements.flatMap(e => e.filter(predicate)));
  }

  // ... values(), findFirst(), matches(), contains(), extend(), extract()
}

// Subject: ~40 lines, pure TypeScript
export class Subject {
  constructor(
    readonly identity: string,
    readonly labels: ReadonlySet<string> = new Set(),
    readonly properties: ReadonlyMap<string, Value> = new Map()
  ) {}

  static fromId(identity: string): Subject {
    return new Subject(identity);
  }

  withLabel(label: string): Subject {
    return new Subject(this.identity, new Set([...this.labels, label]), this.properties);
  }

  withProperty(name: string, value: Value): Subject {
    return new Subject(this.identity, this.labels, new Map([...this.properties, [name, value]]));
  }
}

// Value: ~30 lines, discriminated union
export type Value =
  | { type: 'string'; value: string }
  | { type: 'integer'; value: number }
  | { type: 'float'; value: number }
  | { type: 'boolean'; value: boolean }
  | { type: 'null' }
  | { type: 'symbol'; value: string };

export const Value = {
  string: (s: string): Value => ({ type: 'string', value: s }),
  integer: (n: number): Value => ({ type: 'integer', value: n }),
  float: (n: number): Value => ({ type: 'float', value: n }),
  boolean: (b: boolean): Value => ({ type: 'boolean', value: b }),
  null: (): Value => ({ type: 'null' }),
  symbol: (s: string): Value => ({ type: 'symbol', value: s }),
};

// Gram: WASM call returns JSON, TypeScript builds native Pattern<Subject>
export const Gram = {
  async parse(input: string): Promise<Pattern<Subject>[]> {
    const wasm = await loadWasm();
    const raw: RawPattern[] = JSON.parse(wasm.gram_parse(input));
    return raw.map(patternFromRaw);
  },
  async stringify(patterns: Pattern<Subject>[]): Promise<string> {
    const wasm = await loadWasm();
    return wasm.gram_stringify(JSON.stringify(patterns.map(patternToRaw)));
  },
};
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
| **TypeScript types** | Real TypeScript generics — `Pattern<Subject>` as a true generic class, not an opaque WASM handle; `instanceof` checks work naturally |
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
| JSON serialization at WASM boundary | Gram parse output crosses boundary as JSON string | Cost is one JSON parse per `Gram.parse()` call, not per operation; acceptable for a codec |
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

### Phase 3: Implement native TypeScript Pattern/Subject/Value

Write `Pattern<V>`, `Subject`, `Value` as pure TypeScript. Port operations from the current WASM wrapper source. Validate against the existing test suite by running both the old (WASM) and new (native) implementations against the same inputs.

### Phase 4: Implement native Python Pattern/Subject/Value

Same as Phase 3 for Python. Use `dataclasses` and standard library types. Validate against existing pytest suite.

### Phase 5: Implement native StandardGraph

Port `StandardGraph` to TypeScript and Python. The classification logic (node vs. relationship vs. walk vs. annotation detection) is the core of `GraphClassifier` — straightforward to port.

### Phase 6: Cut over and remove Rust types from bindings

Switch `Gram.parse()` to use the new JSON path and construct native Pattern. Remove `WasmPattern`, `WasmSubject`, `WasmValue`, `WasmStandardGraph` from the WASM surface. Delete `convert.rs`. Shrink `python.rs` to codec-only.

---

## Decision Points

Before committing to this migration, the following questions should be answered:

1. **Is the JSON boundary acceptable for stringify?** If round-trip performance matters (frequent parse-modify-stringify cycles), the JSON boundary adds cost. Profile before deciding.

2. **Is Pattern expected to grow significantly in complexity?** If future plans include complex unification, constraint solving, or pattern matching algorithms that benefit from Rust, keeping Pattern in Rust has more value. If Pattern stays a simple recursive container, native is better.

3. **Is graph algorithm parity (TypeScript vs Python) a priority?** If yes, native Python Pattern enables porting algorithms without PyO3; if not, the motivation is reduced.

4. **Is WASM binary size a concern?** If the library targets browser environments where initial load matters, reducing WASM size has real user impact. If it's server-only, the argument weakens.
