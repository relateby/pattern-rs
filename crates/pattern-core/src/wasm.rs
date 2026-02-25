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
// Validation/Analysis Types (T016, T017 - Phase 4)
// ============================================================================

/// Configurable validation rules for pattern structure.
///
/// Rules can specify limits on nesting depth, element counts, or other structural properties.
/// All rules are optional (undefined/null means no limit).
///
/// Exported to JavaScript as `WasmValidationRules`.
#[wasm_bindgen]
pub struct WasmValidationRules {
    /// Maximum nesting depth allowed (undefined = no limit)
    #[wasm_bindgen(skip)]
    pub max_depth: Option<usize>,
    /// Maximum element count allowed (undefined = no limit)
    #[wasm_bindgen(skip)]
    pub max_elements: Option<usize>,
}

#[wasm_bindgen]
impl WasmValidationRules {
    /// Create new validation rules.
    ///
    /// # Arguments
    /// * `max_depth` - Maximum nesting depth (undefined for no limit)
    /// * `max_elements` - Maximum element count (undefined for no limit)
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const rules = new ValidationRules(10, 100); // Max depth 10, max elements 100
    /// const noLimits = new ValidationRules(undefined, undefined); // No limits
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(max_depth: JsValue, max_elements: JsValue) -> WasmValidationRules {
        let max_depth = js_to_option(&max_depth, js_to_i64).map(|v| v as usize);
        let max_elements = js_to_option(&max_elements, js_to_i64).map(|v| v as usize);

        WasmValidationRules {
            max_depth,
            max_elements,
        }
    }

    /// Get the max_depth value.
    #[wasm_bindgen(getter, js_name = maxDepth)]
    pub fn max_depth(&self) -> JsValue {
        match self.max_depth {
            Some(d) => JsValue::from_f64(d as f64),
            None => JsValue::undefined(),
        }
    }

    /// Get the max_elements value.
    #[wasm_bindgen(getter, js_name = maxElements)]
    pub fn max_elements(&self) -> JsValue {
        match self.max_elements {
            Some(e) => JsValue::from_f64(e as f64),
            None => JsValue::undefined(),
        }
    }
}

impl WasmValidationRules {
    /// Convert to internal ValidationRules type.
    fn to_internal(&self) -> crate::pattern::ValidationRules {
        crate::pattern::ValidationRules {
            max_depth: self.max_depth,
            max_elements: self.max_elements,
            required_fields: vec![], // Not exposed in WASM yet
        }
    }
}

/// Results from structure analysis utilities.
///
/// Provides detailed information about pattern structural characteristics.
///
/// Exported to JavaScript as `WasmStructureAnalysis`.
#[wasm_bindgen]
pub struct WasmStructureAnalysis {
    #[wasm_bindgen(skip)]
    inner: crate::pattern::StructureAnalysis,
}

#[wasm_bindgen]
impl WasmStructureAnalysis {
    /// Get the depth distribution (count of nodes at each depth level).
    ///
    /// # Returns
    /// A JavaScript array where index = depth, value = count
    #[wasm_bindgen(getter, js_name = depthDistribution)]
    pub fn depth_distribution(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for count in &self.inner.depth_distribution {
            arr.push(&JsValue::from_f64(*count as f64));
        }
        arr
    }

    /// Get the element counts at each level.
    ///
    /// # Returns
    /// A JavaScript array where index = level, value = element count
    #[wasm_bindgen(getter, js_name = elementCounts)]
    pub fn element_counts(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for count in &self.inner.element_counts {
            arr.push(&JsValue::from_f64(*count as f64));
        }
        arr
    }

    /// Get the identified nesting patterns.
    ///
    /// # Returns
    /// A JavaScript array of pattern description strings (e.g., "linear", "tree", "balanced")
    #[wasm_bindgen(getter, js_name = nestingPatterns)]
    pub fn nesting_patterns(&self) -> js_sys::Array {
        let arr = js_sys::Array::new();
        for pattern in &self.inner.nesting_patterns {
            arr.push(&JsValue::from_str(pattern));
        }
        arr
    }

    /// Get a human-readable summary of the structure.
    ///
    /// # Returns
    /// A summary string
    #[wasm_bindgen(getter)]
    pub fn summary(&self) -> String {
        self.inner.summary.clone()
    }
}

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
    let keys = js_sys::Object::keys(obj.unchecked_ref::<js_sys::Object>());

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
//
// Exported to JavaScript as `Value` (without the Factory suffix).

