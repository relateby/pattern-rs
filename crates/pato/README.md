# relateby-pato

`relateby-pato` provides the `pato` CLI for working with gram files and the bundled `pato` skill.

## Install

Install from crates.io:

```bash
cargo install relateby-pato
```

The binary name is `pato`.

If you do not want to use Cargo, download the `pato` GitHub Release artifact for
your platform from <https://github.com/relateby/pattern-rs/releases/latest> and
place the binary on `PATH`.

## Use

After installation, make sure `pato` is on `PATH`, then run:

```bash
pato skill
```

That installs or updates the local skill from the bundled canonical package.

## Packaging notes

- The canonical skill source lives at `.agents/skills/pato/`
- The packaged mirror lives under `crates/pato/skill-package/pato/`
- The crate bundle includes the packaged mirror so published releases stay self-contained
