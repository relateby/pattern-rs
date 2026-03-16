interface GramModule {
  parse(input: string): unknown;
  stringify(value: unknown): string;
}

let gramModule: GramModule | null = null;

async function loadGram(): Promise<GramModule> {
  if (gramModule !== null) return gramModule;

  const { init } = await import("./index.js");
  await init();

  try {
    const isNode = typeof process !== "undefined" &&
      process.versions != null &&
      process.versions.node != null;

    if (isNode) {
      const { createRequire } = await import("module");
      const { fileURLToPath } = await import("url");
      const { dirname, resolve } = await import("path");
      const __filename = fileURLToPath(import.meta.url);
      const __dirname = dirname(__filename);
      const require = createRequire(import.meta.url);
      const wasmNodePath = resolve(__dirname, "./wasm-node/pattern_wasm.js");
      const mod = require(wasmNodePath) as { Gram?: GramModule };
      if (mod.Gram) {
        gramModule = mod.Gram;
        return gramModule;
      }
    } else {
      const wasmPath = "./wasm/pattern_wasm.js";
      const mod = await import(/* @vite-ignore */ wasmPath) as { Gram?: GramModule };
      if (mod.Gram) {
        gramModule = mod.Gram;
        return gramModule;
      }
    }
  } catch {
    // fall through to stub
  }

  gramModule = {
    parse: () => {
      throw new Error("Gram.parse: WASM module not loaded. Call await init() first.");
    },
    stringify: () => {
      throw new Error("Gram.stringify: WASM module not loaded. Call await init() first.");
    },
  };
  return gramModule;
}

export const Gram = {
  async parse(input: string): Promise<unknown> {
    const g = await loadGram();
    return g.parse(input);
  },

  async stringify(value: unknown): Promise<string> {
    const g = await loadGram();
    return g.stringify(value);
  },
};
