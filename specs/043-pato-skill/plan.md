# Implementation Plan: `pato skill`

**Branch**: `043-pato-skill` | **Date**: 2026-03-20 | **Spec**: [`/Users/akollegger/Developer/gram-data/pattern-rs/specs/043-pato-skill/spec.md`](/Users/akollegger/Developer/gram-data/pattern-rs/specs/043-pato-skill/spec.md)
**Input**: Feature specification from `/specs/043-pato-skill/spec.md`

## Summary

Add a built-in `pato skill` subcommand that installs the bundled `pato` skill from a
single canonical repository source at `.agents/skills/pato/`. The command will support
project and user scope installs, enforce Vercel-discoverable project installs by using
`.agents/skills/`, allow user-scope client-native installs, protect existing installs
unless replacement is explicitly requested, and report the resolved install path. The
implementation should keep one authoritative skill tree in the repository and make the
crate consume that tree for bundling and installation rather than maintaining a second
editable copy.

## Technical Context

**Language/Version**: Rust 1.70.0+ (workspace MSRV), Edition 2021  
**Primary Dependencies**: `clap`, `std::fs`, `std::path`, existing `relateby-pato` crate modules; one asset-embedding helper may be added if needed to bundle the canonical skill tree from `.agents/skills/pato/` without duplicating source files  
**Storage**: Local filesystem only  
**Testing**: `cargo test -p relateby-pato`, integration tests under `crates/pato/tests/`, plus packaging-oriented verification for bundled skill assets  
**Target Platform**: Native Rust CLI on macOS/Linux/Windows, with behavior that remains valid when the crate is packaged or installed outside the repository root  
**Project Type**: CLI workspace crate  
**Performance Goals**: Skill install path resolution and copy should complete near-instantly for a single small skill package; no streaming or long-running processing required  
**Constraints**: Project-scope installs must land in a Vercel-discoverable path; `.agents/skills/pato/` is the single checked-in source of truth; no network access or registry workflow in v1; avoid a second authoritative in-repo copy  
**Scale/Scope**: One bundled skill package, four supported install combinations (project/user x interoperable/client-native, with client-native limited by spec to user scope), and targeted CLI/integration tests for install behavior

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **Reference Implementation Fidelity**: Pass with documented exception. This feature is
  `pato`-specific CLI functionality and does not correspond to an existing `gram-hs`
  feature to port. No reference implementation parity requirement applies beyond
  documenting the absence of an upstream equivalent.
- **Correctness & Compatibility**: Pass. The design preserves spec-defined install
  behavior, path selection, and overwrite safety as the primary goals.
- **Rust Native Idioms**: Pass. The expected implementation uses `clap`, `PathBuf`,
  `std::fs`, explicit result handling, and focused modules consistent with current
  crate structure.
- **Multi-Target Library Design**: Pass. The feature is isolated to the native CLI
  crate and does not alter core workspace APIs or WASM-facing behavior.
- **External Language Bindings & Examples**: Pass. No Python, TypeScript, or WASM
  binding changes are expected; if user-facing CLI docs/examples change, keep them in
  sync with the final command behavior.
- **Code Quality Workflow**: Must be satisfied before completion by running formatting,
  linting, targeted tests, and relevant local CI checks.

## Project Structure

### Documentation (this feature)

```text
specs/043-pato-skill/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── skill-install.openapi.yaml
└── tasks.md
```

### Source Code (repository root)

```text
.agents/
└── skills/
    └── pato/
        ├── SKILL.md
        ├── references/
        └── assets/

crates/pato/
├── Cargo.toml
├── src/
│   ├── cli.rs
│   ├── lib.rs
│   ├── main.rs
│   ├── commands/
│   │   ├── mod.rs
│   │   └── skill.rs
│   └── skill_install/
│       ├── mod.rs
│       ├── package.rs
│       └── target.rs
└── tests/
    ├── skill_tests.rs
    └── fixtures/
```

**Structure Decision**: Extend the existing `crates/pato` CLI command surface with a
dedicated `commands/skill.rs` handler and a small supporting install module that owns
package enumeration, destination resolution, overwrite checks, and copy/bundle logic.
Keep the skill source itself outside the crate at `.agents/skills/pato/` because the
spec makes that path canonical and Vercel-discoverable at project scope.

## Phase 0: Research Focus

1. Confirm open Agent Skills and Vercel discovery conventions relevant to project and
   user install paths.
2. Confirm the safest Rust strategy for shipping the canonical repository skill package
   without introducing a second maintained source-of-truth.
3. Define packaging and test implications so `cargo package`/published artifact
   behavior does not silently drop bundled skill assets.

## Phase 1: Design Focus

1. Model the canonical skill package, install request, install target, and installed
   skill lifecycle.
2. Specify the contract for resolving install targets and executing installation.
3. Define a quickstart workflow for local validation of the command and canonical skill
   layout.
4. Update Cursor agent context after generating design artifacts.

## Post-Design Constitution Check

- **Reference Implementation Fidelity**: Still passes; no upstream porting obligations
  were introduced by the design.
- **Correctness & Compatibility**: Design keeps one canonical source, explicit install
  rules, and packaging verification, which reduces drift and preserves spec behavior.
- **Rust Native Idioms**: Design remains module-oriented and uses explicit filesystem
  operations rather than hidden side effects.
- **Multi-Target Library Design**: No new non-native behavior introduced.
- **External Language Bindings & Examples**: No binding impact; quickstart covers CLI
  validation only.

## Complexity Tracking

No constitution violations currently require justification.
