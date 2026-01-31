//! WASM bindings for Pattern-Core using wasm-bindgen
//!
//! This module provides JavaScript/TypeScript-friendly bindings for pattern-core,
//! enabling web and Node.js developers to programmatically construct and operate
//! on Pattern and Subject instances.
//!
//! # Usage in JavaScript/TypeScript
//!
//! ```javascript
//! import init, { Pattern, Subject, Value } from 'pattern-core';
//!
//! await init();
//!
//! // Create an atomic pattern
//! const atomic = Pattern.point("hello");
//!
//! // Create a pattern with Subject value
//! const subject = Subject.new("alice", ["Person"], { name: Value.string("Alice") });
//! const pattern = Pattern.point(subject);
//!
//! // Validate a pattern
//! const result = pattern.validate({ maxDepth: 10 });
//! if (result._tag === 'Left') {
//!     console.error('Validation failed:', result.left);
//! }
//! ```
//!
//! # Either-like Return Values
//!
//! Fallible operations return an Either-like value compatible with effect-ts:
//! - Success: `{ _tag: 'Right', right: T }`
//! - Failure: `{ _tag: 'Left', left: E }`
//!
//! This enables seamless integration with functional programming libraries
//! without requiring any wrapper code.

use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Re-export core types for internal use
use crate::pattern::{Pattern, ValidationError};
use crate::subject::{RangeValue, Subject, Symbol, Value};

// ============================================================================
// Module Structure
// ============================================================================
//
// This module is organized into the following sections:
//
// 1. Error/Result Handling (Either-like)
//    - JsEither type for fallible operations
//    - Conversion helpers for Result → Either
//
// 2. Value Conversion Helpers
//    - JsValue ↔ Rust Value conversions
//    - Primitive type handling
//    - Option handling
//
// 3. Value Factories (Phase 3)
//    - JsValue factory methods for all Value variants
//
// 4. Subject Bindings (Phase 3)
//    - Subject constructor and accessors
//
// 5. Pattern Bindings (Phase 3)
//    - Pattern constructors, accessors, and operations
//
// 6. Validation/Analysis Types (Phase 4)
//    - ValidationRules, StructureAnalysis bindings
//
// ============================================================================

// ============================================================================
// 1. Either-like Return Shape (T006)
// ============================================================================
//
// Fallible operations return an Either-like value compatible with effect-ts:
// - Success: { _tag: 'Right', right: T }
// - Failure: { _tag: 'Left', left: E }
//
// This matches the shape of Effect's Either type, enabling:
// - Direct use with Either.match, Either.map, etc.
// - Pattern matching on _tag in TypeScript
// - No wrapper code required for effect-ts integration

/// Create an Either-like Right value (success case).
///
/// Returns a JsValue with shape: `{ _tag: 'Right', right: value }`
pub fn either_right(value: JsValue) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("_tag"),
        &JsValue::from_str("Right"),
    )
    .expect("set _tag");
    js_sys::Reflect::set(&obj, &JsValue::from_str("right"), &value).expect("set right");
    obj.into()
}

/// Create an Either-like Left value (failure case).
///
/// Returns a JsValue with shape: `{ _tag: 'Left', left: error }`
pub fn either_left(error: JsValue) -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &JsValue::from_str("_tag"), &JsValue::from_str("Left"))
        .expect("set _tag");
    js_sys::Reflect::set(&obj, &JsValue::from_str("left"), &error).expect("set left");
    obj.into()
}

/// Convert a Rust Result into an Either-like JsValue.
///
/// - `Ok(value)` → `{ _tag: 'Right', right: value }`
/// - `Err(error)` → `{ _tag: 'Left', left: error }`
pub fn result_to_either<T, E>(result: Result<T, E>) -> JsValue
where
    T: Into<JsValue>,
    E: Into<JsValue>,
{
    match result {
        Ok(value) => either_right(value.into()),
        Err(error) => either_left(error.into()),
    }
}

