<!-- GSD:project-start source:PROJECT.md -->
## Project

**kiss-tictactoe**

A Rust tic-tac-toe demonstration project showcasing clean library/UI separation. The game engine lives in a platform-agnostic library crate (`kiss-tictactoe-engine`) with a clean API; a Dioxus 0.7.x desktop app (`kiss-tictactoe-ui`) provides the interface. Always user vs. engine — no multiplayer, no networking.

**Core Value:** A fully playable, good-looking tic-tac-toe game where the engine trash talks you and can teach you how to play — demonstrating idiomatic Rust architecture to a friend.

### Constraints

- **Tech Stack**: Rust + Dioxus 0.7.x — no exceptions; webview renderer (not Blitz/Native)
- **UI Styling**: Plain CSS only — no Tailwind, no external CSS frameworks
- **Platform**: Desktop (macOS/Windows/Linux) primary; mobile-ready architecture required
- **Engine purity**: `kiss-tictactoe-engine` must have no Dioxus or platform dependency
- **Dioxus version**: Pin exact Dioxus and dioxus-cli versions in Cargo.toml; verify all APIs against 0.7 docs
<!-- GSD:project-end -->

<!-- GSD:stack-start source:research/STACK.md -->
## Technology Stack

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
### Asset Management
| Technology | Exact Version | Purpose | Why |
|------------|---------------|---------|-----|
| `manganis` | **0.7.10** | Compile-time asset bundling | Ships from the Dioxus monorepo; the `asset!()` macro bundles CSS and images into the binary at compile time. Required for CSS to survive `dx build` bundling correctly on all platforms. |
### Documentation
| Technology | Exact Version | Purpose | Why |
|------------|---------------|---------|-----|
| `mdbook` | **0.5.4** | User guide + engine technical manual | Latest stable as of July 6, 2026. Standard Rust ecosystem doc tooling; supports `cargo doc` integration for API reference and prose user guides in the same project. |
### Engine Testing (dev-dependencies on engine crate only)
| Library | Exact Version | Purpose | When to Use |
|---------|---------------|---------|-------------|
| `proptest` | **1.12.0** | Property-based testing | Board validity invariants, AI move legality, win-detection correctness — anything with an input space too large to enumerate. Latest stable as of October 10, 2026. |
| `rstest` | **0.27.0** | Parameterized test cases | Enumerate known board states (forced wins, draws, blocking moves) without boilerplate. Composes with `proptest`. Latest stable. |
## Cargo Workspace Structure
### Workspace `Cargo.toml`
### `kiss-tictactoe-engine/Cargo.toml`
# NO [dependencies] that touch Dioxus or any platform crate
# Engine must remain usable from CLI, web, and mobile without recompilation
### `kiss-tictactoe-ui/Cargo.toml`
### `kiss-tictactoe-ui/Dioxus.toml`
## State Management Within Dioxus 0.7
| Primitive | Use For |
|-----------|---------|
| `use_signal(|| T)` | Local component state (board display, scoreboard) |
| `Signal<T>` | Shared state passed down the component tree |
| `use_context::<T>()` / `provide_context(T)` | App-wide state (game state, mode) without prop drilling |
| `Memo<T>` (via `use_memo`) | Derived values (current player display, win status) |
## Development Workflow
# From kiss-tictactoe-ui/
# From workspace root
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
## Mobile Readiness Note
- `kiss-tictactoe-engine` has zero platform dependencies — it compiles unchanged for mobile targets
- `kiss-tictactoe-ui` uses Dioxus platform features; swapping `"desktop"` for `"mobile"` in the feature list is the only change needed when mobile stabilizes
- Avoid any `cfg!(target_os = ...)` hacks in the engine — keep it pure Rust logic
## Installation Commands
# Install dx CLI (matches dioxus crate version)
# Or use the official installer (installs latest stable)
# Install mdBook
# Verify
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
<!-- GSD:stack-end -->

<!-- GSD:conventions-start source:CONVENTIONS.md -->
## Conventions

Conventions not yet established. Will populate as patterns emerge during development.
<!-- GSD:conventions-end -->

<!-- GSD:architecture-start source:ARCHITECTURE.md -->
## Architecture

Architecture not yet mapped. Follow existing patterns found in the codebase.
<!-- GSD:architecture-end -->

<!-- GSD:skills-start source:skills/ -->
## Project Skills

No project skills found. Add skills to any of: `.claude/skills/`, `.agents/skills/`, `.cursor/skills/`, `.github/skills/`, or `.codex/skills/` with a `SKILL.md` index file.
<!-- GSD:skills-end -->

<!-- GSD:workflow-start source:GSD defaults -->
## GSD Workflow Enforcement

Before using Edit, Write, or other file-changing tools, start work through a GSD command so planning artifacts and execution context stay in sync.

Use these entry points:
- `/gsd-quick` for small fixes, doc updates, and ad-hoc tasks
- `/gsd-debug` for investigation and bug fixing
- `/gsd-execute-phase` for planned phase work

Do not make direct repo edits outside a GSD workflow unless the user explicitly asks to bypass it.
<!-- GSD:workflow-end -->



<!-- GSD:profile-start -->
## Developer Profile

> Profile not yet configured. Run `/gsd-profile-user` to generate your developer profile.
> This section is managed by `generate-claude-profile` -- do not edit manually.
<!-- GSD:profile-end -->
