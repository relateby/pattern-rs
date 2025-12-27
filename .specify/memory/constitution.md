<!--
Sync Impact Report:
Version: [none] → 1.0.0
Modified principles: N/A (initial creation)
Added sections:
  - Core Principles (5 principles)
  - Additional Constraints (multi-target requirements)
  - Development Workflow (reference implementation verification)
Templates requiring updates:
  - ✅ plan-template.md (Constitution Check section aligns with principles)
  - ✅ spec-template.md (no changes needed - compatible with principles)
  - ✅ tasks-template.md (no changes needed - compatible with principles)
Follow-up TODOs: None
-->

# gram-rs Constitution

## Core Principles

### I. Reference Implementation Fidelity
Every feature and function MUST faithfully replicate the behavior of the gram-hs reference implementation. When porting functionality, verify correctness against the Haskell implementation through:
- Behavioral equivalence testing
- Output comparison with reference implementation
- Edge case coverage matching reference behavior
- Documentation alignment with reference semantics

**Rationale**: The gram-hs implementation serves as the authoritative specification. Deviations must be explicitly justified and documented as intentional design decisions, not accidental omissions.

### II. Correctness & Compatibility (NON-NEGOTIABLE)
All implementations MUST prioritize correctness over performance optimizations. Compatibility with the reference implementation's API contracts, data formats, and behavioral guarantees is mandatory. Breaking changes from the reference implementation require explicit justification and documentation.

**Rationale**: This library will be used across multiple target environments (Rust, WASM, external language bindings). Correctness ensures consistent behavior regardless of the target platform, preventing subtle bugs that are difficult to diagnose in cross-platform scenarios.

### III. Rust Native Idioms
While maintaining functional equivalence with gram-hs, the implementation MUST adopt idiomatic Rust patterns:
- Use Rust's type system (enums, Result types, ownership) instead of Haskell-style patterns where appropriate
- Leverage Rust's error handling (Result<T, E>) rather than exceptions or error monads
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Utilize Rust's memory safety guarantees and zero-cost abstractions
- Prefer Rust-native data structures when they provide equivalent functionality

**Rationale**: Idiomatic Rust code is more maintainable, performant, and accessible to Rust developers. It also enables better integration with the Rust ecosystem and tooling.

### IV. Multi-Target Library Design
The library MUST be designed as a shared library that compiles and functions correctly across:
- Native Rust targets (x86_64, ARM, etc.)
- WebAssembly (WASM) targets
- Other target environments as specified

The library MUST NOT include platform-specific code paths unless absolutely necessary. When platform-specific code is required, it MUST be clearly documented and isolated behind feature flags or conditional compilation.

**Rationale**: A shared library design enables code reuse across different deployment scenarios while maintaining a single source of truth for the implementation logic.

### V. External Language Bindings & Examples
The project MUST include minimal, working examples demonstrating usage from external language targets (e.g., JavaScript/TypeScript for WASM, Python, C, etc.). Examples MUST:
- Be minimal and focused on demonstrating core functionality
- Include build/compilation instructions
- Demonstrate basic usage patterns
- Be kept in sync with API changes

**Rationale**: Examples lower the barrier to adoption and serve as living documentation for how to integrate the library into different environments.

## Additional Constraints

### Multi-Target Requirements
- All public APIs MUST be compatible with WASM compilation constraints (no blocking I/O, no file system access unless feature-flagged)
- Dependencies MUST be carefully selected to support all target platforms
- Build configuration MUST support conditional compilation for target-specific features
- Testing MUST include verification on all supported target platforms

### Compatibility Requirements
- API changes that break compatibility with gram-hs reference behavior require explicit documentation and justification
- Version numbering MUST follow semantic versioning to communicate breaking changes
- Migration guides MUST be provided for any intentional behavioral differences from the reference implementation

## Development Workflow

### Reference Implementation Verification
Before marking any feature as complete:
1. Verify behavior matches gram-hs reference implementation
2. Document any intentional deviations with rationale
3. Include test cases that demonstrate equivalence (or document differences)
4. Update examples if API changes affect external language bindings

### Code Review Requirements
- All PRs MUST verify compliance with reference implementation fidelity
- Rust idiom usage MUST be reviewed for appropriateness
- Multi-target compatibility MUST be verified (at minimum, native Rust and WASM)
- Examples for external language bindings MUST be tested if affected by changes

### Testing Discipline
- Unit tests MUST cover core functionality with reference implementation equivalence checks
- Integration tests MUST verify cross-target compatibility
- Examples MUST be tested to ensure they compile and run correctly
- Test coverage SHOULD prioritize correctness-critical paths over edge cases

## Governance

This constitution supersedes all other development practices. Amendments require:
- Documentation of the proposed change
- Rationale for the amendment
- Impact assessment on existing code and examples
- Approval through the project's decision-making process

All PRs and code reviews MUST verify compliance with these principles. Complexity or deviations from the reference implementation MUST be justified. Use project documentation and examples for runtime development guidance.

**Version**: 1.0.0 | **Ratified**: 2025-12-27 | **Last Amended**: 2025-12-27
