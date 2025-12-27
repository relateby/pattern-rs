# Snapshot Testing

This directory contains snapshot tests using the `insta` library.

## Usage

Snapshots are stored in `tests/__snapshots__/` and are automatically created on first run.

## Reviewing Snapshots

After running tests, review snapshot changes:

```bash
cargo insta review
```

Accept intentional changes:

```bash
cargo insta accept
```

## Workflow

1. Write a snapshot test
2. Run tests - snapshots are created automatically
3. If output changes, review with `cargo insta review`
4. Accept intentional changes or investigate regressions

