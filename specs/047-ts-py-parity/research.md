# Research: TypeScript and Python Pattern API Parity

**Phase**: 0 — Research
**Feature**: 047-ts-py-parity
**Date**: 2026-04-16

---

## Decision 1: `duplicate` already exists in TypeScript and Python

**Decision**: `duplicate` is NOT a gap. It already exists in both TypeScript (`ops.ts`) and Python (`_pattern.py`). The original parity proposal was incorrect on this point. Only Rust is missing `duplicate`.

**Rationale**: Direct inspection of source files confirmed:
- TypeScript: `export const duplicate = <V>(p: Pattern<V>): Pattern<Pattern<V>> => new Pattern({ value: p, elements: Data.array(p.elements.map(duplicate)) })`
- Python: `def duplicate(self) -> "Pattern[Pattern[V]]"` (class method)

**Alternatives considered**: N/A — this is a factual correction.

---

## Decision 2: `combine` API — pass combiner function, not Combinable protocol

**Decision**: Implement `combine` as a curried function that takes an explicit value-combination function as its first argument, rather than requiring a `Combinable<V>` interface or protocol.

**Rationale**: 
- TypeScript has no type classes; an interface constraint requires callers to pass an interface-conforming object, which is more verbose.
- Python has no existing `Combinable` protocol in the codebase and adding one for a single operation would be premature.
- The functional approach `combine(combineValues)(a, b)` is idiomatic in the TypeScript curried function style already used by `fold`, `map`, etc.
- The Haskell Semigroup instance implicitly selects the combine operation via type; passing it explicitly is the idiomatic equivalent in TypeScript/Python.

**Haskell semantics**: `(Pattern v1 es1) <> (Pattern v2 es2) = Pattern (v1 <> v2) (es1 ++ es2)`

**TypeScript API**:
```typescript
export const combine =
  <V>(combineValues: (a: V, b: V) => V) =>
  (a: Pattern<V>) =>
  (b: Pattern<V>): Pattern<V> =>
    new Pattern({ value: combineValues(a.value, b.value), elements: Data.array([...a.elements, ...b.elements]) })
```

**Python API** (class method, takes both patterns plus combiner):
```python
def combine(self, other: "Pattern[V]", combine_values: Callable[[V, V], V]) -> "Pattern[V]":
    return Pattern(value=combine_values(self.value, other.value), elements=self.elements + other.elements)
```

**Alternatives considered**:
- `Combinable<V>` interface: more complex API, requires callers to wrap values.
- Overloaded `+` operator in Python: breaks convention, unexpected semantics.

---

## Decision 3: `Pattern.pattern()` and `Pattern.fromList()` as static constructors

**Decision**: Add as static class methods on `Pattern` in both TypeScript and Python (not as standalone functions in `ops.ts`).

**Rationale**: `point()` and `of()` are already static methods on the class. Constructors logically belong on the class, not in ops. This mirrors the Haskell pattern where constructors (`point`, `pattern`, `fromList`) are exported from the module, not from a separate ops module.

**TypeScript**:
```typescript
static pattern<V>(value: V, elements: ReadonlyArray<Pattern<V>>): Pattern<V> {
  return new Pattern({ value, elements: Data.array(elements) })
}

static fromList<V>(value: V, values: ReadonlyArray<V>): Pattern<V> {
  return Pattern.pattern(value, values.map(Pattern.point))
}
```

**Python**:
```python
@classmethod
def pattern(cls, value: V, elements: list["Pattern[V]"]) -> "Pattern[V]":
    return cls(value=value, elements=elements)

@classmethod
def from_list(cls, value: V, values: list[V]) -> "Pattern[V]":
    return cls.pattern(value, [cls.point(v) for v in values])
```

**Alternatives considered**: Standalone functions in ops.ts — rejected because constructors belong on the class.

---

## Decision 4: `unfold` as standalone function in `ops.ts` / module-level in Python

**Decision**: `unfold` is not a constructor (it takes a seed type, not a value of `V`), so it goes in `ops.ts` for TypeScript and as a module-level function in `_pattern.py` for Python.

**Haskell semantics**: `unfold :: (a -> (v, [a])) -> a -> Pattern v`

**TypeScript**:
```typescript
export const unfold =
  <A, V>(expand: (seed: A) => readonly [V, ReadonlyArray<A>]) =>
  (seed: A): Pattern<V> => {
    const [value, childSeeds] = expand(seed)
    return new Pattern({ value, elements: Data.array(childSeeds.map(unfold(expand))) })
  }
```

**Python** (class method for discoverability, takes seed and expand fn):
```python
@classmethod
def unfold(cls, expand: Callable[[A], tuple[V, list[A]]], seed: A) -> "Pattern[V]":
    value, child_seeds = expand(seed)
    return cls(value=value, elements=[cls.unfold(expand, s) for s in child_seeds])
```

**Alternatives considered**: Static method on Pattern class — acceptable but slightly inconsistent with Haskell where it is a standalone function.

---

## Decision 5: `para` follows the same curried pattern as `fold`

**Decision**: `para` is a standalone curried function in `ops.ts` for TypeScript and a class method in Python, consistent with `fold`.

**Haskell semantics**: `para :: (Pattern v -> [r] -> r) -> Pattern v -> r` — fold function receives both the current pattern AND the list of pre-computed child results.

**TypeScript**:
```typescript
export const para =
  <V, R>(f: (p: Pattern<V>, childResults: readonly R[]) => R) =>
  (p: Pattern<V>): R =>
    f(p, p.elements.map(para(f)))
```

