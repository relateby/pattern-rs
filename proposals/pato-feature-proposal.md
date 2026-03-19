# Feature Proposal: `pato` — CLI Tool for Pattern Data and Gram Files

**Status:** Draft — for review and iteration  
**Scope:** Initial design; covers v0.1 through extensibility foundations  
**Crate name:** `relateby-pato` (published), binary name: `pato`  
**Location:** `crates/pato/` within the `pattern-rs` workspace

---

## 1. Purpose

`pato` is a command-line tool for working with pattern data and gram files. It provides
the operations a developer or coding agent needs when building systems on top of the
`relateby-pattern` / `relateby-gram` library stack: parsing, linting, validation,
formatting, and structural inspection.

The name comes from the Spanish for "duck" — a gram file walks, quacks, and swims like
a graph. More practically, it continues the animal-naming convention of the ecosystem
while being short enough to type constantly.

`pato` is designed for use by:
- Developers writing gram files by hand
- Coding agents working with pattern data as part of larger pipelines
- CI/CD systems verifying gram file correctness
- External tools that embed pato as a subprocess

`pato` is not opinionated about project structure. There is no "pato project." Projects
use pato; pato does not own them.

---

## 2. Design Principles

**Zero-config by default.** Every subcommand works on files passed as arguments.
No manifest required, no discovery logic, no implicit configuration loading.
External systems that want configuration can wrap pato or supply arguments.

**Every diagnostic is actionable.** When pato reports a problem, it always tells the
consumer what to do about it — not just what is wrong. This is a first-class design
constraint, not a documentation nicety. It matters especially for agentic consumers,
which cannot reason about a bare error message the way a human can. Every diagnostic
must carry a structured `remediation` alongside the human-readable `message`.

Three grades of remediation exist, and each diagnostic is classified into exactly one:

- **Auto** — pato can rewrite the file correctly without any ambiguity. `pato fmt`
  handles the full set of auto-fixable issues. `pato lint --fix` applies auto
  remediations for lint rules that are in this class.
- **Guided** — pato cannot safely apply the fix without confirmation, but it can
  state the fix precisely: the location to change, what to change it to, and why.
  The remediation is structured data so an agent can act on it directly.
- **Ambiguous** — the correct fix depends on intent that pato cannot infer. Pato
  names the decision the consumer must make and presents the available options. It
  never silently picks one.

The distinction between guided and ambiguous must be maintained rigorously. Guided
means pato is confident there is exactly one right answer. Ambiguous means there are
multiple valid answers and pato cannot choose — but it still eliminates bad answers
and frames the choice clearly. An agent receiving an ambiguous diagnostic knows
exactly what question to ask the user.

**Gram is the native output format.** Pato speaks gram. Diagnostic output is gram by
default — a diagnostic report is itself a valid gram file. This is not a curiosity:
it means pato's output can be consumed by pato and by any tool in the pattern-*
ecosystem without translation. JSON and text are alternative rendering modes for
compatibility and terminal display respectively; gram is the authoritative format.

This has a homoiconic quality: diagnostics are patterns describing problems in
patterns. A future `pato apply` command will consume a diagnostic gram file and
execute the remediations directly, closing the agentic feedback loop entirely:
`pato lint my.gram | pato apply -`.

**Gram output is designed for sequential reading.** Diagnostic gram is structured so
that both humans and LLMs can process it left-to-right, top-to-bottom, without
needing to hold prior context or resolve backward references to assemble meaning. Each
diagnostic block is self-contained: location comes first (orienting the reader
spatially), then the finding, then the remediation. See Appendix A for the modeling
rationale.

**Stream roles are explicit.** Pato subcommands fall into two distinct roles with
different I/O contracts, and these must never be conflated:

- **Diagnostic commands** (lint, check, validate) — gram files are input; diagnostic
  gram is output on stdout. Gram content from input files never appears on stdout.
- **Transform commands** (fmt, parse, ingest, apply) — gram flows in, gram or
  structured data flows out on stdout. Parse errors that halt transformation go to
  stderr.

Stdout is always a clean, machine-parseable stream. Stderr is always human-readable
progress and logging that a pipeline can safely discard.

**Style is configurable, not prescribed.** A future `style` setting will capture
preferences like arrow family (`-->` vs `==>` vs `~~>`) and label separator (`:` vs
`::`). `pato fmt` respects style settings rather than enforcing a single canonical
form. Until style settings are implemented, arrow family and separator choice are
preserved as-is.

