# Top-Level Markdown Files — Review

Review of repo-root `*.md` files: which are temporary/WIP vs long-lived docs.

**Status**: Moves and deletions completed. The three long-lived docs are now in `docs/`; the 10 temp/WIP files have been removed.

---

## Keep at top level

| File | Reason |
|------|--------|
| **README.md** | Standard project entry point. |
| **TODO.md** | Standard place for project TODOs; referenced by README and specs. |

---

## Long-lived → move under `docs/`

| File | Content | Recommendation |
|------|---------|----------------|
| **PORTING_GUIDE.md** | Systematic guide for porting gram-hs → gram-rs (~550 lines). Referenced by README, TODO, `docs/gramref-*.md`, and many specs. | Move to **docs/porting-guide.md** and update all references (README, TODO, docs, specs, plan templates). |
| **SUBJECT-COMBINATION-STRATEGIES.md** | Technical reference: Subject combination strategies (Merge, FirstSubject, etc.) and semigroup laws. | Move to **docs/subject-combination-strategies.md**. Add to README “Documentation” if you want it discoverable. |
| **WASM_BUILD_NOTES.md** | Known issue: gram-codec WASM build fails due to tree-sitter C deps; problem summary, repro, workarounds. | Move to **docs/wasm-build-notes.md** (or `docs/known-issues.md` if you prefer one “issues” doc). |

---

## Temporary / WIP / completion notes (candidates to remove or archive)

These read as one-off completion summaries or session notes, not ongoing docs.

| File | Content | Recommendation |
|------|---------|----------------|
| **CANONICAL-ALIGNMENT-SUMMARY.md** | “Canonical JSON alignment complete” (Jan 2026). | Remove or move to `docs/archive/` or a single `docs/implementation-notes.md` if you want to keep history. |
| **DEAD-CODE-ANALYSIS.md** | Clippy dead-code analysis (Jan 2026). | Same as above — historical analysis. |
| **EXAMPLES-UPDATED.md** | “Examples updated: AST usage” (Jan 2026). | Same — completion note. |
| **FEATURE-BRANCH-COMPLETE.md** | Phase 7 + canonical alignment + interop summary (Jan 2026). | Same — completion note. |
| **PHASE4-COMPLETE.md** | “Phase 4 complete: pattern operations.” | Same — phase completion note. |
| **PHASE7-SUMMARY.md** | “Phase 7 complete: AST output.” | Same — phase completion note. |
| **PROPERTY-RECORDS-WORKING.md** | “Property records: all working” (Jan 2026). | Same — completion note. |
| **PYTHON-BINDINGS-SUMMARY.md** | Session summary for 024-python-pattern-core (Phases 1–5 done, 6 in progress). | Either remove when 024 is done or fold into spec/024 docs. |
| **RELATIONSHIP-FUNCTIONS-COMPLETE.md** | zip3 / zip_with implementation complete (Jan 2026). | Same — completion note. |
| **ZIP-RELATIONSHIPS-ADDED.md** | “Relationship creation functions added” (Jan 2026). | Same — overlaps with above; completion note. |

---

## Summary

- **Keep:** `README.md`, `TODO.md`.
- **Move to docs/:** `PORTING_GUIDE.md` → `docs/porting-guide.md`, `SUBJECT-COMBINATION-STRATEGIES.md` → `docs/subject-combination-strategies.md`, `WASM_BUILD_NOTES.md` → `docs/wasm-build-notes.md` (and update links, especially for PORTING_GUIDE).
- **Remove or archive:** The 10 completion/session/phase summary files listed above; optionally consolidate into one archived “implementation notes” doc.
