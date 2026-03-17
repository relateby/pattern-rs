"""Property-based law tests for Pattern operations.

Tests functor, foldable, and comonad laws for the native Python Pattern.
These laws ensure algebraic correctness of the implementation.
"""
import pytest
from relateby.pattern import Pattern
from relateby.pattern._subject import Subject
from relateby.pattern._value import StringVal


# --- Helpers ---

def mk_subject(identity: str) -> Subject:
    return Subject.from_id(identity)


def mk_pattern(identity: str, *children: Pattern[Subject]) -> Pattern[Subject]:
    return Pattern(value=mk_subject(identity), elements=list(children))


atomic = Pattern.point(mk_subject("a"))
two_level = mk_pattern("root", mk_pattern("child1"), mk_pattern("child2"))
deep = mk_pattern("r", mk_pattern("m", mk_pattern("leaf")))

test_patterns = [atomic, two_level, deep]


# --- Functor laws ---

class TestFunctorLaws:
    def test_identity(self):
        """map(id)(p) == p"""
        for p in test_patterns:
            result = p.map(lambda x: x)
            assert result == p

    def test_composition(self):
        """map(f ∘ g) == map(f) ∘ map(g)"""
        f = lambda s: s.with_label("A")
        g = lambda s: s.with_label("B")
        fg = lambda s: f(g(s))

        for p in test_patterns:
            assert p.map(fg) == p.map(g).map(f)


# --- Foldable ---

class TestFoldable:
    def test_pre_order_root_first(self):
        """fold visits root before children."""
        order = two_level.fold([], lambda acc, s: acc + [s.identity])
        assert order[0] == "root"
        assert "child1" in order
        assert "child2" in order
        assert order.index("root") < order.index("child1")

    def test_atomic_visits_one_value(self):
        assert atomic.fold(0, lambda acc, _: acc + 1) == 1

    def test_values_matches_fold_order(self):
        """values() pre-order order matches fold pre-order order."""
        for p in test_patterns:
            from_values = [s.identity for s in p.values()]
            from_fold = p.fold([], lambda acc, s: acc + [s.identity])
            assert from_values == from_fold


# --- Comonad laws ---

class TestComonadLaws:
    def test_extract_extend(self):
        """extract(extend(f)(p)) == f(p)"""
        f = lambda p: p.depth
        for p in test_patterns:
            result = p.extend(f).extract()
            assert result == f(p)

    def test_extend_extract_identity(self):
        """extend(extract)(p) == p"""
        for p in test_patterns:
            result = p.extend(lambda q: q.extract())
            assert result == p

    def test_duplicate_extract(self):
        """duplicate then extract gives back original."""
        for p in test_patterns:
            dup = p.duplicate()
            assert dup.extract() == p


# --- Pattern structural properties ---

class TestPatternProperties:
    def test_atomic_properties(self):
        assert atomic.depth == 0
        assert atomic.size == 1
        assert atomic.length == 0
        assert atomic.is_atomic is True

    def test_two_level_properties(self):
        assert two_level.depth == 1
        assert two_level.size == 3
        assert two_level.length == 2
        assert two_level.is_atomic is False

    def test_deep_properties(self):
        assert deep.depth == 2
        assert deep.size == 3
        assert deep.length == 1


# --- Structural equality ---

class TestStructuralEquality:
    def test_identical_atomic_patterns_equal(self):
        p1 = Pattern.point(Subject.from_id("x"))
        p2 = Pattern.point(Subject.from_id("x"))
        assert p1 == p2

    def test_different_atomic_patterns_not_equal(self):
        p1 = Pattern.point(Subject.from_id("x"))
        p2 = Pattern.point(Subject.from_id("y"))
        assert p1 != p2

    def test_point_and_of_are_equal(self):
        s = Subject.from_id("a")
        assert Pattern.point(s) == Pattern.of(s)


# --- Subject equality ---

class TestSubjectEquality:
    def test_same_identity_equal(self):
        s1 = Subject.from_id("a")
        s2 = Subject.from_id("a")
        assert s1 == s2

    def test_with_label_preserves_identity(self):
        s1 = Subject.from_id("a").with_label("Person")
        s2 = Subject.from_id("a").with_label("Person")
        assert s1 == s2

    def test_different_labels_not_equal(self):
        s1 = Subject.from_id("a").with_label("Person")
        s2 = Subject.from_id("a").with_label("Employee")
        assert s1 != s2


# --- find_first ---

class TestFindFirst:
    def test_find_matching_value(self):
        result = two_level.find_first(lambda s: s.identity == "child1")
        assert result is not None
        assert result.identity == "child1"

    def test_find_none_when_absent(self):
        result = two_level.find_first(lambda s: s.identity == "nonexistent")
        assert result is None

    def test_find_root_first(self):
        result = two_level.find_first(lambda s: len(s.identity) > 0)
        assert result is not None
        assert result.identity == "root"


# --- filter ---

class TestFilter:
    def test_filter_returns_matching_subtrees(self):
        results = two_level.filter(lambda p: p.is_atomic)
        assert len(results) == 2
        identities = {p.value.identity for p in results}
        assert identities == {"child1", "child2"}

    def test_filter_empty_when_no_match(self):
        results = atomic.filter(lambda p: False)
        assert results == []
