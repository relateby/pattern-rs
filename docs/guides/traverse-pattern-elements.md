# How do I traverse the elements of a pattern?

Each `Pattern<V>` exposes its sub-patterns via `.elements`. Iterating them gives you the direct children; recurse for deeper traversal.

::: code-group

```rust [Rust]
use pattern_core::Pattern;
use pattern_core::Subject;

let a = Pattern::point(Subject::from_id("a"));
let b = Pattern::point(Subject::from_id("b"));
let root = Pattern::pattern(Subject::from_id("root"), vec![a, b]);

for element in root.elements() {
    println!("{:?}", element.value());
}
```

```python [Python]
from relateby.pattern import Pattern, Subject

a = Pattern.point(Subject.from_id("a"))
b = Pattern.point(Subject.from_id("b"))
root = Pattern.pattern(Subject.from_id("root"), [a, b])

for element in root.elements:
    print(element.value)
```

```typescript [TypeScript]
import { Pattern, Subject } from "@relateby/pattern"

const a = Pattern.point(Subject.fromId("a"))
const b = Pattern.point(Subject.fromId("b"))
const root = Pattern.pattern(Subject.fromId("root"), [a, b])

for (const element of root.elements) {
  console.log(element.value)
}
```

:::

Use `elements()` for direct children only. For deep traversal use `fold` or `para`.
