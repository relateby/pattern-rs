import { Data } from "effect"
import { describe, expect, it } from "vitest"
import { allValues, anyValue, combine, contains, depthAt, indicesAt, matches, para, sizeAt, unfold, values } from "../src/ops.js"
import { Pattern } from "../src/pattern.js"
import { Subject } from "../src/subject.js"

describe("Pattern ops", () => {
  it("values returns subjects in pre-order", () => {
    const pattern = new Pattern({
      value: Subject.fromId("root"),
      elements: Data.array([
        Pattern.point(Subject.fromId("left")),
        new Pattern({
          value: Subject.fromId("right"),
          elements: Data.array([Pattern.point(Subject.fromId("leaf"))]),
        }),
      ]),
    })

    expect(values(pattern).map((subject) => subject.identity)).toEqual([
      "root",
      "left",
      "right",
      "leaf",
    ])
  })
})

// Helper tree for US1 tests:
// root(1)
//   left(2)
//   right(3)
//     leaf(4)
const makeTree = () =>
  new Pattern({
    value: 1,
    elements: Data.array([
      Pattern.point(2),
      new Pattern({ value: 3, elements: Data.array([Pattern.point(4)]) }),
    ]),
  })

describe("anyValue", () => {
  it("returns true when root satisfies predicate", () => {
    expect(anyValue((v: number) => v === 1)(makeTree())).toBe(true)
  })

  it("returns true when a leaf satisfies predicate (short-circuits)", () => {
    expect(anyValue((v: number) => v === 4)(makeTree())).toBe(true)
  })

  it("returns false when no value satisfies predicate", () => {
    expect(anyValue((v: number) => v > 100)(makeTree())).toBe(false)
  })

  it("const true always returns true (algebraic law)", () => {
    expect(anyValue((_: number) => true)(makeTree())).toBe(true)
  })

  it("const false on leaf returns false", () => {
    expect(anyValue((_: number) => false)(Pattern.point(42))).toBe(false)
  })
})

describe("allValues", () => {
  it("returns true when all values satisfy predicate", () => {
    expect(allValues((v: number) => v > 0)(makeTree())).toBe(true)
  })

  it("returns false when root fails predicate (short-circuits)", () => {
    expect(allValues((v: number) => v !== 1)(makeTree())).toBe(false)
  })

  it("returns false when a leaf fails predicate", () => {
    expect(allValues((v: number) => v < 4)(makeTree())).toBe(false)
  })

  it("const true always returns true (algebraic law)", () => {
    expect(allValues((_: number) => true)(makeTree())).toBe(true)
  })

  it("const false on non-empty pattern returns false", () => {
    expect(allValues((_: number) => false)(makeTree())).toBe(false)
  })
})

describe("matches", () => {
  it("returns true for identical leaf patterns", () => {
    expect(matches(Pattern.point(42), Pattern.point(42))).toBe(true)
  })

  it("returns false for leaves with different values", () => {
    expect(matches(Pattern.point(1), Pattern.point(2))).toBe(false)
  })

  it("matches(p, p) is always true (reflexivity)", () => {
    const p = makeTree()
    expect(matches(p, p)).toBe(true)
  })

  it("returns true for structurally equal trees", () => {
    expect(matches(makeTree(), makeTree())).toBe(true)
  })

  it("returns false for trees with different structure", () => {
    const a = new Pattern({ value: 1, elements: Data.array([Pattern.point(2)]) })
    const b = new Pattern({ value: 1, elements: Data.array([Pattern.point(3)]) })
    expect(matches(a, b)).toBe(false)
  })
})

describe("para", () => {
  it("leaf pattern passes empty child results to f", () => {
    const leaf = Pattern.point(42)
    const childCount = para<number, number>((_p, rs) => rs.length)(leaf)
    expect(childCount).toBe(0)
  })

  it("computes tree height (0 for leaf, 1 + max(children) for branch)", () => {
    const height = para<number, number>((_p, rs) =>
      rs.length === 0 ? 0 : 1 + Math.max(...rs)
    )
    expect(height(Pattern.point(1))).toBe(0)
    expect(height(makeTree())).toBe(2)
  })

  it("computes same value-sum as fold when f only uses child results", () => {
    // para sum: f(p, rs) = p.value + sum(rs)
    const paraSum = para<number, number>((p, rs) => p.value + rs.reduce((a, b) => a + b, 0))
    // fold sum: same accumulation
    const foldSum = (p: Pattern<number>): number =>
      p.elements.reduce((acc, e) => acc + foldSum(e), p.value)
    expect(paraSum(makeTree())).toBe(foldSum(makeTree()))
  })

  it("f receives the current sub-pattern (not just its value)", () => {
    // f can inspect the full sub-pattern: here we count total nodes via para
    const size = para<number, number>((p, rs) => 1 + rs.reduce((a, b) => a + b, 0))
    expect(size(Pattern.point(99))).toBe(1)
    expect(size(makeTree())).toBe(4) // root + left + right + leaf
  })

  it("nested: height of single-level tree is 1", () => {
    const oneLevel = new Pattern({ value: 0, elements: Data.array([Pattern.point(1), Pattern.point(2)]) })
    const height = para<number, number>((_p, rs) => rs.length === 0 ? 0 : 1 + Math.max(...rs))
    expect(height(oneLevel)).toBe(1)
  })
})

