# Implementation Plan: Unified Documentation Website

**Branch**: `049-doc-website` | **Date**: 2026-05-01 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `specs/049-doc-website/spec.md`

## Summary

Build a VitePress-based documentation site for the `pattern-rs` library that unifies Guides, Explanations, and API Reference across Rust, Python, and TypeScript into one coherent public site — adhering to the Diataxis framework as described in `proposals/doc-website-proposal.md`. The site persists the reader's language preference across visits, generates machine-readable `llms.txt` files for AI discoverability, and deploys automatically to GitHub Pages on pushes to `main`.

## Technical Context

**Language/Version**: TypeScript 5.x (VitePress site + TypeDoc), Shell (build script), Rust 1.70+ (cargo doc), Python 3.13 (pdoc)  
**Primary Dependencies**: VitePress (site framework), cargo doc (Rust API ref), pdoc (Python API ref), TypeDoc with `entryPointStrategy: "packages"` (TypeScript API ref)  
**Storage**: Static files; GitHub Pages hosting; `docs/public/reference/` generated at build time (gitignored)  
**Testing**: Build smoke test (single-command build completes without error), link validation, content review against spec acceptance scenarios  
**Target Platform**: GitHub Pages (static site, public URL)  
**Project Type**: Documentation website  
**Performance Goals**: Full site build completes in under 5 minutes from clean checkout; individual pages load in under 2 seconds  
**Constraints**: No server-side code; localStorage only for language preference; all content generated at build time from markdown sources  
**Scale/Scope**: ~20-30 markdown pages (10 guides + 10 explanations + 1 reference hub + 1 landing); 3 language API sub-sites

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

This feature is documentation infrastructure, not a Rust library port. Constitution principles are evaluated for applicability:

| Principle | Applicability | Status |
|-----------|--------------|--------|
| I. Reference Implementation Fidelity | Content accuracy: all code examples and API descriptions must match the actual `pattern-rs` and `pattern-hs` implementations | PASS — content authoring guidelines enforce this |
| II. Correctness & Compatibility | Code examples in guides must be correct and runnable; API reference must match the published API surface | PASS — examples derived from existing tests/usage docs |
| III. Rust Native Idioms | Rust examples in guides must demonstrate idiomatic Rust; site infrastructure (VitePress/TypeScript) is out of scope for this principle | PASS — not applicable to site tooling |
| IV. Multi-Target Library Design | The site explicitly documents all three targets (Rust, Python, TypeScript) as first-class citizens | PASS — spec FR-008 and FR-009 require this |
| V. External Language Bindings & Examples | The guide and explanation content IS the primary language binding documentation | PASS — covered by all guide pages |

**Gate result: PASS. No violations. No complexity tracking required.**

## Project Structure

### Documentation (this feature)

```text
specs/049-doc-website/
├── plan.md              # This file
├── research.md          # Phase 0 complete
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks)
```

### Source Code (repository root)

```text
docs/                           VitePress project root
├── .vitepress/
│   ├── config.ts               Site metadata, sidebar nav, base URL
│   └── theme/
│       ├── index.ts            Theme entry: registers useLanguageTab via enhanceApp
│       └── useLanguageTab.ts   localStorage composable for language tab persistence
├── index.md                    Landing page (overview + links to all sections)
├── guides/
│   ├── index.md                Guides index (auto-lists all guide pages)
│   └── *.md                    One file per guide question (~10 pages)
├── explanations/
│   ├── index.md                Explanations index (auto-lists all explanation pages)
│   └── *.md                    One file per explanation question (~10 pages)
├── reference/
│   └── index.md                Link hub: /reference/rust/, /reference/python/, /reference/ts/
└── public/
    ├── llms.txt                Generated LLM index (build artifact, gitignored)
    ├── llms-full.txt           Generated full prose dump (build artifact, gitignored)
    └── reference/              Built API docs (gitignored, generated at build time)
        ├── rust/               cargo doc output
        ├── python/             pdoc output
        └── ts/                 TypeDoc output (unified across 3 packages)

docs/scripts/
└── build.sh                    Single orchestration script for full site build

typedoc.json                    TypeDoc config (repo root); entryPointStrategy: "packages"

.github/workflows/
└── docs.yml                    GitHub Pages deployment (new; triggers on push to main)

# Existing developer docs remain in docs/*.md, not included in VitePress sidebar
# (accessible at direct URLs but not listed in navigation)
```

**Structure Decision**: Faithful to `proposals/doc-website-proposal.md` § "Repository Layout". The `typedoc.json` is placed at repo root (not inside `docs/`) so it resolves package paths relative to the workspace root. The `docs/scripts/` directory is added alongside the VitePress project to hold the build orchestration script.

## Phase 0: Research Summary

All unknowns resolved. See `research.md` for full rationale. Key decisions:

