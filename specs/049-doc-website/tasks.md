# Tasks: Unified Documentation Website

**Input**: Design documents from `/specs/049-doc-website/`
**Prerequisites**: plan.md ✓, spec.md ✓, research.md ✓, data-model.md ✓, contracts/ ✓

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story. No test tasks are included (not requested in spec).

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies on incomplete tasks)
- **[Story]**: User story this task belongs to (US1 = Guides, US2 = Explanations, US3 = API Reference, US4 = LLM Files)
- Exact file paths are included in every task description

---

## Phase 1: Setup (Project Initialization)

**Purpose**: Create the VitePress project skeleton and build script scaffolding

- [ ] T001 Create `docs/package.json` with VitePress and TypeDoc as dev dependencies (VitePress v1.x, TypeDoc v0.26+), including `scripts.dev`, `scripts.build`, and `scripts.preview` entries
- [ ] T002 [P] Create `docs/scripts/build.sh` as a `#!/usr/bin/env bash` script with `set -euo pipefail`, descriptive `echo` for each of the 6 build steps, and `exit 0` — steps are stubs at this point; placeholder for each: (1) cargo doc, (2) pdoc, (3) typedoc, (4) llms.txt, (5) llms-full.txt, (6) vitepress build
- [ ] T003 [P] Add generated paths to the root `.gitignore`: `docs/public/reference/`, `docs/public/llms.txt`, `docs/public/llms-full.txt`, `docs/.vitepress/dist/`, `docs/.vitepress/cache/`
- [ ] T004 Run `npm install` inside `docs/` to create `docs/package-lock.json`

---

## Phase 2: Foundational (VitePress Core + Language Preference)

**Purpose**: Working VitePress site with language tab persistence — required before any content page can be tested

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [ ] T005 Create `docs/.vitepress/config.ts` with: `title: "pattern-rs"`, base URL `/`, nav links to Guides / Explanations / Reference, full sidebar for `/guides/` listing all 9 guide page slugs (from plan.md content architecture), full sidebar for `/explanations/` listing all 10 explanation page slugs, `search: { provider: "local" }`, markdown `lineNumbers: true`
- [ ] T006 Create `docs/.vitepress/theme/useLanguageTab.ts` — exports `applyLanguageTab()` and `registerLanguageTabListeners()`: `applyLanguageTab` reads `localStorage.getItem('relateby-lang-tab')` and clicks the matching `.vp-code-group .tabs button` on the current page; `registerLanguageTabListeners` adds a delegated `click` listener on `document` for `.vp-code-group .tabs button` that writes the clicked label to `localStorage`; all DOM access guarded with `typeof window !== 'undefined'`
- [ ] T007 Create `docs/.vitepress/theme/index.ts` — re-exports `DefaultTheme`, overrides `Layout` to call `applyLanguageTab()` on `onMounted` (via `useRoute` watch), and uses `enhanceApp({ router })` to call `registerLanguageTabListeners()` after each route change; both composable functions imported from `./useLanguageTab`
- [ ] T008 Create `docs/index.md` — landing page with VitePress hero config in YAML frontmatter (`hero.name`, `hero.tagline`, `hero.actions` linking to `/guides/` and `/explanations/`), followed by a prose overview of the library (what `Pattern<V>` is, what the library provides, the three supported languages) and a Features section linking to Guides, Explanations, and Reference
- [ ] T009 [P] Create `docs/reference/index.md` — heading "API Reference", brief description, then three cards or a list linking to `/reference/rust/` (Rust), `/reference/python/` (Python), `/reference/ts/` (TypeScript); note that these sub-sites are generated at build time

**Checkpoint**: Run `npm run dev` in `docs/` — site loads at localhost:5173, language tab selection persists across page reloads, sidebar shows all guide and explanation slugs (as 404 stubs is acceptable at this stage)

---

## Phase 3: User Story 1 — Developer Answers a "How Do I…" Question (P1) 🎯 MVP

**Goal**: All guide pages are present, readable, and show idiomatic code in all three languages with persistent language tab selection.

