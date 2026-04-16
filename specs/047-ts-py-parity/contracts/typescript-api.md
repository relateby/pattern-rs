# TypeScript API Contract: Pattern Parity Operations

**Package**: `@relateby/pattern`
**Module**: `typescript/packages/pattern/src/`
**Export path**: All new operations exported from main index

---

## New Static Constructors on `Pattern<V>`

These are added to `pattern.ts` as static class methods.

```typescript
class Pattern<V> {
  // existing constructors ...

  /** Create a non-atomic pattern with explicit children. */
  static pattern<V>(value: V, elements: ReadonlyArray<Pattern<V>>): Pattern<V>

  /** Create a pattern whose children are atomic patterns over a list of values. */
  static fromList<V>(value: V, values: ReadonlyArray<V>): Pattern<V>
}
```

**Export**: Re-export via existing `export { Pattern } from "./pattern.js"` — no index.ts change needed.

---

## New Operations in `ops.ts`

All follow the two-level curried arrow function pattern: `const op = <V, ...>(config) => (p: Pattern<V>): R => ...`

### Predicates

```typescript
/** Returns true if any value in the tree satisfies the predicate. Short-circuits on first match. */
export const anyValue: <V>(pred: (v: V) => boolean) => (p: Pattern<V>) => boolean

/** Returns true if all values in the tree satisfy the predicate. Short-circuits on first failure. */
export const allValues: <V>(pred: (v: V) => boolean) => (p: Pattern<V>) => boolean

/** Returns true when two patterns have the same structure and pairwise-equal values. */
export const matches: <V>(a: Pattern<V>, b: Pattern<V>) => boolean

/** Returns true when `needle` appears as a sub-pattern anywhere in `haystack`. */
export const contains: <V>(needle: Pattern<V>) => (haystack: Pattern<V>) => boolean
```

### Transformations

```typescript
/**
 * Paramorphism: structure-aware bottom-up fold.
 * `f` receives the current sub-pattern and the list of pre-computed results from its children.
 */
export const para: <V, R>(f: (p: Pattern<V>, childResults: readonly R[]) => R) => (p: Pattern<V>) => R

/**
 * Anamorphism: expand a seed value into a Pattern<V> tree.
 * `expand` returns the value for the current node and seeds for its children.
 * Terminates when expand returns an empty children array.
 */
export const unfold: <A, V>(expand: (seed: A) => readonly [V, ReadonlyArray<A>]) => (seed: A) => Pattern<V>

/**
 * Semigroup combination: combine two patterns.
 * `combineValues` determines how root values are merged; elements are concatenated.
 * The combineValues function SHOULD be associative.
 */
export const combine: <V>(combineValues: (a: V, b: V) => V) => (a: Pattern<V>) => (b: Pattern<V>) => Pattern<V>
```

### Comonad Helpers

```typescript
/** Annotate each position with the depth of its subtree (leaf = 0). */
export const depthAt: <V>(p: Pattern<V>) => Pattern<number>

/** Annotate each position with the total number of nodes in its subtree. */
export const sizeAt: <V>(p: Pattern<V>) => Pattern<number>

/** Annotate each position with the 0-based index path from root to that position. */
export const indicesAt: <V>(p: Pattern<V>) => Pattern<number[]>
```

---

## Updated `index.ts` Exports

Add to the existing ops re-export line:

```typescript
export {
  map, fold, filter, findFirst, extend, extract, duplicate, values,
  // NEW:
  anyValue, allValues, matches, contains,
  para, unfold, combine,
  depthAt, sizeAt, indicesAt,
} from "./ops.js"
```

---

## Pipe Composition Examples

```typescript
import { pipe } from "effect"
import { Pattern, anyValue, allValues, contains, para, unfold, combine, depthAt } from "@relateby/pattern"

// anyValue / allValues
const hasLabel = anyValue<Subject>(s => s.labels.has("Person"))
const allLabeled = allValues<Subject>(s => s.labels.size > 0)

// contains (curried for pipe)
const containsNode = contains(Pattern.point(mySubject))
pipe(myPattern, containsNode)  // => boolean

// para: compute tree height
const height = para<Subject, number>(
  (_p, childHeights) => childHeights.length === 0 ? 0 : 1 + Math.max(...childHeights)
)

// unfold: build a binary tree of depth n
const binaryTree = unfold<number, number>(n => [n, n <= 0 ? [] : [n-1, n-1]])
pipe(3, binaryTree)  // Pattern tree with 15 nodes

// combine: merge two patterns (String values → concatenation)
const merged = pipe(
  patternA,
  combine((a: string, b: string) => a + b)(patternB)
)

// depthAt
const withDepths = depthAt(myPattern)  // Pattern<number>
```

---

## Breaking Changes

None. All additions are new exports; no existing API is modified.

---

## Compatibility

All new operations are pure functions operating on `Pattern<V>` in memory. No WASM dependency. Compatible with the existing `@relateby/pattern` Effect.ts integration.