**Two extension mechanisms, one design.** Pato grows through binary extensions
(`pato-foo` binaries on PATH) and convention extensions (file naming and document
`kind` headers). Both are first-class. Binary extensions add new operations; convention
extensions add new interpretations of gram files. The two compose: a convention defines
a kind, a binary implements the operation over it. See §8 for the full specification.

---

## 3. I/O Contract

### 3.1 Stream assignment

| Stream | Contents |
|--------|----------|
| stdin  | Input gram (when `-` is passed as a file argument) |
| stdout | Data output — diagnostic gram (diagnostic commands) or gram/data (transform commands) |
| stderr | Progress, warnings, log messages — never data |

Stdout is always a clean, parseable stream. Stderr is always noise a pipeline can
discard.

### 3.2 Subcommand I/O summary

| Subcommand    | Reads              | Stdout             | Stderr    | Modifies files?        |
|---------------|--------------------|--------------------|-----------|------------------------|
| `lint`        | gram files / stdin | diagnostic gram    | progress  | no (without `--fix`)   |
| `lint --fix`  | gram files         | diagnostic gram    | progress  | yes (auto fixes only)  |
| `check`       | gram files / stdin | diagnostic gram    | progress  | no                     |
| `validate`    | gram files / stdin | diagnostic gram    | progress  | no                     |
| `fmt`         | gram files         | nothing            | progress  | yes (default)          |
| `fmt -`       | stdin              | formatted gram     | progress  | no                     |
| `fmt --check` | gram files         | nothing            | progress  | no                     |
| `parse`       | gram files / stdin | pattern gram/data  | progress  | no                     |
| `rule`        | (none / code arg)  | rule gram          | progress  | no                     |

### 3.3 Output format flags

All subcommands accept `--output-format`:

| Format  | Description |
|---------|-------------|
| `gram`  | **Default.** Gram notation. Machine-parseable and human-readable. |
| `text`  | Terminal-optimised rendering with ANSI colour. Not parseable; for human eyes at a terminal. |
| `json`  | JSON. A compatibility mode for consumers that cannot handle gram. |

`gram` is the default for all subcommands. `text` is a rendering concern, not a
different data format — it conveys the same information as `gram` but formatted for
display. `json` is an escape hatch for external tools, simple scripts, and legacy
integrations.

Note on comments: the `relateby-gram` parser currently drops comments as whitespace.
Gram diagnostic output in v0.1 is therefore fully machine-readable but does not yet
include inline human-explanatory comments. When comment-aware parsing and a `gramdoc`
convention arrive, the same gram output will become simultaneously the machine format
and the human document — the `text` mode will be needed only for interactive terminal
display.

### 3.4 File editing

When a command modifies files in-place (`lint --fix`, `fmt`):
1. Edits are applied in reverse line order within each file (preventing line number
   drift invalidating later edits in the same pass)
2. Writes are atomic (write to temp file, then rename)
3. Changed files are reported on stderr
4. A file is skipped entirely if any ambiguous edit is in scope for it — the
   ambiguous diagnostic is reported instead

Pato owns its line-edit capability internally, keeping the tool self-contained and
portable. This may be revisited if editing requirements grow complex.

---

## 4. Diagnostic Gram Format

Diagnostic output is a valid gram file with `kind: "diagnostics"` in its document
header. It can be parsed by `relateby-gram`, passed to other pato commands, and
eventually consumed by `pato apply`.

### 4.1 Structure

Each diagnostic is represented as a nested pattern: the outer pattern anchors the
source location; the inner pattern carries the finding; remediation steps are either
inline scalar arrays (for simple ordered steps) or nested `Remediation` child patterns
(when each step is rich enough to need its own structure). This nesting reads
left-to-right and top-to-bottom — location first, finding second, remediation last —
matching the natural reading order for both humans and LLMs.

```gram
{ kind: "diagnostics", pato_version: "0.1.0", file: "my.gram" }

[loc1:Location { line: 7, column: 1 } |
  [d1:Diagnostic {
    severity: "error",
    code: "P002",
    rule: "no-duplicate-identity",
    message: "Identity 'alice' is defined twice: first at 3:1, again here",
    remediations: ["Rename this occurrence to a distinct identity"]
  }]
]
```

When a remediation step is rich enough to need its own structure — edit coordinates,
multiple sub-steps, or an ambiguous decision — it becomes a child `Remediation`
pattern instead of a scalar string:

