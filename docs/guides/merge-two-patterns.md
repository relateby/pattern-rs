# How do I merge two patterns?

`combine` merges two patterns: the values are combined using the `Combinable` trait (or a provided function), and the elements of both patterns are concatenated in order.

::: code-group

```rust [Rust]
use pattern_core::Pattern;
use pattern_core::Subject;

let a = Pattern::point(Subject::from_id("alice").with_label("Person"));
let b = Pattern::point(Subject::from_id("alice").with_label("Employee"));

// Subject implements Combinable: merges labels and properties, keeps first identity
let merged = a.combine(b);
```

```python [Python]
from relateby.pattern import Pattern, Subject

a = Pattern.point(Subject.from_id("alice").with_label("Person"))
b = Pattern.point(Subject.from_id("alice").with_label("Employee"))

# combine_values is called on the two Subject values
merged = a.combine(b, lambda va, vb: va.merge(vb))
```

```typescript [TypeScript]
import { Pattern, Subject } from "@relateby/pattern"
import { combine } from "@relateby/pattern"
import { pipe } from "effect"

const a = Pattern.point(Subject.fromId("alice").withLabel("Person"))
const b = Pattern.point(Subject.fromId("alice").withLabel("Employee"))

const merged = pipe(a, combine((va, vb) => va.merge(vb))(b))
```

:::

Elements from `a` appear before elements from `b` in the merged pattern.
