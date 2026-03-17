interface GramModule {
  parse(input: string): unknown;
  parseOne(input: string): unknown;
  stringify(value: unknown): string;
}

let gramModule: GramModule | null = null;

function gramOperationError(operation: string, error: unknown): Error {
  const message = error instanceof Error ? error.message : String(error);
  return new Error(`@relateby/pattern ${operation} failed: ${message}`);
}

async function loadGram(): Promise<GramModule> {
  if (gramModule !== null) return gramModule;

  const { init } = await import("./index.js");
  await init();
  const nodeModulePath = "./wasm-node/pattern_wasm.js";
  const browserModulePath = "./wasm/pattern_wasm.js";
  const unavailableMessage =
    "Gram bindings are unavailable after init(); expected a Gram export from " +
    `${nodeModulePath} (Node) or ${browserModulePath} (browser/bundler).`;

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
      const wasmNodePath = resolve(__dirname, nodeModulePath);
      const mod = require(wasmNodePath) as { Gram?: GramModule };
      if (mod.Gram) {
        gramModule = mod.Gram;
        return gramModule;
      }
    } else {
      const mod = await import(/* @vite-ignore */ browserModulePath) as { Gram?: GramModule };
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
      throw new Error(`Gram.parse: ${unavailableMessage}`);
    },
    parseOne: () => {
      throw new Error(`Gram.parseOne: ${unavailableMessage}`);
    },
    stringify: () => {
      throw new Error(`Gram.stringify: ${unavailableMessage}`);
    },
  };
  return gramModule;
}

export const Gram = {
  async parse(input: string): Promise<unknown> {
    try {
      const g = await loadGram();
      return g.parse(input);
    } catch (error) {
      throw gramOperationError("Gram.parse", error);
    }
  },

  async parseOne(input: string): Promise<unknown> {
    try {
      const g = await loadGram();
      return g.parseOne(input);
    } catch (error) {
      throw gramOperationError("Gram.parseOne", error);
    }
  },

  async stringify(value: unknown): Promise<string> {
    try {
      const g = await loadGram();
      return g.stringify(value);
    } catch (error) {
      throw gramOperationError("Gram.stringify", error);
    }
  },
};