#[wasm_bindgen(js_name = Value)]
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
///
/// Exported to JavaScript as `WasmSubject`.
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
    /// A new Subject instance, or throws an error
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const subject = Subject.new(
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

    /// Get the labels as a JavaScript Set of strings.
    ///
    /// # Returns
    /// A JavaScript Set containing all label strings
    #[wasm_bindgen(getter)]
    pub fn labels(&self) -> js_sys::Set {
        let set = js_sys::Set::new(&JsValue::undefined());
        for label in &self.inner.labels {
            set.add(&JsValue::from_str(label));
        }
        set
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

        // Set labels as Set
        let labels_set = js_sys::Set::new(&JsValue::undefined());
        for label in &self.inner.labels {
            labels_set.add(&JsValue::from_str(label));
        }
        js_sys::Reflect::set(&obj, &JsValue::from_str("labels"), &labels_set).expect("set labels");

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
    /// Handles two representations:
    /// 1. Plain JS objects with `_type: "Subject"` (from `to_js_value()`)
    /// 2. WasmSubject WASM instances with `identity`, `labels`, `properties` getters
    ///
    /// Returns None if the JsValue is not a valid Subject representation.
    pub fn from_js_value(value: &JsValue) -> Option<Self> {
        if !value.is_object() {
            return None;
        }

        let obj: &js_sys::Object = value.unchecked_ref();

        // Check for type marker (plain JS object from to_js_value())
        let type_marker = js_sys::Reflect::get(obj, &JsValue::from_str("_type")).ok();
        let is_plain_subject = type_marker
            .as_ref()
            .and_then(|v| v.as_string())
            .map(|s| s == "Subject")
            .unwrap_or(false);

        // Check if it's a WasmSubject WASM instance (has __wbg_ptr and identity getter)
        let has_wbg_ptr = js_sys::Reflect::get(obj, &JsValue::from_str("__wbg_ptr"))
            .ok()
            .map(|v| !v.is_undefined() && !v.is_null())
            .unwrap_or(false);

        if !is_plain_subject && !has_wbg_ptr {
            return None;
        }

        // Extract identity (works for both plain objects and WASM instances)
        let identity_js = js_sys::Reflect::get(obj, &JsValue::from_str("identity")).ok()?;
        let identity = identity_js.as_string()?;

        // Extract labels (works for both plain objects and WASM instances)
        // Labels may be a JS Array or a JS Set
        let labels_js = js_sys::Reflect::get(obj, &JsValue::from_str("labels")).ok()?;
        let labels_set: std::collections::HashSet<String> = if js_sys::Array::is_array(&labels_js) {
            js_array_to_strings(&labels_js).ok()?.into_iter().collect()
        } else {
            // Try as JS Set: iterate via forEach
            let set: &js_sys::Set = labels_js.unchecked_ref();
            let mut result = std::collections::HashSet::new();
            set.for_each(&mut |v, _, _| {
                if let Some(s) = v.as_string() {
                    result.insert(s);
                }
            });
            result
        };

        // Extract properties (works for both plain objects and WASM instances)
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

    /// Create a WasmSubject from a Rust Subject.
    ///
    /// This allows other crates (like pattern-wasm) to create WasmSubject instances
    /// from native Rust Subject types, enabling proper type consistency when
    /// converting between Rust Pattern<Subject> and JS Pattern<JsValue>.
    ///
    /// # Arguments
    /// * `subject` - A Rust Subject instance
    ///
    /// # Returns
    /// A new WasmSubject wrapping the given Subject
    pub fn from_subject(subject: Subject) -> Self {
        WasmSubject { inner: subject }
    }

    /// Consume this WasmSubject and return the inner Rust Subject.
    ///
    /// This allows other crates to extract the native Rust Subject from a WasmSubject,
    /// enabling conversion from JS Pattern<JsValue> back to Rust Pattern<Subject>.
    ///
    /// # Returns
    /// The inner Subject
    pub fn into_subject(self) -> Subject {
        self.inner
    }

    /// Get a reference to the inner Rust Subject.
    ///
    /// This allows inspection of the Subject without consuming the WasmSubject.
    ///
    /// # Returns
    /// A reference to the inner Subject
    pub fn as_subject(&self) -> &Subject {
        &self.inner
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
/// primitives, objects, Subject instances, or even other Pattern instances (nesting).
///
/// This design matches the Python binding which stores PyAny (any Python object).
///
/// Exported to JavaScript as `WasmPattern`.
#[wasm_bindgen]
#[derive(Clone)]
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
    /// const p1 = Pattern.point("hello");
    /// const p2 = Pattern.point(42);
    /// const p3 = Pattern.point(true);
    ///
    /// // Subject
    /// const subject = new Subject("alice", [], {});
    /// const p4 = Pattern.point(subject);
    ///
    /// // Nesting - Pattern<Pattern<V>>
    /// const p5 = Pattern.point(p1);
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
    /// const p1 = Pattern.of("hello");  // Same as Pattern.point("hello")
    /// const p2 = Pattern.of(42);       // Same as Pattern.point(42)
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
    /// const pattern = Pattern.pattern("parent");
    /// pattern.addElement(Pattern.of("child1"));
    /// pattern.addElement(Pattern.of("child2"));
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
    /// Takes a reference so the same JS pattern can be added to multiple parents;
    /// the inner pattern is cloned. Taking ownership would be inconsistent with
    /// typical JS usage where the same element reference is reused.
    ///
    /// # Arguments
    /// * `element` - A WasmPattern to add as a child
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const parent = Pattern.pattern(Subject.new("parent", [], {}));
    /// parent.addElement(Pattern.of("child1"));
    /// parent.addElement(Pattern.of("child2"));
    /// ```
    #[wasm_bindgen(js_name = addElement)]
    pub fn add_element(&mut self, element: &WasmPattern) {
        // Since Pattern is immutable, we need to reconstruct it
        let mut elements = self.inner.elements().to_vec();
        elements.push(element.inner.clone());
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
    /// const patterns = Pattern.fromValues([1, 2, 3]);
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
    /// const p1 = Pattern.point("hello");
    /// console.log(p1.value); // "hello"
    ///
    /// const p2 = Pattern.point(42);
    /// console.log(p2.value); // 42
    /// ```
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> JsValue {
        self.inner.value().clone()
    }

    /// Get the identity of this pattern's value if it is a Subject.
    ///
    /// Returns the Subject's identity string, or undefined if the value is not a Subject.
    ///
    /// # Returns
    /// The identity string, or undefined
    #[wasm_bindgen(getter)]
    pub fn identity(&self) -> Option<String> {
        WasmSubject::from_js_value(self.inner.value()).map(|s| s.inner.identity.0.clone())
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
    /// const pattern = Pattern.pattern("parent");
    /// pattern.addElement(Pattern.of("child1"));
    /// pattern.addElement(Pattern.of("child2"));
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
    /// const pattern = Pattern.pattern("parent");
    /// pattern.addElement(Pattern.of("child"));
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

    // ========================================================================
    // Inspection Methods (T012 - Phase 4)
    // ========================================================================

    /// Get the total number of nodes in the pattern structure (including all nested patterns).
    ///
    /// # Returns
    /// The total number of nodes (root + all nested nodes)
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const atomic = Pattern.point("atom");
    /// console.log(atomic.size()); // 1
    ///
    /// const pattern = Pattern.pattern("root");
    /// pattern.addElement(Pattern.of("child1"));
    /// pattern.addElement(Pattern.of("child2"));
    /// console.log(pattern.size()); // 3 (root + 2 children)
    /// ```
    #[wasm_bindgen(js_name = size)]
    pub fn size(&self) -> usize {
        self.inner.size()
    }

    /// Get the maximum nesting depth of the pattern structure.
    ///
    /// # Returns
    /// The maximum nesting depth (atomic patterns have depth 0)
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const atomic = Pattern.point("hello");
    /// console.log(atomic.depth()); // 0
    ///
    /// const nested = Pattern.pattern("parent");
    /// const child = Pattern.pattern("child");
    /// child.addElement(Pattern.of("grandchild"));
    /// nested.addElement(child);
    /// console.log(nested.depth()); // 2
    /// ```
    #[wasm_bindgen(js_name = depth)]
    pub fn depth(&self) -> usize {
        self.inner.depth()
    }

    /// Extract all values from the pattern as a flat array (pre-order traversal).
    ///
    /// # Returns
    /// A JavaScript array containing all values in pre-order (root first, then elements)
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern("root");
    /// pattern.addElement(Pattern.of("child1"));
    /// pattern.addElement(Pattern.of("child2"));
    /// const values = pattern.values();
    /// // Returns ["root", "child1", "child2"]
    /// ```
    #[wasm_bindgen(js_name = values)]
    pub fn values(&self) -> js_sys::Array {
        let mut result = js_sys::Array::new();
        self.values_recursive(&mut result);
        result
    }

    /// Helper for values() - recursively collect values in pre-order.
    fn values_recursive(&self, result: &mut js_sys::Array) {
        result.push(&self.inner.value().clone());
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            wasm_elem.values_recursive(result);
        }
    }

    // ========================================================================
    // Query Methods (T013 - Phase 4)
    // ========================================================================

    /// Check if at least one value in the pattern satisfies the given predicate.
    ///
    /// Traverses in pre-order and short-circuits on first match.
    ///
    /// # Arguments
    /// * `predicate` - A JavaScript function that takes a value and returns boolean
    ///
    /// # Returns
    /// true if at least one value satisfies the predicate, false otherwise
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern("hello");
    /// pattern.addElement(Pattern.of("world"));
    /// const hasWorld = pattern.anyValue(v => v === "world"); // true
    /// ```
    #[wasm_bindgen(js_name = anyValue)]
    pub fn any_value(&self, predicate: &js_sys::Function) -> Result<bool, JsValue> {
        self.any_value_recursive(predicate)
    }

    /// Helper for anyValue() - recursive implementation with short-circuit.
    fn any_value_recursive(&self, predicate: &js_sys::Function) -> Result<bool, JsValue> {
        // Check current value
        let this = JsValue::null();
        let result = predicate
            .call1(&this, &self.inner.value().clone())
            .map_err(|e| JsValue::from_str(&format!("Predicate error: {:?}", e)))?;

        if let Some(true) = result.as_bool() {
            return Ok(true);
        }

        // Check elements recursively
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            if wasm_elem.any_value_recursive(predicate)? {
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Check if all values in the pattern satisfy the given predicate.
    ///
    /// Traverses in pre-order and short-circuits on first failure.
    ///
    /// # Arguments
    /// * `predicate` - A JavaScript function that takes a value and returns boolean
    ///
    /// # Returns
    /// true if all values satisfy the predicate, false otherwise
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern("hello");
    /// pattern.addElement(Pattern.of("world"));
    /// const allStrings = pattern.allValues(v => typeof v === "string"); // true
    /// ```
    #[wasm_bindgen(js_name = allValues)]
    pub fn all_values(&self, predicate: &js_sys::Function) -> Result<bool, JsValue> {
        self.all_values_recursive(predicate)
    }

    /// Helper for allValues() - recursive implementation with short-circuit.
    fn all_values_recursive(&self, predicate: &js_sys::Function) -> Result<bool, JsValue> {
        // Check current value
        let this = JsValue::null();
        let result = predicate
            .call1(&this, &self.inner.value().clone())
            .map_err(|e| JsValue::from_str(&format!("Predicate error: {:?}", e)))?;

        if let Some(false) = result.as_bool() {
            return Ok(false);
        }

        // Check elements recursively
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            if !wasm_elem.all_values_recursive(predicate)? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Filter subpatterns that satisfy the given pattern predicate.
    ///
    /// Traverses in pre-order and collects all matching patterns.
    ///
    /// # Arguments
    /// * `predicate` - A JavaScript function that takes a Pattern and returns boolean
    ///
    /// # Returns
    /// A JavaScript array of Pattern instances that satisfy the predicate
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern("root");
    /// pattern.addElement(Pattern.of("leaf1"));
    /// pattern.addElement(Pattern.of("leaf2"));
    /// const leaves = pattern.filter(p => p.isAtomic()); // Returns [leaf1, leaf2]
    /// ```
    #[wasm_bindgen(js_name = filter)]
    pub fn filter(&self, predicate: &js_sys::Function) -> Result<js_sys::Array, JsValue> {
        let result = js_sys::Array::new();
        self.filter_recursive(predicate, &result)?;
        Ok(result)
    }

    /// Helper for filter() - recursive implementation.
    fn filter_recursive(
        &self,
        predicate: &js_sys::Function,
        result: &js_sys::Array,
    ) -> Result<(), JsValue> {
        // Check current pattern
        let this = JsValue::null();
        let wasm_pattern = WasmPattern {
            inner: self.inner.clone(),
        };
        let matches = predicate
            .call1(&this, &JsValue::from(wasm_pattern.clone()))
            .map_err(|e| JsValue::from_str(&format!("Predicate error: {:?}", e)))?;

        if let Some(true) = matches.as_bool() {
            result.push(&JsValue::from(wasm_pattern));
        }

        // Recursively filter elements
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            wasm_elem.filter_recursive(predicate, result)?;
        }

        Ok(())
    }

    /// Find the first subpattern that satisfies the given predicate.
    ///
    /// Performs depth-first pre-order traversal and short-circuits on first match.
    ///
    /// # Arguments
    /// * `predicate` - A JavaScript function that takes a Pattern and returns boolean
    ///
    /// # Returns
    /// The first matching Pattern, or null if no match found
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern("root");
    /// pattern.addElement(Pattern.of("target"));
    /// const found = pattern.findFirst(p => p.value === "target");
    /// ```
    #[wasm_bindgen(js_name = findFirst)]
    pub fn find_first(&self, predicate: &js_sys::Function) -> Result<JsValue, JsValue> {
        self.find_first_recursive(predicate)
    }

    /// Helper for findFirst() - recursive implementation with short-circuit.
    fn find_first_recursive(&self, predicate: &js_sys::Function) -> Result<JsValue, JsValue> {
        // Check current pattern
        let this = JsValue::null();
        let wasm_pattern = WasmPattern {
            inner: self.inner.clone(),
        };
        let matches = predicate
            .call1(&this, &JsValue::from(wasm_pattern.clone()))
            .map_err(|e| JsValue::from_str(&format!("Predicate error: {:?}", e)))?;

        if let Some(true) = matches.as_bool() {
            return Ok(JsValue::from(wasm_pattern));
        }

        // Check elements recursively
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            let found = wasm_elem.find_first_recursive(predicate)?;
            if !found.is_null() {
                return Ok(found);
            }
        }

        Ok(JsValue::null())
    }

    /// Check if two patterns have identical structure (same values and same tree structure).
    ///
    /// # Arguments
    /// * `other` - Another Pattern to compare with
    ///
    /// # Returns
    /// true if the patterns match, false otherwise
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const p1 = Pattern.pattern("root");
    /// p1.addElement(Pattern.of("child"));
    /// const p2 = Pattern.pattern("root");
    /// p2.addElement(Pattern.of("child"));
    /// console.log(p1.matches(p2)); // true
    /// ```
    #[wasm_bindgen(js_name = matches)]
    pub fn matches(&self, other: &WasmPattern) -> bool {
        self.matches_recursive(&other.inner)
    }

    /// Helper for matches() - recursive implementation.
    fn matches_recursive(&self, other: &Pattern<JsValue>) -> bool {
        // Compare values - use JavaScript equality
        let self_val = self.inner.value();
        let other_val = other.value();

        // Try to compare as strings if both are strings
        if let (Some(s1), Some(s2)) = (self_val.as_string(), other_val.as_string()) {
            if s1 != s2 {
                return false;
            }
        } else if let (Some(n1), Some(n2)) = (self_val.as_f64(), other_val.as_f64()) {
            // Compare as numbers
            if n1 != n2 {
                return false;
            }
        } else if let (Some(b1), Some(b2)) = (self_val.as_bool(), other_val.as_bool()) {
            // Compare as booleans
            if b1 != b2 {
                return false;
            }
        } else {
            // For complex types, use JavaScript equality
            let eq = js_sys::Reflect::get(&js_sys::Object::new(), &JsValue::from_str("equals"))
                .ok()
                .and_then(|_| {
                    // Fallback: convert to JSON strings and compare
                    let s1 = js_sys::JSON::stringify(self_val).ok()?;
                    let s2 = js_sys::JSON::stringify(other_val).ok()?;
                    Some(s1.as_string()? == s2.as_string()?)
                })
                .unwrap_or(false);

            if !eq {
                return false;
            }
        }

        // Compare element count
        if self.inner.elements().len() != other.elements().len() {
            return false;
        }

        // Compare elements recursively
        for (self_elem, other_elem) in self.inner.elements().iter().zip(other.elements().iter()) {
            let wasm_self = WasmPattern {
                inner: self_elem.clone(),
            };
            if !wasm_self.matches_recursive(other_elem) {
                return false;
            }
        }

        true
    }

    /// Check if this pattern contains another pattern as a subpattern.
    ///
    /// # Arguments
    /// * `subpattern` - The pattern to search for
    ///
    /// # Returns
    /// true if this pattern contains the subpattern, false otherwise
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern("root");
    /// const child = Pattern.of("child");
    /// pattern.addElement(child);
    /// console.log(pattern.contains(Pattern.of("child"))); // true
    /// ```
    #[wasm_bindgen(js_name = contains)]
    pub fn contains(&self, subpattern: &WasmPattern) -> bool {
        self.contains_recursive(&subpattern.inner)
    }

    /// Helper for contains() - recursive implementation.
    fn contains_recursive(&self, subpattern: &Pattern<JsValue>) -> bool {
        // Check if current pattern matches
        if self.matches_recursive(subpattern) {
            return true;
        }

        // Check elements recursively
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            if wasm_elem.contains_recursive(subpattern) {
                return true;
            }
        }

        false
    }

    // ========================================================================
    // Transformation Methods (T014 - Phase 4)
    // ========================================================================

    /// Transform all values in the pattern using a mapping function.
    ///
    /// Creates a new pattern with the same structure but with values transformed by the function.
    /// The function is applied to each value in the pattern.
    ///
    /// # Arguments
    /// * `f` - A JavaScript function that takes a value and returns a new value
    ///
    /// # Returns
    /// A new Pattern with transformed values
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern("hello");
    /// pattern.addElement(Pattern.of("world"));
    /// const upper = pattern.map(v => typeof v === 'string' ? v.toUpperCase() : v);
    /// // Returns Pattern with values ["HELLO", "WORLD"]
    /// ```
    #[wasm_bindgen(js_name = map)]
    pub fn map(&self, f: &js_sys::Function) -> Result<WasmPattern, JsValue> {
        self.map_recursive(f)
    }

    /// Helper for map() - recursive implementation.
    fn map_recursive(&self, f: &js_sys::Function) -> Result<WasmPattern, JsValue> {
        // Transform current value
        let this = JsValue::null();
        let new_value = f
            .call1(&this, &self.inner.value().clone())
            .map_err(|e| JsValue::from_str(&format!("Map function error: {:?}", e)))?;

        // Recursively transform elements
        let mut new_elements = Vec::new();
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            let mapped_elem = wasm_elem.map_recursive(f)?;
            new_elements.push(mapped_elem.inner);
        }

        Ok(WasmPattern {
            inner: Pattern::pattern(new_value, new_elements),
        })
    }

    /// Fold the pattern into a single value by applying a function with an accumulator.
    ///
    /// Processes values in depth-first, root-first order (pre-order traversal).
    /// The accumulator is threaded through all processing steps.
    ///
    /// # Arguments
    /// * `init` - Initial accumulator value
    /// * `f` - A JavaScript function that takes (accumulator, value) and returns new accumulator
    ///
    /// # Returns
    /// The final accumulated value
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern(10);
    /// pattern.addElement(Pattern.of(20));
    /// pattern.addElement(Pattern.of(30));
    /// const sum = pattern.fold(0, (acc, v) => acc + v); // 60
    /// ```
    #[wasm_bindgen(js_name = fold)]
    pub fn fold(&self, init: JsValue, f: &js_sys::Function) -> Result<JsValue, JsValue> {
        self.fold_recursive(init, f)
    }

    /// Helper for fold() - recursive implementation.
    fn fold_recursive(&self, acc: JsValue, f: &js_sys::Function) -> Result<JsValue, JsValue> {
        // Process current value
        let this = JsValue::null();
        let new_acc = f
            .call2(&this, &acc, &self.inner.value().clone())
            .map_err(|e| JsValue::from_str(&format!("Fold function error: {:?}", e)))?;

        // Process elements recursively (left to right)
        let mut acc = new_acc;
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            acc = wasm_elem.fold_recursive(acc, f)?;
        }

        Ok(acc)
    }

    /// Paramorphism: bottom-up fold with access to both pattern and child results.
    ///
    /// This is a powerful recursion scheme that processes the pattern bottom-up,
    /// giving each node access to both its value and the results of processing its children.
    ///
    /// # Arguments
    /// * `f` - A JavaScript function that takes (pattern, childResults array) and returns a result
    ///
    /// # Returns
    /// The result of the paramorphism
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern("root");
    /// pattern.addElement(Pattern.of("child1"));
    /// pattern.addElement(Pattern.of("child2"));
    ///
    /// // Count nodes: each node returns 1 + sum of child counts
    /// const count = pattern.para((p, childResults) => {
    ///     return 1 + childResults.reduce((sum, r) => sum + r, 0);
    /// });
    /// // Returns 3 (root + 2 children)
    /// ```
    #[wasm_bindgen(js_name = para)]
    pub fn para(&self, f: &js_sys::Function) -> Result<JsValue, JsValue> {
        self.para_recursive(f)
    }

    /// Helper for para() - recursive implementation.
    fn para_recursive(&self, f: &js_sys::Function) -> Result<JsValue, JsValue> {
        // Recursively compute results for all child elements
        let child_results = js_sys::Array::new();
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            let result = wasm_elem.para_recursive(f)?;
            child_results.push(&result);
        }

        // Apply function to current pattern and child results
        let this = JsValue::null();
        let wasm_pattern = WasmPattern {
            inner: self.inner.clone(),
        };
        f.call2(&this, &JsValue::from(wasm_pattern), &child_results)
            .map_err(|e| JsValue::from_str(&format!("Para function error: {:?}", e)))
    }

    // ========================================================================
    // Combination Methods (T015 - Phase 4)
    // ========================================================================

    /// Combine two patterns associatively.
    ///
    /// For JavaScript values, this uses a custom combiner function to combine the values.
    /// Elements are concatenated (left first, then right).
    ///
    /// # Arguments
    /// * `other` - Another Pattern to combine with
    /// * `combiner` - A JavaScript function that takes (value1, value2) and returns combined value
    ///
    /// # Returns
    /// A new Pattern with combined value and concatenated elements
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const p1 = Pattern.pattern("hello");
    /// p1.addElement(Pattern.of("a"));
    /// const p2 = Pattern.pattern(" world");
    /// p2.addElement(Pattern.of("b"));
    /// const combined = p1.combine(p2, (v1, v2) => v1 + v2);
    /// // Result: Pattern("hello world") with elements [a, b]
    /// ```
    #[wasm_bindgen(js_name = combine)]
    pub fn combine(
        &self,
        other: &WasmPattern,
        combiner: &js_sys::Function,
    ) -> Result<WasmPattern, JsValue> {
        // Combine values using the provided function
        let this = JsValue::null();
        let combined_value = combiner
            .call2(
                &this,
                &self.inner.value().clone(),
                &other.inner.value().clone(),
            )
            .map_err(|e| JsValue::from_str(&format!("Combiner function error: {:?}", e)))?;

        // Concatenate elements (left first, then right)
        let mut combined_elements = self.inner.elements().to_vec();
        combined_elements.extend(other.inner.elements().iter().cloned());

        Ok(WasmPattern {
            inner: Pattern::pattern(combined_value, combined_elements),
        })
    }

    // ========================================================================
    // Comonad Methods (T015 - Phase 4)
    // ========================================================================

    /// Extract the decorative value at the current position.
    ///
    /// In Pattern's "decorated sequence" semantics, the value provides information
    /// ABOUT the elements. This operation accesses that decorative information.
    ///
    /// # Returns
    /// The value at this position
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const p = Pattern.point("hello");
    /// console.log(p.extract()); // "hello"
    /// ```
    #[wasm_bindgen(js_name = extract)]
    pub fn extract(&self) -> JsValue {
        self.inner.value().clone()
    }

    /// Compute new decorative information at each position based on subpattern context.
    ///
    /// This is a powerful comonad operation that gives each position access to its full
    /// subpattern context, enabling context-aware computation of new decorations.
    ///
    /// # Arguments
    /// * `f` - A JavaScript function that takes a Pattern and returns a new value
    ///
    /// # Returns
    /// A new Pattern with the same structure but with computed decorative values
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const p = Pattern.pattern("root");
    /// p.addElement(Pattern.of("child1"));
    /// p.addElement(Pattern.of("child2"));
    ///
    /// // Decorate each position with its size
    /// const sizes = p.extend(subpattern => subpattern.size());
    /// console.log(sizes.extract()); // 3 (root has 3 nodes)
    /// ```
    #[wasm_bindgen(js_name = extend)]
    pub fn extend(&self, f: &js_sys::Function) -> Result<WasmPattern, JsValue> {
        self.extend_recursive(f)
    }

    /// Helper for extend() - recursive implementation.
    fn extend_recursive(&self, f: &js_sys::Function) -> Result<WasmPattern, JsValue> {
        // Compute new decoration for current position
        let this = JsValue::null();
        let wasm_pattern = WasmPattern {
            inner: self.inner.clone(),
        };
        let new_value = f
            .call1(&this, &JsValue::from(wasm_pattern))
            .map_err(|e| JsValue::from_str(&format!("Extend function error: {:?}", e)))?;

        // Recursively extend elements
        let mut new_elements = Vec::new();
        for elem in self.inner.elements() {
            let wasm_elem = WasmPattern {
                inner: elem.clone(),
            };
            let extended_elem = wasm_elem.extend_recursive(f)?;
            new_elements.push(extended_elem.inner);
        }

        Ok(WasmPattern {
            inner: Pattern::pattern(new_value, new_elements),
        })
    }

    /// Decorate each position with its depth (maximum nesting level).
    ///
    /// Uses extend to compute the depth at every position.
    ///
    /// # Returns
    /// A Pattern where each position's value is the depth of that subpattern (as a number)
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const p = Pattern.pattern("root");
    /// const child = Pattern.pattern("child");
    /// child.addElement(Pattern.of("grandchild"));
    /// p.addElement(child);
    ///
    /// const depths = p.depthAt();
    /// console.log(depths.extract()); // 2 (root has depth 2)
    /// console.log(depths.elements[0].extract()); // 1 (child has depth 1)
    /// ```
    #[wasm_bindgen(js_name = depthAt)]
    pub fn depth_at(&self) -> WasmPattern {
        // Use extend with a function that computes depth
        let extended = self
            .inner
            .extend(&|subpattern: &Pattern<JsValue>| JsValue::from_f64(subpattern.depth() as f64));

        WasmPattern { inner: extended }
    }

    /// Decorate each position with its subtree size (total node count).
    ///
    /// Uses extend to compute the size at every position.
    ///
    /// # Returns
    /// A Pattern where each position's value is the size of that subpattern (as a number)
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const p = Pattern.pattern("root");
    /// p.addElement(Pattern.of("child1"));
    /// p.addElement(Pattern.of("child2"));
    ///
    /// const sizes = p.sizeAt();
    /// console.log(sizes.extract()); // 3 (root + 2 children)
    /// console.log(sizes.elements[0].extract()); // 1
    /// ```
    #[wasm_bindgen(js_name = sizeAt)]
    pub fn size_at(&self) -> WasmPattern {
        // Use extend with a function that computes size
        let extended = self
            .inner
            .extend(&|subpattern: &Pattern<JsValue>| JsValue::from_f64(subpattern.size() as f64));

        WasmPattern { inner: extended }
    }

    /// Decorate each position with its path from root (sequence of element indices).
    ///
    /// # Returns
    /// A Pattern where each position's value is an array representing the path from root
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const p = Pattern.pattern("root");
    /// const child = Pattern.pattern("child");
    /// child.addElement(Pattern.of("grandchild"));
    /// p.addElement(child);
    ///
    /// const paths = p.indicesAt();
    /// console.log(paths.extract()); // [] (root path)
    /// console.log(paths.elements[0].extract()); // [0] (first child path)
    /// console.log(paths.elements[0].elements[0].extract()); // [0, 0] (grandchild path)
    /// ```
    #[wasm_bindgen(js_name = indicesAt)]
    pub fn indices_at(&self) -> WasmPattern {
        fn go(path: Vec<usize>, pattern: &Pattern<JsValue>) -> Pattern<JsValue> {
            // Convert path to JsValue array
            let path_arr = js_sys::Array::new();
            for idx in &path {
                path_arr.push(&JsValue::from_f64(*idx as f64));
            }

            Pattern {
                value: path_arr.into(),
                elements: pattern
                    .elements()
                    .iter()
                    .enumerate()
                    .map(|(i, elem)| {
                        let mut new_path = path.clone();
                        new_path.push(i);
                        go(new_path, elem)
                    })
                    .collect(),
            }
        }

        WasmPattern {
            inner: go(vec![], &self.inner),
        }
    }

    // ========================================================================
    // Validation/Analysis Methods (T016 - Phase 4)
    // ========================================================================

    /// Validate pattern structure against configurable rules.
    ///
    /// Returns an Either-like value:
    /// - Success: `{ _tag: 'Right', right: undefined }`
    /// - Failure: `{ _tag: 'Left', left: ValidationError }`
    ///
    /// This return shape is compatible with effect-ts Either type.
    ///
    /// # Arguments
    /// * `rules` - ValidationRules specifying constraints
    ///
    /// # Returns
    /// An Either-like JsValue (does not throw)
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern("root");
    /// pattern.addElement(Pattern.of("child"));
    ///
    /// const rules = new ValidationRules(10, 100);
    /// const result = pattern.validate(rules);
    ///
    /// if (result._tag === 'Left') {
    ///     console.error('Validation failed:', result.left.message);
    /// } else {
    ///     console.log('Pattern is valid');
    /// }
    /// ```
    #[wasm_bindgen(js_name = validate)]
    pub fn validate(&self, rules: &WasmValidationRules) -> JsValue {
        let internal_rules = rules.to_internal();

        match self.inner.validate(&internal_rules) {
            Ok(()) => either_right(JsValue::undefined()),
            Err(error) => either_left(validation_error_to_js(&error)),
        }
    }

    /// Analyze the structural characteristics of the pattern.
    ///
    /// Returns detailed information about depth distribution, element counts,
    /// nesting patterns, and a human-readable summary.
    ///
    /// # Returns
    /// A StructureAnalysis object
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// const pattern = Pattern.pattern("root");
    /// pattern.addElement(Pattern.of("child1"));
    /// pattern.addElement(Pattern.of("child2"));
    ///
    /// const analysis = pattern.analyzeStructure();
    /// console.log('Summary:', analysis.summary);
    /// console.log('Depth distribution:', analysis.depthDistribution);
    /// ```
    #[wasm_bindgen(js_name = analyzeStructure)]
    pub fn analyze_structure(&self) -> WasmStructureAnalysis {
        WasmStructureAnalysis {
            inner: self.inner.analyze_structure(),
        }
    }
}

// ============================================================================
// WasmPattern Rust-side helpers (not exposed to JS)
// ============================================================================
//
// These methods allow other Rust crates (like pattern-wasm) to construct
// WasmPattern instances from native Rust types and extract them back.

impl WasmPattern {
    /// Create a WasmPattern from a Rust Pattern<JsValue>.
    ///
    /// This allows other crates to create WasmPattern instances from
    /// Pattern<JsValue> that they've constructed.
    ///
    /// # Arguments
    /// * `pattern` - A Rust Pattern<JsValue>
    ///
    /// # Returns
    /// A new WasmPattern wrapping the given pattern
    pub fn from_pattern(pattern: Pattern<JsValue>) -> Self {
        WasmPattern { inner: pattern }
    }

    /// Consume this WasmPattern and return the inner Pattern<JsValue>.
    ///
    /// This allows other crates to extract the native Rust Pattern from a WasmPattern.
    ///
    /// # Returns
    /// The inner Pattern<JsValue>
    pub fn into_pattern(self) -> Pattern<JsValue> {
        self.inner
    }

    /// Get a reference to the inner Pattern<JsValue>.
    ///
    /// This allows inspection of the Pattern without consuming the WasmPattern.
    ///
    /// # Returns
    /// A reference to the inner Pattern<JsValue>
    pub fn as_pattern(&self) -> &Pattern<JsValue> {
        &self.inner
    }
}

// ============================================================================
// 7. Graph Bindings (T007-T013 - Feature 033)
// ============================================================================
//
// WASM bindings for PatternGraph, ReconciliationPolicy, GraphQuery,
// GraphClass/TraversalDirection constant objects, and algorithm free functions.
//
// All types use the `Native*` JS name prefix to distinguish WASM-backed concrete
// classes from the pure TypeScript interfaces in @relateby/graph.

use crate::graph::graph_classifier::canonical_classifier;
use crate::graph::graph_query::{directed, directed_reverse, undirected, GraphQuery};
use crate::pattern_graph::{from_pattern_graph, from_patterns_with_policy, PatternGraph};
use crate::reconcile::{
    ElementMergeStrategy, LabelMerge, PropertyMerge, ReconciliationPolicy, SubjectMergeStrategy,
};

// ---------------------------------------------------------------------------
// Helper: convert Pattern<Subject> → WasmPattern (via JsValue encoding)
// ---------------------------------------------------------------------------

fn subject_pattern_to_wasm(p: &crate::pattern::Pattern<crate::subject::Subject>) -> WasmPattern {
    let subject_js = WasmSubject::from_subject(p.value.clone()).to_js_value();
    let wasm_p = WasmPattern {
        inner: crate::pattern::Pattern {
            value: subject_js,
            elements: p
                .elements
                .iter()
                .map(|e| subject_pattern_to_wasm(e).inner)
                .collect(),
        },
    };
    wasm_p
}

fn wasm_pattern_to_subject_pattern(
    p: &WasmPattern,
) -> Option<crate::pattern::Pattern<crate::subject::Subject>> {
    let subject = WasmSubject::from_js_value(&p.inner.value)?.into_subject();
    let elements: Vec<_> = p
        .inner
        .elements
        .iter()
        .filter_map(|e| wasm_pattern_to_subject_pattern(&WasmPattern { inner: e.clone() }))
        .collect();
    Some(crate::pattern::Pattern {
        value: subject,
        elements,
    })
}

/// Convert a JsValue (which may be a serialized WasmPattern object) to Pattern<Subject>.
///
/// WasmPattern objects in JS have `value` and `elements` fields. The `value` is a
/// Subject JsValue with `_type: 'Subject'`. This function extracts the Subject and
/// recursively converts child elements.
fn js_value_to_subject_pattern(
    js: &JsValue,
) -> Option<crate::pattern::Pattern<crate::subject::Subject>> {
    if !js.is_object() {
        return None;
    }

    // Extract the `value` field (Subject JsValue)
    let value_js = js_sys::Reflect::get(js, &JsValue::from_str("value")).ok()?;
    let subject = WasmSubject::from_js_value(&value_js)?.into_subject();

    // Extract the `elements` field (array of child patterns)
    let elements_js = js_sys::Reflect::get(js, &JsValue::from_str("elements")).ok()?;
    let elements = if js_sys::Array::is_array(&elements_js) {
        let arr: &js_sys::Array = elements_js.unchecked_ref();
        (0..arr.length())
            .filter_map(|i| js_value_to_subject_pattern(&arr.get(i)))
            .collect()
    } else {
        vec![]
    };

    Some(crate::pattern::Pattern {
        value: subject,
        elements,
    })
}

fn subject_pattern_to_js(p: &crate::pattern::Pattern<crate::subject::Subject>) -> JsValue {
    JsValue::from(subject_pattern_to_wasm(p))
}

fn patterns_to_js_array(
    patterns: &[crate::pattern::Pattern<crate::subject::Subject>],
) -> js_sys::Array {
    let arr = js_sys::Array::new();
    for p in patterns {
        arr.push(&subject_pattern_to_js(p));
    }
    arr
}

// ---------------------------------------------------------------------------
// Helper: parse weight JsValue → TraversalWeight<Subject>
// ---------------------------------------------------------------------------

fn parse_weight(weight_js: &JsValue) -> crate::graph::graph_query::TraversalWeight<Subject> {
    if let Some(s) = weight_js.as_string() {
        match s.as_str() {
            "directed" => directed::<Subject>(),
            "directed_reverse" => directed_reverse::<Subject>(),
            _ => undirected::<Subject>(), // "undirected" or unknown
        }
    } else if weight_js.is_function() {
        let func = js_sys::Function::from(weight_js.clone());
        std::rc::Rc::new(
            move |rel: &crate::pattern::Pattern<Subject>,
                  dir: crate::graph::graph_query::TraversalDirection| {
                let wasm_rel = subject_pattern_to_wasm(rel);
                let dir_str = match dir {
                    crate::graph::graph_query::TraversalDirection::Forward => "forward",
                    crate::graph::graph_query::TraversalDirection::Backward => "backward",
                };
                let result = func.call2(
                    &JsValue::undefined(),
                    &JsValue::from(wasm_rel),
                    &JsValue::from_str(dir_str),
                );
                match result {
                    Ok(v) => v.as_f64().unwrap_or(1.0),
                    Err(_) => 1.0,
                }
            },
        )
    } else {
        undirected::<Subject>()
    }
}

// ---------------------------------------------------------------------------
// WasmReconciliationPolicy (js_name = NativeReconciliationPolicy)
// ---------------------------------------------------------------------------

/// WASM binding for ReconciliationPolicy.
///
/// Governs how identity conflicts are resolved when patterns with the same
/// identity are combined into a PatternGraph.
///
/// Exported to JavaScript as `WasmReconciliationPolicy`.
#[wasm_bindgen]
pub struct WasmReconciliationPolicy {
    #[wasm_bindgen(skip)]
    pub inner: ReconciliationPolicy<SubjectMergeStrategy>,
}

#[wasm_bindgen]
impl WasmReconciliationPolicy {
    /// Incoming pattern replaces existing on identity conflict.
    #[wasm_bindgen(js_name = lastWriteWins)]
    pub fn last_write_wins() -> WasmReconciliationPolicy {
        WasmReconciliationPolicy {
            inner: ReconciliationPolicy::LastWriteWins,
        }
    }

    /// Existing pattern is kept; incoming is discarded on identity conflict.
    #[wasm_bindgen(js_name = firstWriteWins)]
    pub fn first_write_wins() -> WasmReconciliationPolicy {
        WasmReconciliationPolicy {
            inner: ReconciliationPolicy::FirstWriteWins,
        }
    }

    /// Identity conflict is recorded in graph.conflicts; neither wins.
    #[wasm_bindgen(js_name = strict)]
    pub fn strict() -> WasmReconciliationPolicy {
        WasmReconciliationPolicy {
            inner: ReconciliationPolicy::Strict,
        }
    }

    /// Merge labels and properties per strategy.
    ///
    /// # Arguments
    /// * `options` - Optional JS object with `elementStrategy`, `labelMerge`, `propertyMerge`
    #[wasm_bindgen(js_name = merge)]
    pub fn merge_policy(options: JsValue) -> WasmReconciliationPolicy {
        let mut element_strategy = ElementMergeStrategy::UnionElements;
        let mut label_merge = LabelMerge::UnionLabels;
        let mut property_merge = PropertyMerge::ShallowMerge;

        if options.is_object() {
            if let Ok(es) = js_sys::Reflect::get(&options, &JsValue::from_str("elementStrategy")) {
                if let Some(s) = es.as_string() {
                    element_strategy = match s.as_str() {
                        "replace" => ElementMergeStrategy::ReplaceElements,
                        "append" => ElementMergeStrategy::AppendElements,
                        _ => ElementMergeStrategy::UnionElements,
                    };
                }
            }
            if let Ok(lm) = js_sys::Reflect::get(&options, &JsValue::from_str("labelMerge")) {
                if let Some(s) = lm.as_string() {
                    label_merge = match s.as_str() {
                        "intersect" => LabelMerge::IntersectLabels,
                        "left" | "right" => LabelMerge::ReplaceLabels,
                        _ => LabelMerge::UnionLabels,
                    };
                }
            }
            if let Ok(pm) = js_sys::Reflect::get(&options, &JsValue::from_str("propertyMerge")) {
                if let Some(s) = pm.as_string() {
                    property_merge = match s.as_str() {
                        "left" | "right" => PropertyMerge::ReplaceProperties,
                        "merge" => PropertyMerge::ShallowMerge,
                        _ => PropertyMerge::ShallowMerge,
                    };
                }
            }
        }

        let strategy = SubjectMergeStrategy {
            label_merge,
            property_merge,
        };
        WasmReconciliationPolicy {
            inner: ReconciliationPolicy::Merge(element_strategy, strategy),
        }
    }
}

// ---------------------------------------------------------------------------
// WasmPatternGraph (js_name = NativePatternGraph)
// ---------------------------------------------------------------------------

/// WASM binding for PatternGraph<(), Subject>.
///
/// A classified, indexed collection of patterns organized by graph role.
/// Immutable after construction; merge returns a new instance.
///
/// Exported to JavaScript as `WasmPatternGraph`.
#[wasm_bindgen]
pub struct WasmPatternGraph {
    #[wasm_bindgen(skip)]
    pub inner: std::rc::Rc<PatternGraph<(), Subject>>,
}

#[wasm_bindgen]
impl WasmPatternGraph {
    /// Construct a graph from an array of NativePattern instances.
    ///
    /// Patterns whose values are not Subject instances are classified as `other`.
    /// Never throws — unrecognized patterns are silently dropped.
    #[wasm_bindgen(js_name = fromPatterns)]
    pub fn from_patterns(
        patterns: &js_sys::Array,
        policy: Option<WasmReconciliationPolicy>,
    ) -> WasmPatternGraph {
        let classifier = canonical_classifier::<Subject>();
        let policy_inner = policy
            .map(|p| p.inner)
            .unwrap_or(ReconciliationPolicy::LastWriteWins);

        let subject_patterns: Vec<crate::pattern::Pattern<Subject>> = (0..patterns.length())
            .filter_map(|i| {
                let item = patterns.get(i);
                // Items are WasmPattern instances serialized as JS objects.
                // Extract the value field (which is a Subject JsValue) and elements.
                js_value_to_subject_pattern(&item)
            })
            .collect();

        let graph = from_patterns_with_policy(&classifier, &policy_inner, subject_patterns);

        WasmPatternGraph {
            inner: std::rc::Rc::new(graph),
        }
    }

    /// Construct an empty graph.
    #[wasm_bindgen(js_name = empty)]
    pub fn empty() -> WasmPatternGraph {
        WasmPatternGraph {
            inner: std::rc::Rc::new(PatternGraph::empty()),
        }
    }

    /// All node patterns in the graph.
    #[wasm_bindgen(getter)]
    pub fn nodes(&self) -> js_sys::Array {
        patterns_to_js_array(&self.inner.pg_nodes.values().cloned().collect::<Vec<_>>())
    }

    /// All relationship patterns in the graph.
    #[wasm_bindgen(getter)]
    pub fn relationships(&self) -> js_sys::Array {
        patterns_to_js_array(
            &self
                .inner
                .pg_relationships
                .values()
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    /// All walk patterns in the graph.
    #[wasm_bindgen(getter)]
    pub fn walks(&self) -> js_sys::Array {
        patterns_to_js_array(&self.inner.pg_walks.values().cloned().collect::<Vec<_>>())
    }

    /// All annotation patterns in the graph.
    #[wasm_bindgen(getter)]
    pub fn annotations(&self) -> js_sys::Array {
        patterns_to_js_array(
            &self
                .inner
                .pg_annotations
                .values()
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    /// Identity conflicts recorded under the strict policy.
    ///
    /// Returns a JS object mapping identity strings to arrays of conflicting patterns.
    #[wasm_bindgen(getter)]
    pub fn conflicts(&self) -> JsValue {
        let obj = js_sys::Object::new();
        for (id, patterns) in &self.inner.pg_conflicts {
            let arr = patterns_to_js_array(patterns);
            js_sys::Reflect::set(&obj, &JsValue::from_str(&id.0), &arr).ok();
        }
        obj.into()
    }

    /// Total count of non-conflict elements.
    #[wasm_bindgen(getter)]
    pub fn size(&self) -> usize {
        self.inner.pg_nodes.len()
            + self.inner.pg_relationships.len()
            + self.inner.pg_walks.len()
            + self.inner.pg_annotations.len()
            + self.inner.pg_other.len()
    }

    /// Merge this graph with another, returning a new graph.
    ///
    /// Uses LastWriteWins policy for the merge.
    #[wasm_bindgen(js_name = merge)]
    pub fn merge(&self, other: &WasmPatternGraph) -> WasmPatternGraph {
        let classifier = canonical_classifier::<Subject>();
        let policy = ReconciliationPolicy::LastWriteWins;

        // Collect all patterns from both graphs and rebuild
        let all_patterns: Vec<crate::pattern::Pattern<Subject>> = self
            .inner
            .pg_nodes
            .values()
            .chain(self.inner.pg_relationships.values())
            .chain(self.inner.pg_walks.values())
            .chain(self.inner.pg_annotations.values())
            .chain(other.inner.pg_nodes.values())
            .chain(other.inner.pg_relationships.values())
            .chain(other.inner.pg_walks.values())
            .chain(other.inner.pg_annotations.values())
            .cloned()
            .collect();

        let graph = from_patterns_with_policy(&classifier, &policy, all_patterns);

        WasmPatternGraph {
            inner: std::rc::Rc::new(graph),
        }
    }

    /// Return patterns in bottom-up shape-class topological order.
    ///
    /// Returns null if the graph contains a cycle.
    /// Used by paraGraph and paraGraphFixed to determine processing order.
    #[wasm_bindgen(js_name = topoSort)]
    pub fn topo_sort(&self) -> JsValue {
        let query = from_pattern_graph(std::rc::Rc::clone(&self.inner));
        match crate::graph::algorithms::topological_sort(&query) {
            Some(sorted) => JsValue::from(patterns_to_js_array(&sorted)),
            None => JsValue::null(),
        }
    }
}

// ---------------------------------------------------------------------------
// WasmGraphQuery (js_name = NativeGraphQuery)
// ---------------------------------------------------------------------------

/// WASM binding for GraphQuery<Subject>.
///
/// A read-only query handle over a PatternGraph. Provides structural navigation
/// without exposing the underlying storage.
///
/// Exported to JavaScript as `WasmGraphQuery`.
#[wasm_bindgen]
pub struct WasmGraphQuery {
    #[wasm_bindgen(skip)]
    pub inner: GraphQuery<Subject>,
}

#[wasm_bindgen]
impl WasmGraphQuery {
    /// Create a query handle from a NativePatternGraph.
    #[wasm_bindgen(js_name = fromPatternGraph)]
    pub fn from_pattern_graph(graph: &WasmPatternGraph) -> WasmGraphQuery {
        WasmGraphQuery {
            inner: from_pattern_graph(std::rc::Rc::clone(&graph.inner)),
        }
    }

    /// All node patterns.
    #[wasm_bindgen(js_name = nodes)]
    pub fn nodes(&self) -> js_sys::Array {
        patterns_to_js_array(&(self.inner.query_nodes)())
    }

    /// All relationship patterns.
    #[wasm_bindgen(js_name = relationships)]
    pub fn relationships(&self) -> js_sys::Array {
        patterns_to_js_array(&(self.inner.query_relationships)())
    }

    /// Source node of a relationship. Returns null if not found.
    #[wasm_bindgen(js_name = source)]
    pub fn source(&self, rel: &WasmPattern) -> JsValue {
        let subject_rel = match wasm_pattern_to_subject_pattern(rel) {
            Some(r) => r,
            None => return JsValue::null(),
        };
        match (self.inner.query_source)(&subject_rel) {
            Some(p) => subject_pattern_to_js(&p),
            None => JsValue::null(),
        }
    }

    /// Target node of a relationship. Returns null if not found.
    #[wasm_bindgen(js_name = target)]
    pub fn target(&self, rel: &WasmPattern) -> JsValue {
        let subject_rel = match wasm_pattern_to_subject_pattern(rel) {
            Some(r) => r,
            None => return JsValue::null(),
        };
        match (self.inner.query_target)(&subject_rel) {
            Some(p) => subject_pattern_to_js(&p),
            None => JsValue::null(),
        }
    }

    /// All relationships incident to a node.
    #[wasm_bindgen(js_name = incidentRels)]
    pub fn incident_rels(&self, node: &WasmPattern) -> js_sys::Array {
        let subject_node = match wasm_pattern_to_subject_pattern(node) {
            Some(n) => n,
            None => return js_sys::Array::new(),
        };
        patterns_to_js_array(&(self.inner.query_incident_rels)(&subject_node))
    }

    /// Count of incident relationships for a node.
    #[wasm_bindgen(js_name = degree)]
    pub fn degree(&self, node: &WasmPattern) -> usize {
        let subject_node = match wasm_pattern_to_subject_pattern(node) {
            Some(n) => n,
            None => return 0,
        };
        (self.inner.query_degree)(&subject_node)
    }

    /// Look up a node by its identity string. Returns null if not found.
    #[wasm_bindgen(js_name = nodeById)]
    pub fn node_by_id(&self, identity: &str) -> JsValue {
        let sym = Symbol(identity.to_string());
        match (self.inner.query_node_by_id)(&sym) {
            Some(p) => subject_pattern_to_js(&p),
            None => JsValue::null(),
        }
    }

    /// Look up a relationship by its identity string. Returns null if not found.
    #[wasm_bindgen(js_name = relationshipById)]
    pub fn relationship_by_id(&self, identity: &str) -> JsValue {
        let sym = Symbol(identity.to_string());
        match (self.inner.query_relationship_by_id)(&sym) {
            Some(p) => subject_pattern_to_js(&p),
            None => JsValue::null(),
        }
    }
}

// ---------------------------------------------------------------------------
// GraphClass constant object (T009)
// ---------------------------------------------------------------------------

/// String constants for graph element classification.
///
/// Exported to JavaScript as a plain object (not a class).
/// Use these constants as discriminants in transform callbacks.
#[wasm_bindgen]
pub fn graph_class_constants() -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(&obj, &JsValue::from_str("NODE"), &JsValue::from_str("node")).ok();
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("RELATIONSHIP"),
        &JsValue::from_str("relationship"),
    )
    .ok();
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("ANNOTATION"),
        &JsValue::from_str("annotation"),
    )
    .ok();
    js_sys::Reflect::set(&obj, &JsValue::from_str("WALK"), &JsValue::from_str("walk")).ok();
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("OTHER"),
        &JsValue::from_str("other"),
    )
    .ok();
    obj.into()
}

// ---------------------------------------------------------------------------
// TraversalDirection constant object (T011)
// ---------------------------------------------------------------------------

/// String constants for traversal direction.
///
/// Exported to JavaScript as a plain object (not a class).
#[wasm_bindgen]
pub fn traversal_direction_constants() -> JsValue {
    let obj = js_sys::Object::new();
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("FORWARD"),
        &JsValue::from_str("forward"),
    )
    .ok();
    js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("BACKWARD"),
        &JsValue::from_str("backward"),
    )
    .ok();
    obj.into()
}

