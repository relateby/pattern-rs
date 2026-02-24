//! unfold: anamorphism for building a Pattern tree from a seed.
//!
//! Ported from `Pattern.Core.unfold` in the Haskell reference implementation.
//! Implemented iteratively with an explicit work stack to avoid stack overflow
//! on deep hierarchies.

use crate::pattern::Pattern;

/// Anamorphism: expand a seed into a `Pattern<V>` tree.
///
/// `expand` receives a seed and returns the value for the current node plus
/// a list of child seeds. The tree is built iteratively (work stack) to avoid
/// stack overflow for arbitrarily deep hierarchies.
///
/// # Examples
///
/// ```rust
/// use pattern_core::unfold;
///
/// // Build a depth-3 binary tree from a depth seed
/// let tree = unfold(|depth: u32| {
///     if depth == 0 {
///         (depth, vec![])
///     } else {
///         (depth, vec![depth - 1, depth - 1])
///     }
/// }, 2u32);
///
/// assert_eq!(tree.value, 2);
/// assert_eq!(tree.elements.len(), 2);
/// assert_eq!(tree.elements[0].value, 1);
/// ```
pub fn unfold<A, V>(expand: impl Fn(A) -> (V, Vec<A>), seed: A) -> Pattern<V> {
    // Two-phase iterative implementation using an explicit work stack.
    // Work::Expand(seed)       — expand the seed, push children then Collect
    // Work::Collect(value, n)  — pop n children from result stack, assemble Pattern

    enum Work<A, V> {
        Expand(A),
        Collect(V, usize),
    }

    let mut work_stack: Vec<Work<A, V>> = vec![Work::Expand(seed)];
    let mut result_stack: Vec<Pattern<V>> = Vec::new();

    while let Some(item) = work_stack.pop() {
        match item {
            Work::Expand(s) => {
                let (value, children) = expand(s);
                let n = children.len();
                // Collect marker runs after all children are assembled
                work_stack.push(Work::Collect(value, n));
                // Push children in reverse so leftmost is processed first (LIFO)
                for child in children.into_iter().rev() {
                    work_stack.push(Work::Expand(child));
                }
            }
            Work::Collect(value, n) => {
                let start = result_stack.len() - n;
                let elements: Vec<Pattern<V>> = result_stack.drain(start..).collect();
                result_stack.push(Pattern { value, elements });
            }
        }
    }

    result_stack
        .pop()
        .expect("unfold: result stack should have exactly one element")
}
