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
  parse(input: string): unknown
  stringify(patterns: unknown): string
  validate(input: string): string[]
  parseWithHeader(input: string): unknown
  stringifyWithHeader(input: unknown): string
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
      // Primary path: CJS require() — works in pure Node.js without Vite.
      try {
        const { createRequire } = await import("module")
        const { fileURLToPath } = await import("url")
        const { dirname, resolve } = await import("path")
        const __filename = fileURLToPath(import.meta.url)
        const __dirname = dirname(__filename)
        const require = createRequire(import.meta.url)
        const candidatePaths = [
          resolve(__dirname, "./wasm-node/pattern_wasm.js"),
          resolve(__dirname, "../wasm-node/pattern_wasm.js"),
        ]
        for (const wasmPath of candidatePaths) {
          try {
            const mod = require(wasmPath) as { Gram?: WasmGram }
            if (mod.Gram) {
              wasmGram = mod.Gram as WasmGram
              return wasmGram
            }
          } catch {
            // try the next candidate path
          }
        }
      } catch {
        // fileURLToPath failed because Vite transformed import.meta.url to a
        // non-file URL (e.g. an HTTP URL in dev mode, or a Vite dep-cache URL).
        // Fall through to the ESM-based fallback below.
      }

      // Fallback path: ESM dynamic import — works in Vite/vitest environments
      // where createRequire/fileURLToPath cannot resolve the wasm-node module.
      // Vite correctly resolves relative imports against the original source
      // location even when import.meta.url has been transformed.
      for (const relPath of [
        "./wasm-node/pattern_wasm.js",
        "../wasm-node/pattern_wasm.js",
      ]) {
        try {
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          const mod = await import(/* @vite-ignore */ relPath as string) as any
          // CJS interop: Vite exposes named exports directly (mod.Gram);
          // pure Node.js ESM puts module.exports on .default (mod.default.Gram).
          // eslint-disable-next-line @typescript-eslint/no-explicit-any
          const gram = (mod?.Gram ?? (mod?.default as any)?.Gram) as WasmGram | undefined
          if (gram) {
            wasmGram = gram
            return wasmGram
          }
        } catch {
          // try the next candidate path
        }
      }
    } else {
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const mod = await import(/* @vite-ignore */ "./wasm/pattern_wasm.js" as string) as any
      // wasm-pack bundler-target modules export an async init() as the default
      // export.  Call it explicitly to ensure the WASM binary is instantiated
      // in environments where the bundler did not auto-initialize it
      // (e.g. vitest + vite-plugin-wasm, or any env where isNode evaluated to
      // false due to Vite replacing process.versions.node for a browser build).
      if (typeof mod.default === "function") {
        await (mod.default as () => Promise<unknown>)()
      }
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
  wasmGram = {
    parse: unavailable,
    stringify: unavailable,
    validate: () => unavailable(),
    parseWithHeader: unavailable,
    stringifyWithHeader: unavailable,
  }
  return wasmGram
}

// --- Public API ---

