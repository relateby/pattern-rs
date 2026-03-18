//! Comprehensive example demonstrating Comonad operations on Pattern.
//!
//! This example showcases:
//! - Basic extract and extend operations
//! - Helper functions (depth_at, size_at, indices_at)
//! - Practical use cases (visualization, analysis)
//! - Composition with existing Pattern operations
//!
//! Run with: `cargo run --example comonad_usage`

use pattern_core::Pattern;

fn main() {
    println!("=== Comonad Operations Example ===\n");

    // Example 1: Basic extract and extend
    basic_extract_extend();

    // Example 2: Helper functions
    helper_functions();

    // Example 3: Practical use case - Pattern inspector
    pattern_inspector();

    // Example 4: Custom context-aware computation
    custom_computation();

    // Example 5: Composition with existing operations
    composition_example();
}

/// Demonstrates basic extract and extend operations.
fn basic_extract_extend() {
    println!("--- Example 1: Basic Extract and Extend ---");

    // Create a nested pattern
    let p = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern("a", vec![Pattern::point("x")]),
            Pattern::point("b"),
        ],
    );

    // Extract: Get the decorative value
    println!("Root value: {}", p.extract());

    // Extend: Compute depth at each position
    let depths = p.extend(&|subp: &Pattern<&str>| subp.depth());
    println!("Root depth: {}", depths.extract());
    println!("Child 'a' depth: {}", depths.elements()[0].extract());
    println!("Child 'b' depth: {}", depths.elements()[1].extract());
    println!(
        "Nested child 'x' depth: {}\n",
        depths.elements()[0].elements()[0].extract()
    );
}

/// Demonstrates helper functions: depth_at, size_at, indices_at.
fn helper_functions() {
    println!("--- Example 2: Helper Functions ---");

    let p = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern("a", vec![Pattern::point("x"), Pattern::point("y")]),
            Pattern::pattern("b", vec![Pattern::point("z")]),
            Pattern::point("c"),
        ],
    );

    // Depth at each position
    let depths = p.depth_at();
    println!("Depths:");
    println!("  root: {}", depths.extract());
    println!("  a: {}", depths.elements()[0].extract());
    println!("  b: {}", depths.elements()[1].extract());
    println!("  c: {}", depths.elements()[2].extract());

    // Size at each position
    let sizes = p.size_at();
    println!("\nSizes (node counts):");
    println!("  root: {}", sizes.extract());
    println!("  a subtree: {}", sizes.elements()[0].extract());
    println!("  b subtree: {}", sizes.elements()[1].extract());
    println!("  c: {}", sizes.elements()[2].extract());

    // Indices at each position
    let paths = p.indices_at();
    println!("\nPaths from root:");
    println!("  root: {:?}", paths.extract());
    println!("  a: {:?}", paths.elements()[0].extract());
    println!(
        "  a's first child: {:?}",
        paths.elements()[0].elements()[0].extract()
    );
    println!("  b: {:?}", paths.elements()[1].extract());
    println!("  c: {:?}\n", paths.elements()[2].extract());
}

/// Practical use case: Pattern structure inspector.
fn pattern_inspector() {
    println!("--- Example 3: Pattern Inspector ---");

    // Create a more complex pattern
    let p = Pattern::pattern(
        "document",
        vec![
            Pattern::pattern(
                "section1",
                vec![
                    Pattern::pattern("paragraph1", vec![Pattern::point("text1")]),
                    Pattern::point("paragraph2"),
                ],
            ),
            Pattern::pattern(
                "section2",
                vec![
                    Pattern::point("paragraph3"),
                    Pattern::pattern(
                        "subsection",
                        vec![Pattern::point("paragraph4"), Pattern::point("paragraph5")],
                    ),
                ],
            ),
            Pattern::point("footer"),
        ],
    );

    // Compute structural metrics
    let depths = p.depth_at();
    let sizes = p.size_at();
    let paths = p.indices_at();

    // Find maximum depth
    let max_depth = depths.fold(0, |max, &d| max.max(d));
    println!("Maximum nesting depth: {}", max_depth);

    // Find total node count
    let total_nodes = p.size();
    println!("Total nodes: {}", total_nodes);

    // Find heavy subtrees (> 30% of total)
    let threshold = (total_nodes as f64 * 0.3) as usize;
    println!("\nHeavy subtrees (> 30% of total = {}+):", threshold);
    find_heavy_subtrees(&sizes, &paths, threshold, &[]);

    println!();
}

