# Research: pato help and self-documentation

**Feature**: 045-pato-help
**Branch**: `045-pato-help`

---

## Decision 1: Topic embedding strategy

**Decision**: Use `include_str!` macros in a `topic_catalog.rs` module to embed topic markdown files directly into the binary at compile time.

**Rationale**: `include_str!` is a stable Rust built-in that requires no new dependencies. It embeds UTF-8 text at compile time and panics at compile time if the file is missing, making the corpus–binary alignment enforced by the compiler. The file path is relative to the source file, so `include_str!("../../skill-package/pato/reference/gram-notation.md")` works directly.

**Alternatives considered**:

1. **`rust-embed` crate** — Provides a derive macro for embedding entire directories with runtime lookup by path. More flexible but adds a dependency and is heavier than needed for a small, known-at-compile-time topic set.

2. **Extend the existing `build.rs` approach** — `build.rs` already generates a `SKILL_BUNDLE: &[(&str, &[u8])]` constant via `include_bytes!`. This IS used for `pato skill` (see Decision 2 below), but `pato help` uses `include_str!` directly for simpler topic lookup rather than going through the bundle array.

3. **Runtime filesystem read** — The current skill install uses runtime path resolution. For `pato help` this is explicitly wrong per FR-007: the binary must be authoritative and not depend on `pato skill` having been run first.

---

## Decision 2: Topic file location (canonical source)

**Decision**: `crates/pato/skill-package/pato/reference/<topic>.md` is the canonical location for topic files in the repository. These are the files embedded by `include_str!` and also installed by `pato skill`.

**Rationale**: The `skill-package/pato/` tree is already the source for `pato skill` installation (via runtime copy in `skill_install/package.rs`). Adding `reference/` as a subdirectory keeps all skill content in one place. The `.agents/skills/pato/` workspace copy is a derived artifact (installed via `pato skill`); it must not be the embed source.

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

**Decision**: Start with two topics that have existing content or immediate need:

- `gram-notation` — notation format, most-requested
- `stdout-stderr-contracts` — already partially documented in `references/output-contracts.md`

Add `gram-annotation` and `skill-installation` once their content is written; these are registered in the catalog as stubs or omitted until authored.

**Rationale**: The spec assumption says "intentionally small at first." Two topics with real content are better than four with placeholder text. Topic names are part of the public contract once shipped, so only ship topics with complete content.

---

## Key finding: build.rs must be wired in for pato skill to work

The `build.rs` generates `OUT_DIR/skill_bundle.rs` with `include_bytes!` for all files under `skill-package/pato/`. This output is not currently `include!`'d anywhere. After `cargo install relateby-pato`, the `skill-package/pato/` source tree is NOT present on the user's filesystem — only the binary. This means the current runtime filesystem resolution in `skill_install/package.rs` is broken for `cargo install` users. This feature wires in the generated bundle so that `pato skill` extracts embedded content from the binary instead of reading from disk.