```gram
[loc2:Location { line: 7, column: 1 } |
  [d2:Diagnostic {
    severity: "error",
    code: "P002",
    rule: "no-duplicate-identity",
    message: "Identity 'alice' is defined twice: first at 3:1, again here"
  } |
    [r1:Remediation {
      grade: "guided",
      summary: "Rename this occurrence",
      replace: "alice", with: "alice_2", line: 7, column: 1
    }]
  ]
]
```

### 4.2 Remediation grades

**`auto`** — pato applies this without confirmation. Edit coordinates are complete.

```gram
[loc3:Location { line: 4, column: 8 } |
  [d3:Diagnostic {
    severity: "warning",
    code: "P004",
    rule: "label-case",
    message: "Relationship label 'knows' should be uppercase"
  } |
    [r2:Remediation {
      grade: "auto",
      summary: "Recase to KNOWS",
      replace: "knows", with: "KNOWS", line: 4, column: 8
    }]
  ]
]
```

**`guided`** — pato recommends one action; an agent may apply it, a human should
review it.

```gram
[loc4:Location { line: 12, column: 5 } |
  [d4:Diagnostic {
    severity: "error",
    code: "P003",
    rule: "no-duplicate-annotation-key",
    message: "Annotation key 'source' appears twice; only the first is used"
  } |
    [r3:Remediation {
      grade: "guided",
      summary: "Remove the duplicate key at line 12",
      delete_line: 12
    }]
  ]
]
```

**`ambiguous`** — multiple valid fixes exist; pato presents the options and names the
decision. Child patterns are used here because each option needs both a description
and edit coordinates.

```gram
[loc5:Location { line: 5, column: 3 } |
  [d5:Diagnostic {
    severity: "warning",
    code: "P005",
    rule: "dangling-reference",
    message: "'persn' is referenced but not defined in this file",
    decision: "Is 'persn' a misspelling of 'Person', or a missing definition?"
  } |
    [opt1:Option {
      description: "Rename reference to 'Person' (closest match)",
      replace: "persn", with: "Person", line: 5, column: 3
    }]
    [opt2:Option {
      description: "Add a 'persn' definition to this file",
      append: "(persn:Entity)"
    }]
  ]
]
```

### 4.3 Clean file

A file with no diagnostics emits only the header and a summary pattern:

```gram
{ kind: "diagnostics", pato_version: "0.1.0", file: "my.gram" }

(summary:Summary { errors: 0, warnings: 0, auto_fixable: 0 })
```

### 4.4 Multiple files

When multiple files are checked, a `Run` pattern groups the per-file results:

```gram
{ kind: "diagnostics", pato_version: "0.1.0" }

[run:Run { command: "lint" } |
  [fa:FileResult { file: "a.gram", errors: 1, warnings: 0 } |
    [loc1:Location { line: 7, column: 1 } |
      [d1:Diagnostic { ... } | ...]
    ]
  ]
  [fb:FileResult { file: "b.gram", errors: 0, warnings: 2 } |
    ...
  ]
]
```

---

## 5. Diagnostic Codes

Every code is assigned a remediation grade at definition time. Grade is a property of
the rule, not the instance.

| Code | Severity | Grade     | Rule                          | Description                                         |
|------|----------|-----------|-------------------------------|-----------------------------------------------------|
| P001 | error    | guided    | `parse-failure`               | File could not be parsed                            |
| P002 | error    | guided    | `no-duplicate-identity`       | Identity defined more than once in file             |
| P003 | error    | guided    | `no-duplicate-annotation-key` | Annotation has duplicate property key               |
| P004 | warning  | auto      | `label-case`                  | Node label not TitleCase or rel label not UPPERCASE |
| P005 | warning  | ambiguous | `dangling-reference`          | Reference to identity not defined in file           |
| P006 | info     | guided    | `empty-array`                 | `[]` used as value — currently a parse error        |
| P007 | info     | —         | `no-schema`                   | No schema found; semantic checks skipped            |
| P008 | warning  | guided    | `unknown-document-kind`       | `{ kind: "..." }` header value not recognized       |

Codes are stable. New codes are additive. Codes are never reused.

P001 uses `guided` because parse errors often have multiple plausible repairs — pato
identifies the most likely fix but cannot guarantee it.

P005 uses `ambiguous` because a dangling reference could mean the reference is wrong
or the definition is missing. Pato offers both options, using edit distance to suggest
the nearest candidate identity.

P007 carries no grade because it is informational, not a problem.

---

## 6. Exit Code Contract

| Code | Meaning |
|------|---------|
| 0 | Success — no issues |
| 1 | Warnings present, no errors |
| 2 | One or more errors |
| 3 | Tool invocation error (bad arguments, file not found, etc.) |

---

