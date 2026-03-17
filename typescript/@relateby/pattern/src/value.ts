// value.ts — Tagged union of all gram value types
//
// Each variant is a plain interface with a _tag discriminant. Data.tagged()
// creates constructors that provide structural equality via Equal.equals.
// No Data.Case extension needed in effect v3.

import { Data, Schema } from "effect"

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

// --- Constructor namespace — Data.tagged fills _tag automatically ---
// Data.tagged() produces objects with structural equality via Equal.equals.

export const Value = {
  String:       Data.tagged<StringVal>("StringVal"),
  Int:          Data.tagged<IntVal>("IntVal"),
  Float:        Data.tagged<FloatVal>("FloatVal"),
  Bool:         Data.tagged<BoolVal>("BoolVal"),
  Null:         Data.tagged<NullVal>("NullVal"),
  Symbol:       Data.tagged<SymbolVal>("SymbolVal"),
  TaggedString: Data.tagged<TaggedStringVal>("TaggedStringVal"),
  Array:        Data.tagged<ArrayVal>("ArrayVal"),
  Map:          Data.tagged<MapVal>("MapVal"),
  Range:        Data.tagged<RangeVal>("RangeVal"),
  Measurement:  Data.tagged<MeasurementVal>("MeasurementVal"),
} as const

// --- Schema for decoding the JSON interchange payload ---
// Schema.suspend is required because ArrayVal and MapVal reference ValueSchema recursively.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const ValueSchema: Schema.Schema<any> = Schema.Union(
  Schema.TaggedStruct("StringVal",       { value: Schema.String }),
  Schema.TaggedStruct("IntVal",          { value: Schema.Number }),
  Schema.TaggedStruct("FloatVal",        { value: Schema.Number }),
  Schema.TaggedStruct("BoolVal",         { value: Schema.Boolean }),
  Schema.TaggedStruct("NullVal",         {}),
  Schema.TaggedStruct("SymbolVal",       { value: Schema.String }),
  Schema.TaggedStruct("TaggedStringVal", { tag: Schema.String, content: Schema.String }),
  Schema.TaggedStruct("ArrayVal",        { items: Schema.Array(Schema.suspend(() => ValueSchema)) }),
  Schema.TaggedStruct("MapVal",          { entries: Schema.Record({ key: Schema.String, value: Schema.suspend(() => ValueSchema) }) }),
  Schema.TaggedStruct("RangeVal",        { lower: Schema.optional(Schema.Number), upper: Schema.optional(Schema.Number) }),
  Schema.TaggedStruct("MeasurementVal",  { unit: Schema.String, value: Schema.Number }),
)
