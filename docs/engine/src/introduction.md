# Introduction

The kiss-tictactoe engine is a pure Rust library with zero dependencies. It is used by the Dioxus desktop app, but it knows nothing about any user interface, so any front end can drive it.

This manual is a narrative guide to the engine. Per-item reference lives in rustdoc.

API reference: run `cargo doc --workspace --no-deps --open`, or open [the engine API docs](../../../target/doc/kiss_tictactoe_engine/index.html). The link works after `cargo doc` has run and when the built book is opened from disk (not under `mdbook serve`).

Here is a first sample. It is compiled and run against the real engine whenever the docs are checked:

```rust
# extern crate kiss_tictactoe_engine;
use kiss_tictactoe_engine::{Game, MoveResult, Position, TalkEvent, Turn};

let mut game = Game::new();
let turn = game.play(Position { row: 1, col: 1 }).unwrap();
assert_eq!(
    turn,
    Turn { result: MoveResult::Continuing, event: Some(TalkEvent::GoodMove) }
);
```

## About this project

This manual, the engine and the rest of the project were written with Claude, an AI model made by Anthropic (via Claude Code). Claude wrote much of the code, tests and documentation under the direction and review of the project's author.
