# Architecture Patterns

**Domain:** Rust desktop game with platform-agnostic engine library
**Project:** kiss-tictactoe
**Researched:** 2026-10-14
**Confidence:** HIGH (primary sources: Context7 / Dioxus 0.7 official docs)

---

## Recommended Architecture

```
kiss-tictactoe/              ← workspace root
├── Cargo.toml               ← [workspace] manifest
├── Cargo.lock
├── docs/                    ← mdBook at workspace root (not per-crate)
│   ├── book.toml
│   ├── src/
│   │   ├── SUMMARY.md
│   │   ├── user-guide/      ← how-to-play chapters
│   │   └── engine/          ← technical API manual chapters
│   └── book/                ← rendered output (gitignored)
├── crates/
│   ├── kiss-tictactoe-engine/   ← lib crate, zero Dioxus dependency
│   │   ├── Cargo.toml
│   │   └── src/
│   │       └── lib.rs
│   └── kiss-tictactoe-ui/       ← bin crate, depends on engine
│       ├── Cargo.toml
│       ├── assets/
│       │   └── main.css
│       └── src/
│           ├── main.rs
│           └── components/
│               ├── mod.rs
│               ├── board.rs
│               ├── cell.rs
│               ├── speech_bubble.rs
│               ├── scoreboard.rs
│               └── mode_toggle.rs
```

---

## Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|---------------|-------------------|
| `kiss-tictactoe-engine` | Game state, AI (minimax + training), win/draw detection, trash-talk messages, hints | None (pure Rust, no UI) |
| `kiss-tictactoe-ui` | Renders state from engine, handles user input, manages Dioxus signals | Engine lib only |
| `App` (root component) | Provides app-wide context (GameState signal, scoreboard signal), owns engine instance wrapped in signal | All child components |
| `Board` | Renders the 3×3 grid, dispatches click events upward via callback | `Cell` × 9 |
| `Cell` | Renders one square (X / O / empty), fires `onclick` | None |
| `SpeechBubble` | Reads current trash-talk string from context, renders speech bubble | Reads context |
| `Scoreboard` | Reads win/loss/draw counters from context, renders session stats | Reads context |
| `ModeToggle` | Reads and writes mode signal (Normal / Training) | Reads/writes context |

---

## Data Flow

```
User clicks cell
      │
      ▼
Board component → onclick handler (closure capturing GameState signal)
      │
      ▼
signal.write().engine.make_move(cell)      ← calls engine API
      │
      ▼
Engine returns new GameState (board, status, trash_talk, hints)
      │
      ▼
Signal invalidates → Dioxus re-renders affected components
      │
      ├── Board re-reads board array from signal
      ├── SpeechBubble re-reads trash_talk from signal
      ├── Scoreboard re-reads score from signal (updated on game-over)
      └── ModeToggle reads/writes mode signal independently
```

The UI crate never implements game logic. It only calls engine methods and stores the results in Dioxus signals.

---

## Engine API Boundary Design

The engine crate exposes plain Rust types with no Dioxus dependency. The UI wraps them in a signal.

**Engine public API surface (sketch):**

```rust
// In kiss-tictactoe-engine/src/lib.rs

pub struct Game {
    pub board: Board,
    pub status: GameStatus,
    pub current_player: Player,
}

pub struct Board {
    pub cells: [Option<Player>; 9],
}

pub enum GameStatus {
    InProgress,
    Won(Player),
    Draw,
}

pub enum Player { X, O }

pub enum Mode { Normal, Training }

pub struct MoveResult {
    pub game: Game,
    pub trash_talk: String,
    pub hint: Option<usize>,  // Some(cell_index) in Training mode
}

impl Game {
    pub fn new() -> Self { ... }
    pub fn make_player_move(&mut self, cell: usize) -> Result<MoveResult, MoveError> { ... }
    pub fn make_engine_move(&mut self, mode: Mode) -> MoveResult { ... }
    pub fn reset(&mut self) -> MoveResult { ... }
}
```

