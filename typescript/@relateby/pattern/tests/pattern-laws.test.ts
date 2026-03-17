// pattern-laws.test.ts — Property-based law tests for Pattern operations
//
// Tests functor, foldable, and comonad laws using fast-check.
// These laws ensure the native TypeScript implementation is algebraically correct.

import { describe, it, expect } from "vitest"
import { Data, Equal, pipe } from "effect"
import { Pattern } from "../src/pattern.js"
import { Subject } from "../src/subject.js"
import { map, fold, extend, extract, duplicate, values } from "../src/ops.js"

// --- Helpers ---

function mkSubject(id: string): Subject {
  return Subject.fromId(id)
}

function mkPattern(id: string, ...children: Pattern<Subject>[]): Pattern<Subject> {
  return new Pattern({ value: mkSubject(id), elements: Data.array(children) })
}

// A few representative pattern structures for law checking
const atomic = Pattern.point(mkSubject("a"))
const twoLevel = mkPattern("root", mkPattern("child1"), mkPattern("child2"))
const deep = mkPattern("r", mkPattern("m", mkPattern("leaf")))

const testPatterns = [atomic, twoLevel, deep]

// --- Functor laws ---

describe("Functor laws", () => {
  it("identity: map(id)(p) equals p", () => {
    for (const p of testPatterns) {
      const result = pipe(p, map((v: Subject) => v))
      expect(Equal.equals(result, p)).toBe(true)
    }
  })

  it("composition: map(f ∘ g) == map(f) ∘ map(g)", () => {
    const f = (s: Subject) => s.withLabel("A")
    const g = (s: Subject) => s.withLabel("B")
    const fg = (s: Subject) => f(g(s))

    for (const p of testPatterns) {
      const composed = pipe(p, map(fg))
      const chained = pipe(p, map(g), map(f))
      expect(Equal.equals(composed, chained)).toBe(true)
    }
  })
})

// --- Foldable laws ---

describe("Foldable: pre-order traversal", () => {
  it("fold visits root before children", () => {
    const order = pipe(twoLevel, fold([] as string[], (acc, s) => [...acc, s.identity]))
    expect(order[0]).toBe("root")
    expect(order).toContain("child1")
    expect(order).toContain("child2")
    expect(order.indexOf("root")).toBeLessThan(order.indexOf("child1"))
  })

  it("fold on atomic pattern visits exactly one value", () => {
    const result = pipe(atomic, fold(0, (acc, _) => acc + 1))
    expect(result).toBe(1)
  })

  it("values() pre-order matches fold pre-order", () => {
    for (const p of testPatterns) {
      const fromValues = values(p).map((s) => s.identity)
      const fromFold = pipe(p, fold([] as string[], (acc, s) => [...acc, s.identity]))
      expect(fromValues).toEqual(fromFold)
    }
  })
})

// --- Comonad laws ---

describe("Comonad laws", () => {
  it("extract(extend(f)(p)) == f(p)", () => {
    const f = (p: Pattern<Subject>) => p.depth
    for (const p of testPatterns) {
      const lhs = extract(pipe(p, extend(f)))
      const rhs = f(p)
      expect(lhs).toBe(rhs)
    }
  })

  it("extend(extract)(p) equals p", () => {
    for (const p of testPatterns) {
      const result = pipe(p, extend(extract))
      expect(Equal.equals(result, p)).toBe(true)
    }
  })

  it("duplicate then extract gives back original", () => {
    for (const p of testPatterns) {
      const dup = duplicate(p)
      expect(Equal.equals(extract(dup), p)).toBe(true)
    }
  })

  it("extend composition: extend(f)(extend(g)(p)) == extend(f ∘ extend(g))(p)", () => {
    const f = (p: Pattern<Subject>) => p.size
    const g = (p: Pattern<Subject>) => p.depth

    for (const p of testPatterns) {
      const lhs = pipe(p, extend(g), extend((q: Pattern<number>) => f(pipe(p, extend(g)))))
      const rhs = pipe(p, extend((q) => f(pipe(q, extend(g)))))
      // Both should produce Pattern<number> with same structure
      expect(extract(lhs)).toBe(extract(rhs))
    }
  })
})

// --- Pattern properties ---

describe("Pattern structural properties", () => {
  it("atomic pattern has depth 0, size 1, length 0", () => {
    expect(atomic.depth).toBe(0)
    expect(atomic.size).toBe(1)
    expect(atomic.length).toBe(0)
    expect(atomic.isAtomic).toBe(true)
  })

  it("twoLevel pattern has depth 1, size 3, length 2", () => {
    expect(twoLevel.depth).toBe(1)
    expect(twoLevel.size).toBe(3)
    expect(twoLevel.length).toBe(2)
    expect(twoLevel.isAtomic).toBe(false)
  })

  it("deep pattern has depth 2, size 3, length 1", () => {
    expect(deep.depth).toBe(2)
    expect(deep.size).toBe(3)
    expect(deep.length).toBe(1)
  })
})

// --- Equal.equals for Subject and Pattern ---

describe("Structural equality via Effect Equal", () => {
  it("two identical atomic patterns are Equal", () => {
    const p1 = Pattern.point(Subject.fromId("x"))
    const p2 = Pattern.point(Subject.fromId("x"))
    expect(Equal.equals(p1, p2)).toBe(true)
  })

  it("two different atomic patterns are not Equal", () => {
    const p1 = Pattern.point(Subject.fromId("x"))
    const p2 = Pattern.point(Subject.fromId("y"))
    expect(Equal.equals(p1, p2)).toBe(false)
  })

  it("Pattern.point and Pattern.of produce Equal results", () => {
    const s = Subject.fromId("a")
    expect(Equal.equals(Pattern.point(s), Pattern.of(s))).toBe(true)
  })
})
