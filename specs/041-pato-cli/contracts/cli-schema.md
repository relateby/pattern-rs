# Contract: pato CLI Schema

**Version**: v0.1
**Binary name**: `pato`
**Crate name**: `relateby-pato`

## Global Options

```
pato [OPTIONS] <SUBCOMMAND>

Options:
  --version              Print version and exit
  --help                 Print help listing built-ins + discovered pato-* extensions
```

## Exit Code Contract (stable API)

| Code | Meaning |
|------|---------|
| 0 | Success — no issues found |
| 1 | Warnings present, no errors (also: `fmt --check` with unformatted files) |
| 2 | One or more errors |
| 3 | Tool invocation error (bad arguments, file not found, unknown extension) |

Exit code reflects highest severity across all input files in a run.

## Subcommand: `pato lint`

```
pato lint [OPTIONS] <FILES>...
pato lint [OPTIONS] -           # read from stdin
```

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `--fix` | flag | off | Apply all `auto`-grade remediations in-place |
| `--output-format` | `gram\|text\|json` | `gram` | Output rendering mode |

**Stdout**: Diagnostic gram (or JSON/text per `--output-format`)
**Stderr**: Progress messages, file modification notices
**Modifies files**: Only with `--fix`; atomic writes; skips file entirely if any `ambiguous` diagnostic in scope

## Subcommand: `pato fmt`

```
pato fmt [OPTIONS] <FILES>...
pato fmt -                      # stdin → stdout
pato fmt --check <FILES>...     # CI mode: exit 1 if any file would change
```

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `--check` | flag | off | Report without modifying; exit 1 if changes needed |

**Stdout**: Nothing (in-place mode) or formatted gram (stdin mode)
**Stderr**: List of modified files
**Modifies files**: Yes (default); no (with `--check` or `-`)
**Exit codes**: 0 = all clean, 1 = files need formatting (check mode), 2 = parse error

## Subcommand: `pato parse`

```
pato parse [OPTIONS] <FILES>...
pato parse [OPTIONS] -
```

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `--output-format` | `gram\|sexp\|json\|summary` | `gram` | Output format |

**Stdout**: Parsed pattern structure in chosen format
- `gram`: flat top-level pattern sequence, no root wrapper; round-trip stable
- `sexp`: tree-sitter sexp notation, matching gramref output for shared fixtures
- `json`: JSON array of `Pattern<Subject>` objects
- `summary`: plain text counts (nodes, relationships, annotations, walks)
**Stderr**: Parse errors

## Subcommand: `pato rule`

```
pato rule                       # list all rules
pato rule [OPTIONS] <CODE>      # describe a specific rule
```

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `--output-format` | `gram\|json` | `gram` | Output format |

**Stdout**: Gram file of kind `"rule"` (or JSON)
**Stderr**: Error if code unknown (exit 3)

## Subcommand: `pato check`

```
pato check [OPTIONS] <FILES>...
```

| Option | Values | Default | Description |
|--------|--------|---------|-------------|
| `--schema` | `<PATH>` | auto-discovered | Schema file to use for all inputs |
| `--output-format` | `gram\|text\|json` | `gram` | Output rendering mode |

**Schema discovery**: Looks for `<stem>.schema.gram` alongside each input file. If found, P007 is suppressed and schema path is noted on stderr. In v0.1, no semantic validation is performed against the schema — discovery is acknowledged only.

## I/O Contract (all subcommands)

- **Stdout**: Always a clean, machine-parseable stream — data only. Never interleaved with progress.
- **Stderr**: Always human-readable progress, warnings, file modification notices. Never data.
- **Stdin**: Gram content when `-` is passed as the file argument.

## Extension Dispatch

When an unknown subcommand `<CMD>` is invoked:

1. Pato searches `PATH` for a binary named `pato-<CMD>`
2. If found: exec it directly (not via shell) with all remaining arguments forwarded verbatim; inherit pato's stdin/stdout/stderr; relay the extension's exit code as pato's exit code
3. If not found: emit error on stderr, exit with code 3

```
pato foo --arg1 val1 --arg2
  → executes: pato-foo --arg1 val1 --arg2
```

## `pato --help` Extension Discovery

Pato scans `PATH` for binaries whose names begin with `pato-`. For each:
- Invokes `pato-<name> --pato-describe` with a short timeout
- If it exits 0 and produces one line of stdout: that line is shown as the description
- If it does not respond or exits non-zero: listed with no description

Extensions appear in the help listing after built-in subcommands.