**UI wraps engine in a signal (in App):**

```rust
// In kiss-tictactoe-ui/src/main.rs or components/app.rs

use dioxus::prelude::*;
use kiss_tictactoe_engine::{Game, MoveResult, Mode};

#[derive(Clone)]
struct AppState {
    game: Signal<Game>,
    last_move: Signal<Option<MoveResult>>,
    mode: Signal<Mode>,
    score: Signal<Score>,
}

#[component]
pub fn App() -> Element {
    let state = use_context_provider(|| AppState {
        game: Signal::new(Game::new()),
        last_move: Signal::new(None),
        mode: Signal::new(Mode::Normal),
        score: Signal::new(Score::default()),
    });

    rsx! {
        div { class: "app",
            Scoreboard {}
            ModeToggle {}
            SpeechBubble {}
            Board {}
        }
    }
}
```

Child components consume context without prop drilling:

```rust
#[component]
fn Board() -> Element {
    let state = use_context::<AppState>();
    // ...
}
```

---

## State Management Pattern (Dioxus 0.7.x)

**Recommended approach: `use_context_provider` + `use_context` with Signals at the App root.**

Dioxus 0.7.x replaced `use_shared_state_provider`/`use_shared_state` with the context API:

```rust
// Provider (App root):
let state = use_context_provider(|| MyState {
    count: Signal::new(0),
});

// Consumer (any child):
let mut count = use_context::<MyState>().count;
```

**Three tiers of state, in order of preference:**

| Tier | Mechanism | When to Use |
|------|-----------|-------------|
| Local component state | `use_signal(|| value)` | State used only within one component (e.g., hover highlight on a cell) |
| Shared tree state | `use_context_provider` + `use_context::<T>()` | State shared across the component tree (game state, score, mode, trash talk) |
| Global static state | `static FOO: GlobalSignal<T> = Signal::global(...)` | App-wide state requiring no passing; avoid for testability |

For this project: use `use_context_provider` at the `App` root with a struct holding all game signals. This keeps state testable and co-located with the component tree.

---

## Platform-Specific Code Pattern

**Dioxus 0.7.x uses Cargo feature flags to select the renderer** — the same `dioxus::launch(App)` entry point works for all platforms; the right renderer is compiled in based on which feature is active.

**Cargo.toml for the UI crate:**

```toml
[dependencies]
dioxus = { version = "0.7", features = [] }  # No renderer feature here
kiss-tictactoe-engine = { workspace = true }

[features]
default = []
desktop = ["dioxus/desktop"]
mobile  = ["dioxus/mobile"]
bundle  = []  # For Windows subsystem suppression
```

**main.rs entry point (works for desktop and mobile unchanged):**

```rust
use dioxus::prelude::*;

#[cfg_attr(feature = "bundle", windows_subsystem = "windows")]
fn main() {
    dioxus::launch(App);
}
```

**Platform-specific code (e.g., a future native file dialog) behind cfg:**

```rust
// Define a trait in the UI crate
trait PlatformServices {
    fn save_score_export(&self) -> Option<std::path::PathBuf>;
}

// Desktop implementation
#[cfg(feature = "desktop")]
struct DesktopServices;

#[cfg(feature = "desktop")]
impl PlatformServices for DesktopServices {
    fn save_score_export(&self) -> Option<std::path::PathBuf> {
        // native file dialog via rfd or similar
        todo!()
    }
}

// Mobile implementation (for future v2)
#[cfg(feature = "mobile")]
struct MobileServices;

#[cfg(feature = "mobile")]
impl PlatformServices for MobileServices {
    fn save_score_export(&self) -> Option<std::path::PathBuf> {
        None // mobile v2 can implement share sheet
    }
}
```

For v1 (desktop only), the `PlatformServices` trait is the right place to encapsulate anything OS-specific. The engine crate never sees it.

---

## Workspace Cargo.toml Structure

