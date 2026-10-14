# Phase 2: Playable Game - Pattern Map

**Mapped:** 2026-10-14
**Files analyzed:** 13 (new or modified)
**Analogs found:** 5 / 13 (codebase is Phase 1 scaffold; engine files have no prior analog)

---

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `kiss-tictactoe-engine/src/lib.rs` | module root | n/a (declarations only) | itself (placeholder → real) | self |
| `kiss-tictactoe-engine/src/types.rs` | model | n/a (pure types) | none | no analog |
| `kiss-tictactoe-engine/src/game.rs` | service | CRUD | none | no analog |
| `kiss-tictactoe-engine/src/detection.rs` | utility | transform | none | no analog |
| `kiss-tictactoe-engine/src/ai.rs` | utility | transform | none | no analog |
| `kiss-tictactoe-engine/Cargo.toml` | config | n/a | `kiss-tictactoe-ui/Cargo.toml` | partial |
| `kiss-tictactoe-ui/src/main.rs` | component (root) | request-response | itself (placeholder body → real) | self |
| `kiss-tictactoe-ui/src/components/mod.rs` | module root | n/a (declarations only) | `kiss-tictactoe-ui/src/main.rs` | partial |
| `kiss-tictactoe-ui/src/components/board.rs` | component | request-response | `kiss-tictactoe-ui/src/main.rs` | role-match |
| `kiss-tictactoe-ui/src/components/cell.rs` | component | request-response | `kiss-tictactoe-ui/src/main.rs` | role-match |
| `kiss-tictactoe-ui/src/components/game_over_overlay.rs` | component | request-response | `kiss-tictactoe-ui/src/main.rs` | role-match |
| `kiss-tictactoe-ui/assets/main.css` | config (styles) | n/a | itself (minimal reset → full styles) | self |
| `kiss-tictactoe-ui/Cargo.toml` | config | n/a | itself (add engine dep) | self |

---

## Pattern Assignments

### `kiss-tictactoe-engine/src/lib.rs` (module root)

**Status:** Modify existing placeholder (currently 1 line comment)

**Analog:** itself — becomes the real module root

**Pattern: Module root with crate doc and pub mod declarations**
```rust
// Established convention: lib.rs declares pub mods and re-exports the public API.
// No logic lives here — logic goes in the modules declared below.

//! Game engine for kiss-tictactoe.
//!
//! `Game` is the primary entry point. Create one with `Game::new()`, call
//! `game.make_move(pos)` to advance state, and read `game.status()` to detect outcomes.

pub mod types;
pub mod game;
pub mod detection;
pub mod ai;

// Re-export the public API surface at crate root for ergonomic use by the UI crate.
pub use types::{Position, Player, Board, GameStatus, MoveResult, MoveError};
pub use game::Game;
```

---

### `kiss-tictactoe-engine/src/types.rs` (model, pure types)

**Analog:** None in the codebase. Use RESEARCH.md Pattern 1 (lines 219–261) directly.

**Pattern: Public value types — derive macros and documented variants**
```rust
// Source: 02-RESEARCH.md Pattern 1 + decisions D-01, D-02, D-03

/// A position on the 3×3 board. `row` and `col` are each 0–2. (D-02)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    X,
    O,
}

/// 3×3 board; `None` means empty. Indexed `board[row][col]`.
pub type Board = [[Option<Player>; 3]; 3];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Win(Player),
    Draw,
}

/// What the caller learns after a successful `make_move`. (D-03)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Continuing,
    GameOver(GameStatus),
}

/// Why `make_move` failed. (D-03)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    /// The target cell already has a mark.
    CellOccupied,
    /// `row` or `col` is outside 0–2.
    InvalidPosition,
    /// `make_move` was called after the game ended.
    GameAlreadyOver,
}
```

**Convention notes:**
- All public types use `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` — Copy is safe because all types are small value types.
- `Board` is a type alias (not a newtype) for indexing ergonomics: `board[row][col]` maps naturally to `Position { row, col }`.
- `MoveError::GameAlreadyOver` is included per RESEARCH.md recommendation even though D-03 only names two variants.

---

