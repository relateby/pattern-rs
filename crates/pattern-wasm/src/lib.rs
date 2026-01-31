#![allow(clippy::all)]

mod convert;
mod gram;

// Re-export WASM-compatible types from pattern-core
pub use pattern_core::wasm::{
    ValueFactory as Value, WasmPattern as Pattern, WasmSubject as Subject,
};

// Re-export Gram namespace
pub use gram::Gram;
