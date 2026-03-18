#!/usr/bin/env python3
"""Basic native `relateby.pattern` usage."""

from relateby.pattern import Pattern, StringVal, Subject


def main() -> None:
    alice = Subject.from_id("alice").with_label("Person").with_property("name", StringVal("Alice"))
    bob = Subject.from_id("bob").with_label("Person")
    tree = Pattern(value=alice, elements=[Pattern.point(bob)])

    print("root identity:", tree.value.identity)
    print("size:", tree.size)
    print("depth:", tree.depth)
    print("values:", [subject.identity for subject in tree.values()])


if __name__ == "__main__":
    main()
