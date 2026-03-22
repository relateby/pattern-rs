# Gram Records

Records are the object-like value form used in gram for named fields and
metadata.

## What Records Are

- A record is a set of key-value pairs inside braces.
- Records are useful for metadata such as `{ kind: "social" }`.
- At the top of a file, a record acts like a document header.
- The file is still conceptually a pattern collection, but the leading record is
  best treated as metadata in practice.

## Conventions

- Prefer short keys for local metadata.
- Keep field ordering stable when a canonical form matters.
- Use records for value metadata, not for graph structure.
- Keep top-level headers small and descriptive.

## Examples

Simple record:

```gram
{ kind: "social" }
```

Record with multiple fields:

```gram
{ kind: "social", scope: "public", version: 1 }
```

Record used as decoration:

```gram
(person { kind: "social" } name="Ada")
```

## Related Topics

- `gram`
- `gram-values`
- `gram-path_equivalences`
