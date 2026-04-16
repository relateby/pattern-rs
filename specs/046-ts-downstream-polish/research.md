# Research: TypeScript Polish for Downstream Projects

**Feature**: 046-ts-downstream-polish  
**Date**: 2026-04-16  
**Status**: Complete — no NEEDS CLARIFICATION markers remain

## Finding 1 — Actual WASM Adapter Surface

**Decision**: Replace `wasm-types.d.ts` with declarations matching the actual wasm-pack-generated `.d.ts` files.

**Evidence**: `typescript/packages/pattern/wasm/pattern_wasm.d.ts` (identical to the `wasm-node/` variant) declares:

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

The Rust source `adapters/wasm/pattern-wasm/src/gram.rs` only defines `Gram`, but `gram_codec` also contributes `#[wasm_bindgen]`-tagged items (`ParseResult`, `parse_gram`, `parse_to_ast`, `round_trip`, `validate_gram`, `version`) that end up in the combined WASM output.

**Rationale**: The fallback declaration file must mirror what wasm-pack generates. Anything less causes a confusing mismatch between what's documented and what's available.

**Alternatives considered**: Delete `wasm-types.d.ts` entirely. Not chosen because the file provides documentation value and acts as a type fallback before `npm run build:wasm` is run.

---

## Finding 2 — How `gram.ts` Uses WASM

**Decision**: The `wasm-types.d.ts` replacement does not need to change type-checking of `gram.ts`.

**Evidence**: `typescript/packages/pattern/src/gram.ts` loads WASM via runtime dynamic import with `as any` and uses a local `WasmGram` interface for type safety within the module. It never does a statically typed `import` from `../wasm/pattern_wasm.js`. The `wasm-types.d.ts` ambient declarations are therefore not in the type-checking critical path.

**Rationale**: The replacement is purely a documentation/correctness improvement. It will not cause type errors in any existing source file.

---

## Finding 3 — `@relateby/gram` Test Scope

**Decision**: Expand `typescript/packages/gram/tests/public-api.test.ts` to test 7 behavioral cases.

**Evidence**: Current test has 3 assertions — all existence checks (`expect(Gram).toBeDefined()`, `typeof Gram.parse === "function"`, `typeof Gram.stringify === "function"`). The test does not exercise WASM at all.

**Test cases to add**:

| # | Description | Method under test |
|---|---|---|
| 1 | Parse a simple node `(a)` → single-element array | `Gram.parse` |
| 2 | Parse a relationship `(a)-->(b)` → pattern with 2 elements | `Gram.parse` |
| 3 | Round-trip: parse then stringify returns non-empty string | `Gram.parse` + `Gram.stringify` |
| 4 | Validate valid gram → Effect succeeds | `Gram.validate` |
| 5 | Validate invalid gram → Effect fails with `GramParseError` | `Gram.validate` |
| 6 | `init()` resolves without error | `init` |
| 7 | `init()` called twice — no error on second call | `init` |

**Rationale**: The `@relateby/gram` package is a thin re-export, but its consumers don't see that. The tests should verify the operations work end-to-end from this package's import path.

---

## Finding 4 — Annotation Limitation Documentation

**Decision**: Add a "Known Limitation" prose note in the Annotations section of `docs/gram-notation.md`.

**Evidence**: From gram-codec source investigation: when a pattern of the form `@a:L @k(37) (x)` is parsed, the annotation content (the annotating subject's identity/labels/properties) is stored in the wrapping `Subject`'s properties map, not in a dedicated annotation field. This means serialize(parse(input)) may not reproduce the original annotation syntax.

**Content to add**: Placed directly after the existing annotation example, before the summary table:

> **Known Limitation — Annotation Content Preservation**  
> The current parser stores annotation body content (identity, labels, and properties of the annotating subject) as properties on the wrapping subject. This means a parse-then-serialize round-trip may not reproduce the original annotation syntax.
>
> Future work will add configurable annotation serialization: either inline unary notation (`@@a:L @k(37) (x)`) or property-map notation (`[a:L {k:37} | x]`), selectable at serialize time.

**Rationale**: Documents a known behaviour gap so downstream consumers are not surprised. The note also previews the planned fix without overpromising delivery timing.

---

## Resolution Summary

All research questions resolved. No NEEDS CLARIFICATION markers remain. Proceeding to Phase 1 (implementation via `/speckit.tasks`).
