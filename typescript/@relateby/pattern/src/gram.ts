// gram.ts — Effect-based parse/stringify interface
//
// Returns Effect<..., GramParseError> instead of Promise. Errors are in the
// type signature — no thrown exceptions. Callers use Effect.runPromise or
// pipe with Effect operations to compose before running.

import { Effect, HashMap, HashSet, pipe } from "effect"
import { GramParseError } from "./errors.js"
import { Pattern } from "./pattern.js"
import { Subject } from "./subject.js"
import { decodePayload, patternFromRaw } from "./schema.js"

// --- WASM module loader ---

interface WasmGram {
  parseToJson(input: string): string
  stringifyFromJson(input: string): string
}

let wasmGram: WasmGram | null = null

async function loadWasm(): Promise<WasmGram> {
  if (wasmGram !== null) return wasmGram

  const isNode =
    typeof process !== "undefined" &&
    process.versions != null &&
    process.versions.node != null

  try {
    if (isNode) {
      const { createRequire } = await import("module")
      const { fileURLToPath } = await import("url")
      const { dirname, resolve } = await import("path")
      const __filename = fileURLToPath(import.meta.url)
      const __dirname = dirname(__filename)
      const require = createRequire(import.meta.url)
      const wasmPath = resolve(__dirname, "./wasm-node/pattern_wasm.js")
      const mod = require(wasmPath) as { Gram?: WasmGram }
      if (mod.Gram) {
        wasmGram = mod.Gram as WasmGram
        return wasmGram
      }
    } else {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const mod = await import(/* @vite-ignore */ "./wasm/pattern_wasm.js" as string) as any
      if (mod.Gram) {
        wasmGram = mod.Gram as WasmGram
        return wasmGram
      }
    }
  } catch {
    // fall through to unavailable stub
  }

  const unavailable = (): never => {
    throw new Error(
      "Gram WASM bindings are unavailable. " +
      "Ensure wasm/ or wasm-node/ is present (run build:wasm first)."
    )
  }
  wasmGram = { parseToJson: unavailable, stringifyFromJson: unavailable }
  return wasmGram
}

// --- Public API ---

export const Gram = {
  /**
   * Parse gram notation into an array of Pattern<Subject>.
   *
   * Returns Effect<ReadonlyArray<Pattern<Subject>>, GramParseError>.
   * Use Effect.runPromise to convert to a Promise.
   *
   * @example
   * const patterns = await Effect.runPromise(Gram.parse("(a)-->(b)"))
   */
  parse(input: string): Effect.Effect<ReadonlyArray<Pattern<Subject>>, GramParseError> {
    return pipe(
      Effect.tryPromise({
        try:   async () => {
          const wasm = await loadWasm()
          return JSON.parse(wasm.parseToJson(input)) as unknown
        },
        catch: (cause) => new GramParseError({ input, cause }),
      }),
      Effect.flatMap((raw) =>
        Effect.try({
          try:   () => decodePayload(raw).map(patternFromRaw),
          catch: (cause) => new GramParseError({ input, cause }),
        })
      )
    )
  },

  /**
   * Serialize an array of Pattern<Subject> to gram notation.
   *
   * Returns Effect<string, GramParseError>.
   */
  stringify(
    patterns: ReadonlyArray<Pattern<Subject>>
  ): Effect.Effect<string, GramParseError> {
    return Effect.tryPromise({
      try:   async () => {
        const wasm = await loadWasm()
        // Build minimal AstPattern JSON from native Pattern<Subject> objects
        const raw = patterns.map(patternToRaw)
        return wasm.stringifyFromJson(JSON.stringify(raw))
      },
      catch: (cause) => new GramParseError({ input: "", cause }),
    })
  },

  /**
   * Validate gram notation syntax.
   *
   * Returns Effect<void, GramParseError>. Succeeds silently if valid.
   */
  validate(input: string): Effect.Effect<void, GramParseError> {
    return pipe(
      Gram.parse(input),
      Effect.map(() => undefined)
    )
  },
}

// --- Internal: native Pattern<Subject> → raw AstPattern JSON shape ---

function patternToRaw(p: Pattern<Subject>): object {
  return {
    subject: {
      identity:   p.value.identity,
      labels:     [...HashSet.values(p.value.labels)],
      properties: Object.fromEntries(
        [...HashMap.entries(p.value.properties)].map(([k, v]) => [k, valueToRaw(v)])
      ),
    },
    elements: p.elements.map(patternToRaw),
  }
}

// Converts a native Value back to the JSON interchange format.
// Mirrors json_to_value in the Rust json.rs module.
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function valueToRaw(v: any): unknown {
  switch (v._tag) {
    case "StringVal":       return v.value
    case "IntVal":          return v.value
    case "FloatVal":        return v.value
    case "BoolVal":         return v.value
    case "NullVal":         return null
    case "SymbolVal":       return { type: "symbol",      value: v.value }
    case "TaggedStringVal": return { type: "tagged",      tag: v.tag, content: v.content }
    case "RangeVal":        return { type: "range",       lower: v.lower, upper: v.upper }
    case "MeasurementVal":  return { type: "measurement", unit: v.unit, value: v.value }
    case "ArrayVal":        return (v.items as ReadonlyArray<unknown>).map(valueToRaw)
    case "MapVal":          return Object.fromEntries(
      Object.entries(v.entries as Record<string, unknown>).map(([k, val]) => [k, valueToRaw(val)])
    )
    default: return null
  }
}