**Independent Test**: Run `npm run dev` in `docs/`, navigate to any guide page, read the answer, click a language tab, navigate to another guide page — the selected language tab is pre-selected. Every guide page answers exactly one task-oriented question.

- [ ] T010 [US1] Create `docs/guides/index.md` — H1 "Guides", introductory sentence ("Each guide answers one how-to question in Rust, Python, and TypeScript"), then a bulleted list linking to all 9 guide pages with the full question as link text
- [ ] T011 [P] [US1] Create `docs/guides/create-atomic-pattern.md` — H1 "How do I create an atomic pattern?"; prose explaining an atomic pattern (no elements, pure value); `::: code-group` block with `[Rust]` (`Pattern::point(value)`), `[Python]` (`Pattern.point(value)`), `[TypeScript]` (`Pattern.point(value)`) idiomatic examples; link to `/explanations/atomic-vs-elements-pattern`
- [ ] T012 [P] [US1] Create `docs/guides/create-pattern-with-elements.md` — H1 "How do I create a pattern with elements?"; prose explaining elements as sub-patterns; `::: code-group` block with `[Rust]` (`Pattern::pattern(value, vec![...])`), `[Python]` (`Pattern.pattern(value, [...])`), `[TypeScript]` idiomatic examples; link to `/explanations/what-is-decorated-sequence`
- [ ] T013 [P] [US1] Create `docs/guides/give-pattern-a-value.md` — H1 "How do I give a pattern a value?"; prose explaining the value field and `Subject` as a common value type; `::: code-group` block showing construction with a `Subject` value in all three languages using `Subject` with identity and labels
- [ ] T014 [P] [US1] Create `docs/guides/parse-gram-notation.md` — H1 "How do I parse Gram notation into a pattern?"; prose explaining what parsing does; `::: code-group` with `[Rust]` (`parse_gram(input)?`), `[Python]` (`gram.parse(input)`), `[TypeScript]` (`gram.parse(input)`) examples showing a simple Gram string being parsed into a pattern or list of patterns; link to `/explanations/what-is-gram-notation`
- [ ] T015 [P] [US1] Create `docs/guides/serialize-gram-notation.md` — H1 "How do I serialize a pattern to Gram notation?"; prose on serialisation; `::: code-group` with `[Rust]` (`gram_stringify(&patterns)?`), `[Python]` (`gram.stringify(patterns)`), `[TypeScript]` (`gram.stringify(patterns)`) examples; link to `/explanations/gram-notation-and-pattern`
- [ ] T016 [P] [US1] Create `docs/guides/traverse-pattern-elements.md` — H1 "How do I traverse the elements of a pattern?"; prose on iterating elements; `::: code-group` showing `.elements` field access / iteration in all three languages, with a short example that prints each element's value
- [ ] T017 [P] [US1] Create `docs/guides/map-pattern-values.md` — H1 "How do I map over a pattern's values?"; prose on `map` preserving structure while transforming values; `::: code-group` showing `pattern.map(|v| ...)` (Rust), `pattern.map(fn)` (Python), `pattern.map(fn)` (TypeScript); link to `/explanations/what-is-v-in-pattern`
- [ ] T018 [P] [US1] Create `docs/guides/build-graph-from-patterns.md` — H1 "How do I build a graph from patterns?"; prose on `StandardGraph` and `from_gram`; `::: code-group` showing `StandardGraph::from_gram(input)?` (Rust), `StandardGraph.from_gram(input)` (Python), and TypeScript equivalent; link to `/explanations/when-to-use-pattern`
- [ ] T019 [P] [US1] Create `docs/guides/merge-two-patterns.md` — H1 "How do I merge two patterns?"; prose on `combine` and the `Combinable` trait; `::: code-group` showing `a.combine(b, |va, vb| ...)` in Rust, Python, TypeScript with a concrete value-merge function example

**Checkpoint**: All 9 guide pages render in dev server; each page reads in under 2 minutes; language tab selection persists across navigation

---

