# What is Gram notation?

Gram notation is a human-readable text format for serialising `Pattern<Subject>` values. It is the external representation of patterns — what you write in files and what you parse back into in-memory structures.

The general form is:

```
["decoration" | element, element, ...]
```

This encodes any pattern: the bracketed string is the value decoration; the elements after `|` are the sub-patterns.

Gram also provides syntactic sugar for common graph element shapes:

```
(alice:Person { age: 30 })          -- a node with identity, label, and property
(a)-[:KNOWS]->(b)                   -- a directed relationship
(a)-[:KNOWS]-(b)                    -- an undirected relationship
(a)-[:KNOWS]->(b)-[:WORKS_AT]->(c)  -- a walk
```

These shorthand forms expand into the general pattern structure. A node `(alice:Person)` is an atomic pattern (no elements) whose value is the `Subject` `{identity: "alice", labels: ["Person"]}`. A relationship `(a)-[:r]->(b)` is a pattern with two elements — the two endpoint patterns.

Gram is bidirectional: `parse` converts a Gram string to patterns; `stringify` converts patterns back to a Gram string. A valid pattern always round-trips through parse and stringify.
