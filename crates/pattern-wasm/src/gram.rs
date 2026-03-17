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
    ///
    /// # Arguments
    /// * `gram` - Gram notation string to parse
    ///
    /// # Returns
    /// * `Ok(Array)` - Array of Pattern<Subject> instances (empty array for empty input)
    /// * `Err(String)` - Parse error with location information
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const patterns = Gram.parse("(alice:Person) (bob:Person)");
    /// console.log(patterns.length); // 2
    ///
    /// const empty = Gram.parse("");
    /// console.log(empty.length); // 0
    /// ```
    #[wasm_bindgen]
    pub fn parse(gram: &str) -> Result<js_sys::Array, String> {
        // Parse gram notation (handles empty input by returning empty Vec)
        let rust_patterns =
            gram_codec::parse_gram(gram).map_err(|e| format!("Parse error: {}", e))?;

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
    /// # Arguments
    /// * `gram` - Gram notation string to parse
    ///
    /// # Returns
    /// * Pattern<Subject> instance if input contains at least one pattern
    /// * null if input is empty or contains only whitespace
    ///
    /// # Errors
    /// Throws an error if the gram notation is invalid
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Gram.parseOne("(alice:Person {name: \"Alice\"})");
    /// console.log(pattern.value.identity); // "alice"
    ///
    /// const nothing = Gram.parseOne("");
    /// console.log(nothing); // null
    ///
    /// const first = Gram.parseOne("(alice) (bob)");
    /// console.log(first.value.identity); // "alice" (only first pattern returned)
    /// ```
    /// Parse gram notation and return a JSON string array of pattern objects.
    ///
    /// This is the primary interchange function for native TypeScript consumers.
    /// The JSON format uses the "subject" key and canonical value encoding.
    ///
    /// Empty or whitespace-only input returns "[]".
    ///
    /// # Arguments
    /// * `gram` - Gram notation string to parse
    ///
    /// # Returns
    /// * `Ok(String)` - JSON array string of AstPattern objects
    /// * `Err(String)` - Parse error message
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const json = Gram.parseToJson("(alice:Person)");
    /// const patterns = JSON.parse(json);
    /// console.log(patterns[0].subject.identity); // "alice"
    /// ```
    #[wasm_bindgen(js_name = parseToJson)]
    pub fn parse_to_json(gram: &str) -> Result<String, String> {
        gram_codec::gram_parse_to_json(gram)
    }

    /// Serialize a JSON array of pattern objects back to gram notation.
    ///
    /// # Arguments
    /// * `json` - JSON array string of AstPattern objects
    ///
    /// # Returns
    /// * `Ok(String)` - Gram notation string
    /// * `Err(String)` - Error message if deserialization or serialization fails
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const gram = Gram.stringifyFromJson(json);
    /// console.log(gram); // "(alice:Person)"
    /// ```
    #[wasm_bindgen(js_name = stringifyFromJson)]
    pub fn stringify_from_json(json: &str) -> Result<String, String> {
        gram_codec::gram_stringify_from_json(json)
    }

    #[wasm_bindgen(js_name = parseOne)]
    pub fn parse_one(gram: &str) -> Result<JsValue, String> {
        // Parse gram notation
        let rust_patterns =
            gram_codec::parse_gram(gram).map_err(|e| format!("Parse error: {}", e))?;

        // Return null for empty input
        if rust_patterns.is_empty() {
            return Ok(JsValue::null());
        }

        // Convert first pattern to WasmPattern and return
        let wasm_pattern = rust_pattern_to_wasm(&rust_patterns[0]);
        Ok(wasm_pattern.into())
    }
}
