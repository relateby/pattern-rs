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

    /// Convert any JavaScript value to Pattern<Subject> using pattern-lisp compatible mapping.
    ///
    /// This method converts arbitrary JavaScript values into Pattern<Subject> that can be
    /// serialized to gram notation and parsed by pattern-lisp.
    ///
    /// **Input handling:**
    /// - WasmPattern: recursively converts each value in the pattern (like pattern.map(Gram.from))
    /// - WasmSubject: passthrough, wraps in atomic Pattern
    /// - Primitives (string, number, boolean): atomic Pattern with typed Subject
    /// - Arrays: Pattern with "List" label and converted elements as children
    /// - Objects: Pattern with "Map" label and alternating key-value pairs as children
    ///
    /// **Compatibility with pattern-lisp:**
    /// - Numbers → Subject with "Number" label
    /// - Strings → Subject with "String" label
    /// - Booleans → Subject with "Bool" label
    /// - Arrays → Pattern with "List" label
    /// - Objects → Pattern with "Map" label
    ///
    /// # Arguments
    /// * `value` - Any JavaScript value to convert
    /// * `options` - Optional conversion options (currently unused, reserved for future use)
    ///
    /// # Returns
    /// * `Ok(Pattern)` - Pattern<Subject> that can be serialized with stringify()
    /// * `Err(String)` - Error message if conversion fails
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// // Convert primitives directly
    /// const s1 = Gram.from("hello");
    /// Gram.stringify(s1); // "(_0:String {value: \"hello\"})"
    ///
    /// // Convert arrays
    /// const s2 = Gram.from([1, 2, 3]);
    /// // Creates: Pattern with List subject and Number children
    ///
    /// // Convert objects
    /// const s3 = Gram.from({name: "Alice", age: 30});
    /// // Creates: Pattern with Map subject and alternating key-value children
    ///
    /// // Convert existing Pattern (maps over values)
    /// const p = Pattern.pattern("root");
    /// p.addElement(Pattern.of(42));
    /// const converted = Gram.from(p);
    ///
    /// // WasmSubject passthrough
    /// const subject = new Subject("alice", ["Person"], {});
    /// const s4 = Gram.from(subject); // Wraps in atomic Pattern
    /// ```
    #[wasm_bindgen]
    pub fn from(value: JsValue) -> Result<WasmPattern, String> {
        let mut index = 0usize;
        let rust_pattern = js_value_to_pattern_subject(&value, &mut index)?;
        Ok(rust_pattern_to_wasm(&rust_pattern))
    }
}

/// Convert any JsValue to Pattern<Subject>.
///
/// This is the main entry point for converting arbitrary JS values to gram-serializable form.
fn js_value_to_pattern_subject(
    value: &JsValue,
    index: &mut usize,
) -> Result<Pattern<Subject>, String> {
    // Handle null/undefined
    if value.is_null() || value.is_undefined() {
        return Err("Cannot convert null/undefined to Pattern<Subject>".to_string());
    }

    // Check if this is a wasm-bindgen object (has __wbg_ptr)
    if value.is_object() {
        let obj: &js_sys::Object = value.unchecked_ref();

        if js_sys::Reflect::has(obj, &JsValue::from_str("__wbg_ptr")).unwrap_or(false) {
            // Check if it's a WasmPattern by looking for Pattern-specific methods/properties
            // WasmPattern has: value (getter), elements (getter), length(), addElement(), etc.
            let has_add_element =
                js_sys::Reflect::has(obj, &JsValue::from_str("addElement")).unwrap_or(false);

            if has_add_element {
                // This looks like a WasmPattern - reconstruct and convert
                if let Some(wasm_pattern) = try_extract_wasm_pattern(value) {
                    return convert_wasm_pattern_to_subject_pattern(&wasm_pattern, index);
                }
            }

            // Check if it's a WasmSubject by looking for Subject-specific properties
            // WasmSubject has: identity (getter), labels (getter), properties (getter)
            if let Ok(identity_js) = js_sys::Reflect::get(obj, &JsValue::from_str("identity")) {
                if identity_js.as_string().is_some() {
                    // This looks like a WasmSubject - extract and passthrough
                    if let Some(subject) = try_extract_subject_from_wasm(obj) {
                        return Ok(Pattern::point(subject));
                    }
                }
            }
        }
    }

    // For non-Pattern values, convert to Subject (possibly with nested structure)
    let (subject, elements) = js_value_to_subject_with_elements(value, index)?;

    if elements.is_empty() {
        Ok(Pattern::point(subject))
    } else {
        Ok(Pattern::pattern(subject, elements))
    }
}

