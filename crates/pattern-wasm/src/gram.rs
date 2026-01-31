use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

use crate::convert::{rust_pattern_to_wasm, wasm_pattern_to_rust};
use pattern_core::wasm::WasmPattern;
use pattern_core::{Pattern, Subject, Symbol};

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
        let rust_patterns = gram_codec::parse_gram(gram)
            .map_err(|e| format!("Parse error: {}", e))?;

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
    #[wasm_bindgen(js_name = parseOne)]
    pub fn parse_one(gram: &str) -> Result<JsValue, String> {
        // Parse gram notation
        let rust_patterns = gram_codec::parse_gram(gram)
            .map_err(|e| format!("Parse error: {}", e))?;

        // Return null for empty input
        if rust_patterns.is_empty() {
            return Ok(JsValue::null());
        }

        // Convert first pattern to WasmPattern and return
        let wasm_pattern = rust_pattern_to_wasm(&rust_patterns[0]);
        Ok(wasm_pattern.into())
    }

    /// Convert a Pattern<V> to Pattern<Subject> using pattern-lisp compatible mapping.
    ///
    /// This method recursively converts a pattern containing arbitrary JavaScript values
    /// into Pattern<Subject> that can be serialized to gram notation and parsed by pattern-lisp.
    ///
    /// **Compatibility with pattern-lisp:**
    /// - Primitives (string, number, boolean) → atomic pattern with Subject
    /// - Arrays → Pattern with "List" label and elements as children
    /// - Objects → Pattern with "Map" label and key-value pairs as alternating children
    ///
    /// # Arguments
    /// * `pattern` - A Pattern containing any JavaScript values
    /// * `options` - Optional conversion options (currently unused, reserved for future use)
    ///
    /// # Returns
    /// * `Ok(Pattern)` - Pattern<Subject> that can be serialized with stringify()
    /// * `Err(String)` - Error message if conversion fails
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// // Convert pattern of primitives
    /// const p1 = Pattern.point("hello");
    /// const s1 = Gram.from(p1);
    /// Gram.stringify(s1); // "(_0:String {value: \"hello\"})"
    ///
    /// // Convert pattern with nested structure
    /// const parent = Pattern.pattern("root");
    /// parent.addElement(Pattern.of(42));
    /// parent.addElement(Pattern.of(true));
    /// const converted = Gram.from(parent);
    /// ```
    #[wasm_bindgen]
    pub fn from(pattern: &WasmPattern, _options: JsValue) -> Result<WasmPattern, String> {
        let mut index = 0usize;
        let rust_pattern = convert_js_pattern_to_subject_pattern(pattern, &mut index)?;
        Ok(rust_pattern_to_wasm(&rust_pattern))
    }
}

/// Recursively convert a WasmPattern (Pattern<JsValue>) to Pattern<Subject>
/// using pattern-lisp compatible conventions.
fn convert_js_pattern_to_subject_pattern(
    pattern: &WasmPattern,
    index: &mut usize,
) -> Result<Pattern<Subject>, String> {
    let js_value = pattern.value();

    // Convert the value to Subject (or create pattern structure for collections)
    let (subject, collection_elements) = js_value_to_subject_or_pattern(&js_value, index)?;

    // Get the pattern's own elements (children in the tree structure)
    let pattern_element_count = pattern.length();
    let mut converted_elements: Vec<Pattern<Subject>> = Vec::new();

    // First add any collection elements (for List/Map values)
    converted_elements.extend(collection_elements);

    // Then recursively convert the pattern's structural children
    for i in 0..pattern_element_count {
        if let Some(child) = pattern.get_element(i) {
            let converted_child = convert_js_pattern_to_subject_pattern(&child, index)?;
            converted_elements.push(converted_child);
        }
    }

    if converted_elements.is_empty() {
        Ok(Pattern::point(subject))
    } else {
        Ok(Pattern::pattern(subject, converted_elements))
    }
}

