# Testing

The project tests the engine in layers and checks the documentation too.

## The layers

- **Unit tests and rstest tables.** Win detection, event selection and the commentary banks are
  covered with `rstest` parameterized cases that list known boards (forced wins, draws, blocks).
- **proptest invariants.** Property tests generate arbitrary boards and move sequences to check
  things like legal engine moves and win-detection correctness, where the input space is too large
  to list.
- **The exhaustive never-loses proof.** `engine_never_loses_exhaustive` in `ai.rs` plays every
  possible human move sequence. See [Minimax and the correctness proof](minimax.md).
- **End-to-end scenarios.** `tests/gameplay.rs` source drives whole games through the public API.
- **Rustdoc doctests.** Examples in the crate's doc comments are compiled and run by `cargo test`.
- **This manual's samples.** Every Rust sample in the manual is compiled and run by `mdbook test`,
  so the manual fails to build when the API drifts. The one exception that is only compiled is the
  stdin sketch in [Writing a new front end](new-front-end.md).

## Running them

```text
cargo test --workspace
cargo clippy --all-targets -- -D warnings
scripts/check-docs.sh
```

`scripts/check-docs.sh` is the documentation guard. In order, it:

1. checks that mdbook 0.5.4 is installed;
2. checks that both books have a `book.toml` and `SUMMARY.md`;
3. runs `cargo doc` with warnings as errors;
4. runs the `missing_docs` gate on the engine crate;
5. builds the engine into `target/docs-deps`;
6. builds both books and fails on any warning;
7. runs `mdbook test` on the engine manual against that build;
8. checks that every `SUMMARY.md` entry and every relative link resolves.

It prints `docs OK` on success.

## Why `target/docs-deps`

`mdbook test` compiles each sample with `rustdoc`, which needs a compiled copy of the engine to link
against. Building into a dedicated target directory gives it exactly one copy of the library, so
rustdoc never sees conflicting versions. Because samples are linked with `-L` rather than through
Cargo, each sample has a visible `extern crate kiss_tictactoe_engine;` line that brings the crate
into scope, and a visible `fn main() { ... }` wrapper, so the samples are valid Rust in any editor.

## sccache

On machines with a global sccache `rustc-wrapper`, `dx` needs `RUSTC_WRAPPER=""` (see the user
guide's Troubleshooting chapter). The `cargo` tool and the documentation script work as they are.
