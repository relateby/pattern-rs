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
    if not hasattr(gram, "parse_patterns_as_dicts"):
        raise SystemExit("relateby.gram.parse_patterns_as_dicts is missing")

    graph = pattern.StandardGraph.from_gram("(alice:Person)")
    if graph is None:
        raise SystemExit("StandardGraph.from_gram returned None")

    print(f"Python smoke test passed for {args.expect_distribution}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
