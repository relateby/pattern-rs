import {
  Gram,
  GraphClass,
  NativeGraphQuery,
  NativePattern,
  NativePatternGraph,
  NativeReconciliationPolicy,
  NativeSubject,
  NativeValidationRules,
  NativeValue,
  StandardGraph,
  TraversalDirection,
  init,
  toGraphView,
} from "@relateby/pattern";

export async function exercisePublicSurface(): Promise<void> {
  await init();

  const alice = new NativeSubject("alice", ["Person"], {
    name: NativeValue.string("Alice"),
  });
  const bob = new NativeSubject("bob", ["Person"], {});
  const alicePattern = NativePattern.point(alice);
  const bobPattern = NativePattern.point(bob);
  const rules = new NativeValidationRules();
  const policy = NativeReconciliationPolicy.lastWriteWins();
  const nativeGraph = NativePatternGraph.fromPatterns([alicePattern, bobPattern], policy);
  const query = NativeGraphQuery.fromPatternGraph(nativeGraph);
  const standardGraph = StandardGraph.fromPatterns([alicePattern, bobPattern]);
  const parsed = await Gram.parse("(alice:Person)");
  const first = await Gram.parseOne("(alice:Person)");
  const serialized: string = await Gram.stringify(first);
  const nodeCount: number = standardGraph.nodeCount;
  const maybeAlice = standardGraph.node("alice");

  void rules;
  void query;
  void standardGraph;
  void parsed;
  void serialized;
  void nodeCount;
  void maybeAlice;
  void GraphClass.NODE;
  void TraversalDirection.FORWARD;
  void toGraphView;
}
