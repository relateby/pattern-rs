# What is a Subject?

A `Subject` is a self-descriptive value type with three fields:

- **Identity** (`Symbol`): an optional identifier, e.g. `"alice"`. Used to look up a pattern in a `StandardGraph`.
- **Labels** (`Set<String>`): zero or more string labels, e.g. `["Person", "Employee"]`.
- **Properties** (`Map<String, Value>`): zero or more named properties with typed values.

```
Subject { identity: "alice", labels: ["Person"], properties: { age: 30 } }
```

`Pattern<Subject>` is the standard type for property-graph data in this library. Each position in a pattern carries a `Subject` that describes what that element is: a node, an endpoint of a relationship, a relationship itself.

`Subject` implements `Combinable` — two `Subject` values can be merged. The result keeps the first identity, unions the labels, and merges the properties (first value wins on conflict). This makes it natural to refine a subject by combining partial descriptions.

A minimal subject can be created with just an identity and then augmented:

```rust
let s = Subject::from_id("alice")
    .with_label("Person")
    .with_property("age", Value::Int(30));
```