export const Gram = {
  /**
   * Parse gram notation into an array of `Pattern<Subject>`.
   *
   * Each top-level element in the gram document becomes one entry in the
   * returned array.  An empty string returns an empty array.
   *
   * @param input - Gram notation string, e.g. `"(alice:Person)-[:KNOWS]->(bob:Person)"`.
   * @returns `Effect<ReadonlyArray<Pattern<Subject>>, GramParseError>`.
   *   Use `Effect.runPromise` to convert to a `Promise`.
   *
   * @example
   * ```ts
   * import { Gram } from "@relateby/pattern"
   * import { Effect } from "effect"
   *
   * const patterns = await Effect.runPromise(
   *   Gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)")
   * )
   * // patterns[0].value.identity === "alice" (via the walk's subject)
   *
   * // round-trip
   * const gram = await Effect.runPromise(Gram.stringify(patterns))
   * ```
   */
  parse(input: string): Effect.Effect<ReadonlyArray<Pattern<Subject>>, GramParseError> {
    return pipe(
      Effect.tryPromise({
        try:   async () => {
          const wasm = await loadWasm()
          return wasm.parse(input) as unknown
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
   * Serialize an array of `Pattern<Subject>` to gram notation.
   *
   * @param patterns - Patterns to serialize, typically the result of
   *   {@link Gram.parse} or {@link Gram.parseWithHeader}.
   * @returns `Effect<string, GramParseError>`.
   *
   * @example
   * ```ts
   * import { Gram } from "@relateby/pattern"
   * import { Effect } from "effect"
   *
   * const patterns = await Effect.runPromise(Gram.parse("(a)-->(b)"))
   * const gram     = await Effect.runPromise(Gram.stringify(patterns))
   * // "(a)-->(b)"
   * ```
   */
  stringify(
    patterns: ReadonlyArray<Pattern<Subject>>
  ): Effect.Effect<string, GramParseError> {
    return Effect.tryPromise({
      try:   async () => {
        const wasm = await loadWasm()
        const raw = patterns.map(patternToRaw)
        return wasm.stringify(raw)
      },
      catch: (cause) => new GramParseError({ input: "", cause }),
    })
  },

  /**
   * Validate gram notation syntax without constructing patterns.
   *
   * @param input - Gram notation string to validate.
   * @returns `Effect<void, GramParseError>`. Succeeds silently when *input*
   *   is valid; fails with {@link GramParseError} otherwise.
   *
   * @example
   * ```ts
   * import { Gram } from "@relateby/pattern"
   * import { Effect, Either } from "effect"
   *
   * const result = await Effect.runPromise(
   *   Effect.either(Gram.validate("(alice:Person)"))
   * )
   * // Either.isRight(result) === true
   * ```
   */
  validate(input: string): Effect.Effect<void, GramParseError> {
    return pipe(
      Effect.tryPromise({
        try:   async () => {
          const wasm = await loadWasm()
          return wasm.validate(input)
        },
        catch: (cause) => new GramParseError({ input, cause }),
      }),
      Effect.flatMap((errors) =>
        errors.length === 0
          ? Effect.succeed(undefined)
          : Effect.fail(new GramParseError({ input, cause: errors.join("; ") }))
      )
    )
  },

  /**
   * Parse gram notation, separating an optional leading header record from
   * the patterns.
   *
   * A *header* is a bare record — a `{key: value, ...}` block that appears
   * before any graph elements and has no identity or labels.  It is commonly
   * used to store document-level metadata such as schema version or provenance.
   *
   * @param input - Gram notation string, optionally starting with a bare
   *   record header, e.g. `"{version: 1} (alice)-[:KNOWS]->(bob)"`.
   * @returns `Effect<{ header: Record<string, unknown> | undefined, patterns: ReadonlyArray<Pattern<Subject>> }, GramParseError>`.
   *   `header` is `undefined` when no leading bare record is present.
   *
   * @example
   * ```ts
   * import { Gram } from "@relateby/pattern"
   * import { Effect } from "effect"
   *
   * // Document with a header
   * const { header, patterns } = await Effect.runPromise(
   *   Gram.parseWithHeader("{version: 1, source: 'export'} (alice)-[:KNOWS]->(bob)")
   * )
   * // header   → { version: 1, source: "export" }
   * // patterns → [Pattern<Subject>]
   *
   * // Document without a header
   * const { header: h2 } = await Effect.runPromise(
   *   Gram.parseWithHeader("(alice)-[:KNOWS]->(bob)")
   * )
   * // h2 → undefined
   * ```
   */
  parseWithHeader(
    input: string
  ): Effect.Effect<
    { header: Record<string, unknown> | undefined; patterns: ReadonlyArray<Pattern<Subject>> },
    GramParseError
  > {
    return pipe(
      Effect.tryPromise({
        try:   async () => {
          const wasm = await loadWasm()
          return wasm.parseWithHeader(input) as { header: Record<string, unknown> | null; patterns: unknown[] }
        },
        catch: (cause) => new GramParseError({ input, cause }),
      }),
      Effect.flatMap((result) =>
        Effect.try({
          try: () => {
            const header: Record<string, unknown> | undefined =
              result.header == null ? undefined : result.header
            const patterns = decodePayload(result.patterns).map(patternFromRaw)
            return { header, patterns }
          },
          catch: (cause) => new GramParseError({ input, cause }),
        })
      )
    )
  },

  /**
   * Serialize a header record and `Pattern<Subject>` array to gram notation.
   *
   * Produces a gram document whose first element is the header bare record
   * followed by the serialized patterns.  Pass `undefined` as *header* to
   * produce output identical to {@link Gram.stringify}.
   *
   * @param header - Plain object of scalar values to write as the leading
   *   bare record, or `undefined` to omit the header entirely.
   * @param patterns - Patterns to serialize.
   * @returns `Effect<string, GramParseError>`.
   *
   * @example
   * ```ts
   * import { Gram } from "@relateby/pattern"
   * import { Effect } from "effect"
   *
   * const patterns = await Effect.runPromise(Gram.parse("(alice)-[:KNOWS]->(bob)"))
   *
   * const gram = await Effect.runPromise(
   *   Gram.stringifyWithHeader({ version: 1 }, patterns)
   * )
   * // "{version: 1}\n(alice)-[:KNOWS]->(bob)"
   *
   * // Omitting the header is equivalent to plain stringify
   * const gramNoHeader = await Effect.runPromise(
   *   Gram.stringifyWithHeader(undefined, patterns)
   * )
   * // "(alice)-[:KNOWS]->(bob)"
   *
   * // Full round-trip
   * const { header: h2, patterns: p2 } = await Effect.runPromise(
   *   Gram.parseWithHeader(gram)
   * )
   * // h2 → { version: 1 }
   * ```
   */
  stringifyWithHeader(
    header: Record<string, unknown> | undefined,
    patterns: ReadonlyArray<Pattern<Subject>>
  ): Effect.Effect<string, GramParseError> {
    return Effect.tryPromise({
      try:   async () => {
        const wasm = await loadWasm()
        const raw = {
          header: header ?? null,
          patterns: patterns.map(patternToRaw),
        }
        return wasm.stringifyWithHeader(raw)
      },
      catch: (cause) => new GramParseError({ input: "", cause }),
    })
  },
}

// --- Internal: native Pattern<Subject> → raw AstPattern JSON shape ---

function patternToRaw(p: Pattern<Subject>): object {
  return {
    subject: {
      identity:   p.value.identity,
      labels:     [...HashSet.values(p.value.labels)].sort(),
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
    case "NullVal":         throw new Error("JSON null is not representable as a gram value")
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