// ---------------------------------------------------------------------------
// Algorithm free functions (T012)
// ---------------------------------------------------------------------------

/// Breadth-first search from a start node.
///
/// Returns patterns in BFS order. Weight defaults to undirected.
#[wasm_bindgen]
pub fn bfs(query: &WasmGraphQuery, start: &WasmPattern, weight: JsValue) -> js_sys::Array {
    let subject_start = match wasm_pattern_to_subject_pattern(start) {
        Some(s) => s,
        None => return js_sys::Array::new(),
    };
    let w = parse_weight(&weight);
    let result = crate::graph::algorithms::bfs(&query.inner, &w, &subject_start);
    patterns_to_js_array(&result)
}

/// Depth-first search from a start node.
///
/// Returns patterns in DFS order. Weight defaults to undirected.
#[wasm_bindgen]
pub fn dfs(query: &WasmGraphQuery, start: &WasmPattern, weight: JsValue) -> js_sys::Array {
    let subject_start = match wasm_pattern_to_subject_pattern(start) {
        Some(s) => s,
        None => return js_sys::Array::new(),
    };
    let w = parse_weight(&weight);
    let result = crate::graph::algorithms::dfs(&query.inner, &w, &subject_start);
    patterns_to_js_array(&result)
}

/// Shortest path between two nodes.
///
/// Returns null if no path exists. Weight defaults to undirected.
#[wasm_bindgen(js_name = shortestPath)]
pub fn shortest_path(
    query: &WasmGraphQuery,
    start: &WasmPattern,
    end: &WasmPattern,
    weight: JsValue,
) -> JsValue {
    let subject_start = match wasm_pattern_to_subject_pattern(start) {
        Some(s) => s,
        None => return JsValue::null(),
    };
    let subject_end = match wasm_pattern_to_subject_pattern(end) {
        Some(e) => e,
        None => return JsValue::null(),
    };
    let w = parse_weight(&weight);
    match crate::graph::algorithms::shortest_path(&query.inner, &w, &subject_start, &subject_end) {
        Some(path) => JsValue::from(patterns_to_js_array(&path)),
        None => JsValue::null(),
    }
}

