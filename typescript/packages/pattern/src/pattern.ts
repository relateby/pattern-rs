// pattern.ts — Recursive tree structure, foundational data type
//
// Extends Data.Class for structural equality via Equal.equals.
// Uses Data.array() for elements so nested patterns compare structurally.
// Operations are in ops.ts as standalone curried functions.

import { Data } from "effect"

/**
 * A value paired with an ordered list of elements, where each element is
 * itself a `Pattern<V>`. Patterns compose recursively: an atomic `Pattern`
 * has no elements; any other `Pattern` is a value in the context of its
 * constituent patterns.
 *
 * The structure is general-purpose — a `Pattern` can represent anything
 * compositional. A Shakespearean sonnet, for instance, is a `Pattern` whose
 * value is the title and whose elements are stanzas; each stanza is a
 * `Pattern` whose elements are lines; each line is an atomic `Pattern`.
 *
 * ```typescript
 * const sonnet = Pattern.pattern("Sonnet 18", [
 *   Pattern.pattern("Quatrain 1", [
 *     Pattern.point("Shall I compare thee to a summer's day?"),
 *     Pattern.point("Thou art more lovely and more temperate:"),
 *   ]),
 *   Pattern.pattern("Couplet", [
 *     Pattern.point("So long as men can breathe, or eyes can see,"),
 *     Pattern.point("So long lives this, and this gives life to thee."),
 *   ]),
 * ])
 * ```
 *
 * Graph elements are one specialisation: a `Pattern` with no elements is
 * treated as a node, one with two elements as a relationship, and so on.
 * That interpretation is handled by `StandardGraph`, not by `Pattern` itself.
 *
 * Structural equality is provided by extending `effect`'s `Data.Class`: two
 * `Pattern` instances are equal when their `value` fields are equal and their
 * `elements` arrays are structurally equal (deep, recursive).
 *
 * Standalone pipeable operations (`map`, `fold`, `extend`, `para`, etc.) live
 * in `ops.ts` and compose with `effect`'s `pipe`.
 *
 * @typeParam V - The value type stored at every position
 */
export class Pattern<V> extends Data.Class<{
  readonly value:    V
  readonly elements: ReadonlyArray<Pattern<V>>
}> {
  /**
   * Create a leaf node (atomic pattern) holding `value` with no children.
   *
   * @typeParam V - Value type
   * @param value - The value stored at this node
   * @returns A `Pattern<V>` with no children
   */
  static point<V>(value: V): Pattern<V> {
    return new Pattern({ value, elements: Data.array([]) })
  }

  /**
   * Alias for {@link Pattern.point}. Create a leaf node holding `value`.
   *
   * @typeParam V - Value type
   * @param value - The value stored at this node
   * @returns A `Pattern<V>` with no children
   */
  static of<V>(value: V): Pattern<V> {
    return Pattern.point(value)
  }

  /**
   * Create a non-atomic pattern with an explicit list of child patterns.
   *
   * @typeParam V - Value type
   * @param value - The value stored at the root node
   * @param elements - Ordered child patterns
   * @returns A `Pattern<V>` with the given children
   *
   * @example
   * ```typescript
   * const tree = Pattern.pattern("root", [Pattern.point("a"), Pattern.point("b")])
   * // tree.length === 2
   * ```
   */
  static pattern<V>(value: V, elements: ReadonlyArray<Pattern<V>>): Pattern<V> {
    return new Pattern({ value, elements: Data.array([...elements]) })
  }

  /**
   * Create a pattern whose children are atomic leaf nodes over a flat list of values.
   *
   * Equivalent to `Pattern.pattern(value, values.map(Pattern.point))`.
   *
   * @typeParam V - Value type
   * @param value - The value stored at the root node
   * @param values - Values to wrap as leaf children
   * @returns A `Pattern<V>` whose direct children are all leaves
   *
   * @example
   * ```typescript
   * const list = Pattern.fromList("numbers", [1, 2, 3])
   * // list.length === 3, list.elements[0].value === 1
   * ```
   */
  static fromList<V>(value: V, values: ReadonlyArray<V>): Pattern<V> {
    return new Pattern({ value, elements: Data.array([...values].map(v => Pattern.point(v))) })
  }

  /** `true` when this pattern has no elements. */
  get isAtomic(): boolean {
    return this.elements.length === 0
  }

  /**
   * Extract the identity string from a `Subject`-like value, if present.
   *
   * Inspects `this.value` for an `identity` property of type `string`. Returns
   * `undefined` for primitive values or objects without a string `identity` field.
   * Primarily useful when `V` is `Subject` or a compatible record type.
   */
  get identity(): string | undefined {
    if (typeof this.value === "object" && this.value !== null && "identity" in this.value) {
      const candidate = (this.value as { identity?: unknown }).identity
      return typeof candidate === "string" ? candidate : undefined
    }
    return undefined
  }

  /** Number of direct elements (does not recurse). */
  get length(): number {
    return this.elements.length
  }

  /** Total count of patterns in this composition, including this one (recursive). */
  get size(): number {
    return 1 + this.elements.reduce((sum, e) => sum + e.size, 0)
  }

  /**
   * Maximum nesting depth of this pattern's elements.
   * Atomic patterns return `0`; a pattern whose elements are all atomic returns `1`.
   */
  get depth(): number {
    if (this.isAtomic) return 0
    return 1 + Math.max(...this.elements.map((e) => e.depth))
  }
}
