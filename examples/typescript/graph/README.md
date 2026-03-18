# `relateby-graph` Example

Demonstrates using native `Pattern` / `Subject` values from `@relateby/pattern` together with the pure TypeScript graph transforms exported from `@relateby/graph`.

## Prerequisites

```bash
npm install
npm run build --workspace=@relateby/pattern
npm run build --workspace=@relateby/graph

cd examples/typescript/graph
npm install
```

## Running

```bash
npm start
```

The example constructs a small in-memory graph, converts it to a `GraphView`, filters nodes, and maps node values without crossing a WASM boundary.