/// Convert a ValidationError to a JsValue object.
///
/// Returns: `{ message: string, ruleViolated: string, location: string[] }`
pub fn validation_error_to_js(error: &ValidationError) -> JsValue {
    let obj = js_sys::Object::new();

    // Set message
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("message"),
        &JsValue::from_str(&error.message),
    )
    .expect("set message");

    // Set ruleViolated
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("ruleViolated"),
        &JsValue::from_str(&error.rule_violated),
    )
    .expect("set ruleViolated");

    // Set location as array of strings
    let location_arr = js_sys::Array::new();
    for loc in &error.location {
        location_arr.push(&JsValue::from_str(loc));
    }
    js_sys::Reflect::set(&obj, &JsValue::from_str("location"), &location_arr)
        .expect("set location");

    obj.into()
}

// ============================================================================
// 2. Value Conversion Helpers (T005)
// ============================================================================
//
// These helpers convert between JavaScript values and Rust types at the
// WASM boundary. They handle:
// - Primitives: string, number (int/float), boolean
// - Option<T>: null/undefined ↔ None
// - Arrays and objects for complex Value types
// - The Value enum with all its variants

/// Convert a JsValue to a Rust String.
///
/// Returns `None` if the value is not a string.
pub fn js_to_string(value: &JsValue) -> Option<String> {
    value.as_string()
}

/// Convert a JsValue to a Rust i64.
///
/// Returns `None` if the value is not a number or cannot be safely converted.
pub fn js_to_i64(value: &JsValue) -> Option<i64> {
    value.as_f64().map(|f| f as i64)
}

/// Convert a JsValue to a Rust f64.
///
/// Returns `None` if the value is not a number.
pub fn js_to_f64(value: &JsValue) -> Option<f64> {
    value.as_f64()
}

/// Convert a JsValue to a Rust bool.
///
/// Returns `None` if the value is not a boolean.
pub fn js_to_bool(value: &JsValue) -> Option<bool> {
    value.as_bool()
}

/// Check if a JsValue is null or undefined.
pub fn js_is_null_or_undefined(value: &JsValue) -> bool {
    value.is_null() || value.is_undefined()
}

/// Convert a JsValue to an Option<T> using a conversion function.
///
/// Returns `None` if the value is null/undefined, otherwise applies the converter.
pub fn js_to_option<T, F>(value: &JsValue, convert: F) -> Option<T>
where
    F: FnOnce(&JsValue) -> Option<T>,
{
    if js_is_null_or_undefined(value) {
        None
    } else {
        convert(value)
    }
}

/// Convert a JsValue (JS object with string keys) to a HashMap<String, Value>.
///
/// Used for Subject properties and Value::VMap.
pub fn js_object_to_value_map(obj: &JsValue) -> Result<HashMap<String, Value>, String> {
    if !obj.is_object() || obj.is_null() {
        return Err("Expected an object".to_string());
    }

    let mut map = HashMap::new();
    let keys = js_sys::Object::keys(obj.unchecked_ref());

    for i in 0..keys.length() {
        let key = keys.get(i);
        let key_str = key
            .as_string()
            .ok_or_else(|| "Object key is not a string".to_string())?;
        let value = js_sys::Reflect::get(obj, &key).map_err(|_| "Failed to get property")?;
        let rust_value = js_to_value(&value)?;
        map.insert(key_str, rust_value);
    }

    Ok(map)
}

