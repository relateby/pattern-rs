# How do I serialize a pattern to Gram notation?

Serialization converts a list of `Pattern<Subject>` values back to a Gram notation string. The result is a valid Gram document that can be parsed again.

::: code-group

```rust [Rust]
use gram_codec::{parse_gram, to_gram};

let patterns = parse_gram("(alice:Person)")?;
let output = to_gram(&patterns)?;

println!("{}", output); // (alice:Person)
```

```python [Python]
from relateby import gram

patterns = gram.parse("(alice:Person)")
output = gram.stringify(patterns)

print(output)  # (alice:Person)
```

```typescript [TypeScript]
import { Gram } from "@relateby/gram"
import { Effect, pipe } from "effect"

const output = await Effect.runPromise(
  pipe(
    Gram.parse("(alice:Person)"),
    Effect.flatMap(Gram.stringify)
  )
)

console.log(output) // (alice:Person)
```

:::

See also: [How does Gram notation relate to Pattern?](/explanations/gram-notation-and-pattern)
