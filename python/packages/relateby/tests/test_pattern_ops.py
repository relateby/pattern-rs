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


# Helper tree for US1 tests:
# root(1)
#   left(2)
#   right(3)
#     leaf(4)
def make_tree():
    return Pattern(
        value=1,
        elements=[
            Pattern.point(2),
            Pattern(value=3, elements=[Pattern.point(4)]),
        ],
    )


# --- any_value ---

def test_any_value_true_at_root():
    assert make_tree().any_value(lambda v: v == 1) is True


def test_any_value_true_at_leaf():
    assert make_tree().any_value(lambda v: v == 4) is True


def test_any_value_false_when_none_match():
    assert make_tree().any_value(lambda v: v > 100) is False


def test_any_value_const_true_always_true():
    assert make_tree().any_value(lambda _: True) is True


def test_any_value_const_false_on_leaf_is_false():
    assert Pattern.point(42).any_value(lambda _: False) is False


# --- all_values ---

def test_all_values_true_when_all_satisfy():
    assert make_tree().all_values(lambda v: v > 0) is True


def test_all_values_false_when_root_fails():
    assert make_tree().all_values(lambda v: v != 1) is False


def test_all_values_false_when_leaf_fails():
    assert make_tree().all_values(lambda v: v < 4) is False


def test_all_values_const_true_always_true():
    assert make_tree().all_values(lambda _: True) is True


def test_all_values_const_false_returns_false():
    assert make_tree().all_values(lambda _: False) is False


# --- matches ---

def test_matches_identical_leaves():
    assert Pattern.point(42).matches(Pattern.point(42)) is True


def test_matches_different_leaves():
    assert Pattern.point(1).matches(Pattern.point(2)) is False


def test_matches_reflexivity():
    p = make_tree()
    assert p.matches(p) is True


def test_matches_structurally_equal_trees():
    assert make_tree().matches(make_tree()) is True


def test_matches_different_structure():
    a = Pattern(value=1, elements=[Pattern.point(2)])
    b = Pattern(value=1, elements=[Pattern.point(3)])
    assert a.matches(b) is False


# --- contains ---

def test_contains_reflexivity():
    p = make_tree()
    assert p.contains(p) is True


def test_contains_direct_child():
    assert make_tree().contains(Pattern.point(2)) is True


def test_contains_nested_leaf():
    assert make_tree().contains(Pattern.point(4)) is True


def test_contains_false_when_not_present():
    assert make_tree().contains(Pattern.point(99)) is False


def test_contains_subtree():
    subtree = Pattern(value=3, elements=[Pattern.point(4)])
    assert make_tree().contains(subtree) is True


# --- para ---

def test_para_leaf_receives_empty_child_results():
    leaf = Pattern.point(42)
    child_count = leaf.para(lambda _p, rs: len(rs))
    assert child_count == 0


def test_para_tree_height():
    def height(p, rs):
        return 0 if not rs else 1 + max(rs)

    assert Pattern.point(1).para(height) == 0
    assert make_tree().para(height) == 2


def test_para_value_sum_matches_fold():
    def para_sum(p, rs):
        return p.value + sum(rs)

    def fold_sum(p):
        return p.value + sum(fold_sum(e) for e in p.elements)

    assert make_tree().para(para_sum) == fold_sum(make_tree())


def test_para_f_receives_full_sub_pattern():
    # count total nodes via para (f can inspect sub-pattern, not just value)
    def size(p, rs):
        return 1 + sum(rs)

    assert Pattern.point(99).para(size) == 1
    assert make_tree().para(size) == 4  # root + left + right + leaf


def test_para_single_level_height_is_1():
    one_level = Pattern(value=0, elements=[Pattern.point(1), Pattern.point(2)])

    def height(p, rs):
        return 0 if not rs else 1 + max(rs)

    assert one_level.para(height) == 1


# --- Pattern.pattern ---

def test_pattern_creates_non_atomic():
    p = Pattern.pattern(1, [Pattern.point(2), Pattern.point(3)])
    assert p.is_atomic is False
    assert len(p.elements) == 2


def test_pattern_with_empty_elements_equals_point():
    assert Pattern.pattern(42, []) == Pattern.point(42)


def test_pattern_children_are_provided_elements():
    a = Pattern.point("a")
    b = Pattern.point("b")
    p = Pattern.pattern("root", [a, b])
    assert p.elements[0] == a
    assert p.elements[1] == b


# --- Pattern.from_list ---

def test_from_list_creates_atomic_children():
    p = Pattern.from_list("root", ["a", "b", "c"])
    assert len(p.elements) == 3
    assert all(e.is_atomic for e in p.elements)


def test_from_list_with_empty_values_equals_point():
    assert Pattern.from_list(42, []) == Pattern.point(42)


def test_from_list_length_matches_input():
    vals = [1, 2, 3, 4, 5]
    assert len(Pattern.from_list(0, vals).elements) == len(vals)


def test_from_list_child_values_match_input():
    p = Pattern.from_list("root", [10, 20, 30])
    assert [e.value for e in p.elements] == [10, 20, 30]


# --- Pattern.unfold and module-level unfold ---