## 7. Subcommand Surface

### 7.1 v0.1 — Core

#### `pato lint [OPTIONS] <files>...`

Parse each file and report syntactic and stylistic issues. Does not require a schema.
Reports issues as diagnostic gram on stdout.

Checks performed:
- Parse success / failure (with location) → P001
- Duplicate pattern identities within a file → P002
- `@` annotation duplicate key detection → P003
- Label case conventions (warning, not error) → P004
- Dangling forward references → P005
- `[]` used as empty array value → P006
- Document header `{ kind: "..." }` validated against known kinds → P008

Options:
- `--fix` — apply all `auto`-grade remediations in place; report remaining diagnostics
- `--output-format gram|text|json` — default: `gram`

```
pato lint my.gram
pato lint --fix my.gram
pato lint --output-format text my.gram    # coloured terminal output
pato lint --output-format json my.gram    # JSON for legacy consumers
pato lint -                               # read from stdin
```

---

#### `pato parse [OPTIONS] <files>...`

Parse each file and emit the resulting pattern structure.

Output modes via `--output-format`:
- `gram` (default) — round-tripped gram notation; a flat sequence of top-level
  patterns with no implicit root wrapper
- `sexp` — s-expression format, matching `gram-lint` / `gramref` output
- `json` — `Pattern<Subject>` tree as JSON array
- `summary` — node/rel/annotation/walk counts (text; not gram)

There is no implicit root wrapper. A gram file is a flat sequence of top-level
patterns; serialising and reparsing is stable with no additional nesting introduced.

```
pato parse my.gram
pato parse --output-format sexp my.gram
pato parse --output-format json my.gram | jq '.[0].subject'
```

---

#### `pato fmt [OPTIONS] <files>...`

Format gram files to canonical style. Idempotent.

`pato fmt` is the exhaustive application of all `auto`-grade remediations. Any file
that `pato lint` reports with only `auto`-grade diagnostics will be clean after
`pato fmt`.

Canonical style choices:
- Consistent spacing around arrow families
- Single blank line between top-level patterns
- Properties sorted alphabetically within a record
- Arrow family and label separator preserved (style is not yet normalised)
- Document header placed at top of file

```
pato fmt my.gram                 # rewrite in place
pato fmt -                       # stdin → stdout
pato fmt --check **/*.gram       # CI: exit non-zero if any file would change
cat my.gram | pato fmt - | pato lint -   # pipeline
```

---

#### `pato rule [<code>]`

Explain a diagnostic rule. With no argument, lists all known rules. With a code,
emits a full description: its name, severity, remediation grade, what triggers it,
and a minimal gram example that would produce it.

Primarily useful for coding agents that encounter an unfamiliar diagnostic code in
`pato lint` output and need to understand it without reading source code.

```
pato rule              # list all rules with one-line descriptions
pato rule P002         # full description of no-duplicate-identity
pato rule --output-format json P002   # machine-readable rule description
```

The output of `pato rule P002` is itself a gram file of kind `"rule"`, consistent
with pato's native output format.

---

#### `pato check [OPTIONS] <files>...`

Runs lint and (if a schema is discoverable) validate. The default "is this correct?"
command for CI and coding agents.

Schema discovery:
- Same-stem `*.schema.gram` alongside the data file is used automatically
- `--schema <path>` overrides for all input files
- No schema → lint only + P007 info emitted

```
pato check my.gram
pato check --schema types.schema.gram my.gram
pato check --output-format json **/*.gram
```

---

### 7.2 v0.2 — Validation

#### `pato validate [OPTIONS] <files>...`

Semantic validation of gram files against a `*.schema.gram` file. Validates that
patterns conform to declared kinds — node labels, relationship types, required
properties, and property value types.

This subcommand depends on two things that are still in progress:

- **Schema conventions** — `gram-schema-conventions.md` has not yet been written.
  The `*.schema.gram` format (using `::` labels, `==>` arrows, and inline comments
  as the kind description) needs to be specified before validation logic can be
  implemented.
- **`PatternKind`** — the `PatternKind` abstraction being developed in the
  `representation-map` proposal is the programmatic representation of what a schema
  describes. `pato validate` will be implemented once that abstraction is stable in
  `pattern-hs` and ported to `pattern-rs`.

`pato validate` is a placeholder in this proposal. Its design will be driven by
what real schema files look like in practice, informed by the archetype approach
explored separately. Revisit after v0.1 is functioning.

```
pato validate --schema types.schema.gram my.gram
```

---

### 7.3 On the Horizon

