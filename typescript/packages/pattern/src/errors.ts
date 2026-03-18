// errors.ts — Typed error for gram parse/serialize failures
//
// Data.TaggedError gives GramParseError a _tag discriminant, structured fields,
// and a proper Error prototype chain. Errors are returned as Effect failures,
// not thrown as exceptions.

import { Data } from "effect"

export class GramParseError extends Data.TaggedError("GramParseError")<{
  readonly input: string
  readonly cause: unknown
}> {}
