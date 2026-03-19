# Contract: Diagnostic Gram Format

**Version**: v0.1
**Kind header**: `{ kind: "diagnostics", pato_version: "...", file: "..." }`

This is the adopted v0.1 diagnostic contract for `pato lint` and `pato check`.

## Design Principles

- Canonical data is a compact set of problem occurrences.
- Each occurrence carries stable `code`, `rule`, and `remediation` identifiers.
- Per-instance facts are structured properties, not canonical prose.
- Rich human explanation may appear as gram comments, but comments are optional and non-canonical.
- JSON mirrors the canonical structure and omits comments.

## Single-file Output

```gram
{ kind: "diagnostics", pato_version: "0.1.0", file: "my.gram" }

// Identity `alice` is defined twice. The first definition is at 3:1.
// Remediation `rename-duplicate-identity` means one definition should be renamed.
[problem1:Problem {
  line: 7,
  column: 1,
  severity: "error",
  code: "P002",
  rule: "no-duplicate-identity",
  remediation: "rename-duplicate-identity",
  identity: "alice",
  first_line: 3,
  first_column: 1
}]
```

## Remediation Grades

### `auto` — machine-applicable without confirmation

```gram
// Relationship label `knows` should be uppercase.
[problem1:Problem {
  line: 4,
  column: 8,
  severity: "warning",
  code: "P004",
  rule: "label-case",
  remediation: "recase-label",
  label_kind: "relationship",
  observed: "knows",
  expected: "KNOWS"
} |
  (apply1:Apply {
    kind: "replace",
    line: 4,
    column: 8,
    replace: "knows",
    with: "KNOWS"
  })
]
```

### `guided` — one preferred remediation family

```gram
// Annotation key `source` appears more than once before the same pattern.
[problem1:Problem {
  line: 12,
  column: 5,
  severity: "error",
  code: "P003",
  rule: "no-duplicate-annotation-key",
  remediation: "remove-duplicate-annotation",
  key: "source"
}]
```

Guided problems may omit child edits when pato cannot safely encode a single machine edit in v0.1.

### `ambiguous` — multiple valid options

```gram
// `persn` is referenced but not defined in this file.
// Choose whether to rename the reference or add a definition.
[problem1:Problem {
  line: 5,
  column: 3,
  severity: "warning",
  code: "P005",
  rule: "dangling-reference",
  remediation: "resolve-dangling-reference",
  unresolved_identity: "persn",
  suggested_identity: "Person"
} |
  (option1:Option {
    id: "rename-reference",
    kind: "replace",
    line: 5,
    column: 3,
    replace: "persn",
    with: "Person"
  })
  (option2:Option {
    id: "add-definition",
    kind: "append",
    append: "(persn:Entity)"
  })
]
```

### `none` — informational only

```gram
[problem1:Problem {
  line: 1,
  column: 1,
  severity: "info",
  code: "P007",
  rule: "no-schema"
}]
```

## Clean File

```gram
{ kind: "diagnostics", pato_version: "0.1.0", file: "my.gram" }

(summary:Summary { errors: 0, warnings: 0, auto_fixable: 0 })
```

## Multiple Files

```gram
{ kind: "diagnostics", pato_version: "0.1.0" }

[run:Run { command: "lint" } |
  [fa:FileResult { file: "a.gram", errors: 1, warnings: 0 } |
    (problem1:Problem { line: 7, column: 1, severity: "error", code: "P002", rule: "no-duplicate-identity", remediation: "rename-duplicate-identity", identity: "alice", first_line: 3, first_column: 1 })
  ]
  [fb:FileResult { file: "b.gram", errors: 0, warnings: 2 } |
    (problem1:Problem { line: 5, column: 3, severity: "warning", code: "P005", rule: "dangling-reference", remediation: "resolve-dangling-reference", unresolved_identity: "persn", suggested_identity: "Person" })
  ]
]
```

Comments are optional in multi-file output and are omitted by default in machine-oriented contexts.

## Canonical Properties

Every `Problem` carries:

| Property | Type | Notes |
|----------|------|-------|
| `line` | integer | 1-indexed |
| `column` | integer | 1-indexed |
| `severity` | string | `error`, `warning`, `info` |
| `code` | string | Stable P-code |
| `rule` | string | Stable rule identifier |
| `remediation` | string | Stable remediation identifier, when a remediation exists |

Additional problem-specific facts are carried as extra scalar properties such as:

- `identity`
- `first_line`
- `first_column`
- `key`
- `label_kind`
- `observed`
- `expected`
- `unresolved_identity`
- `suggested_identity`
- `kind`

## Child Patterns

### `Apply`

Used for deterministic edits attached to `auto` or `guided` remediations.

| Property | Meaning |
|----------|---------|
| `kind` | `replace`, `deleteLine`, or `append` |
| `line` | 1-indexed line, when applicable |
| `column` | 1-indexed column, when applicable |
| `replace` | Matched text for replace operations |
| `with` | Replacement text for replace operations |
| `delete_line` | Entire line to delete |
| `append` | Content to append |

### `Option`

Used for ambiguous remediations.

| Property | Meaning |
|----------|---------|
| `id` | Stable option identifier within the remediation template |
| `kind` | `replace`, `deleteLine`, or `append` |
| `line` | 1-indexed line, when applicable |
| `column` | 1-indexed column, when applicable |
| `replace` | Matched text for replace operations |
| `with` | Replacement text for replace operations |
| `delete_line` | Entire line to delete |
| `append` | Content to append |

## JSON Parity

JSON uses the same canonical information:

```json
{
  "kind": "diagnostics",
  "patoVersion": "0.1.0",
  "file": "my.gram",
  "problems": [
    {
      "line": 5,
      "column": 3,
      "severity": "warning",
      "code": "P005",
      "rule": "dangling-reference",
      "remediation": "resolve-dangling-reference",
      "facts": {
        "unresolved_identity": "persn",
        "suggested_identity": "Person"
      },
      "options": [
        {
          "id": "rename-reference",
          "edit": {
            "kind": "replace",
            "line": 5,
            "column": 3,
            "replace": "persn",
            "with": "Person"
          }
        },
        {
          "id": "add-definition",
          "edit": {
            "kind": "append",
            "content": "(persn:Entity)"
          }
        }
      ]
    }
  ]
}
```

JSON does not include the optional gram comments.

## Relationship To `pato rule`

`pato rule` exposes the reusable rule and remediation templates referenced by `code`, `rule`,
`remediation`, and `Option.id`. The diagnostic report carries concrete occurrences and bindings;
the rule registry carries reusable explanation and fix-family metadata.
