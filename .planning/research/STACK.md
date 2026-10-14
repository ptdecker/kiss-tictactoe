# Technology Stack

**Project:** kiss-tictactoe
**Researched:** 2026-10-14
**Confidence:** HIGH — all versions verified against crates.io as of October 2026

---

## Recommended Stack

### Rust Toolchain

| Item | Value | Why |
|------|-------|-----|
| Rust edition | **2024** | Stabilized in Rust 1.85.0 (February 2025); the current standard for new greenfield Rust projects. Enables async closures, improved lifetime capture rules for RPIT, and cleaner `unsafe extern` blocks. |
| MSRV policy | Follow Dioxus 0.7.x | Dioxus drives MSRV; pin workspace to whatever dioxus 0.7.10 requires rather than setting an independent MSRV. |

### Core UI Framework

| Technology | Exact Version | Purpose | Why |
|------------|---------------|---------|-----|
| `dioxus` | **0.7.10** | Primary UI framework | Latest stable 0.7.x as of July 30, 2026. Use with `features = ["desktop"]` for the webview renderer (Wry/Tao). Signals-based reactivity built in. |
| `dioxus-cli` (`dx`) | **0.7.10** | Build tool, hot-reload, bundling | Matches dioxus crate version; install via `cargo install dioxus-cli --version 0.7.10` or the official installer. |

Feature selection on `dioxus` is critical — see "What NOT to Use" section.

### Asset Management

| Technology | Exact Version | Purpose | Why |
|------------|---------------|---------|-----|
| `manganis` | **0.7.10** | Compile-time asset bundling | Ships from the Dioxus monorepo; the `asset!()` macro bundles CSS and images into the binary at compile time. Required for CSS to survive `dx build` bundling correctly on all platforms. |

Note: `manganis` is re-exported through the `dioxus` crate's prelude in 0.7; verify whether a direct `manganis` dependency is needed when you first add CSS assets. If `asset!()` resolves without it, drop the explicit dep.

### Documentation

| Technology | Exact Version | Purpose | Why |
|------------|---------------|---------|-----|
| `mdbook` | **0.5.4** | User guide + engine technical manual | Latest stable as of July 6, 2026. Standard Rust ecosystem doc tooling; supports `cargo doc` integration for API reference and prose user guides in the same project. |

Install: `cargo install mdbook --version 0.5.4`

### Engine Testing (dev-dependencies on engine crate only)

| Library | Exact Version | Purpose | When to Use |
|---------|---------------|---------|-------------|
| `proptest` | **1.12.0** | Property-based testing | Board validity invariants, AI move legality, win-detection correctness — anything with an input space too large to enumerate. Latest stable as of October 10, 2026. |
| `rstest` | **0.27.0** | Parameterized test cases | Enumerate known board states (forced wins, draws, blocking moves) without boilerplate. Composes with `proptest`. Latest stable. |

Standard `#[test]` (built into Rust) covers the rest. No additional test framework is needed.

---

## Cargo Workspace Structure

```
kiss-tictactoe/
├── Cargo.toml                   # workspace root (no [package])
├── Cargo.lock                   # COMMIT — this is an application, not a library
├── kiss-tictactoe-engine/
│   ├── Cargo.toml               # lib crate; zero Dioxus dependency
│   └── src/
│       └── lib.rs
└── kiss-tictactoe-ui/
    ├── Cargo.toml               # bin crate; depends on engine + dioxus
    ├── Dioxus.toml              # dx CLI config (lives here, not at workspace root)
    ├── assets/
    │   └── main.css
    └── src/
        └── main.rs
```

### Workspace `Cargo.toml`

```toml
[workspace]
members = ["kiss-tictactoe-engine", "kiss-tictactoe-ui"]
resolver = "2"

[workspace.package]
edition = "2024"
version = "0.1.0"
authors = ["ptdecker <ptdecker@mac.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/ptdecker/kiss-tictactoe"

[workspace.dependencies]
dioxus        = { version = "0.7.10", features = ["desktop"] }
manganis      = "0.7.10"
kiss-tictactoe-engine = { path = "kiss-tictactoe-engine" }
```

### `kiss-tictactoe-engine/Cargo.toml`

```toml
[package]
name    = "kiss-tictactoe-engine"
version.workspace  = true
edition.workspace  = true
authors.workspace  = true
license.workspace  = true

# NO [dependencies] that touch Dioxus or any platform crate
# Engine must remain usable from CLI, web, and mobile without recompilation

[dev-dependencies]
proptest = "1.12.0"
rstest   = "0.27.0"
```

### `kiss-tictactoe-ui/Cargo.toml`

```toml
[package]
name    = "kiss-tictactoe-ui"
version.workspace  = true
edition.workspace  = true
authors.workspace  = true
license.workspace  = true

[[bin]]
name = "kiss-tictactoe"
path = "src/main.rs"

[dependencies]
dioxus        = { workspace = true }
manganis      = { workspace = true }
kiss-tictactoe-engine = { workspace = true }
```

### `kiss-tictactoe-ui/Dioxus.toml`

```toml
[application]
name = "kiss-tictactoe"
out_dir = "dist"
asset_dir = "assets"

[bundle]
identifier = "com.github.ptdecker.kiss-tictactoe"
publisher  = "ptdecker"
icon       = ["assets/icon.png"]
```

The `dx` CLI reads `Dioxus.toml` from the directory where you run `dx`. Run all `dx` commands from `kiss-tictactoe-ui/`.

---

