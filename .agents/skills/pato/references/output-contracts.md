# pato Output Contracts

## Stdout

- Use stdout for data output.
- Keep gram output canonical and parseable.
- Prefer a single, complete result on stdout.

## Stderr

- Use stderr for status, warnings, and errors.
- Do not mix human progress text into stdout.
- If a command fails, explain the reason on stderr.

## Installation output

- `pato skill` should report the resolved install destination.
- If `--print-path` is used, print the path itself clearly and consistently.
