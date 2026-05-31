import { HashMap } from "effect"
import type { Value } from "@relateby/pattern"

/** Effect HashMap keyed by string — the shape of `Subject.properties` internals. */
export type PropMap = HashMap.HashMap<string, Value>

/**
 * Convert native gram properties into plain JavaScript values.
 *
 * Lets consumers decode properties with schema libraries
 * (Effect Schema, Zod, Valibot, etc.) without hand-rolling `_tag` checks.
 *
 * @example
 * ```ts
 * import { Gram, fromGramProps } from "@relateby/pattern-effect"
 * import { Schema } from "effect"
 *
 * const Resource = Schema.Struct({ id: Schema.String, qty: Schema.Int })
 * const { patterns } = await Effect.runPromise(Gram.parseWithHeader("(r:Resource {id: \"r1\", qty: 2})"))
 * const decoded = Schema.decodeUnknownSync(Resource)(fromGramProps(patterns[0]!.value.properties))
 * ```
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
    default: {
      const _exhaustive: never = value
      return _exhaustive
    }
  }
}
