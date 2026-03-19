# Data Model: pato CLI Tool

**Phase**: 1 — Design
**Branch**: `041-pato-cli`
**Date**: 2026-03-18

## Core Entities

### Diagnostic

A finding produced by a lint check on a gram file.

| Field | Type | Constraints |
|-------|------|-------------|
| `severity` | `Severity` | One of: `Error`, `Warning`, `Info` |
| `code` | `DiagnosticCode` | One of P001–P008; stable, never reused |
| `rule` | `&'static str` | Canonical rule name (e.g., `"no-duplicate-identity"`) |
| `message` | `String` | Human-readable description of the finding |
| `location` | `Location` | Line and column in the source file |
| `remediation` | `Remediation` | Exactly one remediation, always present (grade `None` for informational) |

**Invariant**: Every `Diagnostic` has a `Remediation`. Informational codes (P007) use `Remediation::None`.

---

### Severity

```
Error   — process exits with code 2
Warning — process exits with code 1 (unless an Error is also present)
Info    — does not affect exit code
```

---

### DiagnosticCode

Stable P-codes. Never reused. Grade is a property of the code, not the instance.

| Code | Severity | Grade | Rule Name |
|------|----------|-------|-----------|
| P001 | Error | Guided | `parse-failure` |
| P002 | Error | Guided | `no-duplicate-identity` |
| P003 | Error | Guided | `no-duplicate-annotation-key` |
| P004 | Warning | Auto | `label-case` |
| P005 | Warning | Ambiguous | `dangling-reference` |
| P006 | Info | Guided | `empty-array` |
| P007 | Info | None | `no-schema` |
| P008 | Warning | Guided | `unknown-document-kind` |

---

### Remediation

A structured fix attached to a diagnostic. Exactly one grade per rule.

**`Auto`** — pato applies without confirmation. Edit coordinates are complete.
- `summary: String` — one-line description of the fix
- `steps: RemediationSteps` — ordered list of edits to apply

**`Guided`** — pato recommends one precise action; an agent may apply it.
- `summary: String`
- `steps: RemediationSteps`

**`Ambiguous`** — multiple valid fixes; pato presents options and names the decision.
- `summary: String`
- `decision: String` — the question the consumer must answer
- `options: Vec<RemediationOption>` — at least 2 options

**`None`** — informational only; no fix is possible or needed (P007).

---

### RemediationSteps

Mirrors the gram representation: scalar array for simple cases, structured edits for rich cases.

**`Inline(Vec<String>)`** — ordered plain-text instructions (simple guided cases).

**`Structured(Vec<Edit>)`** — ordered machine-applicable edit operations.

---

### Edit

A single machine-applicable file change.

**`Replace`** — replace a span of text at a specific location.
- `file: PathBuf`
- `line: u32`
- `column: u32`
- `replace: String` — text to find
- `with: String` — replacement text

**`DeleteLine`** — remove an entire line.
- `file: PathBuf`
- `line: u32`

**`Append`** — add content at the end of the file.
- `file: PathBuf`
- `content: String`

**Invariant**: Edits within a single `--fix` pass are applied in reverse line order to prevent line number drift.

---

### RemediationOption

One choice in an `Ambiguous` remediation.

| Field | Type |
|-------|------|
| `description` | `String` — human-readable description of this option |
| `edit` | `Edit` — the machine-applicable change for this option |

---

### Location

A source position in a gram file.

| Field | Type | Notes |
|-------|------|-------|
| `line` | `u32` | 1-indexed |
| `column` | `u32` | 1-indexed |

**Source of truth**: `Location` is derived from CST byte spans when pato is operating on parsed
source. The line/column pair remains the stable public contract emitted in diagnostics and edits.

---

### SourceSpan (internal)

An internal source range originating from `gram-codec`'s CST parser.

| Field | Type | Notes |
|-------|------|-------|
| `start` | `usize` | Inclusive byte offset |
| `end` | `usize` | Exclusive byte offset |

**Use**: Drives accurate diagnostic placement and edit targeting inside pato. Not emitted directly
in the v0.1 diagnostic gram contract.

