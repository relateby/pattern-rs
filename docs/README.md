# docs/

The public documentation site for `pattern-rs`, built with [VitePress](https://vitepress.dev).

## Directory layout

```
docs/
├── .vitepress/
│   ├── config.ts          # Site config: title, nav, sidebar, srcExclude
│   └── theme/
│       ├── index.ts       # Theme entry: registers language-tab persistence
│       └── useLanguageTab.ts  # localStorage composable for code-group tabs
├── contributor/           # Internal docs (not published)
├── explanations/          # Conceptual pages ("What is…", "Why is…")
├── guides/                # Task-oriented pages ("How do I…")
├── reference/             # Link hub to generated API sub-sites
├── public/                # Static assets (gitignored: generated API docs, llms files)
├── scripts/
│   └── build.sh           # Full build: API docs → LLM files → VitePress
├── index.md               # Landing page
└── package.json           # VitePress + TypeDoc dependencies
```

`docs/contributor/` holds porting guides, release notes, and dev references. It is excluded from the VitePress build and never appears in the published site.

`docs/public/reference/` is generated at build time (gitignored). Run `./docs/scripts/build.sh` locally to populate it.

## Local preview

Install dependencies once:

```sh
cd docs && npm install
```

**Quick preview** (VitePress content only, no API reference or LLM files):

```sh
cd docs && npm run dev
```

Opens at <http://localhost:5173>. Hot-reloads on markdown edits. The `/reference/` links will be dead until you run the full build.

**Full build** (API reference + LLM files + VitePress):

```sh
# Prerequisite: Python venv with pdoc installed
cd python/packages/relateby && uv venv --python 3.13 .venv && uv pip install '.[dev]' && pip install pdoc

# From repo root:
./docs/scripts/build.sh
```

Then preview the built site:

```sh
cd docs && npm run preview
```

Opens at <http://localhost:4173> with the full site including API reference sub-sites.

## Publishing

The site is published automatically to GitHub Pages by `.github/workflows/docs.yml` on every push to `main`.

The workflow:
1. Checks out the repo and installs Rust, Node 20, and Python 3.13
2. Creates a Python venv and installs `pdoc`
3. Runs `./docs/scripts/build.sh`, which:
   - Generates Rust API docs with `cargo doc` → `docs/public/reference/rust/`
   - Generates Python API docs with `pdoc` → `docs/public/reference/python/`
   - Generates TypeScript API docs with TypeDoc → `docs/public/reference/ts/`
   - Generates `docs/public/llms.txt` and `docs/public/llms-full.txt`
   - Builds the VitePress site → `docs/.vitepress/dist/`
4. Uploads `docs/.vitepress/dist/` as a Pages artifact and deploys it

No manual steps are needed — merging to `main` is sufficient to publish.
