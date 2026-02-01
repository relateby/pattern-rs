//! Conversion layer between Rust Pattern<Subject> and JS Pattern<Subject>
//!
//! This module implements bidirectional conversion:
//! - Rust Pattern<Subject> → WasmPattern (containing WasmSubject instances)
//! - WasmPattern → Rust Pattern<Subject>
//!
//! The key insight is that WasmPattern wraps Pattern<JsValue>, and we store
//! actual WasmSubject instances (not plain objects) as the JsValue. This ensures
//! that `pattern.value instanceof Subject` is true for parsed patterns, matching
//! the behavior of manually constructed patterns.

use pattern_core::wasm::{WasmPattern, WasmSubject};
use pattern_core::{Pattern, Subject};
use wasm_bindgen::prelude::*;

/// Convert a Rust Pattern<Subject> to a WasmPattern.
///
/// Each Subject in the pattern becomes a WasmSubject instance stored as JsValue.
/// This ensures type consistency - parsed patterns have real Subject instances
/// just like manually constructed patterns.
///
/// # Arguments
/// * `pattern` - A Rust Pattern<Subject> (e.g., from gram_codec::parse_gram)
///
/// # Returns
/// A WasmPattern where each value is a WasmSubject instance
pub fn rust_pattern_to_wasm(pattern: &Pattern<Subject>) -> WasmPattern {
    // Convert the Subject to a WasmSubject, then to JsValue
    let wasm_subject = WasmSubject::from_subject(pattern.value().clone());
    let js_value: JsValue = wasm_subject.into();

    // Recursively convert child elements
    let js_elements: Vec<Pattern<JsValue>> = pattern
        .elements()
        .iter()
        .map(|elem| rust_pattern_to_wasm(elem).into_pattern())
        .collect();

    WasmPattern::from_pattern(Pattern::pattern(js_value, js_elements))
}

/// Convert a WasmPattern to a Rust Pattern<Subject>.
///
/// Extracts Subject data from each WasmSubject instance in the pattern.
/// Handles both WasmSubject instances and plain objects with Subject shape.
///
/// # Arguments
/// * `wasm_pattern` - A WasmPattern where values should be Subject representations
///
/// # Returns
/// * `Ok(Pattern<Subject>)` - Successfully converted pattern
/// * `Err(String)` - Conversion error message
pub fn wasm_pattern_to_rust(wasm_pattern: &WasmPattern) -> Result<Pattern<Subject>, String> {
    let inner = wasm_pattern.as_pattern();

    // Convert the JsValue to Subject
    let subject = js_value_to_subject(inner.value())?;

    // Recursively convert child elements
    let rust_elements: Result<Vec<Pattern<Subject>>, String> = inner
        .elements()
        .iter()
        .map(|elem| {
            let child_wasm = WasmPattern::from_pattern(elem.clone());
            wasm_pattern_to_rust(&child_wasm)
        })
        .collect();

    Ok(Pattern::pattern(subject, rust_elements?))
}

/// Convert a JsValue to a Rust Subject.
///
/// This handles two cases:
/// 1. WasmSubject instance - extract using WasmSubject methods
/// 2. Plain JS object with Subject shape - parse fields manually
///
/// # Arguments
/// * `js_value` - A JsValue that should represent a Subject
///
/// # Returns
/// * `Ok(Subject)` - Successfully converted Subject
/// * `Err(String)` - Conversion error message
fn js_value_to_subject(js_value: &JsValue) -> Result<Subject, String> {
    // First, try to detect if this is a WasmSubject instance by checking
    // if it has the expected wasm-bindgen internal structure.
    // WasmSubject instances have a __wbg_ptr property.
    if js_value.is_object() {
        let obj: &js_sys::Object = js_value.unchecked_ref();

        // Check if this looks like a WasmSubject (has __wbg_ptr)
        // If so, we can use the WasmSubject's getter methods via JS interop
        if js_sys::Reflect::has(obj, &JsValue::from_str("__wbg_ptr")).unwrap_or(false) {
            // This is likely a WasmSubject instance
            // We need to call its getter methods to extract the data
            return extract_subject_from_wasm_instance(obj);
        }

        // Otherwise, try to parse as a plain object with Subject shape
        return extract_subject_from_plain_object(obj);
    }

    Err("Expected an object representing a Subject".to_string())
}

