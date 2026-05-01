---
layout: home

hero:
  name: pattern-rs
  tagline: A decorated sequence for Rust, Python, and TypeScript
  actions:
    - theme: brand
      text: Guides
      link: /guides/
    - theme: alt
      text: Explanations
      link: /explanations/
---

## What is pattern-rs?

`pattern-rs` provides the `Pattern<V>` data structure — a value paired with an ordered list of elements, each itself a `Pattern<V>`. This is the *decorated sequence* model: elements form the pattern concept; the value decorates it. An atomic pattern has no elements.

The library is a Rust implementation of the gram-hs reference, with full bindings for Python (via PyO3) and TypeScript (via WebAssembly). All three languages expose equivalent operations with language-idiomatic naming.

**Gram notation** is the human-readable serialisation format for patterns. The general form `["decoration" | element, element, ...]` represents any pattern; the shorthand forms `(node)` and `(a)-[:rel]->(b)` cover common graph element shapes.

## Features

- **[Guides](/guides/)** — Task-oriented answers to "how do I…" questions in Rust, Python, and TypeScript
- **[Explanations](/explanations/)** — Conceptual answers explaining what Pattern is and how it works
- **[Reference](/reference/)** — Full API documentation for all three language targets