/// Convert a JsValue to a Rust Value enum.
///
/// Handles all Value variants by inspecting the JS value type and structure.
/// For complex types (range, measurement, tagged string), expects objects
/// with specific shapes.
pub fn js_to_value(js: &JsValue) -> Result<Value, String> {
    // Check for null/undefined first
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

    // Check for number (try integer first, then decimal)
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
            values.push(js_to_value(&item)?);
        }
        return Ok(Value::VArray(values));
    }

    // Check for object (could be map, range, measurement, or tagged string)
    if js.is_object() {
        let obj: &js_sys::Object = js.unchecked_ref();

        // Check for range: { lower?: number, upper?: number }
        let has_lower = js_sys::Reflect::has(obj, &JsValue::from_str("lower")).unwrap_or(false);
        let has_upper = js_sys::Reflect::has(obj, &JsValue::from_str("upper")).unwrap_or(false);
        if has_lower || has_upper {
            let lower = js_sys::Reflect::get(obj, &JsValue::from_str("lower"))
                .ok()
                .and_then(|v| js_to_option(&v, js_to_f64));
            let upper = js_sys::Reflect::get(obj, &JsValue::from_str("upper"))
                .ok()
                .and_then(|v| js_to_option(&v, js_to_f64));

            // Only treat as range if at least one bound is a number
            if lower.is_some() || upper.is_some() {
                return Ok(Value::VRange(RangeValue { lower, upper }));
            }
        }

        // Check for measurement: { value: number, unit: string }
        let has_value = js_sys::Reflect::has(obj, &JsValue::from_str("value")).unwrap_or(false);
        let has_unit = js_sys::Reflect::has(obj, &JsValue::from_str("unit")).unwrap_or(false);
        if has_value && has_unit {
            let value = js_sys::Reflect::get(obj, &JsValue::from_str("value"))
                .ok()
                .and_then(|v| js_to_f64(&v));
            let unit = js_sys::Reflect::get(obj, &JsValue::from_str("unit"))
                .ok()
                .and_then(|v| js_to_string(&v));

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
                .and_then(|v| js_to_string(&v));
            let content = js_sys::Reflect::get(obj, &JsValue::from_str("content"))
                .ok()
                .and_then(|v| js_to_string(&v));

            if let (Some(t), Some(c)) = (tag, content) {
                return Ok(Value::VTaggedString { tag: t, content: c });
            }
        }

        // Check for symbol: { _type: 'symbol', value: string }
        let type_field = js_sys::Reflect::get(obj, &JsValue::from_str("_type"))
            .ok()
            .and_then(|v| js_to_string(&v));
        if type_field.as_deref() == Some("symbol") {
            let symbol_value = js_sys::Reflect::get(obj, &JsValue::from_str("value"))
                .ok()
                .and_then(|v| js_to_string(&v));
            if let Some(s) = symbol_value {
                return Ok(Value::VSymbol(s));
            }
        }

        // Default: treat as VMap
        let map = js_object_to_value_map(js)?;
        return Ok(Value::VMap(map));
    }

    Err(format!("Cannot convert JS value to Value: {:?}", js))
}

/// Convert a Rust Value to a JsValue.
///
/// Handles all Value variants, converting to appropriate JS types.
pub fn value_to_js(value: &Value) -> JsValue {
    match value {
        Value::VString(s) => JsValue::from_str(s),
        Value::VInteger(i) => JsValue::from_f64(*i as f64),
        Value::VDecimal(d) => JsValue::from_f64(*d),
        Value::VBoolean(b) => JsValue::from_bool(*b),
        Value::VSymbol(s) => {
            // Return as { _type: 'symbol', value: string }
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(
                &obj,
                &JsValue::from_str("_type"),
                &JsValue::from_str("symbol"),
            )
            .expect("set _type");
            js_sys::Reflect::set(&obj, &JsValue::from_str("value"), &JsValue::from_str(s))
                .expect("set value");
            obj.into()
        }
        Value::VTaggedString { tag, content } => {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &JsValue::from_str("tag"), &JsValue::from_str(tag))
                .expect("set tag");
            js_sys::Reflect::set(
                &obj,
                &JsValue::from_str("content"),
                &JsValue::from_str(content),
            )
            .expect("set content");
            obj.into()
        }
        Value::VArray(arr) => {
            let js_arr = js_sys::Array::new();
            for item in arr {
                js_arr.push(&value_to_js(item));
            }
            js_arr.into()
        }
        Value::VMap(map) => {
            let obj = js_sys::Object::new();
            for (key, val) in map {
                js_sys::Reflect::set(&obj, &JsValue::from_str(key), &value_to_js(val))
                    .expect("set map entry");
            }
            obj.into()
        }
        Value::VRange(range) => {
            let obj = js_sys::Object::new();
            if let Some(lower) = range.lower {
                js_sys::Reflect::set(&obj, &JsValue::from_str("lower"), &JsValue::from_f64(lower))
                    .expect("set lower");
            }
            if let Some(upper) = range.upper {
                js_sys::Reflect::set(&obj, &JsValue::from_str("upper"), &JsValue::from_f64(upper))
                    .expect("set upper");
            }
            obj.into()
        }
        Value::VMeasurement { unit, value } => {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &JsValue::from_str("unit"), &JsValue::from_str(unit))
                .expect("set unit");
            js_sys::Reflect::set(
                &obj,
                &JsValue::from_str("value"),
                &JsValue::from_f64(*value),
            )
            .expect("set value");
            obj.into()
        }
    }
}

