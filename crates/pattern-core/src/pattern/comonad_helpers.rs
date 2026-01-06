//! Helper functions demonstrating practical applications of Comonad operations.
//!
//! This module provides position-aware helper functions that compute structural
//! metadata at every position in a pattern:
//!
//! - `depth_at()`: Decorate each position with its depth (maximum nesting level)
//! - `size_at()`: Decorate each position with its subtree size (total node count)
//! - `indices_at()`: Decorate each position with its path from root (sequence of indices)
//!
//! These helpers demonstrate how `extend` enables natural expression of context-aware
//! computation, where functions have access to the full subpattern at each position.

use crate::Pattern;

impl<V> Pattern<V> {
    /// Decorates each position with its depth (maximum nesting level).
    ///
    /// This uses `extend` to compute the depth at every position in the pattern.
    /// Depth is defined as:
    /// - Atomic pattern (no elements): depth 0
    /// - Pattern with elements: depth = 1 + max(child depths)
    ///
    /// # Returns
    ///
    /// A pattern where each position's value is the depth of that subpattern.
    ///
    /// # Complexity
    ///
    /// Time: O(n), Space: O(n)
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let p = Pattern::point("x");
    /// assert_eq!(p.depth_at().extract(), &0);
    ///
    /// let p = Pattern::pattern("root", vec![
    ///     Pattern::pattern("a", vec![Pattern::point("x")]),
    ///     Pattern::point("b")
    /// ]);
    /// let depths = p.depth_at();
    /// assert_eq!(depths.extract(), &2); // root has depth 2
    /// assert_eq!(depths.elements()[0].extract(), &1); // "a" has depth 1
    /// assert_eq!(depths.elements()[1].extract(), &0); // "b" has depth 0
    /// ```
    pub fn depth_at(&self) -> Pattern<usize> {
        self.extend(&|subpattern| subpattern.depth())
    }

    /// Decorates each position with the total node count of its subtree.
    ///
    /// This uses `extend` to compute the size at every position in the pattern.
    /// Size is defined as: 1 (self) + sum of child sizes.
    ///
    /// # Returns
    ///
    /// A pattern where each position's value is the size of that subpattern.
    ///
    /// # Complexity
    ///
    /// Time: O(n), Space: O(n)
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let p = Pattern::point("x");
    /// assert_eq!(p.size_at().extract(), &1);
    ///
    /// let p = Pattern::pattern("root", vec![
    ///     Pattern::point("a"),
    ///     Pattern::point("b")
    /// ]);
    /// let sizes = p.size_at();
    /// assert_eq!(sizes.extract(), &3); // root + 2 children
    /// assert_eq!(sizes.elements()[0].extract(), &1);
    /// assert_eq!(sizes.elements()[1].extract(), &1);
    /// ```
    pub fn size_at(&self) -> Pattern<usize> {
        self.extend(&|subpattern| subpattern.size())
    }

    /// Decorates each position with its path from root (sequence of element indices).
    ///
    /// Unlike `depth_at` and `size_at`, this cannot use `extend` because it requires
    /// tracking the path during traversal. The function passed to `extend` only sees
    /// the local subpattern, not the path from the root.
    ///
    /// Path representation:
    /// - Root: empty vector `[]`
    /// - Child at index i: parent_path + `[i]`
    ///
    /// # Returns
    ///
    /// A pattern where each position's value is its path from root.
    ///
    /// # Complexity
    ///
    /// Time: O(n), Space: O(n * depth) due to path vectors
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let p = Pattern::point("x");
    /// let paths = p.indices_at();
    /// let expected: Vec<usize> = vec![];
    /// assert_eq!(paths.extract(), &expected);
    ///
    /// let p = Pattern::pattern("root", vec![
    ///     Pattern::pattern("a", vec![Pattern::point("x")]),
    ///     Pattern::point("b")
    /// ]);
    /// let paths = p.indices_at();
    /// let expected: Vec<usize> = vec![];
    /// assert_eq!(paths.extract(), &expected); // root path
    /// assert_eq!(paths.elements()[0].extract(), &vec![0]); // first child path
    /// assert_eq!(paths.elements()[0].elements()[0].extract(), &vec![0, 0]); // nested child path
    /// assert_eq!(paths.elements()[1].extract(), &vec![1]); // second child path
    /// ```
    pub fn indices_at(&self) -> Pattern<Vec<usize>> {
        fn go<V>(path: Vec<usize>, pattern: &Pattern<V>) -> Pattern<Vec<usize>> {
            Pattern {
                value: path.clone(),
                elements: pattern
                    .elements()
                    .iter()
                    .enumerate()
                    .map(|(i, elem)| {
                        let mut new_path = path.clone();
                        new_path.push(i);
                        go(new_path, elem)
                    })
                    .collect(),
            }
        }
        go(vec![], self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn depth_at_atomic_pattern() {
        let p = Pattern::point("x");
        let depths = p.depth_at();
        assert_eq!(depths.extract(), &0);
    }

    #[test]
    fn depth_at_with_children() {
        let p = Pattern::pattern("root", vec![Pattern::point("a"), Pattern::point("b")]);
        let depths = p.depth_at();
        assert_eq!(depths.extract(), &1); // has children (even though atomic), so depth 1
    }

    #[test]
    fn depth_at_nested() {
        let p = Pattern::pattern(
            "root",
            vec![Pattern::pattern("a", vec![Pattern::point("x")])],
        );
        let depths = p.depth_at();
        assert_eq!(depths.extract(), &2); // depth through "a" to "x"
        assert_eq!(depths.elements()[0].extract(), &1);
    }

    #[test]
    fn size_at_atomic_pattern() {
        let p = Pattern::point("x");
        let sizes = p.size_at();
        assert_eq!(sizes.extract(), &1);
    }

    #[test]
    fn size_at_with_children() {
        let p = Pattern::pattern(
            "root",
            vec![
                Pattern::point("a"),
                Pattern::point("b"),
                Pattern::point("c"),
            ],
        );
        let sizes = p.size_at();
        assert_eq!(sizes.extract(), &4); // 1 root + 3 children
        assert_eq!(sizes.elements()[0].extract(), &1);
    }

    #[test]
    fn indices_at_atomic_pattern() {
        let p = Pattern::point("x");
        let paths = p.indices_at();
        assert_eq!(paths.extract(), &Vec::<usize>::new());
    }

    #[test]
    fn indices_at_with_children() {
        let p = Pattern::pattern("root", vec![Pattern::point("a"), Pattern::point("b")]);
        let paths = p.indices_at();
        assert_eq!(paths.extract(), &Vec::<usize>::new());
        assert_eq!(paths.elements()[0].extract(), &vec![0]);
        assert_eq!(paths.elements()[1].extract(), &vec![1]);
    }

    #[test]
    fn indices_at_nested() {
        let p = Pattern::pattern(
            "root",
            vec![
                Pattern::pattern("a", vec![Pattern::point("x")]),
                Pattern::point("b"),
            ],
        );
        let paths = p.indices_at();
        assert_eq!(paths.extract(), &Vec::<usize>::new());
        assert_eq!(paths.elements()[0].extract(), &vec![0]);
        assert_eq!(paths.elements()[0].elements()[0].extract(), &vec![0, 0]);
        assert_eq!(paths.elements()[1].extract(), &vec![1]);
    }
}
