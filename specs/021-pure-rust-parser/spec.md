# Feature Specification: Pure Rust Gram Parser

**Feature Branch**: `021-pure-rust-parser`  
**Created**: 2026-01-09  
**Status**: Draft  
**Input**: User description: "Drop the current use of tree-sitter-gram for parsing gram files in gram-codec, in favor of pure Rust implementation based on nom that is checked for conformance against the ../tree-sitter-gram/test/corpus tests but is easy to consumer by downstream projects, particularlt nodejs and python"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Parse Gram with Zero C Dependencies (Priority: P1)

As a downstream developer (Node.js or Python), I need gram-codec to parse gram notation without any C dependencies, so that I can use it in WASM environments (browsers, Node.js) and Python projects without complex build toolchains, native compilation, or platform-specific binaries.

**Why this priority**: The current tree-sitter-gram dependency blocks simple WASM usage because it requires C compilation, emscripten setup, and complex build configurations. This prevents "just works" integration for the primary target platforms (Node.js and Python). Eliminating C dependencies is the foundational requirement that enables all other user stories. Without this, downstream projects cannot easily consume gram-codec.

**Independent Test**: Can be fully tested by building gram-codec for WASM (wasm32-unknown-unknown target) using standard wasm-pack commands with no additional build configuration, custom scripts, or C toolchains. Success means: (1) `wasm-pack build --target web` succeeds without errors, (2) generated WASM loads and runs in browser with simple HTTP server, (3) generated WASM loads and runs in Node.js with standard ES module imports, (4) no emscripten, LLVM, or C compiler required. Delivers immediate value by making gram-codec consumable in JavaScript/TypeScript projects.

**Acceptance Scenarios**:

1. **Given** a developer runs `wasm-pack build --target web crates/gram-codec --features wasm`, **When** building gram-codec, **Then** the build completes successfully without requiring emscripten, LLVM, or any C toolchain
2. **Given** a developer builds gram-codec for WASM, **When** inspecting the build dependencies, **Then** only Rust crates are required (no tree-sitter, no C code, no build.rs C compilation)
3. **Given** WASM artifacts are built, **When** loading them in a browser with a simple HTTP server, **Then** the module loads without errors and parse_gram function is available
4. **Given** WASM artifacts are built, **When** importing in Node.js as ES modules, **Then** the module loads without errors and all API functions work
5. **Given** a developer has only Rust and wasm-pack installed, **When** building gram-codec for WASM, **Then** no additional tools (emscripten, clang, etc.) are needed
6. **Given** gram-codec is compiled to WASM, **When** measuring the binary size, **Then** it is under 500KB (gzipped) due to lack of C runtime overhead
7. **Given** parsing gram notation in WASM, **When** parse_gram is called, **Then** it completes successfully and returns correct Pattern structures identical to native Rust behavior

---

### User Story 2 - Verify Parser Conformance with Tree-Sitter Tests (Priority: P2)

As a gram-codec maintainer, I need the pure Rust parser to pass all tree-sitter-gram test corpus cases, so that I can be confident the parser correctly implements the gram grammar specification and handles all syntax forms, edge cases, and error scenarios defined in the authoritative grammar.

**Why this priority**: The tree-sitter-gram test corpus represents the authoritative specification of valid and invalid gram notation. Parser conformance ensures correctness and completeness. While not blocking initial integration (P1), conformance testing is essential before releasing the pure Rust parser as the default implementation. Without this validation, there's risk of subtle parsing differences or missing syntax support.

**Independent Test**: Can be fully tested by running the tree-sitter-gram test corpus (`../tree-sitter-gram/test/corpus/`) against the pure Rust parser and verifying that all test expectations are met. Testing includes: (1) programmatic execution of corpus tests, (2) comparison of parse results against expected outcomes, (3) reporting test pass/fail rates, (4) identifying any deviations from tree-sitter-gram behavior. Delivers value by proving parser correctness and identifying any gaps in grammar support.

**Acceptance Scenarios**:

1. **Given** the tree-sitter-gram test corpus at `../tree-sitter-gram/test/corpus/`, **When** running these tests against the pure Rust parser, **Then** 100% of valid gram notation tests parse successfully
2. **Given** tree-sitter-gram tests with invalid gram notation, **When** running these tests, **Then** the pure Rust parser correctly reports errors for all invalid cases
3. **Given** tree-sitter-gram tests with expected parse trees, **When** comparing Rust parser output, **Then** the resulting Pattern structures match the expected semantic structure
4. **Given** tree-sitter-gram tests for all syntax forms (nodes, relationships, subject patterns, annotations), **When** parsing with pure Rust, **Then** each syntax form is correctly recognized and parsed
5. **Given** tree-sitter-gram tests with edge cases (deeply nested structures, Unicode, special characters), **When** parsing with pure Rust, **Then** all edge cases are handled correctly
6. **Given** a test harness for corpus testing, **When** running the full test suite, **Then** a detailed report shows pass/fail status for each test case with clear error messages for failures
7. **Given** any failing test cases, **When** reviewing the failures, **Then** the test harness provides diff output showing expected vs actual parse results

---

### User Story 3 - WASM Integration Works Out-of-the-Box (Priority: P3)

As a JavaScript/TypeScript developer, I need to use gram-codec in my web application or Node.js project with standard bundlers and build tools, so that I can parse and serialize gram notation without custom build scripts, environment configuration, or workarounds for C dependencies.

**Why this priority**: After eliminating C dependencies (P1) and verifying correctness (P2), this ensures the practical developer experience is smooth. Standard bundlers (Vite, Webpack, Parcel) and Node.js environments should "just work" with the published npm package. This is the user-facing success criterion that proves the architectural change was successful.

**Independent Test**: Can be fully tested by creating sample projects with popular bundlers (Vite, Webpack) and Node.js, installing gram-codec from its published package, and verifying that parsing and serialization work without any additional configuration. Testing includes: (1) Vite web app example, (2) Node.js ES module example, (3) CommonJS example, (4) verification that no build errors or runtime errors occur. Delivers value by proving the package is consumable in real-world JavaScript/TypeScript projects.

**Acceptance Scenarios**:

1. **Given** a Vite web app project, **When** installing gram-codec via npm and importing it, **Then** the module loads and works without additional vite configuration or plugins
2. **Given** a Node.js project using ES modules, **When** importing gram-codec, **Then** parse_gram and serialize_pattern functions work correctly
3. **Given** a Node.js project using CommonJS, **When** requiring gram-codec, **Then** all API functions are available and work correctly
4. **Given** a Webpack project, **When** building with gram-codec, **Then** the build succeeds and the bundled app works without special WASM configuration
5. **Given** a browser environment, **When** loading gram-codec WASM, **Then** initialization completes in under 100ms and the module is ready to use
6. **Given** gram-codec published to npm, **When** a developer installs it, **Then** the package includes working TypeScript definitions and examples
7. **Given** working examples in the repository, **When** developers follow the examples, **Then** they can successfully integrate gram-codec into their projects within 10 minutes

---

### User Story 4 - Python Bindings Work with PyO3 (Priority: P4)

As a Python developer, I need to use gram-codec in my Python application, so that I can parse and serialize gram notation using a native Python module that is easy to install via pip and performs well for batch processing and data analysis workflows.

**Why this priority**: Python is a major target platform (mentioned explicitly in the user description). While less urgent than WASM support (P3), Python bindings are still essential for the gram ecosystem. PyO3 is the standard solution for Rust-Python integration and should work well with pure Rust code. This is lower priority because WASM/JavaScript is more immediately impacted by the C dependency issue.

**Independent Test**: Can be fully tested by building gram-codec with PyO3 bindings, publishing the Python package, installing it via pip, and verifying that all parsing and serialization functions work in Python. Testing includes: (1) building Python wheels for major platforms (macOS, Linux, Windows), (2) pip installation succeeds, (3) import gram_codec works, (4) API functions (parse, serialize, round_trip) work correctly. Delivers value by making gram-codec available to Python data science, automation, and backend development communities.

**Acceptance Scenarios**:

1. **Given** gram-codec with PyO3 bindings enabled, **When** building for Python, **Then** the build completes successfully using standard maturin or setuptools-rust tooling
2. **Given** a Python wheel is built, **When** installing via `pip install gram-codec`, **Then** installation succeeds on macOS, Linux, and Windows without requiring Rust toolchain or C compiler
3. **Given** gram-codec installed in Python, **When** running `import gram_codec`, **Then** the module loads without errors
4. **Given** the Python API, **When** calling `gram_codec.parse("(hello)")`, **Then** it returns a Pattern-like Python object with correct structure
5. **Given** a Pattern object in Python, **When** calling `gram_codec.serialize(pattern)`, **Then** it returns valid gram notation as a string
6. **Given** Python type hints, **When** using gram-codec in type-checked Python code, **Then** type hints accurately describe function signatures and return types
7. **Given** gram-codec documentation, **When** Python developers read it, **Then** clear examples show how to use the module for common tasks (parse, serialize, validate)

---

### Edge Cases

- What happens when the nom parser encounters malformed gram notation with subtle syntax errors that tree-sitter might handle differently?
- What happens when parsing very large gram files (megabytes) with thousands of patterns - does pure Rust parser maintain performance comparable to tree-sitter?
- What happens when the tree-sitter-gram test corpus is updated with new syntax forms - how is the pure Rust parser updated to match?
- What happens during the migration period when both tree-sitter and nom parsers exist - how do we ensure behavioral consistency?
- What happens when WASM memory is constrained - does the pure Rust parser handle memory allocation gracefully?
- What happens when Python bindings are built for different Python versions (3.8, 3.9, 3.10, 3.11, 3.12) - are they all compatible?
- What happens when npm package size needs to be minimized - can WASM binary be further optimized?
- What happens when developers encounter parsing errors - does the nom parser provide error messages as helpful as tree-sitter?
- What happens when the gram grammar evolves - is nom flexible enough to adapt to grammar changes?
- What happens when downstream projects have conflicting dependencies - does pure Rust reduce dependency conflicts compared to tree-sitter?

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST implement a pure Rust parser using the nom parser combinator library with no C dependencies
- **FR-002**: Parser MUST support the complete gram grammar as defined in tree-sitter-gram, including all syntax forms: nodes, relationships, subject patterns, annotations, comments, all value types (strings, numbers, booleans, arrays, ranges, tagged strings)
- **FR-003**: Parser MUST compile to wasm32-unknown-unknown target using standard Rust WASM toolchain (no emscripten, no C compiler required)
- **FR-004**: System MUST provide a test harness that runs tree-sitter-gram test corpus (`../tree-sitter-gram/test/corpus/`) against the pure Rust parser
- **FR-005**: Test harness MUST report pass/fail status for each corpus test case, with detailed error information (expected vs actual) for failures
- **FR-006**: Parser MUST achieve 100% conformance with tree-sitter-gram valid syntax tests (all valid gram notation parses successfully)
- **FR-007**: Parser MUST achieve 100% conformance with tree-sitter-gram invalid syntax tests (all invalid gram notation is rejected with errors)
- **FR-008**: Parser MUST provide error messages with location information (line, column) and descriptive error details (expected vs found tokens)
- **FR-009**: System MUST build for WASM targets using `wasm-pack build` with no additional build scripts or environment configuration
- **FR-010**: WASM build MUST produce artifacts (wasm binary, JS bindings, TypeScript definitions) that load and work in browsers and Node.js
- **FR-011**: WASM binary size MUST be under 500KB when gzipped
- **FR-012**: System MUST support Python bindings via PyO3 feature flag, allowing compilation to native Python extension modules
- **FR-013**: Python bindings MUST expose parse, serialize, and validate functions with Python-friendly signatures
- **FR-014**: System MUST maintain the same public API (function signatures, return types) as the current gram-codec to minimize breaking changes
- **FR-015**: Parser performance MUST be comparable to tree-sitter-gram for typical gram files (parse 1000 patterns in under 100ms)
- **FR-016**: System MUST include examples demonstrating: (1) WASM usage in browsers, (2) Node.js ES module usage, (3) Python usage
- **FR-017**: System MUST provide migration documentation explaining how to switch from tree-sitter to pure Rust parser, including any API changes or behavioral differences
- **FR-018**: Parser MUST handle Unicode characters, emoji, and international characters identically to tree-sitter-gram
- **FR-019**: System MUST completely remove tree-sitter-gram dependency in favor of the nom parser (immediate hard switch with no coexistence period or fallback mechanism)
- **FR-020**: nom parser implementation MUST be modular and well-documented to facilitate future grammar updates

