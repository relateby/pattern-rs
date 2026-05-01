# What is a Pattern?

A `Pattern<V>` is a value of type `V` paired with an ordered list of elements, where each element is itself a `Pattern<V>`. This is the *decorated sequence* model: the elements form the pattern concept; the value decorates it.

```
Pattern<V> = { value: V, elements: [Pattern<V>] }
```

An atomic pattern has no elements — just a value. Any other pattern has both a value and at least one element.

The type is recursive and general-purpose. It can represent anything that has a value and zero or more ordered sub-parts. The most common concrete instantiation is `Pattern<Subject>`, where each position carries a `Subject` (identity, labels, properties). This is the type produced by parsing Gram notation.

The value at any position can be read with `.value` (or `.extract()` in Rust). The elements are accessible via `.elements`. Structure is preserved by all operations that transform values — only `map` and `combine` change what's stored; neither changes the shape.
