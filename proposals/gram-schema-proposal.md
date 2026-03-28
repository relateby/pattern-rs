# Feature Proposal: Gram-Native Schema

**Status**: Draft — for review and iteration
**Date**: 2026-03-23
**Scope**: Exploration through library and CLI integration
**Depends on**: `041-pato-cli` (pato CLI foundation), `042-gram-cst-parser` (CST parser)

---

## 1. Purpose

This proposal describes a gram-native schema system for pattern-rs. A schema is itself
a gram file — an archetypal document that shows canonical example structures, where
syntax choices in those examples may be normative, and where value constraints are
expressed using tagged strings.

The feature has four phases that build on each other:

1. **Exploration** — collaborative authoring of schema and data examples (no code)
2. **Validation library** — rule-based conformance checking of data against schema
3. **Generation library** — schema-driven data generation (unfold)
4. **CLI integration** — `pato check`, `pato generate`, and related commands

Unusually for this project, **Phase 1 is collaborative design work rather than code
generation**. The schema vocabulary, constraint dialect, and surface conventions must
be settled through example authoring before any implementation begins. Getting this
right in gram files first prevents premature commitment to a design that turns out to
be awkward to write or read.

---

## 2. Background and Motivation

### The archetypal schema approach

A gram schema is not a separate meta-format. It is a gram file with a header record:

```gram
{ kind: "schema" }
```

The file shows canonical example structures. The examples are not merely illustrative —
they define the contract at three levels:

1. **Structure**: which nodes, relationships, annotations, and properties are present
2. **Surface form**: which syntax choices are canonical (`:` vs `::`, arrow family,
   annotation style)
3. **Value constraints**: how property types and value restrictions are expressed

This approach is already partially explored in `data/schema/` and documented in
`specs/041-pato-cli/contracts/schema-gram.md`. The open questions recorded there —
what does presence imply, which surface forms are normative, how are constraints
written — are exactly what Phase 1 will resolve.

### The `gram` constraint dialect

Value constraints in schema files use tagged strings:

```gram
(::Person {
  name::  ts`string`,
  age::   gram`(:Range { min: 0, max: 130 })`,
  bio::   gram`(:TaggedString { tag: "md" })`
})
```

External dialect tags (`ts`, `zod`, `cypher`, `re`, `pydantic`) are useful for human
readability and documentation, but each one represents a significant implementation
effort: a parser for that dialect's syntax, a semantic mapping to gram's value space,
and ongoing maintenance as external specs evolve.

The `gram` dialect is different. It is a small, closed vocabulary of constraint
patterns using gram's own notation. This project owns the full stack — CST, value
types, pattern matching — so the `gram` dialect can be implemented completely and
correctly without external dependencies.

### Lineage: a pattern-backed lisp

The `gram` constraint dialect has a specific lineage: a side project that implemented
a small lisp interpreter backed by patterns and serializable as gram. The schema
constraint vocabulary is the smallest useful fragment of that language — just the
type-assertion layer, with no general evaluation, closures, or recursion.

This means:

- Constraints are **labeled patterns with property bags as arguments**
- The label is the dispatch key (e.g. `Range`, `TaggedString`, `Array`)
- The properties are the parameters (e.g. `min`, `max`, `tag`, `items`)
- Evaluation is pattern-matching on the constraint node followed by a value check

The content of a `gram`-tagged string is valid gram notation — parseable by the
existing gram CST parser applied to the tagged string content. The constraint
vocabulary is a subset of gram, not a new language.

### Validation and generation are symmetric

Because the `gram` constraint dialect is declarative and bounded, every constraint
has an obvious inverse:

- Validation: given `(:Range { min: 0, max: 130 })` and a value, check membership
- Generation: given `(:Range { min: 0, max: 130 })`, produce a value in `[0, 130]`

This symmetry means validation and generation can be developed together. The schema
is not just a validator — it is a complete specification from which arbitrary
conforming data can be unfolded. This is the `unfold` operation: the corecursive
counterpart to the `fold` already present in `Pattern<V>`.

---

## 3. Design Principles

**Schema is gram.** A schema file is a valid gram file. It can be parsed, linted, and
formatted by pato without special-casing. The `kind: "schema"` header is the only
structural requirement.

