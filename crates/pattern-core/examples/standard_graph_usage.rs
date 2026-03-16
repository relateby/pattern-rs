//! StandardGraph usage examples
//!
//! Demonstrates the ergonomic StandardGraph API for building and querying graphs.
//!
//! Run with:
//! ```bash
//! cargo run --package relateby-pattern --example standard_graph_usage
//! ```

use pattern_core::graph::StandardGraph;
use pattern_core::subject::Subject;
use pattern_core::Pattern;

fn main() {
    println!("=== StandardGraph Usage Examples ===\n");

    // -----------------------------------------------------------------------
    // 1. Fluent Subject construction
    // -----------------------------------------------------------------------
    println!("--- 1. SubjectBuilder ---");

    let alice = Subject::build("alice")
        .label("Person")
        .property("name", "Alice Smith")
        .property("age", 30i64)
        .property("active", true)
        .done();

    println!("Identity: {}", alice.identity);
    println!("Labels:   {:?}", alice.labels);
    println!("Props:    {:?}", alice.properties);
    println!();

    // -----------------------------------------------------------------------
    // 2. Build a graph element by element
    // -----------------------------------------------------------------------
    println!("--- 2. Element-by-element construction ---");

    let mut g = StandardGraph::new();

    // Add nodes — keep subjects in scope to use as endpoint references
    let alice = Subject::build("alice")
        .label("Person")
        .property("name", "Alice")
        .done();
    let bob = Subject::build("bob")
        .label("Person")
        .property("name", "Bob")
        .done();
    let carol = Subject::build("carol")
        .label("Person")
        .property("name", "Carol")
        .done();
    g.add_node(alice.clone());
    g.add_node(bob.clone());
    g.add_node(carol.clone());

    // Add relationships — pass Subject objects directly (chaining works too)
    g.add_relationship(
        Subject::build("r1")
            .label("KNOWS")
            .property("since", 2020i64)
            .done(),
        &alice,
        &bob,
    )
    .add_relationship(Subject::build("r2").label("KNOWS").done(), &bob, &carol);

    println!("Nodes:         {}", g.node_count());
    println!("Relationships: {}", g.relationship_count());
    println!("Empty?         {}", g.is_empty());
    println!();

    // -----------------------------------------------------------------------
    // 3. Query the graph
    // -----------------------------------------------------------------------
    println!("--- 3. Querying ---");

    // Element access by identity
    if let Some(node) = g.node(&"alice".into()) {
        println!("Found alice: labels={:?}", node.value.labels);
    }

    // Source and target of a relationship
    if let Some(src) = g.source(&"r1".into()) {
        println!("r1 source: {}", src.value.identity);
    }
    if let Some(tgt) = g.target(&"r1".into()) {
        println!("r1 target: {}", tgt.value.identity);
    }

    // Neighbors (bidirectional)
    let bob_neighbors = g.neighbors(&"bob".into());
    let neighbor_names: Vec<&str> = bob_neighbors
        .iter()
        .map(|n| n.value.identity.0.as_str())
        .collect();
    println!("Bob's neighbors: {:?}", neighbor_names);
    println!("Bob's degree:    {}", g.degree(&"bob".into()));
    println!();

    // -----------------------------------------------------------------------
    // 4. Iterate over elements
    // -----------------------------------------------------------------------
    println!("--- 4. Iteration ---");

    println!("All nodes:");
    for (id, _node) in g.nodes() {
        println!("  - {}", id);
    }

    println!("All relationships:");
    for (id, rel) in g.relationships() {
        let src = rel.elements[0].value.identity.0.as_str();
        let tgt = rel.elements[1].value.identity.0.as_str();
        println!("  - {} ({} -> {})", id, src, tgt);
    }
    println!();

    // -----------------------------------------------------------------------
    // 5. Pattern ingestion (classify automatically)
    // -----------------------------------------------------------------------
    println!("--- 5. Pattern ingestion ---");

    let mut g2 = StandardGraph::new();
    // Add raw patterns — they get classified by shape
    g2.add_pattern(Pattern::point(
        Subject::build("x").label("Standalone").done(),
    ));
    g2.add_pattern(Pattern::pattern(
        Subject::build("e1").label("EDGE").done(),
        vec![
            Pattern::point(Subject::build("x").done()),
            Pattern::point(Subject::build("y").done()),
        ],
    ));

    println!("g2 nodes:         {}", g2.node_count());
    println!("g2 relationships: {}", g2.relationship_count());
    println!();

    // -----------------------------------------------------------------------
    // 6. Escape hatches — interop with abstract types
    // -----------------------------------------------------------------------
    println!("--- 6. Escape hatches ---");

    // Get the underlying PatternGraph
    let pg = g.as_pattern_graph();
    println!("PatternGraph nodes: {}", pg.pg_nodes.len());

    // Create a GraphQuery for algorithm use
    let query = g.as_query();
    let all_nodes = (query.query_nodes)();
    println!("GraphQuery nodes:   {}", all_nodes.len());

    // Create a GraphView snapshot
    let snapshot = g.as_snapshot();
    println!("GraphView elements: {}", snapshot.view_elements.len());
    println!();

    // -----------------------------------------------------------------------
    // 7. Placeholder auto-creation
    // -----------------------------------------------------------------------
    println!("--- 7. Placeholder auto-creation ---");

    let mut g3 = StandardGraph::new();
    // Add a relationship without pre-creating nodes — placeholders are created.
    // Use Subject::from_id when you only have identity strings.
    g3.add_relationship(
        Subject::build("r1").label("LINKS").done(),
        &Subject::from_id("foo"),
        &Subject::from_id("bar"),
    );

    println!("Nodes auto-created: {}", g3.node_count());
    println!("foo exists? {}", g3.node(&"foo".into()).is_some());

    println!("\n=== Done ===");
}
