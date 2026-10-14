# Contributing to kiss-tictactoe

Thank you for your interest in contributing! This is a demonstration project showcasing idiomatic Rust architecture, so contributions that stay within the Keep It Simple Stupid principle are most welcome.

## How to Report Bugs

Please open a [GitHub Issue](https://github.com/ptdecker/kiss-tictactoe/issues) using the **Bug Report** template. Include:

- **Operating system** and version
- **Rust version** (`rustc --version`)
- **dioxus-cli version** (`dx --version`)
- **Steps to reproduce** the problem
- **Expected behavior** vs. **actual behavior**

The more detail you provide, the faster we can diagnose and fix the issue.

## How to Submit a Pull Request

1. Fork the repository and create a branch from the relevant phase branch (e.g., `gsd/phase-02-*`) or `main`.
2. Make your changes, following the Code Style guidelines below.
3. Open a pull request against the appropriate branch and describe what your change does and why.
4. PRs that add features should reference a GitHub Issue.

## Code Style

Before submitting, please run:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
```

Both commands must produce zero warnings and zero errors. PRs with clippy warnings will be asked to fix them before merge.

## Scope Note

kiss-tictactoe is a focused demonstration project. PRs are most likely to be accepted when they:

- Fix a genuine bug
- Improve documentation or developer experience
- Stay within the existing engine/UI separation architecture

Large feature additions or architectural changes are better discussed in a GitHub Issue first.
