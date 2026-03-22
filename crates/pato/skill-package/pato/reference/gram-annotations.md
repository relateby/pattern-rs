# Gram Annotations

Annotations are the gram syntax for attaching metadata to a pattern without
changing the underlying pattern content.

## Core Idea

- A single optional `@@` annotation may supply the wrapper identity and labels.
- Zero or more `@` annotations may follow and contribute properties.
- Multiple `@` annotations stack into one property record.
- The wrapped pattern remains the thing being described.

## Syntax

The grammar requires the `@@` identity/label wrapper, when present, to appear
before any property annotations:

```gram
@@someId:Label1:Label2
 @k1("v1") @k2(42)
(a:Thing)
```

This is equivalent to:

```gram
[someId:Label1:Label2 { k1: "v1", k2: 42 } | (a:Thing)]
```

## Semantic Rule

- Annotation syntax is the unary form of pattern decoration.
- The wrapped pattern is the single element of the annotation wrapper.
- `@` properties merge into a single property record.
- `@@` adds identity and labels to the wrapper subject.

## Examples

Property annotations only:

```gram
@tag("primary") @source("manual") (node)
```

Identified wrapper with merged properties:

```gram
@@provenance:Source @confidence(0.9) (node)
```

## Related Topics

- `gram`
- `gram-patterns`
- `gram-path_equivalences`
