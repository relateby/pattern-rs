# Schema Modeling Exploration

This directory captures an early exploration of what a gram-native schema story could look like.

## Purpose

The goal is to test an archetypal schema approach:

- the schema itself is a `.gram` file
- the file shows canonical example structures rather than an external meta-format
- comments explain semantics that are not obvious from shape alone
- `::` is used for type/schema slots
- tagged strings defer value constraints either to an external, well-known
  language or to a small gram-native constraint DSL
- surface syntax is part of the schema contract: node labels, relationship
  labels, property slots, arrow families, annotation forms, and composition
  style may all be normative

Put differently, these examples are intentionally not all styled the same way.
Some demonstrate a schema that is uniform throughout; others demonstrate a
schema where domain archetypes, relationship archetypes, and schema-meta nodes
use different surface forms on purpose.

Examples in this directory use tags such as:

- `ts` for TypeScript-like type signatures
- `re` for regular expressions
- `zod` for runtime-validator-style expressions
- `cypher` for graph-type assertions (see https://neo4j.com/docs/cypher-manual/current/schema/graph-types/)
- `pydantic` for Python-model-style constraints
- `gram` for gram-native value forms such as ranges, measurements, and tagged strings

## Files

- `01-archetypal-ts.schema.gram`: a uniform schema using `::` node labels,
  `::` property slots, and fat-arrow relationships
- `02-ts-and-re.schema.gram`: single-colon node labels with `::` property slots
  and thin-arrow relationships
- `03-zod-runtime.schema.gram`: `::` node labels paired with thin-arrow
  relationships to show those choices can vary independently
- `04-cypher-graph-assertions.schema.gram`: single-colon domain nodes with
  fat-arrow relationships and single-colon schema-meta nodes
- `05-mixed-dialects.schema.gram`: several dialect tags plus an explicit
  annotation example to show annotation form as part of schema style

## Questions To Review

- Does presence in the example imply "required", "allowed", or merely "illustrative"?
- Are extra labels and extra properties allowed by default?
- Should relationship endpoint constraints live in the structure, in tagged strings, or both?
- Should `ts` mean literal TypeScript syntax, or a TS-inspired constraint dialect?
- Which tags should be treated as first-class by pato, if any?
- Which style choices should be validated as hard requirements vs soft preferences?

All `.schema.gram` files here are intended to be exploratory, human-readable, and parseable.
