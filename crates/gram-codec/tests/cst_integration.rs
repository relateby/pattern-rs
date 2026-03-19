//! Integration tests for the optional CST parser surface.

#[cfg(feature = "cst")]
mod cst;

#[cfg(feature = "cst")]
#[path = "corpus/mod.rs"]
mod corpus;
