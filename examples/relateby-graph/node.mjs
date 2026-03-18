/**
 * relateby-graph Node.js example
 *
 * Demonstrates:
 * 1. Building a graph from native Pattern/Subject values
 * 2. Converting to a GraphView
 * 3. Applying mapGraph + filterGraph via @relateby/graph
 *
 * Run:
 *   node node.mjs
 */

import { Pattern, Subject, Value } from "@relateby/pattern";
import { filterGraph, mapGraph, SpliceGap, toGraphView } from "@relateby/graph";

const alice = Pattern.point(Subject.fromId("alice").withLabel("Person"));
const bob = Pattern.point(Subject.fromId("bob").withLabel("Person"));
const company = Pattern.point(Subject.fromId("acme").withLabel("Company"));

const graph = {
  nodes: [alice, bob, company],
  relationships: [],
  walks: [],
  annotations: [],
  conflicts: {},
  size: 3,
  merge(other) {
    return {
      ...this,
      nodes: [...this.nodes, ...other.nodes],
      size: this.size + other.size,
    };
  },
  topoSort() {
    return this.nodes;
  },
};

const view = toGraphView(graph);

const peopleOnly = filterGraph(
  (cls, pattern) => cls.tag !== "GNode" || pattern.identity !== "acme",
  SpliceGap,
)(view);

const renamed = mapGraph({
  mapNode: (pattern) =>
    new Pattern({
      ...pattern,
      value: pattern.value.withProperty("processed", Value.Bool({ value: true })),
    }),
})(view);

console.log(`Graph nodes: ${view.viewElements.length}`);
console.log(`People-only nodes: ${peopleOnly.viewElements.length}`);
console.log(
  "Processed node IDs:",
  renamed.viewElements.map(([, pattern]) => pattern.identity),
);
