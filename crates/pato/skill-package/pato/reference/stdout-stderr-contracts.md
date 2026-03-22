# Stdout-Stderr Contracts

This topic explains how `pato` splits data output from human-facing status and
error messages.

## Rule Summary

- Use stdout for data that callers may pipe, capture, or compare.
- Use stderr for warnings, usage text, errors, and other human-facing messages.
- Keep stdout clean so it can be redirected without extra filtering.
- Avoid mixing progress chatter into stdout.

## Rationale

Separating the streams keeps `pato` predictable in scripts and agent workflows.
It lets topic output, parsed data, and formatted gram flow through stdout while
leaving diagnostics and failures on stderr.

## Stdout

Write to stdout when the command is returning content as data.

Examples:

```text
pato help gram
```

```text
pato parse example.gram
```

## Stderr

Write to stderr when the command is telling the user what happened or what to
do next.

Examples:

```text
pato help
```

```text
pato help no-such-topic
```

## Related Topics

- `gram`
- `pato help`
