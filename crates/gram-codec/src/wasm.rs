//! WASM bindings for Gram Codec
//!
//! This module provides WebAssembly bindings for the gram codec,
//! enabling use in browsers and Node.js environments.
//!
//! # Usage
//!
//! ```javascript
//! import init, { parse_gram, validate_gram } from './gram_codec.js';
//!
//! await init();
//!
//! // Parse and validate gram notation
//! try {
//!     const result = parse_gram("(alice)-[:KNOWS]->(bob)");
//!     console.log("Valid gram notation");
//!     console.log("Pattern count:", result.pattern_count);
//!     console.log("Root identifiers:", result.identifiers);
//! } catch (e) {
//!     console.error("Parse error:", e);
//! }
//!
//! // Quick validation
//! const isValid = validate_gram("(hello)-->(world)");
//! console.log("Is valid:", isValid);
//! ```

use crate::ast::AstPattern;
use wasm_bindgen::prelude::*;

/// Result of parsing gram notation
#[wasm_bindgen]
pub struct ParseResult {
    /// Number of top-level patterns parsed
    pattern_count: usize,
    /// Identifiers of root patterns (for debugging)
    identifiers: Vec<String>,
}

#[wasm_bindgen]
impl ParseResult {
    #[wasm_bindgen(getter)]
    pub fn pattern_count(&self) -> usize {
        self.pattern_count
    }

    #[wasm_bindgen(getter)]
    pub fn identifiers(&self) -> Vec<String> {
        self.identifiers.clone()
    }
}

/// Parse Gram notation text and return summary information
///
/// This is useful for validation and basic structure inspection.
/// For full pattern manipulation, use the native Rust API.
///
/// # Arguments
///
/// * `input` - Gram notation text to parse
///
/// # Returns
///
/// ParseResult with pattern count and identifiers
///
/// # Errors
///
/// Throws JavaScript error if parsing fails
#[wasm_bindgen]
pub fn parse_gram(input: &str) -> Result<ParseResult, JsValue> {
    // Parse using the native parser (now uses nom, not tree-sitter)
    let patterns =
        crate::parse_gram(input).map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    // Extract identifiers from root patterns
    let identifiers: Vec<String> = patterns
        .iter()
        .map(|p| p.value().identity.0.clone())
        .collect();

    Ok(ParseResult {
        pattern_count: patterns.len(),
        identifiers,
    })
}

/// Validate gram notation syntax
///
/// Quick validation check without returning parsed structure.
///
/// # Arguments
///
/// * `input` - Gram notation text to validate
///
/// # Returns
///
/// `true` if valid, `false` if invalid
#[wasm_bindgen]
pub fn validate_gram(input: &str) -> bool {
    crate::validate_gram(input).is_ok()
}

/// Round-trip test: parse and serialize back to gram notation
///
/// Useful for testing round-trip correctness in JavaScript environments.
///
/// # Arguments
///
/// * `input` - Original gram notation
///
/// # Returns
///
/// Serialized gram notation (may differ in formatting but should be semantically equivalent)
#[wasm_bindgen]
pub fn round_trip(input: &str) -> Result<String, JsValue> {
    // Parse
    let patterns =
        crate::parse_gram(input).map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    // Serialize all patterns
    crate::serialize_patterns(&patterns)
        .map_err(|e| JsValue::from_str(&format!("Serialize error: {}", e)))
}

/// Parse gram notation to AST (JavaScript-friendly)
///
/// Returns a single pattern as a JavaScript object.
/// This is the recommended way to parse gram in JavaScript/TypeScript.
///
/// The returned object has structure:
/// ```javascript
/// {
///   subject: {
///     identity: string,
///     labels: string[],
///     properties: object
///   },
///   elements: AstPattern[]
/// }
/// ```
///
/// # Example
///
/// ```javascript
/// import init, { parse_to_ast } from './gram_codec.js';
/// await init();
///
/// const ast = parse_to_ast("(alice:Person {name: 'Alice'})");
/// console.log(ast.subject.identity);  // "alice"
/// console.log(ast.subject.labels);    // ["Person"]
/// console.log(ast.subject.properties.name);  // "Alice"
/// ```
#[wasm_bindgen]
pub fn parse_to_ast(input: &str) -> Result<JsValue, JsValue> {
    let ast = crate::parse_to_ast(input)
        .map_err(|e| JsValue::from_str(&format!("Parse error: {}", e)))?;

    // Serialize to JSON string first (workaround for serde-wasm-bindgen HashMap issue)
    let json_str = serde_json::to_string(&ast)
        .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))?;

    // Parse JSON string in JavaScript
    let js_json = js_sys::JSON::parse(&json_str)
        .map_err(|e| JsValue::from_str(&format!("JSON parse error: {:?}", e)))?;

    Ok(js_json)
}

/// Get version information for the gram codec
#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_gram() {
        assert!(validate_gram("(hello)"));
        assert!(validate_gram("(a)-->(b)"));
        assert!(!validate_gram("(unclosed"));
    }

    #[test]
    fn test_parse_gram() {
        let result = parse_gram("(alice) (bob)").unwrap();
        assert_eq!(result.pattern_count, 2);
        assert_eq!(result.identifiers.len(), 2);
    }

    #[test]
    fn test_round_trip() {
        let input = "(hello)";
        let output = round_trip(input).unwrap();
        assert_eq!(output, "(hello)");
    }
}