`pato graph` is deferred until query and transform features exist — at that point the
right output model will be clearer, and `pato graph` output will naturally be gram.

- `pato ingest <source>` — CSV/JSON → gram on stdout
- `pato apply <diagnostic.gram>` — execute remediations from a diagnostic gram file;
  closes the agentic loop: `pato lint my.gram | pato apply -`
- `pato canonicalize` — convert between pattern form and canonical graph form
  (see Appendix B)
- `pato query <pattern>` — graph query against a gram file; outputs matching patterns
  as gram
- `pato diff <a.gram> <b.gram>` — structural diff as gram
- `pato graph` — topology summary as gram

Each is a candidate for a `pato-foo` extension binary before promotion to built-in.

---

## 8. Extensibility

Pato has two distinct extension mechanisms, operating at different layers. Both are
first-class — neither is a workaround.

### 8.1 Binary extensions: `pato-foo`

If `pato xyz` is called and `xyz` is not a built-in subcommand, pato searches `PATH`
for a binary named `pato-xyz` and executes it. This follows the `cargo`-style
convention, with the specific behaviour described below.

**Dispatch mechanics**

- Pato constructs the binary name as `pato-<subcommand>` and searches `PATH`
- The extension binary is exec'd directly — not via a shell — with all remaining
  arguments forwarded verbatim
- Pato's stdin, stdout, and stderr are passed through to the extension transparently;
  pato does not buffer or inspect the extension's output
- The extension's exit code is relayed verbatim as pato's exit code
- The extension inherits pato's full environment

**I/O contract**

Extensions are expected to follow the same I/O conventions as built-in subcommands:
- Stdout carries data — gram by default, honouring `--output-format` if the extension
  supports it
- Stderr carries progress and logging — never data
- The exit code contract from §6 applies: 0 = success, 1 = warnings, 2 = errors,
  3 = invocation error

Pato does not enforce these conventions on extensions — it cannot — but they are the
contract that makes extensions composable with built-in subcommands and with each other.

**Help text discovery**

`pato --help` lists discovered extensions alongside built-in subcommands. Discovery
works by scanning `PATH` for binaries whose names begin with `pato-`. An extension
declares its one-line description by responding to `pato-foo --pato-describe` with a
single line of plain text on stdout. If the binary does not support `--pato-describe`,
pato lists it with no description.

```
pato --help        # lists built-ins and any discovered pato-foo binaries
pato-apply --pato-describe   # → "Apply remediations from a diagnostic gram file"
```

**Naming and publishing conventions**

| Aspect | Convention |
|--------|------------|
| Binary name | `pato-<name>` |
| Crate name | `relateby-pato-<name>` |
| crates.io | Published under the `relateby` namespace |
| Versioning | Extensions declare their own versions independently |

There is no formal compatibility contract between pato and extensions beyond the I/O
conventions above. Extensions that depend on a minimum pato version should document
that requirement.

**Examples of anticipated binary extensions**

- `pato-apply` — execute remediations from a diagnostic gram file
- `pato-ingest` — convert CSV/JSON to gram
- `pato-canonicalize` — convert between pattern form and graph form
- `pato-graphtype` — validate against graph type archetypes (see §7.3)

Each of these may start life as a binary extension before promotion to a built-in if
usage warrants it.

### 8.2 Convention extensions

The second extension mechanism is lighter: a new file naming convention or document
header `kind` value defines a new category of gram file, with associated tooling built
around it. No binary is required; the convention itself is the extension point.

Established conventions:
- `*.schema.gram` — schema files using `::` labels and `==>` arrows to describe kinds
  of patterns; `pato check` discovers them automatically via same-stem lookup
- `{ kind: "diagnostics" }` — pato's own diagnostic output format; any tool that can
  parse gram can consume it

Emerging conventions (not yet implemented):
- `*.graphtype.gram` — graph type archetypes describing a kind of graph by example,
  with inline comments elaborating on semantics; a future `pato-graphtype` extension
  would validate data files against them

Convention extensions are the right choice when:
- The extension is about *interpreting* a gram file differently, not about *running* a
  new operation
- The convention can be self-describing in gram notation with inline comments
- The tooling can be developed independently as a `pato-foo` binary first

The two mechanisms compose naturally: a `*.graphtype.gram` convention extension is
validated by a `pato-graphtype` binary extension. The convention defines the kind; the
binary implements the operation.

---

## 9. Crate Structure