/// All paths between two nodes.
///
/// Returns an array of path arrays. Weight defaults to undirected.
#[wasm_bindgen(js_name = allPaths)]
pub fn all_paths(
    query: &WasmGraphQuery,
    start: &WasmPattern,
    end: &WasmPattern,
    weight: JsValue,
) -> js_sys::Array {
    let subject_start = match wasm_pattern_to_subject_pattern(start) {
        Some(s) => s,
        None => return js_sys::Array::new(),
    };
    let subject_end = match wasm_pattern_to_subject_pattern(end) {
        Some(e) => e,
        None => return js_sys::Array::new(),
    };
    let w = parse_weight(&weight);
    let paths = crate::graph::algorithms::all_paths(&query.inner, &w, &subject_start, &subject_end);
    let outer = js_sys::Array::new();
    for path in &paths {
        outer.push(&JsValue::from(patterns_to_js_array(path)));
    }
    outer
}

/// Connected components of the graph.
///
/// Returns an array of component arrays. Weight defaults to undirected.
#[wasm_bindgen(js_name = connectedComponents)]
pub fn connected_components(query: &WasmGraphQuery, weight: JsValue) -> js_sys::Array {
    let w = parse_weight(&weight);
    let components = crate::graph::algorithms::connected_components(&query.inner, &w);
    let outer = js_sys::Array::new();
    for component in &components {
        outer.push(&JsValue::from(patterns_to_js_array(component)));
    }
    outer
}