**Python**:
```python
def para(self, f: Callable[["Pattern[V]", list[R]], R]) -> R:
    return f(self, [e.para(f) for e in self.elements])
```

**Alternatives considered**: Accepting value and sub-patterns separately — rejected; the Haskell API passes the entire sub-pattern, which is more powerful.

---

## Decision 6: Comonad helpers are `extend` specializations

**Decision**: `depth_at`, `size_at`, `indices_at` (Python) / `depthAt`, `sizeAt`, `indicesAt` (TypeScript) are standalone functions (TypeScript ops.ts) and class methods (Python) implemented via `extend`.

**Haskell**: `depthAt = extend depth`

**Implementation pattern (TypeScript)**:
```typescript
export const depthAt = <V>(p: Pattern<V>): Pattern<number> =>
  extend((sub: Pattern<V>) => sub.depth)(p)

export const sizeAt = <V>(p: Pattern<V>): Pattern<number> =>
  extend((sub: Pattern<V>) => sub.size)(p)

export const indicesAt = <V>(p: Pattern<V>): Pattern<number[]> => {
  const go = (indices: number[]) => (sub: Pattern<V>): Pattern<number[]> =>
    new Pattern({ value: indices, elements: Data.array(sub.elements.map((e, i) => go([...indices, i])(e))) })
  return go([])(p)
}
```

Note: `indicesAt` cannot be expressed as a simple `extend` because the path indices depend on position within the parent, not just the subtree. It requires a separate recursive helper.

**Python pattern**:
```python
def depth_at(self) -> "Pattern[int]":
    return self.extend(lambda sub: sub.depth)

def size_at(self) -> "Pattern[int]":
    return self.extend(lambda sub: sub.size)

def indices_at(self) -> "Pattern[list[int]]":
    def go(pat: "Pattern[V]", path: list[int]) -> "Pattern[list[int]]":
        return Pattern(
            value=path,
            elements=[go(e, path + [i]) for i, e in enumerate(pat.elements)]
        )
    return go(self, [])
```

---

## Decision 7: Python graph transforms go in a new `_graph_transforms.py` module

**Decision**: Graph transform functions (`map_graph`, `map_all_graph`, `filter_graph`, `fold_graph`, `map_with_context`, `para_graph`) are implemented as standalone functions in a new file `_graph_transforms.py`, operating on `StandardGraph` objects. They are exported from `relateby.pattern`.

**Rationale**:
- `StandardGraph` handles classification and query; transforms are a separate concern.
- TypeScript has the same separation (`transforms.ts` is separate from the graph class).
- Standalone functions avoid growing `StandardGraph` into a God class.

**Alternatives considered**: Methods on `StandardGraph` — rejected for separation of concerns.

---

## Decision 8: `anyValue` / `allValues` are standalone functions in TypeScript, class methods in Python

**Haskell**: `anyValue :: (v -> Bool) -> Pattern v -> Bool` — uses Foldable's `any`.

**Decision**: Follow existing patterns — standalone curried functions in TypeScript `ops.ts`, class methods in Python `_pattern.py`.

**TypeScript**:
```typescript
export const anyValue =
  <V>(pred: (v: V) => boolean) =>
  (p: Pattern<V>): boolean => {
    if (pred(p.value)) return true
    return p.elements.some(anyValue(pred))
  }

export const allValues =
  <V>(pred: (v: V) => boolean) =>
  (p: Pattern<V>): boolean => {
    if (!pred(p.value)) return false
    return p.elements.every(allValues(pred))
  }
```

**Python**:
```python
def any_value(self, predicate: Callable[[V], bool]) -> bool:
    if predicate(self.value):
        return True
    return any(e.any_value(predicate) for e in self.elements)

def all_values(self, predicate: Callable[[V], bool]) -> bool:
    if not predicate(self.value):
        return False
    return all(e.all_values(predicate) for e in self.elements)
```

---

## Decision 9: `matches` uses existing equality, `contains` is recursive

**Haskell**: `matches = (==)`, `contains haystack needle = matches haystack needle || any (contains needle) (elements haystack)`

**TypeScript** — `Pattern` uses `Data.Class` for structural equality, so `Equal.equals(a, b)` works:
```typescript
export const matches = <V>(a: Pattern<V>, b: Pattern<V>): boolean =>
  Equal.equals(a, b)

export const contains = <V>(needle: Pattern<V>) => (haystack: Pattern<V>): boolean =>
  Equal.equals(haystack, needle) || haystack.elements.some(contains(needle))
```

**Python** — `@dataclass` gives structural `__eq__`:
```python
def matches(self, other: "Pattern[V]") -> bool:
    return self == other

def contains(self, needle: "Pattern[V]") -> bool:
    return self == needle or any(e.contains(needle) for e in self.elements)
```

---

## Correction to Parity Proposal

The `proposals/language-parity-proposal.md` incorrectly listed `duplicate` as missing from TypeScript and Python. Both already have it. The corrected gap list:

**TypeScript missing** (from Haskell reference):
`anyValue`, `allValues`, `matches`, `contains`, `para`, `unfold`, `fromList` constructor, `Pattern.pattern()` constructor, `depthAt`, `sizeAt`, `indicesAt`, `combine`

**Python missing** (from Haskell reference, same as TypeScript plus):
`any_value`, `all_values`, `matches`, `contains`, `para`, `unfold`, `from_list` classmethod, `pattern()` classmethod, `depth_at`, `size_at`, `indices_at`, `combine`, plus all graph transforms

**Rust missing**: `duplicate` (trivial), Subject property helpers
