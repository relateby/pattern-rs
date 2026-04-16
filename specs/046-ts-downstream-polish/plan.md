# Implementation Plan: TypeScript Polish for Downstream Projects

**Branch**: `046-ts-downstream-polish` | **Date**: 2026-04-16 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `specs/046-ts-downstream-polish/spec.md`

## Summary

Remove a large stale WASM type declaration file and replace it with one that accurately reflects the actual adapter surface; expand `@relateby/gram` tests from 3 surface-existence assertions to substantive behavioral coverage; and document the annotation content limitation in `gram-notation.md`. All three changes are in the TypeScript/documentation layer — no Rust changes required.

## Technical Context

**Language/Version**: TypeScript 5.x (all three changes), Markdown (documentation)  
**Primary Dependencies**: `vitest` (existing test runner), `@relateby/pattern` (existing), `effect >= 3.0.0` (peer)  
**Storage**: N/A  
**Testing**: `vitest` — run via `npm test` in each package  
**Target Platform**: npm (browser + Node.js via dual wasm/wasm-node builds)  
**Project Type**: npm library packages  
**Performance Goals**: N/A  
**Constraints**: Must not break any existing test; must not introduce new exports that conflict with generated `.d.ts` files  
**Scale/Scope**: 3 files changed (wasm-types.d.ts replaced, gram test file expanded, gram-notation.md amended)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|---|---|---|
| I. Reference Implementation Fidelity | ✅ PASS | This feature is about the TypeScript packaging layer, not a Rust port. The WASM surface is authoritative from `adapters/wasm/pattern-wasm/src/`. No gram-hs reference needed. |
| II. Correctness & Compatibility | ✅ PASS | Removing stale declarations *improves* correctness. The replacement must exactly mirror the actual generated `.d.ts`. |
| III. Rust Native Idioms | ✅ N/A | No Rust changes. |
| IV. Multi-Target Library Design | ✅ PASS | Both module variants (`../wasm/` and `../wasm-node/`) are identical and must be updated together. |
| V. External Language Bindings & Examples | ✅ PASS | This directly improves the TypeScript consumer experience. |

No violations. No Complexity Tracking table required.

## Project Structure

### Documentation (this feature)

```text
specs/046-ts-downstream-polish/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # N/A — no data model changes
├── contracts/           # N/A — WASM contract already exists in generated .d.ts files
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
typescript/packages/pattern/src/
└── wasm-types.d.ts                    # REPLACE — accurate fallback declarations

typescript/packages/gram/tests/
└── public-api.test.ts                 # EXPAND — substantive behavioral tests

docs/
└── gram-notation.md                   # AMEND — annotation limitation note
```

## Phase 0: Research

### Finding 1 — Actual WASM surface

**Decision**: Replace `wasm-types.d.ts` with declarations that mirror the actual generated `.d.ts` exactly.

**Evidence**: Reading `typescript/packages/pattern/wasm/pattern_wasm.d.ts` (and the identical `wasm-node/` variant) reveals the real surface is:

```typescript
export class Gram {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  static parseToJson(gram: string): string;
  static stringifyFromJson(json: string): string;
  static validate(gram: string): Array<any>;
}

export class ParseResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  readonly identifiers: string[];
  readonly pattern_count: number;
}

export function parse_gram(input: string): ParseResult;
export function parse_to_ast(input: string): any;
export function round_trip(input: string): string;
export function validate_gram(input: string): boolean;
export function version(): string;
```

The current `wasm-types.d.ts` declares none of these accurately — it describes a completely different, obsolete surface from a prior architecture.

**Rationale**: The `wasm-types.d.ts` file provides ambient fallback module declarations for TypeScript compilation before `npm run build:wasm` has run. It must match what wasm-pack generates.

**Alternative considered**: Delete `wasm-types.d.ts` entirely. Rejected because `gram.ts` uses dynamic `import(... as string) as any`, so this is safe, but the file serves as documentation of the WASM surface for developers reading the source.

### Finding 2 — How `gram.ts` uses WASM

**Decision**: `wasm-types.d.ts` does not need to change the type safety of `gram.ts` — it uses a local `WasmGram` interface and dynamic imports with `as any`. The file is referenced only as ambient module declarations for `../wasm/pattern_wasm.js` and `../wasm-node/pattern_wasm.js`.

**Rationale**: Even though no source file does a typed `import ... from "../wasm/pattern_wasm.js"`, the ambient declarations serve as documentation and prevent any future accidental use of the stale API.

### Finding 3 — `@relateby/gram` test scope

**Decision**: Expand `@relateby/gram/tests/public-api.test.ts` to test actual gram operations via the `Gram` namespace re-exported from `@relateby/pattern`.

**Rationale**: The package re-exports `Gram` directly from `@relateby/pattern`. Tests should verify the operations work end-to-end, not just that the export is defined. Since `@relateby/pattern` tests already cover the underlying implementation exhaustively, the `@relateby/gram` tests should focus on the package's unique surface: the `Gram` namespace operations and the `init()` helper.

**Test cases needed**:
1. `Gram.parse` of a simple node returns a pattern array
2. `Gram.parse` of a relationship returns a pattern with elements
3. `Gram.parse` + `Gram.stringify` round-trip (result is valid gram)
4. `Gram.validate` returns success for valid gram
5. `Gram.validate` returns a `GramParseError` for invalid gram
6. `init()` resolves without error
7. `init()` is idempotent (calling twice does not throw)

### Finding 4 — Annotation limitation documentation

**Decision**: Add a "Known Limitations" note directly in the Annotations section of `docs/gram-notation.md`.

**Content**: The note should explain that annotation body content is currently stored as properties on the wrapping subject's `Subject`, so a parse-then-serialize round-trip may not reproduce the original annotation syntax. Future work will add configurable serialization: either inline unary notation (`@@a:L @k(37) (x)`) or property-map notation (`[a:L {k:37} | x]`).

## Phase 1: Design

### 1. `wasm-types.d.ts` replacement

The file has two `declare module` blocks — one for `"../wasm/pattern_wasm.js"` (browser bundler target) and one for `"../wasm-node/pattern_wasm.js"` (Node.js CJS target). The generated `.d.ts` files are identical for both targets, so both blocks will be identical.

Replace the entire file with accurate declarations that mirror `wasm/pattern_wasm.d.ts`. Keep the comment header explaining the fallback purpose. Remove all legacy interface types (`PatternWasmSubjectLike`, `PatternWasmPatternLike`, etc.) that were only used by the now-deleted declarations.

### 2. `@relateby/gram` test expansion

The existing test file is 12 lines. Expand it to cover the 7 test cases from Finding 3. Tests use `Effect.runPromise` to await Effect-returning operations. Import `GramParseError` from `@relateby/pattern` to test error cases.

Structure:
```
describe("@relateby/gram public API")
  it("exports Gram and init")            ← existing, keep
  describe("Gram.parse")
    it("parses a node")
    it("parses a relationship")
  describe("Gram.stringify")
    it("round-trips a parsed pattern")
  describe("Gram.validate")
    it("succeeds for valid gram")
    it("fails for invalid gram")
  describe("init()")
    it("resolves without error")
    it("is idempotent")
```

### 3. `gram-notation.md` annotation note

Add a "Known Limitation" callout directly after the existing Annotations section description, before the summary mapping table. The note is prose — no code changes required.

## Agent Context Update

Run after Phase 1 design is complete:
```bash
.specify/scripts/bash/update-agent-context.sh claude
```

No new technologies are introduced by this feature; the agent context update is a housekeeping step only.
