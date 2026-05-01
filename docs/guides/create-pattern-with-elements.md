# How do I create a pattern with elements?

A pattern with elements pairs a value with an ordered list of sub-patterns. Each element is itself a `Pattern<V>`, so the structure composes recursively.

::: code-group

```rust [Rust]
use pattern_core::Pattern;
use pattern_core::Subject;

let a = Pattern::point(Subject::from_id("a"));
let b = Pattern::point(Subject::from_id("b"));
let root = Pattern::pattern(Subject::from_id("root"), vec![a, b]);

assert_eq!(root.elements().len(), 2);
```

```python [Python]
from relateby.pattern import Pattern, Subject

a = Pattern.point(Subject.from_id("a"))
b = Pattern.point(Subject.from_id("b"))
root = Pattern.pattern(Subject.from_id("root"), [a, b])

assert len(root.elements) == 2
```

```typescript [TypeScript]
import { Pattern, Subject } from "@relateby/pattern"

const a = Pattern.point(Subject.fromId("a"))
const b = Pattern.point(Subject.fromId("b"))
const root = Pattern.pattern(Subject.fromId("root"), [a, b])

console.log(root.elements.length === 2) // true
```

:::

See also: [What is a 'decorated sequence'?](/explanations/what-is-decorated-sequence)