**The `gram` dialect is first-class; external dialects are opaque.** The `gram`
constraint dialect is fully implemented and validated. External tags (`ts`, `zod`,
`cypher`, etc.) are preserved and round-tripped but treated as opaque strings unless
a plugin registers a handler. Extension dispatch (already in pato's design) is the
natural hook for external dialect validators.

**Presence semantics are explicit.** Whether a property or relationship appearing in
the schema means "required", "allowed", or "illustrative" must be declared, not
inferred. The default and the override mechanism are settled in Phase 1.

**Open and closed are separate axes.** Vocabulary openness (are extra labels and
properties allowed?) and composition openness (are larger structures composed from
valid archetypes allowed?) are independent validation-time settings. They must not
be collapsed into a single open/closed switch.

**Surface form is part of the contract.** A schema may specify that `::` labels are
canonical for domain nodes, or that fat-arrow relationships are canonical for a
given relationship type. Whether non-canonical but semantically equivalent forms are
errors, warnings, or accepted is a policy decision — but the schema can express the
preference.

**Validation and generation share a model.** The constraint vocabulary is designed so
that every constraint can both validate a value and generate a conforming value. This
is not an afterthought — it is a design constraint on the vocabulary itself.

---

## 4. Phase 1: Exploration (Collaborative Design — No Code)

### Goal

Settle the schema vocabulary, constraint dialect, presence semantics, and surface
conventions through example authoring. The deliverables are gram files, not code.

### Why this phase is different

Most features in this project begin with a spec and move quickly to implementation.
Schema is different because the design space is genuinely open. The questions in
`data/schema/README.md` — what does presence imply, which surface forms are
normative, how are constraints written — cannot be answered by reasoning alone.
They require writing real schemas against real data files and discovering what
feels right and what is awkward.

This phase is explicitly collaborative: the human author and the AI assistant work
together to draft, critique, and refine examples. No code is written. The output
is a set of example files and a vocabulary document that becomes the normative
reference for Phases 2 and 3.

### Process

1. **Reverse-engineer from existing data** — take concrete gram data files (from
   `data/` or examples) and write schemas that would validate them
2. **Author forward** — write a schema first, then write data files that conform to it
3. **Stress-test presence semantics** — deliberately write data that is missing
   optional vs required properties and verify the schema expresses the distinction
4. **Stress-test the `gram` dialect** — cover all native gram value forms: integers,
   decimals, strings, booleans, symbols, tagged strings, arrays, maps, measurements,
   ranges, dates
5. **Compare surface styles** — write the same schema in multiple surface forms
   (`:` vs `::`, thin vs fat arrows) and decide which choices are normative for which
   contexts
6. **Identify gaps** — note any value forms or structural patterns that the `gram`
   dialect cannot express

### Deliverables

- `data/schema/` — expanded set of schema examples covering diverse domains
- `data/schema/vocabulary.md` — the canonical `gram` constraint vocabulary:
  labels, properties, semantics, and examples for each constraint type
- `data/schema/presence-semantics.md` — how presence, optionality, and requirement
  are expressed in archetypal schemas
- Updated `data/schema/README.md` — reflecting settled decisions

### The `gram` constraint vocabulary (initial candidates)

These labels and properties are starting points, to be validated and refined in
Phase 1:

**Scalar types**

| Label | Properties | Meaning |
|---|---|---|
| `:Integer` | `min`, `max`, `bounds` | Integer value, optional range |
| `:Decimal` | `min`, `max`, `bounds` | Decimal/float value, optional range |
| `:String` | `min_length`, `max_length`, `pattern` | String value, optional constraints |
| `:Boolean` | — | Boolean value |
| `:Symbol` | — | Gram symbol (unquoted identifier) |
| `:Range` | `type`, `min`, `max`, `bounds` | Numeric range with type annotation |
| `:Measurement` | `unit`, `value_type` | Numeric value with a unit |

**String forms**

| Label | Properties | Meaning |
|---|---|---|
| `:TaggedString` | `tag` | Tagged string with a specific tag |
| `:FencedString` | — | Triple-backtick fenced string |

**Collection types**

| Label | Properties | Meaning |
|---|---|---|
| `:Array` | `items`, `min_length`, `max_length` | Array with element type constraint |
| `:Map` | `keys`, `values` | Map with key/value type constraints |

**Presence modifiers**

| Label | Properties | Meaning |
|---|---|---|
| `:Optional` | `of` | Wraps another constraint; value may be absent |
| `:Required` | `of` | Wraps another constraint; value must be present |
| `:OneOf` | `values` | Enumeration of allowed values |

---

## 5. Phase 2: Validation Library

### Goal

Add rule-based schema validation to `pattern-core`: read a schema file, parse the
archetypal structures and constraints, and validate a data gram file against the
schema with configurable open/closed world semantics.

### Scope

**In scope for v1:**
- Structural validation: required labels, required properties, relationship endpoints
- Value validation using the `gram` constraint dialect
- Open vs closed vocabulary (extra labels and properties)
- Open vs closed composition (extra structural patterns)
- Diagnostic output compatible with pato's existing diagnostic gram format

**Deferred:**
- External dialect evaluation (`ts`, `zod`, `cypher`, etc.) — opaque, preserved
- Cross-file schema composition (schema inheritance or import)
- Annotation-form validation (whether `@key` vs `@@key` is normative)

### Architecture

Schema validation lives in a new module within `pattern-core` (or a new
`schema-core` crate if the surface area warrants it). The key types:

- **`SchemaDocument`** — a parsed schema file: a collection of archetypal patterns
  with their constraint slots resolved
- **`ConstraintVocabulary`** — the `gram` dialect interpreter: maps constraint
  labels to validation functions
- **`ValidationConfig`** — vocabulary openness, composition openness, presence
  defaults
- **`ValidationResult`** — a collection of `Violation` values, each with a location,
  a message, and a severity

The `gram` constraint dialect is parsed by applying the existing gram CST parser to
the tagged string content. Constraint evaluation is pattern-matching on the resulting
`Pattern<Subject>` — the same machinery already used throughout the library.

### Open/closed semantics

Two independent axes, configured independently:

```
vocabulary: open | closed
  open:   extra labels and properties in data are allowed
  closed: only labels and properties declared in the schema are allowed

composition: open | closed
  open:   larger structures composed from valid archetypes are allowed
  closed: only the exact structural patterns shown in the schema are allowed
```

Default (when not specified in the schema header): `vocabulary: open`,
`composition: open`. This matches the least-surprising behavior for new users.

---

## 6. Phase 3: Generation Library (Unfold)

### Goal

Add schema-driven data generation to `pattern-core`: given a schema, produce
arbitrary gram data files that conform to it.

### The unfold operation

`Pattern<V>` already has `fold` (catamorphism) and `para` (paramorphism). Generation
is the corecursive counterpart: an `unfold` (anamorphism) that seeds from a schema
and expands outward into conforming data.

The `gram` constraint dialect is designed so every constraint has an obvious
generator:

- `:Integer { min: 0, max: 100 }` → a random integer in `[0, 100]`
- `:String { min_length: 3 }` → a random string of at least 3 characters
- `:TaggedString { tag: "md" }` → a tagged string with tag `md` and placeholder content
- `:Array { items: "string", min_length: 1 }` → a non-empty array of strings
- `:OneOf { values: ["a", "b", "c"] }` → one of the enumerated values

Generation modes:

- **Minimal** — produce the smallest valid instance (required fields only, minimum
  cardinalities, simplest values)
- **Typical** — produce a representative instance with reasonable values
- **Exhaustive** — enumerate all structurally distinct valid instances (useful for
  testing)
- **Random** — produce a random valid instance (requires a seeded RNG for
  reproducibility)

### Relationship to testing

Schema-driven generation is the natural source of property-based test fixtures.
A schema defines the valid input space; the generator samples from it. This connects
directly to the existing `proptest` infrastructure in the project.

---

## 7. Phase 4: CLI Integration

### Goal

Expose schema validation and generation through `pato`, building on the extension
dispatch and subcommand infrastructure from `041-pato-cli`.

### `pato check` (v0.2)

`pato check` already exists in v0.1 with schema discovery stubbed out (P007
informational diagnostic when no schema is found). In v0.2, when a schema is present,
`pato check` performs full structural and `gram`-dialect validation:

```
pato check my.gram                    # auto-discover my.schema.gram
pato check --schema types.schema.gram my.gram
pato check --vocabulary closed my.gram
pato check --composition closed my.gram
```

Violations are emitted as diagnostic gram, compatible with the existing diagnostic
format. Exit codes follow the existing contract (0/1/2/3).

### `pato generate`

A new subcommand (or `pato-generate` extension) that produces conforming data from
a schema:

```
pato generate my.schema.gram                    # minimal instance to stdout
pato generate --mode typical my.schema.gram
pato generate --mode random --seed 42 my.schema.gram
pato generate --count 10 my.schema.gram         # 10 instances
```

Output is valid gram on stdout. The `--seed` flag makes generation reproducible.

### `pato schema`

An introspection subcommand for working with schema files directly:

```
pato schema lint my.schema.gram       # lint the schema itself
pato schema explain my.schema.gram    # human-readable summary of the schema
pato schema vocab                     # list the built-in gram constraint vocabulary
```

### External dialect extensions

External dialect validators plug in via extension dispatch:

```
pato-ts-constraints    # validates ts`...` tagged strings
pato-zod-constraints   # validates zod`...` tagged strings
```

These are independent binaries, not part of the core pato distribution. The
`pato check` command discovers and invokes them when it encounters tagged strings
with matching dialect tags.

---

## 8. Sequencing and Dependencies

```
Phase 1 (Exploration)
  └─► Phase 2 (Validation)
        └─► Phase 4 (pato check v0.2)
  └─► Phase 3 (Generation)
        └─► Phase 4 (pato generate)
```

Phase 1 must complete before Phase 2 or 3 begin. The vocabulary document from
Phase 1 is the normative reference for both implementations.

Phases 2 and 3 can proceed in parallel once Phase 1 is complete, since they share
the schema parsing layer but diverge at evaluation vs generation.

Phase 4 depends on both Phase 2 and Phase 3 being sufficiently stable.

### Existing foundation

- `data/schema/` — early schema examples already authored
- `specs/041-pato-cli/contracts/schema-gram.md` — schema contract (exploratory)
- `041-pato-cli` — pato CLI with `pato check` stub and extension dispatch
- `042-gram-cst-parser` — CST parser used for constraint content parsing
- `Pattern<V>` fold/para — foundation for the unfold operation

---

## 9. Open Questions

These questions are for Phase 1 to resolve through example authoring:

1. **Presence default**: does a property appearing in the schema archetype mean
   "required", "allowed", or "illustrative" by default? What is the override syntax?

2. **Relationship endpoint constraints**: do endpoint labels live in the structural
   pattern, in tagged strings, or both? How are union endpoints expressed
   (e.g., "any `Resident`")?

3. **Schema composition**: can one schema reference or include another? Is this
   needed in v1?

4. **`gram` dialect completeness**: are there gram-native value forms that the
   initial vocabulary cannot express? (Dates, durations, URIs, geographic
   coordinates?)

5. **Surface form normative scope**: is arrow family (thin vs fat) always normative,
   or only when the schema explicitly uses fat arrows? Same question for `:` vs `::`.

6. **Constraint identity**: should constraint patterns in `gram`-tagged strings have
   identities, or are they always anonymous? (Relevant for error messages and for
   referencing shared constraints.)

7. **Schema versioning**: should schema files carry a version or compatibility
   declaration? What does a breaking schema change look like?

---

## 10. Non-Goals

- **A separate schema meta-format.** The schema is gram. There is no separate
  schema language, no JSON Schema equivalent, no external DSL.
- **Full external dialect evaluation in v1.** `ts`, `zod`, `cypher`, and similar
  tags are opaque in the core library. Extension dispatch handles them.
- **Schema inheritance or composition in v1.** One schema file, one contract.
- **IDE integration.** Out of scope for this proposal; a natural follow-on once
  the vocabulary is stable.
- **Schema inference.** Generating a schema from existing data is a separate feature;
  this proposal covers authoring schemas by hand or collaboratively.