/// Returns true if the graph contains a directed cycle.
#[wasm_bindgen(js_name = hasCycle)]
pub fn has_cycle(query: &WasmGraphQuery) -> bool {
    crate::graph::algorithms::has_cycle(&query.inner)
}

/// Returns true if the graph is connected.
///
/// Weight defaults to undirected.
#[wasm_bindgen(js_name = isConnected)]
pub fn is_connected(query: &WasmGraphQuery, weight: JsValue) -> bool {
    let w = parse_weight(&weight);
    crate::graph::algorithms::is_connected(&query.inner, &w)
}

/// Topological sort of the graph.
///
/// Returns null if the graph contains a cycle.
#[wasm_bindgen(js_name = topologicalSort)]
pub fn topological_sort(query: &WasmGraphQuery) -> JsValue {
    match crate::graph::algorithms::topological_sort(&query.inner) {
        Some(sorted) => JsValue::from(patterns_to_js_array(&sorted)),
        None => JsValue::null(),
    }
}

/// Degree centrality for all nodes.
///
/// Returns a JS object mapping identity strings to normalized scores.
#[wasm_bindgen(js_name = degreeCentrality)]
pub fn degree_centrality(query: &WasmGraphQuery) -> JsValue {
    let scores = crate::graph::algorithms::degree_centrality(&query.inner);
    let obj = js_sys::Object::new();
    for (id, score) in &scores {
        js_sys::Reflect::set(&obj, &JsValue::from_str(&id.0), &JsValue::from_f64(*score)).ok();
    }
    obj.into()
}

