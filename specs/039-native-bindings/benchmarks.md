# Phase 8 Benchmarks

## WASM Size

Measured on the `pattern-wasm` node target before and after removing the `pattern-core` exports:

| Artifact | Before | After | Reduction |
|---|---:|---:|---:|
| `typescript/@relateby/pattern/wasm-node/pattern_wasm_bg.wasm` | 628,170 bytes | 309,597 bytes | 50.7% |
| `typescript/@relateby/pattern/wasm/pattern_wasm_bg.wasm` | 577,373 bytes | 309,597 bytes | 46.4% |

Result: SC-004 met.

## TypeScript Fold

Benchmark: `fold` over a 10,000-node tree of `Pattern<Subject>` values.

- Baseline: pre-cutover WASM bridge built from `HEAD` and invoked through `WasmPattern.fold(...)`
- Native: current `@relateby/pattern` build using the pure TypeScript `fold(...)` helper
- Reducer: sum of `subject.identity.length`

| Implementation | Mean time |
|---|---:|
| WASM bridge baseline | 4.09 ms |
| Native TypeScript | 0.40 ms |

Observed speedup: 10.30x

Result: SC-002 met.

## Python Fold

Benchmark: `fold` over a 1,000-node tree of `Pattern[Subject]` values.

- Baseline: pre-cutover `pattern_core` wheel built from `HEAD`, with each run converting the native dataclass tree into PyO3 `pattern_core.Subject` / `pattern_core.Pattern` objects before calling `fold`
- Native: current pure-Python `relateby.pattern.Pattern.fold(...)`
- Reducer: sum of `subject.identity` string lengths

| Implementation | Mean time |
|---|---:|
| PyO3 round-trip baseline | 3.22 ms |
| Native Python | 0.17 ms |

Observed speedup: 19.07x

Result: SC-003 met.
