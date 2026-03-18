//! Minimal compatibility shim for the legacy `pattern_core` Python extension.
//!
//! The supported Python API now lives in the pure-Python `relateby.pattern`
//! package. This module remains only so the combined wheel can continue to ship
//! a `pattern_core` extension during the cutover.

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn pattern_core(_py: Python, _m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