```
crates/pato/
├── Cargo.toml           # package: relateby-pato, binary: pato
├── README.md
├── src/
│   ├── main.rs          # CLI entry point, subcommand dispatch
│   ├── cli.rs           # clap definitions for all subcommands
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── lint.rs
│   │   ├── parse.rs
│   │   ├── fmt.rs
│   │   └── check.rs
│   ├── diagnostics.rs      # Diagnostic, DiagnosticCode, Severity, Remediation types
│   ├── diagnostic_gram.rs  # Serialise Diagnostic → gram notation per §4
│   ├── output.rs           # OutputFormat enum; gram/text/json rendering
│   ├── editor.rs           # In-place file editing (reverse-order, atomic writes)
│   ├── schema.rs           # Schema discovery logic
│   └── extensions.rs       # pato-foo discovery (PATH scan), dispatch, --pato-describe query
└── tests/
    ├── fixtures/
    │   ├── valid/           # gram files with no diagnostics
    │   ├── invalid/         # gram files with known diagnostics (one per code)
    │   └── schema/          # *.schema.gram files for validate/check tests
    ├── lint_tests.rs
    ├── parse_tests.rs
    ├── fmt_tests.rs
    └── check_tests.rs
```

### Key types in `diagnostics.rs`

```rust
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub message: String,
    pub location: Location,
    pub rule: &'static str,
    pub remediation: Remediation,
}

pub enum Remediation {
    Auto      { summary: String, steps: RemediationSteps },
    Guided    { summary: String, steps: RemediationSteps },
    Ambiguous { summary: String, decision: String, options: Vec<RemediationOption> },
    None,  // informational only (P007)
}

/// Either a scalar list of instructions (simple case) or structured Edit patterns
/// (rich case). Mirrors the gram representation: scalar array vs child patterns.
pub enum RemediationSteps {
    Inline(Vec<String>),
    Structured(Vec<Edit>),
}

pub struct RemediationOption {
    pub description: String,
    pub edit: Edit,
}

pub enum Edit {
    Replace   { file: PathBuf, line: u32, column: u32, replace: String, with: String },
    DeleteLine { file: PathBuf, line: u32 },
    Append    { file: PathBuf, content: String },
}
```

`diagnostic_gram.rs` serialises these types to gram notation per §4 using
`relateby-gram::to_gram` as the underlying serialiser.

**Dependency note:** the `ScopeQuery` typeclass and `PatternKind` type being
developed in `proposals/representation-map.md` will eventually influence how
`diagnostic_gram.rs` and `editor.rs` are structured — specifically, how transforms
are expressed and how diagnostic gram output relates to the broader
`RepresentationMap` machinery. For v0.1, these modules are implemented directly
without that abstraction layer; refactoring to use it is a v0.2+ concern once
`ScopeQuery` is stable in `pattern-rs`.

---

## 10. Dependencies

```toml
[dependencies]
relateby-pattern = { path = "../pattern-core", version = "0.1.0" }
relateby-gram    = { path = "../gram-codec",   version = "0.1.0" }
clap             = { version = "4", features = ["derive"] }
serde            = { version = "1", features = ["derive"] }
serde_json       = "1"
thiserror        = "1"
walkdir          = "2"   # for future --recursive flag
```

No async runtime.

---

## 11. Implementation Sequence

**Step 1 — Scaffold**
- Create `crates/pato/` with `Cargo.toml`, binary target; add to workspace
- Implement `pato --version`, `pato --help`, extension dispatch
- Tests: invocation smoke test; unknown subcommand dispatches to `pato-foo`

**Step 2 — Diagnostics and gram serialisation infrastructure**
- Implement `Diagnostic`, `Remediation`, `RemediationSteps`, `Edit` types in
  `diagnostics.rs`
- Implement `diagnostic_gram.rs` — serialise a `Vec<Diagnostic>` to gram per §4;
  use scalar `remediations` array for `Inline` steps, child `Remediation` patterns
  for `Structured` steps
- Implement `output.rs` — `OutputFormat` enum; gram (default), text, json rendering;
  `text` rendering is a coloured compact view of the same gram structure, not a
  separate data model
- Tests: serialise one diagnostic of each remediation grade to gram; verify the
  output parses cleanly with `relateby-gram`; verify JSON serialisation round-trips

**Step 3 — `pato lint`**
- Wire `relateby-gram::parse_gram_notation` → P001 (guided)
- Implement duplicate identity detection → P002 (guided)
- Implement duplicate annotation key detection → P003 (guided)
- Implement label case warnings → P004 (auto)
- Implement dangling reference warnings → P005 (ambiguous); edit distance for
  nearest candidate identity
