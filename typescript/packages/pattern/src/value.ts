// value.ts — Tagged union of all gram value types
//
// Each variant is a plain discriminated-union interface. Factory functions fill
// _tag automatically, matching the shape produced by Data.tagged() in Effect.

// --- Variant interfaces ---

export interface StringVal       { readonly _tag: "StringVal";       readonly value: string }
export interface IntVal          { readonly _tag: "IntVal";          readonly value: number }
export interface FloatVal        { readonly _tag: "FloatVal";        readonly value: number }
export interface BoolVal         { readonly _tag: "BoolVal";         readonly value: boolean }
export interface NullVal         { readonly _tag: "NullVal" }
export interface SymbolVal       { readonly _tag: "SymbolVal";       readonly value: string }
export interface TaggedStringVal { readonly _tag: "TaggedStringVal"; readonly tag: string; readonly content: string }
export interface ArrayVal        { readonly _tag: "ArrayVal";        readonly items: ReadonlyArray<Value> }
// Note: MapVal uses Record (not Map) since the JSON interchange uses plain objects
export interface MapVal          { readonly _tag: "MapVal";          readonly entries: Readonly<Record<string, Value>> }
export interface RangeVal        { readonly _tag: "RangeVal";        readonly lower?: number; readonly upper?: number }
export interface MeasurementVal  { readonly _tag: "MeasurementVal";  readonly unit: string; readonly value: number }

export type Value =
  StringVal | IntVal | FloatVal | BoolVal | NullVal | SymbolVal |
  TaggedStringVal | ArrayVal | MapVal | RangeVal | MeasurementVal

// --- Constructor namespace — factory functions fill _tag automatically ---

export const Value = {
  String:       (args: Omit<StringVal,       "_tag">): StringVal       => ({ _tag: "StringVal",       ...args }),
  Int:          (args: Omit<IntVal,          "_tag">): IntVal          => ({ _tag: "IntVal",          ...args }),
  Float:        (args: Omit<FloatVal,        "_tag">): FloatVal        => ({ _tag: "FloatVal",        ...args }),
  Bool:         (args: Omit<BoolVal,         "_tag">): BoolVal         => ({ _tag: "BoolVal",         ...args }),
  Null:         (_args?: Record<never, never>): NullVal                => ({ _tag: "NullVal" }),
  Symbol:       (args: Omit<SymbolVal,       "_tag">): SymbolVal       => ({ _tag: "SymbolVal",       ...args }),
  TaggedString: (args: Omit<TaggedStringVal, "_tag">): TaggedStringVal => ({ _tag: "TaggedStringVal", ...args }),
  Array:        (args: Omit<ArrayVal,        "_tag">): ArrayVal        => ({ _tag: "ArrayVal",        ...args }),
  Map:          (args: Omit<MapVal,          "_tag">): MapVal          => ({ _tag: "MapVal",          ...args }),
  Range:        (args: Omit<RangeVal,        "_tag">): RangeVal        => ({ _tag: "RangeVal",        ...args }),
  Measurement:  (args: Omit<MeasurementVal,  "_tag">): MeasurementVal  => ({ _tag: "MeasurementVal",  ...args }),

  equals(a: Value, b: Value): boolean {
    if (a._tag !== b._tag) return false
    const aRec = a as unknown as Record<string, unknown>
    const bRec = b as unknown as Record<string, unknown>
    const keys = Object.keys(aRec).filter(k => k !== "_tag")
    return keys.every(k => {
      const av = aRec[k], bv = bRec[k]
      if (Array.isArray(av) && Array.isArray(bv)) {
        return av.length === bv.length &&
          av.every((item, i) => Value.equals(item as Value, bv[i] as Value))
      }
      if (isRecord(av) && isRecord(bv)) {
        const aKeys = Object.keys(av), bKeys = Object.keys(bv)
        return aKeys.length === bKeys.length &&
          aKeys.every(ek => Value.equals(av[ek] as Value, bv[ek] as Value))
      }
      return av === bv
    })
  },
} as const

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value)
}

function asOptionalNumber(value: unknown, field: string): number | undefined {
  if (value === undefined || value === null) return undefined
  if (typeof value === "number") return value
  throw new TypeError(`Expected ${field} to be a number`)
}

/**
 * Decode a raw JSON-interchange value from Rust into the native tagged Value union.
 */
export function valueFromRaw(raw: unknown): Value {
  if (typeof raw === "string") return Value.String({ value: raw })
  if (typeof raw === "boolean") return Value.Bool({ value: raw })
  if (typeof raw === "number") {
    return Number.isInteger(raw)
      ? Value.Int({ value: raw })
      : Value.Float({ value: raw })
  }
  if (Array.isArray(raw)) {
    return Value.Array({ items: raw.map(valueFromRaw) })
  }
  if (!isRecord(raw)) {
    throw new TypeError("Unsupported raw value")
  }

  const typeTag = typeof raw.type === "string" ? raw.type : undefined
  switch (typeTag) {
    case "symbol":
      if (typeof raw.value !== "string") throw new TypeError("Expected symbol value to be a string")
      return Value.Symbol({ value: raw.value })
    case "tagged":
      if (typeof raw.tag !== "string") throw new TypeError("Expected tagged value tag to be a string")
      if (typeof raw.content !== "string") throw new TypeError("Expected tagged value content to be a string")
      return Value.TaggedString({ tag: raw.tag, content: raw.content })
    case "range":
      return Value.Range({
        lower: asOptionalNumber(raw.lower, "range.lower"),
        upper: asOptionalNumber(raw.upper, "range.upper"),
      })
    case "measurement":
      if (typeof raw.unit !== "string") throw new TypeError("Expected measurement unit to be a string")
      if (typeof raw.value !== "number") throw new TypeError("Expected measurement value to be a number")
      return Value.Measurement({ unit: raw.unit, value: raw.value })
    default:
      return Value.Map({
        entries: Object.fromEntries(
          Object.entries(raw).map(([key, value]) => [key, valueFromRaw(value)])
        ),
      })
  }
}