/// Betweenness centrality for all nodes.
///
/// Returns a JS object mapping identity strings to scores.
/// Weight defaults to undirected.
#[wasm_bindgen(js_name = betweennessCentrality)]
pub fn betweenness_centrality(query: &WasmGraphQuery, weight: JsValue) -> JsValue {
    let w = parse_weight(&weight);
    let scores = crate::graph::algorithms::betweenness_centrality(&query.inner, &w);
    let obj = js_sys::Object::new();
    for (id, score) in &scores {
        js_sys::Reflect::set(&obj, &JsValue::from_str(&id.0), &JsValue::from_f64(*score)).ok();
    }
    obj.into()
}

/// Minimum spanning tree.
///
/// Returns an array of relationship patterns. Weight defaults to undirected.
#[wasm_bindgen(js_name = minimumSpanningTree)]
pub fn minimum_spanning_tree(query: &WasmGraphQuery, weight: JsValue) -> js_sys::Array {
    let w = parse_weight(&weight);
    let tree = crate::graph::algorithms::minimum_spanning_tree(&query.inner, &w);
    patterns_to_js_array(&tree)
}

/// Returns all walks containing the given node.
#[wasm_bindgen(js_name = queryWalksContaining)]
pub fn query_walks_containing(query: &WasmGraphQuery, node: &WasmPattern) -> js_sys::Array {
    let subject_node = match wasm_pattern_to_subject_pattern(node) {
        Some(n) => n,
        None => return js_sys::Array::new(),
    };
    let classifier = canonical_classifier::<Subject>();
    let walks =
        crate::graph::algorithms::query_walks_containing(&classifier, &query.inner, &subject_node);
    patterns_to_js_array(&walks)
}

