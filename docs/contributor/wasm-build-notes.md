# WASM Build Notes for gram-codec

**Component**: `gram-codec`  
**Status**: ✅ WASM builds work — no special tooling required

---

## Summary

`gram-codec` compiles cleanly to `wasm32-unknown-unknown` using standard `cargo` and `wasm-pack`. The crate uses a pure Rust `nom`-based parser with no C dependencies.

```bash
# Standard build — works as-is
cargo build -p gram-codec --target wasm32-unknown-unknown --features wasm
```

---

## Dependency Chain

`gram-codec` has no tree-sitter dependencies:

```
gram-codec (Rust)
  ├── pattern-core (Rust)
  ├── nom 7 (pure Rust)
  ├── serde / serde_json (pure Rust)
  ├── thiserror (pure Rust)
  └── [optional] wasm-bindgen / js-sys (WASM feature)
```

Tree-sitter lives exclusively in `external/tree-sitter-gram/`, which is a standalone
external grammar used for the `gram-lint` CLI tool and compliance testing. It is not
a dependency of `gram-codec` and is not part of the Cargo workspace.

---

## Building for WASM

### Prerequisites

Ensure the WASM target is installed for the **rustup-managed** toolchain (not a
Homebrew-installed `rustc`, which does not include the WASM std):

```bash
rustup target add wasm32-unknown-unknown
```

Verify the correct `rustc` is on your PATH:

```bash
which rustc          # Should be ~/.rustup/toolchains/.../bin/rustc
                     # NOT /usr/local/bin/rustc (Homebrew)
rustc --version
```

If Homebrew's `rustc` takes precedence, prefix your build commands:

```bash
export PATH="$HOME/.rustup/toolchains/stable-$(rustup show active-toolchain | cut -d' ' -f1 | sed 's/stable-//')/bin:$PATH"
# Or simply use: rustup run stable cargo build ...
```

### cargo build

```bash
# Without WASM bindings (pure Rust, no JS interop)
cargo build -p gram-codec --target wasm32-unknown-unknown

# With wasm-bindgen bindings (JS/TS interop)
cargo build -p gram-codec --target wasm32-unknown-unknown --features wasm
```

### wasm-pack

```bash
cd crates/gram-codec
wasm-pack build --target web -- --features wasm
# Output: pkg/gram_codec_bg.wasm, pkg/gram_codec.js, pkg/gram_codec.d.ts
```

---

## Common Pitfall: Homebrew rustc

macOS users who installed Rust via Homebrew (`brew install rust`) may have
`/usr/local/bin/rustc` on their PATH before the rustup toolchain. Homebrew's
`rustc` does not include the `wasm32-unknown-unknown` std, so builds fail with:

```
error[E0463]: can't find crate for `std`
  = note: the `wasm32-unknown-unknown` target may not be installed
```

This error is misleading — the target *is* installed, but for the rustup toolchain,
not the Homebrew one. The fix is to ensure the rustup `rustc` is used (see above).

---

## Historical Note

An earlier document in this repository described a WASM build blocker caused by
tree-sitter C dependencies. That description was inaccurate: `gram-codec` never
depended on tree-sitter. The parser was always implemented in pure Rust using `nom`.
The document has been replaced with this one.
