use wasm_bindgen::prelude::*;

/// Gram namespace for serializing and parsing patterns
#[wasm_bindgen]
pub struct Gram;

#[wasm_bindgen]
impl Gram {
    /// Stringify a pattern (single or array) to gram notation
    ///
    /// In JavaScript, this is overloaded via TypeScript definitions to accept:
    /// - Pattern<Subject> → string
    /// - Pattern<Subject>[] → string
    ///
    /// TODO: Implement in Phase 3 (T007)
    #[wasm_bindgen(js_name = stringify)]
    pub fn stringify(_pattern_or_array: JsValue) -> String {
        String::new()
    }

    /// Parse gram notation string into an array of patterns
    ///
    /// Empty or whitespace-only input returns an empty array.
    ///
    /// TODO: Implement in Phase 3 (T008)
    #[wasm_bindgen]
    pub fn parse(_gram: &str) -> js_sys::Array {
        js_sys::Array::new()
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
