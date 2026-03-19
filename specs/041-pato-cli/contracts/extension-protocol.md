# Contract: pato Extension Protocol

**Version**: v0.1

## Binary Naming

| Aspect | Convention |
|--------|------------|
| Binary name | `pato-<name>` (e.g., `pato-apply`, `pato-ingest`) |
| Crate name | `relateby-pato-<name>` |
| crates.io namespace | `relateby` |

## Dispatch Mechanics

When `pato <unknown-subcommand> [args...]` is invoked:

1. Pato constructs `pato-<unknown-subcommand>`
2. Searches `PATH` for the binary
3. If found: `execv(pato-<name>, [<name>, args...])` — direct exec, not via shell
4. If not found: writes error to stderr, exits with code 3

Streams are inherited, not redirected. Pato does not buffer or inspect extension output.

## Extension I/O Contract

Extensions MUST follow the same conventions as built-in subcommands:

- **Stdout**: Data — gram by default, honoring `--output-format` if supported
- **Stderr**: Progress and logging only — never data
- **Exit codes**: 0 = success, 1 = warnings, 2 = errors, 3 = invocation error

These conventions are not enforced by pato (it cannot inspect the extension). They are the contract that makes extensions composable with built-in subcommands.

## Self-Description

Extensions declare their one-line help text by responding to `--pato-describe`:

```
pato-apply --pato-describe
→ stdout: "Apply remediations from a diagnostic gram file"
→ exit: 0
```

Rules:
- One line of plain text on stdout (no ANSI codes)
- Exit with code 0
- If `--pato-describe` is not supported: exit non-zero; pato lists the extension with no description

## Help Text Discovery

`pato --help` discovers extensions by scanning `PATH` for binaries whose names begin with `pato-`. For each candidate:

1. Invokes `<binary> --pato-describe` with a short timeout (implementation defined, ~1 second)
2. If responds successfully: uses the single line as the description
3. If does not respond or exits non-zero: lists with no description

Extensions appear in the help output after built-in subcommands under a "Extensions" section.

## Version Independence

There is no formal compatibility contract between pato and extensions beyond the I/O conventions above. Extensions that depend on a minimum pato version MUST document that requirement themselves. Pato does not check extension compatibility.