- Implement `editor.rs` (reverse-order edits, atomic writes)
- Wire `--fix` to apply `auto` remediations via `editor.rs`
- Tests: one fixture per code; verify remediation grade and gram output structure;
  verify `--fix` produces a file that subsequently lints clean; verify gram output
  is itself parseable by `relateby-gram`

**Step 4 — `pato fmt`**
- Implement canonical formatting rules (exhaustive `auto` remediations)
- Implement `-` (stdin→stdout) and `--check` modes
- Idempotency: `fmt(fmt(x)) == fmt(x)` for all fixtures
- Tests: before/after pairs; confirm `pato lint` reports zero `auto` diagnostics
  on all `pato fmt` output

**Step 5 — `pato parse`**
- Implement `gram` output (flat top-level pattern sequence, no root wrapper)
- Implement `sexp` output (matching `gramref` reference output)
- Implement `json` output (array of `Pattern<Subject>`)
- Implement `summary` output (counts)
- Tests: verify gram round-trip stability; verify sexp matches `gramref` for
  shared fixtures; verify no root-wrapper nesting on repeated round-trips

**Step 5b — `pato rule`**
- Implement rule registry: each diagnostic code has a name, description, remediation
  grade, and a minimal fixture gram snippet that would trigger it
- Implement `pato rule` with no argument: emit a gram file listing all rules as
  `Rule` patterns with `code`, `severity`, `grade`, `description` properties
- Implement `pato rule <code>`: emit a single `Rule` pattern with a `trigger_example`
  child pattern containing the minimal gram snippet
- Tests: verify all P-codes have registry entries; verify gram output parses cleanly

**Step 6 — `pato check`**
- Compose lint + schema discovery + validate stub
- Same-stem `*.schema.gram` discovery; `--schema` override
- P007 when no schema found
- Tests: with/without schema; explicit schema path

---

## 12. Open Questions

**Q1 — Style settings.** Arrow family and label separator preferences will eventually
be configurable. Until then, `pato fmt` preserves the author's choices. Style settings
are an additive future feature.

**Q2 — Gramdoc and comment preservation.** The current parser drops comments. Gram
diagnostic output in v0.1 is machine-readable but does not include inline explanatory
comments. When comment-aware parsing arrives, the same gram output becomes the unified
human+machine document — the `text` rendering mode may become unnecessary for most
purposes.

**Q3 — Glob expansion.** Delegated to shell in v0.1. `walkdir` is available for a
future `--recursive` flag.

**Q4 — Multi-file schema.** `--schema <path>` applies one schema to all inputs.
Directory-level schema conventions are deferred, to be driven by actual usage.

**Q5 — `pato apply` and diagnostic gram stability.** The agentic loop
(`pato lint | pato apply -`) depends on `pato apply` consuming diagnostic gram and
executing edits. The diagnostic gram schema in §4 is designed with this in mind and
should be treated as stable API from v0.1 onwards, even though `pato apply` itself
is deferred.

---

## Appendix A: Diagnostic Gram — Modeling Rationale

This appendix records the design exploration that led to the diagnostic gram format
in §4. It is not normative but is preserved because the reasoning is useful context
for future contributors and for understanding what was considered and why.

### The core question

A diagnostic is a statement *about* something in a source file. Gram offers several
structural primitives for representing this: node, annotation, relationship, walk, and
general pattern. Each positions the semantic elements differently.

### Options considered

**As a node with relationships** — `(d1:Diagnostic {...})` connected to
`(loc1:Location {...})` via `(d1)-[:AT]->(loc1)`. Familiar but requires edge
traversal to assemble a complete picture. The relationship between diagnostic and
location is mechanical — location isn't really a peer entity, it's metadata.

**As an annotation** — `@@d1:Diagnostic (loc1:Location)`. Elegant when the annotated
thing is present in scope, but we don't have the source pattern available — only a
reference to a location. Annotation-of-a-location conflates the location with the
finding.

**As a relationship** — `(loc1)-[d1:Diagnostic {...}]->(r1:Remediation)`. Expressive
— the diagnostic *is* the assertion connecting problem to remedy — but strains under
ambiguous remediations where there are multiple outgoing options, and has nowhere to
point for informational diagnostics with no remediation.

**As a walk** — encodes a narrative path through the problem. Unwieldy for attaching
rich properties to the diagnostic itself; requires holding the entire chain in working
memory.

**As a nested pattern** — `[loc:Location | [d:Diagnostic | [r:Remediation]]]`.
Front-loads location (spatial orientation) before finding before remedy. Handles
variable arity naturally: zero remediation children for informational diagnostics,
one for guided, multiple for ambiguous. Reads left-to-right and top-to-bottom.

