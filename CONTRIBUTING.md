# Contributing

Thank you for your interest in contributing to Distributed Forensic Recovery.

## Before Contributing

Please read:

- `README.md`
- `ROADMAP.md`
- `ARCHITECTURE.md`
- `SECURITY.md`

This project handles forensic evidence. Correctness, evidence integrity, reproducibility, and clear documentation are more important than speed of implementation.

## Development Principles

Contributors must:

- Never modify original evidence.
- Keep evidence access read-only.
- Validate assumptions about offsets, lengths, and filesystem structures.
- Record important decisions and limitations.
- Add tests for new behavior.
- Avoid hiding errors or silently treating uncertain results as verified.
- Keep changes focused and reviewable.
- Update documentation when behavior or architecture changes.

## Branching

Create a separate branch for each change.

Example branch names:

    feature/<short-description>
    fix/<short-description>
    docs/<short-description>
    test/<short-description>
    chore/<short-description>

Before creating a new branch, synchronize the local `main` branch with `upstream/main` when an upstream repository is configured.

## Development Workflow

1. Create or select the appropriate issue.
2. Synchronize with the latest project changes.
3. Create a focused branch.
4. Implement the change.
5. Add or update tests.
6. Run formatting and validation checks.
7. Review the complete diff.
8. Confirm that no evidence files or generated artifacts are included.
9. Commit with a clear message.
10. Push the branch and open a pull request.

## Testing Requirements

Changes should include appropriate tests for:

- Normal behavior
- Invalid input
- Truncated evidence
- Boundary offsets
- Incorrect lengths
- Unsupported structures
- Error handling
- Deterministic output

For recovery-related changes, tests should use synthetic or dedicated test evidence rather than real investigative evidence.

## Pull Requests

Pull requests should explain:

- What changed
- Why the change was needed
- How it was implemented
- What tests were run
- Any known limitations
- Any forensic-integrity considerations

Do not claim that a file was successfully recovered unless the implementation and validation support that claim.

## Commit Messages

Use concise, descriptive commit messages.

Examples:

    docs: add initial architecture documentation
    feat: add bounded evidence reader
    test: cover truncated evidence reads
    fix: reject invalid recovery ranges

## Review Standards

Every change should be reviewed for:

- Logical correctness
- Safety
- Error handling
- Boundary conditions
- Cross-platform behavior
- Test coverage
- Documentation accuracy
- Unintended changes

A passing test alone is not sufficient if the implementation is logically incorrect.

## Reporting Problems

Please open an issue for bugs, design questions, documentation problems, or proposed improvements. Include enough information to reproduce the problem without sharing sensitive evidence.