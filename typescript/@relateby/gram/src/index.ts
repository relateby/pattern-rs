// @relateby/gram — Gram notation codec
//
// Provides Gram.parse and Gram.stringify via the WASM-backed Gram codec.
// Requires @relateby/pattern to be initialized first (call await init() from @relateby/pattern).

import { init as patternInit } from "@relateby/pattern";

// Re-export init for convenience
export { init } from "@relateby/pattern";

// ---------------------------------------------------------------------------
// Gram namespace
// ---------------------------------------------------------------------------

interface GramModule {
  parse(input: string): unknown;
  stringify(value: unknown): string;
}

let gramModule: GramModule | null = null;

async function loadGram(): Promise<GramModule> {
  if (gramModule !== null) return gramModule;

  // Ensure pattern WASM is initialized
  await patternInit();

  // Access the Gram module from the global WASM exports
  // After init(), the WASM module is loaded and Gram is available
  // We use a dynamic import with a variable to avoid TypeScript path resolution
  const wasmPath = "@relateby/pattern/wasm/pattern_wasm.js";
  try {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const mod = await import(/* @vite-ignore */ wasmPath) as { Gram?: GramModule };
    if (mod.Gram) {
      gramModule = mod.Gram;
      return gramModule;
    }
  } catch {
    // Fall through to stub
  }

  // Fallback stub
  gramModule = {
    parse: (_input: string): unknown => {
      throw new Error("Gram.parse: WASM module not loaded. Call await init() first.");
    },
    stringify: (_value: unknown): string => {
      throw new Error("Gram.stringify: WASM module not loaded. Call await init() first.");
    },
  };
  return gramModule;
}

/**
 * Gram notation codec.
 *
 * Requires WASM initialization. Call `await init()` from `@relateby/pattern` first,
 * or use the `init` re-export from this module.
 */
export const Gram = {
  /**
   * Parse Gram notation string into a Pattern structure.
   *
   * @param input - Gram notation string
   * @returns Parsed pattern structure
   */
  async parse(input: string): Promise<unknown> {
    const g = await loadGram();
    return g.parse(input);
  },

  /**
   * Serialize a Pattern structure to Gram notation string.
   *
   * @param value - Pattern structure to serialize
   * @returns Gram notation string
   */
  async stringify(value: unknown): Promise<string> {
    const g = await loadGram();
    return g.stringify(value);
  },
};
