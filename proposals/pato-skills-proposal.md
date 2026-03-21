# Feature Proposal: `pato` Skills and `pato skill`

**Status:** Draft — for review and iteration  
**Scope:** Publishable `pato` skill package + local skill instantiation via CLI  
**Crate name:** `relateby-pato` (published), binary name: `pato`  
**Location:** canonical skill bundled with `crates/pato/`; proposal captured in `proposals/`

---

## 1. Purpose

`pato` already serves developers and coding agents as a CLI for working with gram
files. This proposal extends that role by packaging `pato`'s workflow knowledge as
an installable agent skill and by adding a built-in `pato skill` subcommand that can
materialize that skill into well-known local locations.

The goal is twofold:

- Make `pato` easier for agents to use correctly without requiring ad hoc prompt
  engineering in every project.
- Publish one portable skill package that can be shared through an open skills
  ecosystem and instantiated locally for specific agent products.

This proposal intentionally separates the **publishable skill artifact** from the
**installation target**. The published artifact should be neutral and reusable; local
installation is an adapter concern.

---

## 2. Design Principles

**One canonical skill package.** `pato` should define a single source-of-truth skill
package in the open Agent Skills directory format. Product-specific installations
should be generated from that package rather than maintained as divergent copies.

**Open-format first.** The canonical package should target the Agent Skills format
documented at [agentskills.io](https://agentskills.io/home), using a directory
containing `SKILL.md` plus optional `references/`, `assets/`, and `scripts/`.

**Cross-client interoperability over product lock-in.** The canonical install target
is `.agents/skills/`, which `agentskills.io` recommends as the interoperable
convention. Client-native locations such as `.cursor/skills/` remain important, but
they are adapters rather than the primary package identity.

**Progressive disclosure.** The main `SKILL.md` should stay concise and load only the
instructions an agent needs on each activation. Detailed material should move to
referenced support files.

**Real workflows, not generic advice.** The skill should teach concrete `pato`
workflows: when to run `pato lint`, when to prefer `pato check`, how to interpret
`pato rule`, how to re-run validation after edits, and how to preserve machine-clean
stdout/stderr contracts.

**Zero-config local installation.** `pato skill` should work without a manifest,
registry login, or external service. It should install the bundled skill into a
well-known location and stop.

---

## 3. Background: What Agent Skills Recommends

The guidance at `agentskills.io` strongly supports this design:

- A publishable skill is a directory with a `SKILL.md` entry point.
- `SKILL.md` must include YAML frontmatter with `name` and `description`.
- Skills should be grounded in real expertise and refined through actual execution.
- `SKILL.md` should stay under roughly 500 lines / 5000 tokens.
- Supporting detail should be moved into `references/`, `assets/`, or `scripts/`.
- Skills-compatible tools should scan both client-native directories and
  `.agents/skills/` for interoperability.
- Project-level skills should override user-level skills when names collide.

This means the best practice for a publishable `pato` skill is:

1. Author it as a neutral Agent Skills package.
2. Validate it as a package, not as a Cursor-only convention.
3. Install it locally into whichever well-known location a client expects.
4. Keep the package content product-agnostic unless a client truly requires an
   adapter-specific variation.

---

## 4. Proposed Skill Package

The canonical bundled package should look like:

```text
pato/
├── SKILL.md
├── references/
│   ├── workflows.md
│   └── output-contracts.md
└── assets/
    └── examples.md
```

Recommended contents:

- `SKILL.md`
  - what `pato` does
  - when to activate the skill
  - the default decision rules for `lint`, `check`, `fmt`, `parse`, and `rule`
  - a short validation loop: run command, inspect output, apply change, re-run
- `references/workflows.md`
  - common agent workflows such as "inspect gram file", "fix diagnostics", and
    "understand an unfamiliar P-code"
- `references/output-contracts.md`
  - stdout/stderr expectations and why diagnostic/output streams should stay clean
- `assets/examples.md`
  - short example invocations and input/output patterns

The skill should not require bundled scripts in v1 unless there is a repeated,
testable workflow that is genuinely better captured as an executable helper.

---

## 5. Canonical Location In-Repo

The canonical skill package lives at the repository root:

```text
.agents/skills/pato/
```

This makes it Vercel-discoverable and keeps it alongside other agent skills in the
repository. The `pato` crate mirrors this tree under `crates/pato/skill-package/pato/`
for packaging (so published crates are self-contained), but `.agents/skills/pato/` is
the single authoritative source of truth that `build.rs` reads at build time.

The package should still use the published skill name `pato`, and its `SKILL.md`
frontmatter should match the directory name exactly.

---

## 6. `pato skill` Command

Add a new built-in subcommand:

```text
pato skill
```

Its job in v1 is simple: install the bundled canonical skill locally.

### 6.1 Proposed interface

```text
pato skill [--scope project|user] [--target agents|cursor] [--force] [--print-path]
```

Defaults:

- `--scope project`
- `--target agents`

### 6.2 Resolved locations

| Scope | Target | Destination |
|-------|--------|-------------|
| project | `agents` | `.agents/skills/pato/` |
| user | `agents` | `~/.agents/skills/pato/` |
| user | `cursor` | `~/.cursor/skills/pato/` |

Project-scope installs are only supported for the `agents` target to preserve Vercel
discoverability. Invoking `pato skill --scope project --target cursor` is rejected
with an error.

### 6.3 Behavior

- Copy the bundled canonical package recursively to the resolved destination.
- Refuse to overwrite an existing skill unless `--force` is passed.
- Print the installed path on success.
- Keep the installed contents semantically identical to the canonical package.

The command should remain local-only in v1. It does not fetch from a registry, upload
to one, or mutate project configuration outside the chosen skill directory.

---

## 7. Why `.agents/skills/` Is The Canonical Default

The open Agent Skills guidance recommends scanning both client-native locations and
`.agents/skills/`. For a publishable skill, `.agents/skills/` is the best default
because it is:

- product-neutral
- interoperable across compatible clients
- a better basis for registry submission
- a cleaner source-of-truth than any one editor's private convention

This does not reduce support for Cursor. It simply places Cursor support in the right
layer: installation and discovery, not canonical authoring.

---

## 8. CLI Integration

This proposal fits naturally into the existing built-in command model.

Update:

- `crates/pato/src/cli.rs` to add `Skill(SkillArgs)`
- `crates/pato/src/main.rs` to dispatch `Commands::Skill`
- `crates/pato/src/commands/skill.rs` for the command implementation

Keep `crates/pato/src/extensions.rs` unchanged. `pato skill` is a built-in command,
not a `pato-skill` PATH extension.

---

## 9. Publishing Best Practices For The `pato` Skill

Based on `agentskills.io`, the best practice for developing a publishable skill is:

**Start from real work.** Derive the skill from actual `pato` usage patterns,
corrections, diagnostics, and examples rather than writing a generic CLI helper.

**Package one coherent unit.** The skill should cover "use `pato` effectively for gram
workflows" rather than expanding into all graph tooling or all agent behavior.

**Keep the core tight.** Put only the reusable, high-value instructions in `SKILL.md`.
Push explanatory detail into one-level-deep support files.

**Prefer defaults over option menus.** For example:

- use `pato check` when the user asks "is this file correct?"
- use `pato lint` when the task is diagnostic-only
- use `pato rule <code>` when a P-code needs explanation

**Include gotchas.** Non-obvious facts are often the highest-value part of the skill,
for example:

- `pato` uses gram as its default machine format
- stdout and stderr have distinct machine/human roles
- `pato fmt` and `pato lint --fix` should be followed by re-validation

**Refine with execution.** Run the skill against real tasks, inspect agent traces, and
cut vague or low-value instructions.

**Validate the package.** The final package should be checked with the Agent Skills
reference tooling, such as `skills-ref validate`, before submission or publication.

---

## 10. Implementation Sketch

### Phase 1: Bundle the canonical skill

- Add the canonical package under `crates/pato/skills/pato/`
- Write `SKILL.md`
- Add only minimal `references/` or `assets/` needed for progressive disclosure

### Phase 2: Add installer support

- Resolve scope + target into a destination path
- Copy the package recursively
- Handle collisions and `--force`

### Phase 3: Add command wiring

- Extend CLI parsing
- Implement `pato skill`
- Report success and destination cleanly

### Phase 4: Test

- CLI argument parsing
- project/user install paths
- `agents`/`cursor` targets
- collision and overwrite behavior
- basic frontmatter validation for bundled `SKILL.md`

---

## 11. Future Extensions

This proposal deliberately stops short of registry operations, but it prepares for
them cleanly.

Possible later additions:

- `pato skill validate`
- `pato skill package`
- `pato skill publish`
- support for additional well-known client locations
- support for installing from remote registries or packaged artifacts

Those features should build on the same canonical package rather than inventing a new
artifact format.

---

## 12. Open Questions

**Q1 — Cursor adapter policy.** Should installation into `.cursor/skills/` copy the
package byte-for-byte, or should it optionally add Cursor-specific metadata later if
needed?

**Q2 — Validation dependency.** Should `skills-ref` be a documented optional tool for
authors only, or should `pato` eventually expose a built-in validation wrapper?

**Q3 — User feedback.** Should `pato skill` print only the final destination, or also
mention the canonical source package path for debugging?

**Q4 — Name stability.** Is `pato` the final published skill name, or should the skill
name be more specific, such as `pato-cli`? The shorter name is attractive if the skill
is expected to be the canonical agent entry point for the tool.

---

## 13. Summary

The right way to make `pato` publishable as a skill is to treat the skill as a
portable Agent Skills package first and a product-specific installation second. The
canonical artifact should be neutral, concise, and grounded in real `pato` workflows.

`pato skill` should then act as the local installer for that bundled artifact,
materializing it into `.agents/skills/` by default and supporting client-native
locations such as `.cursor/skills/` as explicit targets.
