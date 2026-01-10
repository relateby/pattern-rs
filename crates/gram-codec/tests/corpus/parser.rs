//! Parser for tree-sitter test corpus .txt files
//!
//! Test corpus format:
//! ```text
//! ==================
//! Test Name
//! ==================
//!
//! input gram notation
//!
//! ---
//!
//! (expected s-expression)
//! ```

use super::CorpusTest;
use std::path::Path;

/// Parse a corpus .txt file into individual test cases
pub fn parse_corpus_file(path: &Path) -> Result<Vec<CorpusTest>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {}: {}", path.display(), e))?;

    parse_corpus_content(&content, path)
}

/// Parse corpus file content into test cases
fn parse_corpus_content(content: &str, source_file: &Path) -> Result<Vec<CorpusTest>, String> {
    let mut tests = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        // Skip blank lines
        if lines[i].trim().is_empty() {
            i += 1;
            continue;
        }

        // Look for test header: ==================
        if lines[i].starts_with("==================") {
            match parse_single_test(&lines, i, source_file) {
                Ok((test, next_line)) => {
                    tests.push(test);
                    i = next_line;
                }
                Err(e) => {
                    eprintln!("Warning: Skipping test at line {}: {}", i + 1, e);
                    // Skip to next potential test header
                    i += 1;
                    while i < lines.len() && !lines[i].starts_with("==================") {
                        i += 1;
                    }
                }
            }
        } else {
            i += 1;
        }
    }

    Ok(tests)
}

/// Parse a single test case starting at the given line
fn parse_single_test(
    lines: &[&str],
    start_line: usize,
    source_file: &Path,
) -> Result<(CorpusTest, usize), String> {
    let mut i = start_line;

    // Line 0: ==================
    if !lines[i].starts_with("==================") {
        return Err(format!("Expected test header at line {}", i + 1));
    }
    i += 1;

    // Line 1: Test Name
    if i >= lines.len() {
        return Err("Unexpected end of file after test header".to_string());
    }
    let test_name = lines[i].trim().to_string();
    i += 1;

    // Line 2: ==================
    if i >= lines.len() || !lines[i].starts_with("==================") {
        return Err(format!("Expected closing header at line {}", i + 1));
    }
    i += 1;

    // Skip blank lines before input
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }

    // Collect input lines until ---
    let mut input_lines = Vec::new();
    while i < lines.len() && !lines[i].starts_with("---") {
        input_lines.push(lines[i]);
        i += 1;
    }

    if i >= lines.len() {
        return Err("Unexpected end of file: no separator '---' found".to_string());
    }

    // Remove trailing blank lines from input
    while !input_lines.is_empty() && input_lines.last().unwrap().trim().is_empty() {
        input_lines.pop();
    }

    let input = input_lines.join("\n");

    // Skip the --- separator
    i += 1;

    // Skip blank lines before expected output
    while i < lines.len() && lines[i].trim().is_empty() {
        i += 1;
    }

    // Collect expected S-expression lines
    let mut sexp_lines = Vec::new();
    let mut found_next_test = false;

    while i < lines.len() {
        if lines[i].starts_with("==================") {
            found_next_test = true;
            break;
        }
        sexp_lines.push(lines[i]);
        i += 1;
    }

    // Remove trailing blank lines from expected output
    while !sexp_lines.is_empty() && sexp_lines.last().unwrap().trim().is_empty() {
        sexp_lines.pop();
    }

    let expected_sexp = sexp_lines.join("\n");

    if input.is_empty() {
        return Err(format!("Test '{}' has empty input", test_name));
    }

    if expected_sexp.is_empty() {
        return Err(format!("Test '{}' has empty expected output", test_name));
    }

    let test = CorpusTest::new(
        test_name,
        source_file.to_path_buf(),
        start_line + 1, // 1-indexed for humans
        input,
        expected_sexp,
    );

    Ok((test, i))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_parse_simple_test() {
        let content = r#"==================
Simple Node
==================

(hello)

---

(gram_pattern
  (node_pattern
    identifier: (symbol)))
"#;

        let tests = parse_corpus_content(content, &PathBuf::from("test.txt")).unwrap();
        assert_eq!(tests.len(), 1);
        assert_eq!(tests[0].name, "Simple Node");
        assert_eq!(tests[0].input, "(hello)");
        assert!(tests[0].expected_sexp.contains("gram_pattern"));
    }

    #[test]
    fn test_parse_multiple_tests() {
        let content = r#"==================
Test One
==================

(a)

---

(gram_pattern
  (node_pattern))

==================
Test Two
==================

(b)

---

(gram_pattern
  (node_pattern))
"#;

        let tests = parse_corpus_content(content, &PathBuf::from("test.txt")).unwrap();
        assert_eq!(tests.len(), 2);
        assert_eq!(tests[0].name, "Test One");
        assert_eq!(tests[1].name, "Test Two");
    }
}
