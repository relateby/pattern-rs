// pattern.ts — Recursive tree structure, foundational data type
//
// Extends Data.Class for structural equality via Equal.equals.
// Uses Data.array() for elements so nested patterns compare structurally.
// Operations are in ops.ts as standalone curried functions.

import { Data } from "effect"

export class Pattern<V> extends Data.Class<{
  readonly value:    V
  readonly elements: ReadonlyArray<Pattern<V>>
}> {
  static point<V>(value: V): Pattern<V> {
    return new Pattern({ value, elements: Data.array([]) })
  }

  static of<V>(value: V): Pattern<V> {
    return Pattern.point(value)
  }

  get isAtomic(): boolean {
    return this.elements.length === 0
  }

  get identity(): string | undefined {
    if (typeof this.value === "object" && this.value !== null && "identity" in this.value) {
      const candidate = (this.value as { identity?: unknown }).identity
      return typeof candidate === "string" ? candidate : undefined
    }
    return undefined
  }

  get length(): number {
    return this.elements.length
  }

  get size(): number {
    return 1 + this.elements.reduce((sum, e) => sum + e.size, 0)
  }

  get depth(): number {
    if (this.isAtomic) return 0
    return 1 + Math.max(...this.elements.map((e) => e.depth))
  }
}
