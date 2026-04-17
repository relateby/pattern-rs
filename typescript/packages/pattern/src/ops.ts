// ops.ts — Standalone curried pipeable operations on Pattern<V>
//
// All operations are pure standalone functions (not methods) so they compose
// with effect-ts pipe() and can be tree-shaken independently.

import { Data, Equal, Option, pipe } from "effect"
import { Pattern } from "./pattern.js"

// --- Functor ---

/**
 * Transform every value in a pattern, preserving its structure.
 *
 * Visits this pattern's value first, then each element's values in order (pre-order).
 *
 * @typeParam V - Value type of the input pattern
 * @typeParam U - Value type of the output pattern
 * @param fn - Mapping function applied to each value
 * @returns A curried function that accepts a `Pattern<V>` and returns a `Pattern<U>`
 *
 * @example
 * ```typescript
 * const doubled = pipe(Pattern.fromList(1, [2, 3]), map(n => n * 2))
 * // Pattern<number> with value 2 and elements [4, 6]
 * ```
 */
export const map =
  <V, U>(fn: (v: V) => U) =>
  (p: Pattern<V>): Pattern<U> =>
    new Pattern({ value: fn(p.value), elements: Data.array(p.elements.map(map(fn))) })

// --- Foldable ---

/**
 * Accumulate values across a pattern, visiting each value before its elements (pre-order).
 *
 * @typeParam V - Value type of the pattern
 * @typeParam R - Accumulator / result type
 * @param init - Initial accumulator value
 * @param fn - Reducer applied to each value in order
 * @returns A curried function that accepts a `Pattern<V>` and returns `R`
 *
 * @example
 * ```typescript
 * const sum = pipe(Pattern.fromList(1, [2, 3, 4]), fold(0, (acc, n) => acc + n))
 * // 10
 * ```
 */
export const fold =
  <V, R>(init: R, fn: (acc: R, v: V) => R) =>
  (p: Pattern<V>): R =>
    p.elements.reduce((acc, e) => pipe(e, fold(acc, fn)), fn(init, p.value))

/**
 * Collect all sub-patterns that satisfy a predicate, in pre-order.
 *
 * @typeParam V - Value type of the pattern
 * @param pred - Predicate tested against each sub-pattern
 * @returns A curried function that accepts a `Pattern<V>` and returns matching sub-patterns
 */
export const filter =
  <V>(pred: (p: Pattern<V>) => boolean) =>
  (p: Pattern<V>): ReadonlyArray<Pattern<V>> => {
    const results: Array<Pattern<V>> = []
    if (pred(p)) results.push(p)
    for (const e of p.elements) {
      for (const r of filter(pred)(e)) results.push(r)
    }
    return results
  }

/**
 * Return the first value satisfying the predicate, or `Option.none()`. Short-circuits pre-order.
 *
 * @typeParam V - Value type of the pattern
 * @param pred - Predicate tested against each raw value
 * @returns A curried function that accepts a `Pattern<V>` and returns `Option<V>`
 */
export const findFirst =
  <V>(pred: (v: V) => boolean) =>
  (p: Pattern<V>): Option.Option<V> => {
    if (pred(p.value)) return Option.some(p.value)
    return p.elements.reduce(
      (found: Option.Option<V>, e) => Option.orElse(found, () => pipe(e, findFirst(pred))),
      Option.none()
    )
  }

// --- Comonad ---

/**
 * Context-aware map: `fn` receives the full sub-pattern at each position rather than
 * just its value. Useful for annotating each position with structural information.
 *
 * @typeParam V - Value type of the input pattern
 * @typeParam U - Value type of the output pattern
 * @param fn - Function from a sub-pattern to a new value; called at every position
 * @returns A curried function that accepts a `Pattern<V>` and returns a `Pattern<U>`
 *
 * @example
 * ```typescript
 * // Annotate every position with its composition size
 * const sized = pipe(myPattern, extend(sub => sub.size))
 * ```
 */