/// Extract Subject data from a WasmSubject instance.
///
/// Calls the getter methods (identity, labels, properties) on the WasmSubject.
fn extract_subject_from_wasm_instance(obj: &js_sys::Object) -> Result<Subject, String> {
    use pattern_core::Symbol;

    // Get identity via getter
    let identity_js = js_sys::Reflect::get(obj, &JsValue::from_str("identity"))
        .map_err(|_| "Failed to get identity from WasmSubject")?;
    let identity = identity_js.as_string().ok_or("Identity is not a string")?;

    // Get labels via getter (returns JS array)
    let labels_js = js_sys::Reflect::get(obj, &JsValue::from_str("labels"))
        .map_err(|_| "Failed to get labels from WasmSubject")?;
    let labels = js_array_to_string_set(&labels_js)?;

    // Get properties via getter (returns JS object)
    let properties_js = js_sys::Reflect::get(obj, &JsValue::from_str("properties"))
        .map_err(|_| "Failed to get properties from WasmSubject")?;
    let properties = js_object_to_value_map(&properties_js)?;

    Ok(Subject {
        identity: Symbol(identity),
        labels,
        properties,
    })
}

/// Extract Subject data from a plain JS object with Subject shape.
///
/// Expects: { identity: string, labels: string[], properties: object }
fn extract_subject_from_plain_object(obj: &js_sys::Object) -> Result<Subject, String> {
    use pattern_core::Symbol;

    // Extract identity
    let identity_js = js_sys::Reflect::get(obj, &JsValue::from_str("identity"))
        .map_err(|_| "Missing identity field")?;
    let identity = identity_js.as_string().ok_or("Identity must be a string")?;

    // Extract labels
    let labels_js = js_sys::Reflect::get(obj, &JsValue::from_str("labels"))
        .map_err(|_| "Missing labels field")?;
    let labels = js_array_to_string_set(&labels_js)?;

    // Extract properties
    let properties_js = js_sys::Reflect::get(obj, &JsValue::from_str("properties"))
        .map_err(|_| "Missing properties field")?;
    let properties = js_object_to_value_map(&properties_js)?;

    Ok(Subject {
        identity: Symbol(identity),
        labels,
        properties,
    })
}

/// Convert a JS array to HashSet<String>.
fn js_array_to_string_set(arr: &JsValue) -> Result<std::collections::HashSet<String>, String> {
    if !js_sys::Array::is_array(arr) {
        return Err("Expected an array for labels".to_string());
    }

    let arr: &js_sys::Array = arr.unchecked_ref();
    let mut result = std::collections::HashSet::new();

    for i in 0..arr.length() {
        let item = arr.get(i);
        let s = item
            .as_string()
            .ok_or_else(|| format!("Label at index {} is not a string", i))?;
        result.insert(s);
    }

    Ok(result)
}

/// Convert a JS object to HashMap<String, Value>.
pub fn js_object_to_value_map(
    obj: &JsValue,
) -> Result<std::collections::HashMap<String, pattern_core::subject::Value>, String> {
    use std::collections::HashMap;

    if !obj.is_object() || obj.is_null() {
        return Err("Expected an object for properties".to_string());
    }

    let mut map = HashMap::new();
    let keys = js_sys::Object::keys(obj.unchecked_ref());

    for i in 0..keys.length() {
        let key = keys.get(i);
        let key_str = key
            .as_string()
            .ok_or_else(|| "Property key is not a string".to_string())?;
        let value = js_sys::Reflect::get(obj, &key).map_err(|_| "Failed to get property value")?;
        let rust_value = js_value_to_value(&value)?;
        map.insert(key_str, rust_value);
    }

    Ok(map)
}

