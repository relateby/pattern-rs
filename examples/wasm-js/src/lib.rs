//! WASM/JavaScript example for gram library
//!
//! This example demonstrates how to use the gram library from JavaScript/TypeScript
//! in a WebAssembly environment.

use wasm_bindgen::prelude::*;

/// Example function demonstrating library usage from WASM
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! (from gram-rs)", name)
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