### Why nested pattern wins

LLMs process tokens sequentially. A representation that front-loads the semantically
important information before the details works *with* sequential processing rather
than against it. The nested pattern model satisfies this:

- **Location first** — the reader is oriented spatially before encountering the finding
- **Finding second** — the diagnostic label and properties are immediately available
- **Remediation last** — steps follow naturally; simple cases use a scalar
  `remediations` array; rich cases use child `Remediation` patterns

The scalar array vs. child pattern split mirrors gram's own property depth budget:
use scalar values when they suffice; promote to structural patterns when richer
representation is needed. This is idiomatic gram rather than a special case.

### What the node-with-relationships model is good for

The node/relationship model is better suited for *querying* — "find all diagnostics
with grade 'auto' across all files" is a natural graph traversal. The nested pattern
model is better suited for *reading* — processing one diagnostic at a time in
document order.

This is not a contradiction. Once `pato canonicalize` exists, diagnostic gram in
pattern form can be converted to canonical graph form for query purposes, and back.
Both representations are informationally equivalent; the conventions define a lossless
round-trip. See Appendix B.

---

## Appendix B: Pattern Form and Graph Form — The Codec Model

This appendix captures design thinking about `pato canonicalize` and the relationship
between gram as a pattern notation and gram as a graph notation.

### Graphs are a special case of patterns

Most graph tools treat graphs as the primitive and other structures as special cases.
The pattern-* model inverts this: patterns are the primitive, and graphs emerge as a
special case when elements are nodes (arity 0) and relationships (arity 2). A sequence
of remediation steps, a located diagnostic, a structured edit are all valid patterns
that don't need to be forced into graph shape to be representable or useful.

This is worth surfacing explicitly to users: gram notation is not a graph serialisation
format that happens to be readable — it is a pattern notation that can represent graphs
among many other structures.

### The codec

Converting between pattern form and graph form is not a lossy transformation. A
`pato canonicalize` command would adopt systematic conventions for mapping all pattern
structure into a graph, preserving semantic detail through labeling and topology
conventions. The resulting graph is a *normal graph with pattern semantics* — every
node, relationship, and property in the graph corresponds to something in the original
pattern, and the original pattern is fully recoverable.

The two representations serve different purposes:

- **Pattern form** — optimised for authoring, reading, and sequential processing by
  humans and LLMs. Nesting makes local structure immediately visible.
- **Graph form** — optimised for querying, traversal, and integration with graph-native
  tools (Neo4j, graph query languages). Global structure is immediately traversable.

The choice between them is a deployment concern, not a semantic one.

### Implications for pato

Diagnostic gram output (§4) is in pattern form. It is designed for reading. When
diagnostic gram needs to be *queried* — "find all auto-fixable diagnostics across a
run" — it should first be canonicalized to graph form. `pato apply` operates on
pattern form (it reads diagnostics sequentially and executes edits). A future graph
query over diagnostics would operate on graph form.

This also means `pato canonicalize` sits at the boundary between pato's pattern-
oriented subcommands and any future graph-query subcommands — it is the bridge between
the two regimes.

### On arity and structural preservation

The arity convention in pattern form (0 = node, 1 = annotation, 2 = relationship,
N ≥ 3 = group/list/hyperedge) must be represented faithfully in graph form. A pattern
with three elements is not the same as a pattern with two elements plus a dangling
node. The canonicalization conventions must encode arity explicitly — likely as a
property on the corresponding graph node — so that round-trip recovery is
unambiguous.


---


## Appendix C: RepresentationMap

The design thinking that grew out of this proposal's diagnostic gram modeling
discussion — connecting `GraphTransform`, `PatternEquivalence`, and the need for named,
invertible, composable mappings between semantically distinct representations — has been
extracted into a standalone proposal:

**`proposals/representation-map.md`**

That document covers: the gap between syntactic equivalence and semantic equivalence,
the `RepresentationMap` type and its relationship to `GraphTransform`, composability,
the two-register model (pattern form vs. graph form), a concrete example using the
diagnostic map, the Haskell → Rust → pato implementation sequence, and open questions
for the prototype.

The short version: `RepresentationMap` is a typed, named, invertibility-declared
wrapper around two `GraphTransform`s, with a `roundTripProperty` that is the machine-
checkable definition of losslessness for that mapping. `pato canonicalize` is the
eventual CLI surface; `pattern-hs` is where the abstraction gets validated first.
