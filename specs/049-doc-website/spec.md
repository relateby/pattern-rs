# Feature Specification: Unified Documentation Website

**Feature Branch**: `049-doc-website`  
**Created**: 2026-05-01  
**Status**: Draft  
**Input**: Unified documentation for concepts and all 3 languages (Rust, Python, TypeScript) as described in `proposals/doc-website-proposal.md`.

## Overview

A unified documentation site for the `pattern-rs` library that serves Rust, Python, and TypeScript developers from a single, coherent source. The site organises content into three Diataxis modes — Guides, Explanations, and Reference — so that each type of question a developer might have is answered in the right place and style. The site must also be machine-readable, producing structured files that allow AI assistants to ingest the entire documentation in a single pass.

The proposal (`proposals/doc-website-proposal.md`) is the authoritative source for architectural decisions. Any departure from the proposal requires explicit justification before implementation.

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Developer Answers a "How Do I…" Question (Priority: P1)

A developer working in any of the three supported languages wants to accomplish a specific task — for example, "How do I create a pattern with elements?" — and needs a quick, focused answer with working code in their language.

**Why this priority**: This is the most common documentation use case. Guides are the first thing a developer reaches for when stuck on a task, and high-quality guides directly reduce time-to-productivity. Without guides, all other site content has no entry point for the majority of developers.

**Independent Test**: The guides section can be built and deployed independently of explanations and API reference. A site with only `/guides/` already delivers substantial value and is a shippable MVP.

**Acceptance Scenarios**:

1. **Given** a developer searching for a task-oriented answer, **When** they visit `/guides/`, **Then** they see an index listing every available how-to question, ordered by topic.
2. **Given** a developer on a guide page, **When** they read the page, **Then** the answer is presented as a focused Q&A: the question is the page heading, the answer is prose plus one or more code blocks, and the page can be read in under two minutes.
3. **Given** a developer using Python, **When** they view a guide with multi-language code examples, **Then** the Python tab is shown by default (or their previously selected language is restored automatically).
4. **Given** a developer who selects the Rust tab on any guide page, **When** they navigate to any other page on the site, **Then** the Rust tab is pre-selected on all subsequent code blocks without any further action.

---

### User Story 2 — Developer Understands a Concept (Priority: P2)

A developer wants to understand *what* something is or *why* it works the way it does — for example, "What is a Pattern?" or "Why is Pattern not a tree?" — and needs a conceptual explanation that builds accurate mental models rather than teaching a task.

**Why this priority**: Conceptual understanding makes Guides more effective and reduces repeated misuse of the library. Explanations answer the "why" questions that cause long-term confusion when unanswered. They are prioritised after Guides because developers reach for task help first.

**Independent Test**: The explanations section can be built and deployed independently. Each page is self-contained with no assumed reading order.

**Acceptance Scenarios**:

1. **Given** a developer on the `/explanations/` index, **When** they scan the page, **Then** every listed question is phrased as a "What is…", "Why is…", or similar understanding-oriented question (not a task).
2. **Given** a developer reading an explanation page, **When** they finish the page, **Then** they have encountered no step-by-step task instructions — code appears only to illustrate a concept, never to teach a procedure.
3. **Given** a developer who arrives at an explanation page from a search engine, **When** they read the page, **Then** the page is self-contained and requires no other page to be read first.
4. **Given** a developer on a guide page, **When** the guide references a concept, **Then** the guide links to the corresponding explanation page rather than re-explaining the concept inline.

---

### User Story 3 — Developer Looks Up an API Symbol (Priority: P3)

A developer knows the name of a function, type, or method and wants the authoritative specification: its signature, parameters, return type, and any documented constraints.

**Why this priority**: API reference is essential for experienced users but presupposes that the developer already knows what to look for. Guides and Explanations create the context that makes Reference useful.

**Independent Test**: The `/reference/` section and its three language sub-paths can be built independently. A developer can navigate directly to `/reference/rust/`, `/reference/python/`, or `/reference/ts/` and find the complete API surface for that language.

**Acceptance Scenarios**:

1. **Given** a developer on `/reference/`, **When** they view the page, **Then** they see clear links to each of the three language API references (Rust, Python, TypeScript).
2. **Given** a developer who follows the Rust reference link, **When** the page loads, **Then** they see standard Rust documentation generated from the source code, covering all public types and functions.
3. **Given** a developer who follows the Python reference link, **When** the page loads, **Then** they see Python API documentation covering the `relateby.pattern` and `relateby.gram` public surfaces.
4. **Given** a developer who follows the TypeScript reference link, **When** the page loads, **Then** they see TypeScript API documentation covering all exported types and functions.
5. **Given** a guide or explanation page that references a specific API symbol, **When** the developer clicks the reference link, **Then** they land directly on that symbol's documentation page.

---

### User Story 4 — AI Tool Consumes the Documentation (Priority: P4)

An AI assistant or automated tool needs to ingest the library's documentation to answer developer questions accurately. The tool should be able to access a machine-optimised index and a full-text dump without crawling individual pages.

**Why this priority**: The proposal explicitly calls for `llms.txt` discoverability as a first-class feature, not an afterthought. This enables AI-assisted development workflows that rely on this library and makes the documentation corpus available as a training or retrieval resource.

**Independent Test**: The two generated files (`/llms.txt` and `/llms-full.txt`) can be validated independently of the rendered site by checking their content against the source markdown files.

**Acceptance Scenarios**:

1. **Given** an AI tool requesting `/llms.txt`, **When** the file is served, **Then** it contains a one-paragraph library description, a bulleted list of every guide and explanation page (with the question as link text), and links to each language's API reference — all in a format suitable for inclusion in an LLM context window.
2. **Given** an AI tool requesting `/llms-full.txt`, **When** the file is served, **Then** it contains the complete prose text of all guides and explanations in a single document, ordered conceptually (explanations first, then guides), without any HTML markup.
3. **Given** the site build completes, **When** either file is inspected, **Then** its content matches the current source markdown files — it is not stale from a previous build.
4. **Given** a reader of `llms-full.txt`, **When** they scan the document, **Then** each guide and explanation appears as a self-contained question-and-answer chunk, usable as an independent retrieval unit.

---

### Edge Cases

- What happens when a guide's code example differs meaningfully between Rust, Python, and TypeScript (e.g., due to language idioms)? Each language tab shows the idiomatic version for that language; the page still answers one question.
- What happens if a reader visits the site for the first time with no stored language preference? The default tab order is Rust, Python, TypeScript; Rust is shown first.
- What happens if a reader's browser blocks `localStorage`? The site falls back to the default tab order on every page load; no error is shown.
- What happens when a new guide or explanation page is added? The landing page index, sidebar navigation, and both `llms.txt` files are all updated at the next build — no manual registration step is required.
- What happens if one of the three API reference builds fails? The site build fails loudly rather than silently serving stale or missing reference docs.

## Requirements *(mandatory)*

### Functional Requirements

**Site Structure**

- **FR-001**: The site MUST serve content at the following top-level paths: `/` (landing), `/guides/`, `/explanations/`, `/reference/`, `/reference/rust/`, `/reference/python/`, `/reference/ts/`.
- **FR-002**: The landing page MUST provide a clear overview of the library and link to all major sections (guides, explanations, reference).
- **FR-003**: The `/guides/` index MUST list every guide page with its question as the link text.
- **FR-004**: The `/explanations/` index MUST list every explanation page with its question as the link text.
- **FR-005**: The `/reference/` page MUST link to each of the three language API references.

**Content Model**

- **FR-006**: Each guide page MUST answer exactly one task-oriented question. The question MUST appear as the page heading.
- **FR-007**: Each explanation page MUST answer exactly one conceptual question. The question MUST appear as the page heading and MUST be phrased to address understanding (what, why, how does), not task execution (how do I).
- **FR-008**: Guide pages MUST cover all three languages wherever behaviour differs; explanation pages MUST use code only to ground a concept, not to teach a task.
- **FR-009**: Multi-language code blocks MUST be presented as tabbed groups with tabs labelled `Rust`, `Python`, and `TypeScript`.

**Language Preference**