### `kiss-tictactoe-engine/src/game.rs` (service, CRUD)

**Analog:** None in the codebase. Use RESEARCH.md Pattern 1 (API shape) and Pattern 5 (minimax call site).

**Pattern: Game struct with private fields and public methods**
```rust
// Source: 02-RESEARCH.md Pattern 1 (D-01 through D-04)

use crate::types::{Board, GameStatus, MoveError, MoveResult, Player, Position};
use crate::detection::detect_status;
use crate::ai::best_engine_move;

pub struct Game {
    board: Board,
    current_player: Player,
    status: GameStatus,
    history: Vec<Position>,  // store for Phase 3 review (discretion decision)
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: [[None; 3]; 3],
            current_player: Player::X,   // X always goes first (discretion decision)
            status: GameStatus::InProgress,
            history: Vec::new(),
        }
    }

    /// Places the human's mark at `pos`, then (if game not over) places the engine's mark.
    /// One call advances both turns. (D-04)
    pub fn make_move(&mut self, pos: Position) -> Result<MoveResult, MoveError> {
        if self.status != GameStatus::InProgress {
            return Err(MoveError::GameAlreadyOver);
        }
        if pos.row > 2 || pos.col > 2 {
            return Err(MoveError::InvalidPosition);
        }
        if self.board[pos.row as usize][pos.col as usize].is_some() {
            return Err(MoveError::CellOccupied);
        }

        // Place human mark (always X per discretion decision)
        self.board[pos.row as usize][pos.col as usize] = Some(Player::X);
        self.history.push(pos);
        self.status = detect_status(&self.board);

        if self.status != GameStatus::InProgress {
            return Ok(MoveResult::GameOver(self.status));
        }

        // Engine responds (always O)
        let engine_pos = best_engine_move(&self.board);
        self.board[engine_pos.row as usize][engine_pos.col as usize] = Some(Player::O);
        self.history.push(engine_pos);
        self.status = detect_status(&self.board);

        Ok(if self.status == GameStatus::InProgress {
            MoveResult::Continuing
        } else {
            MoveResult::GameOver(self.status)
        })
    }

    pub fn status(&self) -> GameStatus {
        self.status
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn history(&self) -> &[Position] {
        &self.history
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
```

**Error handling pattern:** `make_move` returns `Result<MoveResult, MoveError>` — caller uses `let _ = game.write().make_move(pos)` in the UI (errors are UI-prevented but API stays honest per D-03).

---

### `kiss-tictactoe-engine/src/detection.rs` (utility, transform)

**Analog:** None in the codebase. Use RESEARCH.md Code Examples (lines 562–597) directly.

**Pattern: Compile-time win lines + full-board draw check**
```rust
// Source: 02-RESEARCH.md Code Examples section

use crate::types::{Board, GameStatus};

const WINNING_LINES: [[(usize, usize); 3]; 8] = [
    // rows
    [(0,0),(0,1),(0,2)],
    [(1,0),(1,1),(1,2)],
    [(2,0),(2,1),(2,2)],
    // columns
    [(0,0),(1,0),(2,0)],
    [(0,1),(1,1),(2,1)],
    [(0,2),(1,2),(2,2)],
    // diagonals
    [(0,0),(1,1),(2,2)],
    [(0,2),(1,1),(2,0)],
];

/// Returns `GameStatus::Win(p)` if player `p` has completed a line,
/// `GameStatus::Draw` if the board is full with no winner,
/// or `GameStatus::InProgress` otherwise.
pub fn detect_status(board: &Board) -> GameStatus {
    for line in &WINNING_LINES {
        let [a, b, c] = *line;
        if let (Some(p1), Some(p2), Some(p3)) =
            (board[a.0][a.1], board[b.0][b.1], board[c.0][c.1])
        {
            if p1 == p2 && p2 == p3 {
                return GameStatus::Win(p1);
            }
        }
    }
    if board.iter().all(|row| row.iter().all(|cell| cell.is_some())) {
        return GameStatus::Draw;
    }
    GameStatus::InProgress
}
```

