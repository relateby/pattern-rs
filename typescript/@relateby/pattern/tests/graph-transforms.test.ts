import { describe, expect, it } from "vitest"
import {
  Pattern,
  Subject,
  ReplaceWithSurrogate,
  SpliceGap,
  filterGraph,
  toGraphView,
} from "../src/index.js"

function graphFromPatterns(
  nodes: ReadonlyArray<Pattern<Subject>>,
  relationships: ReadonlyArray<Pattern<Subject>>
) {
  return {
    nodes,
    relationships,
    walks: [],
    annotations: [],
    conflicts: {},
    size: nodes.length + relationships.length,
    merge(other: ReturnType<typeof graphFromPatterns>) {
      return graphFromPatterns(
        [...this.nodes, ...other.nodes],
        [...this.relationships, ...other.relationships]
      )
    },
    topoSort() {
      return [...this.nodes, ...this.relationships]
    },
  }
}

describe("graph transforms", () => {
  it("removes relationships whose endpoints were filtered out", () => {
    const alice = Pattern.point(Subject.fromId("alice").withLabel("Person"))
    const bob = Pattern.point(Subject.fromId("bob").withLabel("Person"))
    const knows = new Pattern({
      value: Subject.fromId("r1").withLabel("KNOWS"),
      elements: [alice, bob],
    })

    const view = toGraphView(graphFromPatterns([alice, bob], [knows]))
    const filtered = filterGraph((_cls, pattern) => pattern.identity !== "bob", SpliceGap)(view)

    expect(filtered.viewElements.map(([, pattern]) => pattern.identity)).toEqual(["alice"])
  })

  it("can replace a removed relationship endpoint with a surrogate", () => {
    const alice = Pattern.point(Subject.fromId("alice").withLabel("Person"))
    const bob = Pattern.point(Subject.fromId("bob").withLabel("Person"))
    const unknown = Pattern.point(Subject.fromId("unknown").withLabel("Placeholder"))
    const knows = new Pattern({
      value: Subject.fromId("r1").withLabel("KNOWS"),
      elements: [alice, bob],
    })

    const view = toGraphView(graphFromPatterns([alice, bob], [knows]))
    const filtered = filterGraph(
      (_cls, pattern) => pattern.identity !== "bob",
      ReplaceWithSurrogate(unknown)
    )(view)

    const relationship = filtered.viewElements.find(([, pattern]) => pattern.identity === "r1")?.[1]
    expect(relationship?.elements.map((pattern) => pattern.identity)).toEqual(["alice", "unknown"])
  })
})
