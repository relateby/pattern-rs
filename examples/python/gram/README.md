# `relateby.gram` Examples

These examples use the supported public package boundary from the combined Python distribution:

```bash
pip install relateby-pattern
```

Use:

```python
from relateby.gram import GramParseError, gram_stringify, gram_validate, parse_gram, round_trip
```

`parse_gram()` returns native `Pattern[Subject]` values, `gram_validate()` returns a list of error strings, and parse failures raise `GramParseError`.

## Quick Start

```python
from relateby.gram import gram_validate, parse_gram, round_trip

patterns = parse_gram("(alice)-[:KNOWS]->(bob)")
print(len(patterns))
print(patterns[0].value.identity)

print(gram_validate("(alice:Person)"))
print(round_trip("(alice:Person)"))
```

## Running the examples

```bash
python examples/python/gram/quickstart.py
python examples/python/gram/demo.py
```

Both examples expect `relateby-pattern` to be installed and use only `relateby.gram`.
