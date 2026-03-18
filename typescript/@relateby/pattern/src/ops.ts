// ops.ts — Standalone curried pipeable operations on Pattern<V>
//
// All operations are pure standalone functions (not methods) so they compose
// with effect-ts pipe() and can be tree-shaken independently.

import { Data, Option, pipe } from "effect"
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
