// errors.ts — Typed error for gram parse/serialize failures
//
// Plain Error subclass with a _tag discriminant. GramParseError is thrown
// by Gram methods and caught by callers (or wrapped in Effect by the adapter).

export class GramParseError extends Error {
  readonly _tag = "GramParseError" as const
  readonly input: string
  readonly cause: unknown

  constructor({ input, cause }: { readonly input: string; readonly cause: unknown }) {
    super(cause instanceof Error ? cause.message : String(cause))
    this.name = "GramParseError"
    this.input = input
    this.cause = cause
  }
}
