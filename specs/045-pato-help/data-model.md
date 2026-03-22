# Data Model: pato help and self-documentation

**Feature**: 045-pato-help

---

## Entities

### TopicEntry

A single help topic embedded in the binary.

| Field | Type | Description |
|-------|------|-------------|
| `name` | `&'static str` | Stable public identifier (e.g., `"gram-notation"`). Maps directly to the markdown filename without extension. |
| `content` | `&'static str` | Full markdown content, embedded at compile time via `include_str!`. |

**Invariants**:
- `name` is lowercase, hyphen-separated, no spaces or special characters.
- `name` exactly matches the basename of the source file: `reference/<name>.md`.
- `content` is non-empty (enforced by `include_str!` compile-time check — missing file is a compile error).

**Lifetime**: `'static` — all topic content is embedded in the binary.

---

### TopicCatalog

The complete set of topics available at runtime.

| Field | Type | Description |
|-------|------|-------------|
| `topics` | `&'static [TopicEntry]` | Ordered slice of all embedded topics. |

**Operations**:

```
find(name: &str) -> Option<&'static TopicEntry>
    Linear search by name. Returns the entry if found.

names() -> impl Iterator<Item = &'static str>
    Returns topic names in catalog order (for listing).
```

**Population**: Populated entirely at compile time via `include_str!` in `topic_catalog.rs`. No runtime registration or discovery.

---

## Source File Mapping

```
crates/pato/
└── skill-package/pato/
    └── reference/               ← canonical topic corpus
        ├── gram-notation.md     → TopicEntry { name: "gram-notation", ... }
        └── stdout-stderr-contracts.md
                                 → TopicEntry { name: "stdout-stderr-contracts", ... }
```

Each file becomes one `TopicEntry`. The catalog slice is defined once in `topic_catalog.rs` and imported everywhere that needs topic lookup.

---

## HelpArgs (CLI)

Arguments for the `help` subcommand, added to `cli.rs`.

| Field | Type | Description |
|-------|------|-------------|
| `topic` | `Option<String>` | The topic name to display. If `None`, list available topics. |

---

## Exit Codes

| Scenario | Code |
|----------|------|
| Topic found and printed | `0` |
| No topic given (list mode) | `1` |
| Unknown topic | `1` |

---

## State Transitions

```
pato help
  ├─ topic = None          → print topic list        → exit 1
  ├─ topic = known name    → print topic content     → exit 0
  └─ topic = unknown name  → print error + topic list → exit 1
```

No persistent state. All resolution is pure lookup against the static catalog.
