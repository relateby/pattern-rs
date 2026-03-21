# Quickstart: `pato skill`

## Goal

Validate the `pato skill` workflow end-to-end for the bundled canonical skill package.

## Preconditions

- The canonical skill package exists at `.agents/skills/pato/`
- The `pato` crate builds successfully
- You are on branch `043-pato-skill`

## Validate the canonical package

1. Confirm the canonical package root exists:

   ```bash
   ls ".agents/skills/pato"
   ```

2. Confirm the required entry file exists:

   ```bash
   ls ".agents/skills/pato/SKILL.md"
   ```

## Run the default project install

When running from the repository root, `.agents/skills/pato/` already exists (the
canonical source). The first `pato skill` invocation will succeed, but the project
install target is the same directory, so you may see an "already exists" error if you
run the command a second time. Use a separate temporary project directory or pass
`--force` to replace.

1. Run from the repository root (installs to `.agents/skills/pato/`; use `--force` to
   overwrite if the directory already exists):

   ```bash
   cargo run -p relateby-pato -- skill --force
   ```

   Or run from a clean project directory:

   ```bash
   mkdir /tmp/my-project && cargo run -p relateby-pato -- skill
   ```

2. Verify the installed project path exists and contains `SKILL.md`:

   ```bash
   ls ".agents/skills/pato"
   ```

3. Confirm the command reports the resolved install destination.

## Run user-scope installs

1. Interoperable user install:

   ```bash
   cargo run -p relateby-pato -- skill --scope user
   ```

2. Client-native user install:

   ```bash
   cargo run -p relateby-pato -- skill --scope user --target cursor
   ```

3. Verify each command writes only to its selected destination.

## Validate overwrite protection

1. Run an install once.
2. Run the same install again without replacement enabled and confirm the command
   fails without modifying the existing install.
3. Re-run with explicit replacement enabled:

   ```bash
   cargo run -p relateby-pato -- skill --force
   ```

4. Confirm the install succeeds and the destination stays valid.

## Validation commands

Run the feature's core checks:

```bash
cargo test -p relateby-pato skill_tests
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
```

If you need to validate the packaged artifact path, run a packaging dry run for
`relateby-pato` and confirm the bundle contains the canonical skill files.
