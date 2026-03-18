# Pattern Core Examples

Examples demonstrating the pattern-core library functionality.

## Prerequisites

The pattern-core crate is the foundation for pattern data structures in pattern-rs.

## Examples

### `paramorphism_usage.rs`

Comprehensive demonstration of paramorphism (structure-aware folding) on patterns.

**Run:**
```bash
cargo run --package pattern-core --example paramorphism_usage
```

**What it demonstrates:**

1. **Basic Sum** - Para can do everything fold can do
2. **Depth-Weighted Computation** - Access pattern structure during folding
3. **Element-Count-Aware Aggregation** - Compute based on number of elements
4. **Nesting Statistics** - Calculate multiple statistics in one traversal
5. **Structure-Preserving Transformation** - Transform while maintaining structure

**Key Concepts:**
- `para` provides access to both pattern structure and element results
- Bottom-up evaluation (elements processed first, then parent)
- Single traversal with O(n) time complexity
- More powerful than `fold` for structure-aware computations


### `comonad_usage.rs`

Comprehensive demonstration of comonad operations on patterns.

**Run:**
```bash
cargo run --package pattern-core --example comonad_usage
```

**What it demonstrates:**

1. **Basic Comonad Operations**
   - `extract()` - Get the value at the current focus
   - `extend()` - Apply a function to all contexts

2. **Helper Functions**
   - `depth()` - Calculate depth at each node
   - `size()` - Count nodes in each subtree
   - `path()` - Compute path from root

3. **Traversal Operations**
   - `collect_all()` - Gather all values
   - `collect_leaves()` - Get leaf values only
   - `find()` - Search for specific values

4. **Transformation Operations**
   - `map_with_context()` - Transform using context
   - `annotate_depth()` - Add depth annotations
   - `enrich_with_metadata()` - Add computed metadata

5. **Advanced Examples**
   - Tree analysis
   - Hierarchical transformations
   - Context-aware computations
   - Zipper-like navigation

## Key Concepts

### Comonad

A comonad provides:
- **extract**: `W a → a` - Get the value at focus
- **extend**: `(W a → b) → W a → W b` - Apply function to all contexts

For patterns, this allows:
- Inspecting each node with full context
- Computing values based on neighborhood
- Hierarchical transformations
- Tree analysis operations

### Pattern Structure

```rust
pub struct Pattern<V> {
    pub value: V,
    pub elements: Vec<Pattern<V>>,
}
```

Patterns are recursive tree structures where:
- Each node has a value of type `V`
- Each node can have child patterns (elements)
- Leaf patterns have empty elements vector

## Common Use Cases

### 1. Tree Analysis

```rust
use pattern_core::Pattern;

// Calculate depth of each node
let depths = pattern.extend(|p| p.depth());

// Count descendants
let sizes = pattern.extend(|p| p.size());
```

### 2. Context-Aware Transformation

```rust
// Transform each node using its context
let enriched = pattern.map_with_context(|p| {
    format!("{} (depth: {})", p.extract(), p.depth())
});
```

### 3. Hierarchical Navigation

```rust
// Find path to a specific value
if let Some(path) = pattern.find(&target_value) {
    println!("Found at: {:?}", path);
}
```

### 4. Metadata Annotation

```rust
// Add metadata to each node
let annotated = pattern.enrich_with_metadata(|p| {
    Metadata {
        depth: p.depth(),
        size: p.size(),
        path: p.path(),
    }
});
```

## Performance

Comonad operations are:
- **Time**: O(n) for single traversal, O(n²) for extend with expensive functions
- **Space**: O(n) for result tree, O(d) for recursion stack (d = depth)
- **Laziness**: Operations are eager (not lazy) in Rust

For large trees:
- Use iterative approaches when possible
- Consider memoization for expensive extend functions
- Profile before optimizing

## Testing

Run the example to see all operations in action:

```bash
cargo run --package pattern-core --example comonad_usage
```

Expected output shows:
- Basic comonad operations (extract, extend)
- Helper function results (depth, size, path)
- Traversal outputs (collect_all, collect_leaves, find)
- Transformation results (map_with_context, annotations)
- Advanced examples with detailed trees

## Next Steps

- Read the comonad implementation: `crates/pattern-core/src/comonad.rs`
- Check the tests: `crates/pattern-core/tests/comonad_tests.rs`
- Review the spec: `specs/018-comonad-instance/spec.md`
- Try modifying the example to explore different use cases

## References

- **Comonad Laws**: See `specs/018-comonad-instance/spec.md`
- **Pattern Structure**: See `crates/pattern-core/src/pattern.rs`
- **API Documentation**: Run `cargo doc --package pattern-core --open`
