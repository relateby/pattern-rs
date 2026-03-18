# `relateby-graph` Example

Demonstrates using native `Pattern` / `Subject` values from `@relateby/pattern` together with the pure TypeScript graph transforms from `@relateby/graph`.

## Prerequisites

```bash
cd typescript/@relateby/pattern
npm install
npm run build

cd ../graph
npm install
npm run build

cd ../../examples/relateby-graph
npm install
```

## Running

```bash
node node.mjs
```

The example constructs a small in-memory graph, converts it to a `GraphView`, filters nodes, and maps node values without crossing a WASM boundary.
