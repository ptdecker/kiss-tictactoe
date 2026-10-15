# kiss-tictactoe

A Rust tic-tac-toe demonstration project showcasing clean library/UI separation.

The game engine lives in a platform-agnostic library crate (`kiss-tictactoe-engine`) with a clean public API. A [Dioxus 0.7](https://dioxuslabs.com/) desktop app (`kiss-tictactoe-ui`) provides the interface. Always user vs. engine — no multiplayer, no networking.

**Core value:** A fully playable, good-looking tic-tac-toe game where the engine trash talks you — demonstrating idiomatic Rust architecture.

## Features

- A perfect minimax engine that never loses
- Context-aware trash talk in a fading speech bubble
- Session scoreboard (Wins, Draws, Losses)
- New Game and Reset Scores controls
- An engine crate with zero dependencies

## Architecture

```
kiss-tictactoe/
├── kiss-tictactoe-engine/   # Platform-agnostic game logic (no UI deps)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── game.rs
│   │   ├── types.rs
│   │   ├── ai.rs
│   │   ├── events.rs
│   │   ├── commentary.rs
│   │   └── detection.rs
│   └── tests/gameplay.rs
├── kiss-tictactoe-ui/       # Dioxus 0.7 desktop app
│   ├── src/
│   │   ├── main.rs
│   │   ├── state.rs
│   │   └── components/
│   ├── assets/main.css
│   └── Dioxus.toml
├── docs/
│   ├── user-guide/          # mdBook for players
│   └── engine/              # mdBook for engine users
└── scripts/check-docs.sh
```

The engine crate has zero platform dependencies — it compiles unchanged for desktop, web, and mobile targets. The UI crate depends on the engine; the engine never depends on the UI.

## Platform

v1 targets macOS only. Other platforms are planned.

## Prerequisites

- [Rust](https://rustup.rs/) (stable, edition 2024)
- [dioxus-cli](https://dioxuslabs.com/learn/0.7/getting_started/) 0.7.10
- [mdbook](https://rust-lang.github.io/mdBook/) 0.5.4 (to build the docs)

```sh
cargo install dioxus-cli --version 0.7.10 --locked
cargo install mdbook --version 0.5.4 --locked
```

## Building

```sh
# Build all crates
cargo build --workspace

# Verify engine purity (must return empty output)
cargo tree -p kiss-tictactoe-engine | grep dioxus
```

## Running

```sh
cd kiss-tictactoe-ui
dx serve
```

If `dx` fails with `sccache: Compiler not supported`, a global `build.rustc-wrapper = "sccache"` in `~/.cargo/config.toml` is combining with dx's own rustc wrapper. Either add this to `~/.zshrc` and open a new terminal:

```sh
dx() { RUSTC_WRAPPER= command dx "$@"; }
```

or prefix a single run:

```sh
RUSTC_WRAPPER="" dx serve
```

## Testing

```sh
cargo test --workspace
```

## Documentation

- [User guide](docs/user-guide/src/introduction.md)
- [Engine manual](docs/engine/src/introduction.md)

```sh
mdbook build docs/user-guide
mdbook build docs/engine
cargo doc --workspace --no-deps --open
scripts/check-docs.sh
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Please read the [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

## License

MIT — see [LICENSE](LICENSE).
