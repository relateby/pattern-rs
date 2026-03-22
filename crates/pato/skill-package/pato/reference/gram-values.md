# Gram Value Types

Gram values are the data carried by a pattern or record. This topic covers the
full value surface defined by the tree-sitter grammar and carried through the
`pattern_core::Subject` value model.

## Value Types

- `integer`
- `decimal`
- `hexadecimal`
- `octal`
- `measurement`
- `range`
- `boolean`
- `string`
- `tagged string`
- `symbol`
- `array`
- `map`

## Notes

- Scalar forms include numbers, booleans, strings, symbols, ranges, and
  measurements.
- `array` and `map` are value forms in the grammar and are supported by the CST
  and `Subject` value model.
- `string` includes single-quoted, double-quoted, backtick-quoted, and fenced
  multiline forms.
- `tagged string` pairs a type tag with string content.

## Guidance

- Use plain strings for names and labels when a quoted value is clearer.
- Use numbers for numeric data.
- Use booleans for simple state.
- Use records when you want named fields.
- Use arrays when the value itself is an ordered collection.
- Use maps when you want a nested key/value structure inside a value position.

## Examples

Integer:

```gram
(point x=42)
```

Decimal:

```gram
(point ratio=3.14)
```

Boolean:

```gram
(flag active=true)
```

Range and measurement:

```gram
(scale span=1..10 weight=5kg)
```

String and tagged string:

```gram
(label value="Ada" tagged=url`https://example.com`)
```

Array and map:

```gram
(meta tags=["social", "graph", "gram"] details={ kind: "social", scope: "public" })
```

## Related Topics

- `gram`
- `gram-records`
- `gram-annotations`