**Test pattern (inline `#[cfg(test)]` with rstest):**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Player::{O, X};
    use rstest::rstest;

    #[rstest]
    #[case([[Some(X),Some(X),Some(X)],[None,None,None],[None,None,None]], GameStatus::Win(X))]
    #[case([[None,Some(O),None],[None,Some(O),None],[None,Some(O),None]], GameStatus::Win(O))]
    #[case([[Some(X),Some(O),Some(X)],[Some(X),Some(X),Some(O)],[Some(O),Some(X),Some(O)]], GameStatus::Draw)]
    #[case([[None,None,None],[None,None,None],[None,None,None]], GameStatus::InProgress)]
    fn win_detection(#[case] board: Board, #[case] expected: GameStatus) {
        assert_eq!(detect_status(&board), expected);
    }
}
```

---

### `kiss-tictactoe-engine/src/ai.rs` (utility, transform)

**Analog:** None in the codebase. Use RESEARCH.md Pattern 5 (lines 375–425) directly.

**Pattern: Minimax with alpha-beta pruning, depth-adjusted scoring**
```rust
// Source: 02-RESEARCH.md Pattern 5
// Critical: depth-adjusted scores prevent "indifferent" play (Pitfall 4)

use crate::types::{Board, GameStatus, Player, Position};
use crate::detection::detect_status;

fn empty_cells(board: &Board) -> impl Iterator<Item = Position> + '_ {
    (0u8..3).flat_map(move |row| {
        (0u8..3).filter_map(move |col| {
            if board[row as usize][col as usize].is_none() {
                Some(Position { row, col })
            } else {
                None
            }
        })
    })
}

fn minimax(board: &Board, depth: i32, is_maximizing: bool, alpha: i32, beta: i32) -> i32 {
    match detect_status(board) {
        GameStatus::Win(Player::X) => return 10 - depth,  // sooner win scores higher
        GameStatus::Win(Player::O) => return depth - 10,  // sooner O win scores lower
        GameStatus::Draw => return 0,
        GameStatus::InProgress => {}
    }

    if is_maximizing {
        let mut best = i32::MIN;
        let mut alpha = alpha;
        for pos in empty_cells(board) {
            let mut b = *board;
            b[pos.row as usize][pos.col as usize] = Some(Player::X);
            best = best.max(minimax(&b, depth + 1, false, alpha, beta));
            alpha = alpha.max(best);
            if beta <= alpha { break; }
        }
        best
    } else {
        let mut best = i32::MAX;
        let mut beta = beta;
        for pos in empty_cells(board) {
            let mut b = *board;
            b[pos.row as usize][pos.col as usize] = Some(Player::O);
            best = best.min(minimax(&b, depth + 1, true, alpha, beta));
            beta = beta.min(best);
            if beta <= alpha { break; }
        }
        best
    }
}

/// Returns the best move for the engine (O, minimizer).
/// Panics if no empty cell exists — caller must check `status == InProgress` first.
pub fn best_engine_move(board: &Board) -> Position {
    let mut best_score = i32::MAX;
    let mut best_pos = None;
    for pos in empty_cells(board) {
        let mut b = *board;
        b[pos.row as usize][pos.col as usize] = Some(Player::O);
        let score = minimax(&b, 0, true, i32::MIN, i32::MAX);
        if score < best_score {
            best_score = score;
            best_pos = Some(pos);
        }
    }
    best_pos.expect("best_engine_move called with no empty cells")
}
```

**Test pattern (inline `#[cfg(test)]` with proptest):**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use crate::game::Game;

    proptest! {
        #[test]
        fn engine_never_makes_illegal_move(moves in prop::collection::vec(0u8..9, 0..=4)) {
            // Build a game state from a sequence of valid human moves and verify
            // the engine never places on an occupied cell.
            // (Implementation detail: strategy generates valid position sequences)
        }
    }
}
```

---

### `kiss-tictactoe-engine/Cargo.toml` (config)

**Analog:** `kiss-tictactoe-ui/Cargo.toml` (same workspace, same structure)

**Existing analog (lines 1–9 of `kiss-tictactoe-ui/Cargo.toml`):**
```toml
[package]
name = "kiss-tictactoe-ui"
version = "0.1.0"
edition = "2024"
license = "MIT"

