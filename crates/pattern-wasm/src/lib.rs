#![allow(clippy::all)]

mod convert;
mod gram;

// Re-export WASM-compatible types from pattern-core
pub use pattern_core::wasm::{WasmPattern as Pattern, WasmSubject as Subject, ValueFactory as Value};

// Re-export Gram namespace
pub use gram::Gram;