/// Convert a JS array to a Vec<String> (for labels).
pub fn js_array_to_strings(arr: &JsValue) -> Result<Vec<String>, String> {
    if !js_sys::Array::is_array(arr) {
        return Err("Expected an array".to_string());
    }

    let arr: &js_sys::Array = arr.unchecked_ref();
    let mut result = Vec::with_capacity(arr.length() as usize);

    for i in 0..arr.length() {
        let item = arr.get(i);
        let s = item
            .as_string()
            .ok_or_else(|| format!("Array item at index {} is not a string", i))?;
        result.push(s);
    }

    Ok(result)
}

/// Convert a Vec<String> to a JS array.
pub fn strings_to_js_array(strings: &[String]) -> JsValue {
    let arr = js_sys::Array::new();
    for s in strings {
        arr.push(&JsValue::from_str(s));
    }
    arr.into()
}

/// Convert a HashMap<String, Value> to a JS object.
pub fn value_map_to_js(map: &HashMap<String, Value>) -> JsValue {
    let obj = js_sys::Object::new();
    for (key, val) in map {
        js_sys::Reflect::set(&obj, &JsValue::from_str(key), &value_to_js(val))
            .expect("set property");
    }
    obj.into()
}

// ============================================================================
// 3. Value Factories (T009 - Phase 3)
// ============================================================================
//
// These factory methods provide JavaScript-friendly constructors for all Value variants.
// They accept JsValue arguments and convert them to the appropriate Rust Value types.

#[wasm_bindgen]
pub struct ValueFactory;

#[wasm_bindgen]
impl ValueFactory {
    /// Create a VString value.
    ///
    /// # Arguments
    /// * `s` - A JavaScript string
    ///
    /// # Returns
    /// A JsValue representing a Value::VString
    #[wasm_bindgen(js_name = string)]
    pub fn string(s: &str) -> JsValue {
        value_to_js(&Value::VString(s.to_string()))
    }

    /// Create a VInteger value.
    ///
    /// # Arguments
    /// * `n` - A JavaScript number (will be converted to i64)
    ///
    /// # Returns
    /// A JsValue representing a Value::VInteger
    #[wasm_bindgen(js_name = int)]
    pub fn int(n: f64) -> JsValue {
        value_to_js(&Value::VInteger(n as i64))
    }

    /// Create a VDecimal value.
    ///
    /// # Arguments
    /// * `n` - A JavaScript number (f64)
    ///
    /// # Returns
    /// A JsValue representing a Value::VDecimal
    #[wasm_bindgen(js_name = decimal)]
    pub fn decimal(n: f64) -> JsValue {
        value_to_js(&Value::VDecimal(n))
    }

    /// Create a VBoolean value.
    ///
    /// # Arguments
    /// * `b` - A JavaScript boolean
    ///
    /// # Returns
    /// A JsValue representing a Value::VBoolean
    #[wasm_bindgen(js_name = boolean)]
    pub fn boolean(b: bool) -> JsValue {
        value_to_js(&Value::VBoolean(b))
    }