## State Management Within Dioxus 0.7

No external state management crate is needed. Dioxus 0.7 ships signals-based reactivity as a first-class built-in:

| Primitive | Use For |
|-----------|---------|
| `use_signal(|| T)` | Local component state (board display, scoreboard) |
| `Signal<T>` | Shared state passed down the component tree |
| `use_context::<T>()` / `provide_context(T)` | App-wide state (game state, mode) without prop drilling |
| `Memo<T>` (via `use_memo`) | Derived values (current player display, win status) |

Game state (board, current player, scores, mode) should live in one `provide_context` block at the app root. Components read via `use_context`. The engine is called imperatively on player actions; results flow back through signals.

---

## Development Workflow

```bash
# From kiss-tictactoe-ui/
dx serve --desktop         # hot-reload dev server (desktop window)
dx build --platform desktop --release  # production bundle

# From workspace root
cargo test -p kiss-tictactoe-engine    # engine unit + property tests only
cargo clippy --workspace               # lint all crates
cargo doc --workspace --open           # API reference
```

---

## Alternatives Considered and Rejected

| Category | Recommended | Alternative | Why Rejected |
|----------|-------------|-------------|--------------|
| UI framework | `dioxus` 0.7.10 (desktop feature) | `dioxus-native` (Blitz/Vello) | Experimental GPU renderer; project constraint explicitly requires webview renderer, not Blitz/Native |
| Platform feature | `features = ["desktop"]` on main crate | Separate `dioxus-desktop` crate | Redundant; `desktop` feature on the main crate pulls in the same renderer — separate crate dep adds noise |
| CSS approach | Plain CSS + `asset!()` | Tailwind CSS | Project constraint: plain CSS only; Tailwind adds a Node.js build step |
| CSS approach | Plain CSS + `asset!()` | `dioxus-material-icons` or similar | Out of scope; adds external visual design system the project doesn't need |
| State management | Built-in Dioxus signals | `leptos-signals`, external stores | Signals are built into Dioxus 0.7 — external library adds dep for zero benefit at this scale |
| Testing | `proptest` + `rstest` + `#[test]` | `quickcheck` | `proptest` is more ergonomic for structured inputs (boards, positions); actively maintained with higher version at 1.12.0 |
| Fullstack | n/a | `dioxus-fullstack` | SSR/server functions are not needed for a local desktop game |
| Router | n/a | `dioxus-router` | No page routing needed; single-view desktop app |
| Documentation | `mdbook` | `rustdoc` only | Project requires both a user guide (prose) and API docs; mdBook serves the user guide, `cargo doc` covers the API — they complement rather than replace |

---

## Mobile Readiness Note

**Mobile builds are out of scope for v1** per PROJECT.md. However, the architecture must remain mobile-ready.

The standalone `dioxus-mobile` crate is at **0.6.2 stable** (the 0.7.x version is still in alpha as of research date). Mobile support for 0.7 exists as a feature flag in the main `dioxus` crate (`features = ["mobile"]`), but it is not yet stable.

**Mobile-ready architecture means:**
- `kiss-tictactoe-engine` has zero platform dependencies — it compiles unchanged for mobile targets
- `kiss-tictactoe-ui` uses Dioxus platform features; swapping `"desktop"` for `"mobile"` in the feature list is the only change needed when mobile stabilizes
- Avoid any `cfg!(target_os = ...)` hacks in the engine — keep it pure Rust logic

Do not add `features = ["mobile"]` yet. When Dioxus 0.7.x mobile stabilizes, a single feature flag change enables mobile builds without touching engine code.

---

## Installation Commands

```bash
# Install dx CLI (matches dioxus crate version)
cargo install dioxus-cli --version 0.7.10

# Or use the official installer (installs latest stable)
curl -fsSL https://dioxuslabs.com/install.sh | bash

# Install mdBook
cargo install mdbook --version 0.5.4

# Verify
dx --version    # should show 0.7.10
mdbook --version  # should show 0.5.4
```

---

## Sources

- crates.io: `dioxus` — https://crates.io/crates/dioxus (latest stable: 0.7.10, July 30, 2026)
- crates.io: `dioxus-cli` — https://crates.io/crates/dioxus-cli (latest stable: 0.7.10, July 30, 2026)
- crates.io: `dioxus-desktop` — https://crates.io/crates/dioxus-desktop (0.7.10, confirms renderer)
- crates.io: `dioxus-mobile` — https://crates.io/crates/dioxus-mobile (0.6.2 stable; 0.7.0-alpha.2 pre-release)
- crates.io: `manganis` — https://crates.io/crates/manganis (0.7.10, Dioxus monorepo)
- crates.io: `mdbook` — https://crates.io/crates/mdbook (0.5.4, July 6, 2026)
- crates.io: `proptest` — https://crates.io/crates/proptest (1.12.0, October 10, 2026)
- crates.io: `rstest` — https://crates.io/crates/rstest (0.27.0)
- Context7 / Dioxus docs: Feature flags, workspace structure, platform priority order — https://github.com/dioxuslabs/dioxus/blob/main/packages/dioxus/src/launch.rs
- Context7 / Dioxus docs: Desktop readme — https://github.com/dioxuslabs/dioxus/blob/main/packages/desktop/src/readme.md
- Context7 / Dioxus docs: Dioxus.toml config — https://github.com/rustgrow/dioxus-context7-en/blob/main/guides/tools/configure.md
- Rust 1.85.0 release: Rust 2024 edition stabilized — https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/