**Root `Cargo.toml`:**

```toml
[workspace]
members = [
    "crates/kiss-tictactoe-engine",
    "crates/kiss-tictactoe-ui",
]
resolver = "2"

[workspace.dependencies]
dioxus = { version = "0.7" }
kiss-tictactoe-engine = { path = "crates/kiss-tictactoe-engine" }
```

**Engine `Cargo.toml`:**

```toml
[package]
name = "kiss-tictactoe-engine"
version = "0.1.0"
edition = "2024"

[dependencies]
# No dioxus. No platform crates. Pure Rust only.
# rand = "0.9"  # if needed for training mode randomization
```

**UI `Cargo.toml`:**

```toml
[package]
name = "kiss-tictactoe-ui"
version = "0.1.0"
edition = "2024"

[dependencies]
dioxus = { workspace = true }
kiss-tictactoe-engine = { workspace = true }

[features]
default = []
desktop = ["dioxus/desktop"]
mobile  = ["dioxus/mobile"]
bundle  = []
```

---

## Minimal Dioxus 0.7.x Desktop App

A minimal working desktop app (verified against Dioxus 0.7 official docs):

```rust
use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div { class: "app",
            h1 { "Kiss Tic-Tac-Toe" }
        }
    }
}
```

Build and run:

```bash
# Development
dx serve --platform desktop

# Which is equivalent to enabling:
# features = ["desktop"], target = host, profile = "desktop-dev"

# Production bundle
dx bundle --platform desktop
```

---

## mdBook Documentation Organization

**Place mdBook at workspace root in `docs/`** (not inside either crate). This is the standard Rust workspace pattern for project-level documentation that spans multiple crates.

```
docs/
├── book.toml           ← root mdBook config
└── src/
    ├── SUMMARY.md
    ├── README.md        ← landing page
    ├── user-guide/
    │   ├── README.md
    │   ├── how-to-play.md
    │   └── training-mode.md
    └── engine/
        ├── README.md
        ├── api-reference.md
        └── architecture.md
```

**`docs/book.toml`:**

```toml
[book]
title = "Kiss Tic-Tac-Toe"
authors = ["ptdecker"]
src = "src"
language = "en"

[output.html]
no-section-label = false

[output.html.fold]
enable = true
level = 1
```

Keep `docs/book/` (rendered output) in `.gitignore`. Auto-generate rustdoc via `cargo doc` separately; it does not need to be embedded in the mdBook for v1.

---

## Build Order

The engine crate has no dependencies on the UI, so it must be built and tested first.

**Development workflow:**

```bash
# 1. Test engine in isolation (must pass before touching UI)
cargo test -p kiss-tictactoe-engine

# 2. Build/run UI (depends on engine)
dx serve --platform desktop

# 3. Build docs independently
cd docs && mdbook build
```

**CI pipeline order:**

```
Step 1: cargo fmt --check
Step 2: cargo clippy --workspace -- -D warnings
Step 3: cargo test -p kiss-tictactoe-engine   ← engine tests, no UI needed
Step 4: cargo build -p kiss-tictactoe-ui --features desktop
Step 5: mdbook build docs/
```

**Rationale:** The engine is the dependency; it must be correct before integrating with the UI. This also lets engine tests run without needing dioxus-cli or a display environment.

---

## Mobile-Readiness Patterns

Architecture is mobile-ready even though v1 ships desktop only.

| Concern | Desktop v1 | Mobile v2 (future) |
|---------|-----------|-------------------|
| Renderer selection | `features = ["desktop"]` → webview | `features = ["mobile"]` → mobile renderer |
| Entry point | `dioxus::launch(App)` | Same, no change |
| Platform services | `#[cfg(feature = "desktop")]` impl | Add `#[cfg(feature = "mobile")]` impl |
| Layout | Responsive CSS (flexbox, no fixed px breakpoints) | Same CSS works; test with small viewport |
| Engine | Zero changes needed | Zero changes needed |
| Async/coroutines | `use_coroutine` for engine calls | Same API on mobile |

