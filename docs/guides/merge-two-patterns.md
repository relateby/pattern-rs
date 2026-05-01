# How do I merge two patterns?

`combine` merges two patterns, concatenating their elements in order. The root values are combined using the `Combinable` trait in Rust, and a caller-supplied function in Python and TypeScript.

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

# Subject has no built-in merge; construct the combined Subject manually
merged = a.combine(b, lambda va, vb: Subject(
    identity=va.identity,
    labels=va.labels | vb.labels,
    properties={**va.properties, **vb.properties},
))
```

```typescript [TypeScript]
import { Pattern, Subject } from "@relateby/pattern"
import { combine } from "@relateby/pattern"
import { pipe, HashSet, HashMap } from "effect"

const a = Pattern.point(Subject.fromId("alice").withLabel("Person"))
const b = Pattern.point(Subject.fromId("alice").withLabel("Employee"))

// Subject has no built-in merge; construct the combined Subject manually
const merged = pipe(
  a,
  combine((va, vb) => new Subject({
    identity: va.identity,
    labels: HashSet.fromIterable([...va.labels, ...vb.labels]),
    properties: HashMap.fromIterable([...va.properties, ...vb.properties]),
  }))(b)
)
```

:::

Elements from `a` appear before elements from `b` in the merged pattern.
