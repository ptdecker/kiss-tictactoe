# kiss-tictactoe

A Rust tic-tac-toe demonstration project showcasing clean library/UI separation.

The game engine lives in a platform-agnostic library crate (`kiss-tictactoe-engine`) with a clean public API. A [Dioxus 0.7](https://dioxuslabs.com/) desktop app (`kiss-tictactoe-ui`) provides the interface. Always user vs. engine — no multiplayer, no networking.

**Core value:** A fully playable, good-looking tic-tac-toe game where the engine trash talks you and can teach you how to play — demonstrating idiomatic Rust architecture.

## Architecture

```
kiss-tictactoe/
├── kiss-tictactoe-engine/   # Platform-agnostic game logic (no UI deps)
│   └── src/lib.rs
└── kiss-tictactoe-ui/       # Dioxus 0.7 desktop app
    ├── src/main.rs
    ├── assets/main.css
    └── Dioxus.toml
```

The engine crate has zero platform dependencies — it compiles unchanged for desktop, web, and mobile targets. The UI crate depends on the engine; the engine never depends on the UI.

## Prerequisites

- [Rust](https://rustup.rs/) (stable, edition 2024)
- [dioxus-cli](https://dioxuslabs.com/learn/0.6/getting_started/) 0.7.10

```sh
cargo install dioxus-cli --version 0.7.10 --locked
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

## Testing

```sh
cargo test --workspace
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Please read the [Code of Conduct](CODE_OF_CONDUCT.md) before contributing.

## License

MIT — see [LICENSE](LICENSE).
