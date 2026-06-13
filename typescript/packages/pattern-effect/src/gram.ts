// gram.ts — Effect-wrapped versions of @relateby/pattern Gram operations
//
// Lifts the Promise-based Gram API into Effect<T, GramParseError> so that
// Effect ecosystem users get full type-level error tracking and composability.

import { Effect } from "effect"
import { Gram as NativeGram, GramParseError, Pattern, Subject } from "@relateby/pattern"

function wrapParseError(input: string) {
  return (cause: unknown): GramParseError =>
    cause instanceof GramParseError ? cause : new GramParseError({ input, cause })
}

export const Gram = {
  parse: (input: string): Effect.Effect<ReadonlyArray<Pattern<Subject>>, GramParseError> =>
    Effect.tryPromise({ try: () => NativeGram.parse(input), catch: wrapParseError(input) }),

  stringify: (patterns: ReadonlyArray<Pattern<Subject>>): Effect.Effect<string, GramParseError> =>
    Effect.tryPromise({ try: () => NativeGram.stringify(patterns), catch: wrapParseError("") }),

  validate: (input: string): Effect.Effect<void, GramParseError> =>
    Effect.tryPromise({ try: () => NativeGram.validate(input), catch: wrapParseError(input) }),

  parseWithHeader: (input: string): Effect.Effect<{
    header: Record<string, unknown> | undefined
    patterns: ReadonlyArray<Pattern<Subject>>
  }, GramParseError> =>
    Effect.tryPromise({ try: () => NativeGram.parseWithHeader(input), catch: wrapParseError(input) }),

  stringifyWithHeader: (
    header: Record<string, unknown> | undefined,
    patterns: ReadonlyArray<Pattern<Subject>>,
  ): Effect.Effect<string, GramParseError> =>
    Effect.tryPromise({ try: () => NativeGram.stringifyWithHeader(header, patterns), catch: wrapParseError("") }),
}
