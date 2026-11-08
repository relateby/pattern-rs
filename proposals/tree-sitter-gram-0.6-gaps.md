# tree-sitter-gram 0.3.11 → 0.6.1: what pattern-rs still has to catch up on

Status: notes, written on `feat/update-tree-sitter-gram` right after the
submodule bump. Only two grammar-affecting commits sit between those versions
(`d173414` doc_comment, `31e9d5c` arrays in maps); the rest is CLI/extension
tooling inside tree-sitter-gram and does not touch us.

## 1. Doc comments (`/** ... */`, gramdoc)

Done in this branch: both parsers accept `/** */`; the CST keeps top-level and
pre-annotation doc comments as `SyntaxKind::DocComment` with raw text, and nested
ones in `SyntaxNode.doc_comments`.

Gaps (found by probing, not by reading):

- **Nested doc comments: captured (raw).** `SyntaxNode.doc_comments` holds
  `DocComment { text, span }` for doc comments inside a node's record, a
  subject pattern's record or element list, a relationship record, or between
  annotations and the annotated element. Child patterns keep their own. What is
  still missing: they are not tied to a specific property or element. A
  consumer has to compare spans, and the record property spans are not in the CST.
- **No gramdoc structure.** Nothing parses comment contents (description,
  `@property`, `@label`, `@alias`, `@reference`, `@element(s)`, `@source`,
  `@target`, `@self`). Needs tree-sitter-gramdoc as a dependency or a small
  Rust parser.
- **No attachment.** The grammar leaves attachment to consumers. Rules proposed
  in `external/tree-sitter-gram/proposals/tree-sitter-gramdoc.md`: next sibling
  in the same container; a comment before `@a (n)` documents the annotated
  pattern, one between the annotations and `(n)` documents `n`; trailing
  comments are orphans; several in a row all attach, in order.
- **Lowering drops them.** `Pattern<Subject>` has nowhere to keep docs, so
  `pato parse` / `pato fmt` via lowering lose them. (`pato fmt` works from the
  CST and keeps top-level ones verbatim.) Where docs live at the Pattern level
  is undecided; check what gram-data/gram-ontology settled before choosing.
- **pato fmt refuses documents with nested doc comments** (records, subjects,
  annotations, header record). It rebuilds output from lowered subjects and raw
  fragments, which would drop them and let their text be scanned as syntax. Needs
  real support before it can rewrite such files.
- **pato lint** does not yet validate `@property`/`@label` names or
  `@element` counts (proposal question 8, `gram-lint-rules.md`).

## 2. Maps may contain arrays of scalars

Grammar: `map_entry` value is now `scalar | array`. Depth budget is
scalar 0, array 1, map 2, record 3.

- Parses and lowers: `{identity: {name: "x", aliases: ["a", "b"]}}` is valid
  in both parsers and appears correctly in JSON output.
- **Serializer: fixed.** Maps are written directly in `serialize_property_value`
  (the codec `Value` has no map variant), with the grammar depth check, and
  `pato parse` reports a serialize error instead of panicking. Round-trip tests in
  `tests/map_roundtrip.rs`. Measurements serialize as `<integer><unit>` (see section 5).
- **CST path flattens maps to strings.** `cst/parser.rs` (~lines 340, 370)
  converts `VMap` to `Value::String` for annotations.
- **nom map/array depth: aligned.** `parser/value.rs` now bounds depth like the grammar
  (arrays: scalars; map entries: scalar or array of scalars). A test runs both parsers
  over nine shapes (`nom_and_cst_agree_on_map_and_array_depth`).
- Docs not updated: any place that says map values are "scalar only".

## 3. New JSON interchange schema

`external/tree-sitter-gram/docs/pattern.schema.json` (v0.1.0, `Pattern<Subject>`
as JSON). Tag names (`symbol`, `tagged`, `range`, `measurement`) match
`gram-codec/src/json.rs`. Not yet checked: required fields, `identity` always a
string, array/object value rules. Worth a conformance test against the schema.

## 4. Smaller things

- `Cargo.toml` requirement was `tree-sitter-gram = "0.3"`; now `"0.6"`. Check
  publish/release flow (`docs/contributor/release.md`) since the dependency is a
  path dependency and the crate version changed.
- Wasm/Python/TS bindings were not rebuilt or re-tested for the bump
  (`./scripts/build-dev.sh`).
- `docs/gram.ebnf`, `gram-reference.md` and `gram.g4` in tree-sitter-gram were
  updated for both changes; `gram.g4` was not in the diff stat I read, so it may
  lag the other grammars.

## 5. Measurements

- tree-sitter-gram's `measurement` token is `-?(0|[1-9]\d*)[a-zA-Z]+`: **integer
  magnitude only**. The EBNF and g4 agree; only the comment above the rule in
  `grammar.js` shows a decimal form (`3.5kg`). The nom parser reads decimals, so
  `parse_gram` accepts `3.5kg` and `parse_gram_cst` rejects it. Raised upstream as
  [gram-data/tree-sitter-gram#34](https://github.com/gram-data/tree-sitter-gram/issues/34)
  (integer-only vs. widen the regex). Until it is settled the two parsers disagree
  on decimal measurements; if upstream keeps integer-only, tighten nom's
  `measurement` to match.
- Serializer writes `168cm`, refuses non-integer magnitudes, non-finite or huge
  values, and units that are not ASCII letters, since the grammar would reject
  the output. If upstream widens the regex, relax `serialize_measurement`.
- Still open: the CST path turns measurement annotation values into strings.

## Suggested order

1. (done) Capture nested doc comments in the CST.
2. (done) Fix map serialization and the `pato parse` panic.
3. (done) Align the nom map parser with the grammar.
4. Add gramdoc parsing and the attachment rules, then decide the Pattern-level home.
