import { Effect, HashSet, Option } from "effect"
import { describe, expect, it } from "vitest"
import { GramParseError } from "../src/errors.js"
import { Pattern } from "../src/pattern.js"
import { StandardGraph } from "../src/standard-graph.js"
import { Subject } from "../src/subject.js"

function node(id: string): Pattern<Subject> {
  return Pattern.point(Subject.fromId(id))
}

function labelledNode(id: string, label: string): Pattern<Subject> {
  return Pattern.point(Subject.fromId(id).withLabel(label))
}

function relationship(id: string, source: string, target: string): Pattern<Subject> {
  return new Pattern({
    value: Subject.fromId(id),
    elements: [node(source), node(target)],
  })
}

function labelledRelationship(
  id: string,
  sourceId: string,
  sourceLabel: string,
  targetId: string,
  targetLabel: string,
): Pattern<Subject> {
  return new Pattern({
    value: Subject.fromId(id),
    elements: [labelledNode(sourceId, sourceLabel), labelledNode(targetId, targetLabel)],
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

  describe("back-reference label preservation", () => {
    it("preserves node labels when a later back-reference omits them", () => {
      // Pattern 1 establishes red:Red and blue:Blue.
      // Pattern 2 uses back-references (same ids, no labels).
      // The labels from Pattern 1 must survive.
      const p1 = labelledRelationship("go1", "red", "Red", "blue", "Blue")
      const p2 = new Pattern({
        value: Subject.fromId("go2"),
        elements: [node("blue"), node("red")],
      })

      const graph = StandardGraph.fromPatterns([p1, p2])

      const red = Option.getOrThrow(graph.node("red"))
      const blue = Option.getOrThrow(graph.node("blue"))

      expect(HashSet.has(red.value.labels, "Red")).toBe(true)
      expect(HashSet.has(blue.value.labels, "Blue")).toBe(true)
      expect(graph.relationshipCount).toBe(2)
    })

    it("accumulates labels when different occurrences contribute different labels", () => {
      const p1 = labelledNode("n", "First")
      const p2 = labelledNode("n", "Second")

      const graph = StandardGraph.fromPatterns([p1, p2])

      const n = Option.getOrThrow(graph.node("n"))
      expect(HashSet.has(n.value.labels, "First")).toBe(true)
      expect(HashSet.has(n.value.labels, "Second")).toBe(true)
    })

    it("three-node cycle preserves all labels", () => {
      // (green:Green)-[:go1]->(red:Red)
      const p1 = new Pattern({
        value: Subject.fromId("go1"),
        elements: [labelledNode("green", "Green"), labelledNode("red", "Red")],
      })
      // (red)-[:go2]->(blue:Blue)
      const p2 = new Pattern({
        value: Subject.fromId("go2"),
        elements: [node("red"), labelledNode("blue", "Blue")],
      })
      // (blue)-[:go3]->(green)
      const p3 = new Pattern({
        value: Subject.fromId("go3"),
        elements: [node("blue"), node("green")],
      })

      const graph = StandardGraph.fromPatterns([p1, p2, p3])

      const green = Option.getOrThrow(graph.node("green"))
      const red = Option.getOrThrow(graph.node("red"))
      const blue = Option.getOrThrow(graph.node("blue"))

      expect(HashSet.has(green.value.labels, "Green")).toBe(true)
      expect(HashSet.has(red.value.labels, "Red")).toBe(true)
      expect(HashSet.has(blue.value.labels, "Blue")).toBe(true)
    })
  })
})