/// Returns all elements that share a container with the given node.
#[wasm_bindgen(js_name = queryCoMembers)]
pub fn query_co_members(
    query: &WasmGraphQuery,
    node: &WasmPattern,
    container: &WasmPattern,
) -> js_sys::Array {
    let subject_node = match wasm_pattern_to_subject_pattern(node) {
        Some(n) => n,
        None => return js_sys::Array::new(),
    };
    let subject_container = match wasm_pattern_to_subject_pattern(container) {
        Some(c) => c,
        None => return js_sys::Array::new(),
    };
    let members =
        crate::graph::algorithms::query_co_members(&query.inner, &subject_node, &subject_container);
    patterns_to_js_array(&members)
}

/// Returns all annotations of the given target element.
#[wasm_bindgen(js_name = queryAnnotationsOf)]
pub fn query_annotations_of(query: &WasmGraphQuery, target: &WasmPattern) -> js_sys::Array {
    let subject_target = match wasm_pattern_to_subject_pattern(target) {
        Some(t) => t,
        None => return js_sys::Array::new(),
    };
    let classifier = canonical_classifier::<Subject>();
    let annotations =
        crate::graph::algorithms::query_annotations_of(&classifier, &query.inner, &subject_target);
    patterns_to_js_array(&annotations)
}
