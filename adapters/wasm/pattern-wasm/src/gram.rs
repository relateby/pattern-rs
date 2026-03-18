use wasm_bindgen::prelude::*;

/// Gram namespace for JSON-based parsing and serialization.
#[wasm_bindgen]
pub struct Gram;

#[wasm_bindgen]
impl Gram {
    #[wasm_bindgen(js_name = parseToJson)]
    pub fn parse_to_json(gram: &str) -> Result<String, String> {
        gram_codec::gram_parse_to_json(gram)
    }

    #[wasm_bindgen(js_name = stringifyFromJson)]
    pub fn stringify_from_json(json: &str) -> Result<String, String> {
        gram_codec::gram_stringify_from_json(json)
    }

    #[wasm_bindgen(js_name = validate)]
    pub fn validate(gram: &str) -> js_sys::Array {
        let errors_json = gram_codec::gram_validate_to_json(gram);
        match js_sys::JSON::parse(&errors_json) {
            Ok(value) => js_sys::Array::from(&value),
            Err(_) => {
                let errors = js_sys::Array::new();
                errors.push(&JsValue::from_str("Failed to decode validation errors"));
                errors
            }
        }
    }
}
