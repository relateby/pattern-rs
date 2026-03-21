---
name: pato
description: Use the pato CLI to inspect, lint, format, parse, and explain gram files. Use when pato is already installed and on PATH, or when updating this local pato skill with `pato skill`.
---

# pato

Use this skill when working with gram files through the `pato` CLI.

## Prerequisites

- `pato` must already be installed and available on `PATH`
- If `pato` is missing, install the CLI first before using the workflows below

## When to use

- You need to inspect, lint, format, parse, or explain gram files
- You need to install or update the local `pato` skill for a project or user
- You need the command output to stay machine-friendly and parseable

## Core workflow

1. Prefer `pato lint` for validation and `pato fmt` for canonical formatting.
2. Use `pato check` when you want linting plus schema discovery in one pass.
3. Use `pato rule <code>` when you need an explanation for a P-code.
4. Re-run the same command after edits to confirm the result is stable.

## Install or update the skill

1. Run `pato skill`.
2. Use `--scope user` for a user-level install.
3. Use `--target cursor` only for user-scope client-native installs.

## Notes

- Project installs must stay discoverable by Vercel-compatible skills tooling.
- The canonical repository source for this skill is `.agents/skills/pato/`.
- Keep stdout for data and stderr for status or errors.

## References

- See [workflows](references/workflows.md) for common `pato` usage patterns.
- See [output-contracts](references/output-contracts.md) for stream conventions.