### Key Entities

- **nom Parser**: Pure Rust parser combinator library providing zero-copy parsing with no C dependencies. Compiles cleanly to WASM and works seamlessly with PyO3. Used to implement all gram grammar rules.
- **gram-codec**: The Rust crate containing the parser and serializer. Currently depends on tree-sitter-gram (C code). Will be refactored to use pure Rust nom-based parser instead.
- **tree-sitter-gram**: Reference grammar specification and test corpus located at `../tree-sitter-gram/`. Used as the authoritative definition of gram syntax and expected parse behavior, but not used as a code dependency.
- **Test Corpus**: Collection of test cases in `../tree-sitter-gram/test/corpus/` defining valid and invalid gram notation examples with expected parse results. Used to verify pure Rust parser conformance.
- **WASM Target**: WebAssembly compilation target (wasm32-unknown-unknown) that enables gram-codec to run in browsers and Node.js without native binaries or platform-specific compilation.
- **PyO3**: Rust-to-Python binding framework enabling gram-codec to be packaged as a native Python extension module installable via pip.
- **Pattern**: The core data structure representing parsed gram notation, consisting of Subject (value with identifier, labels, properties) and elements (nested Patterns).

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Developers can build gram-codec for WASM using only `wasm-pack build --target web` with no additional tools or configuration (zero C dependencies verified)
- **SC-002**: WASM build produces artifacts under 500KB (gzipped) that load in browsers and Node.js with standard import statements
- **SC-003**: Pure Rust parser achieves 100% pass rate on tree-sitter-gram valid syntax test corpus (all valid gram notation parses correctly)
- **SC-004**: Pure Rust parser achieves 100% pass rate on tree-sitter-gram invalid syntax test corpus (all invalid gram notation is rejected)
- **SC-005**: Parser performance is within 20% of tree-sitter-gram baseline (measured by parsing 1000 pattern test file in under 120ms vs tree-sitter's ~100ms)
- **SC-006**: Working examples demonstrate WASM usage in Vite/Webpack projects and Node.js within 10 minutes of setup (from npm install to successful parse_gram call)
- **SC-007**: Python bindings build successfully for macOS, Linux, and Windows, and install via pip without requiring Rust toolchain on end-user machines
- **SC-008**: Python API functions (parse, serialize, validate) work correctly in Python 3.8+ with performance within 50% of native Rust (acceptable overhead for convenience)
- **SC-009**: Migration from tree-sitter to nom parser is completed without breaking existing API contracts (same function signatures, same Pattern structures)
- **SC-010**: Developer experience is improved as measured by: elimination of build errors related to C compilation, removal of platform-specific build requirements, and simplified installation instructions in README

## Assumptions

- nom parser combinator library is capable of expressing the complete gram grammar with acceptable performance and error reporting quality
- tree-sitter-gram test corpus accurately represents all valid gram syntax and provides sufficient coverage for conformance testing
- The test corpus is accessible at `../tree-sitter-gram/test/corpus/` relative to the gram-rs repository root
- Pattern data structures and public API of gram-codec remain unchanged (only the parser implementation changes, not the data model)
- Downstream projects (Node.js and Python) prefer ease of installation over absolute peak performance (within 20-50% of native performance is acceptable)
- Standard Rust WASM toolchain (rustc, wasm-pack) provides sufficient optimization for WASM binary size under 500KB gzipped
- PyO3 is the appropriate choice for Python bindings (not pyo3-pack alternatives or ctypes FFI)
- Migration can be completed in a single release (no extended period of supporting both tree-sitter and nom in parallel unless needed for validation)
- Behavioral differences between tree-sitter and nom parsers (if any) can be resolved by aligning nom implementation with tree-sitter-gram corpus expectations
- Error messages from nom parser can be made comparable in quality to tree-sitter's error reporting (location information, helpful descriptions)
- The build complexity reduction outweighs any potential performance tradeoffs of pure Rust vs C parsing
- Existing users of gram-codec (if any) can adopt the new parser with minimal migration effort (ideally zero breaking changes)
- tree-sitter-gram repository remains stable as the authoritative grammar reference and test corpus
