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

1. Build and run the command:

   ```bash
   cargo run -p relateby-pato -- skill
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
3. Re-run with explicit replacement enabled and confirm the install succeeds.

## Validation commands

Run the feature's core checks:

```bash
cargo test -p relateby-pato skill_tests
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
```

If asset bundling logic depends on packaging configuration, also run the relevant
packaging verification step for `relateby-pato`.
