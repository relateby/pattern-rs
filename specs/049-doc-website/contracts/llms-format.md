# Contract: LLM File Formats

The site exposes two machine-readable files following the [llmstxt.org](https://llmstxt.org/) convention. These are stable public interfaces for AI tools and automated documentation consumers.

## `/llms.txt` — Concise Index

Intended for inclusion in an LLM context window. Fits in ~2,000 tokens.

### Format

```
# pattern-rs

> [One-paragraph description of the library: what Pattern<V> is, what the library provides, 
> what languages are supported (Rust, Python, TypeScript), and what Gram notation is.]

## Explanations

- [What is a Pattern?](/explanations/what-is-pattern)
- [What is a "decorated sequence"?](/explanations/what-is-decorated-sequence)
- [Why is Pattern not a tree?](/explanations/why-pattern-not-tree)
- ... (one line per explanation page)

## Guides

- [How do I create an atomic pattern?](/guides/create-atomic-pattern)
- [How do I parse Gram notation into a pattern?](/guides/parse-gram-notation)
- ... (one line per guide page)

## API Reference

- [Rust API](/reference/rust/)
- [Python API](/reference/python/)
- [TypeScript API](/reference/ts/)
```

### Rules

- Section headers are `## Explanations`, `## Guides`, `## API Reference` (exactly)
- Each page is one line: `- [Question text](/path)`
- The question text is the exact H1 heading of the page (the question itself)
- Explanations section appears before Guides section
- No additional prose or commentary

## `/llms-full.txt` — Full Prose

Intended for single-request ingestion of all documentation. Suitable for retrieval-augmented generation.

### Format

```
[content of docs/explanations/what-is-pattern.md, frontmatter stripped]

---

[content of docs/explanations/what-is-decorated-sequence.md, frontmatter stripped]

---

... (all explanation pages in slug-alphabetical order within each section)

---

[content of docs/guides/create-atomic-pattern.md, frontmatter stripped]

---

... (all guide pages in slug-alphabetical order)
```

### Rules

- Explanations section appears before Guides section (conceptual before task)
- Within each section, pages are ordered alphabetically by slug
- Each page is separated by `\n---\n` (newline, three hyphens, newline)
- YAML frontmatter is stripped (lines between `---` delimiters at file start)
- Code blocks are preserved as-is (they are part of the answer)
- HTML is not present (content is pure markdown from source files)
- API reference HTML sub-sites are excluded entirely
- No table of contents, no header, no footer — raw concatenation only

## Stability Guarantee

Both files are regenerated at every build from source. Their format is a public contract:
- Adding new pages: new entries appear in both files at the next build (non-breaking)
- Removing pages: entries disappear (potentially breaking for cached consumers)
- Renaming slugs: URLs change in `llms.txt` (breaking for any consumer with stored links)
- Changing section names (`## Explanations`, etc.): breaking for any parser expecting them
