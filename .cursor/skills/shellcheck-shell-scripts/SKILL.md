---
name: shellcheck-shell-scripts
description: Runs ShellCheck on shell scripts and fixes findings before completing shell-script changes. Use when creating or editing .sh files, release scripts, CI helpers, or any shell-based automation.
---

# ShellCheck for Shell Scripts

## Instructions

When a task creates or changes a shell script:

1. Identify the modified `.sh` files.
2. Run `shellcheck` on each changed script.
3. Fix any warnings or errors before finishing the task.
4. Re-run `shellcheck` until the scripts are clean.
5. If `shellcheck` is unavailable, report that clearly and do not claim validation passed.

## Scope

Use this skill for:

- new shell scripts
- edited shell scripts
- CI helper scripts
- release scripts
- repository automation in `scripts/*.sh`

## Validation Standard

- Prefer zero ShellCheck warnings for changed scripts.
- Treat new warnings as regressions.
- Preserve existing behavior while fixing style and safety issues.

## Practical Notes

- Keep fixes minimal and localized.
- Preserve quoting, exit handling, and portability.
- If a script is intentionally exempt from a warning, document why in the script or task output.
