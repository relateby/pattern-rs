## Summary

Propose `gramdoc`: structured documentation comments for gram, in the style of [cypherdoc](https://github.com/gram-data/tree-sitter-cypher/blob/main/proposals/cypherdoc-injection.md). Concretely:

1. tree-sitter-gram recognizes a `/** ... */` block as a `doc_comment` node.
2. `queries/injections.scm` injects the language `gramdoc` into `doc_comment` nodes.
3. `gramdoc` is a standalone tree-sitter grammar in the gram-data org (its own repo, like cypherdoc), adopted here by injection.

## Motivation

This came out of a spike in `relateby/pattern-rs` (`data/schema/ontology/`, branch `spike/ontology-graph-spec-gram`) that expresses the [Neo4j ontology-graph-spec](https://github.com/neo4j/ontology-graph-spec) in gram. The spec adds `description`, `aliases` and `reference` to graph-type-like definitions, so that people and LLMs can tell what an element *means* ("A schema tells an agent what exists, not what it means").

Gram's stance is that qualification is external: a node doesn't know its context, so nesting meaning inside an element is the wrong shape. Meaning should attach from outside (annotations, element references), and the same element should be able to carry different meanings in different contexts (language, audience).

That leaves a gap for the human-readable layer. We want descriptions and aliases for an element, and for individual properties of an element, without nesting them in the element's record and without inventing a property-addressing syntax. A documentation comment placed next to the thing it documents does both, and it is "meta-meta data": it can sit on the original element or on any annotation attached to it.

Driving example (`person.ontology.gram`):

```gram
/**
 * A human individual.
 *
 * @property name The full name of the person.
 */
(Person {
    name: "STRING!"
})

/**
 * Un être humain.
 *
 * @property name Le nom complet de la personne.
 */
[:French {
      name: "nom"
} | Person ]
```

Here the second pattern is an annotation-style context (`:French`) over `Person`, with its own gramdoc, and `@property name` addresses a property by name inside the comment instead of in the structure.

## Current state (checked against `240a78a`)

- The only comment form is `comment: token(seq("//", /.*/))`, an `extras` entry. `/** */` and `/* */` are not comments.
- Parsing the example above today produces `ERROR` nodes for each `/** ... */` block, and the `@property name ...` text inside is parsed as an annotation.
- `//` comments are kept as `(comment)` nodes. `test/corpus/comments.txt` covers top-level, end-of-line and before-pattern placement; the `:error` cases are only where `//` would swallow a closing delimiter.
- Not covered by the corpus, but probed against the current grammar: `comment` nodes parse as siblings of `record_property` inside records, between subject-pattern elements, inside relationship records, and before or between annotations. So positional doc comments are feasible at the parser level.
- Prior decision: `specs/010-publish-lib/plan.md` rejected a `tree-sitter-gramdoc` grammar, but that was about gram embedded in markdown (use markdown injection). This proposal is the opposite direction, documentation inside gram, and doesn't conflict with it.

Downstream note (relateby/pattern-rs `crates/gram-codec`, from reading the code, not executed): `parse_gram` uses the nom parser and discards comments. The tree-sitter path (`parse_gram_cst`) retains `comment` nodes as `SyntaxKind::Comment`, but lowering to `Pattern<Subject>` drops them, and record-internal comments are likely not captured at all. That is a separate pattern-rs follow-up once the grammar side is settled.

## Proposed design

**Host grammar (tree-sitter-gram)**
- Add a `doc_comment` token for `/** ... */`, as an `extras` entry alongside `comment`. Its precedence over any future block-comment form needs to be explicit.
- Add to `queries/injections.scm`:
  ```scm
  ((doc_comment) @injection.content
    (#set! injection.language "gramdoc"))
  ```
- Corpus tests for doc comments at top level, before annotations, before elements, inside records (before a `record_property`), inside subject-pattern element lists, and unterminated blocks.

**Sub-grammar (`tree-sitter-gramdoc`, separate repo)**
Modeled on the cypherdoc layout:
- `grammar.js`, `queries/{highlights,tags}.scm`, `test/corpus/*.txt` split by concern (tags, names, descriptions).
- Leading `*` on each line is whitespace via `extras`; the grammar handles `/**` and `*/` itself.
- Structure: optional name, free-prose description, then tags. Named nodes (`name`, `description`, tag nodes) so queries can address them.
- Injected content is the whole `/** ... */` text, as in cypherdoc.

**Attachment**
A doc comment documents the next element in source order: an element, an annotation, or a `record_property` when placed inside a record. Because annotations are patterns whose single element is the annotated thing, a doc comment can sit on the original element or on any annotation attached to it. Attachment is a consumer rule over adjacency unless we decide the host grammar should make it structural.

**Initial tag set (to settle)**
- `@property <name> <description>` for documenting a property of the element, so no property-addressing syntax is needed in the structure.
- A tag for aliases and one for external references, covering the spec's `aliases` and `reference`.
- The description prose covers the spec's `description`.

## Open questions

1. **Tag syntax.** cypherdoc uses `@param {type} name - description` (distinct required/optional nodes, a deliberately retained `-`). The example above uses `@property name description` with no braces or dash. Which do we adopt, and do we want `{type}` at all given gram already has `::` and tagged strings for types?
2. **Sigil overlap.** Gram annotations use `@` too (`@key(id)`). They don't clash grammatically since tags live inside comments, but we should decide whether that is acceptable for readability.
3. **Tag set.** Beyond `@property`: names for aliases and references; whether to allow arbitrary user tags.
4. **Description format.** Plain prose, or Markdown (gram already injects `md`)?
5. **Block comments.** Do we also add plain `/* */`, or only `/** */`?
6. **Attachment.** Adjacency convention only, or structural? What about a doc comment followed by annotations and then an element?
7. **Context variants.** The French example documents the same element in a second context. Does gramdoc need a language/context marker, or does the surrounding annotation carry that?
8. **pato / lint.** Should `pato` validate that `@property` names exist on the documented element?

## Next steps

1. Settle the questions above in this issue.
2. Implement `doc_comment` and the injection rule in tree-sitter-gram, with corpus tests.
3. Create `tree-sitter-gramdoc` in the gram-data org.
4. Follow-up in relateby/pattern-rs: retain doc comments through the CST and decide where they live at the `Pattern` level, then resume the ontology spike.
