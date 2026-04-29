# Contract: Python API — relateby.gram (048)

Module: `relateby.gram`

All functions raise `GramParseError` on both parse failure and serialization failure.

---

## `parse(input: str) -> list[Pattern[Subject]]`

Parse gram notation into a list of language-native Pattern objects.

```python
from relateby.gram import parse
from relateby.pattern import Pattern, Subject

patterns: list[Pattern[Subject]] = parse("(alice)-[:KNOWS]->(bob)")
```

**Input**: gram notation string (may be empty or whitespace)
**Output**: list of `Pattern[Subject]` (empty list if input is empty)
**Errors**: raises `GramParseError` if input is syntactically invalid
**Note**: if a leading bare record is present, it appears as the first element with empty identity, no labels, and non-empty properties. Use `parse_with_header` to separate it.

**Alias**: `parse_gram` (retained for backwards compatibility)

---

## `stringify(patterns: list[Pattern[Subject]]) -> str`

Serialize a list of Pattern objects to gram notation.

```python
from relateby.gram import stringify

gram_text: str = stringify(patterns)
```

**Input**: list of `Pattern[Subject]` (may be empty)
**Output**: gram notation string; patterns joined by newlines
**Errors**: raises `GramParseError` if a pattern contains values not representable in gram notation (e.g. nested maps)

**Alias**: `gram_stringify` (retained for backwards compatibility)

---

## `parse_with_header(input: str) -> tuple[dict | None, list[Pattern[Subject]]]`

Parse gram notation, separating an optional leading header record from the patterns.

```python
from relateby.gram import parse_with_header

header, patterns = parse_with_header("{version: 1} (alice)-[:KNOWS]->(bob)")
# header == {"version": 1}
# patterns == [Pattern(...)]

header, patterns = parse_with_header("(alice)-[:KNOWS]->(bob)")
# header is None
# patterns == [Pattern(...)]
```

**Input**: gram notation string
**Output**: two-tuple `(header, patterns)` where `header` is `dict | None` and `patterns` is `list[Pattern[Subject]]`
**Errors**: raises `GramParseError` if input is syntactically invalid
**Guarantee**: the returned pattern list never contains the header record — it is always separated into the first tuple element

---

## `stringify_with_header(header: dict | None, patterns: list[Pattern[Subject]]) -> str`

Serialize a header record and a list of Pattern objects to gram notation.

```python
from relateby.gram import stringify_with_header

gram_text = stringify_with_header({"version": 1}, patterns)
# → "{version: 1}\n(alice)-[:KNOWS]->(bob)"

gram_text = stringify_with_header(None, patterns)
# → "(alice)-[:KNOWS]->(bob)"  (same as stringify)

gram_text = stringify_with_header({"version": 1}, [])
# → "{version: 1}"
```

**Input**: `header` (`dict | None`), `patterns` (list of `Pattern[Subject]`)
**Output**: gram notation string with header (if non-None) as the first line, followed by patterns
**Errors**: raises `GramParseError` if header or patterns contain values not representable in gram notation

---

## `validate(input: str) -> list[str]`

Validate gram notation syntax. Returns error strings; empty list means valid.

```python
errors = validate("(alice)-[:KNOWS]->(bob)")  # []
errors = validate("(unclosed")                  # ["Parse error at 1:9: ..."]
```

**Note**: existing `gram_validate` function — unchanged, kept as is.

---

## Round-trip guarantee

```python
header, patterns = parse_with_header(gram_text)
recovered = stringify_with_header(header, patterns)
header2, patterns2 = parse_with_header(recovered)
assert header == header2
assert patterns == patterns2
```

---

## `__all__` exports

```python
__all__ = [
    "GramParseError",
    "parse",
    "parse_with_header",
    "stringify",
    "stringify_with_header",
    "validate",
    # Aliases (backwards compat)
    "parse_gram",
    "gram_stringify",
    "gram_validate",
    "round_trip",
]
```