## Phase 4: User Story 2 — Developer Understands a Concept (P2)

**Goal**: All explanation pages are present and answer conceptual questions without teaching tasks. Code appears only to illustrate concepts.

**Independent Test**: Navigate to any explanation page; verify the heading begins with "What is…", "Why is…", "How does…", or "When should…"; verify no step-by-step instructions appear; verify each page is self-contained and requires no other page to be read first.

- [ ] T020 [US2] Create `docs/explanations/index.md` — H1 "Explanations", introductory sentence ("Each explanation answers a conceptual question about Pattern and its design"), then a bulleted list linking to all 10 explanation pages with the full question as link text
- [ ] T021 [P] [US2] Create `docs/explanations/what-is-pattern.md` — H1 "What is a Pattern?"; prose explaining the decorated-sequence model: a value paired with an ordered list of elements, each itself a `Pattern<V>`; short illustrative code block (not a task) showing the `Pattern<Subject>` type signature; note that atomic patterns have no elements; avoid tree/graph framing
- [ ] T022 [P] [US2] Create `docs/explanations/what-is-decorated-sequence.md` — H1 "What is a 'decorated sequence'?"; prose explaining elements as the sequence, value as the decoration; illustrate with `["decoration" | elem1, elem2]` Gram notation; explain why "decorated sequence" is a more accurate mental model than "node" or "tree node"
- [ ] T023 [P] [US2] Create `docs/explanations/why-pattern-not-tree.md` — H1 "Why is Pattern not a tree?"; prose explaining that while `Pattern<V>` is recursive and looks tree-like, it is more general: a decorated sequence can be used as a graph element, a walk, or a pure value without hierarchical semantics; explain that calling it a tree leads to wrong expectations about traversal, equality, and composition
- [ ] T024 [P] [US2] Create `docs/explanations/what-is-subject.md` — H1 "What is a Subject?"; prose explaining `Subject` as a self-descriptive value: identity (symbol), labels (set of strings), properties (map of string → value); explain that `Pattern<Subject>` is the standard type for property-graph data; brief illustrative code block showing a `Subject` with identity and labels
- [ ] T025 [P] [US2] Create `docs/explanations/what-is-gram-notation.md` — H1 "What is Gram notation?"; prose explaining Gram as a human-readable serialisation format for patterns; show the general `[value | elem, elem]` form; show the parenthesis/arrow syntactic sugar for graph shapes `(node)`, `(a)-[:rel]->(b)`; explain bidirectionality (parse ↔ stringify)
- [ ] T026 [P] [US2] Create `docs/explanations/gram-notation-and-pattern.md` — H1 "How does Gram notation relate to Pattern?"; prose explaining that Gram is a serialisation of `Pattern<Subject>`, not a separate data model; any valid Gram string round-trips through parse + stringify; illustrate with a short Gram snippet and its pattern equivalent
- [ ] T027 [P] [US2] Create `docs/explanations/atomic-vs-elements-pattern.md` — H1 "What is the difference between an atomic pattern and a pattern with elements?"; prose: atomic = value only, no elements (like a leaf); pattern with elements = value + ordered list of sub-patterns; explain `is_atomic` predicate; no task instructions — just concept grounding
- [ ] T028 [P] [US2] Create `docs/explanations/three-language-bindings.md` — H1 "How do the three language bindings relate to each other?"; prose explaining: Rust is the native implementation; Python bindings use PyO3 exposing the same API surface via `.pyi` stubs; TypeScript uses WASM via wasm-bindgen; all three expose equivalent operations but with language-idiomatic naming and types; link to `/reference/` for API details
- [ ] T029 [P] [US2] Create `docs/explanations/when-to-use-pattern.md` — H1 "When should I use Pattern versus a plain graph library?"; prose: use `Pattern<V>` when the data is inherently a decorated sequence (graph elements, structured notation, values with ordered sub-values); use a plain graph library when the primary operations are traversal and query over a large connected graph; `StandardGraph` bridges both — it stores `Pattern<Subject>` elements in a queryable structure
- [ ] T030 [P] [US2] Create `docs/explanations/what-is-v-in-pattern.md` — H1 "What does the `V` in `Pattern<V>` mean?"; prose: `V` is the value type parameter — any type can decorate a pattern; common choices are `Subject` (property graph), `String` (text tagging), or custom application types; the `map` operation transforms `Pattern<V>` into `Pattern<U>` by applying a function to every value; brief illustrative snippet showing `pattern.map(...)` changing the type

