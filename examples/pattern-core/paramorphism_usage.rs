//! Paramorphism: Structure-Aware Folding Examples
//!
//! This example demonstrates the `para` method, which provides structure-aware
//! folding over patterns. Unlike `fold` (which only sees values), paramorphism
//! gives the folding function access to the full pattern structure at each position.
//!
//! Run with: cargo run --example paramorphism_usage

use pattern_core::Pattern;

fn main() {
    println!("=== Paramorphism Examples ===\n");

    // Example 1: Basic sum (equivalent to fold)
    basic_sum();

    // Example 2: Depth-weighted computation
    depth_weighted_sum();

    // Example 3: Element-count-aware aggregation
    element_count_aggregation();

    // Example 4: Computing multiple statistics in one pass
    nesting_statistics();

    // Example 5: Structure-preserving transformation
    structure_preserving_transformation();
}

/// Example 1: Basic sum - para can do everything fold can do
fn basic_sum() {
    println!("1. Basic Sum (equivalent to fold)");
    println!("   Pattern: 10 with elements [5, 3]");

    let p = Pattern::pattern(10, vec![Pattern::point(5), Pattern::point(3)]);

    // Using para
    let para_sum: i32 = p.para(|pat, rs| *pat.value() + rs.iter().sum::<i32>());

    // Using fold for comparison
    let fold_sum: i32 = p.fold(0, |acc, v| acc + v);

    println!("   Para sum: {}", para_sum);
    println!("   Fold sum: {}", fold_sum);
    println!("   Both produce: 18 (10 + 5 + 3)\n");

    assert_eq!(para_sum, fold_sum);
}

/// Example 2: Depth-weighted sum - para can access structure
fn depth_weighted_sum() {
    println!("2. Depth-Weighted Sum");
    println!("   Pattern: 10 with elements [5, 3]");
    println!("   Formula: value * depth + sum(element_results)");

    let p = Pattern::pattern(10, vec![Pattern::point(5), Pattern::point(3)]);

    let depth_weighted: i32 = p.para(|pat, rs| {
        let value = *pat.value();
        let depth = pat.depth() as i32;
        let element_sum: i32 = rs.iter().sum();
        value * depth + element_sum
    });

    println!("   Root (depth=1): 10 * 1 = 10");
    println!("   Leaves (depth=0): 5 * 0 = 0, 3 * 0 = 0");
    println!("   Total: {}\n", depth_weighted);

    assert_eq!(depth_weighted, 10);
}

/// Example 3: Element-count-aware aggregation
fn element_count_aggregation() {
    println!("3. Element-Count-Aware Aggregation");
    println!("   Pattern: 10 with elements [pattern(5, [2]), 3]");
    println!("   Formula: value * element_count + sum(element_results)");

    let p = Pattern::pattern(
        10,
        vec![Pattern::pattern(5, vec![Pattern::point(2)]), Pattern::point(3)],
    );

    let result: i32 = p.para(|pat, rs| {
        let value = *pat.value();
        let elem_count = pat.elements().len() as i32;
        let element_sum: i32 = rs.iter().sum();
        value * elem_count + element_sum
    });

    println!("   Root: 10 * 2 + (5 + 0) = 25");
    println!("   Middle: 5 * 1 + 0 = 5");
    println!("   Leaves: 2 * 0 = 0, 3 * 0 = 0");
    println!("   Total: {}\n", result);

    assert_eq!(result, 25);
}

/// Example 4: Computing multiple statistics in one traversal
fn nesting_statistics() {
    println!("4. Nesting Statistics (sum, count, max_depth)");
    println!("   Pattern: 1 with [pattern(2, [3]), 4]");

    let p = Pattern::pattern(
        1,
        vec![Pattern::pattern(2, vec![Pattern::point(3)]), Pattern::point(4)],
    );

    type Stats = (i32, usize, usize); // (sum, count, max_depth)

    let (sum, count, max_depth): Stats = p.para(|pat, rs: &[Stats]| {
        let value = *pat.value();
        let depth = pat.depth();

        // Aggregate from element results
        let (child_sum, child_count, child_max_depth) = rs
            .iter()
            .fold((0_i32, 0_usize, 0_usize), |(s, c, d), (s2, c2, d2)| {
                (s + s2, c + c2, d.max(*d2))
            });

        (value + child_sum, 1 + child_count, depth.max(child_max_depth))
    });

    println!("   Sum: {} (1 + 2 + 3 + 4)", sum);
    println!("   Count: {} nodes", count);
    println!("   Max depth: {}\n", max_depth);

    assert_eq!(sum, 10);
    assert_eq!(count, 4);
    assert_eq!(max_depth, 2);
}

/// Example 5: Structure-preserving transformation
fn structure_preserving_transformation() {
    println!("5. Structure-Preserving Transformation");
    println!("   Pattern: 1 with [pattern(2, [3])]");
    println!("   Transform: multiply each value by (depth + 1)");

    let p = Pattern::pattern(1, vec![Pattern::pattern(2, vec![Pattern::point(3)])]);

    let transformed: Pattern<i32> = p.para(|pat, rs: &[Pattern<i32>]| {
        let value = *pat.value();
        let depth = pat.depth() as i32;
        let new_value = value * (depth + 1);

        Pattern::pattern(new_value, rs.to_vec())
    });

    println!("   Original values: [1, 2, 3]");
    println!("   Depths: [2, 1, 0]");
    println!("   Transformed values: [3, 4, 3]");
    println!("   Root: 1 * (2+1) = {}", transformed.value());
    println!(
        "   First element: 2 * (1+1) = {}",
        transformed.elements()[0].value()
    );
    println!(
        "   Leaf: 3 * (0+1) = {}\n",
        transformed.elements()[0].elements()[0].value()
    );

    assert_eq!(*transformed.value(), 3);
    assert_eq!(*transformed.elements()[0].value(), 4);
    assert_eq!(*transformed.elements()[0].elements()[0].value(), 3);
}