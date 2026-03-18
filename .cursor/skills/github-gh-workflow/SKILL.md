---
name: github-gh-workflow
description: Uses the GitHub CLI for pull requests, reviews, comments, issues, releases, and GitHub Actions runs. Use when GitHub, GitHub Actions, pull requests, PR comments, issues, checks, workflow runs, or GitHub URLs are mentioned.
---

# GitHub GH Workflow

## Quick Start

Prefer the `gh` CLI for GitHub work instead of browser-based exploration or ad hoc API calls.

Use this skill whenever the user mentions:
- GitHub
- pull requests or PRs
- PR reviews or comments
- issues
- checks, statuses, or GitHub Actions
- a GitHub URL

## Core Rules

- Use `gh` as the default interface for GitHub tasks.
- If the user gives a GitHub URL, use `gh` to inspect the related PR, issue, commit, run, or repo.
- Read current state before acting: inspect the PR, issue, checks, or run before proposing changes.
- Prefer read-only inspection first, then make changes only when the user asks.
- When posting comments or creating PRs/issues, avoid duplicating existing discussion.
- Return the relevant URL or identifier after completing a GitHub action.

## Pull Request Workflow

When working with a pull request:

1. Identify the PR and repo context.
2. Read the full PR state before acting:
   - summary and metadata
   - changed files
   - commits
   - checks
   - review comments and discussion
3. If the user asks for review, focus first on bugs, regressions, risks, and missing tests.
4. If the user asks to create or update a PR, make sure the branch is pushed before creating it.
5. When creating PR bodies, keep them concise and structured:
   - summary
   - test plan
   - risks or follow-ups when relevant

Useful commands:

```bash
gh pr view <number> --comments
gh pr diff <number>
gh pr checks <number>
gh pr create --title "..." --body "..."
gh pr comment <number> --body "..."
gh pr review <number> --comment --body "..."
```

## Issue Workflow

When working with issues:

1. Read the issue before replying or making changes.
2. Check labels, assignees, status, linked PRs, and related discussion.
3. Summarize the current state before proposing next steps.
4. If creating a new issue, write a clear title and a body with:
   - problem
   - context
   - reproduction or evidence
   - expected outcome

Useful commands:

```bash
gh issue view <number> --comments
gh issue list
gh issue create --title "..." --body "..."
gh issue comment <number> --body "..."
```

## Comments And Reviews

- Read existing comments first so new comments add value.
- Keep comments specific, actionable, and grounded in the code or run output.
- Prefer one clear comment over many fragmented comments.
- Use multiline bodies when needed so formatting stays intact.

For longer comments, prefer a heredoc:

```bash
gh pr comment <number> --body "$(cat <<'EOF'
Concise comment here.

- Key point
- Suggested next step
EOF
)"
```

## GitHub Actions Workflow

When the user mentions checks, CI, workflows, or failed runs:

1. Inspect recent runs for the branch, PR, or workflow.
2. Read failing job details before diagnosing.
3. Prefer failed logs first to reduce noise.
4. Summarize the failing job, likely cause, and next step.

Useful commands:

```bash
gh run list
gh run view <run-id>
gh run view <run-id> --log-failed
gh workflow list
```

## Best Practices

- Prefer `gh pr view`, `gh issue view`, and `gh run view` before taking action.
- Use `gh api` when built-in subcommands do not expose the needed data cleanly.
- Keep PR and issue summaries focused on why, status, and next steps.
- If authentication or repo access fails, surface that clearly and stop instead of guessing.
- Do not merge, close, reopen, comment, or edit GitHub resources unless the user asked for that action.

## Output Style

When reporting back:

- Start with the result or key finding.
- Include the relevant PR, issue, or run URL when available.
- Summarize the important state, not raw command output.
- For reviews, list findings first and keep the summary brief.
