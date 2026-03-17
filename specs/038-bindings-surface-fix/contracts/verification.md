# Verification Contract: Surface Alignment

**Feature**: 038-bindings-surface-fix | **Date**: 2026-03-17

## Goal

Prevent public-surface drift between runtime exports, wrappers, generated declarations, shipped stubs, and official developer guidance for TypeScript and Python.

## Required Verification Layers

### 1. Runtime Export Inventory

**TypeScript**:
- Inspect generated WASM targets and the packaged top-level entry point.
- Confirm the documented public symbol set is present and intentionally mapped.

**Python**:
- Inspect the installed wheel’s `relateby.pattern` and `relateby.gram` exports.
- Confirm wrapper-added public symbols are available from the installed package.

### 2. Consumer Import Validation

**TypeScript**:
- Validate intended imports against the packed npm artifact.
- Fail if documented imports require internal module paths.

**Python**:
- Validate intended imports against the built wheel.
- Fail if documented imports require internal/native modules.

### 3. Type and Stub Alignment

**TypeScript**:
- Confirm package-level type declarations agree with the packaged runtime behavior.
- Fail if a declared public method, property, return type, or export family is absent or materially different at runtime.

**Python**:
- Confirm shipped stubs agree with the installed public package behavior.
- Fail if wrapper-owned public symbols are missing from stubs or if stubs advertise runtime-incompatible behavior.

### 4. Executable Guidance Validation

- Execute or otherwise validate official quick-start and representative examples for both languages.
- Fail if examples use unsupported imports, outdated symbol names, stale return shapes, or invalid chaining/setup behavior.

## Minimum Public Workflow Coverage

The verification suite for this feature must cover at least one representative workflow from each category:

1. Package import and initialization
2. Core pattern/value/subject usage
3. Graph-oriented usage
4. Parsing and serialization usage
5. Error-path behavior for invalid input or incomplete setup

## Release Gate

The feature is not release-ready unless all verification layers pass for both languages against packaged artifacts where those artifacts exist.
