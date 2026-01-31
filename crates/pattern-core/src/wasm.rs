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
/// Exported to JavaScript as `ValidationRules`.
#[wasm_bindgen(js_name = ValidationRules)]
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
/// Exported to JavaScript as `StructureAnalysis`.
#[wasm_bindgen(js_name = StructureAnalysis)]
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
/// Exported to JavaScript as `Subject` (without the Wasm prefix).
#[wasm_bindgen(js_name = Subject)]
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

    /// Convert a primitive JavaScript value to a Subject using pattern-lisp compatible mapping.
    ///
    /// This method provides sensible defaults for converting primitive JavaScript types to Subjects,
    /// using labels compatible with pattern-lisp's Codec.hs for interoperability:
    /// - **string**: Label "String", value stored in "value" property
    /// - **number**: Label "Number", value stored in "value" property
    /// - **boolean**: Label "Bool", value stored in "value" property
    /// - **Subject instance**: Passthrough unchanged
    ///
    /// **Note**: Arrays and objects should use `Gram.from()` instead, which creates proper
    /// Pattern structures with elements (not Subject properties) for pattern-lisp compatibility.
    ///
    /// # Arguments
    /// * `value` - A primitive JavaScript value (string, number, boolean) or Subject
    /// * `options` - Optional conversion options object with fields:
    ///   - `label?: string` - Custom label override (default: type-based)
    ///   - `valueProperty?: string` - Property name for value (default: "value")
    ///   - `index?: number` - Index for identity generation (default: 0)
    ///
    /// # Returns
    /// A new Subject instance
    ///
    /// # Example (JavaScript)
    /// ```javascript
    /// // Primitive conversion with pattern-lisp compatible defaults
    /// const s1 = Subject.fromValue("hello");
    /// // Result: { identity: "_0", labels: ["String"], properties: { value: "hello" } }
    ///
    /// const s2 = Subject.fromValue(42);
    /// // Result: { identity: "_0", labels: ["Number"], properties: { value: 42 } }
    ///
    /// const s3 = Subject.fromValue(true);
    /// // Result: { identity: "_0", labels: ["Bool"], properties: { value: true } }
    ///
    /// // Subject passthrough
    /// const subject = new Subject("alice", ["Person"], {name: "Alice"});
    /// const s4 = Subject.fromValue(subject); // Returns same subject
    /// ```
    #[wasm_bindgen(js_name = fromValue)]
    pub fn from_value(value: JsValue, options: JsValue) -> Result<WasmSubject, JsValue> {
        // Extract options (all optional)
        let label_override = if options.is_object() && !options.is_null() {
            js_sys::Reflect::get(&options, &JsValue::from_str("label"))
                .ok()
                .and_then(|v| {
                    if v.is_undefined() {
                        None
                    } else {
                        v.as_string()
                    }
                })
        } else {
            None
        };

        let value_property = if options.is_object() && !options.is_null() {
            js_sys::Reflect::get(&options, &JsValue::from_str("valueProperty"))
                .ok()
                .and_then(|v| {
                    if v.is_undefined() {
                        None
                    } else {
                        v.as_string()
                    }
                })
                .unwrap_or_else(|| "value".to_string())
        } else {
            "value".to_string()
        };

        let index = if options.is_object() && !options.is_null() {
            js_sys::Reflect::get(&options, &JsValue::from_str("index"))
                .ok()
                .and_then(|v| if v.is_undefined() { None } else { v.as_f64() })
                .map(|f| f as usize)
                .unwrap_or(0)
        } else {
            0
        };

        // Auto-generate identity: _0, _1, _2, ...
        let identity = format!("_{}", index);

        // Check if value is already a WasmSubject instance (passthrough)
        if value.is_object() && !value.is_null() {
            let obj: &js_sys::Object = value.unchecked_ref();

            // Check for __wbg_ptr (wasm-bindgen wrapper)
            let is_wasm_subject =
                js_sys::Reflect::has(obj, &JsValue::from_str("__wbg_ptr")).unwrap_or(false);

            if is_wasm_subject {
                // Try to extract as existing WasmSubject
                if let Some(existing) = WasmSubject::from_js_value(&value) {
                    return Ok(existing);
                }
            }
        }

        // Convert based on type
        if value.is_null() || value.is_undefined() {
            return Err(JsValue::from_str(
                "Cannot convert null/undefined to Subject",
            ));
        }

        // Boolean - use "Bool" for pattern-lisp compatibility
        if let Some(b) = value.as_bool() {
            let label = label_override.unwrap_or_else(|| "Bool".to_string());
            let mut properties = HashMap::new();
            properties.insert(value_property, Value::VBoolean(b));

            let mut labels = std::collections::HashSet::new();
            labels.insert(label);

            return Ok(WasmSubject {
                inner: Subject {
                    identity: Symbol(identity),
                    labels,
                    properties,
                },
            });
        }

        // String
        if let Some(s) = value.as_string() {
            let label = label_override.unwrap_or_else(|| "String".to_string());
            let mut properties = HashMap::new();
            properties.insert(value_property, Value::VString(s));

            let mut labels = std::collections::HashSet::new();
            labels.insert(label);

            return Ok(WasmSubject {
                inner: Subject {
                    identity: Symbol(identity),
                    labels,
                    properties,
                },
            });
        }

        // Number
        if let Some(n) = value.as_f64() {
            let label = label_override.unwrap_or_else(|| "Number".to_string());
            let mut properties = HashMap::new();

            // Check if it's a safe integer
            let val = if n.fract() == 0.0 && n >= i64::MIN as f64 && n <= i64::MAX as f64 {
                Value::VInteger(n as i64)
            } else {
                Value::VDecimal(n)
            };
            properties.insert(value_property, val);

            let mut labels = std::collections::HashSet::new();
            labels.insert(label);

            return Ok(WasmSubject {
                inner: Subject {
                    identity: Symbol(identity),
                    labels,
                    properties,
                },
            });
        }

        // For arrays and objects, recommend using Gram.from instead
        if js_sys::Array::is_array(&value) {
            return Err(JsValue::from_str(
                "Arrays should use Gram.from() for pattern-lisp compatible serialization",
            ));
        }

        if value.is_object() {
            return Err(JsValue::from_str(
                "Objects should use Gram.from() for pattern-lisp compatible serialization",
            ));
        }

        Err(JsValue::from_str(&format!(
            "Cannot convert value to Subject: {:?}",
            value
        )))
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
/// Exported to JavaScript as `Pattern` (without the Wasm prefix).
#[wasm_bindgen(js_name = Pattern)]
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