1. **Language tab persistence**: Custom `useLanguageTab.ts` composable via `enhanceApp` hook. No suitable plugin exists; upstream VitePress PR is unreleased. Matches proposal exactly.
2. **TypeDoc**: `entryPointStrategy: "packages"` covers `@relateby/pattern`, `@relateby/gram`, `@relateby/graph` in one run, producing a unified output at `docs/public/reference/ts/`.
3. **pdoc**: Reads existing `.pyi` stubs directly; single run covers `relateby.pattern` and `relateby.gram`.
4. **cargo doc**: `--workspace --no-deps`; output copied from `target/doc/` to `docs/public/reference/rust/`.
5. **LLM files**: Shell script segment in `build.sh`; no Node.js step required.
6. **Existing `docs/*.md`**: Remain in place; excluded from VitePress sidebar by configuration.
7. **GitHub Pages**: New `docs.yml` workflow; triggers on `main` push.

## Phase 1: Design

### Content Architecture

The site follows Diataxis strictly. The boundary between Guides and Explanations is enforced by heading grammar:

- **Guide headings** MUST begin with "How do I…" — task-oriented
- **Explanation headings** MUST begin with "What is…", "Why is…", "How does…", or "When should…" — understanding-oriented

Initial content set (from proposal sample question sets):

**Guides** (`docs/guides/`):

| Slug | Question |
|------|----------|
| `create-atomic-pattern` | How do I create an atomic pattern? |
| `create-pattern-with-elements` | How do I create a pattern with elements? |
| `give-pattern-a-value` | How do I give a pattern a value? |
| `parse-gram-notation` | How do I parse Gram notation into a pattern? |
| `serialize-gram-notation` | How do I serialize a pattern to Gram notation? |
| `traverse-pattern-elements` | How do I traverse the elements of a pattern? |
| `map-pattern-values` | How do I map over a pattern's values? |
| `build-graph-from-patterns` | How do I build a graph from patterns? |
| `merge-two-patterns` | How do I merge two patterns? |

**Explanations** (`docs/explanations/`):

| Slug | Question |
|------|----------|
| `what-is-pattern` | What is a Pattern? |
| `what-is-decorated-sequence` | What is a "decorated sequence"? |
| `why-pattern-not-tree` | Why is Pattern not a tree? |
| `what-is-subject` | What is a Subject? |
| `what-is-gram-notation` | What is Gram notation? |
| `gram-notation-and-pattern` | How does Gram notation relate to Pattern? |
| `atomic-vs-elements-pattern` | What is the difference between an atomic pattern and a pattern with elements? |
| `three-language-bindings` | How do the three language bindings relate to each other? |
| `when-to-use-pattern` | When should I use Pattern versus a plain graph library? |
| `what-is-v-in-pattern` | What does the `V` in `Pattern<V>` mean? |

### Build Pipeline

The `docs/scripts/build.sh` script runs the following steps in order. Any step failure aborts the build.

```
Step 1: cargo doc --workspace --no-deps
        Copy target/doc/ → docs/public/reference/rust/

Step 2: pdoc relateby --output-dir ../../docs/public/reference/python
        (run from python/packages/relateby/ with venv active)

Step 3: typedoc --options typedoc.json
        (outputs directly to docs/public/reference/ts/ per typedoc.json)

Step 4: Generate docs/public/llms.txt
        (shell: write header + iterate over explanation/guide .md files)

Step 5: Generate docs/public/llms-full.txt
        (shell: concatenate explanations/*.md then guides/*.md, strip frontmatter)

Step 6: vitepress build docs
        (outputs to docs/.vitepress/dist/)
```

### VitePress Configuration Sketch

`docs/.vitepress/config.ts` key settings:

- `base`: `/` (or repo-name prefix if GitHub Pages project page)
- `title`: `pattern-rs`
- `themeConfig.nav`: links to Guides, Explanations, Reference
- `themeConfig.sidebar`: manually configured for `/guides/` and `/explanations/`; no sidebar for `/reference/` (passthrough)
- `themeConfig.search`: VitePress built-in local search enabled
- Existing `docs/*.md` files excluded from sidebar via explicit sidebar configuration (they remain accessible by direct URL)

### Language Tab Persistence

`useLanguageTab.ts` composable:
1. On mount: reads `localStorage.getItem('relateby-lang-tab')` 
2. Applies stored label to all `.vp-code-group .tabs button` elements matching the stored label
3. On click: writes the clicked tab label to `localStorage`
4. Wraps all DOM access in `if (typeof window !== 'undefined')` guard for SSR

Registered in `docs/.vitepress/theme/index.ts` via:
```typescript
export default {
  ...DefaultTheme,
  enhanceApp({ app, router }) {
    if (!import.meta.env.SSR) {
      router.onAfterRouteChanged = () => applyLanguageTab()
    }
  }
}
```

### GitHub Pages Deployment

`.github/workflows/docs.yml`:
- Trigger: `push` to `main`
- Permissions: `pages: write`, `id-token: write`
- Steps: checkout → setup Rust → setup Node → setup Python → run `docs/scripts/build.sh` → upload artifact → deploy to Pages
- Environment: `github-pages`

### gitignore Additions

Add to repo `.gitignore`:
```
docs/public/reference/
docs/public/llms.txt
docs/public/llms-full.txt
docs/.vitepress/dist/
docs/.vitepress/cache/
typedoc.json  # NOT gitignored — this is source
```

## Complexity Tracking

*(No constitution violations — section not applicable)*
