# Quick Start: Comonad Operations on Pattern

**Feature**: 018-comonad-instance  
**Date**: 2026-01-05

## What is this?

Comonad operations allow you to work with Pattern's "decorated sequence" semantics:
- **Elements ARE the pattern** (e.g., `["A", "B", "A"]`)
- **Value DECORATES the elements** (e.g., `"sonata"` provides information about the pattern)

Comonad operations let you:
1. **Extract** decorative information at any position
2. **Compute** new decorative information based on context (the subpattern)

## Installation

Already included in `pattern-core` crate (version TBD).

```toml
[dependencies]
pattern-core = "0.1"  # Version TBD
```

## Basic Usage

### Extract: Get the Decoration

```rust
use pattern_core::Pattern;

let p = Pattern {
    value: "sonata",
    elements: vec![
        Pattern { value: "A", elements: vec![] },
        Pattern { value: "B", elements: vec![] },
        Pattern { value: "A", elements: vec![] },
    ]
};

// Extract the decorative value
let decoration = p.extract();
assert_eq!(decoration, &"sonata");
```

### Extend: Compute New Decorations

```rust
use pattern_core::Pattern;

let p = Pattern {
    value: "root",
    elements: vec![
        Pattern {
            value: "a",
            elements: vec![
                Pattern { value: "x", elements: vec![] }
            ]
        },
        Pattern { value: "b", elements: vec![] },
    ]
};

// Decorate each position with its depth
let depths = p.extend(&|subpattern| subpattern.depth());

// Result:
// Pattern {
//     value: 2,              // root has depth 2 (max nesting is 2 levels)
//     elements: [
//         Pattern {
//             value: 1,      // "a" has depth 1 (one level of nesting)
//             elements: [
//                 Pattern { value: 0, elements: [] }  // "x" is atomic
//             ]
//         },
//         Pattern { value: 0, elements: [] }  // "b" is atomic
//     ]
// }

assert_eq!(depths.extract(), &2);
assert_eq!(depths.elements[0].extract(), &1);
assert_eq!(depths.elements[0].elements[0].extract(), &0);
assert_eq!(depths.elements[1].extract(), &0);
```

## Helper Functions

### Depth at Each Position

```rust
let p = Pattern {
    value: "root",
    elements: vec![
        Pattern {
            value: "a",
            elements: vec![point("x")]
        },
        point("b")
    ]
};

let depths = p.depth_at();
println!("Root depth: {}", depths.extract());  // 2
println!("First child depth: {}", depths.elements[0].extract());  // 1
```

**Use cases**: Visualizing nesting complexity, identifying deeply nested areas

### Size at Each Position

```rust
let p = Pattern {
    value: "root",
    elements: vec![
        Pattern {
            value: "a",
            elements: vec![point("x")]
        },
        point("b")
    ]
};

let sizes = p.size_at();
println!("Root size: {}", sizes.extract());  // 4 (total nodes)
println!("First child size: {}", sizes.elements[0].extract());  // 2
```

**Use cases**: Finding large subtrees, understanding pattern distribution

### Path to Each Position

```rust
let p = Pattern {
    value: "root",
    elements: vec![
        Pattern {
            value: "a",
            elements: vec![point("x")]
        },
        point("b")
    ]
};

let paths = p.indices_at();
println!("Root path: {:?}", paths.extract());  // []
println!("First child path: {:?}", paths.elements[0].extract());  // [0]
println!("Nested child path: {:?}", paths.elements[0].elements[0].extract());  // [0, 0]
println!("Second child path: {:?}", paths.elements[1].extract());  // [1]
```

**Use cases**: Addressing specific positions, navigation, generating links

## Common Patterns

### Pattern 1: Annotate for Visualization

```rust
use pattern_core::Pattern;

#[derive(Debug, Clone)]
struct VisualMetadata {
    depth: usize,
    size: usize,
    label: String,
}

fn annotate_for_viz(p: &Pattern<String>) -> Pattern<VisualMetadata> {
    // Get structural metadata
    let depths = p.depth_at();
    let sizes = p.size_at();
    
    // Combine using extend
    p.extend(&|subp| {
        // Find corresponding depth and size
        // (In practice, you'd navigate to the matching position)
        VisualMetadata {
            depth: subp.depth(),
            size: subp.size(),
            label: subp.extract().clone(),
        }
    })
}

// Usage
let p = Pattern { value: "root".to_string(), elements: vec![point("a".to_string())] };
let annotated = annotate_for_viz(&p);
println!("{:?}", annotated.extract());  // VisualMetadata with depth, size, label
```