**CSS guidance for mobile-readiness:** Use `vw`/`vh`/`%` units and CSS Grid/Flexbox. Avoid `px` for the board layout. A 3×3 grid in CSS Grid with `aspect-ratio: 1` cells scales cleanly from phone to desktop without media queries.

**What changes at the feature flag level:**

```toml
# dx serve --ios  enables:
# target = aarch64-apple-ios-sim
# features = "mobile"  (i.e., dioxus/mobile)
# profile = "ios-dev"
```

No changes to `App`, `Board`, or engine code are needed when enabling mobile — only the Cargo feature and any `#[cfg(feature = "mobile")]` platform-service implementations.

---

## Anti-Patterns to Avoid

### Anti-Pattern 1: Engine logic bleeding into the UI crate
**What goes wrong:** Game rules, move validation, or AI code in Dioxus components.
**Why bad:** Breaks reusability, can't test engine without running the UI.
**Instead:** Engine crate is the single source of truth for all game logic. UI only calls engine methods and reacts to their return values.

### Anti-Pattern 2: Prop drilling game state through many components
**What goes wrong:** Passing `game`, `score`, `mode` as props through 3+ component layers.
**Why bad:** Fragile, verbose, couples component hierarchy to data shape.
**Instead:** Provide `AppState` via `use_context_provider` at `App` root; children consume with `use_context::<AppState>()`.

### Anti-Pattern 3: Mutable state directly in engine struct without wrapping in Signal
**What goes wrong:** Calling `game.make_move()` but storing game outside a Signal.
**Why bad:** Dioxus will not know to re-render; UI goes stale.
**Instead:** Always wrap the engine Game in `Signal::new(Game::new())`. Mutate via `signal.write().make_move(cell)`.

### Anti-Pattern 4: Pinning dioxus features in `[dependencies]` of the UI crate directly
**What goes wrong:** Hard-coding `features = ["desktop"]` in the dependency declaration.
**Why bad:** Impossible to build for mobile later without touching the dependency line.
**Instead:** `dioxus = { version = "0.7" }` with no features in dependencies; desktop/mobile features declared only in `[features]` and activated by `dx serve --platform`.

### Anti-Pattern 5: mdBook inside either crate
**What goes wrong:** `crates/kiss-tictactoe-engine/docs/` for user-guide content.
**Why bad:** User-facing docs have nothing to do with the engine crate; engine-specific API docs belong in rustdoc (`///` comments), not a separate mdBook chapter inside the crate.
**Instead:** `docs/` at workspace root for the mdBook user guide and architecture narrative; rustdoc auto-generated via `cargo doc` for API reference.

---

## Scalability Considerations

| Concern | For this project (single desktop app) | If scope grows |
|---------|--------------------------------------|---------------|
| State complexity | Single `AppState` struct in context is sufficient | Split into `GameContext` + `UiContext` if more than ~5 signals |
| Engine variants | One `Game` struct with `Mode` enum is sufficient | New engine crate per variant (e.g., 4×4 board) without touching UI |
| Platform targets | Desktop only (v1) | Add `#[cfg(feature = "mobile")]` service impls; no App component changes |
| Documentation | Single mdBook with 2 sections | Separate mdBooks per audience (user vs developer) |

---

## Sources

- Dioxus 0.7 official documentation (via Context7 `/llmstxt/dioxuslabs_learn_0_7_llms-full_txt`): https://dioxuslabs.com/learn/0.7/llms-full.txt
- Dioxus workspace structure pattern: Context7, HIGH confidence
- Dioxus state management (signals, context): Context7, HIGH confidence
- Dioxus platform feature flags: Context7, HIGH confidence
- Dioxus `launch` / `LaunchBuilder` API: Context7, HIGH confidence
- mdBook workspace-root placement: convention from Rust ecosystem practice, MEDIUM confidence (no single canonical reference found)
