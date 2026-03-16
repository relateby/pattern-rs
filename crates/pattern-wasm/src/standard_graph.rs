use wasm_bindgen::prelude::*;

use pattern_core::wasm::{
    WasmGraphQuery, WasmPattern, WasmPatternGraph, WasmStandardGraph, WasmSubject,
};

/// StandardGraph exposed from pattern-wasm.
///
/// Wraps `WasmStandardGraph` from pattern-core and adds `fromGram`, which
/// must live here because `pattern-core` cannot depend on `gram-codec`
/// (gram-codec depends on pattern-core for types — circular).
///
/// All other methods delegate to the inner `WasmStandardGraph`.
#[wasm_bindgen]
pub struct StandardGraph {
    #[wasm_bindgen(skip)]
    pub inner: WasmStandardGraph,
}

impl Default for StandardGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
impl StandardGraph {
    /// Create an empty StandardGraph.
    #[wasm_bindgen(constructor)]
    pub fn new() -> StandardGraph {
        StandardGraph {
            inner: WasmStandardGraph::new(),
        }
    }

    /// Parse gram notation into a StandardGraph.
    ///
    /// Empty or whitespace-only input returns an empty graph.
    ///
    /// # Errors
    /// Throws a JavaScript `Error` on invalid gram syntax.
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const g = StandardGraph.fromGram("(alice:Person) (bob:Person)");
    /// console.log(g.nodeCount); // 2
    /// ```
    #[wasm_bindgen(js_name = fromGram)]
    pub fn from_gram(input: &str) -> Result<StandardGraph, JsValue> {
        let patterns =
            gram_codec::parse_gram(input).map_err(|e| JsValue::from_str(&format!("{}", e)))?;
        Ok(StandardGraph {
            inner: WasmStandardGraph::from_patterns(
                &patterns
                    .iter()
                    .map(|p| wasm_bindgen::JsValue::from(crate::convert::rust_pattern_to_wasm(p)))
                    .collect::<js_sys::Array>(),
            ),
        })
    }

    /// Create from an array of Pattern<Subject> instances.
    #[wasm_bindgen(js_name = fromPatterns)]
    pub fn from_patterns(patterns: &js_sys::Array) -> StandardGraph {
        StandardGraph {
            inner: WasmStandardGraph::from_patterns(patterns),
        }
    }

    /// Wrap an existing NativePatternGraph.
    #[wasm_bindgen(js_name = fromPatternGraph)]
    pub fn from_pattern_graph(graph: &WasmPatternGraph) -> StandardGraph {
        StandardGraph {
            inner: WasmStandardGraph::from_pattern_graph(graph),
        }
    }

    // --- Element addition ---

    /// Add a node to the graph.
    #[wasm_bindgen(js_name = addNode)]
    pub fn add_node(&mut self, subject: &WasmSubject) {
        self.inner.add_node(subject);
    }

    /// Add a relationship to the graph.
    #[wasm_bindgen(js_name = addRelationship)]
    pub fn add_relationship(&mut self, subject: &WasmSubject, source_id: &str, target_id: &str) {
        self.inner.add_relationship(subject, source_id, target_id);
    }

    /// Add a walk to the graph.
    #[wasm_bindgen(js_name = addWalk)]
    pub fn add_walk(&mut self, subject: &WasmSubject, relationship_ids: &js_sys::Array) {
        self.inner.add_walk(subject, relationship_ids);
    }

    /// Add an annotation to the graph.
    #[wasm_bindgen(js_name = addAnnotation)]
    pub fn add_annotation(&mut self, subject: &WasmSubject, element_id: &str) {
        self.inner.add_annotation(subject, element_id);
    }

    /// Add a single pattern.
    #[wasm_bindgen(js_name = addPattern)]
    pub fn add_pattern(&mut self, pattern: &WasmPattern) {
        self.inner.add_pattern(pattern);
    }

    /// Add multiple patterns.
    #[wasm_bindgen(js_name = addPatterns)]
    pub fn add_patterns(&mut self, patterns: &js_sys::Array) {
        self.inner.add_patterns(patterns);
    }

    // --- Element access ---

    /// Get a node by identity.
    #[wasm_bindgen(js_name = node)]
    pub fn node(&self, id: &str) -> Option<WasmPattern> {
        self.inner.node(id)
    }

    /// Get a relationship by identity.
    #[wasm_bindgen(js_name = relationship)]
    pub fn relationship(&self, id: &str) -> Option<WasmPattern> {
        self.inner.relationship(id)
    }

    /// Get a walk by identity.
    #[wasm_bindgen(js_name = walk)]
    pub fn walk(&self, id: &str) -> Option<WasmPattern> {
        self.inner.walk(id)
    }

    /// Get an annotation by identity.
    #[wasm_bindgen(js_name = annotation)]
    pub fn annotation(&self, id: &str) -> Option<WasmPattern> {
        self.inner.annotation(id)
    }

    // --- Count getters ---

    #[wasm_bindgen(getter, js_name = nodeCount)]
    pub fn node_count(&self) -> usize {
        self.inner.node_count()
    }

    #[wasm_bindgen(getter, js_name = relationshipCount)]
    pub fn relationship_count(&self) -> usize {
        self.inner.relationship_count()
    }

    #[wasm_bindgen(getter, js_name = walkCount)]
    pub fn walk_count(&self) -> usize {
        self.inner.walk_count()
    }

    #[wasm_bindgen(getter, js_name = annotationCount)]
    pub fn annotation_count(&self) -> usize {
        self.inner.annotation_count()
    }

    #[wasm_bindgen(getter, js_name = isEmpty)]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[wasm_bindgen(getter, js_name = hasConflicts)]
    pub fn has_conflicts(&self) -> bool {
        self.inner.has_conflicts()
    }

    // --- Iteration getters ---

    #[wasm_bindgen(getter, js_name = nodes)]
    pub fn nodes(&self) -> js_sys::Array {
        self.inner.nodes()
    }

    #[wasm_bindgen(getter, js_name = relationships)]
    pub fn relationships(&self) -> js_sys::Array {
        self.inner.relationships()
    }

    #[wasm_bindgen(getter, js_name = walks)]
    pub fn walks(&self) -> js_sys::Array {
        self.inner.walks()
    }

    #[wasm_bindgen(getter, js_name = annotations)]
    pub fn annotations(&self) -> js_sys::Array {
        self.inner.annotations()
    }

    // --- Graph-native queries ---

    #[wasm_bindgen(js_name = source)]
    pub fn source(&self, rel_id: &str) -> Option<WasmPattern> {
        self.inner.source(rel_id)
    }

    #[wasm_bindgen(js_name = target)]
    pub fn target(&self, rel_id: &str) -> Option<WasmPattern> {
        self.inner.target(rel_id)
    }

    #[wasm_bindgen(js_name = neighbors)]
    pub fn neighbors(&self, node_id: &str) -> js_sys::Array {
        self.inner.neighbors(node_id)
    }

    #[wasm_bindgen(js_name = degree)]
    pub fn degree(&self, node_id: &str) -> usize {
        self.inner.degree(node_id)
    }

    // --- Escape hatches ---

    #[wasm_bindgen(js_name = asPatternGraph)]
    pub fn as_pattern_graph(&self) -> WasmPatternGraph {
        self.inner.as_pattern_graph()
    }

    #[wasm_bindgen(js_name = asQuery)]
    pub fn as_query(&self) -> WasmGraphQuery {
        self.inner.as_query()
    }
}
