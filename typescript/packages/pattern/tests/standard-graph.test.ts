import { Effect, Option } from "effect"
import { describe, expect, it } from "vitest"
import { GramParseError } from "../src/errors.js"
import { Pattern } from "../src/pattern.js"
import { StandardGraph } from "../src/standard-graph.js"
import { Subject } from "../src/subject.js"

function node(id: string): Pattern<Subject> {
  return Pattern.point(Subject.fromId(id))
}

function relationship(id: string, source: string, target: string): Pattern<Subject> {
  return new Pattern({
    value: Subject.fromId(id),
    elements: [node(source), node(target)],
  })
}

describe("StandardGraph", () => {
  it("classifies nodes and relationships from patterns", () => {
    const rel = relationship("r1", "alice", "bob")
    const graph = StandardGraph.fromPatterns([rel])

    expect(graph.nodeCount).toBe(2)
    expect(graph.relationshipCount).toBe(1)
    expect(Option.isSome(graph.node("alice"))).toBe(true)
    expect(Option.isSome(graph.relationship("r1"))).toBe(true)
  })

  it("classifies annotations, walks, and other patterns", () => {
    const rel1 = relationship("r1", "a", "b")
    const rel2 = relationship("r2", "b", "c")
    const annotation = new Pattern({
      value: Subject.fromId("ann1"),
      elements: [node("annotated")],
    })
    const walk = new Pattern({
      value: Subject.fromId("walk1"),
      elements: [rel1, rel2],
    })
    const other = new Pattern({
      value: Subject.fromId("other1"),
      elements: [node("x"), node("y"), node("z")],
    })

    const graph = StandardGraph.fromPatterns([annotation, walk, other])

    expect(graph.annotationCount).toBe(1)
    expect(graph.walkCount).toBe(1)
    expect(graph.relationshipCount).toBe(2)
    expect(graph.nodeCount).toBe(4)
    expect(graph.other().map((pattern) => pattern.value.identity)).toContain("other1")
  })

  it("returns explicit Option values for lookups", () => {
    const graph = StandardGraph.fromPatterns([node("alice")])

    expect(Option.isSome(graph.node("alice"))).toBe(true)
    expect(Option.isNone(graph.node("missing"))).toBe(true)
    expect(Option.isNone(graph.relationship("missing"))).toBe(true)
  })

  it("fromGram composes parse and classify", async () => {
    const graph = await Effect.runPromise(StandardGraph.fromGram("(a:Person)-->(b:Person)"))

    expect(graph.nodeCount).toBe(2)
    expect(graph.relationshipCount).toBe(1)
    expect(Option.isSome(graph.node("a"))).toBe(true)
  })

  it("fromGram preserves parse failures", async () => {
    const result = await Effect.runPromise(Effect.either(StandardGraph.fromGram("(alice")))

    expect(result._tag).toBe("Left")
    if (result._tag === "Left") {
      expect(result.left).toBeInstanceOf(GramParseError)
    }
  })
})
