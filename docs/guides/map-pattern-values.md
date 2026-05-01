# How do I map over a pattern's values?

`map` applies a function to every value in the pattern — both the root value and all element values — while preserving the structure exactly. It transforms `Pattern<V>` into `Pattern<U>`.

::: code-group

```rust [Rust]
use pattern_core::Pattern;

let p = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);
let doubled = p.map(|v| v * 2);

// doubled has value 2 and elements [4, 6]
```

```python [Python]
from relateby.pattern import Pattern

p = Pattern.pattern(1, [Pattern.point(2), Pattern.point(3)])
doubled = p.map(lambda v: v * 2)

# doubled has value 2 and elements [4, 6]
```

```typescript [TypeScript]
import { Pattern } from "@relateby/pattern"
import { map } from "@relateby/pattern"
import { pipe } from "effect"

const p = Pattern.pattern(1, [Pattern.point(2), Pattern.point(3)])
const doubled = pipe(p, map((v) => v * 2))

// doubled.value === 2, doubled.elements[0].value === 4
```

:::

See also: [What does the `V` in `Pattern<V>` mean?](/explanations/what-is-v-in-pattern)