export const extend =
  <V, U>(fn: (p: Pattern<V>) => U) =>
  (p: Pattern<V>): Pattern<U> =>
    new Pattern({ value: fn(p), elements: Data.array(p.elements.map(extend(fn))) })

/** Extract the root value. */
export const extract = <V>(p: Pattern<V>): V => p.value

/** Replace each position's value with its own sub-pattern, enabling `extend` composition. */
export const duplicate = <V>(p: Pattern<V>): Pattern<Pattern<V>> =>
  new Pattern({ value: p, elements: Data.array(p.elements.map(duplicate)) })

// --- Extra utility ---

// --- Structural predicates ---

/**
 * Return `true` if any value satisfies the predicate. Short-circuits pre-order.
 *
 * @typeParam V - Value type of the pattern
 * @param pred - Predicate to test against each node value
 * @returns A curried function that accepts a `Pattern<V>` and returns `boolean`
 */
export const anyValue =
  <V>(pred: (v: V) => boolean) =>
  (p: Pattern<V>): boolean => {
    if (pred(p.value)) return true
    return p.elements.some(anyValue(pred))
  }

/**
 * Return `true` if every value satisfies the predicate. Short-circuits pre-order.
 *
 * @typeParam V - Value type of the pattern
 * @param pred - Predicate to test against each node value
 * @returns A curried function that accepts a `Pattern<V>` and returns `boolean`
 */
export const allValues =
  <V>(pred: (v: V) => boolean) =>
  (p: Pattern<V>): boolean => {
    if (!pred(p.value)) return false
    return p.elements.every(allValues(pred))
  }

/**
 * Structural equality — `true` if both patterns have the same shape and values.
 *
 * @typeParam V - Value type (must be structurally comparable via `effect` `Equal`)
 * @param a - First pattern
 * @param b - Second pattern
 * @returns `true` when `a` and `b` are structurally equal
 */
export const matches = <V>(a: Pattern<V>, b: Pattern<V>): boolean =>
  Equal.equals(a, b)

/**
 * Return `true` if `needle` appears anywhere inside `haystack` (including at the root).
 *
 * @typeParam V - Value type of the pattern
 * @param needle - The subtree to search for
 * @returns A curried function that accepts a `haystack` `Pattern<V>` and returns `boolean`
 */
export const contains =
  <V>(needle: Pattern<V>) =>
  (haystack: Pattern<V>): boolean =>
    Equal.equals(haystack, needle) || haystack.elements.some(contains(needle))

// --- Paramorphism ---

/**
 * Structure-aware fold: `f` receives both the current sub-pattern and the
 * pre-computed results for its elements. Processing is bottom-up (elements before
 * the pattern containing them).
 *
 * Unlike `fold`, which only sees raw values, `para` exposes the full sub-pattern at
 * each position, making it suitable for computations that need structural context
 * alongside accumulated results.
 *
 * @typeParam V - Value type of the pattern
 * @typeParam R - Result type produced at each node
 * @param f - Algebra receiving the current node and its children's results
 * @returns A curried function that accepts a `Pattern<V>` and returns `R`
 *
 * @example
 * ```typescript
 * // Build a nested summary where each node knows its depth contribution
 * const depthSummary = pipe(
 *   myPattern,
 *   para((node, childResults) => ({
 *     value: node.value,
 *     childCount: node.elements.length,
 *     subtotalNodes: childResults.reduce((s, r) => s + r.subtotalNodes, 1),
 *   }))
 * )
 * ```
 */
export const para =
  <V, R>(f: (p: Pattern<V>, childResults: ReadonlyArray<R>) => R) =>
  (p: Pattern<V>): R =>
    f(p, p.elements.map(para(f)))

// --- Semigroup combination ---

