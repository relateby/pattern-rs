from __future__ import annotations

import argparse
import sys


def main() -> int:
    parser = argparse.ArgumentParser(description="Smoke-test relateby Python imports")
    parser.add_argument(
        "--expect-distribution",
        default="relateby-pattern",
        help="Expected distribution name for logging only",
    )
    args = parser.parse_args()

    import relateby.pattern as pattern  # noqa: WPS433
    import relateby.gram as gram  # noqa: WPS433

    if not hasattr(pattern, "StandardGraph"):
        raise SystemExit("relateby.pattern.StandardGraph is missing")
    if not hasattr(gram, "parse_gram"):
        raise SystemExit("relateby.gram.parse_gram is missing")
    if not hasattr(gram, "validate_gram"):
        raise SystemExit("relateby.gram.validate_gram is missing")
    if not hasattr(gram, "round_trip"):
        raise SystemExit("relateby.gram.round_trip is missing")

    result = gram.parse_gram("(alice:Person)")
    if getattr(result, "pattern_count", 0) != 1:
        raise SystemExit("relateby.gram.parse_gram returned an unexpected ParseResult")

    if gram.validate_gram("(alice:Person)") is not True:
        raise SystemExit("relateby.gram.validate_gram returned False for valid input")

    if gram.round_trip("(alice:Person)") != "(alice:Person)":
        raise SystemExit("relateby.gram.round_trip returned an unexpected result")

    graph = pattern.StandardGraph.from_gram("(alice:Person)")
    if graph is None:
        raise SystemExit("relateby.pattern.StandardGraph.from_gram returned None")
    if getattr(graph, "node_count", 0) != 1:
        raise SystemExit("relateby.pattern.StandardGraph.from_gram returned the wrong graph shape")

    try:
        gram.parse_gram("(alice")
    except ValueError as exc:
        if "relateby.gram.parse_gram" not in str(exc):
            raise SystemExit("relateby.gram.parse_gram did not raise the normalized public error")
    else:
        raise SystemExit("relateby.gram.parse_gram accepted invalid input")

    try:
        pattern.StandardGraph.from_gram("(alice")
    except ValueError as exc:
        if "relateby.pattern" not in str(exc):
            raise SystemExit("relateby.pattern.StandardGraph.from_gram did not raise the normalized public error")
    else:
        raise SystemExit("relateby.pattern.StandardGraph.from_gram accepted invalid input")

    print(f"Python smoke test passed for {args.expect_distribution}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
