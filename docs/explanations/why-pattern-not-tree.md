# Why is Pattern not a tree?

`Pattern<V>` is recursive and looks tree-like in structure, but calling it a tree leads to wrong expectations about traversal, equality, and composition.

A tree implies:
- A single root with a distinguished root position
- A clear parent-child hierarchy
- Traversal from root to leaves
- Nodes at "higher" levels are more significant than nodes at "lower" levels

None of these holds in general for `Pattern<V>`. A pattern can represent:
- A graph relationship, where the two element patterns are endpoints — neither is "above" the other
- A walk, where elements are traversed in order as a sequence
- A pure value with no elements at all (atomic pattern)
- A line of a poem, where the elements are words and the parent is a stanza

The *decorated sequence* model makes no hierarchy claim. Elements are ordered sub-parts; the value decorates them. The meaning of that relationship is determined by the context, not the structure.

Calling `Pattern<V>` a tree also suggests tree-specific algorithms (depth-first search, parent pointers, tree balancing) that do not apply. The correct operations are `map`, `fold`, `para`, `combine` — structure-preserving transformations, not tree traversals.
