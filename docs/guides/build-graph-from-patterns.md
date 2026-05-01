# How do I build a graph from patterns?

`StandardGraph` interprets `Pattern<Subject>` elements as graph elements (nodes, relationships, walks). The easiest way to populate it is directly from a Gram notation string.

::: code-group

```rust [Rust]
use gram_codec::FromGram;
use pattern_core::StandardGraph;

let graph = StandardGraph::from_gram("(alice:Person)-[:KNOWS]->(bob:Person)")?;

println!("nodes: {}", graph.node_count());
println!("relationships: {}", graph.relationship_count());
```

```python [Python]
from relateby.pattern import StandardGraph

graph = StandardGraph.from_gram("(alice:Person)-[:KNOWS]->(bob:Person)")

print(f"nodes: {graph.node_count()}")
print(f"relationships: {graph.relationship_count()}")
```

```typescript [TypeScript]
import { StandardGraph } from "@relateby/pattern"
import { Effect } from "effect"

const graph = await Effect.runPromise(
  StandardGraph.fromGram("(alice:Person)-[:KNOWS]->(bob:Person)")
)

console.log(`nodes: ${graph.nodeCount}`)
console.log(`relationships: ${graph.relationshipCount}`)
```

:::

See also: [When should I use Pattern versus a plain graph library?](/explanations/when-to-use-pattern)
