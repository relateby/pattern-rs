// schema.ts — Decode pipeline for the JSON interchange format
//
// The Rust gram-codec produces a JSON array of AstPattern objects using the
// "subject" key. This module validates and decodes that payload into native
// Pattern<Subject> objects without any external schema library.

import { valueFromRaw } from "./value.js"
import { Subject } from "./subject.js"
import { Pattern } from "./pattern.js"

// --- Raw types (JSON interchange format from Rust gram-codec) ---

interface RawSubject {
  identity:   string
  labels:     ReadonlyArray<string>
  properties: Record<string, unknown>
}

interface RawPattern {
  subject:  RawSubject
  elements: ReadonlyArray<RawPattern>
}

// --- Runtime validators ---

function isRecord(v: unknown): v is Record<string, unknown> {
  return typeof v === "object" && v !== null && !Array.isArray(v)
}

function isRawSubject(v: unknown): v is RawSubject {
  if (!isRecord(v)) return false
  if (typeof v["identity"] !== "string") return false
  if (!Array.isArray(v["labels"]) || !(v["labels"] as unknown[]).every(l => typeof l === "string")) return false
  if (!isRecord(v["properties"])) return false
  return true
}

function isRawPattern(v: unknown): v is RawPattern {
  if (!isRecord(v)) return false
  if (!isRawSubject(v["subject"])) return false
  if (!Array.isArray(v["elements"]) || !(v["elements"] as unknown[]).every(isRawPattern)) return false
  return true
}

export function validatePayload(raw: unknown): ReadonlyArray<RawPattern> {
  if (!Array.isArray(raw) || !(raw as unknown[]).every(isRawPattern)) {
    throw new TypeError("Invalid pattern payload from gram codec")
  }
  return raw as ReadonlyArray<RawPattern>
}

// --- Constructor from raw JSON ---

export function patternFromRaw(raw: RawPattern): Pattern<Subject> {
  const subject = Subject.from({
    identity: raw.subject.identity,
    labels:   raw.subject.labels,
    properties: Object.fromEntries(
      Object.entries(raw.subject.properties).map(([k, v]) => [k, valueFromRaw(v)])
    ),
  })
  return new Pattern({ value: subject, elements: raw.elements.map(patternFromRaw) })
}
