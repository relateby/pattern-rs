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
$ pato help gram-notation
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

## Topic Name Registry (v1)

The following topic names are part of the stable public contract. Once published, a topic name MUST NOT be removed or renamed without a major version bump.

| Topic Name | Source File |
|------------|-------------|
| `gram-notation` | `skill-package/pato/reference/gram-notation.md` |
| `stdout-stderr-contracts` | `skill-package/pato/reference/stdout-stderr-contracts.md` |

Topics to be added in follow-on work (content must be authored before shipping):

| Topic Name | Source File |
|------------|-------------|
| `gram-annotation` | `skill-package/pato/reference/gram-annotation.md` |
| `skill-installation` | `skill-package/pato/reference/skill-installation.md` |

---

## Skill Install Contract Extension

`pato skill` already installs the full `skill-package/pato/` tree. With this feature, that tree gains a `reference/` subdirectory. After installation, the following paths MUST exist:

```
.agents/skills/pato/reference/gram-notation.md
.agents/skills/pato/reference/stdout-stderr-contracts.md
```

These files are derived from the embedded corpus and MUST match the binary's embedded content exactly (byte-for-byte aside from any line-ending normalization).

---

## Stability Guarantees

- Topic names are stable once published (semver-protected).
- Topic file content MAY be updated across patch releases (corrections, examples).
- Topic content structure (headings, code blocks) SHOULD remain stable across minor releases.
- The `topic` argument is case-sensitive and must be the exact registered name.
