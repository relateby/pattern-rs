# What does the `V` in `Pattern<V>` mean?

`V` is the value type parameter. Every position in a `Pattern<V>` — the root and each element, recursively — holds a value of type `V`. The type parameter lets you use any value type as the decoration.

Common choices:

| Type `V` | Use case |
|----------|----------|
| `Subject` | Property-graph data (the default; produced by `parse_gram`) |
| `String` | Text annotation or labelling |
| `i32`, `f64` | Numeric computation over structured data |
| Custom struct | Application-specific domain types |

The `map` operation transforms `Pattern<V>` into `Pattern<U>` by applying a function to every value:

```rust
let p: Pattern<i32> = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);
let p_str: Pattern<String> = p.map(|n| n.to_string());
// p_str has value "1" and elements ["2", "3"]
```

This is the primary way to change the value type while keeping the structure. The shape (which positions have elements) is never altered by `map`.

The constraint on `V` varies by operation: `map` requires nothing from `V`; `combine` requires `V: Combinable`; storing in a `StandardGraph` requires `V = Subject`. This lets the library be general without forcing all users to adopt `Subject`.
