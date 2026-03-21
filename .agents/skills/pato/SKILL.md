---
name: pato
description: Install and use the pato CLI for gram files. Use when working with gram files, P-codes, `pato lint`, `pato fmt`, `pato parse`, `pato rule`, or `pato skill`.
---

# pato

Use this skill when you need to work with gram files through `pato`.

## When to use

- You need to inspect, lint, format, parse, or explain gram files
- You need to install or update the local `pato` skill for a project or user
- You need the command output to stay machine-friendly and parseable

## Core workflow

1. Prefer `pato lint` for validation and `pato fmt` for canonical formatting.
2. Use `pato check` when you want linting plus schema discovery in one pass.
3. Use `pato rule <code>` when you need an explanation for a P-code.
4. Re-run the same command after edits to confirm the result is stable.

## Gotchas

- Project installs must stay discoverable by Vercel-compatible skills tooling.
- The canonical repository source for this skill is `.agents/skills/pato/`.
- Keep stdout for data and stderr for status or errors.

## References

- See [workflows](references/workflows.md) for common `pato` usage patterns.
- See [output contracts](references/output-contracts.md) for stream conventions.
