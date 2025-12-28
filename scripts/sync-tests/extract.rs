//! Test case extraction from gram-hs
//!
//! This module provides utilities for extracting test cases from the gram-hs
//! reference implementation and converting them to a format usable by gram-rs.
//!
//! # Using gram-hs CLI for Test Suite Generation
//!
//! The `gram-hs` CLI tool can generate test suites directly:
//!
//! ```bash
//! gram-hs generate --type suite --count 100 --seed 42 --complexity standard \
//!     --format json --value-only > tests/common/test_cases.json
//! ```
//!
//! See [gram-hs CLI Testing Guide](../../../docs/gram-hs-cli-testing-guide.md) for
//! detailed usage examples and integration patterns.

use serde_json::Value;
use std::fs;
use std::path::Path;

/// Validate JSON test case format
pub fn validate_test_case_format(json: &Value) -> Result<(), String> {
    // Placeholder validation - will be fully implemented
    // Validates against test-sync-format.md schema from feature 002
    if json.get("version").is_some() && json.get("test_cases").is_some() {
        Ok(())
    } else {
        Err("Invalid test case format: missing version or test_cases".to_string())
    }
}

/// Extract test cases from JSON file
pub fn extract_test_cases_from_json<P: AsRef<Path>>(path: P) -> Result<Vec<Value>, String> {
    let content = fs::read_to_string(path.as_ref())
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let json: Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    validate_test_case_format(&json)?;
    
    if let Some(test_cases) = json.get("test_cases").and_then(|v| v.as_array()) {
        Ok(test_cases.clone())
    } else {
        Err("No test_cases array found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_test_case_format() {
        let valid_json = serde_json::json!({
            "version": "1.0",
            "test_cases": []
        });
        assert!(validate_test_case_format(&valid_json).is_ok());
    }
}

