// ops.ts — Standalone curried pipeable operations on Pattern<V>
//
// All operations are pure standalone functions (not methods) so they compose
// with effect-ts pipe() and can be tree-shaken independently.

import { Data, Equal, Option, pipe } from "effect"
import { Pattern } from "./pattern.js"

// --- Functor ---

/** Transform every value in the tree, preserving structure. Pre-order. */
export const map =
  <V, U>(fn: (v: V) => U) =>
  (p: Pattern<V>): Pattern<U> =>
    new Pattern({ value: fn(p.value), elements: Data.array(p.elements.map(map(fn))) })

// --- Foldable ---

/** Accumulate values via pre-order traversal (root first, then children). */
export const fold =
  <V, R>(init: R, fn: (acc: R, v: V) => R) =>
  (p: Pattern<V>): R =>
    p.elements.reduce((acc, e) => pipe(e, fold(acc, fn)), fn(init, p.value))

/** Collect all matching subtrees in pre-order. */
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

/** Return the first value matching the predicate, or Option.none(). Short-circuits. */
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

/** Context-aware map: fn sees the full subtree at each position. */
export const extend =
  <V, U>(fn: (p: Pattern<V>) => U) =>
  (p: Pattern<V>): Pattern<U> =>
    new Pattern({ value: fn(p), elements: Data.array(p.elements.map(extend(fn))) })

/** Extract the root value. */
export const extract = <V>(p: Pattern<V>): V => p.value

/** Each node's value becomes its own subtree. Enables extend composition. */
export const duplicate = <V>(p: Pattern<V>): Pattern<Pattern<V>> =>
  new Pattern({ value: p, elements: Data.array(p.elements.map(duplicate)) })

// --- Extra utility ---

// --- Structural predicates ---

/** Return true if any value satisfies the predicate. Short-circuits pre-order. */
export const anyValue =
  <V>(pred: (v: V) => boolean) =>
  (p: Pattern<V>): boolean => {
    if (pred(p.value)) return true
    return p.elements.some(anyValue(pred))
  }

/** Return true if every value satisfies the predicate. Short-circuits pre-order. */
export const allValues =
  <V>(pred: (v: V) => boolean) =>
  (p: Pattern<V>): boolean => {
    if (!pred(p.value)) return false
    return p.elements.every(allValues(pred))
  }

/** Structural equality — true if both patterns have the same shape and values. */
export const matches = <V>(a: Pattern<V>, b: Pattern<V>): boolean =>
  Equal.equals(a, b)

/** Return true if needle appears anywhere in haystack (including at root). Curried: needle => haystack. */
export const contains =
  <V>(needle: Pattern<V>) =>
  (haystack: Pattern<V>): boolean =>
    Equal.equals(haystack, needle) || haystack.elements.some(contains(needle))

// --- Paramorphism ---

/** Structure-aware fold: fn receives both the current sub-pattern and pre-computed child results. Bottom-up. */
export const para =
  <V, R>(f: (p: Pattern<V>, childResults: ReadonlyArray<R>) => R) =>
  (p: Pattern<V>): R =>
    f(p, p.elements.map(para(f)))

// --- Semigroup combination ---

/** Combine two patterns: merge root values via combineValues, concatenate elements. */
export const combine =
  <V>(combineValues: (a: V, b: V) => V) =>
  (a: Pattern<V>) =>
  (b: Pattern<V>): Pattern<V> =>
    new Pattern({ value: combineValues(a.value, b.value), elements: Data.array([...a.elements, ...b.elements]) })

// --- Anamorphism ---

/** Expand a seed value into a Pattern<V> tree. Terminates when expand returns empty children. */
export const unfold =
  <A, V>(expand: (seed: A) => readonly [V, ReadonlyArray<A>]) =>
  (seed: A): Pattern<V> => {
    const [value, childSeeds] = expand(seed)
    const inner = unfold<A, V>(expand)
    return new Pattern<V>({ value, elements: Data.array([...childSeeds].map(inner)) })
  }

// --- Comonad position helpers ---

/** Annotate every position with its depth (0 for leaves). */
export const depthAt = <V>(p: Pattern<V>): Pattern<number> =>
  extend((sub: Pattern<V>) => sub.depth)(p)

/** Annotate every position with its subtree size (1 for leaves). */
export const sizeAt = <V>(p: Pattern<V>): Pattern<number> =>
  extend((sub: Pattern<V>) => sub.size)(p)

/** Annotate every position with its root-path index list ([] for root). */
export const indicesAt = <V>(p: Pattern<V>): Pattern<number[]> => {
  const go = (indices: number[]) => (sub: Pattern<V>): Pattern<number[]> =>
    new Pattern({ value: indices, elements: Data.array(sub.elements.map((e, i) => go([...indices, i])(e))) })
  return go([])(p)
}

/** Return all values in pre-order traversal order. */
export const values = <V>(p: Pattern<V>): ReadonlyArray<V> => {
  const result: Array<V> = []

  const visit = (node: Pattern<V>): void => {
    result.push(node.value)
    for (const element of node.elements) visit(element)
  }

  visit(p)
  return result
}
