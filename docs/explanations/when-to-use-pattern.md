# When should I use Pattern versus a plain graph library?

Use `Pattern<V>` when your data is inherently a *decorated sequence* — when what you have is a value accompanied by an ordered list of sub-parts, and that structure matters.

Good fits for `Pattern<V>`:
- **Graph elements you are constructing or transforming**: nodes, relationships, walks as first-class values that you build up, map over, or serialize.
- **Structured notation**: Gram documents, formatted text, nested structured data where the shape carries meaning.
- **Custom value types**: You have a type `V` and you want to compose values with sub-values while preserving structure through transformations.

A plain graph library is a better fit when:
- Your primary operation is **traversal and query** over a large connected graph (e.g., shortest path, reachability, community detection).
- You need **index structures** over millions of nodes for fast lookup.
- The graph is **mutable in place** with concurrent writers.

`StandardGraph` bridges both worlds. It stores `Pattern<Subject>` elements in a queryable structure indexed by identity. Use it when you need to look up elements by ID, query by label, or retrieve relationships from both endpoints. It is not a general-purpose graph database — it is a structured container for patterns that you have already built.

If you are parsing Gram files and building in-memory graphs for processing, `StandardGraph` is the right tool. If you are running graph algorithms at scale, reach for a graph database or a dedicated graph algorithm library.