/// Convert a JsValue to a Rust Value enum.
fn js_value_to_value(js: &JsValue) -> Result<pattern_core::subject::Value, String> {
    use pattern_core::subject::Value;

    // Check for null/undefined
    if js.is_null() || js.is_undefined() {
        return Err("Cannot convert null/undefined to Value".to_string());
    }

    // Check for boolean
    if let Some(b) = js.as_bool() {
        return Ok(Value::VBoolean(b));
    }

    // Check for string
    if let Some(s) = js.as_string() {
        return Ok(Value::VString(s));
    }

    // Check for number
    if let Some(n) = js.as_f64() {
        // Check if it's a safe integer
        if n.fract() == 0.0 && n >= i64::MIN as f64 && n <= i64::MAX as f64 {
            return Ok(Value::VInteger(n as i64));
        }
        return Ok(Value::VDecimal(n));
    }

    // Check for array
    if js_sys::Array::is_array(js) {
        let arr: &js_sys::Array = js.unchecked_ref();
        let mut values = Vec::with_capacity(arr.length() as usize);
        for i in 0..arr.length() {
            let item = arr.get(i);
            values.push(js_value_to_value(&item)?);
        }
        return Ok(Value::VArray(values));
    }

    // Check for object (could be symbol, range, measurement, tagged string, or map)
    if js.is_object() {
        let obj: &js_sys::Object = js.unchecked_ref();

        // Check for symbol: { _type: 'symbol', value: string }
        if let Ok(type_field) = js_sys::Reflect::get(obj, &JsValue::from_str("_type")) {
            if let Some(type_str) = type_field.as_string() {
                if type_str == "symbol" {
                    let symbol_value = js_sys::Reflect::get(obj, &JsValue::from_str("value"))
                        .ok()
                        .and_then(|v| v.as_string())
                        .ok_or_else(|| "Symbol missing value field".to_string())?;
                    return Ok(Value::VSymbol(symbol_value));
                }
            }
        }

        // Check for range: { lower?: number, upper?: number }
        let has_lower = js_sys::Reflect::has(obj, &JsValue::from_str("lower")).unwrap_or(false);
        let has_upper = js_sys::Reflect::has(obj, &JsValue::from_str("upper")).unwrap_or(false);
        if has_lower || has_upper {
            let lower = js_sys::Reflect::get(obj, &JsValue::from_str("lower"))
                .ok()
                .and_then(|v| {
                    if v.is_null() || v.is_undefined() {
                        None
                    } else {
                        v.as_f64()
                    }
                });
            let upper = js_sys::Reflect::get(obj, &JsValue::from_str("upper"))
                .ok()
                .and_then(|v| {
                    if v.is_null() || v.is_undefined() {
                        None
                    } else {
                        v.as_f64()
                    }
                });

            if lower.is_some() || upper.is_some() {
                return Ok(Value::VRange(pattern_core::subject::RangeValue {
                    lower,
                    upper,
                }));
            }
        }

        // Check for measurement: { value: number, unit: string }
        let has_value = js_sys::Reflect::has(obj, &JsValue::from_str("value")).unwrap_or(false);
        let has_unit = js_sys::Reflect::has(obj, &JsValue::from_str("unit")).unwrap_or(false);
        if has_value && has_unit {
            let value = js_sys::Reflect::get(obj, &JsValue::from_str("value"))
                .ok()
                .and_then(|v| v.as_f64());
            let unit = js_sys::Reflect::get(obj, &JsValue::from_str("unit"))
                .ok()
                .and_then(|v| v.as_string());

            if let (Some(v), Some(u)) = (value, unit) {
                return Ok(Value::VMeasurement { unit: u, value: v });
            }
        }

        // Check for tagged string: { tag: string, content: string }
        let has_tag = js_sys::Reflect::has(obj, &JsValue::from_str("tag")).unwrap_or(false);
        let has_content = js_sys::Reflect::has(obj, &JsValue::from_str("content")).unwrap_or(false);
        if has_tag && has_content {
            let tag = js_sys::Reflect::get(obj, &JsValue::from_str("tag"))
                .ok()
                .and_then(|v| v.as_string());
            let content = js_sys::Reflect::get(obj, &JsValue::from_str("content"))
                .ok()
                .and_then(|v| v.as_string());

            if let (Some(t), Some(c)) = (tag, content) {
                return Ok(Value::VTaggedString { tag: t, content: c });
            }
        }

        // Default: treat as VMap
        let map = js_object_to_value_map(js)?;
        return Ok(Value::VMap(map));
    }

    Err(format!("Cannot convert JS value to Value: {:?}", js))
}
