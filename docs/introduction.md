# Introduction to the Pattern Data Structure

## Overview

Pattern concepts are everywhere—from design patterns in software architecture to musical patterns in composition. The `gram-rs` library provides the **Pattern** data structure, designed to support thinking about pattern concepts as first-class entities.

This guide introduces the core concepts and terminology used throughout the library. Once you understand the concepts, you can explore the **[Gram Notation Reference](gram-notation.md)** or dive into the **[Rust Usage Guide](rust-usage.md)**.

## Core Concepts: The Decorated Sequence

A **Pattern** is a **decorated sequence**. This is the fundamental model of the library:

- The **elements** of the pattern form the pattern concept itself—they are the constituents that define the pattern.
- The **value** of the pattern provides **decoration**—it is information *about* the pattern concept as a whole.

For example, consider a simple relationship: "Alice knows Bob".
- The elements are "Alice" and "Bob".
- The value is "knows".

In this way, the pattern explicitly pairs the constituents with the decoration that describes their relationship or context.

## What Is a Pattern?

A **Pattern** is a **decorated sequence**: the elements form the pattern concept itself, and the value provides decoration about that pattern concept.

For example, consider a musical pattern like "A B B A" (an enclosed rhyme pattern). In Gram notation:

```gram
["Enclosed rhyme" | A, B, B, A]
```

Here:
- The **elements** (`A, B, B, A`) **are** the pattern concept—they form the sequence that defines the pattern concept.
- The **value** (`"Enclosed rhyme"`) provides decoration—it describes what kind of pattern concept this is.

This simple structure enables powerful ways to represent, manipulate, and reason about pattern concepts.

## Why Patterns Matter: Explicit Patterns vs. Implicit Traversals

Many systems represent pattern concepts implicitly. For example, in a traditional knowledge graph, a "route" is encoded as an implicit traversal of nodes connected by relationships.

Consider the famous **"Route 66"**. 
- It's a sequence of road segments and cities.
- It also has a wealth of decoration: its history, its pop culture significance, its nickname "The Mother Road".

In a standard graph database, you might represent Route 66 as nodes for cities connected by "CONNECTS" relationships. But the concept of "Route 66" itself is **implicit**—to find it, you have to perform a traversal across many nodes and relationships.

The Pattern data structure makes pattern concepts **explicit**. Route 66 as a Pattern explicitly represents both:
- The sequence of road segments (as elements).
- The decoration about the route (as the value).

By making pattern concepts explicit, `gram-rs` allows you to:
- **Compare** equivalent pattern concepts (different paths that achieve the same outcome).
- **Compose** small pattern structures into more complex ones.
- **Factor** complex patterns to extract common elements.
- **Decorate** entire sequences with context that wouldn't fit on any single element.

## Glossary of Terms

| Term | Definition |
|------|------------|
| **Pattern** | The core recursive data structure, consisting of a Value and a sequence of Elements. |
| **Value** | The decoration or metadata associated with a pattern concept. |
| **Elements** | The sequence of sub-patterns that form the pattern concept. |
| **Atomic Pattern** | A pattern with zero elements. Often conceptually represents a "node" or "leaf". |
| **Nested Pattern** | A pattern whose elements are themselves patterns, allowing for hierarchical or recursive structures. |
| **Gram Notation** | The standard textual representation for Pattern structures. |
| **Explicit Pattern** | A representation where the sequence of elements and their decoration are first-class entities. |
