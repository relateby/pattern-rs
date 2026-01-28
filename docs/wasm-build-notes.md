# Issue: gram-codec WASM Build Fails Due to tree-sitter C Dependencies

**Component**: `gram-codec`  
**Severity**: High - Blocks all WASM-based gram applications  
**Issue Type**: Build Configuration

---

## Problem Summary

`gram-codec` cannot be compiled to WebAssembly (wasm32-unknown-unknown target) using standard Rust WASM tooling (`wasm-pack`, `trunk`) because tree-sitter C dependencies require emscripten or wasi-sdk for compilation.

This blocks any browser-based or WASM-first applications that want to use gram-codec for parsing gram notation.

---

## Steps to Reproduce

1. Clone gram-rs repository
2. Attempt to build gram-codec for WASM:
   ```bash
   cd crates/gram-codec
   wasm-pack build --target web -- --features wasm
   ```

**Result**: Build fails with C compilation error

---

## Error Output

```
error occurred in cc-rs: command did not execute successfully

cargo:warning=In file included from src/./alloc.c:1:
cargo:warning=./alloc.h:9:10: fatal error: 'stdio.h' file not found
cargo:warning=    9 | #include <stdio.h>
cargo:warning=      |          ^~~~~~~~~
cargo:warning=1 error generated.

Error: Compiling your crate to WebAssembly failed
Caused by: failed to execute `cargo build`: exited with exit status: 101
  full command: "cargo" "build" "--lib" "--release" "--target" "wasm32-unknown-unknown" "--features" "wasm"
```

---

## Root Cause

The dependency chain includes C code without WASM build tooling:

```
gram-codec (Rust)
  └─> tree-sitter 0.25 (C code)
  └─> tree-sitter-gram 0.2 (C code)
```

When compiling to wasm32-unknown-unknown:
- Rust code compiles fine with `rustc`
- C code fails because `clang` lacks sysroot headers for WASM target
- Requires emscripten (`emcc`) or wasi-sdk to provide WASM-compatible C standard library

---

## Impact

**Blocks**:
- Browser-based gram editors (e.g., pattern-edit)
- WASM-based CLI tools
- Any web application using gram-codec
- The existing `examples/gram-codec-wasm-web/` example

**Affects**:
- All downstream projects that want to run gram parsing in browsers
- Developer experience (requires additional tooling setup)

---

## Expected Behavior

`wasm-pack build --target web -- --features wasm` should succeed and produce:
- `pkg/gram_codec_bg.wasm` - WASM binary
- `pkg/gram_codec.js` - JS bindings
- `pkg/gram_codec.d.ts` - TypeScript definitions

Developers should be able to use gram-codec in WASM without manual emscripten setup.

---

## Recommended Fix

### Option 1: Document emscripten Requirement (Quick Fix)

Add to gram-rs README and gram-codec README:

```markdown
## Building for WebAssembly

gram-codec depends on tree-sitter's C code, which requires emscripten for WASM compilation.

### Prerequisites

Install emscripten:
```bash
git clone https://github.com/emscripten-core/emsdk.git ~/emsdk
cd ~/emsdk
./emsdk install latest
./emsdk activate latest
source ~/emsdk/emsdk_env.sh
```

### Build WASM

```bash
# Set C compiler for WASM target
export CC_wasm32_unknown_unknown=emcc
export AR_wasm32_unknown_unknown=emar

# Build with wasm-pack
cd crates/gram-codec
wasm-pack build --target web -- --features wasm
```

The output will be in `crates/gram-codec/pkg/`.
```

**Pros**: 
- Quick to implement (documentation only)
- Works with existing tooling
- Standard solution for Rust+C WASM projects

**Cons**: 
- Requires developers to install emscripten (~2GB, 1-2 hour setup)
- Additional build complexity

---

### Option 2: CI/CD Pre-built WASM Artifacts (Best UX)

Set up GitHub Actions to:
1. Install emscripten in CI
2. Build gram-codec WASM on releases
3. Publish as GitHub release artifacts or npm package
4. Downstream projects use pre-built WASM

**Pros**: 
- No emscripten requirement for users
- Clean developer experience
- Can be published to npm for easy consumption

**Cons**: 
- Requires CI/CD setup
- Users can't build from source easily

---

### Option 3: Provide Build Script (Medium Fix)

Create `crates/gram-codec/build-wasm.sh`:

```bash
#!/bin/bash
set -e

# Check for emscripten
if ! command -v emcc &> /dev/null; then
    echo "Error: emscripten not found"
    echo "Install from: https://emscripten.org/docs/getting_started/downloads.html"
    exit 1
fi

# Set compilers
export CC_wasm32_unknown_unknown=emcc
export AR_wasm32_unknown_unknown=emar

# Build
wasm-pack build --target web -- --features wasm

echo "✅ WASM build complete: pkg/"
```

**Pros**: 
- Standardized build process
- Clear error messages
- Easy to document

**Cons**: 
- Still requires emscripten

---

## Alternative: Pure Rust Parser (Not Recommended)

Replace tree-sitter-gram with a pure Rust parser.

**Why not recommended**:
- tree-sitter-gram is the authoritative grammar definition
- Significant implementation effort (2-4 weeks)
- Risk of grammar divergence
- Ongoing maintenance burden
- Would break ecosystem consistency

---

## Testing After Fix

Once WASM builds work, verify:

1. **Build succeeds**:
   ```bash
   wasm-pack build --target web -- --features wasm
   ls pkg/  # Should contain .wasm and .js files
   ```

2. **Example works**:
   ```bash
   cd examples/gram-codec-wasm-web
   python3 -m http.server 8000
   # Open http://localhost:8000 and test parsing
   ```

3. **File size reasonable**:
   ```bash
   ls -lh pkg/*.wasm  # Should be < 2MB uncompressed
   ```

---

## Related

- Existing WASM support: `src/wasm.rs` with `wasm-bindgen` bindings
- Existing example: `examples/gram-codec-wasm-web/`
- `wasm` feature flag already defined in `Cargo.toml`

This suggests WASM support was intended but C compilation was not fully configured.

---

## Additional Context

Downstream project (`pattern-edit`) has:
- ✅ Implemented full Leptos WASM application
- ✅ All components using gram-codec correctly
- ✅ Native Rust builds work perfectly
- ❌ WASM build blocked by this issue

The code is ready and waiting for gram-codec WASM to work.

---

## Proposed Solution (My Recommendation)

**Immediate**: Document emscripten requirement (Option 1)
- Add build instructions to README
- Update examples/gram-codec-wasm-web/README.md
- Note requirement in gram-codec docs

**Long-term**: Set up CI/CD for pre-built WASM (Option 2)
- GitHub Actions workflow with emscripten
- Publish to npm as `@gram-data/gram-codec` (optional)
- Release artifacts contain `pkg/` directory

This gives developers a working path now, with better UX later.

---

## References

- Emscripten: https://emscripten.org/
- wasm-pack: https://rustwasm.github.io/wasm-pack/
- Similar projects: sql.js, ffmpeg.wasm (both use emscripten for C→WASM)
- wasi-sdk: https://github.com/WebAssembly/wasi-sdk (alternative to emscripten)

---

**Labels**: `enhancement`, `WASM`, `build`, `documentation`  
**Priority**: High (blocks WASM use cases)
