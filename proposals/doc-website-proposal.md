# Documentation Website Proposal

## Overview

A unified documentation site for the `pattern-rs` library covering Rust, Python, and TypeScript APIs. The site adopts the [Diataxis](https://diataxis.fr/) framework, which provides a clear, principled separation of documentation modes. Three of the four modes are in scope.

## Framework Mapping

| Mode | Purpose | Implementation |
|------|---------|----------------|
| **Reference** | Authoritative API specification | Language-native tooling, served as static subpaths |
| **Guides** | Task-oriented Q&A ("How do I…") | `/guides/` — terse Q&A in VitePress markdown |
| **Explanation** | Conceptual Q&A ("What is…", "Why…") | `/explanations/` — terse Q&A in VitePress markdown |
| ~~Tutorials~~ | *Deferred* | Not in scope |

Both guides and explanations use a Q&A format: a question as the heading, a focused answer in prose and code. This mirrors how developers search for documentation and produces content that is useful as a training corpus for AI assistants.

The key Diataxis distinction to maintain: guides answer *how* (task-oriented, reader wants to act), explanations answer *what* and *why* (understanding-oriented, reader wants to think).

### Language Preference

Every multi-language code block uses VitePress's code groups, with tabs labelled `Rust`, `Python`, and `TypeScript`. A small custom composable stores the reader's last-selected language in `localStorage` and applies it as the default tab on every page load. The reader can override it at any time by clicking a different tab — the new choice is persisted immediately. This means a Python developer never has to re-select the Python tab after the first visit.

## Site Architecture

**Framework**: [VitePress](https://vitepress.dev/) — markdown-first, fast build, good sidebar navigation, and native support for serving arbitrary static assets alongside the main site.

**Subpath aggregation**: Each language's API docs are built independently by their native tooling and placed in VitePress's `public/` directory before the site build. VitePress serves them as static passthrough assets. No custom theming or cross-linking of the API docs is required — they are reference material, linked *to* from guides and explanations.

### URL Structure

```
/                               Landing page (overview, links to all sections)
/guides/                        How-to Q&A index
/guides/create-a-pattern        Example guide page
/explanations/                  Conceptual Q&A index
/explanations/what-is-pattern   Example explanation page
/reference/                     API reference hub (link page to all three)
/reference/rust/                rustdoc output (static passthrough)
/reference/python/              pdoc output (static passthrough)
/reference/ts/                  TypeDoc output (static passthrough)
```

## API Reference Toolchain

| Language | Tool | Notes |
|----------|------|-------|
| Rust | `cargo doc --no-deps` | Standard rustdoc; outputs to `target/doc/` |
| Python | [pdoc](https://pdoc.dev/) | Reads PyO3-generated stubs and docstrings; lightweight, no config required |
| TypeScript | [TypeDoc](https://typedoc.org/) | Reads `.d.ts` and JSDoc comments; HTML output |

pdoc is preferred over Sphinx for Python: the PyO3-generated surface is clean enough that Sphinx's cross-reference machinery adds complexity without benefit.

Each tool runs as part of the site build, with output copied to `docs/public/reference/{rust,python,ts}/`.

## Content Structure

### `/guides/` — How-to Q&A

Each page answers one task-oriented question. Where behavior or syntax differs across languages, a single page covers all three with language-tabbed code blocks. Pages should be completable in under two minutes of reading.

Sample question set (not exhaustive):
- How do I create an atomic pattern?
- How do I create a pattern with elements?
- How do I give a pattern a value?
- How do I parse Gram notation into a pattern?
- How do I serialize a pattern to Gram notation?
- How do I traverse the elements of a pattern?
- How do I map over a pattern's values?
- How do I build a graph from patterns?
- How do I query a graph?
- How do I merge two patterns?

### `/explanations/` — Conceptual Q&A

Each page answers one conceptual question. Code appears only to ground a concept, not to teach a task. Pages should be self-contained — no assumed reading order.

Sample question set (not exhaustive):
- What is a Pattern?
- What is a "decorated sequence"?
- Why is Pattern not a tree?
- What is a Subject?
- What is Gram notation?
- How does Gram notation relate to Pattern?
- What is the difference between an atomic pattern and a pattern with elements?
- How do the three language bindings relate to each other?
- When should I use Pattern versus a plain graph library?
- What does the `V` in `Pattern<V>` mean?

## LLM Discoverability

The site generates two files per the [llms.txt](https://llmstxt.org/) convention:

**`/llms.txt`** — a concise index suitable for inclusion in an LLM context window. Contains:
- One-paragraph description of Pattern and the library
- Bulleted links to every guide and explanation page, with the question as the link text
- Link to each language's API reference

**`/llms-full.txt`** — the full concatenated text of all guides and explanations in a single file, ordered conceptually (explanations first, then guides). This allows an LLM to ingest the entire prose documentation in one pass without crawling.

Both files are generated at build time from the VitePress content source. The Q&A structure of the prose content means `llms-full.txt` is already well-suited for retrieval — each question/answer pair is a self-contained chunk.

The API reference subdirs (rustdoc, pdoc, TypeDoc HTML) are excluded from `llms-full.txt`; LLM consumers should use the source `.md` stubs and type signatures instead, which are available in the repository.

## Repository Layout

```
docs/                          VitePress project root
├── .vitepress/
│   ├── config.ts              Sidebar nav, site metadata
│   └── theme/
│       └── useLanguageTab.ts  localStorage composable for language preference
├── index.md                   Landing page
├── guides/
│   └── *.md                   One file per guide question
├── explanations/
│   └── *.md                   One file per explanation question
├── reference/
│   └── index.md               Link hub to /reference/rust/, /python/, /ts/
└── public/
    ├── llms.txt               Generated LLM index (build artifact)
    ├── llms-full.txt          Generated full prose dump (build artifact)
    └── reference/             Built API docs (gitignored, generated at build time)
        ├── rust/
        ├── python/
        └── ts/
```

## Build Script

A single script (`docs/scripts/build.sh`) orchestrates the full site build:

1. `cargo doc --no-deps --workspace` → copy `target/doc/` to `docs/public/reference/rust/`
2. `pdoc relateby` → output to `docs/public/reference/python/`
3. `typedoc` → output to `docs/public/reference/ts/`
4. Generate `docs/public/llms.txt` and `docs/public/llms-full.txt` from markdown sources
5. `vitepress build docs`

CI publishes the `docs/.vitepress/dist/` output to GitHub Pages on pushes to `main`.

## Open Questions

**Reference landing page**: Should `/reference/` embed API indexes inline or just link out? Linking out is simpler and avoids staleness. Inline embedding would require periodic scraping or a unified doc format — not worth the complexity at this stage.

**Versioning**: API docs from `cargo doc` reflect the current workspace. No versioned docs in scope for the initial site; a `vitepress-plugin-versioning` approach can be added later.
