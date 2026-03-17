#!/usr/bin/env python3
"""Interactive demo for the public relateby.gram package."""

from relateby.gram import parse_gram, round_trip, validate_gram


def print_header(title):
    print(f"\n{'=' * 60}")
    print(f"  {title}")
    print("=" * 60)


def example_parse():
    print_header("Parse Gram Notation")
    for gram in [
        "(alice)-[:KNOWS]->(bob)",
        "(a) (b) (c)",
        '[team:Team {name: "DevRel"} | (alice), (bob), (charlie)]',
    ]:
        try:
            result = parse_gram(gram)
            print(f"\n✓ {gram}")
            print(f"  pattern_count = {result.pattern_count}")
            print(f"  identifiers   = {result.identifiers}")
        except ValueError as exc:
            print(f"\n✗ {gram}")
            print(f"  {exc}")


def example_validate():
    print_header("Validate Gram Notation")
    for gram in ["(hello)", "(a)-->(b)", "(unclosed"]:
        print(f"{gram!r} -> {validate_gram(gram)}")


def example_round_trip():
    print_header("Round Trip")
    original = "(alice:Person)-[:KNOWS]->(bob:Person)"
    print("original  =", original)
    print("serialized=", round_trip(original))


def example_error_handling():
    print_header("Error Handling")
    try:
        parse_gram("(invalid")
    except ValueError as exc:
        print(exc)


def main():
    example_parse()
    example_validate()
    example_round_trip()
    example_error_handling()


if __name__ == "__main__":
    main()

