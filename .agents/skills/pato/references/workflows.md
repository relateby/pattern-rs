# pato Workflows

## Prerequisite

- `pato` must already be available on `PATH`

## Inspect a gram file

1. Run `pato parse file.gram`.
2. If the output needs to be machine-consumed, keep `--output-format gram` or `json`.
3. Re-run after changes to confirm the result is stable.

## Fix diagnostics

1. Run `pato lint file.gram`.
2. If the output reports auto-fixable issues, run `pato lint --fix file.gram`.
3. Run `pato lint file.gram` again to verify the file is clean.

## Understand a P-code

1. Run `pato rule P00N`.
2. Read the rule name, grade, and trigger example.
3. Use the trigger example to confirm the rule is understood in context.

## Install the skill

1. Run `pato skill` after `pato` is installed.
2. Use `--scope user` for a user-level install.
3. Use `--target cursor` only for user-scope client-native installs.
