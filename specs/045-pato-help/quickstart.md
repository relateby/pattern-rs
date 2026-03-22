# Quickstart: pato help implementation

**Feature**: 045-pato-help

This guide walks a developer through implementing `pato help <topic>` from scratch.

---

## 1. Add topic markdown files

Create the `reference/` directory under the skill package and write the initial topic files:

```
crates/pato/skill-package/pato/reference/
├── gram-notation.md
└── stdout-stderr-contracts.md
```

Each file must open with a `# Topic Name` heading and contain: definition, semantics, examples, and optionally related topics. Keep each file under ~150 lines.

---

## 2. Create `topic_catalog.rs`

Add `crates/pato/src/topic_catalog.rs`:

```rust
pub struct TopicEntry {
    pub name: &'static str,
    pub content: &'static str,
}

pub static TOPICS: &[TopicEntry] = &[
    TopicEntry {
        name: "gram-notation",
        content: include_str!("../skill-package/pato/reference/gram-notation.md"),
    },
    TopicEntry {
        name: "stdout-stderr-contracts",
        content: include_str!("../skill-package/pato/reference/stdout-stderr-contracts.md"),
    },
];

pub fn find_topic(name: &str) -> Option<&'static TopicEntry> {
    TOPICS.iter().find(|e| e.name == name)
}

pub fn topic_names() -> impl Iterator<Item = &'static str> {
    TOPICS.iter().map(|e| e.name)
}
```

Register in `lib.rs`:

```rust
pub mod topic_catalog;
```

---

## 3. Add `HelpArgs` to `cli.rs`

In `cli.rs`, add a `Help` variant to the `Commands` enum and define `HelpArgs`:

```rust
/// Show help for a specific topic
Help(HelpArgs),
```

```rust
#[derive(Args, Debug)]
pub struct HelpArgs {
    /// Topic name (e.g., gram-notation). Omit to list all topics.
    pub topic: Option<String>,
}
```

---

## 4. Create `commands/help.rs`

```rust
use std::process::ExitCode;
use crate::cli::HelpArgs;
use crate::topic_catalog::{find_topic, topic_names};

pub fn run(args: &HelpArgs) -> ExitCode {
    match &args.topic {
        Some(name) => match find_topic(name) {
            Some(entry) => {
                print!("{}", entry.content);
                ExitCode::SUCCESS
            }
            None => {
                eprintln!("error: unknown topic '{}'", name);
                print_topic_list();
                ExitCode::FAILURE
            }
        },
        None => {
            eprintln!("usage: pato help <topic>");
            print_topic_list();
            ExitCode::FAILURE
        }
    }
}

fn print_topic_list() {
    eprintln!("\nAvailable topics:");
    for name in topic_names() {
        eprintln!("  {}", name);
    }
}
```

Register in `commands/mod.rs`:

```rust
pub mod help;
```

---

## 5. Wire dispatch in `main.rs`

Add the `Help` arm to the match statement in `main.rs`:

```rust
Commands::Help(args) => commands::help::run(&args),
```

---

## 6. Verify skill install picks up `reference/`

The existing `skill_install/package.rs` copies the full `skill-package/pato/` tree. Since `reference/` is a subdirectory, `pato skill` will install it automatically. No changes needed to the install code.

Verify with:

```bash
cargo run -p relateby-pato -- skill --scope project
ls .agents/skills/pato/reference/
```

---

## 7. Test

Run the existing test suite:

```bash
cargo test -p relateby-pato
```

Add tests in `tests/help_tests.rs` covering:
- Known topic returns exit 0 and prints content
- Unknown topic returns exit 1 and prints topic list
- No-topic invocation returns exit 1 and prints topic list
- All catalog entries resolve to non-empty content
