# Quickstart: Unified Documentation Website (049-doc-website)

## Local Development

### 1. Install Node.js dependencies

```bash
cd docs
npm install
```

### 2. Start the dev server

```bash
npx vitepress dev docs
```

The site is available at `http://localhost:5173`. Hot-reload is active for markdown edits.

> **Note**: The dev server does not run the API reference generators. Pages that link to `/reference/rust/`, `/reference/python/`, or `/reference/ts/` will show 404 in dev mode until you run the full build.

### 3. Full build (with API reference)

```bash
./docs/scripts/build.sh
```

This runs all generators and produces the deployable site at `docs/.vitepress/dist/`.

Preview the built site:
```bash
npx vitepress preview docs
```

## Writing Content

### Adding a Guide

1. Create `docs/guides/<slug>.md` with this frontmatter and heading:
   ```markdown
   ---
   title: How do I create an atomic pattern?
   ---

   # How do I create an atomic pattern?

   [answer prose]

   ::: code-group

   ```rust [Rust]
   // idiomatic Rust example
   ```

   ```python [Python]
   # idiomatic Python example
   ```

   ```typescript [TypeScript]
   // idiomatic TypeScript example
   ```

   :::
   ```

2. Add the entry to `docs/guides/index.md` and update `docs/.vitepress/config.ts` sidebar.
3. The `llms.txt` and `llms-full.txt` files are regenerated automatically at the next full build.

### Adding an Explanation

Same structure as a Guide, but:
- The heading MUST begin with "What is…", "Why is…", "How does…", or "When should…"
- Code is optional and illustrative only — no task procedures
- File goes in `docs/explanations/<slug>.md`

## Verifying Content

- Tab labels must be exactly `Rust`, `Python`, `TypeScript` for language persistence to work
- Test persistence: select Python tab → reload page → Python tab should be pre-selected
- Run a full build to verify all API reference generators complete without error

## CI / Deployment

Push to `main` triggers `.github/workflows/docs.yml`, which runs the full build and deploys to GitHub Pages automatically. No manual deploy step is needed.
