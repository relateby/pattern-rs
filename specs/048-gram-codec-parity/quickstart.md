# Quickstart: Gram Codec Header API (048)

## Python

```python
from relateby.gram import parse, stringify, parse_with_header, stringify_with_header

# Parse a gram document with a header record
header, patterns = parse_with_header("{version: 1, source: 'export'} (alice)-[:KNOWS]->(bob)")
print(header)    # {'version': 1, 'source': 'export'}
print(patterns)  # [Pattern(...)]

# Serialize back with the header
gram_text = stringify_with_header(header, patterns)
print(gram_text)
# {source: "export", version: 1}
# (alice)-[:KNOWS]->(bob)

# Document with no header
header2, patterns2 = parse_with_header("(alice)-[:KNOWS]->(bob)")
print(header2)   # None

# Basic parse / stringify (unchanged API)
patterns3 = parse("(a)-->(b)")
gram_text2 = stringify(patterns3)
```

## TypeScript

```typescript
import { Gram } from "@relateby/pattern"
import { Effect } from "effect"

// Parse a gram document with a header record
const { header, patterns } = await Effect.runPromise(
  Gram.parseWithHeader("{version: 1} (alice)-[:KNOWS]->(bob)")
)
console.log(header)   // { version: 1 }
console.log(patterns) // [Pattern<Subject>]

// Serialize back with the header
const gramText = await Effect.runPromise(
  Gram.stringifyWithHeader(header, patterns)
)
console.log(gramText)
// {version: 1}
// (alice)-[:KNOWS]->(bob)

// Document with no header
const { header: h2, patterns: p2 } = await Effect.runPromise(
  Gram.parseWithHeader("(alice)-[:KNOWS]->(bob)")
)
console.log(h2)  // undefined

// Basic parse / stringify (existing API — unchanged)
const patterns2 = await Effect.runPromise(Gram.parse("(a)-->(b)"))
const gram2 = await Effect.runPromise(Gram.stringify(patterns2))
```

## Round-trip test

Both Python and TypeScript guarantee that:

```
parse_with_header(stringify_with_header(header, patterns))
== (header, patterns)
```
