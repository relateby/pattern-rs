# Data Model: Unified Documentation Website (049-doc-website)

The documentation site has no database. Its "data" is content — markdown files and generated assets. The entities below describe the content types, their structure, and how they relate.

## Content Entities

### Guide Page

A markdown file in `docs/guides/<slug>.md` answering one task-oriented question.

| Field | Type | Constraints |
|-------|------|-------------|
| `slug` | string | URL-safe, kebab-case; unique within `/guides/` |
| `title` | string | MUST begin with "How do I…" |
| `question` | string | Identical to `title`; used as the H1 heading |
| `answer` | prose + code blocks | Completable in under 2 minutes; uses language tab groups for multi-language code |
| `frontmatter.title` | string | Used by VitePress for sidebar and `<title>` tag |

**Validation rules**:
- `title` MUST start with "How do I"
- MUST contain at least one language tab group (`::: code-group`) when behaviour differs across languages
- MUST NOT contain step-by-step task instructions that also appear in another guide (no duplication)
- Links to related explanations SHOULD be present when a concept is referenced

**Relationships**: Links to → Explanation Pages (for concept references); Links from ← Guides index page, sidebar, `llms.txt`

---

### Explanation Page

A markdown file in `docs/explanations/<slug>.md` answering one conceptual question.

| Field | Type | Constraints |
|-------|------|-------------|
| `slug` | string | URL-safe, kebab-case; unique within `/explanations/` |
| `title` | string | MUST begin with "What is…", "Why is…", "How does…", or "When should…" |
| `question` | string | Identical to `title`; used as the H1 heading |
| `answer` | prose (+ optional illustrative code) | Self-contained; no assumed reading order; code appears only to ground a concept |
| `frontmatter.title` | string | Used by VitePress for sidebar and `<title>` tag |

**Validation rules**:
- `title` MUST NOT begin with "How do I" (that phrasing belongs in Guides)
- Code blocks, if present, illustrate concepts — they do not teach procedures
- MUST NOT cross-reference another explanation as a prerequisite
- SHOULD use language tab groups when illustrating the same concept in multiple languages

**Relationships**: Links from ← Guide Pages (concept references); Links from ← Explanations index, sidebar, `llms.txt`

---

### Language Tab Group

A multi-language code block using VitePress `::: code-group` syntax, appearing inside Guide or Explanation pages.

| Field | Type | Constraints |
|-------|------|-------------|
| `tabs` | list of `{label, code}` | MUST include `[Rust]`, `[Python]`, and `[TypeScript]` tabs unless a language genuinely has no equivalent |
| `label` | string | One of exactly: `Rust`, `Python`, `TypeScript` |
| `code` | fenced code block | Idiomatic for the target language; compilable/runnable |

**Validation rules**:
- Tab labels MUST use exact capitalisation: `Rust`, `Python`, `TypeScript`
- Each language tab shows the idiomatic version for that language — not a literal translation
- Missing a language tab is acceptable only when the feature does not exist in that language (document why)

---

### Language Preference

A browser-side persisted value with no server representation.

| Field | Type | Constraints |
|-------|------|-------------|
| `localStorage key` | `'relateby-lang-tab'` | Fixed key name used by `useLanguageTab.ts` |
| `value` | string | One of: `'Rust'`, `'Python'`, `'TypeScript'`; defaults to `'Rust'` if absent or invalid |

---

### API Reference Sub-site (× 3)

Three independently generated static HTML sites, served as passthrough assets.

| Language | Source | Output Path | Generator |
|----------|--------|-------------|-----------|
| Rust | `crates/pattern-core/`, `crates/gram-codec/` | `docs/public/reference/rust/` | `cargo doc --workspace --no-deps` |
| Python | `python/packages/relateby/relateby/` | `docs/public/reference/python/` | `pdoc relateby` |
| TypeScript | `typescript/packages/pattern/`, `gram/`, `graph/` | `docs/public/reference/ts/` | `typedoc --options typedoc.json` |

**Constraints**:
- Generated at build time; gitignored; never manually edited
- Must reflect the current workspace state at build time
- Failure to generate any one aborts the build

---

### LLM Index (`llms.txt`)

Generated file at `docs/public/llms.txt`, served at `/llms.txt`.

| Section | Content |
|---------|---------|
| Header | `# pattern-rs` + one-paragraph library description |
| Explanations list | Bulleted links: `- [Question text](/explanations/<slug>)` |
| Guides list | Bulleted links: `- [Question text](/guides/<slug>)` |
| API Reference list | Links to `/reference/rust/`, `/reference/python/`, `/reference/ts/` |

**Constraints**:
- Generated from source markdown frontmatter; never manually edited
- Format follows [llmstxt.org](https://llmstxt.org/) convention
- Must be regenerated at every build; stale content is a build defect

---

### LLM Full Text (`llms-full.txt`)

Generated file at `docs/public/llms-full.txt`, served at `/llms-full.txt`.

| Property | Value |
|----------|-------|
| Order | Explanations first (conceptual ordering), then Guides (task ordering) |
| Separator | `---` between each page |
| Content | Raw markdown prose (frontmatter stripped, HTML API reference excluded) |
| Format | Plain text; no HTML markup; each Q&A pair is a self-contained chunk |

**Constraints**:
- Generated from source markdown; never manually edited
- API reference HTML is excluded
- Each chunk (question + answer) must be independently meaningful as a retrieval unit

## State Transitions

The only stateful entity is the Language Preference:

```
[no preference stored]
       │
       │ first tab click
       ▼
[preference = 'Rust' | 'Python' | 'TypeScript']
       │
       │ tab click on any page
       ▼
[preference = new selection]   (value replaced, never deleted by the site)
```

On every page load: stored preference → applied to all code groups on page.  
On tab click: new selection → replaces stored preference → applied immediately.  
On absent/invalid storage value: defaults to `'Rust'` (first tab in group order).
