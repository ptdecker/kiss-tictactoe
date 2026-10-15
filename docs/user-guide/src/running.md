# Running the game

This chapter covers how to build and launch the game on macOS, the only platform supported in this release.

## Prerequisites

- Rust stable, installed with [rustup](https://rustup.rs/), with support for the 2024 edition.
- dioxus-cli 0.7.10:

  ```sh
  cargo install dioxus-cli --version 0.7.10 --locked
  ```

- The Xcode command line tools, if the build asks for them (`xcode-select --install`).

## Build

```sh
git clone https://github.com/ptdecker/kiss-tictactoe
cd kiss-tictactoe
cargo build --workspace
```

## Run

```sh
cd kiss-tictactoe-ui
dx serve
```

A window titled kiss-tictactoe opens with the game. To stop, press Ctrl-C in the terminal or close the window.

If `dx` fails with `sccache: Compiler not supported`, the fix is in [Troubleshooting](troubleshooting.md).

## Other platforms

Instructions for Windows, Linux, iOS, Android and web are planned for a later release.
