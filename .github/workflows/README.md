# GitHub Actions Workflow Testing

This directory contains GitHub Actions workflows for CI/CD.

## Testing Workflows Locally

### Quick Local CI Script (Easiest)

The simplest way to run all CI checks locally is using the provided script:

```bash
./scripts/ci-local.sh
```

This script runs all the same checks that GitHub Actions runs:
- Format check
- Clippy lint
- Native build
- WASM build (if target is installed)
- Tests

**No Docker required!** This is the fastest way to verify your changes before pushing.

### Using `act` (Full Workflow Simulation)

`act` is a tool that runs GitHub Actions workflows locally. It can use Docker (default) or run directly on your host machine.

#### Prerequisites

- `act` installed (macOS: `brew install act`)
- **For Docker mode**: Docker must be installed and running
- **For self-hosted mode**: No Docker required, but tools (Rust, cargo, etc.) must be installed on your system

#### Running with Docker (Default)

This provides the most accurate simulation of GitHub Actions' Ubuntu environment:

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

5. **Run a specific job with a specific target** (for the build matrix):
   ```bash
   # Run build job with wasm32 target
   act -j build --matrix target:wasm32-unknown-unknown
   ```

#### Running Without Docker (Self-Hosted)

You can run workflows directly on your host machine without Docker using the `-P` flag:

**For Ubuntu workflows** (our CI uses `ubuntu-latest`):
```bash
# Run all jobs without Docker
act push -P ubuntu-latest=-self-hosted

# Run a specific job without Docker
act -j build -P ubuntu-latest=-self-hosted
act -j test -P ubuntu-latest=-self-hosted
act -j lint -P ubuntu-latest=-self-hosted
act -j format -P ubuntu-latest=-self-hosted
```

**For macOS workflows**:
```bash
act -P macos-latest=-self-hosted
```

**For Windows workflows**:
```bash
act -P windows-latest=-self-hosted
```

**Note**: When running self-hosted, the workflow executes directly on your system. This means:
- No Docker isolation (uses your system state)
- Faster execution (no container overhead)
- May behave differently than GitHub's Ubuntu environment
- All required tools (Rust, cargo, etc.) must be installed on your host

#### Tips

- **Docker mode**: First run will download Docker images (can take a few minutes)
- **Self-hosted mode**: Faster startup, but less isolation
- Use `act -j <job-name>` to test individual jobs quickly
- Use `act push` to simulate a full push event (runs all jobs)
- If you get permission errors, `act` needs to create cache directories in `~/.cache/act`
- For this Rust project, self-hosted mode works well since the workflow only runs Rust commands

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

