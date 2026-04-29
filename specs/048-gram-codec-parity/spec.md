# Feature Specification: Gram Codec Binding Parity

**Feature Branch**: `048-gram-codec-parity`
**Created**: 2026-04-28
**Status**: Draft
**Input**: User description: "gram codec parity for python and typescript to match the 4 methods available in rust for parse/serialize with/without header"

## Overview

The Rust gram-codec library exposes four core methods for reading and writing gram notation. The Python and TypeScript language bindings each expose only a subset of these methods, leaving developers unable to work with gram documents that carry a header record without dropping down to manual workarounds. This feature closes that gap by surfacing the full four-method surface in both bindings, using **language-native `Pattern<Subject>`** objects as the currency for patterns in both directions.

The four methods, stated in language-neutral terms:

| Method | Description |
|--------|-------------|
| `parse(input)` | Parse gram notation → `list[Pattern<Subject>]` |
| `parseWithHeader(input)` | Parse gram notation → optional header record + `list[Pattern<Subject>]` |
| `stringify(patterns)` | Serialize `list[Pattern<Subject>]` → gram notation |
| `stringifyWithHeader(header, patterns)` | Serialize header record + `list[Pattern<Subject>]` → gram notation |

**Current binding status:**

- TypeScript (`Gram` namespace in `@relateby/pattern`): `parse` and `stringify` already work with language-native `Pattern<Subject>` objects. `parseWithHeader` and `stringifyWithHeader` are absent.
- Python (`relateby.gram`): The existing `parse_gram` returns a thin `ParseResult` wrapper (count + identifiers only, not Pattern objects) and must be replaced by `parse`. `stringify` does not yet exist. All four canonical names (`parse`, `stringify`, `parse_with_header`, `stringify_with_header`) need to be established.

## Clarifications

### Session 2026-04-28

- Q: Should both bindings parse gram to produce language-native `Pattern<Subject>` objects, and accept language-native `Pattern<Subject>` objects to serialize to gram? → A: Yes — all four methods in both bindings use language-native `Pattern<Subject>` as the pattern currency.
- Q: Should replacing Python's thin `parse_gram` (returning `ParseResult`) with a proper `parse` returning `list[Pattern[Subject]]` be treated as a breaking change? → A: No — fixing parse to return the actual parsed data is addressing a critical gap, not a regression.
- Q: How should `parseWithHeader` return its two values? → A: TypeScript: named object `{ header, patterns }` inside an Effect / Python: plain two-tuple `(dict | None, list[Pattern])`.
- Q: What naming convention for Python's four functions? → A: `parse`, `parse_with_header`, `stringify`, `stringify_with_header` — matching TypeScript names for cross-language consistency.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Read a gram file that has a header record (Priority: P1)

A developer loads a `.gram` file that begins with a bare property record (e.g. `{version: 1, source: "export"}`) followed by graph patterns. Using `parseWithHeader`, they receive the header properties as a native map and the patterns as a list of `Pattern<Subject>` objects, so they can act on the metadata without filtering or converting anything manually.

**Why this priority**: Header records are used as document metadata (version, provenance, schema hints). Without `parseWithHeader`, every consumer must hand-roll the same filtering logic or silently receive a polluted pattern list.

**Independent Test**: Can be fully tested by calling `parseWithHeader` on a gram string containing a leading record and asserting (a) the header contains the expected properties and (b) the returned list contains language-native `Pattern<Subject>` objects and does not include the header element.

**Acceptance Scenarios**:

1. **Given** a gram string `{version: 1} (a)-->(b)`, **When** `parseWithHeader` is called, **Then** the header contains `{version: 1}` and the pattern list contains exactly one language-native `Pattern<Subject>` representing the relationship.
2. **Given** a gram string with no leading record `(a)-->(b)`, **When** `parseWithHeader` is called, **Then** the header is absent/null/None and the pattern list contains one language-native `Pattern<Subject>`.
3. **Given** an invalid gram string, **When** `parseWithHeader` is called, **Then** a parse error is raised with a descriptive message.

