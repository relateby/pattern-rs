# Contract: Diagnostic Gram Format

**Version**: v0.1 (stable from v0.1 onwards)
**Kind header**: `{ kind: "diagnostics", pato_version: "...", file: "..." }`

This format is stable API. Future `pato apply` will consume it. Do not make breaking changes without a version increment in the header.

## Single-file Output

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

## Remediation Grades

### `auto` — machine-applicable without confirmation

```gram
[loc:Location { line: 4, column: 8 } |
  [d:Diagnostic {
    severity: "warning",
    code: "P004",
    rule: "label-case",
    message: "Relationship label 'knows' should be uppercase"
  } |
    [r:Remediation {
      grade: "auto",
      summary: "Recase to KNOWS",
      replace: "knows", with: "KNOWS", line: 4, column: 8
    }]
  ]
]
```

### `guided` — one precise fix recommended

```gram
[loc:Location { line: 12, column: 5 } |
  [d:Diagnostic {
    severity: "error",
    code: "P003",
    rule: "no-duplicate-annotation-key",
    message: "Annotation key 'source' appears twice; only the first is used"
  } |
    [r:Remediation {
      grade: "guided",
      summary: "Remove the duplicate key at line 12",
      delete_line: 12
    }]
  ]
]
```

### `ambiguous` — multiple valid fixes; consumer must choose

```gram
[loc:Location { line: 5, column: 3 } |
  [d:Diagnostic {
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

## Clean File (no diagnostics)

```gram
{ kind: "diagnostics", pato_version: "0.1.0", file: "my.gram" }

(summary:Summary { errors: 0, warnings: 0, auto_fixable: 0 })
```

## Multiple Files

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

## Scalar vs Structured Remediations

**Scalar `remediations` array** (simple guided — inline string steps):
```gram
[d:Diagnostic {
  ...
  remediations: ["Step 1: do this", "Step 2: do that"]
}]
```

**Child `Remediation` patterns** (rich — has edit coordinates):
```gram
[d:Diagnostic { ... } |
  [r:Remediation {
    grade: "guided",
    summary: "...",
    replace: "old", with: "new", line: 7, column: 3
  }]
]
```

The choice mirrors gram's property depth budget: use scalars when they suffice; promote to structural patterns when edit coordinates or multiple sub-steps are needed.

## Edit Properties in Remediation Patterns

| Property | Applicable to | Description |
|----------|---------------|-------------|
| `replace` | auto, guided | Text to find |
| `with` | auto, guided | Replacement text |
| `line` | auto, guided | Line number (1-indexed) |
| `column` | auto, guided | Column number (1-indexed) |
| `delete_line` | guided | Line number to delete entirely |
| `append` | guided, ambiguous options | Content to append to file |

## Rule Output Format

`pato rule` emits a gram file of kind `"rule"`:

```gram
{ kind: "rule", pato_version: "0.1.0" }

(p002:Rule {
  code: "P002",
  name: "no-duplicate-identity",
  severity: "error",
  grade: "guided",
  description: "Identity defined more than once in the same file"
})
```

With `pato rule P002` (full detail):

```gram
{ kind: "rule", pato_version: "0.1.0" }

[p002:Rule {
  code: "P002",
  name: "no-duplicate-identity",
  severity: "error",
  grade: "guided",
  description: "Identity defined more than once in the same file"
} |
  [example:TriggerExample {
    description: "Minimal gram that triggers P002"
  } |
    (alice:Person)
    (alice:Employee)
  ]
]
```
