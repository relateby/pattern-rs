# Research: pato help and self-documentation

**Feature**: 045-pato-help
**Branch**: `045-pato-help`

---

## Decision 1: Topic embedding strategy

**Decision**: Use `include_str!` macros in a `topic_catalog.rs` module to embed topic markdown files directly into the binary at compile time.

**Rationale**: `include_str!` is a stable Rust built-in that requires no new dependencies. It embeds UTF-8 text at compile time and panics at compile time if the file is missing, making the corpus–binary alignment enforced by the compiler. The file paths are generated from the markdown corpus so the build stays in sync without a hand-maintained topic list.

**Alternatives considered**:

1. **`rust-embed` crate** — Provides a derive macro for embedding entire directories with runtime lookup by path. More flexible but adds a dependency and is heavier than needed for a small, known-at-compile-time topic set.

2. **Extend the existing `build.rs` approach** — `build.rs` already generates a `SKILL_BUNDLE: &[(&str, &[u8])]` constant via `include_bytes!`. This IS used for `pato skill` (see Decision 2 below), but `pato help` uses `include_str!` directly for simpler topic lookup rather than going through the bundle array.

3. **Runtime filesystem read** — The current skill install uses runtime path resolution. For `pato help` this is explicitly wrong per FR-007: the binary must be authoritative and not depend on `pato skill` having been run first.

---

## Decision 2: Topic file location (canonical source)

**Decision**: The repository keeps matching copies of the topic corpus in `.agents/skills/pato/reference/` and `crates/pato/skill-package/pato/reference/`. `build.rs` reads from the workspace copy when it exists and falls back to the packaged copy for source distributions, but it does not rewrite either tree.

**Rationale**: The workspace copy is convenient for local development and `pato skill` installs, while the packaged copy is what ships in source distributions. Keeping both trees aligned preserves build reproducibility without making the build script mutate tracked files.

**Note on existing `references/` directory**: `skill-package/pato/references/` (plural) contains `output-contracts.md` and `workflows.md` — general skill reference docs. The new `reference/` (singular) holds the topic docs for `pato help <topic>`. These are distinct: `references/` is for agent context, `reference/` is for CLI topic lookup.

---

## Decision 3: Clap -h vs --help differentiation

**Decision**: Use clap's built-in short/long help differentiation. Short descriptions (on `Commands` enum variants) appear in `-h`. Long descriptions (`long_about` on subcommands) appear in `--help`. No custom `HelpCommand` help text needed — clap handles this natively.

**Rationale**: Clap 4 automatically renders `-h` with just the summary and `--help` with full text when `about` and `long_about` are both set. The `next_line_help` setting further compresses `-h` output. This keeps the implementation minimal and idiomatic.

---

## Decision 4: `pato help` rendering mode (resolves FR-004 / Proposal Q1)

**Decision**: Print raw markdown to stdout for v1. No terminal-formatting library.

**Rationale**: Raw markdown is readable in a terminal and directly usable in prompts (FR-004 goal). It avoids adding a rendering dependency (e.g., `termimad`, `bat`). The spec explicitly allows this ("Raw markdown output is acceptable for v1"). If richer rendering is needed later it can be added without changing the topic format or the public topic name contract.

---

## Decision 5: `pato help` with no topic / unknown topic (resolves FR-005)

**Decision**: `pato help` with no topic arg prints a usage line and the list of available topics, then exits with code 1. An unknown topic prints an error message naming the unknown topic, then the same list, then exits with code 1.

**Rationale**: The spec FR-005 requires "enumerate the valid topic names." Printing them on every failure is cheap and directly actionable. Exit code 1 signals failure to callers and agents.

---

## Decision 6: Initial topic set (resolves Proposal §5.3)

**Decision**: Start with the shipped grammar-oriented topic set and the stdout/stderr contract topic:

- `gram` — notation overview
- `gram-patterns` — pattern shapes
- `gram-values` — value forms
- `gram-records` — record forms
- `gram-annotations` — annotation syntax
- `gram-graph_elements` — graph element notation
- `gram-path_equivalences` — notation equivalences
- `gram-graph_gram` — graph grammar subset
- `stdout-stderr-contracts` — already partially documented in `references/output-contracts.md`

**Rationale**: The spec assumption says "intentionally small at first," but the shipped corpus now includes the core gram reference topics needed for `pato help`. Topic names are part of the public contract once shipped, so only ship complete topics.

---

## Key finding: build.rs must be wired in for pato skill to work

The `build.rs` generates `OUT_DIR/skill_bundle.rs` with `include_bytes!` for all files under `skill-package/pato/`, and `lib.rs` includes it for the install path. `pato skill` therefore extracts embedded content from the binary instead of reading from disk, which keeps `cargo install relateby-pato` self-sufficient.