**Checkpoint**: All 10 explanation pages render in dev server; no page contains step-by-step task instructions; each page is self-contained

---

## Phase 5: User Story 3 — Developer Looks Up an API Symbol (P3)

**Goal**: Three language API reference sub-sites are generated by the build script and served at `/reference/rust/`, `/reference/python/`, `/reference/ts/`.

**Independent Test**: Run `./docs/scripts/build.sh` (or partial steps 1–3). Navigate to `docs/public/reference/rust/`, `docs/public/reference/python/`, and `docs/public/reference/ts/` — each contains HTML files with the API documentation for that language. The `/reference/` link hub links to all three.

- [ ] T031 [US3] Create `typedoc.json` at repo root: `entryPointStrategy: "packages"`, `entryPoints: ["typescript/packages/pattern", "typescript/packages/gram", "typescript/packages/graph"]`, `packageOptions: { "entryPoints": ["src/index.ts"], "tsconfig": "tsconfig.json" }`, `out: "docs/public/reference/ts"`, `githubPages: false`, `name: "pattern-rs TypeScript API"`; install TypeDoc: add `"typedoc": "^0.26.0"` to `docs/package.json` devDependencies and run `npm install` in `docs/`
- [ ] T032 [US3] Replace Step 1 stub in `docs/scripts/build.sh` with the cargo doc command: `cargo doc --workspace --no-deps 2>&1` followed by `rm -rf docs/public/reference/rust && mkdir -p docs/public/reference/rust && cp -r target/doc/. docs/public/reference/rust/`; verify it generates output by running this step alone and checking `docs/public/reference/rust/relateby_pattern/index.html` exists
- [ ] T033 [US3] Replace Step 2 stub in `docs/scripts/build.sh` with the pdoc command: activate the Python virtualenv at `python/packages/relateby/.venv`, then run `pdoc relateby --output-dir "$(pwd)/docs/public/reference/python"` from `python/packages/relateby/`; add a check at the top of this step that the venv exists and prints an actionable error if not; verify `docs/public/reference/python/relateby/index.html` is generated
- [ ] T034 [US3] Replace Step 3 stub in `docs/scripts/build.sh` with the TypeDoc command: `npx typedoc --options typedoc.json` (run from repo root); verify `docs/public/reference/ts/index.html` is generated
- [ ] T035 [P] [US3] Update `docs/reference/index.md` to include specific navigation: add a brief description of what each reference covers (Rust crates `relateby-pattern` and `relateby-gram`; Python subpackages `relateby.pattern` and `relateby.gram`; TypeScript packages `@relateby/pattern`, `@relateby/gram`, `@relateby/graph`) and link each to its sub-path

**Checkpoint**: Steps 1–3 of `docs/scripts/build.sh` run without error; all three `docs/public/reference/*/` directories contain valid HTML; `/reference/` page clearly links to all three

---

## Phase 6: User Story 4 — AI Tool Consumes the Documentation (P4)

**Goal**: `/llms.txt` and `/llms-full.txt` are generated at build time from the markdown source and pass the format contract.

**Independent Test**: Run Steps 4–5 of `docs/scripts/build.sh`. Inspect `docs/public/llms.txt` — verify it has a `# pattern-rs` header, `## Explanations` section, `## Guides` section, and `## API Reference` section matching the format in `contracts/llms-format.md`. Inspect `docs/public/llms-full.txt` — verify explanations appear before guides, each page is separated by `\n---\n`, and YAML frontmatter is absent.

