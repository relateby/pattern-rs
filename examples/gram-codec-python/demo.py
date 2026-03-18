#!/usr/bin/env python3
"""Interactive demo for the public relateby.gram package."""

from relateby.gram import GramParseError, gram_validate, parse_gram, round_trip


def print_header(title: str) -> None:
    print(f"\n{'=' * 60}")
    print(f"  {title}")
    print("=" * 60)


def example_parse() -> None:
    print_header("Parse Gram Notation")
    for gram in [
        "(alice)-[:KNOWS]->(bob)",
        "(a) (b) (c)",
        '[team:Team {name: "DevRel"} | (alice), (bob), (charlie)]',
    ]:
        try:
            patterns = parse_gram(gram)
            print(f"\n✓ {gram}")
            print(f"  pattern_count = {len(patterns)}")
            print(f"  identities    = {[pattern.value.identity for pattern in patterns]}")
        except GramParseError as exc:
            print(f"\n✗ {gram}")
            print(f"  {exc}")


def example_validate() -> None:
    print_header("Validate Gram Notation")
    for gram in ["(hello)", "(a)-->(b)", "(unclosed"]:
        print(f"{gram!r} -> {gram_validate(gram)}")


def example_round_trip() -> None:
    print_header("Round Trip")
    original = "(alice:Person)-[:KNOWS]->(bob:Person)"
    print("original  =", original)
    print("serialized=", round_trip(original))


def example_error_handling() -> None:
    print_header("Error Handling")
    try:
        parse_gram("(invalid")
    except GramParseError as exc:
        print(exc)


def main() -> None:
    example_parse()
    example_validate()
    example_round_trip()
    example_error_handling()


if __name__ == "__main__":
    main()
