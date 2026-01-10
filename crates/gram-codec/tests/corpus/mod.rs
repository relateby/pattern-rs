//! Corpus test infrastructure for tree-sitter-gram conformance testing
//!
//! This module provides infrastructure to load and run test cases from the
//! tree-sitter-gram test corpus, validating that the nom parser produces
//! semantically equivalent results.

use std::path::{Path, PathBuf};

pub mod parser;
pub mod runner;
pub mod validator;

/// A single test case from the corpus
#[derive(Debug, Clone, PartialEq)]
pub struct CorpusTest {
    /// Test name (from the header)
    pub name: String,

    /// Source file this test came from
    pub source_file: PathBuf,

    /// Line number where test starts
    pub line: usize,

    /// Input gram notation to parse
    pub input: String,

    /// Expected S-expression output from tree-sitter
    pub expected_sexp: String,
}

impl CorpusTest {
    /// Create a new corpus test
    pub fn new(
        name: String,
        source_file: PathBuf,
        line: usize,
        input: String,
        expected_sexp: String,
    ) -> Self {
        Self {
            name,
            source_file,
            line,
            input,
            expected_sexp,
        }
    }

    /// Run this test using the nom parser
    pub fn run(&self) -> CorpusTestResult {
        use gram_codec::parse_gram;

        // Try to parse with nom parser
        match parse_gram(&self.input) {
            Ok(patterns) => {
                // Successful parse - validate against expected S-expression
                match validator::validate_patterns(&patterns, &self.expected_sexp) {
                    Ok(()) => CorpusTestResult::Pass,
                    Err(msg) => CorpusTestResult::Mismatch {
                        expected: self.expected_sexp.clone(),
                        actual: format!("{:?}", patterns),
                        message: msg,
                    },
                }
            }
            Err(e) => CorpusTestResult::ParseError {
                input: self.input.clone(),
                error: format!("{}", e),
            },
        }
    }
}

/// Result of running a corpus test
#[derive(Debug, Clone)]
pub enum CorpusTestResult {
    /// Test passed - parsed correctly and matches expected structure
    Pass,

    /// Test parsed but structure doesn't match expected S-expression
    Mismatch {
        expected: String,
        actual: String,
        message: String,
    },

    /// Test failed to parse (but should have)
    ParseError { input: String, error: String },
}

impl CorpusTestResult {
    /// Check if test passed
    pub fn is_pass(&self) -> bool {
        matches!(self, CorpusTestResult::Pass)
    }

    /// Get failure message if test failed
    pub fn failure_message(&self) -> Option<String> {
        match self {
            CorpusTestResult::Pass => None,
            CorpusTestResult::Mismatch {
                expected,
                actual,
                message,
            } => Some(format!(
                "Structure mismatch:\n{}\n\nExpected:\n{}\n\nActual:\n{}",
                message, expected, actual
            )),
            CorpusTestResult::ParseError { input, error } => {
                Some(format!("Parse failed for:\n{}\n\nError: {}", input, error))
            }
        }
    }
}

/// Collection of corpus tests from multiple files
#[derive(Debug, Clone)]
pub struct CorpusTestSuite {
    /// All tests in the suite
    pub tests: Vec<CorpusTest>,

    /// Root directory of the corpus
    pub corpus_dir: PathBuf,
}

impl CorpusTestSuite {
    /// Create an empty test suite
    pub fn new(corpus_dir: PathBuf) -> Self {
        Self {
            tests: Vec::new(),
            corpus_dir,
        }
    }

    /// Load all tests from the corpus directory
    pub fn load(corpus_dir: impl AsRef<Path>) -> Result<Self, String> {
        let corpus_dir = corpus_dir.as_ref();

        if !corpus_dir.exists() {
            return Err(format!(
                "Corpus directory does not exist: {}",
                corpus_dir.display()
            ));
        }

        let mut suite = Self::new(corpus_dir.to_path_buf());

        // Find all .txt files in corpus directory
        let txt_files = std::fs::read_dir(corpus_dir)
            .map_err(|e| format!("Failed to read corpus directory: {}", e))?
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "txt")
                    .unwrap_or(false)
            })
            .map(|entry| entry.path())
            .collect::<Vec<_>>();

        // Load tests from each file
        for file_path in txt_files {
            match parser::parse_corpus_file(&file_path) {
                Ok(tests) => suite.tests.extend(tests),
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", file_path.display(), e);
                }
            }
        }

        Ok(suite)
    }

    /// Get the number of tests in the suite
    pub fn test_count(&self) -> usize {
        self.tests.len()
    }

    /// Run all tests and return results
    pub fn run(&self) -> Vec<(CorpusTest, CorpusTestResult)> {
        self.tests
            .iter()
            .map(|test| (test.clone(), test.run()))
            .collect()
    }
}
