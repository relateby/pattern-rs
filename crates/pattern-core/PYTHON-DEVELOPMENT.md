# Python Development Guide for pattern-core

## Recommended Tool: uv (Preferred)

This project uses **[uv](https://github.com/astral-sh/uv)** - an extremely fast Python package installer and resolver written in Rust by Astral.

### Why uv?

- ⚡ **10-100x faster** than pip
- 🦀 **Written in Rust** - perfect for Rust+Python projects
- 📦 **pyproject.toml native** - first-class support
- 🔒 **Built-in venv management** - no manual activation needed
- 🎯 **Drop-in pip replacement** - same commands
- 🌐 **Universal resolver** - handles complex dependencies better

### Installing uv

```bash
# macOS/Linux
curl -LsSf https://astral.sh/uv/install.sh | sh

# Or via pip (if you prefer)
pip install uv

# Or via cargo (you already have Rust!)
cargo install --git https://github.com/astral-sh/uv uv
```

## Quick Start with uv (Recommended)

```bash
# Navigate to crate
cd crates/pattern-core

# Create venv and install dev dependencies (one command!)
uv venv
uv pip install -e ".[dev]"

# Activate venv (optional - uv commands work without activation)
source .venv/bin/activate  # macOS/Linux
# .venv\Scripts\activate    # Windows

# Build the extension
maturin develop --features python

# Run tests
pytest tests/python/ -v
```

## Installation Strategies

### Strategy 1: uv + Project Dependencies (Recommended)

Maturin is declared in `pyproject.toml` as both a **build dependency** and **dev dependency**:

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]  # Used during build

[project.optional-dependencies]
dev = ["maturin>=1.0,<2.0", "pytest>=7.0", ...]  # For development
```

**Complete setup with uv:**

```bash
cd crates/pattern-core

# Create venv (uv auto-detects .venv)
uv venv

# Install dependencies (automatically creates and uses .venv)
uv pip install -e ".[dev]"

# Verify installation
source .venv/bin/activate  # Optional
which maturin  # Should point to .venv/bin/maturin
maturin --version

# Build and test
maturin develop --features python
pytest tests/python/ -v
```

**Benefits:**
- ✅ **Blazing fast** - 10-100x faster than pip
- ✅ **Version pinned** per project (reproducible)
- ✅ **Isolated** in virtual environment
- ✅ **CI/CD compatible** - same commands everywhere
- ✅ **Declarative** - dependencies in pyproject.toml
- ✅ **Rust-friendly** - built by Rust developers for Rust+Python

### Strategy 2: pip + venv (Traditional Alternative)

If you prefer pip or don't want to install uv:

```bash
cd crates/pattern-core

# Create and activate virtual environment
python -m venv .venv
source .venv/bin/activate  # macOS/Linux
# .venv\Scripts\activate    # Windows

# Install with dev dependencies
pip install -e ".[dev]"

# Build and test
maturin develop --features python
pytest tests/python/ -v
```

**Comparison with uv:**
- ⚠️ **Slower** - pip can take 10-100x longer
- ✅ **Widely known** - standard Python tooling
- ⚠️ **Manual venv** - must activate before each session
- ✅ **No extra install** - comes with Python

### Strategy 3: uvx for Global CLI (Advanced)

If you use maturin as a **CLI tool** across many projects:

```bash
# Install maturin globally with uvx (isolated)
uvx maturin --version

# Or use pipx
pipx install maturin

# Still use uv for project dependencies
cd crates/pattern-core
uv venv
uv pip install -e ".[dev]"
```

**When to use:**
- You frequently create new Rust+Python projects
- You want maturin CLI always available
- You still use venv for project-specific deps

## Recommended Workflow

### Initial Setup (with uv)

```bash
# Navigate to crate
cd crates/pattern-core

# Create virtual environment and install dependencies
uv venv
uv pip install -e ".[dev]"

# Verify installation
source .venv/bin/activate  # Optional
which maturin  # Should point to .venv/bin/maturin
maturin --version
```

### Development Loop (with uv)

```bash
# After changing Rust code:
maturin develop --features python

# Run tests (uv auto-activates venv for installed tools)
pytest tests/python/ -v

# Run specific test file
pytest tests/python/test_operations.py -v

# Run with coverage
pytest tests/python/ --cov=pattern_core --cov-report=term-missing

# Update dependencies (if pyproject.toml changes)
uv pip install -e ".[dev]"
```

### Development Loop (with pip)

```bash
# Ensure venv is activated
source .venv/bin/activate  # Required with pip

# After changing Rust code:
maturin develop --features python

# Run tests
pytest tests/python/ -v

# Run specific test file
pytest tests/python/test_operations.py -v

# Run with coverage
pytest tests/python/ --cov=pattern_core --cov-report=term-missing
```

### CI/CD Setup

#### Option 1: With uv (Faster CI - Recommended)

```yaml
name: Python Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: ["3.8", "3.9", "3.10", "3.11", "3.12"]
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install uv
      uses: astral-sh/setup-uv@v4
      with:
        enable-cache: true
    
    - name: Set up Python ${{ matrix.python-version }}
      uses: actions/setup-python@v5
      with:
        python-version: ${{ matrix.python-version }}
    
    - name: Set up Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Install development dependencies
      working-directory: crates/pattern-core
      run: |
        uv venv
        uv pip install -e ".[dev]"
    
    - name: Build Python extension
      working-directory: crates/pattern-core
      run: |
        source .venv/bin/activate
        maturin develop --features python
    
    - name: Run tests
      working-directory: crates/pattern-core
      run: |
        source .venv/bin/activate
        pytest tests/python/ -v --cov
    
    - name: Upload coverage
      uses: codecov/codecov-action@v4