---

### DocumentKind

A recognized value for the `kind` property in a gram document header. Checked by P008.

**Recognized in v0.1**:
- `"diagnostics"` — pato diagnostic output
- `"rule"` — pato rule registry output

**How checked**: `pato lint` emits P008 (warning, guided) when a `{ kind: "..." }` document header is present but the value is not in the recognized set.

---

### OutputFormat

The output rendering mode, controlled by `--output-format`.

| Variant | Description |
|---------|-------------|
| `Gram` | Default. Valid gram notation. Machine-parseable. |
| `Text` | ANSI-colored terminal rendering. Auto-detects TTY: colors enabled when stdout is a terminal, stripped when piped/redirected. |
| `Json` | JSON. Compatibility mode for consumers that cannot handle gram. |

---

## State: File Processing

```
Input file
    │
    ▼
[parse_gram_cst]
    │── CST errors ──► P001 Diagnostic (guided)
    ▼
CST tree (Pattern<SyntaxNode>) + SourceSpans
    │
    ├── [lower]
    │       ▼
    │   Parsed patterns (Vec<Pattern<Subject>>)
    │
    ▼
[lint checks]
    │── P002 duplicate identity (guided; CST spans for both occurrences)
    │── P003 duplicate annotation key (guided; SyntaxNode.annotations)
    │── P004 label case (auto; CST label spans)
    │── P005 dangling reference (ambiguous; CST refs + lowered identity set)
    │── P006 empty array (guided)
    │── P008 unknown document kind (guided; document/header node from CST)
    ▼
Vec<Diagnostic>
    │
    ├── [--fix]
    │       └── apply Auto edits in reverse-line-order
    │           skip file entirely if any Ambiguous in scope
    │
    ▼
DiagnosticReport
    │
    ▼
[render]
    ├── gram (default) → stdout
    ├── text (TTY-aware) → stdout
    └── json → stdout
```

---

## Multi-file Grouping

When multiple files are processed, diagnostics are wrapped in a `Run` structure:

```
Run
└── FileResult (file: "a.gram", errors: N, warnings: M)
    └── Location (line: L, column: C)
        └── Diagnostic { ... }
            └── Remediation { ... }
```

Exit code reflects the highest severity across all files in the run.

---

## Lint Rule Details

### P002: no-duplicate-identity

**Trigger**: The same identity string appears as the `identity` field of more than one `Subject` in the same file.

**Detection**: Traverse the CST, collect all definition-site identities with their `SourceSpan`s, and
emit P002 on the second and subsequent occurrences. Convert spans to `Location` for output.

**Grade**: Guided — the fix is to rename one occurrence, but pato cannot choose which without context.

---

### P004: label-case

**Trigger**: A relationship label (arity-2 pattern) is not all-UPPERCASE. A node label (arity-0 or arity-1 pattern) is not TitleCase (first letter uppercase, rest lowercase-or-mixed).

**Detection**: Use CST node kind plus preserved label text/spans to determine whether the label is on
a node or relationship, then emit the auto-fix at the exact span-derived location.

**Grade**: Auto — the correct casing is deterministic and unambiguous.

---

### P005: dangling-reference

**Trigger**: A pattern uses an identity as a reference but that identity has no definition in the same file.

**Detection**: Collect definition/reference occurrences from the CST, then compare reference
identities against the lowered semantic definition set. Emit P005 at the reference span when the
identity is not defined in-file.

**Ambiguous options** (exactly 2 per diagnostic):
1. Rename the reference to the nearest defined identity (by Levenshtein distance via `strsim`)
2. Add a definition for the referenced identity to the file

---

### P008: unknown-document-kind

**Trigger**: The file's first pattern is a bare record (no identity, no labels, no elements) with a `kind` property whose value is not a recognized `DocumentKind`.

**Detection**: Read the document/header subject from the CST document root, then validate its
`kind` property against the known set.

**Grade**: Guided — the fix is to change the `kind` value to a recognized one, or remove it.
