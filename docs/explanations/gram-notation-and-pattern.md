# How does Gram notation relate to Pattern?

Gram notation is a serialisation of `Pattern<Subject>`, not a separate data model. There is no "Gram data structure" — when you parse Gram, you get back `Pattern<Subject>` values. When you stringify patterns, you get back Gram text.

The relationship is:

```
Gram string  →  parse()   →  [Pattern<Subject>, ...]
[Pattern<Subject>, ...]  →  stringify()  →  Gram string
```

A valid `Pattern<Subject>` always round-trips: `stringify(parse(s)) == s` (up to normalisation of whitespace and property ordering).

This means Gram is not a layer on top of Pattern — it *is* Pattern, expressed as text. The Gram codec (the `gram-codec` crate / `relateby.gram` module / `@relateby/gram` package) provides the translation in both directions, but the underlying type is always `Pattern<Subject>`.

Gram notation can express any `Pattern<Subject>`. The shorthand forms (parentheses, arrows) are editorial sugar for common shapes. The general `[value | elem, elem]` form covers everything else.