    /// Create a VSymbol value.
    ///
    /// # Arguments
    /// * `s` - A JavaScript string representing the symbol
    ///
    /// # Returns
    /// A JsValue representing a Value::VSymbol
    #[wasm_bindgen(js_name = symbol)]
    pub fn symbol(s: &str) -> JsValue {
        value_to_js(&Value::VSymbol(s.to_string()))
    }

    /// Create a VArray value.
    ///
    /// # Arguments
    /// * `arr` - A JavaScript array of values
    ///
    /// # Returns
    /// A JsValue representing a Value::VArray, or throws an error
    #[wasm_bindgen(js_name = array)]
    pub fn array(arr: &JsValue) -> Result<JsValue, JsValue> {
        if !js_sys::Array::is_array(arr) {
            return Err(JsValue::from_str("Expected an array"));
        }

        let js_arr: &js_sys::Array = arr.unchecked_ref();
        let mut values = Vec::with_capacity(js_arr.length() as usize);

        for i in 0..js_arr.length() {
            let item = js_arr.get(i);
            let val = js_to_value(&item).map_err(|e| JsValue::from_str(&e))?;
            values.push(val);
        }

        Ok(value_to_js(&Value::VArray(values)))
    }

    /// Create a VMap value.
    ///
    /// # Arguments
    /// * `obj` - A JavaScript object with string keys and value values
    ///
    /// # Returns
    /// A JsValue representing a Value::VMap, or throws an error
    #[wasm_bindgen(js_name = map)]
    pub fn map(obj: &JsValue) -> Result<JsValue, JsValue> {
        let map = js_object_to_value_map(obj).map_err(|e| JsValue::from_str(&e))?;
        Ok(value_to_js(&Value::VMap(map)))
    }

    /// Create a VRange value.
    ///
    /// # Arguments
    /// * `lower` - Optional lower bound (null/undefined for unbounded)
    /// * `upper` - Optional upper bound (null/undefined for unbounded)
    ///
    /// # Returns
    /// A JsValue representing a Value::VRange
    #[wasm_bindgen(js_name = range)]
    pub fn range(lower: JsValue, upper: JsValue) -> JsValue {
        let lower_val = js_to_option(&lower, js_to_f64);
        let upper_val = js_to_option(&upper, js_to_f64);
        value_to_js(&Value::VRange(RangeValue {
            lower: lower_val,
            upper: upper_val,
        }))
    }

    /// Create a VMeasurement value.
    ///
    /// # Arguments
    /// * `value` - The numeric value
    /// * `unit` - The unit string (e.g., "kg", "m", "s")
    ///
    /// # Returns
    /// A JsValue representing a Value::VMeasurement
    #[wasm_bindgen(js_name = measurement)]
    pub fn measurement(value: f64, unit: &str) -> JsValue {
        value_to_js(&Value::VMeasurement {
            unit: unit.to_string(),
            value,
        })
    }
}

// ============================================================================
// 4. Subject Bindings (T010 - Phase 3)
// ============================================================================
//
// WASM bindings for Subject constructors and accessors.
// JavaScript/TypeScript developers can create and inspect Subject instances.

/// WASM binding for Subject.
///
/// Provides constructors and accessors for Subject instances from JavaScript/TypeScript.
#[wasm_bindgen]
pub struct WasmSubject {
    inner: Subject,
}

