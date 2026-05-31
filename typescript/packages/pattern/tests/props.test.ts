import { HashMap } from "effect"
import { describe, expect, it } from "vitest"
import { Value, fromGramProps } from "../src/index.js"

describe("fromGramProps", () => {
  it("flattens primitive gram property values", () => {
    const props = HashMap.fromIterable([
      ["name", Value.String({ value: "Alice" })],
      ["age", Value.Int({ value: 42 })],
      ["active", Value.Bool({ value: true })],
      ["ratio", Value.Float({ value: 0.5 })],
      ["none", Value.Null({})],
    ] as const)

    expect(fromGramProps(props)).toEqual({
      name: "Alice",
      age: 42,
      active: true,
      ratio: 0.5,
      none: null,
    })
  })

  it("flattens nested array/map/tagged values", () => {
    const props = HashMap.fromIterable([
      ["code", Value.TaggedString({ tag: "h3", content: "8f283082aa20c00" })],
      ["distance", Value.Measurement({ unit: "km", value: 10 })],
      ["span", Value.Range({ lower: 1, upper: 3 })],
      ["tags", Value.Array({ items: [Value.String({ value: "a" }), Value.Int({ value: 1 })] })],
      [
        "meta",
        Value.Map({
          entries: {
            nested: Value.Bool({ value: true }),
            symbolish: Value.Symbol({ value: "person" }),
          },
        }),
      ],
    ] as const)

    expect(fromGramProps(props)).toEqual({
      code: { tag: "h3", content: "8f283082aa20c00" },
      distance: { unit: "km", value: 10 },
      span: { lower: 1, upper: 3 },
      tags: ["a", 1],
      meta: {
        nested: true,
        symbolish: "person",
      },
    })
  })
})