describe("Pattern.pattern", () => {
  it("creates a non-atomic pattern with explicit children", () => {
    const p = Pattern.pattern(1, [Pattern.point(2), Pattern.point(3)])
    expect(p.isAtomic).toBe(false)
    expect(p.elements.length).toBe(2)
  })

  it("Pattern.pattern(v, []) equals Pattern.point(v)", () => {
    expect(matches(Pattern.pattern(42, []), Pattern.point(42))).toBe(true)
  })

  it("children are exactly the provided elements", () => {
    const a = Pattern.point("a")
    const b = Pattern.point("b")
    const p = Pattern.pattern("root", [a, b])
    expect(matches(p.elements[0], a)).toBe(true)
    expect(matches(p.elements[1], b)).toBe(true)
  })
})

describe("Pattern.fromList", () => {
  it("creates atomic children from a list of values", () => {
    const p = Pattern.fromList("root", ["a", "b", "c"])
    expect(p.elements.length).toBe(3)
    expect(p.elements.every(e => e.isAtomic)).toBe(true)
  })

  it("fromList with empty values equals point", () => {
    expect(matches(Pattern.fromList(42, []), Pattern.point(42))).toBe(true)
  })

  it("elements.length equals input values.length", () => {
    const vals = [1, 2, 3, 4, 5]
    expect(Pattern.fromList(0, vals).elements.length).toBe(vals.length)
  })

  it("each child holds the corresponding value", () => {
    const p = Pattern.fromList("root", [10, 20, 30])
    expect(p.elements.map(e => e.value)).toEqual([10, 20, 30])
  })
})

describe("unfold", () => {
  it("expands to a leaf when expand returns empty children", () => {
    const p = unfold((_n: number) => [_n, []] as const)(42)
    expect(p.isAtomic).toBe(true)
    expect(p.value).toBe(42)
  })

  it("countdown: builds a descending chain", () => {
    const chain = unfold((n: number) => [n, n > 0 ? [n - 1] : []] as const)(3)
    expect(chain.value).toBe(3)
    expect(chain.elements[0].value).toBe(2)
    expect(chain.elements[0].elements[0].value).toBe(1)
    expect(chain.elements[0].elements[0].elements[0].value).toBe(0)
    expect(chain.elements[0].elements[0].elements[0].isAtomic).toBe(true)
  })

  it("binary tree of depth 2 has 7 nodes", () => {
    const tree = unfold((n: number) => [n, n <= 0 ? [] : [n - 1, n - 1]] as const)(2)
    expect(tree.depth).toBe(2)
    expect(tree.size).toBe(7)
  })

  it("unfold terminates on empty-children expand (no infinite loop)", () => {
    const p = unfold((_s: string) => [_s, []] as const)("leaf")
    expect(p.isAtomic).toBe(true)
  })
})

describe("combine", () => {
  it("merges root values via combineValues function", () => {
    const a = Pattern.point("hello")
    const b = Pattern.point(" world")
    const merged = combine((x: string, y: string) => x + y)(a)(b)
    expect(merged.value).toBe("hello world")
    expect(merged.elements.length).toBe(0)
  })

  it("concatenates elements from both patterns", () => {
    const a = Pattern.fromList("root", [1, 2])
    const b = Pattern.fromList("root", [3, 4])
    const merged = combine((x: number, y: number) => x + y)(a)(b)
    expect(merged.elements.length).toBe(4)
    expect(merged.elements.map(e => e.value)).toEqual([1, 2, 3, 4])
  })

  it("combine with identity value produces original elements (identity law)", () => {
    const p = Pattern.fromList(1, [2, 3])
    const empty = Pattern.pattern(0, [])
    const merged = combine((x: number, y: number) => x + y)(p)(empty)
    // value: 1+0=1, elements: [2,3] ++ [] = [2,3]
    expect(merged.value).toBe(1)
    expect(merged.elements.length).toBe(2)
  })

  it("is associative with an associative combineValues function", () => {
    const a = Pattern.pattern(1, [Pattern.point(10)])
    const b = Pattern.pattern(2, [Pattern.point(20)])
    const c = Pattern.pattern(3, [Pattern.point(30)])
    const addV = (x: number, y: number) => x + y
    // (a combine b) combine c
    const leftAssoc = combine(addV)(combine(addV)(a)(b))(c)
    // a combine (b combine c)
    const rightAssoc = combine(addV)(a)(combine(addV)(b)(c))
    expect(leftAssoc.value).toBe(rightAssoc.value)
    expect(leftAssoc.elements.length).toBe(rightAssoc.elements.length)
    expect(leftAssoc.elements.map(e => e.value)).toEqual(rightAssoc.elements.map(e => e.value))
  })

  it("combining two atomic patterns gives an atomic pattern when combineValues is used", () => {
    const a = Pattern.point(10)
    const b = Pattern.point(20)
    const merged = combine((x: number, y: number) => x * y)(a)(b)
    expect(merged.isAtomic).toBe(true)
    expect(merged.value).toBe(200)
  })
})

