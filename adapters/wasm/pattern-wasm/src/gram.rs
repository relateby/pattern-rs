use serde::Serialize;
use wasm_bindgen::prelude::*;

/// Gram namespace: parse and serialize gram notation via WebAssembly.
#[wasm_bindgen]
pub struct Gram;

#[wasm_bindgen]
impl Gram {
    /// Parse gram notation into an array of AstPattern objects (as a JS value).
    #[wasm_bindgen]
    pub fn parse(gram: &str) -> Result<JsValue, JsValue> {
        if gram.trim().is_empty() {
            let empty: Vec<gram_codec::AstPattern> = vec![];
            let serializer = serde_wasm_bindgen::Serializer::json_compatible();
            return empty
                .serialize(&serializer)
                .map_err(|e| JsValue::from_str(&e.to_string()));
        }
        let patterns =
            gram_codec::parse_gram(gram).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let asts: Vec<gram_codec::AstPattern> = patterns
            .iter()
            .map(gram_codec::AstPattern::from_pattern)
            .collect();
        let serializer = serde_wasm_bindgen::Serializer::json_compatible();
        asts.serialize(&serializer)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Serialize an array of AstPattern objects (JS value) to gram notation.
    #[wasm_bindgen]
    pub fn stringify(patterns_js: JsValue) -> Result<String, JsValue> {
        let asts: Vec<gram_codec::AstPattern> = serde_wasm_bindgen::from_value(patterns_js)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let patterns: Vec<gram_codec::Pattern<gram_codec::Subject>> = asts
            .iter()
            .map(|ast| ast.to_pattern())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JsValue::from_str(&e))?;
        gram_codec::to_gram(&patterns).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Validate gram notation; returns an array of error strings (empty = valid).
    #[wasm_bindgen]
    pub fn validate(gram: &str) -> js_sys::Array {
        match gram_codec::validate_gram(gram) {
            Ok(()) => js_sys::Array::new(),
            Err(e) => {
                let errors = js_sys::Array::new();
                errors.push(&JsValue::from_str(&e.to_string()));
                errors
            }
        }
    }

    /// Parse gram notation, returning { header, patterns } where header is the
    /// optional leading bare record.
    #[wasm_bindgen(js_name = parseWithHeader)]
    pub fn parse_with_header(gram: &str) -> Result<JsValue, JsValue> {
        let (header, patterns) = gram_codec::parse_gram_with_header(gram)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        let result = gram_codec::ParseWithHeaderResult::from_parts(header, patterns);
        let serializer = serde_wasm_bindgen::Serializer::json_compatible();
        result
            .serialize(&serializer)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Serialize { header, patterns } to gram notation.
    #[wasm_bindgen(js_name = stringifyWithHeader)]
    pub fn stringify_with_header(input: JsValue) -> Result<String, JsValue> {
        let result: gram_codec::ParseWithHeaderResult =
            serde_wasm_bindgen::from_value(input).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let header = result
            .header_to_record()
            .map_err(|e| JsValue::from_str(&e))?
            .unwrap_or_default();
        let patterns: Vec<gram_codec::Pattern<gram_codec::Subject>> = result
            .patterns
            .iter()
            .map(|ast| ast.to_pattern())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| JsValue::from_str(&e))?;
        gram_codec::to_gram_with_header(header, &patterns)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}