#[wasm_bindgen]
impl WasmSubject {
    /// Create a new Subject.
    ///
    /// # Arguments
    /// * `identity` - A string representing the subject's identity symbol
    /// * `labels` - A JavaScript array of label strings (can be empty)
    /// * `properties` - A JavaScript object with string keys and Value values (can be empty object)
    ///
    /// # Returns
    /// A new WasmSubject instance, or throws an error
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const subject = WasmSubject.new(
    ///     "alice",
    ///     ["Person", "User"],
    ///     { name: Value.string("Alice"), age: Value.int(30) }
    /// );
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(
        identity: &str,
        labels: &JsValue,
        properties: &JsValue,
    ) -> Result<WasmSubject, JsValue> {
        // Parse labels
        let labels_vec = js_array_to_strings(labels).map_err(|e| JsValue::from_str(&e))?;
        let labels_set: std::collections::HashSet<String> = labels_vec.into_iter().collect();

        // Parse properties
        let props = js_object_to_value_map(properties).map_err(|e| JsValue::from_str(&e))?;

        Ok(WasmSubject {
            inner: Subject {
                identity: Symbol(identity.to_string()),
                labels: labels_set,
                properties: props,
            },
        })
    }

    /// Get the identity symbol as a string.
    ///
    /// # Returns
    /// The identity symbol string
    #[wasm_bindgen(getter)]
    pub fn identity(&self) -> String {
        self.inner.identity.0.clone()
    }

    /// Get the labels as a JavaScript array of strings.
    ///
    /// # Returns
    /// A JavaScript array containing all label strings
    #[wasm_bindgen(getter)]
    pub fn labels(&self) -> JsValue {
        let labels_vec: Vec<String> = self.inner.labels.iter().cloned().collect();
        strings_to_js_array(&labels_vec)
    }

    /// Get the properties as a JavaScript object.
    ///
    /// # Returns
    /// A JavaScript object mapping property keys to Value JsValues
    #[wasm_bindgen(getter)]
    pub fn properties(&self) -> JsValue {
        value_map_to_js(&self.inner.properties)
    }
}

// Conversion helpers for WasmSubject ↔ JsValue
impl WasmSubject {
    /// Convert this WasmSubject to a JsValue for use in patterns.
    ///
    /// This allows WasmSubject instances to be stored in Pattern<JsValue>.
    pub fn to_js_value(&self) -> JsValue {
        let obj = js_sys::Object::new();

        // Set identity
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("identity"),
            &JsValue::from_str(&self.inner.identity.0),
        )
        .expect("set identity");

        // Set labels as array
        let labels_arr = js_sys::Array::new();
        for label in &self.inner.labels {
            labels_arr.push(&JsValue::from_str(label));
        }
        js_sys::Reflect::set(&obj, &JsValue::from_str("labels"), &labels_arr).expect("set labels");

        // Set properties
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("properties"),
            &value_map_to_js(&self.inner.properties),
        )
        .expect("set properties");

        // Add a type marker so we can identify Subjects
        js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("_type"),
            &JsValue::from_str("Subject"),
        )
        .expect("set _type");

        obj.into()
    }

    /// Try to convert a JsValue back to a WasmSubject.
    ///
    /// Returns None if the JsValue is not a valid Subject representation.
    pub fn from_js_value(value: &JsValue) -> Option<Self> {
        if !value.is_object() {
            return None;
        }

        let obj: &js_sys::Object = value.unchecked_ref();

        // Check for type marker
        let type_marker = js_sys::Reflect::get(obj, &JsValue::from_str("_type")).ok()?;
        if type_marker.as_string()? != "Subject" {
            return None;
        }

        // Extract identity
        let identity_js = js_sys::Reflect::get(obj, &JsValue::from_str("identity")).ok()?;
        let identity = identity_js.as_string()?;

        // Extract labels
        let labels_js = js_sys::Reflect::get(obj, &JsValue::from_str("labels")).ok()?;
        let labels_vec = js_array_to_strings(&labels_js).ok()?;
        let labels_set: std::collections::HashSet<String> = labels_vec.into_iter().collect();

        // Extract properties
        let properties_js = js_sys::Reflect::get(obj, &JsValue::from_str("properties")).ok()?;
        let properties = js_object_to_value_map(&properties_js).ok()?;

        Some(WasmSubject {
            inner: Subject {
                identity: Symbol(identity),
                labels: labels_set,
                properties,
            },
        })
    }
}

