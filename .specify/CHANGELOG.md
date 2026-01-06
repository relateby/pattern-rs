# .specify Directory Changelog

All notable changes to the `.specify` templates, constitution, and scripts will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [1.0.2] - 2026-01-05

### Added

#### Code Quality Verification in Feature Workflow

- **Constitution** (`memory/constitution.md`):
  - Added explicit code quality checks to Development Workflow (steps 7-8)
  - Requirements before marking features complete:
    - Format check: `cargo fmt --all -- --check`
    - Linting: `cargo clippy --workspace -- -D warnings`
    - CI validation: `scripts/ci-local.sh`
    - Test verification: `cargo test --workspace`
  
- **Tasks Template** (`templates/tasks-template.md`):
  - Expanded "Polish & Cross-Cutting Concerns" phase with structured sections:
    - Documentation & Examples
    - Code Quality
    - **Code Quality Checks (REQUIRED)** - New section with explicit steps
    - Performance & Optimization
    - Final Verification
  - Added specific tasks for:
    - Running formatters (`cargo fmt --all`)
    - Running linters (`cargo clippy --workspace -- -D warnings`)
    - Running CI checks (`scripts/ci-local.sh`)
    - Verifying tests (`cargo test --workspace`)
    - Fixing any failures before completion
  - Added CHANGELOG.md and TODO.md update tasks to Final Verification

### Changed

- Updated constitution version from 1.0.1 to 1.0.2
- Last amended date updated to 2026-01-05

### Rationale

The Comonad implementation (feature 018) revealed that explicit code quality checks were being performed but not formally documented in the workflow. This update ensures:
1. All features follow consistent code quality standards
2. Formatting is checked before completion (prevents CI failures)
3. The `scripts/ci-local.sh` script is utilized as intended
4. Feature completion criteria are clear and comprehensive

## [1.0.1] - 2025-12-27

### Added

- Reference Implementation Location section in constitution
- Local path details for gram-hs reference (`../gram-hs`)

### Changed

- Updated I. Reference Implementation Fidelity principle
- Updated Development Workflow verification steps
- Updated plan-template.md Constitution Check section

### Rationale

Made the local gram-hs reference implementation path explicit to facilitate easier porting from Haskell to Rust.

## [1.0.0] - 2025-12-27

### Initial Version

- Core principles established
- Templates created for specs, plans, tasks, checklists
- Development workflow defined
- Constitution ratified

[1.0.2]: https://github.com/gram-data/gram-rs/compare/v1.0.1...v1.0.2
[1.0.1]: https://github.com/gram-data/gram-rs/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/gram-data/gram-rs/releases/tag/v1.0.0

