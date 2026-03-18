#!/usr/bin/env python3
"""Native `Pattern` operations in pure Python."""

from relateby.pattern import Pattern


def main() -> None:
    tree = Pattern(
        value="root",
        elements=[
            Pattern.point("left"),
            Pattern(value="right", elements=[Pattern.point("leaf")]),
        ],
    )

    print("values:", tree.values())
    print("sum of lengths:", tree.fold(0, lambda acc, value: acc + len(value)))
    print("first starting with 'lea':", tree.find_first(lambda value: value.startswith("lea")))
    print("depth decoration:", tree.extend(lambda subtree: subtree.depth).values())


if __name__ == "__main__":
    main()
