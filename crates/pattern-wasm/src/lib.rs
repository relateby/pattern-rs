mod convert;
mod gram;

// Re-export WASM-compatible types from pattern-core
pub use pattern_core::wasm::{
    ValueFactory as Value, WasmPattern as Pattern, WasmStructureAnalysis as StructureAnalysis,
    WasmSubject as Subject, WasmValidationRules as ValidationRules,
};

// Re-export graph types under Native* names
pub use pattern_core::wasm::{
    WasmGraphQuery as NativeGraphQuery, WasmPatternGraph as NativePatternGraph,
    WasmReconciliationPolicy as NativeReconciliationPolicy,
};

// Re-export graph constant object constructors
pub use pattern_core::wasm::{
    graph_class_constants as GraphClassConstants,
    traversal_direction_constants as TraversalDirectionConstants,
};

// Re-export algorithm free functions
pub use pattern_core::wasm::{
    all_paths, betweenness_centrality, bfs, connected_components, degree_centrality, dfs,
    has_cycle, is_connected, minimum_spanning_tree, query_annotations_of, query_co_members,
    query_walks_containing, shortest_path, topological_sort,
};

// Re-export Gram namespace
pub use gram::Gram;
