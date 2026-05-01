# Research: Unified Documentation Website (049-doc-website)

## VitePress Code Group Tab Persistence

**Decision**: Implement custom `useLanguageTab.ts` composable registered via the VitePress `enhanceApp` hook.

**Rationale**: VitePress v1 has no built-in `localStorage` persistence for code-group tab selection. A PR (#5012) is in progress upstream but is not released. The `vitepress-plugin-tabs` and `vitepress-plugin-code-group` community plugins do not provide cross-page persistence. The proposal's custom composable approach is the correct choice and avoids an external plugin dependency for a small, well-defined behaviour.

**Implementation details**:
- Markdown syntax: `::: code-group` fenced blocks with `[Rust]`, `[Python]`, `[TypeScript]` tab labels
- The composable intercepts tab-click events on code groups, writes the selected label to `localStorage`, and on page load applies the stored label to all code groups on the page
- Integration point: `.vitepress/theme/index.ts` → `enhanceApp({ app })` hook
- SSR guard required: `if (!import.meta.env.SSR)` before `localStorage` access

**Alternatives considered**: Waiting for upstream PR (uncertain timeline), `vitepress-plugin-tabs` (no persistence), `vitepress-plugin-code-group` (3 years old, no persistence).

---

## TypeDoc Multi-Package Configuration

**Decision**: Use TypeDoc `entryPointStrategy: "packages"` with a single `typedoc.json` at the repo root, covering all three TypeScript packages in one build run.

**Rationale**: TypeDoc's packages strategy is designed for exactly this monorepo layout. It processes each package directory independently using the package's own `tsconfig.json`, then merges all declarations into a single unified HTML site with cross-package links. Three packages, one command, one output directory.

**Packages covered**:
- `typescript/packages/pattern` → `@relateby/pattern` (`Pattern<V>`, `Subject`, `StandardGraph`, etc.)
- `typescript/packages/gram` → `@relateby/gram` (`parse`, `stringify`, etc.)
- `typescript/packages/graph` → `@relateby/graph` (graph interfaces and transforms)

**Concrete config** (`typedoc.json` at repo root):
```json
{
  "entryPointStrategy": "packages",
  "entryPoints": [
    "typescript/packages/pattern",
    "typescript/packages/gram",
    "typescript/packages/graph"
  ],
  "packageOptions": {
    "entryPoints": ["src/index.ts"],
    "tsconfig": "tsconfig.json"
  },
  "out": "docs/public/reference/ts",
  "githubPages": false
}
```

**Alternatives considered**: Three separate TypeDoc runs with separate output dirs (would require a `/reference/ts/` link hub page listing three sub-sites); TypeDoc monorepo plugin (unnecessary given built-in packages strategy).

---

## Python API Reference: pdoc vs Sphinx

**Decision**: Use `pdoc` as specified in the proposal. No deviation warranted.

**Rationale**: The `relateby` Python package has clean `.pyi` stubs at `python/packages/relateby/relateby/pattern/__init__.pyi` and `relateby/gram/__init__.pyi`. These stubs fully declare the public API surface including generics, overloads, and type aliases. `pdoc` reads `.pyi` files directly and renders them without configuration. Sphinx would add `conf.py`, extension management, and cross-reference machinery that the PyO3-generated surface does not benefit from.

**Build command**: `pdoc relateby --output-dir docs/public/reference/python` (run from `python/packages/relateby/` with the package installed or venv active)

**Output**: pdoc generates a self-contained HTML site with navigation for all subpackages (`relateby.pattern`, `relateby.gram`).

---

## Rust API Reference

**Decision**: `cargo doc --workspace --no-deps` as specified in the proposal.

**Crate names and output paths**:
- `relateby-pattern` → `target/doc/relateby_pattern/`
- `relateby-gram` → `target/doc/relateby_gram/`
- `relateby-pato` → `target/doc/relateby_pato/` (CLI tool; may be excluded from public API docs with `--exclude`)

**Copy to**: `docs/public/reference/rust/` (full `target/doc/` contents — includes cross-crate index)

---

## Existing `docs/` Content

**Decision**: The existing `docs/*.md` files (porting-guide, python-usage, wasm-usage, etc.) are developer-facing internal documentation. They remain in place and are accessible via direct URL within the VitePress site, but are not included in the main sidebar navigation.

**Rationale**: VitePress renders all markdown files it finds in the source directory, but the sidebar is manually configured. Existing files will be accessible at their natural paths (e.g., `/python-usage`) for internal links in `CLAUDE.md` without modification. No files need to be moved. A future task could organise them under a `/developer/` section if desired.

---

## GitHub Pages Deployment

**Decision**: New `.github/workflows/docs.yml` workflow using `actions/configure-pages`, `actions/upload-pages-artifact`, and `actions/deploy-pages`. Triggered on push to `main`.

**Rationale**: The repo has `ci.yml` and `publish.yml` but no Pages deployment workflow. A dedicated `docs.yml` is cleanest — it isolates the docs build from the library CI pipeline and can be independently disabled or modified.

---

## LLM File Generation

**Decision**: Shell script segment within `docs/scripts/build.sh` that reads all markdown sources and concatenates them.

**llms.txt format** (per [llmstxt.org](https://llmstxt.org/)):
```
# pattern-rs

> [one-paragraph library description]

## Explanations

- [What is a Pattern?](/explanations/what-is-pattern)
- ...

## Guides

- [How do I create an atomic pattern?](/guides/create-atomic-pattern)
- ...

## API Reference

- [Rust](/reference/rust/)
- [Python](/reference/python/)
- [TypeScript](/reference/ts/)
```

**llms-full.txt**: Raw concatenation of all `docs/explanations/*.md` then `docs/guides/*.md` in conceptual order, separated by `---` dividers, with YAML frontmatter stripped.

**Generation approach**: `find docs/explanations docs/guides -name '*.md' | sort | xargs` with a simple sed/awk pass to strip frontmatter and inject dividers. No Node.js build step required.
