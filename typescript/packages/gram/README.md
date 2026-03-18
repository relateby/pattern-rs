# `@relateby/gram`

WASM-backed Gram notation parsing and serialization for relateby.

## Install

```bash
npm install @relateby/gram @relateby/pattern effect
```

## Package Role

`@relateby/gram` exposes the Gram codec as a focused package while reusing the `@relateby/pattern` WASM initialization flow.

## Quick Start

```typescript
import { Effect } from "effect"
import { init, Gram } from "@relateby/gram"

await init()
const parsed = await Effect.runPromise(Gram.parse("(alice:Person)"))
const rendered = await Effect.runPromise(Gram.stringify(parsed))
console.log(rendered)
```