- [ ] T036 [US4] Replace Step 4 stub in `docs/scripts/build.sh` with `llms.txt` generation: write a shell function that (a) writes the header block (`# pattern-rs\n\n> [description]\n\n`), (b) iterates `docs/explanations/*.md` in slug-alphabetical order to emit `- [title](/explanations/slug)` lines under `## Explanations`, (c) iterates `docs/guides/*.md` in slug-alphabetical order under `## Guides`, (d) writes the `## API Reference` block with fixed links; extract the `title` from each file's YAML frontmatter using `grep` and `sed`
- [ ] T037 [US4] Replace Step 5 stub in `docs/scripts/build.sh` with `llms-full.txt` generation: write a shell function that (a) processes `docs/explanations/*.md` then `docs/guides/*.md` in slug-alphabetical order, (b) for each file: strips the YAML frontmatter block (`---` to `---`) using `sed`, (c) appends the stripped content followed by `\n---\n` separator; output to `docs/public/llms-full.txt`
- [ ] T038 [US4] Replace Step 6 stub in `docs/scripts/build.sh` with `npx vitepress build docs` (run from repo root); add `mkdir -p docs/public` before Step 4 to ensure the output directory exists before LLM file generation

**Checkpoint**: `./docs/scripts/build.sh` runs end-to-end without error; `docs/.vitepress/dist/` contains the built site; `docs/public/llms.txt` and `docs/public/llms-full.txt` match the format in `contracts/llms-format.md`

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: CI/CD integration, final configuration tuning, and full build smoke test

- [ ] T039 Create `.github/workflows/docs.yml` — trigger: `push` to `main`; permissions: `pages: write`, `id-token: write`; jobs: `build` (checkout → setup Rust stable → setup Node 20 → setup Python 3.13 → activate venv and install pdoc → run `./docs/scripts/build.sh`) and `deploy` (needs: build; uses: `actions/deploy-pages@v4` with artifact uploaded from `docs/.vitepress/dist/`); environment: `name: github-pages`
- [ ] T040 [P] Run the full build locally: `./docs/scripts/build.sh` from repo root; verify (a) all six steps complete without error, (b) `docs/.vitepress/dist/` contains a full site, (c) `npx vitepress preview docs` serves the site and all `/reference/`, `/guides/`, `/explanations/` paths resolve, (d) language tab selection persists after a hard reload
- [ ] T041 [P] Verify sidebar configuration completeness in `docs/.vitepress/config.ts`: every guide page slug in Phase 3 appears in the `/guides/` sidebar; every explanation page slug in Phase 4 appears in the `/explanations/` sidebar; sidebar labels match the H1 question text of each page (abbreviated to fit sidebar); no 404s in sidebar navigation
- [ ] T042 [P] Run code quality checks: `cargo fmt --all -- --check` (or `cargo fmt --all`), `cargo clippy --workspace`, `cargo test --workspace`, `./scripts/ci-local.sh`; fix any formatting or lint warnings introduced by the new `typedoc.json` or workflow file; confirm all tests pass
- [ ] T043 Verify `llms.txt` coverage: count the lines in `## Explanations` and `## Guides` sections of `docs/public/llms.txt` — confirm 10 explanation entries and 9 guide entries; verify each entry's question text matches the H1 of the corresponding page; verify all three API reference links are present under `## API Reference`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies — start immediately
- **Foundational (Phase 2)**: Depends on Phase 1 (npm install) — blocks all user story work
- **US1 Guides (Phase 3)**: Depends on Phase 2 (VitePress core) — guides need config and theme
- **US2 Explanations (Phase 4)**: Depends on Phase 2 — independent of Phase 3
- **US3 API Reference (Phase 5)**: Depends on Phase 1 (package.json for TypeDoc) — independent of Phases 3 and 4; requires Rust/Python/Node toolchains
- **US4 LLM Files (Phase 6)**: Depends on Phases 3, 4 (needs all markdown source pages to exist), and Phase 5 (build.sh steps 1–3 must be in place before step 4–6 are appended)
- **Polish (Phase 7)**: Depends on all user story phases complete

### User Story Dependencies

