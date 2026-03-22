# CLI Contract: pato help

**Feature**: 045-pato-help

This contract defines the stable public interface for the `pato help` subcommand and the topic name registry.

---

## Subcommand: `pato help [<topic>]`

### Synopsis

```
pato help [<topic>]
```

### Arguments

| Argument | Required | Description |
|----------|----------|-------------|
| `<topic>` | No | A topic name to display. If omitted, lists available topics. |

### Behavior

**Case 1: Known topic**

```
$ pato help gram
```

- Prints the full markdown content of the topic to **stdout**.
- Exits with code **0**.
- Output is raw markdown, stable across patch versions.

**Case 2: No topic given**

```
$ pato help
```

- Prints a short usage line and the list of all available topic names to **stderr**.
- Exits with code **1**.

**Case 3: Unknown topic**

```
$ pato help no-such-topic
```

- Prints an error message naming the unknown topic to **stderr**.
- Prints the list of available topic names to **stderr**.
- Exits with code **1**.

### Stream contract

Follows the existing `pato` stdout/stderr convention:
- **stdout**: topic content only (data-clean; safe to pipe or redirect).
- **stderr**: usage errors, unknown topic messages, and topic listings.

---

## Topic Name Registry

The current shipped topic names are part of the public contract. Once published, a topic name MUST NOT be removed or renamed without a major version bump.

| Topic Name | Source File |
|------------|-------------|
| `gram` | `skill-package/pato/reference/gram.md` |
| `gram-patterns` | `skill-package/pato/reference/gram-patterns.md` |
| `gram-values` | `skill-package/pato/reference/gram-values.md` |
| `gram-records` | `skill-package/pato/reference/gram-records.md` |
| `gram-annotations` | `skill-package/pato/reference/gram-annotations.md` |
| `gram-graph_elements` | `skill-package/pato/reference/gram-graph_elements.md` |
| `gram-path_equivalences` | `skill-package/pato/reference/gram-path_equivalences.md` |
| `gram-graph_gram` | `skill-package/pato/reference/gram-graph_gram.md` |
| `stdout-stderr-contracts` | `skill-package/pato/reference/stdout-stderr-contracts.md` |

---

## Skill Install Contract Extension

`pato skill` already installs the full `skill-package/pato/` tree. With this feature, that tree gains a `reference/` subdirectory. After installation, the following paths MUST exist:

```
.agents/skills/pato/reference/gram.md
.agents/skills/pato/reference/gram-patterns.md
.agents/skills/pato/reference/gram-values.md
.agents/skills/pato/reference/gram-records.md
.agents/skills/pato/reference/gram-annotations.md
.agents/skills/pato/reference/gram-graph_elements.md
.agents/skills/pato/reference/gram-path_equivalences.md
.agents/skills/pato/reference/gram-graph_gram.md
.agents/skills/pato/reference/stdout-stderr-contracts.md
```

These files are derived from the embedded corpus and MUST match the binary's embedded content exactly (byte-for-byte aside from any line-ending normalization).

---

## Stability Guarantees

- Topic names are stable once published (semver-protected) and match the markdown file basenames exactly.
- Topic names are case-sensitive and may include hyphens and underscores.
- Topic file content MAY be updated across patch releases (corrections, examples).
- Topic content structure (headings, code blocks) SHOULD remain stable across minor releases.
- The `topic` argument is case-sensitive and must be the exact registered name.
