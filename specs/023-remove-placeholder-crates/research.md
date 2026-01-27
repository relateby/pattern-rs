# Research: Remove Unused Placeholder Crates

**Feature**: 023-remove-placeholder-crates  
**Date**: 2026-01-27

## Research Tasks

### Task 1: Verify Crate Usage and Dependencies

**Question**: Are the placeholder crates (pattern-store, pattern-ops, pattern-wasm) actually unused?

**Research Method**: 
- Search codebase for references to these crates
- Check Cargo.toml files for dependencies
- Verify workspace member configuration

**Findings**:
- ✅ No dependencies found: Searched entire codebase - no Cargo.toml files reference these crates as dependencies
- ✅ Workspace uses wildcard: Root Cargo.toml uses `members = ["crates/*", "benches"]`, so removal doesn't require manual member list updates
- ✅ Only self-references: Each placeholder crate only contains minimal placeholder code with no external usage

**Decision**: All three crates (pattern-store, pattern-ops, pattern-wasm) are confirmed unused and safe to remove.

**Rationale**: Comprehensive search confirms zero dependencies. Workspace wildcard configuration means removal is automatic.

**Alternatives Considered**: 
- Keep crates with "TODO" comments: Rejected - creates confusion about available functionality
- Move to separate "future" directory: Rejected - unnecessary complexity, git history preserves them

---

### Task 2: Documentation Reference Audit

**Question**: Are there any documentation references to these placeholder crates that need updating?

**Research Method**:
- Search documentation files for crate names
- Check README files
- Review spec files and architecture documents

**Findings**:
- ✅ Found references in: README.md, TODO.md, specs/021-pure-rust-parser/ARCHITECTURE.md, specs/021-pure-rust-parser/DECISIONS.md, specs/021-pure-rust-parser/AST-DESIGN.md
- ✅ References are mostly architectural discussions or future plans
- ✅ No API documentation references these crates

**Decision**: Update documentation to remove or clarify references to removed crates.

**Rationale**: Documentation should reflect current state. References in architectural docs can be updated to note these were placeholders that were removed.

**Alternatives Considered**:
- Leave documentation as-is: Rejected - creates confusion for developers reading docs
- Add deprecation notices: Rejected - crates never had functionality to deprecate

---

### Task 3: Git History Preservation

**Question**: How should we handle git history for removed crates?

**Research Method**:
- Review git best practices for removing directories
- Consider impact on repository size and history

**Findings**:
- ✅ Git preserves history: Removing files/directories doesn't delete git history
- ✅ History remains accessible: Can view removed files via `git log --all --full-history -- <path>`
- ✅ No special handling needed: Standard `git rm` or file deletion preserves history

**Decision**: Use standard git removal process. History is automatically preserved.

**Rationale**: Git's design preserves history for removed files. No special process needed.

**Alternatives Considered**:
- Archive crates to separate branch: Rejected - unnecessary, history already preserved
- Use git submodule: Rejected - adds complexity for unused code

---

## Summary

All research confirms that removing the placeholder crates is safe and straightforward:
1. ✅ No dependencies exist
2. ✅ Workspace configuration doesn't require manual updates
3. ✅ Documentation references can be easily updated
4. ✅ Git history is automatically preserved

No blockers or concerns identified. Proceeding with removal.
