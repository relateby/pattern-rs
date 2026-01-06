//! Comonad operations for Pattern.
//!
//! This module provides comonad operations (`extract` and `extend`) that work with Pattern's
//! "decorated sequence" semantics, where the value decorates the elements.
//!
//! # Decorated Sequence Semantics
//!
//! Pattern is fundamentally a "decorated sequence":
//! - **Elements ARE the pattern** - the actual content (e.g., `["A", "B", "A"]`)
//! - **Value DECORATES the elements** - provides information about them (e.g., `"sonata"`)
//!
//! This is what makes Pattern a natural Comonad:
//! - `extract`: Access the decorative information (the value)
//! - `extend`: Compute new decorative information based on context (the subpattern)
//!
//! # Examples
//!
//! ```
//! use pattern_core::Pattern;
//!
//! // Create a pattern
//! let p = Pattern::point("root");
//!
//! // Extract the decorative value
//! assert_eq!(p.extract(), &"root");
//!
//! // Compute new decorations based on context
//! let depths = p.extend(&|subpattern| subpattern.depth());
//! assert_eq!(depths.extract(), &0); // Atomic pattern has depth 0
//! ```

use crate::Pattern;

impl<V> Pattern<V> {
    /// Extracts the decorative value at the current position.
    ///
    /// In Pattern's "decorated sequence" semantics, the value provides information
    /// ABOUT the elements (the actual content). This operation accesses that decorative
    /// information.
    ///
    /// # Returns
    ///
    /// A reference to the value field (the decoration).
    ///
    /// # Complexity
    ///
    /// Time: O(1), Space: O(1)
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let p = Pattern::point(42);
    /// assert_eq!(p.extract(), &42);
    ///
    /// let p = Pattern::pattern("root", vec![
    ///     Pattern::point("a"),
    ///     Pattern::point("b")
    /// ]);
    /// assert_eq!(p.extract(), &"root");
    /// ```
    #[inline]
    pub fn extract(&self) -> &V {
        &self.value
    }

    /// Computes new decorative information at each position based on subpattern context.
    ///
    /// This is the key Comonad operation. It takes a context-aware function that receives
    /// the full subpattern at each position and computes new decorative information.
    ///
    /// The function `f` is called with the entire subpattern (not just the value), enabling
    /// context-aware computation of new decorations.
    ///
    /// # Type Parameters
    ///
    /// - `W`: The type of new decorative values
    /// - `F`: The function type (must be `Fn(&Pattern<V>) -> W`)
    ///
    /// # Arguments
    ///
    /// - `f`: Context-aware function that computes new decoration from subpattern
    ///
    /// # Returns
    ///
    /// A new pattern with the same structure, but decorated with computed values.
    ///
    /// # Complexity
    ///
    /// Time: O(n) where n = node count, Space: O(n)
    ///
    /// # Examples
    ///
    /// ```
    /// use pattern_core::Pattern;
    ///
    /// let p = Pattern::pattern("root", vec![
    ///     Pattern::pattern("a", vec![Pattern::point("x")]),
    ///     Pattern::point("b")
    /// ]);
    ///
    /// // Decorate each position with its depth
    /// let depths = p.extend(&|subpattern| subpattern.depth());
    /// assert_eq!(depths.extract(), &2); // root has depth 2
    /// assert_eq!(depths.elements()[0].extract(), &1); // "a" has depth 1
    /// ```
    ///
    /// # Comonad Laws
    ///
    /// This operation satisfies the Comonad laws:
    ///
    /// 1. **Left Identity**: `extract(extend(f, p)) == f(p)`
    /// 2. **Right Identity**: `extend(extract, p) == p`
    /// 3. **Associativity**: `extend(f, extend(g, p)) == extend(f ∘ extend(g), p)`
    pub fn extend<W, F>(&self, f: &F) -> Pattern<W>
    where
        F: Fn(&Pattern<V>) -> W,
    {
        Pattern {
            value: f(self),
            elements: self.elements.iter().map(|elem| elem.extend(f)).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_returns_value() {
        let p = Pattern::point(42);
        assert_eq!(p.extract(), &42);
    }

    #[test]
    fn extend_applies_function_at_all_positions() {
        let p = Pattern::pattern(1, vec![Pattern::point(2), Pattern::point(3)]);

        // Count nodes at each position
        let sizes = p.extend(&|subp: &Pattern<i32>| subp.size());
        assert_eq!(*sizes.extract(), 3); // root has 3 nodes
        assert_eq!(*sizes.elements()[0].extract(), 1); // first child has 1 node
        assert_eq!(*sizes.elements()[1].extract(), 1); // second child has 1 node
    }
}
