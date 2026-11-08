# Ontology spike

Spike: express the [Neo4j ontology-graph-spec](https://github.com/neo4j/ontology-graph-spec)
(JSON/YAML, schema 1.0.0) in gram notation. Builds on `../graphtype/`.

Status: exploratory. Not yet decided whether this belongs in pattern-rs or in a
smaller, focused project (a gram dialect for compatible ontologies plus
translation to and from the JSON/YAML form).

## Design stance

Qualification is external. A node does not know its context, so anything the
spec nests inside an element (`description`, `aliases`, `reference`, `indexes`,
`display`, `custom`, data-source mappings) is expressed as a separate pattern
that refers to the element by identity. The same element can then appear in
many contexts (per-audience or per-language descriptions, several diagrams,
several data sources).

Consequence: the spec's nested ownership becomes referential integrity, which
tooling and editors must check. The spec's `rules.adoc` reference rules are the
starting list.

## Artifacts

- `movies.ontology.gram`: translation of the spec's `movies.yaml` (copy in
  `movies.yaml`). Parses with `pato parse`. Covers nodes, a relationship,
  property types, `must_exist`, `unique`, descriptions and aliases.
- `person.ontology.gram`: sketch of gramdoc (`/** ... */` with `@property`) as the
  human-readable meaning layer. Does not parse yet: the grammar has no doc
  comments. Proposal drafted in `proposals/gramdoc-issue.md` (not filed).
- Not started: an example with indexes, display and data-source mappings. This
  needs the spec's schema for those sections, which was not consulted here.

## Referential-integrity checks

Derived from `movies.ontology.gram`, not from the spec's `rules.adoc`. That file
is still the authoritative list and should be reconciled against this one.

1. Every `@on(x)` names an identity defined in the file.
2. Every `@property(k)` is a key in the record of the element named by the
   `@on(...)` on the same pattern.
3. Each relationship's `from` and `to` are node identities (not relationship or
   meaning identities).
4. Every member listed in an `Ontology` or `Glossary` pattern is defined.
5. Identities are unique across the file (spec ids share one namespace per kind).
6. Each `Meaning` is listed in at least one `Glossary` (warning, not an error).