def test_unfold_leaf_when_no_children():
    p = Pattern.unfold(lambda n: (n, []), 42)
    assert p.is_atomic is True
    assert p.value == 42


def test_unfold_countdown_chain():
    chain = Pattern.unfold(lambda n: (n, [n - 1] if n > 0 else []), 3)
    assert chain.value == 3
    assert chain.elements[0].value == 2
    assert chain.elements[0].elements[0].value == 1
    assert chain.elements[0].elements[0].elements[0].value == 0
    assert chain.elements[0].elements[0].elements[0].is_atomic is True


def test_unfold_binary_tree():
    tree = Pattern.unfold(lambda n: (n, [n - 1, n - 1] if n > 0 else []), 2)
    assert tree.depth == 2
    assert tree.size == 7  # 1 + 2 + 4


def test_unfold_terminates_on_empty_children():
    p = Pattern.unfold(lambda s: (s, []), "leaf")
    assert p.is_atomic is True


def test_module_level_unfold_alias():
    from relateby.pattern import unfold
    p = unfold(lambda n: (n, []), 5)
    assert p.value == 5
    assert p.is_atomic is True


# --- combine ---

def test_combine_merges_root_values():
    a = Pattern.point("hello")
    b = Pattern.point(" world")
    merged = a.combine(b, lambda x, y: x + y)
    assert merged.value == "hello world"
    assert merged.elements == []


def test_combine_concatenates_elements():
    a = Pattern.from_list("root", [1, 2])
    b = Pattern.from_list("root", [3, 4])
    merged = a.combine(b, lambda x, y: x + y)
    assert len(merged.elements) == 4
    assert [e.value for e in merged.elements] == [1, 2, 3, 4]


def test_combine_identity_law():
    # combining p with an identity (value 0, no elements) preserves p's structure
    p = Pattern.pattern(1, [Pattern.point(2), Pattern.point(3)])
    empty = Pattern.pattern(0, [])
    merged = p.combine(empty, lambda x, y: x + y)
    assert merged.value == 1
    assert len(merged.elements) == 2


def test_combine_associativity():
    a = Pattern.pattern(1, [Pattern.point(10)])
    b = Pattern.pattern(2, [Pattern.point(20)])
    c = Pattern.pattern(3, [Pattern.point(30)])
    add = lambda x, y: x + y
    # (a combine b) combine c
    left_assoc = a.combine(b, add).combine(c, add)
    # a combine (b combine c)
    right_assoc = a.combine(b.combine(c, add), add)
    assert left_assoc.value == right_assoc.value
    assert len(left_assoc.elements) == len(right_assoc.elements)
    assert [e.value for e in left_assoc.elements] == [e.value for e in right_assoc.elements]


def test_combine_two_atomic_patterns():
    a = Pattern.point(10)
    b = Pattern.point(20)
    merged = a.combine(b, lambda x, y: x * y)
    assert merged.is_atomic is True
    assert merged.value == 200


# --- depth_at ---

def test_depth_at_leaf_has_depth_0():
    result = Pattern.point(42).depth_at()
    assert result.value == 0
    assert result.is_atomic is True


def test_depth_at_root_of_tree():
    assert make_tree().depth_at().value == 2


def test_depth_at_children_have_correct_depths():
    annotated = make_tree().depth_at()
    # left child (point(2)) is a leaf → depth 0
    assert annotated.elements[0].value == 0
    # right child (3 with one leaf) has depth 1
    assert annotated.elements[1].value == 1


def test_depth_at_preserves_structure():
    annotated = make_tree().depth_at()
    assert len(annotated.elements) == 2
    assert len(annotated.elements[1].elements) == 1


# --- size_at ---

def test_size_at_leaf_has_size_1():
    result = Pattern.point(42).size_at()
    assert result.value == 1
    assert result.is_atomic is True


def test_size_at_root_of_tree():
    assert make_tree().size_at().value == 4


def test_size_at_children_have_correct_sizes():
    annotated = make_tree().size_at()
    # left child (point(2)) is a leaf → size 1
    assert annotated.elements[0].value == 1
    # right child (3 with one leaf) has size 2
    assert annotated.elements[1].value == 2


def test_size_at_preserves_structure():
    annotated = make_tree().size_at()
    assert len(annotated.elements) == 2


# --- indices_at ---

def test_indices_at_root_has_empty_path():
    assert Pattern.point(99).indices_at().value == []


def test_indices_at_leaf_root_has_empty_path():
    assert Pattern.point(42).indices_at().value == []


def test_indices_at_direct_children_have_single_element_paths():
    annotated = make_tree().indices_at()
    assert annotated.elements[0].value == [0]
    assert annotated.elements[1].value == [1]


def test_indices_at_grandchild_has_two_element_path():
    annotated = make_tree().indices_at()
    # right child (index 1) has one child at index 0 → path [1, 0]
    assert annotated.elements[1].elements[0].value == [1, 0]


def test_indices_at_preserves_structure():
    annotated = make_tree().indices_at()
    assert len(annotated.elements) == 2
    assert len(annotated.elements[1].elements) == 1
