use wasm_bindgen::prelude::*;

use crate::convert::{rust_pattern_to_wasm, wasm_pattern_to_rust};
use pattern_core::wasm::WasmPattern;

/// Gram namespace for serializing and parsing patterns
#[wasm_bindgen]
pub struct Gram;

#[wasm_bindgen]
impl Gram {
    /// Stringify a single pattern to gram notation
    ///
    /// # Arguments
    /// * `pattern` - A Pattern<Subject> to serialize
    ///
    /// # Returns
    /// * `Ok(String)` - The gram notation string
    /// * `Err(String)` - Error message if serialization fails
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.point(new Subject("alice", ["Person"], {name: "Alice"}));
    /// const gram = Gram.stringify(pattern);
    /// console.log(gram); // (alice:Person {name: "Alice"})
    /// ```
    #[wasm_bindgen(js_name = stringify)]
    pub fn stringify(pattern: &WasmPattern) -> Result<String, String> {
        let rust_pattern = wasm_pattern_to_rust(pattern)
            .map_err(|e| format!("Failed to convert pattern: {}", e))?;

        gram_codec::to_gram_pattern(&rust_pattern)
            .map_err(|e| format!("Serialization failed: {}", e))
    }

    /// Parse gram notation string into an array of patterns
    ///
    /// Empty or whitespace-only input returns an empty array.
    #[wasm_bindgen]
    pub fn parse(gram: &str) -> Result<js_sys::Array, String> {
        // Parse gram notation
        let rust_patterns = gram_codec::parse_gram(gram)
            .map_err(|e| format!("Parse failed: {}", e))?;

        // Convert to JS array of WasmPattern instances
        let js_array = js_sys::Array::new_with_length(rust_patterns.len() as u32);

        for (i, rust_pattern) in rust_patterns.iter().enumerate() {
            let wasm_pattern = rust_pattern_to_wasm(rust_pattern);
            js_array.set(i as u32, wasm_pattern.into());
        }

        Ok(js_array)
    }

    /// Parse gram notation and return the first pattern or null
    ///
    /// Returns null for empty or whitespace-only input.
    ///
    /// TODO: Implement in Phase 4 (T010)
    #[wasm_bindgen(js_name = parseOne)]
    pub fn parse_one(_gram: &str) -> JsValue {
        JsValue::null()
    }

    /// Convert a Pattern<V> to Pattern<Subject> using conventional conversion
    ///
    /// Implemented as pattern.map(v => Subject.fromValue(v, options))
    ///
    /// TODO: Implement in Phase 5 (T014)
    #[wasm_bindgen]
    pub fn from(_pattern: JsValue, _options: JsValue) -> JsValue {
        JsValue::null()
    }
}
