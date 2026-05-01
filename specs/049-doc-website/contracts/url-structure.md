# Contract: URL Structure

The site MUST serve content at these paths. This is the public interface — any change to these paths is a breaking change requiring redirects.

## Navigable Pages

| Path | Content | Source |
|------|---------|--------|
| `/` | Landing page | `docs/index.md` |
| `/guides/` | Guides index | `docs/guides/index.md` |
| `/guides/<slug>` | Individual guide | `docs/guides/<slug>.md` |
| `/explanations/` | Explanations index | `docs/explanations/index.md` |
| `/explanations/<slug>` | Individual explanation | `docs/explanations/<slug>.md` |
| `/reference/` | API reference hub | `docs/reference/index.md` |

## API Reference Sub-sites (Static Passthrough)

| Path | Generator | Source Copy |
|------|-----------|------------|
| `/reference/rust/` | cargo doc | `docs/public/reference/rust/` |
| `/reference/python/` | pdoc | `docs/public/reference/python/` |
| `/reference/ts/` | TypeDoc | `docs/public/reference/ts/` |

These paths serve the full HTML output of each documentation generator as static assets. VitePress passes them through without transformation.

## Machine-Readable Files

| Path | Format | Content |
|------|--------|---------|
| `/llms.txt` | llmstxt.org convention | Concise index of all pages for LLM context |
| `/llms-full.txt` | Plain text | Full prose of all guides and explanations |

## Slug Conventions

- Guide slugs: `<verb>-<object>` format, e.g., `create-atomic-pattern`, `parse-gram-notation`
- Explanation slugs: `<noun>-phrase` or `<question-words>-phrase` format, e.g., `what-is-pattern`, `why-pattern-not-tree`
- All slugs: lowercase kebab-case, no special characters, unique within their section