/// Convert a WasmPattern to Pattern<Subject> by mapping over its structure.
/// This preserves the Pattern's tree structure while converting each value.
fn convert_wasm_pattern_to_subject_pattern(
    pattern: &WasmPattern,
    index: &mut usize,
) -> Result<Pattern<Subject>, String> {
    let js_value = pattern.value();

    // Convert the value at this node
    let (subject, value_elements) = js_value_to_subject_with_elements(&js_value, index)?;

    // Recursively convert the pattern's structural children
    let mut converted_elements: Vec<Pattern<Subject>> = Vec::new();

    // First add any elements that came from the value itself (e.g., List/Map elements)
    converted_elements.extend(value_elements);

    // Then recursively convert the pattern's structural children
    let pattern_element_count = pattern.length();
    for i in 0..pattern_element_count {
        if let Some(child) = pattern.get_element(i) {
            let converted_child = convert_wasm_pattern_to_subject_pattern(&child, index)?;
            converted_elements.push(converted_child);
        }
    }

    if converted_elements.is_empty() {
        Ok(Pattern::point(subject))
    } else {
        Ok(Pattern::pattern(subject, converted_elements))
    }
}

/// Convert a JsValue to a Subject and any associated elements.
///
/// Returns (Subject, Vec<Pattern<Subject>>) where:
/// - For primitives: Subject with typed label, empty elements
/// - For arrays: Subject with "List" label, elements are converted array items
/// - For objects: Subject with "Map" label, elements are alternating key-value pairs
/// - For WasmSubject: extracted Subject, empty elements
fn js_value_to_subject_with_elements(
    value: &JsValue,
    index: &mut usize,
) -> Result<(Subject, Vec<Pattern<Subject>>), String> {
    // Handle null/undefined
    if value.is_null() || value.is_undefined() {
        return Err("Cannot convert null/undefined to Subject".to_string());
    }

    // Check for WasmSubject passthrough (wasm-bindgen object with identity property)
    if value.is_object() {
        let obj: &js_sys::Object = value.unchecked_ref();
        if js_sys::Reflect::has(obj, &JsValue::from_str("__wbg_ptr")).unwrap_or(false) {
            if let Ok(identity_js) = js_sys::Reflect::get(obj, &JsValue::from_str("identity")) {
                if identity_js.as_string().is_some() {
                    if let Some(subject) = try_extract_subject_from_wasm(obj) {
                        return Ok((subject, vec![]));
                    }
                }
            }
        }
    }

    // Boolean - use "Bool" for pattern-lisp compatibility
    if let Some(b) = value.as_bool() {
        let current_index = *index;
        *index += 1;

        let mut properties = HashMap::new();
        properties.insert(
            "value".to_string(),
            pattern_core::subject::Value::VBoolean(b),
        );

        let mut labels = HashSet::new();
        labels.insert("Bool".to_string());

        return Ok((
            Subject {
                identity: Symbol(format!("_{}", current_index)),
                labels,
                properties,
            },
            vec![],
        ));
    }

    // String
    if let Some(s) = value.as_string() {
        let current_index = *index;
        *index += 1;

        let mut properties = HashMap::new();
        properties.insert(
            "value".to_string(),
            pattern_core::subject::Value::VString(s),
        );

        let mut labels = HashSet::new();
        labels.insert("String".to_string());

        return Ok((
            Subject {
                identity: Symbol(format!("_{}", current_index)),
                labels,
                properties,
            },
            vec![],
        ));
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

        return Ok((
            Subject {
                identity: Symbol(format!("_{}", current_index)),
                labels,
                properties,
            },
            vec![],
        ));
    }

    // Array - create List pattern with elements as children
    if js_sys::Array::is_array(value) {
        let arr: &js_sys::Array = value.unchecked_ref();
        let mut elements = Vec::new();

        for i in 0..arr.length() {
            let item = arr.get(i);
            // Recursively convert each array item to Pattern<Subject>
            let item_pattern = js_value_to_pattern_subject(&item, index)?;
            elements.push(item_pattern);
        }

        // List decoration has no properties, just the label
        let mut labels = HashSet::new();
        labels.insert("List".to_string());

        return Ok((
            Subject {
                identity: Symbol("".to_string()),
                labels,
                properties: HashMap::new(),
            },
            elements,
        ));
    }

    // Plain object - convert to Map pattern with alternating key-value elements
    if value.is_object() {
        let obj: &js_sys::Object = value.unchecked_ref();
        let keys = js_sys::Object::keys(obj);
        let mut elements = Vec::new();

        for i in 0..keys.length() {
            let key = keys.get(i);
            if let Some(key_str) = key.as_string() {
                // Add key as String pattern
                let key_index = *index;
                *index += 1;

                let mut key_properties = HashMap::new();
                key_properties.insert(
                    "value".to_string(),
                    pattern_core::subject::Value::VString(key_str.clone()),
                );
                let mut key_labels = HashSet::new();
                key_labels.insert("String".to_string());

                elements.push(Pattern::point(Subject {
                    identity: Symbol(format!("_{}", key_index)),
                    labels: key_labels,
                    properties: key_properties,
                }));

                // Add value - recursively convert
                if let Ok(val) = js_sys::Reflect::get(obj, &key) {
                    let val_pattern = js_value_to_pattern_subject(&val, index)?;
                    elements.push(val_pattern);
                }
            }
        }

        // Map decoration has no properties, just the label
        let mut labels = HashSet::new();
        labels.insert("Map".to_string());

        return Ok((
            Subject {
                identity: Symbol("".to_string()),
                labels,
                properties: HashMap::new(),
            },
            elements,
        ));
    }

    Err(format!("Cannot convert value to Subject: {:?}", value))
}