/**
 * Combine two patterns by merging their root values and concatenating their element lists.
 *
 * The resulting pattern's root value is `combineValues(a.value, b.value)`.
 * All elements from `a` appear before all elements from `b` in the combined children.
 *
 * @typeParam V - Value type of all three patterns
 * @param combineValues - Binary operation used to merge the two root values
 * @returns A curried `(a: Pattern<V>) => (b: Pattern<V>) => Pattern<V>` function
 *
 * @example
 * ```typescript
 * const merged = pipe(
 *   Pattern.point("hello"),
 *   combine((a, b) => `${a} ${b}`)(Pattern.point("world"))
 * )
 * // Pattern with root "hello world" and no children
 * ```
 */
export const combine =
  <V>(combineValues: (a: V, b: V) => V) =>
  (a: Pattern<V>) =>
  (b: Pattern<V>): Pattern<V> =>
    new Pattern({ value: combineValues(a.value, b.value), elements: Data.array([...a.elements, ...b.elements]) })

// --- Anamorphism ---

/**
 * Expand a seed value into a `Pattern<V>` tree (anamorphism / unfold).
 *
 * Recursively applies `expand` to the seed and to every child seed it produces,
 * terminating naturally when `expand` returns an empty children array.
 *
 * @typeParam A - Seed type used to drive expansion
 * @typeParam V - Value type of the produced pattern
 * @param expand - Function from a seed to a `[value, childSeeds]` pair
 * @returns A curried function that accepts an initial seed and returns `Pattern<V>`
 *
 * @example
 * ```typescript
 * // Build a binary tree counting down from n
 * const countdown = pipe(
 *   3,
 *   unfold((n: number) => [n, n > 0 ? [n - 1, n - 1] : []])
 * )
 * // Pattern<number> rooted at 3 with two subtrees rooted at 2, etc.
 * ```
 */
export const unfold =
  <A, V>(expand: (seed: A) => readonly [V, ReadonlyArray<A>]) =>
  (seed: A): Pattern<V> => {
    const [value, childSeeds] = expand(seed)
    const inner = unfold<A, V>(expand)
    return new Pattern<V>({ value, elements: Data.array([...childSeeds].map(inner)) })
  }

// --- Comonad position helpers ---

/** Annotate every position with its nesting depth (0 for atomic patterns). */
export const depthAt = <V>(p: Pattern<V>): Pattern<number> =>
  extend((sub: Pattern<V>) => sub.depth)(p)

/** Annotate every position with its composition size (1 for atomic patterns). */
export const sizeAt = <V>(p: Pattern<V>): Pattern<number> =>
  extend((sub: Pattern<V>) => sub.size)(p)

/**
 * Annotate every position with its index path from the outermost pattern.
 *
 * The outermost pattern receives `[]`; its element at position `i` receives `[i]`;
 * that element's element at position `j` receives `[i, j]`; and so on.
 *
 * @typeParam V - Value type of the input pattern
 * @param p - The pattern to annotate
 * @returns A `Pattern<number[]>` where each position's value is its index path
 *
 * @example
 * ```typescript
 * const p = Pattern.pattern("outer", [Pattern.point("a"), Pattern.point("b")])
 * const indexed = indicesAt(p)
 * // outermost value: []
 * // first element value: [0]
 * // second element value: [1]
 * ```
 */
export const indicesAt = <V>(p: Pattern<V>): Pattern<number[]> => {
  const go = (indices: number[]) => (sub: Pattern<V>): Pattern<number[]> =>
    new Pattern({ value: indices, elements: Data.array(sub.elements.map((e, i) => go([...indices, i])(e))) })
  return go([])(p)
}

/**
 * Return all values in order: this pattern's value first, then each element's values
 * recursively (pre-order).
 *
 * @typeParam V - Value type of the pattern
 * @param p - The pattern to collect values from
 * @returns A flat array of all values in pre-order
 */
export const values = <V>(p: Pattern<V>): ReadonlyArray<V> => {
  const result: Array<V> = []

  const visit = (node: Pattern<V>): void => {
    result.push(node.value)
    for (const element of node.elements) visit(element)
  }

  visit(p)
  return result
}