/// Helper function to find heavy subtrees.
fn find_heavy_subtrees(
    sizes: &Pattern<usize>,
    paths: &Pattern<Vec<usize>>,
    threshold: usize,
    current_path: &[usize],
) {
    if *sizes.extract() > threshold {
        println!("  Path {:?}: {} nodes", paths.extract(), sizes.extract());
    }

    for (i, (size_child, path_child)) in sizes
        .elements()
        .iter()
        .zip(paths.elements().iter())
        .enumerate()
    {
        let mut new_path = current_path.to_vec();
        new_path.push(i);
        find_heavy_subtrees(size_child, path_child, threshold, &new_path);
    }
}

/// Custom context-aware computation: balance factor.
fn custom_computation() {
    println!("--- Example 4: Custom Context-Aware Computation ---");

    let p = Pattern::pattern(
        "root",
        vec![
            Pattern::point("a"),
            Pattern::point("b"),
            Pattern::point("c"),
        ],
    );

    // Compute balance factor at each position
    // (How evenly distributed are the child sizes?)
    let balances = p.extend(&|subp: &Pattern<&str>| {
        if subp.elements().is_empty() {
            1.0 // Atomic patterns are perfectly balanced
        } else {
            let sizes: Vec<usize> = subp.elements().iter().map(|e| e.size()).collect();

            let avg = sizes.iter().sum::<usize>() as f64 / sizes.len() as f64;
            let variance =
                sizes.iter().map(|&s| (s as f64 - avg).powi(2)).sum::<f64>() / sizes.len() as f64;

            1.0 / (1.0 + variance) // Higher value = more balanced
        }
    });

    println!("Balance factors (1.0 = perfectly balanced):");
    println!("  root: {:.3}", balances.extract());
    println!();
}

/// Composition with existing Pattern operations.
fn composition_example() {
    println!("--- Example 5: Composition with Existing Operations ---");

    let p = Pattern::pattern(
        "root",
        vec![
            Pattern::pattern("a", vec![Pattern::point("x")]),
            Pattern::pattern(
                "b",
                vec![
                    Pattern::pattern("y", vec![Pattern::point("z")]),
                    Pattern::point("w"),
                ],
            ),
            Pattern::point("c"),
        ],
    );

    // Compute total depth across all positions (using fold)
    let total_depth: usize = p.depth_at().fold(0, |acc, &d| acc + d);
    println!("Sum of all depths: {}", total_depth);

    // Find positions with depth > 1 (using filter and map)
    let deep_positions = p.depth_at();
    let positions_with_depth_gt_1 = count_positions_with_depth(&deep_positions, 1);
    println!(
        "Number of positions with depth > 1: {}",
        positions_with_depth_gt_1
    );

    // Map over decorated pattern
    let depth_labels = p.depth_at().map(|d| format!("depth={}", d));
    println!("\nDepth labels:\n  root: {}", depth_labels.extract());
    println!("  first child: {}", depth_labels.elements()[0].extract());

    // Combine multiple metrics in one pass
    #[derive(Debug, Clone)]
    #[allow(dead_code)]
    struct Metrics {
        depth: usize,
        size: usize,
        balance: f64,
    }

    let metrics = p.extend(&|subp: &Pattern<&str>| {
        let sizes: Vec<usize> = subp.elements().iter().map(|e| e.size()).collect();
        let balance = if sizes.is_empty() {
            1.0
        } else {
            let avg = sizes.iter().sum::<usize>() as f64 / sizes.len() as f64;
            let variance =
                sizes.iter().map(|&s| (s as f64 - avg).powi(2)).sum::<f64>() / sizes.len() as f64;
            1.0 / (1.0 + variance)
        };

        Metrics {
            depth: subp.depth(),
            size: subp.size(),
            balance,
        }
    });

    println!("\nCombined metrics at root:\n  {:?}\n", metrics.extract());
}

/// Helper to count positions with depth greater than threshold.
fn count_positions_with_depth(depths: &Pattern<usize>, threshold: usize) -> usize {
    let mut count = 0;
    if *depths.extract() > threshold {
        count += 1;
    }
    for child in depths.elements() {
        count += count_positions_with_depth(child, threshold);
    }
    count
}