- **FR-010**: The site MUST persist the reader's most recently selected language tab across page loads using browser storage.
- **FR-011**: On every page load, the persisted language MUST be automatically applied as the default tab selection for all code groups on that page.
- **FR-012**: The reader MUST be able to override the persisted language on any page by selecting a different tab; the new selection MUST be persisted immediately.

**API Reference**

- **FR-013**: Rust API documentation MUST be generated from the workspace source code and served at `/reference/rust/`.
- **FR-014**: Python API documentation MUST be generated from the `relateby` package source and served at `/reference/python/`.
- **FR-015**: TypeScript API documentation MUST be generated from the TypeScript package sources and served at `/reference/ts/`.
- **FR-016**: All three API reference builds MUST run as part of a single build invocation; a failure in any one MUST abort the full build.

**LLM Discoverability**

- **FR-017**: The site MUST serve `/llms.txt` containing a library description, a bulleted index of all guide and explanation pages (question as link text), and links to each language's API reference.
- **FR-018**: The site MUST serve `/llms-full.txt` containing the complete prose text of all guides and explanations in a single document, ordered with explanations first, then guides.
- **FR-019**: Both LLM files MUST be generated automatically from the source markdown at build time; manual maintenance is not acceptable.
- **FR-020**: The HTML API reference output MUST be excluded from `llms-full.txt`; only prose markdown content is included.

**Build and Deployment**

- **FR-021**: A single build script MUST orchestrate the full build in order: Rust API docs → Python API docs → TypeScript API docs → LLM files → site build.
- **FR-022**: The site MUST be deployable to GitHub Pages from the output of the build script.
- **FR-023**: Deployment MUST trigger automatically on pushes to the `main` branch via CI.

### Key Entities

- **Guide Page**: A documentation page answering one task-oriented question. Has a question heading, prose answer, and language-tabbed code blocks. Lives at `/guides/<slug>`.
- **Explanation Page**: A documentation page answering one conceptual question. Has a question heading and prose answer; code is incidental. Lives at `/explanations/<slug>`.
- **Language Tab Group**: A code block with three tabs (Rust, Python, TypeScript) presenting equivalent functionality in each language.
- **Language Preference**: A browser-persisted setting recording the reader's last-selected language tab. Applied automatically on every page load.
- **API Reference Sub-site**: Static HTML documentation for one language, generated from source code and served as a passthrough asset. Three instances: Rust, Python, TypeScript.
- **LLM Index** (`llms.txt`): A machine-readable file listing all documentation pages with questions as link text, suitable for inclusion in an LLM context window.
- **LLM Full Text** (`llms-full.txt`): A machine-readable file containing the concatenated prose text of all documentation pages, ordered conceptually.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A developer can find a relevant guide or explanation page from the site index in under 30 seconds without using search.
- **SC-002**: Every guide page can be read and understood in under two minutes.
- **SC-003**: A developer's language preference is automatically applied on their second and all subsequent visits, requiring zero manual tab selections after the first.
- **SC-004**: All three language API references are reachable from the `/reference/` page in two clicks or fewer.
- **SC-005**: An AI tool can retrieve the full prose documentation of the library in a single HTTP request to `/llms-full.txt`.
- **SC-006**: The full site build — including all three API reference docs and LLM files — completes successfully from a clean checkout using a single command.
- **SC-007**: The site is published automatically to a public URL within 10 minutes of a merge to `main`.
- **SC-008**: Every guide and explanation page in the initial launch covers the questions listed in the proposal's sample question sets.

## Assumptions

- The initial launch does not include versioned documentation; all content reflects the current workspace state.
- Tutorials (the fourth Diataxis mode) are explicitly out of scope per the proposal.
- The `/reference/` sub-sites link out to their hosted paths rather than embedding API content inline; this avoids staleness from any caching or scraping approach.
- The reader's language preference is stored in `localStorage`; no server-side session or account is required.
- The site is publicly accessible without authentication.
- `pdoc` is used for Python API docs in preference to Sphinx, as justified in the proposal: the PyO3-generated surface is clean enough that Sphinx cross-reference machinery adds complexity without benefit.