[dependencies]
dioxus = { version = "0.7.10", features = ["desktop"] }
manganis = "0.7.10"
```

**Target pattern for engine (copy structure, add only dev-deps):**
```toml
[package]
name = "kiss-tictactoe-engine"
version = "0.1.0"
edition = "2024"
license = "MIT"

[dependencies]
# intentionally empty — engine has zero runtime deps (ENGI-03 purity gate)

[dev-dependencies]
proptest = "1.12.0"
rstest = "0.27.0"
```

---

### `kiss-tictactoe-ui/src/main.rs` (component root, request-response)

**Analog:** itself — replace the placeholder `div` body; keep the `asset!()` and `dioxus::launch` frame intact.

**Existing code to preserve (lines 1–7 of `kiss-tictactoe-ui/src/main.rs`):**
```rust
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}
```

**Target pattern for App component body:**
```rust
// Source: 02-RESEARCH.md Pattern 2 (lines 270–284)
// Add: mod components declaration; import Signal<Game>; use_signal

mod components;
use components::board::Board;
use kiss_tictactoe_engine::Game;

// ... (MAIN_CSS and main() preserved) ...

#[component]
fn App() -> Element {
    let game = use_signal(|| Game::new());

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div { class: "game-container",
            h1 { "Tic-Tac-Toe" }
            Board { game }
        }
    }
}
```

---

### `kiss-tictactoe-ui/src/components/mod.rs` (module root)

**Analog:** `kiss-tictactoe-ui/src/main.rs` (partial — established the file-per-component structure)

**Pattern: Simple pub mod declarations, one per component file**
```rust
// Source: RESEARCH.md recommended project structure

pub mod board;
pub mod cell;
pub mod game_over_overlay;
```

---

### `kiss-tictactoe-ui/src/components/board.rs` (component, request-response)

**Analog:** `kiss-tictactoe-ui/src/main.rs` (role-match — only existing Dioxus component)

**Existing component pattern from analog (lines 9–15 of `main.rs`):**
```rust
#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div { "kiss-tictactoe — coming soon" }
    }
}
```

**Target pattern for Board — extends the `#[component]` pattern with Signal prop, iteration, and conditional rendering:**
```rust
// Source: 02-RESEARCH.md Pattern 3 (lines 291–329)
// Key rule: clone board/status OUT of .read() before the rsx! block to avoid
// holding the Ref guard (RESEARCH.md Pitfall 1 / anti-patterns).

use dioxus::prelude::*;
use kiss_tictactoe_engine::{Board as GameBoard, Game, GameStatus, Position};
use crate::components::cell::Cell;
use crate::components::game_over_overlay::GameOverOverlay;

#[component]
pub fn Board(game: Signal<Game>) -> Element {
    let board = game.read().board().clone();
    let status = game.read().status();   // Copy type — guard drops immediately

    rsx! {
        div { class: "board-wrapper",
            div { class: "board",
                {board.iter().enumerate().flat_map(|(row, cols)| {
                    cols.iter().enumerate().map(move |(col, &cell)| {
                        let pos = Position { row: row as u8, col: col as u8 };
                        let interactive = cell.is_none() && status == GameStatus::InProgress;
                        rsx! {
                            Cell {
                                key: "{row}-{col}",
                                pos,
                                content: cell,
                                interactive,
                                on_click: move |p: Position| {
                                    let _ = game.write().make_move(p);
                                }
                            }
                        }
                    })
                })}
            }
            if status != GameStatus::InProgress {
                GameOverOverlay {
                    status,
                    on_play_again: move |_| game.set(Game::new()),
                }
            }
        }
    }
}
```

---

### `kiss-tictactoe-ui/src/components/cell.rs` (component, request-response)

**Analog:** `kiss-tictactoe-ui/src/main.rs` (role-match)