### Pattern 2: Find Heavy Subtrees

```rust
fn find_heavy_subtrees(p: &Pattern<String>, threshold: usize) -> Vec<Vec<usize>> {
    let sizes = p.size_at();
    let paths = p.indices_at();
    
    let mut heavy = Vec::new();
    
    // Traverse both patterns together
    fn collect<V>(
        sizes: &Pattern<usize>,
        paths: &Pattern<Vec<usize>>,
        threshold: usize,
        result: &mut Vec<Vec<usize>>
    ) {
        if *sizes.extract() > threshold {
            result.push(paths.extract().clone());
        }
        for (size_child, path_child) in sizes.elements.iter().zip(paths.elements.iter()) {
            collect(size_child, path_child, threshold, result);
        }
    }
    
    collect(&sizes, &paths, threshold, &mut heavy);
    heavy
}

// Usage
let p = /* some pattern */;
let heavy_paths = find_heavy_subtrees(&p, 10);
println!("Heavy subtrees at: {:?}", heavy_paths);
```

### Pattern 3: Custom Context-Aware Metrics

```rust
// Compute balance factor (how evenly distributed children are)
fn balance_factors(p: &Pattern<String>) -> Pattern<f64> {
    p.extend(&|subp| {
        if subp.elements.is_empty() {
            1.0  // Atomic patterns are perfectly balanced
        } else {
            let sizes: Vec<usize> = subp.elements.iter()
                .map(|e| e.size())
                .collect();
            
            if sizes.is_empty() {
                return 1.0;
            }
            
            let avg = sizes.iter().sum::<usize>() as f64 / sizes.len() as f64;
            let variance = sizes.iter()
                .map(|&s| (s as f64 - avg).powi(2))
                .sum::<f64>() / sizes.len() as f64;
            
            1.0 / (1.0 + variance)  // Higher value = more balanced
        }
    })
}

// Usage
let p = /* some pattern */;
let balances = balance_factors(&p);
println!("Balance at root: {}", balances.extract());
```

### Pattern 4: Compose with Existing Operations

```rust
// Compute total depth across all positions
let total_depth: usize = pattern
    .depth_at()                    // Pattern<usize>
    .fold(0, |acc, d| acc + d);    // Fold over decorations

// Find positions with depth > 5
let deep_positions = pattern
    .depth_at()                    // Pattern<usize>
    .filter(|p| *p.extract() > 5); // Filter by decoration

// Map over decorated pattern
let depth_labels = pattern
    .depth_at()                    // Pattern<usize>
    .map(|d| format!("depth {}", d));  // Pattern<String>
```

## Real-World Example: Pattern Inspector

```rust
use pattern_core::Pattern;

struct InspectorReport {
    total_nodes: usize,
    max_depth: usize,
    max_size: usize,
    heavy_subtrees: Vec<Vec<usize>>,  // Paths to subtrees > 50% of total
}

fn inspect(p: &Pattern<String>) -> InspectorReport {
    let total = p.size();
    let depths = p.depth_at();
    let sizes = p.size_at();
    let paths = p.indices_at();
    
    // Find max depth
    let max_depth = depths.fold(0, |max, &d| max.max(d));
    
    // Find max size
    let max_size = sizes.fold(0, |max, &s| max.max(s));
    
    // Find heavy subtrees (> 50% of total)
    let threshold = total / 2;
    let mut heavy = Vec::new();
    
    fn collect_heavy(
        sizes: &Pattern<usize>,
        paths: &Pattern<Vec<usize>>,
        threshold: usize,
        result: &mut Vec<Vec<usize>>
    ) {
        if *sizes.extract() > threshold {
            result.push(paths.extract().clone());
        }
        for (s, p) in sizes.elements.iter().zip(paths.elements.iter()) {
            collect_heavy(s, p, threshold, result);
        }
    }
    
    collect_heavy(&sizes, &paths, threshold, &mut heavy);
    
    InspectorReport {
        total_nodes: total,
        max_depth,
        max_size,
        heavy_subtrees: heavy,
    }
}

// Usage
let pattern = /* load pattern */;
let report = inspect(&pattern);
println!("Pattern has {} nodes", report.total_nodes);
println!("Maximum depth: {}", report.max_depth);
println!("Maximum subtree size: {}", report.max_size);
println!("Heavy subtrees: {:?}", report.heavy_subtrees);
```

