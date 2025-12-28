# GitHub Actions Workflow Testing

This directory contains GitHub Actions workflows for CI/CD.

## Testing Workflows Locally

### Using `act` (Recommended)

`act` is a tool that runs GitHub Actions workflows locally using Docker.

#### Installation

**macOS (Homebrew)**:
```bash
brew install act
```

**Linux**:
```bash
curl -s https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash
```

**Windows (Chocolatey)**:
```bash
choco install act-cli
```

#### Usage

1. **List all workflows and jobs**:
   ```bash
   act -l
   ```

2. **Run a specific job**:
   ```bash
   act -j build
   act -j test
   act -j lint
   act -j format
   ```

3. **Run all jobs for a push event**:
   ```bash
   act push
   ```

4. **Run with verbose output**:
   ```bash
   act -v -j build
   ```

#### Prerequisites

- Docker must be installed and running
- The workflow will use Docker containers to simulate GitHub's runner environment

### Manual Validation

You can validate the YAML syntax without running the workflow:

```bash
# Using Ruby (if available)
ruby -ryaml -e "YAML.load_file('.github/workflows/ci.yml'); puts 'YAML syntax is valid'"

# Using Python (if pyyaml is installed)
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"
```

### Workflow Structure

The CI workflow includes four jobs:

1. **build**: Compiles the workspace for multiple targets (native and WASM)
2. **test**: Runs all workspace tests
3. **lint**: Runs clippy with strict warnings
4. **format**: Verifies code formatting

All jobs use `actions-rs/toolchain@v1` with explicit `toolchain: stable` input.

