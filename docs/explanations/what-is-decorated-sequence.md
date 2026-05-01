# What is a 'decorated sequence'?

A decorated sequence has two parts: the *sequence* (the elements) and the *decoration* (the value). In `Pattern<V>`, the elements are the sequence and the value is the decoration.

The general Gram notation form makes this visible:

```
["decoration" | element1, element2, element3]
```

The string `"decoration"` is the value. `element1`, `element2`, `element3` are the elements — each itself a pattern. The vertical bar separates decoration from sequence.

This framing is more accurate than calling `Pattern<V>` a node, a tree, or a record. A node suggests graph membership. A tree suggests a parent-child hierarchy with a root. A record suggests a flat bag of named fields. None of these captures what `Pattern<V>` actually is.

A decorated sequence captures: *here is a value, and here are the ordered sub-things it is made of or relates to*. What those sub-things mean — graph elements, lines of a poem, nested JSON arrays — is determined by how you interpret the pattern, not by the pattern itself.