## Understanding "Decorated Sequences"

**Key Concept**: In Pattern, the elements ARE the content, the value IS decoration.

```rust
Pattern {
    value: "sonata",           // ← Decoration (information ABOUT the pattern)
    elements: ["A", "B", "A"]  // ← Content (the pattern itself)
}
```

This is different from typical tree structures where values are content:

```rust
// Typical tree: values are content
Tree {
    value: "some data",        // ← Content
    children: [...]            // ← More content
}

// Pattern: value decorates elements
Pattern {
    value: "metadata",         // ← Information about elements
    elements: [...]            // ← The actual content
}
```

**Why Comonad?** Comonad operations naturally work with this "decoration-as-information" model:
- `extract`: Access the decorative information
- `extend`: Compute new decorative information based on context

## API Cheat Sheet

```rust
// Extract decoration
let decoration: &V = pattern.extract();

// Compute new decorations based on context
let decorated: Pattern<W> = pattern.extend(&|subpattern| {
    // Function receives full subpattern, computes new decoration
    compute_decoration(subpattern)
});

// Helper: Depth at each position
let depths: Pattern<usize> = pattern.depth_at();

// Helper: Size at each position
let sizes: Pattern<usize> = pattern.size_at();

// Helper: Path to each position
let paths: Pattern<Vec<usize>> = pattern.indices_at();

// Compose with existing operations
pattern
    .depth_at()                      // Decorate with depths
    .map(|d| format!("depth {}", d)) // Transform decorations
    .fold(String::new(), |acc, s| acc + &s);  // Aggregate
```

## Performance Tips

1. **Single pass is efficient**: `depth_at()`, `size_at()`, `indices_at()` are all O(n) single-pass operations
2. **Avoid repeated decoration**: If you need multiple metrics, compute them in one `extend` call
3. **Fold for aggregation**: Use `fold` to aggregate decorated patterns instead of manual iteration

```rust
// GOOD: Compute multiple metrics in one pass
let annotated = pattern.extend(&|p| {
    VisualMetadata {
        depth: p.depth(),
        size: p.size(),
        // ... other metrics
    }
});

// AVOID: Multiple passes
let depths = pattern.depth_at();
let sizes = pattern.size_at();
let paths = pattern.indices_at();
// Now you have to zip these together manually
```

## Troubleshooting

### "Cannot move out of `pattern`"

**Problem**: Trying to use pattern after consuming it.

**Solution**: Use `&pattern` or clone if needed.

```rust
// BAD
let depths = pattern.depth_at();  // Consumes if method takes self
let sizes = pattern.size_at();    // Error: pattern moved

// GOOD
let depths = pattern.depth_at();  // If method takes &self
let sizes = pattern.size_at();    // OK
```

### "Function might not be pure"

**Problem**: Function passed to `extend` has side effects.

**Solution**: Comonad operations require pure functions. Move side effects outside.

```rust
// BAD
let mut count = 0;
let result = pattern.extend(&|p| {
    count += 1;  // Side effect!
    p.depth()
});

// GOOD
let result = pattern.extend(&|p| p.depth());
let count = result.size();  // Count afterward
```

### "Performance is slow"

**Check**:
1. Are you calling `depth_at()` or `size_at()` repeatedly? Cache the result.
2. Is pattern extremely large (100k+ nodes)? Consider streaming or chunking.
3. Is the function passed to `extend` expensive? Profile and optimize.

## Next Steps

- **Read the spec**: [spec.md](./spec.md) for full requirements
- **See API contracts**: [contracts/comonad.md](./contracts/comonad.md) for detailed specifications
- **Understand the model**: [data-model.md](./data-model.md) for in-depth explanation
- **Check examples**: Look in `crates/pattern-core/examples/` (when available)

## Questions?

- **What is a Comonad?** A mathematical abstraction for context-aware computation. In Pattern, it means computing decorations based on subpattern context.
- **Why not just use map?** `map` only sees individual values. `extend` sees the entire subpattern at each position.
- **Do I need to understand category theory?** No! Think of it as "compute decoration based on what's here."
- **Is this the same as Functor?** No. Functor (`map`) transforms values. Comonad (`extend`) computes new values from context.

## References

- **Feature Spec**: [spec.md](./spec.md)
- **API Contracts**: [contracts/comonad.md](./contracts/comonad.md)
- **Data Model**: [data-model.md](./data-model.md)
- **Haskell Reference**: `../pattern-hs/libs/pattern/src/Pattern/Core.hs`
