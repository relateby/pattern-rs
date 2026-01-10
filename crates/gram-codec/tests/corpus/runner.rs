//! Test runner for corpus test suite

use super::{CorpusTest, CorpusTestResult, CorpusTestSuite};
use std::collections::HashMap;

/// Statistics for a corpus test run
#[derive(Debug, Clone)]
pub struct CorpusTestStats {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub pass_rate: f64,
}

impl CorpusTestStats {
    pub fn from_results(results: &[(CorpusTest, CorpusTestResult)]) -> Self {
        let total = results.len();
        let passed = results.iter().filter(|(_, r)| r.is_pass()).count();
        let failed = total - passed;
        let pass_rate = if total > 0 {
            (passed as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        Self {
            total,
            passed,
            failed,
            pass_rate,
        }
    }
}

/// Detailed report of test results
#[derive(Debug, Clone)]
pub struct CorpusTestReport {
    pub stats: CorpusTestStats,
    pub results: Vec<(CorpusTest, CorpusTestResult)>,
    pub failures_by_file: HashMap<String, Vec<(CorpusTest, CorpusTestResult)>>,
}

impl CorpusTestReport {
    /// Generate a report from test results
    pub fn from_results(results: Vec<(CorpusTest, CorpusTestResult)>) -> Self {
        let stats = CorpusTestStats::from_results(&results);

        // Group failures by file
        let mut failures_by_file: HashMap<String, Vec<(CorpusTest, CorpusTestResult)>> =
            HashMap::new();

        for (test, result) in &results {
            if !result.is_pass() {
                let file_name = test
                    .source_file
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();

                failures_by_file
                    .entry(file_name)
                    .or_insert_with(Vec::new)
                    .push((test.clone(), result.clone()));
            }
        }

        Self {
            stats,
            results,
            failures_by_file,
        }
    }

    /// Print a summary report
    pub fn print_summary(&self) {
        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║           Corpus Test Report                               ║");
        println!("╠════════════════════════════════════════════════════════════╣");
        println!(
            "║  Total Tests:  {:>5}                                      ║",
            self.stats.total
        );
        println!(
            "║  Passed:       {:>5}  (✓)                                 ║",
            self.stats.passed
        );
        println!(
            "║  Failed:       {:>5}  (✗)                                 ║",
            self.stats.failed
        );
        println!(
            "║  Pass Rate:    {:>5.1}%                                    ║",
            self.stats.pass_rate
        );
        println!("╚════════════════════════════════════════════════════════════╝\n");

        if !self.failures_by_file.is_empty() {
            println!("Failed Tests by File:");
            println!("─────────────────────");

            let mut files: Vec<_> = self.failures_by_file.keys().collect();
            files.sort();

            for file in files {
                let failures = &self.failures_by_file[file];
                println!("\n  📄 {} ({} failures)", file, failures.len());

                for (test, _) in failures {
                    println!("     ✗ {} (line {})", test.name, test.line);
                }
            }
            println!();
        }
    }

    /// Print detailed failure information
    pub fn print_failures(&self) {
        if self.stats.failed == 0 {
            return;
        }

        println!("\n╔════════════════════════════════════════════════════════════╗");
        println!("║           Detailed Failure Information                     ║");
        println!("╚════════════════════════════════════════════════════════════╝\n");

        for (test, result) in &self.results {
            if let Some(msg) = result.failure_message() {
                println!("─────────────────────────────────────────────────────────────");
                println!("✗ Test: {}", test.name);
                println!(
                    "  File: {} (line {})",
                    test.source_file.display(),
                    test.line
                );
                println!("\n{}\n", msg);
            }
        }
    }
}

/// Run a corpus test suite and generate a report
pub fn run_suite(suite: &CorpusTestSuite) -> CorpusTestReport {
    let results = suite.run();
    CorpusTestReport::from_results(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::corpus::{CorpusTest, CorpusTestResult};
    use std::path::PathBuf;

    #[test]
    fn test_stats_calculation() {
        let tests = vec![
            (
                CorpusTest::new(
                    "Test 1".to_string(),
                    PathBuf::from("test.txt"),
                    1,
                    "(a)".to_string(),
                    "(gram_pattern)".to_string(),
                ),
                CorpusTestResult::Pass,
            ),
            (
                CorpusTest::new(
                    "Test 2".to_string(),
                    PathBuf::from("test.txt"),
                    10,
                    "(b)".to_string(),
                    "(gram_pattern)".to_string(),
                ),
                CorpusTestResult::ParseError {
                    input: "(b)".to_string(),
                    error: "test error".to_string(),
                },
            ),
        ];

        let stats = CorpusTestStats::from_results(&tests);
        assert_eq!(stats.total, 2);
        assert_eq!(stats.passed, 1);
        assert_eq!(stats.failed, 1);
        assert_eq!(stats.pass_rate, 50.0);
    }
}