- **US1 (P1)**: After Phase 2 — no dependency on US2, US3, US4
- **US2 (P2)**: After Phase 2 — no dependency on US1, US3, US4
- **US3 (P3)**: After Phase 1 — no dependency on US1 or US2 for tooling; build.sh integration comes after US1/US2 so content exists for the final vitepress build
- **US4 (P4)**: After US1 and US2 (needs guide and explanation pages to concatenate)

### Within Each Phase

- All [P]-marked tasks within a phase can run in parallel (they write different files)
- Content pages (T011–T019, T021–T030) are all [P] — can be written in any order or all at once
- Build script steps (T032–T038) must be added sequentially (editing the same file)

---

## Parallel Execution Examples

### Parallel: Phase 2 Foundational

```
T006 useLanguageTab.ts  ─────────────────────────────────┐
T007 theme/index.ts     ─────────────────────────────────┤ (T007 needs T006)
T008 docs/index.md      ────────────────────────────────┐│ (T008 is independent)
T009 reference/index.md ───────────────────────────────┐││ (T009 is independent)
```

Run T005, T008, T009 in parallel. T006 then T007 sequentially (same module).

### Parallel: Phase 3 — All Guide Pages

```
T011 create-atomic-pattern.md
T012 create-pattern-with-elements.md
T013 give-pattern-a-value.md       ← All 9 can be written in parallel
T014 parse-gram-notation.md
T015 serialize-gram-notation.md
T016 traverse-pattern-elements.md
T017 map-pattern-values.md
T018 build-graph-from-patterns.md
T019 merge-two-patterns.md
```

### Parallel: Phase 4 — All Explanation Pages

```
T021–T030 ← All 10 can be written in parallel (different files)
```

### Parallel: Phase 5 — API Reference Build Steps

```
T031 typedoc.json  ─────────────────────── (independent of T032-T034)
T032 cargo doc step in build.sh  ─────────┐
T033 pdoc step in build.sh        ────────┤ (sequential — same file)
T034 TypeDoc step in build.sh     ────────┘
T035 docs/reference/index.md  ─────────── (independent)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete **Phase 1**: Setup — VitePress project and package.json
2. Complete **Phase 2**: Foundational — site loads, language tabs persist
3. Complete **Phase 3**: US1 — all 9 guide pages written and rendering
4. **STOP and VALIDATE**: Run `npm run dev`, read each guide, test tab persistence
5. This is a shippable MVP — guides alone deliver significant developer value

### Incremental Delivery

1. Setup + Foundational → site skeleton
2. Add US1 (Guides) → test independently → shippable MVP
3. Add US2 (Explanations) → test independently → conceptual layer complete
4. Add US3 (API Reference) → test independently → full reference coverage
5. Add US4 (LLM Files) → test independently → AI discoverability complete
6. Polish → GitHub Pages deployed → publicly accessible

### Parallel Team Strategy

With multiple contributors:

- **Developer A**: Phase 2 (Foundational: VitePress config + theme composable)
- **Developer B** (after Phase 1): US3 (typedoc.json + build.sh steps 1–3, toolchain setup)
- Once Phase 2 is done:
  - **Developer A**: US1 (guide pages)
  - **Developer C**: US2 (explanation pages)
  - **Developer B**: US4 (LLM generation in build.sh)

---

## Notes

- [P] tasks write different files — safe to run in parallel with no merge conflicts
- [Story] labels trace each task back to the user story and its acceptance scenarios in spec.md
- Language tab labels must be exactly `Rust`, `Python`, `TypeScript` (capitalisation matters for `useLanguageTab.ts`)
- All code examples in guide and explanation pages must be idiomatic for the target language — derive from existing stubs in `python/packages/relateby/relateby/**/__init__.pyi` and `typescript/packages/*/src/index.ts` for type accuracy
- Run `./docs/scripts/build.sh` after Phase 6 before marking Phase 7 complete
- `docs/public/reference/` is gitignored — do not commit generated API docs
- Commit after each phase or logical group; the branch is `049-doc-website`
