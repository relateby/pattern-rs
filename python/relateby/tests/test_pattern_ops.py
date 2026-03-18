from relateby.pattern import Pattern, Subject


def test_values_returns_subjects_in_pre_order():
    pattern = Pattern(
        value=Subject.from_id("root"),
        elements=[
            Pattern.point(Subject.from_id("left")),
            Pattern(
                value=Subject.from_id("right"),
                elements=[Pattern.point(Subject.from_id("leaf"))],
            ),
        ],
    )

    assert [subject.identity for subject in pattern.values()] == [
        "root",
        "left",
        "right",
        "leaf",
    ]
