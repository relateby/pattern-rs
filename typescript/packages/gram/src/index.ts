import { Effect } from "effect";

import { Gram as PatternGram } from "@relateby/pattern";

export const Gram = PatternGram;

/**
 * Warm the shared WASM-backed Gram surface for callers that prefer explicit init.
 * The underlying `@relateby/pattern` package still performs lazy loading on demand.
 */
export async function init(): Promise<void> {
  await Effect.runPromise(PatternGram.validate("(init)"));
}
