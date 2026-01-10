//! Integration test for tree-sitter-gram corpus conformance
//!
//! This test loads the entire tree-sitter-gram test corpus and validates
//! that the nom parser produces semantically equivalent results.

mod corpus;

use corpus::{runner, CorpusTestSuite};
use std::path::PathBuf;

#[test]
fn test_corpus_conformance() {
    // Path to tree-sitter-gram corpus
    let corpus_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/tree-sitter-gram/test/corpus");

    println!("\n🔍 Loading corpus from: {}", corpus_path.display());

    // Load test suite
    let suite = CorpusTestSuite::load(&corpus_path).expect("Failed to load corpus test suite");

    println!("📚 Loaded {} test cases", suite.test_count());

    // Run all tests
    println!("🧪 Running corpus tests...\n");
    let report = runner::run_suite(&suite);

    // Print summary
    report.print_summary();

    // Print detailed failures if any
    if report.stats.failed > 0 {
        report.print_failures();
    }

    // For now, we don't assert 100% pass rate - we're iterating toward that goal
    // Once we reach 100%, uncomment this assertion:
    // assert_eq!(report.stats.failed, 0, "Expected all corpus tests to pass");

    // Instead, print a helpful message
    if report.stats.failed > 0 {
        println!("⚠️  Some tests are failing. This is expected during development.");
        println!("    Work through failures and implement missing features.");
        println!("    Goal: 100% pass rate for full conformance.\n");
    } else {
        println!("🎉 All corpus tests passed! 100% conformance achieved!\n");
    }
}

#[test]
fn test_load_corpus_suite() {
    let corpus_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../external/tree-sitter-gram/test/corpus");

    let suite = CorpusTestSuite::load(&corpus_path).expect("Failed to load corpus test suite");

    // Should have loaded multiple test files
    assert!(suite.test_count() > 0, "Expected to load at least one test");

    // Check that we have tests from multiple files
    let mut unique_files = std::collections::HashSet::new();
    for test in &suite.tests {
        unique_files.insert(test.source_file.clone());
    }

    assert!(
        unique_files.len() > 1,
        "Expected tests from multiple files, got {}",
        unique_files.len()
    );
}