// ============================================================================
// 5. Pattern Bindings (T007, T008, T011 - Phase 3)
// ============================================================================
//
// WASM bindings for Pattern constructors, accessors, and operations.
// JavaScript/TypeScript developers can create and manipulate Pattern<V> instances.
//
// CRITICAL: Pattern<V> is generic over ANY value type V. The Python binding stores
// PyAny (any Python object). The WASM binding MUST mirror this by storing JsValue
// (any JavaScript value). This allows patterns to hold primitives, objects, Subjects,
// or even other Patterns (nesting).

/// WASM binding for Pattern<JsValue>.
///
/// Provides constructors and accessors for Pattern instances from JavaScript/TypeScript.
/// This binding wraps Pattern<JsValue>, allowing patterns to hold any JavaScript value:
/// primitives, objects, WasmSubject instances, or even other WasmPattern instances (nesting).
///
/// This design matches the Python binding which stores PyAny (any Python object).
#[wasm_bindgen]
pub struct WasmPattern {
    inner: Pattern<JsValue>,
}

#[wasm_bindgen]
impl WasmPattern {
    // ========================================================================
    // Constructors (T007)
    // ========================================================================

    /// Create an atomic pattern from any JavaScript value.
    ///
    /// Accepts any JavaScript value: primitives (string, number, boolean),
    /// objects, WasmSubject instances, or even other WasmPattern instances (for nesting).
    ///
    /// This matches the Python binding which accepts PyAny (any Python object).
    ///
    /// # Arguments
    /// * `value` - Any JavaScript value
    ///
    /// # Returns
    /// A new atomic WasmPattern containing that value
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// // Primitives
    /// const p1 = WasmPattern.point("hello");
    /// const p2 = WasmPattern.point(42);
    /// const p3 = WasmPattern.point(true);
    ///
    /// // Subject
    /// const subject = new WasmSubject("alice", [], {});
    /// const p4 = WasmPattern.point(subject);
    ///
    /// // Nesting - Pattern<Pattern<V>>
    /// const p5 = WasmPattern.point(p1);
    /// ```
    #[wasm_bindgen(js_name = point)]
    pub fn point(value: JsValue) -> WasmPattern {
        WasmPattern {
            inner: Pattern::point(value),
        }
    }

    /// Alias for point(). Create an atomic pattern from any JavaScript value.
    ///
    /// This is identical to `point()` - just a different name following functional
    /// programming convention where `of` is used to "lift" a value into a functor.
    ///
    /// # Arguments
    /// * `value` - Any JavaScript value
    ///
    /// # Returns
    /// A new atomic WasmPattern containing that value
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const p1 = WasmPattern.of("hello");  // Same as WasmPattern.point("hello")
    /// const p2 = WasmPattern.of(42);       // Same as WasmPattern.point(42)
    /// ```
    #[wasm_bindgen(js_name = of)]
    pub fn of(value: JsValue) -> WasmPattern {
        Self::point(value) // Just delegate to point - identical behavior
    }

    /// Create a pattern with a value and no elements (builder pattern).
    ///
    /// Due to wasm-bindgen limitations with custom types in arrays, this creates
    /// an empty pattern. Use `addElement()` to add children.
    ///
    /// # Arguments
    /// * `value` - Any JavaScript value
    ///
    /// # Returns
    /// A new WasmPattern with the value and no elements
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = WasmPattern.pattern("parent");
    /// pattern.addElement(WasmPattern.of("child1"));
    /// pattern.addElement(WasmPattern.of("child2"));
    /// ```
    #[wasm_bindgen(js_name = pattern)]
    pub fn pattern(value: JsValue) -> WasmPattern {
        WasmPattern {
            inner: Pattern::pattern(value, vec![]),
        }
    }

    /// Add a child pattern element to this pattern.
    ///
    /// This method mutates the pattern by adding a new child element.
    ///
    /// # Arguments
    /// * `element` - A WasmPattern to add as a child
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const parent = WasmPattern.pattern(WasmSubject.new("parent", [], {}));
    /// parent.addElement(WasmPattern.of("child1"));
    /// parent.addElement(WasmPattern.of("child2"));
    /// ```
    #[wasm_bindgen(js_name = addElement)]
    pub fn add_element(&mut self, element: WasmPattern) {
        // Since Pattern is immutable, we need to reconstruct it
        let mut elements = self.inner.elements().to_vec();
        elements.push(element.inner);
        self.inner = Pattern::pattern(self.inner.value().clone(), elements);
    }

