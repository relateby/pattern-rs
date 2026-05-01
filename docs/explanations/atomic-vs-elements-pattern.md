# What is the difference between an atomic pattern and a pattern with elements?

An **atomic pattern** holds a value and has no elements. It is a leaf — there is nothing beneath it.

A **pattern with elements** holds a value and has one or more sub-patterns as elements. The elements are ordered and each is itself a `Pattern<V>`.

In Gram notation:
- `(alice:Person)` is an atomic pattern. The node has a `Subject` value and no elements.
- `(alice:Person)-[:KNOWS]->(bob:Person)` expands to a pattern with two elements: the `alice` pattern and the `bob` pattern, decorated by the relationship subject.

The `is_atomic()` predicate tests whether a pattern has no elements:

```rust
let p = Pattern::point(subject);
assert!(p.is_atomic()); // true — no elements

let q = Pattern::pattern(subject, vec![p]);
assert!(!q.is_atomic()); // false — has one element
```

Atomic patterns are the base case for all recursive operations. `map`, `fold`, and `para` reach the atomic patterns and return. A pattern entirely composed of atomic patterns in its elements list has depth 1.
