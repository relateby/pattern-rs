# Contract: Build Interface

The documentation build is a single-command operation. This contract defines the stable interface between the build script and its callers (developers, CI).

## Invocation

```bash
./docs/scripts/build.sh
```

Run from the repository root. No arguments. No environment configuration required beyond a working Rust toolchain, Python virtualenv with `pdoc` installed, and Node.js with `typedoc` and `vitepress` available.

## Prerequisites (caller's responsibility)

| Tool | Where to find |
|------|--------------|
| `cargo` | Rust toolchain (rustup) |
| `pdoc` | `pip install pdoc` in the Python virtualenv |
| `typedoc` | `npm install` in the VitePress project or globally |
| `vitepress` | `npm install` in `docs/` |
| Node.js 18+ | Required by VitePress |
| Python 3.8+ | Required by pdoc |

## Outputs

| Artifact | Path | Notes |
|----------|------|-------|
| Rust API docs | `docs/public/reference/rust/` | Overwritten on each run |
| Python API docs | `docs/public/reference/python/` | Overwritten on each run |
| TypeScript API docs | `docs/public/reference/ts/` | Overwritten on each run |
| LLM index | `docs/public/llms.txt` | Overwritten on each run |
| LLM full text | `docs/public/llms-full.txt` | Overwritten on each run |
| Site output | `docs/.vitepress/dist/` | Deploy this directory to GitHub Pages |

## Exit Behaviour

- Exit code `0`: all steps succeeded; `docs/.vitepress/dist/` is ready to deploy
- Exit code non-zero: at least one step failed; output is incomplete and must not be deployed
- On failure: the script prints the failing step name to stderr before exiting

## Step Order (must not be reordered)

1. `cargo doc --workspace --no-deps` + copy
2. `pdoc relateby` + place in output path
3. `typedoc --options typedoc.json`
4. Generate `llms.txt`
5. Generate `llms-full.txt`
6. `vitepress build docs`

Steps 1–5 must complete before step 6 because VitePress copies `docs/public/` into `dist/` during build.

## CI Integration

The `.github/workflows/docs.yml` workflow calls this script directly. The workflow is the only caller that deploys the output. Local runs produce the same output but do not deploy.
