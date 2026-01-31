# Unified WASM / JavaScript API Contract

**Feature**: 028-unified-gram-wasm  
**Date**: 2026-01-31

## Scope

This contract defines the **single entry point** for the pattern-wasm package. Consumers import Pattern, Subject, Value, and Gram from one module. Pattern, Subject, and Value semantics align with 027-wasm-pattern-typescript-parity (see [wasm-api.md](../../027-wasm-pattern-typescript-parity/contracts/wasm-api.md)); this document adds the **Gram namespace** and **single-module** requirements.

## Module: pattern_wasm (single entry)

JavaScript/TypeScript consumers load the WASM module (e.g. wasm-pack output or init). The following MUST be available from one import surface (e.g. `import { Pattern, Subject, Value, Gram } from 'gram'` or equivalent).

### Pattern, Subject, Value

- Same API as 027 contract: Pattern (point, of, pattern, fromValues; value, elements; map, fold, para, validate, etc.), Subject (new, identity, labels, properties), Value (string, int, decimal, boolean, etc.).
- **Subject.fromValue(value, options?)**: Convert an arbitrary JS value to a Subject using the conventional mapping (string, number, boolean, object, Subject passthrough). Options: label, valueProperty, identity (e.g. (value, index) => string). Reusable for building patterns; Gram.from uses it under the hood.
- pattern-wasm re-exports or wraps pattern-core WASM bindings so that one package provides these types.

### Gram (namespace)

**stringify**:
- `Gram.stringify(pattern: Pattern<Subject>): string` — Serialize a single pattern to gram notation. MUST accept only Pattern&lt;Subject&gt; (or equivalent JS type). MUST NOT expose internal AST.
- `Gram.stringify(patterns: Pattern<Subject>[]): string` — Serialize multiple patterns to gram notation (e.g. concatenated or document form per gram-codec).

**parse**:
- `Gram.parse(gram: string): Pattern<Subject>[]` — Parse gram notation into one or more patterns. Returns array; empty or whitespace-only input MUST return empty array `[]`. Invalid input MUST report a clear error (e.g. throw with message or return Result-like); MUST NOT expose internal parser structures.

**parseOne**:
- `Gram.parseOne(gram: string): Pattern<Subject> | null` — Parse gram notation and return the first pattern, or null if none (e.g. empty/whitespace or no top-level pattern). Invalid input: same error behavior as parse.

**from** (conventional conversion):
- `Gram.from<V>(pattern: Pattern<V>, options?: FromOptions): Pattern<Subject>` — Convert Pattern&lt;V&gt; to Pattern&lt;Subject&gt;. **Implementation**: `pattern.map(v => Subject.fromValue(v, options))`. Options passed through to Subject.fromValue; defaults per data-model (String, Number, Boolean, Object, Subject passthrough).

### Error and empty-input behavior

- **Empty or whitespace-only parse input**: `parse` returns `[]`; `parseOne` returns `null`. No throw.
- **Invalid gram notation**: Clear error (e.g. thrown Error with message, or Result/Either); MUST NOT expose internal AST or parser types.
- **stringify with non-Subject pattern**: MUST reject (e.g. type check or runtime error) or document that only Pattern&lt;Subject&gt; is accepted; require use of Gram.from for other patterns.

## Round-trip

- For any valid Pattern&lt;Subject&gt; p: `Gram.parse(Gram.stringify(p))` MUST yield an array whose first element is structurally and semantically equivalent to p (per spec SC-003). Equivalence is defined by pattern-core semantics (identity, labels, properties, element structure).

## Non-goals (out of scope for this contract)

- Exposing gram-codec AST (GramPattern, GramNode, etc.) to JS.
- REST/HTTP endpoints; this is a WASM/JS API only.