/// Try to extract a WasmPattern from a JsValue.
/// Returns None if the value is not a valid WasmPattern.
fn try_extract_wasm_pattern(value: &JsValue) -> Option<WasmPattern> {
    // WasmPattern stores Pattern<JsValue> internally
    // We need to reconstruct it from the JS representation

    if !value.is_object() {
        return None;
    }

    let obj: &js_sys::Object = value.unchecked_ref();

    // Get the value at this node
    let node_value = js_sys::Reflect::get(obj, &JsValue::from_str("value")).ok()?;

    // Create the pattern with this value
    let mut pattern = WasmPattern::point(node_value);

    // Get the elements array
    let elements_js = js_sys::Reflect::get(obj, &JsValue::from_str("elements")).ok()?;
    if js_sys::Array::is_array(&elements_js) {
        let arr: &js_sys::Array = elements_js.unchecked_ref();
        for i in 0..arr.length() {
            let elem = arr.get(i);
            if let Some(child_pattern) = try_extract_wasm_pattern(&elem) {
                pattern.add_element(child_pattern);
            }
        }
    }

    Some(pattern)
}

/// Try to extract a Subject from a wasm-bindgen object that looks like a WasmSubject.
fn try_extract_subject_from_wasm(obj: &js_sys::Object) -> Option<Subject> {
    // Get identity
    let identity_js = js_sys::Reflect::get(obj, &JsValue::from_str("identity")).ok()?;
    let identity = identity_js.as_string()?;

    // Get labels
    let labels_js = js_sys::Reflect::get(obj, &JsValue::from_str("labels")).ok()?;
    let mut labels = HashSet::new();
    if js_sys::Array::is_array(&labels_js) {
        let arr: &js_sys::Array = labels_js.unchecked_ref();
        for i in 0..arr.length() {
            if let Some(label) = arr.get(i).as_string() {
                labels.insert(label);
            }
        }
    }

    // Get properties
    let props_js = js_sys::Reflect::get(obj, &JsValue::from_str("properties"))
        .ok()
        .unwrap_or(JsValue::undefined());
    let properties = if props_js.is_object() && !props_js.is_null() {
        extract_subject_properties(&props_js).unwrap_or_default()
    } else {
        HashMap::new()
    };

    Some(Subject {
        identity: Symbol(identity),
        labels,
        properties,
    })
}

/// Extract Subject properties from a JS object.
/// Uses the canonical implementation from pattern_core where possible.
fn extract_subject_properties(
    obj: &JsValue,
) -> Result<HashMap<String, pattern_core::subject::Value>, String> {
    use crate::convert::js_object_to_value_map;
    js_object_to_value_map(obj)
}
