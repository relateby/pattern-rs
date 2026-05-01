# How do I create an atomic pattern?

An atomic pattern holds exactly one value and has no elements. It is the simplest `Pattern<V>` — a leaf with nothing beneath it.

::: code-group

```rust [Rust]
use pattern_core::Pattern;
use pattern_core::Subject;

// Create an atomic pattern holding a Subject value
let subject = Subject::from_id("alice");
let p = Pattern::point(subject);

assert!(p.elements().is_empty());
```

```python [Python]
from relateby.pattern import Pattern, Subject

subject = Subject.from_id("alice")
p = Pattern.point(subject)

assert p.is_atomic()
```

```typescript [TypeScript]
import { Pattern, Subject } from "@relateby/pattern"

const subject = Subject.fromId("alice")
const p = Pattern.point(subject)

// p.elements is an empty array
console.log(p.elements.length === 0) // true
```

:::

See also: [What is the difference between an atomic pattern and a pattern with elements?](/explanations/atomic-vs-elements-pattern)