    /// Create an array of atomic patterns from an array of values.
    ///
    /// Each value in the input array is lifted to an atomic pattern using `point()`.
    /// Returns a JavaScript array of WasmPattern instances, not a single nested pattern.
    ///
    /// This matches the Python binding's `fromValues` behavior.
    ///
    /// # Arguments
    /// * `values` - A JavaScript array of any values
    ///
    /// # Returns
    /// A JavaScript array of WasmPattern instances (one atomic pattern per value)
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const patterns = WasmPattern.fromValues([1, 2, 3]);
    /// // Returns [Pattern.point(1), Pattern.point(2), Pattern.point(3)]
    /// console.log(patterns.length); // 3
    /// console.log(patterns[0].value); // 1
    /// ```
    #[wasm_bindgen(js_name = fromValues)]
    pub fn from_values(values: &JsValue) -> Result<js_sys::Array, JsValue> {
        if !js_sys::Array::is_array(values) {
            return Err(JsValue::from_str("Values must be an array"));
        }

        let arr: &js_sys::Array = values.unchecked_ref();
        let result = js_sys::Array::new();

        for i in 0..arr.length() {
            let item = arr.get(i);
            // Create atomic pattern for each value
            let pattern = WasmPattern::point(item);
            // Convert WasmPattern to JsValue and push to result array
            result.push(&JsValue::from(pattern));
        }

        Ok(result)
    }

    // ========================================================================
    // Accessors (T008)
    // ========================================================================

    /// Get the value at the root of this pattern.
    ///
    /// Returns the JavaScript value stored in this pattern (can be any type).
    ///
    /// # Returns
    /// The value as a JsValue
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const p1 = WasmPattern.point("hello");
    /// console.log(p1.value); // "hello"
    ///
    /// const p2 = WasmPattern.point(42);
    /// console.log(p2.value); // 42
    /// ```
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        self.inner.value().clone()
    }

    /// Get the nested elements (sub-patterns) of this pattern as an array.
    ///
    /// Returns a JavaScript array of WasmPattern instances.
    ///
    /// # Returns
    /// A JavaScript array of WasmPattern instances
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = WasmPattern.pattern("parent");
    /// pattern.addElement(WasmPattern.of("child1"));
    /// pattern.addElement(WasmPattern.of("child2"));
    /// console.log(pattern.elements.length); // 2
    /// console.log(pattern.elements[0].value); // "child1"
    /// ```
    #[wasm_bindgen(getter)]
    pub fn elements(&self) -> js_sys::Array {
        let result = js_sys::Array::new();
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            result.push(&JsValue::from(wasm_elem));
        }
        result
    }

    /// Get a child element by index.
    ///
    /// # Arguments
    /// * `index` - The index of the element to retrieve (0-based)
    ///
    /// # Returns
    /// A WasmPattern if the index is valid, or undefined
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = WasmPattern.pattern("parent");
    /// pattern.addElement(WasmPattern.of("child"));
    /// const child = pattern.getElement(0);
    /// ```
    #[wasm_bindgen(js_name = getElement)]
    pub fn get_element(&self, index: usize) -> Option<WasmPattern> {
        self.inner.elements().get(index).map(|elem| WasmPattern {
            inner: elem.clone(),
        })
    }

    /// Get the number of direct child elements.
    ///
    /// # Returns
    /// The number of elements (0 for atomic patterns)
    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.inner.length()
    }

    /// Check if this pattern is atomic (has no children).
    ///
    /// # Returns
    /// true if the pattern has no elements, false otherwise
    #[wasm_bindgen(js_name = isAtomic)]
    pub fn is_atomic(&self) -> bool {
        self.inner.is_atomic()
    }
}