---

### User Story 2 - Write a gram file with a header record (Priority: P1)

A developer serializes a collection of language-native `Pattern<Subject>` objects and wants to prepend document-level metadata as a header record. Using `stringifyWithHeader`, they supply the header properties and the patterns and receive a single gram string with the header as the first element.

**Why this priority**: Round-trip correctness — if `parseWithHeader` can read a header, `stringifyWithHeader` must be able to write one. Together they form the complete read/write pair needed for file-based workflows.

**Independent Test**: Can be fully tested by calling `stringifyWithHeader` with a known header and a list of `Pattern<Subject>` objects and asserting the output is valid gram notation beginning with the header record.

**Acceptance Scenarios**:

1. **Given** a header `{version: 1}` and a pattern list of `Pattern<Subject>` objects, **When** `stringifyWithHeader` is called, **Then** the output is valid gram notation beginning with the header record.
2. **Given** an empty header and a pattern list, **When** `stringifyWithHeader` is called, **Then** the output contains only the serialized patterns with no leading record.
3. **Given** a header and an empty pattern list, **When** `stringifyWithHeader` is called, **Then** the output contains only the header record.
4. **Given** a round-trip: `stringifyWithHeader(header, patterns)` → output → `parseWithHeader(output)`, **Then** the recovered header and patterns equal the originals.

---

### User Story 3 - Parse and stringify without a header record (Priority: P1)

A developer uses `parse` and `stringify` in Python (`relateby.gram.parse`, `relateby.gram.stringify`) and receives / supplies language-native `Pattern<Subject>` objects — the same experience TypeScript's `Gram.parse` / `Gram.stringify` already provides.

**Why this priority**: `parse` and `stringify` are the foundation the header variants build on, and their correct behavior (returning/accepting native Pattern objects) is a prerequisite for the rest of the surface to be consistent.

**Independent Test**: Call `parse("(a)-->(b)")` in Python and assert the result is a list of native Pattern objects with the correct structure; call `stringify([pattern])` and assert a valid gram string is returned.

**Acceptance Scenarios**:

1. **Given** a gram string, **When** `parse` is called in Python, **Then** the result is a `list` of language-native `Pattern<Subject>` objects (not a thin wrapper or JSON string).
2. **Given** a list of language-native `Pattern<Subject>` objects, **When** `stringify` is called in Python, **Then** a valid gram notation string is returned.
3. **Given** a gram string, **When** `parse` is called in TypeScript (existing `Gram.parse`), **Then** the result is `ReadonlyArray<Pattern<Subject>>` — no change from current behavior.
4. **Given** a list of `Pattern<Subject>`, **When** `stringify` is called in TypeScript (existing `Gram.stringify`), **Then** a valid gram notation string is returned — no change from current behavior.

---

### Edge Cases