/// Convert a JsValue to either:
/// - A Subject (for primitives) with empty elements vec
/// - A Subject (for collections) with elements vec containing the collection items
fn js_value_to_subject_or_pattern(
    value: &JsValue,
    index: &mut usize,
) -> Result<(Subject, Vec<Pattern<Subject>>), String> {
    // Handle null/undefined
    if value.is_null() || value.is_undefined() {
        return Err("Cannot convert null/undefined to Subject".to_string());
    }

    // Boolean - use "Bool" for pattern-lisp compatibility
    if let Some(b) = value.as_bool() {
        let current_index = *index;
        *index += 1;

        let mut properties = HashMap::new();
        properties.insert("value".to_string(), pattern_core::subject::Value::VBoolean(b));

        let mut labels = HashSet::new();
        labels.insert("Bool".to_string());

        return Ok((Subject {
            identity: Symbol(format!("_{}", current_index)),
            labels,
            properties,
        }, vec![]));
    }

    // String
    if let Some(s) = value.as_string() {
        let current_index = *index;
        *index += 1;

        let mut properties = HashMap::new();
        properties.insert("value".to_string(), pattern_core::subject::Value::VString(s));

        let mut labels = HashSet::new();
        labels.insert("String".to_string());

        return Ok((Subject {
            identity: Symbol(format!("_{}", current_index)),
            labels,
            properties,
        }, vec![]));
    }

    // Number
    if let Some(n) = value.as_f64() {
        let current_index = *index;
        *index += 1;

        let mut properties = HashMap::new();
        let val = if n.fract() == 0.0 && n >= i64::MIN as f64 && n <= i64::MAX as f64 {
            pattern_core::subject::Value::VInteger(n as i64)
        } else {
            pattern_core::subject::Value::VDecimal(n)
        };
        properties.insert("value".to_string(), val);

        let mut labels = HashSet::new();
        labels.insert("Number".to_string());

        return Ok((Subject {
            identity: Symbol(format!("_{}", current_index)),
            labels,
            properties,
        }, vec![]));
    }

    // Array - create List pattern with elements as children
    if js_sys::Array::is_array(value) {
        let arr: &js_sys::Array = value.unchecked_ref();
        let mut elements = Vec::new();

        for i in 0..arr.length() {
            let item = arr.get(i);
            let (item_subject, item_elements) = js_value_to_subject_or_pattern(&item, index)?;
            if item_elements.is_empty() {
                elements.push(Pattern::point(item_subject));
            } else {
                elements.push(Pattern::pattern(item_subject, item_elements));
            }
        }

        // List decoration has no properties, just the label
        let mut labels = HashSet::new();
        labels.insert("List".to_string());

        return Ok((Subject {
            identity: Symbol("".to_string()),
            labels,
            properties: HashMap::new(),
        }, elements));
    }

    // Check if this is a WasmSubject instance (passthrough)
    if value.is_object() {
        let obj: &js_sys::Object = value.unchecked_ref();

        // Check for __wbg_ptr (wasm-bindgen wrapper indicates WasmSubject)
        if js_sys::Reflect::has(obj, &JsValue::from_str("__wbg_ptr")).unwrap_or(false) {
            // Try to extract Subject data from WasmSubject
            if let Ok(identity_js) = js_sys::Reflect::get(obj, &JsValue::from_str("identity")) {
                if let Some(identity) = identity_js.as_string() {
                    // Extract labels
                    let labels_js = js_sys::Reflect::get(obj, &JsValue::from_str("labels"))
                        .ok()
                        .unwrap_or(JsValue::undefined());
                    let mut labels = HashSet::new();
                    if js_sys::Array::is_array(&labels_js) {
                        let arr: &js_sys::Array = labels_js.unchecked_ref();
                        for i in 0..arr.length() {
                            if let Some(label) = arr.get(i).as_string() {
                                labels.insert(label);
                            }
                        }
                    }

                    // Extract properties
                    let props_js = js_sys::Reflect::get(obj, &JsValue::from_str("properties"))
                        .ok()
                        .unwrap_or(JsValue::undefined());
                    let properties = if props_js.is_object() && !props_js.is_null() {
                        js_object_to_subject_properties(&props_js)?
                    } else {
                        HashMap::new()
                    };

                    return Ok((Subject {
                        identity: Symbol(identity),
                        labels,
                        properties,
                    }, vec![]));
                }
            }
        }

        // Plain object - convert to Map pattern with alternating key-value elements
        let keys = js_sys::Object::keys(obj);
        let mut elements = Vec::new();

        for i in 0..keys.length() {
            let key = keys.get(i);
            if let Some(key_str) = key.as_string() {
                // Add key as String pattern
                let key_index = *index;
                *index += 1;

                let mut key_properties = HashMap::new();
                key_properties.insert("value".to_string(), pattern_core::subject::Value::VString(key_str.clone()));
                let mut key_labels = HashSet::new();
                key_labels.insert("String".to_string());

                elements.push(Pattern::point(Subject {
                    identity: Symbol(format!("_{}", key_index)),
                    labels: key_labels,
                    properties: key_properties,
                }));

                // Add value
                if let Ok(val) = js_sys::Reflect::get(obj, &key) {
                    let (val_subject, val_elements) = js_value_to_subject_or_pattern(&val, index)?;
                    if val_elements.is_empty() {
                        elements.push(Pattern::point(val_subject));
                    } else {
                        elements.push(Pattern::pattern(val_subject, val_elements));
                    }
                }
            }
        }

        // Map decoration has no properties, just the label
        let mut labels = HashSet::new();
        labels.insert("Map".to_string());

        return Ok((Subject {
            identity: Symbol("".to_string()),
            labels,
            properties: HashMap::new(),
        }, elements));
    }

    Err(format!("Cannot convert value to Subject: {:?}", value))
}

/// Convert a JS object to Subject properties map
fn js_object_to_subject_properties(
    obj: &JsValue,
) -> Result<HashMap<String, pattern_core::subject::Value>, String> {
    use pattern_core::subject::Value;

    let mut map = HashMap::new();

    if !obj.is_object() || obj.is_null() {
        return Ok(map);
    }

    let keys = js_sys::Object::keys(obj.unchecked_ref());

    for i in 0..keys.length() {
        let key = keys.get(i);
        if let Some(key_str) = key.as_string() {
            if let Ok(val) = js_sys::Reflect::get(obj, &key) {
                if let Some(s) = val.as_string() {
                    map.insert(key_str, Value::VString(s));
                } else if let Some(n) = val.as_f64() {
                    if n.fract() == 0.0 && n >= i64::MIN as f64 && n <= i64::MAX as f64 {
                        map.insert(key_str, Value::VInteger(n as i64));
                    } else {
                        map.insert(key_str, Value::VDecimal(n));
                    }
                } else if let Some(b) = val.as_bool() {
                    map.insert(key_str, Value::VBoolean(b));
                }
                // Skip complex types in properties for now
            }
        }
    }

    Ok(map)
}
