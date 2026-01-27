# Python Tests for Pattern-Core

This directory contains Python tests for the pattern-core Python bindings.

## Prerequisites

### Install uv (Recommended)

```bash
# macOS/Linux
curl -LsSf https://astral.sh/uv/install.sh | sh

# Or via pip
pip install uv

# Or via cargo (you already have Rust!)
cargo install --git https://github.com/astral-sh/uv uv
```

### Install Development Dependencies

**Option 1: With uv (Recommended - 10-100x faster)**
```bash
# Create venv and install all dev dependencies
cd crates/pattern-core
uv venv
uv pip install -e ".[dev]"
```

This installs:
- `maturin` - For building the Rust extension
- `pytest` - For running tests
- `pytest-cov` - For coverage reports

**Option 2: With pip (Traditional)**
```bash
# Create and activate virtual environment
python -m venv .venv
source .venv/bin/activate  # macOS/Linux
# .venv\Scripts\activate    # Windows

# Install all dev dependencies
pip install -e ".[dev]"
```

**Option 3: Install individually**
```bash
pip install maturin pytest pytest-cov
```

## Building the Extension

Build and install the pattern-core Python extension in development mode:

```bash
cd /Users/akollegger/Developer/gram-data/gram-rs/crates/pattern-core
maturin develop --features python
```

This will compile the Rust code and install the `pattern_core` module in your current Python environment.

## Running Tests

Once the extension is built, run all tests:

```bash
pytest tests/python/
```

Run specific test files:

```bash
pytest tests/python/test_pattern.py
pytest tests/python/test_subject.py
pytest tests/python/test_operations.py
pytest tests/python/test_subject_combination.py
pytest tests/python/test_validation.py
```

Run with verbose output:

```bash
pytest tests/python/ -v
```

## Test Coverage

### test_pattern.py
- Pattern construction (`point`, `pattern`, `from_list`)
- Pattern structure (value, elements)

### test_subject.py
- Subject construction
- Label operations (add, remove, has)
- Property operations (set, get, remove)

### test_operations.py
- **Inspection**: `length`, `size`, `depth`, `is_atomic`, `values`
- **Queries**: `any_value`, `all_values`, `filter`, `find_first`, `matches`, `contains`
- **Transformations**: `map`, `fold`
- **Combination**: `combine`
- **Comonad**: `extract`, `extend`, `depth_at`, `size_at`, `indices_at`
- **PatternSubject operations**

### test_subject_combination.py
- Subject combination strategies: `merge`, `first`, `last`, `empty`
- Custom combination functions
- Element concatenation
- Associativity
- Error handling

### test_validation.py
- ValidationRules creation
- Pattern validation (max_depth, max_elements)
- Structure analysis
- PatternSubject validation

## Troubleshooting

### Module not found error

If you get `ModuleNotFoundError: No module named 'pattern_core'`, make sure you've run:

```bash
maturin develop --features python
```

### Build errors

Make sure you have:
- Rust toolchain installed
- Python 3.8+ installed
- maturin installed (`pip install maturin`)

### Test failures

Check that the Rust code compiles:

```bash
cargo check --package pattern-core --features python
```

Run Rust tests:

```bash
cargo test --package pattern-core --lib
```

## CI Integration

**Option 1: With uv (Recommended - Faster CI)**

```yaml
- name: Install uv
  uses: astral-sh/setup-uv@v4
  with:
    enable-cache: true

- name: Set up Python
  uses: actions/setup-python@v5
  with:
    python-version: '3.11'

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

- name: Run Python tests
  working-directory: crates/pattern-core
  run: |
    source .venv/bin/activate
    pytest tests/python/ -v --cov
```

**Option 2: With pip (Traditional)**

```yaml
- name: Set up Python
  uses: actions/setup-python@v5
  with:
    python-version: '3.11'

- name: Install development dependencies
  working-directory: crates/pattern-core
  run: pip install -e ".[dev]"

- name: Build Python extension
  working-directory: crates/pattern-core
  run: maturin develop --features python

- name: Run Python tests
  run: pytest crates/pattern-core/tests/python/ -v --cov
```
