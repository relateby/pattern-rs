// gram.ts — Promise-based parse/stringify interface
//
// All methods return Promise<T>, rejecting with GramParseError on failure.
// Effect users can wrap these via @relateby/pattern-effect.

import { GramParseError } from "./errors.js"
import { Pattern } from "./pattern.js"
import { Subject } from "./subject.js"
import { validatePayload, patternFromRaw } from "./schema.js"

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
   * @returns `Promise<ReadonlyArray<Pattern<Subject>>>`, rejecting with `GramParseError` on failure.
   *
   * @example
   * ```ts
   * import { Gram } from "@relateby/pattern"
   *
   * const patterns = await Gram.parse("(alice:Person)-[:KNOWS]->(bob:Person)")
   * const gram = await Gram.stringify(patterns)
   * ```
   */
  async parse(input: string): Promise<ReadonlyArray<Pattern<Subject>>> {
    try {
      const wasm = await loadWasm()
      const raw = wasm.parse(input)
      return validatePayload(raw).map(patternFromRaw)
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input, cause })
    }
  },

  /**
   * Serialize an array of `Pattern<Subject>` to gram notation.
   *
   * @param patterns - Patterns to serialize.
   * @returns `Promise<string>`, rejecting with `GramParseError` on failure.
   *
   * @example
   * ```ts
   * import { Gram } from "@relateby/pattern"
   *
   * const patterns = await Gram.parse("(a)-->(b)")
   * const gram = await Gram.stringify(patterns)
   * ```
   */
  async stringify(patterns: ReadonlyArray<Pattern<Subject>>): Promise<string> {
    try {
      const wasm = await loadWasm()
      return wasm.stringify(patterns.map(patternToRaw))
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input: "(stringify)", cause })
    }
  },

  /**
   * Validate gram notation syntax without constructing patterns.
   *
   * @param input - Gram notation string to validate.
   * @returns `Promise<void>`, rejecting with `GramParseError` when input is invalid.
   */
  async validate(input: string): Promise<void> {
    try {
      const wasm = await loadWasm()
      const errors = wasm.validate(input) as string[]
      if (errors.length > 0) {
        throw new GramParseError({ input, cause: errors.join("; ") })
      }
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input, cause })
    }
  },

  /**
   * Parse gram notation, separating an optional leading header record from
   * the patterns.
   *
   * @param input - Gram notation string, optionally starting with a bare record header.
   * @returns `Promise<{ header, patterns }>`, rejecting with `GramParseError` on failure.
   *
   * @example
   * ```ts
   * import { Gram } from "@relateby/pattern"
   *
   * const { header, patterns } = await Gram.parseWithHeader("{version: 1} (alice)-[:KNOWS]->(bob)")
   * ```
   */
  async parseWithHeader(input: string): Promise<{
    header: Record<string, unknown> | undefined
    patterns: ReadonlyArray<Pattern<Subject>>
  }> {
    try {
      const wasm = await loadWasm()
      const result = wasm.parseWithHeader(input) as { header: Record<string, unknown> | null; patterns: unknown[] }
      const header = result.header == null ? undefined : result.header
      const patterns = validatePayload(result.patterns).map(patternFromRaw)
      return { header, patterns }
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input, cause })
    }
  },

  /**
   * Serialize a header record and `Pattern<Subject>` array to gram notation.
   *
   * @param header - Plain object to write as the leading bare record, or `undefined` to omit.
   * @param patterns - Patterns to serialize.
   * @returns `Promise<string>`, rejecting with `GramParseError` on failure.
   *
   * @example
   * ```ts
   * import { Gram } from "@relateby/pattern"
   *
   * const patterns = await Gram.parse("(alice)-[:KNOWS]->(bob)")
   * const gram = await Gram.stringifyWithHeader({ version: 1 }, patterns)
   * ```
   */
  async stringifyWithHeader(
    header: Record<string, unknown> | undefined,
    patterns: ReadonlyArray<Pattern<Subject>>,
  ): Promise<string> {
    try {
      const wasm = await loadWasm()
      return wasm.stringifyWithHeader({ header: header ?? null, patterns: patterns.map(patternToRaw) })
    } catch (cause) {
      throw cause instanceof GramParseError ? cause : new GramParseError({ input: "(stringifyWithHeader)", cause })
    }
  },
}

// --- Internal: native Pattern<Subject> → raw AstPattern JSON shape ---

function patternToRaw(p: Pattern<Subject>): object {
  return {
    subject: {
      identity:   p.value.identity,
      labels:     [...p.value.labels].sort(),
      properties: Object.fromEntries(
        Object.entries(p.value.properties).map(([k, v]) => [k, valueToRaw(v)])
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
