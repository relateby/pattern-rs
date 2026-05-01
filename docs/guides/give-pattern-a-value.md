# How do I give a pattern a value?

Every `Pattern<V>` carries a value of type `V`. The most common type is `Subject`, which carries identity, labels, and properties. Pass the value as the first argument to any constructor.

::: code-group

```rust [Rust]
use pattern_core::Pattern;
use pattern_core::Subject;

// Subject with identity and a label
let subject = Subject::from_id("alice")
    .with_label("Person");

let p = Pattern::point(subject);
```

```python [Python]
from relateby.pattern import Pattern, Subject

subject = Subject.from_id("alice").with_label("Person")
p = Pattern.point(subject)
```

```typescript [TypeScript]
import { Pattern, Subject } from "@relateby/pattern"

const subject = Subject.fromId("alice").withLabel("Person")
const p = Pattern.point(subject)
```

:::

`Subject` supports chaining: add multiple labels with `.with_label()` and properties with `.with_property()`.

See also: [What is a Subject?](/explanations/what-is-subject)