```

**Benefits of uv in CI:**
- ⚡ **Faster builds** - dependency resolution is 10-100x faster
- 💾 **Better caching** - uv has efficient caching built-in
- 🔒 **Reliable** - consistent resolution across environments

#### Option 2: With pip (Traditional)

```yaml
name: Python Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: ["3.8", "3.9", "3.10", "3.11", "3.12"]
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Set up Python ${{ matrix.python-version }}
      uses: actions/setup-python@v5
      with:
        python-version: ${{ matrix.python-version }}
    
    - name: Set up Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Install development dependencies
      working-directory: crates/pattern-core
      run: |
        pip install -e ".[dev]"
    
    - name: Build Python extension
      working-directory: crates/pattern-core
      run: |
        maturin develop --features python
    
    - name: Run tests
      run: |
        pytest crates/pattern-core/tests/python/ -v --cov
    
    - name: Upload coverage
      uses: codecov/codecov-action@v4
```

## Virtual Environment Best Practices

### Why Use Virtual Environments?

1. **Isolation**: Each project has its own dependencies
2. **Reproducibility**: `pyproject.toml` defines exact requirements
3. **Safety**: Won't break system Python or other projects
4. **Testing**: Can test against multiple Python versions

### Managing Virtual Environments

#### With uv (Recommended)

```bash
# Create (uv auto-detects .venv)
uv venv

# Activate (optional - uv tools work without activation)
source .venv/bin/activate  # macOS/Linux
.venv\Scripts\activate     # Windows

# Install dependencies (works with or without activation)
uv pip install -e ".[dev]"

# Deactivate (if activated)
deactivate

# Delete (when done)
rm -rf .venv
```

#### With pip (Traditional)

```bash
# Create
python -m venv .venv

# Activate (required for pip)
source .venv/bin/activate  # macOS/Linux
.venv\Scripts\activate     # Windows

# Install dependencies (requires activation)
pip install -e ".[dev]"

# Deactivate
deactivate

# Delete (when done)
rm -rf .venv
```

### .gitignore

The `.venv` directory should be ignored:

```gitignore
# Virtual environments
.venv/
venv/
env/
```

## FAQ

### Q: Do I need maturin installed to use the wheel?

**No.** End users installing via `pip install pattern-core` don't need maturin. It's only needed for:
- Building from source
- Development

### Q: Can I use poetry/pipenv/other tools?

**Yes.** They all respect `pyproject.toml`:

```bash
# uv (Recommended)
uv venv
uv pip install -e ".[dev]"
maturin develop --features python

# Poetry
poetry install --with dev
poetry run maturin develop

# Pipenv
pipenv install --dev
pipenv run maturin develop

# PDM
pdm install -d
pdm run maturin develop
```

### Q: What about conda?

**Yes.** Conda environments work with uv or pip:

```bash
# Create conda environment
conda create -n pattern-core python=3.11
conda activate pattern-core

# Install with uv (faster)
uv pip install -e ".[dev]"
maturin develop --features python

# Or install with pip (traditional)
pip install -e ".[dev]"
maturin develop --features python
```

### Q: Why uv instead of pip?

**Performance:** uv is written in Rust and is 10-100x faster than pip. For context:

| Tool | Time to install deps | Resolver speed |
|------|---------------------|----------------|
| **uv** | ~0.5s | Very fast ⚡ |
| pip | ~5-50s | Slow 🐌 |
| poetry | ~3-30s | Medium 🚶 |

**Additional benefits:**
- Better error messages
- More reliable dependency resolution
- Built-in venv management
- No external dependencies
- Made by the Ruff team (Astral) - Rust developers for Python tools

### Q: Why not just use `cargo test`?

Cargo tests the **Rust** code. Python tests verify:
- PyO3 bindings work correctly
- Python API is Pythonic
- Type conversions are correct
- Error handling works from Python
- Python-specific features (callbacks, etc.)

Both test suites are important!

## Summary

**Best Practice: uv + Project Dependencies + Virtual Environment**

```bash
# One-time setup (recommended)
cd crates/pattern-core
uv venv
uv pip install -e ".[dev]"

# Daily development
maturin develop --features python  # After Rust changes
pytest tests/python/ -v             # Run tests

# Update dependencies (when pyproject.toml changes)
uv pip install -e ".[dev]"
```

**Alternative with pip:**

```bash
# One-time setup (traditional)
cd crates/pattern-core
python -m venv .venv
source .venv/bin/activate
pip install -e ".[dev]"

# Daily development (must activate venv first)
source .venv/bin/activate           # Required
maturin develop --features python
pytest tests/python/ -v
```

**Why this matters:**
- ⚡ **uv is 10-100x faster** - especially important in CI/CD
- 🦀 **Rust-native tooling** - perfect for Rust+Python projects
- 📦 **Standard pyproject.toml** - no lock files or extra config needed
- 🔒 **Reproducible** - same dependencies everywhere
- 🧪 **Isolated** - won't affect other projects or system Python

This approach ensures reproducible, isolated, and CI-compatible development with maximum performance.
