# Contributing to kiss-tictactoe

Thank you for your interest in contributing!

This is a demonstration project showcasing idiomatic Rust architecture, so contributions that stay
within the Keep It Simple Stupid principle are most welcome.

## How to Report Bugs

Please open a [GitHub Issue](https://github.com/ptdecker/kiss-tictactoe/issues) using the
**Bug Report** template. Include:

- **Operating system** and version
- **Rust version** (`rustc --version`)
- **dioxus-cli version** (`dx --version`)
- **Steps to reproduce** the problem
- **Expected behavior** vs. **actual behavior**

The more detail you provide, the faster we can diagnose and fix the issue.

## How to Submit a Pull Request

1. Fork the repository and create a branch from the relevant phase branch (e.g., `gsd/phase-02-*`)
   or `main`.
2. Make your changes, following the Code Style guidelines below.
3. Open a pull request against the appropriate branch and describe what your change does and why.
4. PRs that add features should reference a GitHub Issue.

## Code Style

Before submitting, please run:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
```

Both commands must produce zero warnings and zero errors. PRs with clippy warnings will be asked to
fix them before merge.

## Documentation

The two mdBooks live in `docs/user-guide` and `docs/engine`. They require mdbook 0.5.4:

```sh
cargo install mdbook --version 0.5.4 --locked
```

Markdown is linted with [rumdl](https://rumdl.dev) (install with
`cargo install rumdl --version 0.2.79 --locked`). Lines wrap at 100 columns; run `rumdl fmt .` to
reflow and fix. Before submitting documentation changes, run `scripts/check-docs.sh`; it must print
`docs OK`. Books are built locally; there is no publishing step.

## Scope Note

kiss-tictactoe is a focused demonstration project. PRs are most likely to be accepted when they:

- Fix a genuine bug
- Improve documentation or developer experience
- Stay within the existing engine/UI separation architecture

Large feature additions or architectural changes are better discussed in a GitHub Issue first.

## Dependency advisories

Two Dependabot alerts were dismissed because the fix cannot be applied within the pinned Dioxus
0.7.10. Both come from Dioxus's `wry`/`tao` stack and need Dioxus 0.8, which is still a pre-release.
Revisit them when a stable Dioxus release updates those crates.

- `glib` 0.18 (medium): only compiled on Linux through the gtk/webkit2gtk stack. It is not built on
  macOS, the v1 target, and the app does not use the affected iterator.
- `rand` 0.7.3 (low): a build-time dependency (`phf_generator` via `kuchikiki` via `wry`). It is not
  linked into the app, and the advisory needs a custom logger that calls `rand::rng()`.
