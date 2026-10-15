# Troubleshooting

## sccache: Compiler not supported

**Symptom:** `dx serve` or `dx build` fails with `sccache: Compiler not supported`.

**Cause:** Your global `~/.cargo/config.toml` sets `build.rustc-wrapper = "sccache"`. dx adds its own rustc wrapper on top, so cargo ends up running `sccache dx rustc`, and sccache does not recognise `dx` as a compiler. Plain `cargo build` and `cargo test` are unaffected.

**Fix 1 (permanent):** add this shell function to `~/.zshrc`, then open a new terminal:

```sh
dx() { RUSTC_WRAPPER= command dx "$@"; }
```

**Fix 2 (one-off, or in scripts):** clear the wrapper for a single run:

```sh
RUSTC_WRAPPER="" dx serve
```

## mdbook: command not found

Building these docs needs mdbook 0.5.4. If the command is missing or is a different version, install it:

```sh
cargo install mdbook --version 0.5.4 --locked
```