- What happens when the gram string contains only a header record and no patterns? (`parseWithHeader` returns a populated header and an empty pattern list.)
- What happens when the header contains value types not representable in gram notation (e.g. nested maps)? A serialization error is raised.
- What happens when `stringifyWithHeader` is called with a null/None/empty header? The output is identical to calling `stringify` alone.
- How does `parseWithHeader` behave on an empty string? Returns an absent header and an empty pattern list — matching the behavior of `parse` on empty input.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Python binding MUST expose `parse(input: str) → list[Pattern[Subject]]` returning language-native Pattern objects (not a thin wrapper, not JSON).
- **FR-002**: Python binding MUST expose `stringify(patterns: list[Pattern[Subject]]) → str` accepting language-native Pattern objects and returning gram notation.
- **FR-003**: Python binding MUST expose `parse_with_header(input: str) → tuple[dict | None, list[Pattern[Subject]]]` returning a two-tuple of (optional header dict, list of language-native Pattern objects).
- **FR-004**: Python binding MUST expose `stringify_with_header(header: dict, patterns: list[Pattern[Subject]]) → str` accepting a header map and language-native Pattern objects, returning gram notation.
- **FR-005**: TypeScript binding MUST expose a `parseWithHeader(input: string)` function returning `Effect<{ header: Record<string, unknown> | undefined, patterns: ReadonlyArray<Pattern<Subject>> }, GramParseError>`.
- **FR-006**: TypeScript binding MUST expose a `stringifyWithHeader(header: Record<string, unknown>, patterns: ReadonlyArray<Pattern<Subject>>)` function returning `Effect<string, GramParseError>`.
- **FR-007**: All six affected functions — Python: `parse` (replacement), `stringify`, `parse_with_header`, `stringify_with_header` (three new); TypeScript: `parseWithHeader`, `stringifyWithHeader` (two new) — MUST produce results consistent with their Rust counterparts (same parse logic, same serialization format).
- **FR-008**: TypeScript existing `Gram.parse` and `Gram.stringify` signatures MUST remain unchanged — they already satisfy the language-native Pattern requirement.
- **FR-009**: All binding functions MUST surface errors as the idiomatic error type for their language (Effect error channel in TypeScript, raised exception in Python).
- **FR-010**: TypeScript type signatures for the new `parseWithHeader` and `stringifyWithHeader` functions MUST be exported so downstream consumers can statically type their usage.

### Key Entities

- **Header Record**: A flat map of string keys to gram-compatible values that appears as the optional first element of a gram document. Carries document-level metadata (version, provenance, schema tags). Has no identity or labels.
- **Pattern\<Subject\>**: A decorated sequence element — the core data type in the pattern model. In Python: the native `Pattern` class from `relateby.pattern`. In TypeScript: `Pattern<Subject>` from `@relateby/pattern`. All four methods use this as the exclusive pattern currency.
- **Gram Document**: A gram notation string, optionally starting with a header record followed by zero or more patterns.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A Python developer can call `parse` and receive a list of native `Pattern<Subject>` objects directly usable with other `relateby.pattern` APIs — no JSON parsing or conversion step required.
- **SC-002**: A Python developer can call `parse_with_header` and receive the header and pattern list as separate, directly usable values in a single call.
- **SC-003**: A TypeScript developer can call `parseWithHeader` and destructure `{ header, patterns }` from the resolved Effect, with full static type information available for both fields.
- **SC-004**: A round-trip — `parse` → `stringify` and `parseWithHeader` → `stringifyWithHeader` — produces output that re-parses to identical values in both Python and TypeScript.
- **SC-005**: 100% of existing TypeScript tests for `Gram.parse`, `Gram.stringify`, and `Gram.validate` continue to pass after this feature is merged.
- **SC-006**: The new and upgraded functions are covered by tests that exercise all acceptance scenarios defined in this specification.

## Assumptions

- The Rust `parse_gram`, `parse_gram_with_header`, `to_gram`, and `to_gram_with_header` implementations are stable and will not change during this work; the bindings defer entirely to them.
- "Header record" means a bare property map `{...}` with no identity or labels as the first (and only first) element of a gram document — consistent with the current Rust implementation.
- The Python `parse_gram` thin wrapper (returning `ParseResult` with count and identifiers only) is a critical gap, not a baseline to preserve. Replacing it with `parse` returning native `Pattern<Subject>` objects closes that gap. The JSON interchange functions (`gram_parse_to_json`, `gram_stringify_from_json`) remain available for callers that need raw JSON interchange.
- The Python binding ships through the existing `relateby.gram` package surface; no new package or module is introduced.
- The TypeScript binding ships as additions to the existing `Gram` namespace object in `@relateby/pattern`; no new entry point is introduced.
- TypeScript header type is an object/map of string keys to gram-compatible values; precise static typing of value variants is out of scope for this feature.
