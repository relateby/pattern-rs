#!/usr/bin/env python3
"""More complete `relateby.gram` example using native Pattern values."""

from relateby.gram import GramParseError, gram_stringify, gram_validate, parse_gram


def main() -> None:
    gram = "(alice:Person)-[:KNOWS]->(bob:Person)"
    patterns = parse_gram(gram)

    print("parsed:", len(patterns), "pattern(s)")
    print("root identity:", patterns[0].value.identity)
    print("validation errors:", gram_validate(gram))
    print("stringified:", gram_stringify(patterns))

    try:
        parse_gram("(alice")
    except GramParseError as error:
        print("error input:", error.input)
        print("error cause:", error.cause)


if __name__ == "__main__":
    main()