**Target pattern — multiple conditional `class:` attributes (RESEARCH.md Pattern 4):**
```rust
// Source: 02-RESEARCH.md Pattern 4 (lines 338–368)

use dioxus::prelude::*;
use kiss_tictactoe_engine::{Player, Position};

#[component]
pub fn Cell(
    pos: Position,
    content: Option<Player>,
    interactive: bool,
    on_click: EventHandler<Position>,
) -> Element {
    rsx! {
        div {
            class: "cell",
            class: if content == Some(Player::X) { "cell--x" },
            class: if content == Some(Player::O) { "cell--o" },
            class: if !interactive { "cell--frozen" },
            onclick: move |_| {
                if interactive {
                    on_click.call(pos);
                }
            },
            {match content {
                Some(Player::X) => "X",
                Some(Player::O) => "O",
                None => "",
            }}
        }
    }
}
```

---

### `kiss-tictactoe-ui/src/components/game_over_overlay.rs` (component, request-response)

**Analog:** `kiss-tictactoe-ui/src/main.rs` (role-match)

**Target pattern — EventHandler<()> with match on GameStatus:**
```rust
// Source: 02-RESEARCH.md Code Examples / GameOverOverlay section (lines 601–624)
// D-06: outcome text is user-centric ("You win!" etc.)

use dioxus::prelude::*;
use kiss_tictactoe_engine::{GameStatus, Player};

#[component]
pub fn GameOverOverlay(status: GameStatus, on_play_again: EventHandler<()>) -> Element {
    let outcome_text = match status {
        GameStatus::Win(Player::X) => "You win!",
        GameStatus::Win(Player::O) => "Engine wins!",
        GameStatus::Draw => "Draw!",
        GameStatus::InProgress => unreachable!("overlay only shown on game over"),
    };

    rsx! {
        div { class: "game-over-overlay",
            span { class: "outcome-text", "{outcome_text}" }
            button {
                class: "play-again-btn",
                onclick: move |_| on_play_again.call(()),
                "Play again"
            }
        }
    }
}
```

---

### `kiss-tictactoe-ui/assets/main.css` (config/styles)

**Analog:** itself — the existing 5-line reset is the foundation. Append all Phase 2 styles.

**Existing code to preserve (lines 1–8 of `assets/main.css`):**
```css
*, *::before, *::after {
    box-sizing: border-box;
}

body {
    margin: 0;
}
```

**Target pattern — append below the existing reset:**
```css
/* Source: 02-UI-SPEC.md CSS Custom Properties section + Layout section + Board Grid section */

:root {
  /* Layout */
  --board-max: 360px;
  --cell-size: 112px;
  --grid-gap: 4px;

  /* Color */
  --color-bg: #F9FAFB;
  --color-grid: #1E293B;
  --color-text: #1E293B;
  --color-x: #2563EB;
  --color-o: #E85D04;
  --color-hover: rgba(37, 99, 235, 0.10);
  --color-overlay-bg: rgba(30, 41, 59, 0.92);
  --color-overlay-text: #FFFFFF;

  /* Typography */
  --font-size-body: 16px;
  --font-size-heading: 24px;
  --font-size-display: 40px;

  /* Spacing */
  --space-xs: 4px;
  --space-sm: 8px;
  --space-md: 16px;
  --space-lg: 24px;
  --space-xl: 32px;
  --space-2xl: 48px;
}

/* Responsive cell sizing at narrow viewports (UI-06, D-08) */
@media (max-width: 360px) {
  :root {
    --cell-size: calc((100vw - 2 * var(--space-md) - 2 * var(--grid-gap)) / 3);
  }
}
```

---

### `kiss-tictactoe-ui/Cargo.toml` (config)

**Analog:** itself — add one dependency line.

**Existing code (lines 1–9 of `Cargo.toml`):**
```toml
[package]
name = "kiss-tictactoe-ui"
version = "0.1.0"
edition = "2024"
license = "MIT"

[dependencies]
dioxus = { version = "0.7.10", features = ["desktop"] }
manganis = "0.7.10"
```

**Target pattern — add engine path dep:**
```toml
[dependencies]
dioxus = { version = "0.7.10", features = ["desktop"] }
manganis = "0.7.10"
kiss-tictactoe-engine = { path = "../kiss-tictactoe-engine" }
```

