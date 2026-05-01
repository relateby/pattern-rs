# How do I parse Gram notation into a pattern?

Parsing converts a Gram notation string into a list of `Pattern<Subject>` values. Each top-level element in the Gram string becomes one pattern.

::: code-group

```rust [Rust]
use gram_codec::parse_gram;

let input = "(alice:Person)-[:KNOWS]->(bob:Person)";
let patterns = parse_gram(input)?;

println!("parsed {} patterns", patterns.len());
```

```python [Python]
from relateby import gram

patterns = gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)")

print(f"parsed {len(patterns)} patterns")
```

```typescript [TypeScript]
import { Gram } from "@relateby/gram"
import { Effect } from "effect"

const patterns = await Effect.runPromise(
  Gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)")
)

console.log(`parsed ${patterns.length} patterns`)
```

:::

See also: [What is Gram notation?](/explanations/what-is-gram-notation)
