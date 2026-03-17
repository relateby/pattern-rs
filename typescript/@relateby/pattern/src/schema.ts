// schema.ts — Decode pipeline for the JSON interchange format
//
// The Rust gram-codec produces a JSON array of AstPattern objects using the
// "subject" key (not "value"). This module validates and decodes that payload
// into native Pattern<Subject> objects.
//
// Schema.suspend is required for the self-referential elements array.
// The entire tree is decoded and validated in one pass before any Pattern is
// constructed — invalid codec output surfaces as a GramParseError, not a crash.

import { Data, HashMap, HashSet, Schema } from "effect"
import { Value, ValueSchema } from "./value.js"
import { Subject } from "./subject.js"
import { Pattern } from "./pattern.js"

// --- Raw types (JSON interchange format from Rust gram-codec) ---

interface RawSubject {
  identity:   string
  labels:     ReadonlyArray<string>
  properties: Record<string, Value>
}

interface RawPattern {
  subject:  RawSubject
  elements: ReadonlyArray<RawPattern>
}

// --- Schemas ---

const RawSubjectSchema = Schema.Struct({
  identity:   Schema.String,
  labels:     Schema.Array(Schema.String),
  properties: Schema.Record({ key: Schema.String, value: ValueSchema }),
})

// Schema.suspend is required because RawPatternSchema references itself via elements
const RawPatternSchema: Schema.Schema<RawPattern> = Schema.Struct({
  subject:  RawSubjectSchema,
  elements: Schema.Array(Schema.suspend((): Schema.Schema<RawPattern> => RawPatternSchema)),
})

export const decodePayload = Schema.decodeUnknownSync(Schema.Array(RawPatternSchema))

// --- Constructor from raw JSON ---

export function patternFromRaw(raw: RawPattern): Pattern<Subject> {
  const subject = new Subject({
    identity:   raw.subject.identity,
    labels:     HashSet.fromIterable(raw.subject.labels),
    properties: HashMap.fromIterable(
      Object.entries(raw.subject.properties as Record<string, Value>)
    ),
  })
  const elements = Data.array(
    (raw.elements as ReadonlyArray<RawPattern>).map(patternFromRaw)
  )
  return new Pattern({ value: subject, elements })
}
