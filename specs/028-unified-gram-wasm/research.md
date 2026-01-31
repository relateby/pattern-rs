# Research: Unified Gram WASM Package

**Feature**: 028-unified-gram-wasm  
**Date**: 2026-01-31

## 1. Single WASM crate depending on two WASM-capable crates

**Decision**: Add a new crate **pattern-wasm** that depends on `pattern-core` and `gram-codec` with their `wasm` features enabled. Build a single `.wasm` binary and one JS/TS entry point. The crate name reflects that the pattern data structure is the dominant feature.

**Rationale**: wasm-pack and Cargo support transitive WASM dependencies; the resulting module is one artifact. Consumers import one package instead of wiring pattern-core and gram-codec separately. Existing pattern-core and gram-codec WASM bindings can remain for advanced use; pattern-wasm is the recommended single entry for the “Pattern + gram notation” workflow.

**Alternatives considered**:
- Publish two packages and document “use both”: Rejected; spec requires one consumable artifact.
- Merge pattern-core and gram-codec WASM into one of the crates: Rejected; would blur crate boundaries and duplicate codec logic.

## 2. Exposing Gram namespace (stringify, parse, parseOne, from) from Rust/WASM

**Decision**: Implement a `Gram` object or namespace in Rust and expose it via wasm-bindgen (e.g. `#[wasm_bindgen] impl Gram { ... }` or module-level `stringify`/`parse` functions grouped under one JS object). JS surface: `Gram.stringify(pattern)`, `Gram.parse(text)`, `Gram.parseOne(text)`, `Gram.from(pattern, options?)`.

**Rationale**: Matches spec and unified-wasm-spec.md; mirrors JSON.stringify/parse naming. gram-codec already provides `parse_gram_notation()` → `Vec<Pattern<Subject>>` and `to_gram_pattern(&Pattern<Subject>)` → `String`; pattern-wasm wraps these and maps Rust types to/from existing pattern-core WASM types (Pattern, Subject).

**Alternatives considered**:
- Free functions only (e.g. `gramStringify`, `gramParse`): Rejected; spec and design doc require a Gram namespace for familiarity.

## 3. Converting Rust Pattern&lt;Subject&gt; to JS Pattern/Subject for parse return

**Decision**: Implement a conversion layer in pattern-wasm that, given Rust `Vec<Pattern<Subject>>` from `gram_codec::parse_gram_notation()`, builds the equivalent JS Pattern and Subject instances using the same constructors that pattern-core’s WASM exposes (Subject.new, Pattern.point, Pattern.pattern). No new gram-codec WASM API for full Pattern&lt;Subject&gt; required; conversion lives in pattern-wasm.

**Rationale**: gram-codec’s current WASM API returns a summary (e.g. ParseResult); it does not expose full Pattern&lt;Subject&gt; to JS. pattern-wasm already depends on pattern-core and gram-codec as Rust libraries, so it can call `parse_gram_notation()` and then recursively construct JS Pattern/Subject via pattern-core’s WASM bindings or by reimplementing the same shape in pattern-wasm’s own exports. Preferred: use pattern-core’s Subject/Pattern constructors from pattern-wasm so a single JS type system (Pattern, Subject, Value) is used.

**Alternatives considered**:
- Extend gram-codec WASM to return full Pattern&lt;Subject&gt;: Possible but pushes conversion into gram-codec and duplicates pattern-core’s JS shape there; keeping conversion in pattern-wasm keeps “one place that knows the JS surface” and leaves gram-codec focused on Rust API.

## 4. Conventional conversion Pattern&lt;V&gt; → Pattern&lt;Subject&gt; (Subject.fromValue + Gram.from)

**Decision**: Implement **Subject.fromValue(value, options?)** in pattern-wasm (or pattern-core WASM surface) to own the convention: turn arbitrary JS values (string, number, boolean, object, Subject) into a Subject with default identity generation, type-based labels (e.g. "String", "Number"), and value property (or passthrough for Subject). Options: label, valueProperty, identity (e.g. (value, index) => string). **Gram.from(pattern, options?)** is implemented as `pattern.map(v => Subject.fromValue(v, options))`, so the convention lives on Subject and Gram.from is convenience sugar. Users can also do `pattern.map(Subject.fromValue)` then `Gram.stringify(...)` explicitly.

**Rationale**: Subject is the value type with identity, labels, properties; "turn this JS value into a Subject" belongs on Subject. Gram stays focused on stringify/parse. Subject.fromValue is reusable (e.g. when building patterns step by step) without going through Gram. Spec FR-006 and User Story 3 are satisfied by Gram.from; the implementation delegates to Subject.fromValue.

**Alternatives considered**:
- Only Gram.from owning the convention: Rejected; conversion is a Subject concern and more reusable on Subject.
- Only document manual mapping: Rejected; spec requires a supported conversion.

## 5. TypeScript definitions and single entry point

**Decision**: Ship unified TypeScript definitions (e.g. `typescript/gram.d.ts` or similar) that export Pattern, Subject, Value, and Gram from one module so consumers can `import { Pattern, Subject, Value, Gram } from 'gram'` (or the published package name). Align with 027-wasm-pattern-typescript-parity contracts for Pattern/Subject/Value; add Gram namespace types.

**Rationale**: Spec and design doc require a single import and full type safety. One `.d.ts` (or a small set re-exported from index) keeps the contract clear and matches “single entry point.”

**Alternatives considered**:
- Separate packages for types: Rejected; spec requires one artifact and one import.
