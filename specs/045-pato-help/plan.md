# Implementation Plan: pato help and self-documentation

**Branch**: `045-pato-help` | **Date**: 2026-03-22 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/045-pato-help/spec.md`

## Summary

Add `pato help <topic>` as a new built-in subcommand. Topic markdown files are embedded into the binary at compile time via `include_str!` and serve as the single source of truth. `pato skill` installs these same files to `.agents/skills/pato/reference/` as a derived artifact. Initial topic set: `gram-notation` and `stdout-stderr-contracts`.

## Technical Context

**Language/Version**: Rust 1.70.0 (workspace MSRV), Edition 2021
**Primary Dependencies**: `clap` v4 with derive (existing), no new dependencies
**Storage**: Topic content embedded in binary via `include_str!` (compile-time static)
**Testing**: `cargo test -p relateby-pato`
**Target Platform**: Native CLI binary (Linux, macOS, Windows). Not a WASM target.
**Project Type**: CLI tool
**Performance Goals**: Help output must appear instantly (sub-10ms); topic lookup is a linear scan over a small static slice.
**Constraints**: No new crate dependencies. Must not require `pato skill` to have been run for `pato help` to work.
**Scale/Scope**: 2 initial topics; designed to grow to ~10 topics without structural changes.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Reference Implementation Fidelity | ✅ N/A | `pato` is a new CLI tool, not a port of gram-hs. No equivalent in gram-hs to compare against. |
| II. Correctness & Compatibility | ✅ Pass | Topic name–to–file mapping is compile-time enforced. Embed via `include_str!` fails at compile time if file is missing. |
| III. Rust Native Idioms | ✅ Pass | `include_str!`, static slices, `Option` return for lookup. No non-idiomatic patterns. |
| IV. Multi-Target Library Design | ✅ N/A | `pato` is a CLI binary, not a library compiled for WASM. This feature adds no blocking I/O or filesystem access at the library layer. |
| V. External Language Bindings & Examples | ✅ Pass | `quickstart.md` provides a usage example. No API surface change to the library crate. |

**Post-Phase-1 re-check**: No violations introduced by design. `topic_catalog.rs` is `pub` in `lib.rs`, usable in tests without invoking the binary.

## Project Structure

### Documentation (this feature)

```text
specs/045-pato-help/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── cli-contract.md  # Phase 1 output
└── tasks.md             # Phase 2 output (/speckit.tasks command)
```

### Source Code (repository root)

```text
crates/pato/
├── skill-package/pato/
│   ├── SKILL.md
│   ├── assets/
│   │   └── examples.md
│   ├── references/              # existing: agent context docs (unchanged)
│   │   ├── output-contracts.md
│   │   └── workflows.md
│   └── reference/               # NEW: topic docs for pato help
│       ├── gram-notation.md
│       └── stdout-stderr-contracts.md
└── src/
    ├── lib.rs                   # add: pub mod topic_catalog;
    ├── main.rs                  # add: Commands::Help dispatch
    ├── cli.rs                   # add: HelpArgs, Commands::Help variant
    ├── topic_catalog.rs         # NEW: embedded topic registry
    └── commands/
        ├── mod.rs               # add: pub mod help;
        └── help.rs              # NEW: help command handler

tests/
└── help_tests.rs                # NEW: integration tests for help command
```

**Structure Decision**: Single project layout extending the existing `crates/pato/` crate. No new crates, no new workspace members.

## Implementation Phases

### Phase 1: Topic corpus

1. Create `crates/pato/skill-package/pato/reference/` directory
2. Author `gram-notation.md` — definition, syntax, semantics, 2–4 examples
3. Author `stdout-stderr-contracts.md` — adapted from existing `references/output-contracts.md`
4. Verify each file opens with `# Topic Name` and is under ~150 lines

**Dev bootstrapping note**: `build.rs` prefers embedding from `.agents/skills/pato/` (the workspace-installed location) over `skill-package/pato/` when the former exists. If `.agents/skills/pato/` is present on a developer's machine and does not yet contain `reference/`, the `SKILL_BUNDLE` embedded during that build will not include the new topic files. Before Phase 4 (wiring `SKILL_BUNDLE`), developers must either:
- Delete `.agents/skills/pato/` so `build.rs` falls through to `skill-package/pato/`, or
- Copy the new `reference/` files into `.agents/skills/pato/reference/` manually.
After Phase 5 is complete, running `pato skill --force` will keep the two in sync automatically.

### Phase 2: Embedded catalog

5. Create `crates/pato/src/topic_catalog.rs` with `TopicEntry`, `TOPICS` static, `find_topic()`, `topic_names()`
6. Register `pub mod topic_catalog;` in `lib.rs`
7. Confirm `cargo build -p relateby-pato` compiles (proves embed paths resolve)

### Phase 3: CLI wiring

8. Add `HelpArgs` struct and `Commands::Help(HelpArgs)` variant to `cli.rs`
9. Create `commands/help.rs` with `run()` function
10. Register `pub mod help;` in `commands/mod.rs`
11. Add `Commands::Help(args) => commands::help::run(&args)` dispatch in `main.rs`

### Phase 4: Embedded skill install

`build.rs` already generates `$OUT_DIR/skill_bundle.rs` containing `SKILL_BUNDLE: &[(&str, &[u8])]` via `include_bytes!` for all files under `skill-package/pato/`. This generated file is currently not referenced anywhere. This phase wires it in to replace the runtime filesystem lookup.

12. Add `include!(concat!(env!("OUT_DIR"), "/skill_bundle.rs"));` to `lib.rs` to expose `SKILL_BUNDLE`
13. Rewrite `skill_install/package.rs`: replace `locate_canonical_bundle()` + filesystem copy with a function that iterates `SKILL_BUNDLE` and writes each embedded file to the install target path
14. Remove or guard the `locate_canonical_bundle()` fallback paths that read from the source tree on disk
15. Confirm `pato skill --scope project` works and installs all files including `reference/`

### Phase 5: Tests and polish

14. Write `tests/help_tests.rs`:
    - known topic → stdout has content, exit 0
    - unknown topic → stderr has error + topic list, exit 1
    - no topic → stderr has usage + topic list, exit 1
    - all catalog entries → non-empty content
15. Run `cargo test -p relateby-pato`
16. Run `cargo clippy --workspace -- -D warnings`
17. Run `cargo fmt --all`

## Complexity Tracking

No constitution violations. No complexity justification needed.
