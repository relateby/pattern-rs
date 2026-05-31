import { HashMap } from "effect"
import type { Value } from "./value.js"

export type PropMap = HashMap.HashMap<string, Value>

/**
 * Convert native gram properties into plain JavaScript values.
 *
 * This lets consumers decode properties with schema libraries
 * (Effect Schema, Zod, Valibot, etc.) without hand-rolling `_tag` checks.
 */
export function fromGramProps(props: PropMap): Record<string, unknown> {
  return Object.fromEntries(
    [...HashMap.entries(props)].map(([key, value]) => [key, valueToUnknown(value)])
  )
}

function valueToUnknown(value: Value): unknown {
  switch (value._tag) {
    case "StringVal":
    case "IntVal":
    case "FloatVal":
    case "BoolVal":
    case "SymbolVal":
      return value.value
    case "NullVal":
      return null
    case "TaggedStringVal":
      return { tag: value.tag, content: value.content }
    case "RangeVal":
      return { lower: value.lower, upper: value.upper }
    case "MeasurementVal":
      return { unit: value.unit, value: value.value }
    case "ArrayVal":
      return value.items.map(valueToUnknown)
    case "MapVal":
      return Object.fromEntries(
        Object.entries(value.entries).map(([key, nested]) => [key, valueToUnknown(nested)])
      )
  }
}