---

## Shared Patterns

### Dioxus Component Declaration
**Source:** `kiss-tictactoe-ui/src/main.rs` lines 9–15
**Apply to:** `main.rs` (App), `board.rs`, `cell.rs`, `game_over_overlay.rs`
```rust
// Every Dioxus component uses this exact signature shape:
#[component]
pub fn ComponentName(prop: Type, ...) -> Element {
    rsx! {
        // ...
    }
}
// Exception: App in main.rs is not pub (crate-private entry point).
```

### Signal Borrow Safety
**Source:** 02-RESEARCH.md Pitfall 1 (lines 489–506)
**Apply to:** `board.rs` (anywhere `game` signal is read before written)
```rust
// WRONG — panics if guard is held open while .write() is called
let status = game.read().status();  // guard NOT dropped if status is not Copy
game.write().make_move(pos);        // panic: already borrowed

// CORRECT — Copy types drop the guard immediately after the semicolon
let status = game.read().status();  // GameStatus is Copy; guard drops here
game.write().make_move(pos);        // fine

// CORRECT — non-Copy types: scope the read in a block
let board = { game.read().board().clone() };  // clone exits scope; guard drops
```

### Engine Import Convention
**Source:** 02-CONTEXT.md code context section
**Apply to:** `main.rs`, `board.rs`, `cell.rs`, `game_over_overlay.rs`
```rust
// Engine crate name in Cargo.toml is "kiss-tictactoe-engine" (hyphenated).
// Rust converts hyphens to underscores for the `use` path:
use kiss_tictactoe_engine::{Game, GameStatus, Player, Position, MoveResult};
```

### CSS Custom Property Usage
**Source:** 02-UI-SPEC.md CSS Custom Properties section
**Apply to:** `assets/main.css` — every measurement and color value
```css
/* No magic numbers anywhere in Phase 2 CSS. Always use a token: */
.some-element {
  color: var(--color-x);
  padding: var(--space-md);
  font-size: var(--font-size-body);
}
```

### Rust Edition 2024 Conventions
**Source:** `kiss-tictactoe-ui/Cargo.toml` line 4 + `CLAUDE.md`
**Apply to:** All `.rs` files
```rust
// Both crates use edition = "2024" (Rust 1.85+).
// This enables async closures and improved lifetime capture for RPIT.
// No edition-specific pitfalls for synchronous game logic, but be aware:
// - use `use<'a>` syntax for explicit lifetime capture in RPIT if needed
// - `unsafe extern` blocks use new syntax if unsafe FFI is added (not needed here)
```

---

## No Analog Found

Files where the codebase has no close match. Planner should use RESEARCH.md patterns as the authoritative source for these.

| File | Role | Data Flow | Reason |
|------|------|-----------|--------|
| `kiss-tictactoe-engine/src/types.rs` | model | n/a | No existing types module; project is pre-engine |
| `kiss-tictactoe-engine/src/game.rs` | service | CRUD | No existing service/struct module in project |
| `kiss-tictactoe-engine/src/detection.rs` | utility | transform | No existing utility module in project |
| `kiss-tictactoe-engine/src/ai.rs` | utility | transform | No existing algorithmic code in project |

For all four engine files, the RESEARCH.md code examples (lines 209–425 and 562–668) are the authoritative patterns — they are derived from locked decisions (D-01 through D-04), official Dioxus docs, and verified crate versions.

---

## Metadata

**Analog search scope:** `kiss-tictactoe-engine/src/`, `kiss-tictactoe-ui/src/`, `kiss-tictactoe-ui/assets/`
**Files scanned:** 6 (all non-generated source files in the Phase 1 scaffold)
**Pattern extraction date:** 2026-10-14

**Why so few analogs:** This is a greenfield project at end of Phase 1. The scaffold contains exactly: one Dioxus component (`main.rs`), one empty engine lib (`lib.rs`), one CSS reset (`main.css`), and three Cargo manifests. All engine logic and all UI components beyond `App` are net-new for Phase 2. RESEARCH.md code examples serve as the analog for engine files; `main.rs` is the pattern source for all UI component files.
