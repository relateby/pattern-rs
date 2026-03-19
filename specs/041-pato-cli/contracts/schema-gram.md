# Contract: Archetypal Schema Gram

**Version**: draft  
**Status**: exploratory contract recorded in `041-pato-cli`; not yet enforced by `pato check`

## Purpose

This document records the currently preferred direction for future gram schema validation in pato.

The schema itself is a gram file. It is **archetypal** rather than merely descriptive:

- the file shows canonical example structures
- syntax choices in those examples may be normative
- comments explain semantics that are not obvious from structure alone
- value/type constraints live in schema slots using tagged strings

This contract is intended to stabilize the branch's schema vocabulary before semantic validation is
implemented.

## File Kind

Schema files are gram files with a header record:

```gram
{ kind: "schema" }
```

Additional metadata may be added, for example:

```gram
{ kind: "schema", mode: "archetypal", vocabulary: "closed", composition: "open" }
```

The meanings of `vocabulary` / `composition` are validation-time semantics and are not yet part of
the implemented pato CLI surface.

## Archetypal Semantics

An archetypal schema describes a contract at three levels:

1. **Structure**: which nodes, relationships, annotations, and properties are exemplified
2. **Surface form**: which syntax choices are canonical (for example `:`
   vs `::`, arrow family, annotation form)
3. **Constraint language**: how property/value constraints are written

Examples:

- `(::Person)` may mean that the `::` form is canonical for that node type
- `(::Person) =[::WORKS_ON]=> (::Project)` may mean that the fat-arrow family is canonical for that relationship type
- an identified or stacked annotation form in the schema may imply the expected annotation surface form in source

Whether non-canonical but semantically similar forms are rejected, warned about, or accepted is a
future validation-policy concern.

## Surface Syntax Is Part Of The Schema

The archetypal examples are not just about abstract shape. They may also define the preferred
surface grammar for each construct position.

That means a schema may specify, independently or together:

- node label style: `(:Person)` vs `(::Person)`
- relationship label style: `-[:KNOWS]->` vs `=[::KNOWS]=>`
- property slot style: `name: "Alice"` vs `name:: ts\`string\``
- annotation style: inline, identified, stacked, or bracketed-reference forms
- composition style: whether an archetype is shown as a single pattern, split across multiple
  patterns, or augmented by separate annotation patterns

These choices may be:

- uniform across the whole schema
- different for domain archetypes vs schema/meta nodes
- different for nodes, relationships, annotations, and properties

For example, a schema may intentionally use:

- `(::Person)` for domain nodes
- `-[:WORKS_ON]->` for domain relationships
- `name:: ts\`string\`` for schema slots
- `(schema_rule:Assertion { ... })` for meta-level helper nodes

The important point is that style is itself part of the contract being exemplified, not just a
rendering preference layered on top later.

## Open vs Closed Validation

Validation strictness should distinguish two axes:

### Vocabulary openness

- **closed vocabulary**: only labels, properties, relationship kinds, syntax forms, and constraint tags described by the schema are allowed
- **open vocabulary**: extra labels, properties, relationship kinds, syntax forms, or tags may appear

### Composition openness

- **closed composition**: only the kinds of compositions explicitly exemplified by the schema are allowed
- **open composition**: larger structures composed from valid archetypes are allowed even if not explicitly enumerated

These axes are separate and should not be collapsed into one mode switch.

## Schema Slots

The preferred convention for property/value constraints is:

- use `::` for schema/type slots
- use tagged strings as the slot values

Examples:

```gram
(::Person {
  name:: ts`string`,
  email:: re`^[^@ ]+@[^@ ]+[.][^@ ]+$`
})
```

The outer schema remains ordinary gram. The tag determines how the slot value should be
interpreted.

## Tagged Constraint Dialects

The branch's exploratory examples currently use these tags:

- `ts`
- `re`
- `zod`
- `cypher`
- `pydantic`
- `gram`

Not all tags need to be understood by pato itself. Tags may remain opaque and be delegated to
downstream validators.

## `gram` Dialect

Some gram value forms are unusual enough that mainstream external languages are awkward fits:

- ranges
- measurements with units
- tagged/fenced strings
- symbol-vs-string distinctions

For those cases, the preferred direction is a small **gram-native constraint dialect** carried
inside a tagged string with tag `gram`.

Important: gram does **not** allow patterns as direct property values. Therefore the explicit
constraint structure must be encoded inside the tagged string, not embedded directly in the outer
schema tree.

Example:

```gram
(::Person {
  age:: gram`(:Range { type: "int", min: 0, max: 130 })`,
  bio:: gram`(:TaggedString { tag: "md" })`,
  height:: gram`(:Measurement { unit: "cm", value_type: "number" })`
})
```

Here, the content of `gram\`...\`` is a tiny gram-shaped DSL whose semantics come from conventional
labels and properties.

## Conventional Constraint Labels

The exact mini-DSL is still exploratory, but these labels are currently plausible:

- `:Range`
- `:Measurement`
- `:TaggedString`
- `:Symbol`
- `:String`
- `:Integer`
- `:Decimal`
- `:Boolean`
- `:Array`
- `:Map`

and properties such as:

- `type`
- `value_type`
- `unit`
- `tag`
- `min`
- `max`
- `bounds`
- `items`

These names are conventions for the future schema DSL, not yet a ratified implementation contract.

## Example

```gram
{ kind: "schema" }

// Canonical node type. This implies both the shape and the preferred surface form.
(::Person {
  name:: ts`string`,
  email:: re`^[^@ ]+@[^@ ]+[.][^@ ]+$`,
  age:: gram`(:Range { type: "int", min: 0, max: 130 })`,
  bio:: gram`(:TaggedString { tag: "md" })`
})

// Canonical relationship type.
(::Person) =[::WORKS_ON {
  role:: ts`"lead" | "support" | "reviewer"`
}]=> (::Project)
```

## Non-Goals For This Branch

- No semantic schema validation is implemented yet in `pato check`
- No fixed policy yet for whether non-canonical surface forms are errors vs warnings
- No requirement that pato itself understand every external dialect tag
- No finalized grammar yet for the inner `gram` mini-DSL