describe("depthAt", () => {
  it("leaf has depth 0", () => {
    const result = depthAt(Pattern.point(42))
    expect(result.value).toBe(0)
    expect(result.isAtomic).toBe(true)
  })

  it("root of makeTree has depth 2", () => {
    expect(depthAt(makeTree()).value).toBe(2)
  })

  it("direct child of makeTree root has correct depth", () => {
    const annotated = depthAt(makeTree())
    // left child (point(2)) is a leaf → depth 0
    expect(annotated.elements[0].value).toBe(0)
    // right child (3 with one child) has depth 1
    expect(annotated.elements[1].value).toBe(1)
  })

  it("structure is preserved (same shape as input)", () => {
    const annotated = depthAt(makeTree())
    expect(annotated.elements.length).toBe(2)
    expect(annotated.elements[1].elements.length).toBe(1)
  })
})

describe("sizeAt", () => {
  it("leaf has size 1", () => {
    const result = sizeAt(Pattern.point(42))
    expect(result.value).toBe(1)
    expect(result.isAtomic).toBe(true)
  })

  it("root of makeTree has size 4", () => {
    expect(sizeAt(makeTree()).value).toBe(4)
  })

  it("leaf children have size 1", () => {
    const annotated = sizeAt(makeTree())
    // left child (point(2)) is a leaf → size 1
    expect(annotated.elements[0].value).toBe(1)
    // right child (3 with one leaf) has size 2
    expect(annotated.elements[1].value).toBe(2)
  })

  it("structure is preserved (same shape as input)", () => {
    const annotated = sizeAt(makeTree())
    expect(annotated.elements.length).toBe(2)
  })
})

describe("indicesAt", () => {
  it("root has empty index path []", () => {
    expect(indicesAt(Pattern.point(99)).value).toEqual([])
  })

  it("leaf pattern root has path []", () => {
    expect(indicesAt(Pattern.point(42)).value).toEqual([])
  })

  it("direct children have single-element paths", () => {
    const annotated = indicesAt(makeTree())
    expect(annotated.elements[0].value).toEqual([0])
    expect(annotated.elements[1].value).toEqual([1])
  })

  it("grandchild has two-element path", () => {
    const annotated = indicesAt(makeTree())
    // right child (index 1) has one child at index 0 → path [1, 0]
    expect(annotated.elements[1].elements[0].value).toEqual([1, 0])
  })

  it("structure is preserved (same shape as input)", () => {
    const annotated = indicesAt(makeTree())
    expect(annotated.elements.length).toBe(2)
    expect(annotated.elements[1].elements.length).toBe(1)
  })
})

describe("contains", () => {
  it("contains(p)(p) is always true (reflexivity)", () => {
    const p = makeTree()
    expect(contains(p)(p)).toBe(true)
  })

  it("finds needle at root", () => {
    const tree = makeTree()
    expect(contains(tree)(tree)).toBe(true)
  })

  it("finds needle as direct child", () => {
    const needle = Pattern.point(2)
    expect(contains(needle)(makeTree())).toBe(true)
  })

  it("finds needle as nested leaf", () => {
    const needle = Pattern.point(4)
    expect(contains(needle)(makeTree())).toBe(true)
  })

  it("returns false when needle is not present", () => {
    expect(contains(Pattern.point(99))(makeTree())).toBe(false)
  })

  it("finds subtree, not just leaves", () => {
    const subtree = new Pattern({ value: 3, elements: Data.array([Pattern.point(4)]) })
    expect(contains(subtree)(makeTree())).toBe(true)
  })
})
