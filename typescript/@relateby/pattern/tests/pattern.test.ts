import { Effect, Equal, HashMap, HashSet, Option, pipe } from "effect"
import { describe, expect, it } from "vitest"
import {
  Gram,
  Pattern,
  StandardGraph,
  Subject,
  Value,
  findFirst,
  fold,
  values,
} from "../src/index.js"

describe("@relateby/pattern", () => {
  it("builds native Subject values immutably", () => {
    const alice = Subject.fromId("alice")
      .withLabel("Person")
      .withProperty("name", Value.String({ value: "Alice" }))

    expect(alice.identity).toBe("alice")
    expect([...HashSet.values(alice.labels)]).toContain("Person")
    expect([...HashMap.entries(alice.properties)]).toContainEqual([
      "name",
      Value.String({ value: "Alice" }),
    ])
    expect(
      Equal.equals(
        alice,
        Subject.fromId("alice")
          .withLabel("Person")
          .withProperty("name", Value.String({ value: "Alice" }))
      )
    ).toBe(true)
  })

  it("supports native Pattern metrics and operations", () => {
    const tree = new Pattern({
      value: "root",
      elements: [
        Pattern.point("left"),
        new Pattern({ value: "right", elements: [Pattern.point("leaf")] }),
      ],
    })

    expect(tree.isAtomic).toBe(false)
    expect(tree.length).toBe(2)
    expect(tree.size).toBe(4)
    expect(tree.depth).toBe(2)
    expect(values(tree)).toEqual(["root", "left", "right", "leaf"])
    expect(pipe(tree, fold(0, (acc, value) => acc + value.length))).toBe(17)
    expect(pipe(tree, findFirst((value) => value.startsWith("lea")))).toEqual(Option.some("leaf"))
  })

  it("parses, validates, and stringifies gram notation through the JSON codec bridge", async () => {
    const parsed = await Effect.runPromise(Gram.parse("(alice:Person)-->(bob:Person)"))
    const serialized = await Effect.runPromise(Gram.stringify(parsed))
    await Effect.runPromise(Gram.validate("(alice:Person)-->(bob:Person)"))
    const graph = StandardGraph.fromPatterns(parsed)

    expect(parsed).toHaveLength(1)
    expect(graph.nodeCount).toBe(2)
    expect(graph.relationshipCount).toBe(1)
    expect(serialized).toContain("alice")
    expect(serialized).toContain("bob")
  })

  it("decodes subject properties from the JSON codec bridge", async () => {
    const [parsed] = await Effect.runPromise(
      Gram.parse('(alice:Person {name: "Alice", age: 42, active: true})')
    )

    expect(parsed?.value.identity).toBe("alice")
    expect([...HashMap.entries(parsed?.value.properties ?? HashMap.empty())]).toContainEqual([
      "name",
      Value.String({ value: "Alice" }),
    ])
    expect([...HashMap.entries(parsed?.value.properties ?? HashMap.empty())]).toContainEqual([
      "age",
      Value.Int({ value: 42 }),
    ])
    expect([...HashMap.entries(parsed?.value.properties ?? HashMap.empty())]).toContainEqual([
      "active",
      Value.Bool({ value: true }),
    ])
  })

  it("classifies native patterns with StandardGraph", () => {
    const alice = Pattern.point(Subject.fromId("alice").withLabel("Person"))
    const bob = Pattern.point(Subject.fromId("bob").withLabel("Person"))
    const knows = new Pattern({
      value: Subject.fromId("r1").withLabel("KNOWS"),
      elements: [alice, bob],
    })

    const graph = StandardGraph.fromPatterns([knows])

    expect(graph.nodeCount).toBe(2)
    expect(graph.relationshipCount).toBe(1)
    expect(Option.getOrUndefined(graph.node("alice"))?.value.identity).toBe("alice")
    expect(Option.getOrUndefined(graph.source("r1"))?.value.identity).toBe("alice")
    expect(Option.getOrUndefined(graph.target("r1"))?.value.identity).toBe("bob")
  })
})
