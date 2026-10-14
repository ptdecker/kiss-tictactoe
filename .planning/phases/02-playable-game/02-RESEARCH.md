# Phase 2: Playable Game - Research

**Researched:** 2026-10-14
**Domain:** Rust game engine (minimax AI) + Dioxus 0.7 desktop UI
**Confidence:** HIGH

---

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions

- **D-01:** Engine exposes a `Game` struct with methods: `Game::new()`, `game.make_move(pos)`, `game.status()`, `game.board()`. Types: `Game`, `Board`, `GameStatus`, `Player`, `MoveResult`, `MoveError`.
- **D-02:** Board positions use `Position { row: u8, col: u8 }` — named fields, not flat index or tuple.
- **D-03:** `make_move` returns `Result<MoveResult, MoveError>`. `MoveError` covers `CellOccupied` and `InvalidPosition`.
- **D-04:** Engine moves automatically inside `make_move` — one call places player mark and engine responds. No separate `engine_move()` call.
- **D-05:** Game outcome shown as a board overlay/banner. Board goes non-interactive; overlay appears on top.
- **D-06:** Outcome text: "You win!" / "Engine wins!" / "Draw!" — user-centric language.
- **D-07:** Phase 2 overlay includes a "Play again" button that resets to `Game::new()`. Phase 3 supersedes this.
- **D-08:** Fixed max-size board (~360px), centered flexbox. Below max-size it fills viewport width.
- **D-09:** Single centered column layout: title → board. Phase 3 appends bubble + scoreboard + controls.
- **D-10:** CSS custom properties (`--cell-size`, `--board-max`, `--color-x`, `--color-o`, `--color-bg`, etc.) on `:root`. No magic numbers.
- **D-11:** Clean minimal cell style: borders define the grid via CSS Grid gap technique; no cell background fill. X and O in cell center. Hover states on empty cells.

### Claude's Discretion

- Player assignment: user is X (goes first), engine is O. Hardcoded for Phase 2.
- Board cells: 3×3 CSS Grid (not flexbox rows).
- Move history: Claude decides whether `Game` stores `Vec<Position>` internally. Store it — cheap, useful for Phase 3 review.
- Minimax: alpha-beta pruning acceptable; full traversal also fine at 3×3.

### Deferred Ideas (OUT OF SCOPE)

None — discussion stayed within phase scope.
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ENGI-01 | Game state tracks board contents, current player, and move history | `Game` struct with `Board` (9-cell array), `Player` enum, `Vec<Position>` history; all owned values, no refs across call boundaries |
| ENGI-02 | Engine detects win (any row/col/diagonal) and draw (board full, no winner) after every move | 8 win lines enumerable at compile time; detect on every `make_move` call; `GameStatus` enum with `InProgress`, `Win(Player)`, `Draw` variants |
| ENGI-03 | Engine exposes clean public API with zero Dioxus dependency | `pub` items only in engine; `cargo tree -p kiss-tictactoe-engine \| grep dioxus` must return nothing |
| ENGI-04 | Engine AI uses minimax and plays optimally (always draws against perfect play) | 3×3 game tree is tiny (max 9! leaves); alpha-beta optional but clean; validated by property tests over all reachable board states |
| UI-01 | User can click any empty cell; occupied and invalid cells are non-interactive | `Cell` component with `interactive: bool` prop; CSS `pointer-events: none` on occupied/frozen cells; click handler only fires for interactive cells |
| UI-06 | UI layout is responsive from phone width (320px) upward using plain CSS | Flexbox column layout; `min(var(--board-max), calc(100vw - 2 * var(--space-md)))` for board width; `@media (max-width: 360px)` scales cell size |
</phase_requirements>

---

## Summary

Phase 2 builds from a scaffold (Phase 1 deliverable: `App` component with CSS asset wiring, empty engine lib) to a fully playable tic-tac-toe game. The work splits cleanly into three verticals: (1) implement the engine crate — `Game` struct, win/draw detection, minimax AI; (2) build the Dioxus UI components — `Board`, `Cell`, `GameOverOverlay`; (3) wire them together — add engine as a dependency, wire `Signal<Game>` through the component tree, verify engine purity gate.

The Dioxus 0.7 API is stable and well-documented. Signals are the reactive primitive; `use_signal(|| T)` creates one in a component, signals are `Copy` so they move freely into closures, and `.write()` triggers a re-render. The most consequential runtime pitfall is holding a `.read()` guard while calling `.write()` on the same signal — this panics at runtime. The pattern to avoid it is well-understood: scope the read in a block, clone the value out, then write.

The engine layer is pure Rust with no dependencies — the decision to make engine moves happen inside `make_move` (D-04) means the UI only calls one function per player move and re-reads the board. Minimax on a 3×3 board (max 362,880 leaf nodes) is instantaneous on any modern hardware; no loading state or animation is needed.

**Primary recommendation:** Implement as three sequential thin slices — engine types and tests first, then UI components with mock data, then wire and verify the purity gate. This satisfies MVP vertical-slice mode and gives the planner natural task boundaries.

---

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Game state (board, current player, move history) | Engine (lib crate) | — | Pure Rust value types; must remain platform-agnostic per ENGI-03 |
| Win/draw detection | Engine (lib crate) | — | Logic belongs where state lives; no UI concern |
| Minimax AI decision | Engine (lib crate) | — | Computation-only; zero rendering concern |
| Cell click → game update | UI (Dioxus component) | Engine API | UI handles the event, delegates to engine via `make_move` |
| Reactive re-render on state change | UI (Dioxus signals) | — | Dioxus `Signal<Game>` triggers re-render on `.write()` |
| Game-over display / overlay | UI (Dioxus component) | Engine `GameStatus` | Engine provides status enum; UI renders the overlay |
| CSS layout, responsiveness | UI (CSS in `assets/main.css`) | — | Presentation-only; no engine concern |
| "Play again" reset | UI (Dioxus component) | Engine `Game::new()` | UI owns the signal; resets by calling `Game::new()` |

---

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `dioxus` | **0.7.10** | Desktop UI framework, signals, RSX | Pinned in CLAUDE.md; `features = ["desktop"]` for webview renderer (Wry/Tao) |
| `manganis` | **0.7.10** | Compile-time CSS asset bundling via `asset!()` | Already wired in Phase 1; required for CSS to survive `dx build` on all platforms |
| `kiss-tictactoe-engine` | workspace | Game engine crate | Workspace sibling; path dep, zero external packaging |

### Dev Dependencies (engine crate only)

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `proptest` | **1.12.0** | Property-based testing | Win detection correctness, AI move legality for all reachable board states |
| `rstest` | **0.27.0** | Parameterized test cases | Known board positions: forced wins, draws, blocking moves |

### No New Runtime Dependencies

Phase 2 adds zero new runtime crates beyond what Phase 1 established. The engine has no dependencies. The UI depends only on `dioxus` and `manganis`, both already present.

**Installation (engine crate dev deps):**
```toml
# kiss-tictactoe-engine/Cargo.toml
[dev-dependencies]
proptest = "1.12.0"
rstest = "0.27.0"
```

**Wire engine into UI:**
```toml
# kiss-tictactoe-ui/Cargo.toml
[dependencies]
dioxus = { version = "0.7.10", features = ["desktop"] }
manganis = "0.7.10"
kiss-tictactoe-engine = { path = "../kiss-tictactoe-engine" }
```

**Version verification:**
```
cargo search proptest  →  proptest = "1.12.0"  [VERIFIED: crates.io registry]
cargo search rstest    →  rstest = "0.27.0"    [VERIFIED: crates.io registry]
dx --version           →  dioxus 0.7.10         [VERIFIED: local install]
```

---

## Package Legitimacy Audit

> slopcheck could not be installed in this session (system pip protection blocked `--break-system-packages`). All packages below are tagged `[ASSUMED]` per the graceful-degradation protocol. The planner must add a `checkpoint:human-verify` task before each install.

| Package | Registry | Age | Downloads | Source Repo | slopcheck | Disposition |
|---------|----------|-----|-----------|-------------|-----------|-------------|
| `proptest` | crates.io | ~8 yrs (first seen in Debian 1.0.0-3, 2023; derived derivation 2017) | ~26.7M total | github.com/proptest-rs/proptest | unavailable | [ASSUMED] — planner adds verify checkpoint |
| `rstest` | crates.io | ~5 yrs (Ecosyste.ms) | ~10.9M total | github.com/la10736/rstest | unavailable | [ASSUMED] — planner adds verify checkpoint |
| `kiss-tictactoe-engine` | workspace-local | this project | n/a | workspace sibling | n/a | Approved — not an external package |

**Packages removed due to slopcheck [SLOP] verdict:** none

**Packages flagged as suspicious [SUS]:** none

*slopcheck was unavailable at research time. All external packages above are tagged `[ASSUMED]`. The planner must gate each install behind a `checkpoint:human-verify` task. Both `proptest` and `rstest` are widely used, long-established crates in the Rust ecosystem — risk is low, but verification is mandatory per the protocol.*

---

## Architecture Patterns

### System Architecture Diagram

```
Human click on Cell
        │
        ▼
Cell component (Dioxus)
  ├─ checks `interactive` prop
  └─ fires `on_click: EventHandler<Position>`
        │
        ▼
Board component (holds Signal<Game>)
  └─ game.write().make_move(pos)
        │ returns Result<MoveResult, MoveError>
        ▼
kiss-tictactoe-engine
  ├─ validates position
  ├─ places player mark (X)
  ├─ detects win/draw
  ├─ runs minimax for engine move (O)  ─────► places O mark
  ├─ detects win/draw again
  └─ returns MoveResult (or MoveError)
        │
        ▼ (Signal write triggers re-render)
App (owns Signal<Game> via use_signal)
  └─ re-renders Board
        │
        ├─ Board reads game.read().board() → renders 9 Cells
        │   ├─ Cell state: empty|X|O + interactive|frozen
        │   └─ CSS class driven: .cell / .cell--x / .cell--o / .cell--frozen
        │
        └─ if game.read().status() != InProgress
              └─ GameOverOverlay (position: absolute over board)
                    ├─ outcome text: "You win!" | "Engine wins!" | "Draw!"
                    └─ "Play again" button → game.set(Game::new())
```

### Recommended Project Structure

```
kiss-tictactoe-engine/
├── src/
│   ├── lib.rs          # pub mod declarations + crate doc comment
│   ├── types.rs        # Position, Player, Board, GameStatus, MoveResult, MoveError
│   ├── game.rs         # Game struct impl (new, make_move, status, board, history)
│   ├── detection.rs    # win/draw detection logic
│   └── ai.rs           # minimax (+ optional alpha-beta)
│   # Tests inline in each module or in tests/ subdirectory

kiss-tictactoe-ui/
├── src/
│   ├── main.rs                         # dioxus::launch(App); App component owns Signal<Game>
│   └── components/
│       ├── mod.rs                      # pub mod board; pub mod cell; pub mod game_over_overlay;
│       ├── board.rs                    # Board component
│       ├── cell.rs                     # Cell component
│       └── game_over_overlay.rs        # GameOverOverlay component
├── assets/
│   └── main.css                        # :root tokens + all CSS
└── Dioxus.toml                         # existing, no changes needed
```

### Pattern 1: Engine Public API

**What:** Pure Rust struct with associated methods, zero platform deps.

**When to use:** Always — this is the locked API shape from D-01 through D-03.

```rust
// Source: CONTEXT.md decisions D-01, D-02, D-03, D-04

/// Board position using named fields (D-02)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player { X, O }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameStatus {
    InProgress,
    Win(Player),
    Draw,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveResult {
    Continuing,
    GameOver(GameStatus),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveError {
    CellOccupied,
    InvalidPosition,
    GameAlreadyOver,
}

pub struct Game { /* private fields */ }

impl Game {
    pub fn new() -> Self { /* ... */ }
    /// Places player mark, then engine moves if game not over. (D-04)
    pub fn make_move(&mut self, pos: Position) -> Result<MoveResult, MoveError> { /* ... */ }
    pub fn status(&self) -> GameStatus { /* ... */ }
    pub fn board(&self) -> &Board { /* ... */ }
    pub fn history(&self) -> &[Position] { /* ... */ }
}

/// Board is a 3×3 array of Option<Player>
pub type Board = [[Option<Player>; 3]; 3];
```

### Pattern 2: Signal-Owned Game State in Dioxus 0.7

**What:** `App` owns `Signal<Game>`, passes it to child components. Children read board state, fire events back up.

**When to use:** For this app — single source of truth, clean re-render on every game state change.

```rust
// Source: Dioxus 0.7 official docs — dioxuslabs.com/learn/0.7/essentials/basics/signals

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

### Pattern 3: Passing Signal Down and Writing from Child

**What:** Pass `Signal<Game>` as a prop. Child calls `.write().make_move(pos)` in event handler.

**When to use:** When a child needs to mutate parent state. Signal is `Copy` so it moves into closures freely.

```rust
// Source: Dioxus 0.7 docs — dioxuslabs.com/learn/0.7/essentials/basics/signals

#[component]
fn Board(game: Signal<Game>) -> Element {
    let board = game.read().board().clone();  // clone to avoid holding guard
    let status = game.read().status();

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

### Pattern 4: Dynamic CSS Classes in RSX

**What:** Multiple `class:` attributes on the same element. Dioxus joins them automatically.

**When to use:** Cell state classes (`.cell--x`, `.cell--o`, `.cell--frozen`).

```rust
// Source: Dioxus 0.7 docs — dioxuslabs.com/learn/0.7/essentials/ui/attributes

#[component]
fn Cell(
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

### Pattern 5: Minimax with Alpha-Beta Pruning

**What:** Recursive minimax with alpha-beta bounds. Engine is O (minimizer); player is X (maximizer).

**When to use:** Always — this is the required AI algorithm (ENGI-04). Full traversal also acceptable for 3×3.

```rust
// Source: [ASSUMED] — standard minimax algorithm, well-known CS algorithm

fn minimax(board: &Board, depth: i32, is_maximizing: bool, alpha: i32, beta: i32) -> i32 {
    match detect_winner(board) {
        Some(GameStatus::Win(Player::X)) => return 10 - depth,  // sooner win is better
        Some(GameStatus::Win(Player::O)) => return depth - 10,
        Some(GameStatus::Draw) => return 0,
        _ => {}
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
    best_pos.expect("no moves available")
}
```

### Pattern 6: Responsive CSS Board

**What:** Board fills viewport minus padding, max 360px; cells scale below 360px via media query.

**When to use:** Always — this is the approved responsive layout from UI-06 and D-08.

```css
/* Source: 02-UI-SPEC.md — approved design contract */

:root {
  --board-max: 360px;
  --cell-size: 112px;
  --grid-gap: 4px;
  /* ... other tokens ... */
}

.board-wrapper {
  position: relative;
  width: min(var(--board-max), calc(100vw - 2 * var(--space-md)));
}

.board {
  display: grid;
  grid-template-columns: repeat(3, var(--cell-size));
  grid-template-rows: repeat(3, var(--cell-size));
  gap: var(--grid-gap);
  background-color: var(--color-grid); /* reveals as grid lines */
}

/* Responsive: scale cells below 360px viewport (UI-06, 320px minimum) */
@media (max-width: 360px) {
  :root {
    --cell-size: calc((100vw - 2 * var(--space-md) - 2 * var(--grid-gap)) / 3);
  }
}
```

### Anti-Patterns to Avoid

- **Reading then writing the same signal in one scope:** `game.read().something(); game.write().mutate();` panics at runtime. Drop the read guard first (use a block or `.cloned()`), then write.
- **Holding `.read()` guard across an `await` point:** Guards stay open over await; other work that writes to the same signal will panic. Clone needed values before awaiting.
- **Calling `game.write()` in a Dioxus render function body (outside event handler):** This causes infinite re-render loops. Mutations only in event handlers or async tasks.
- **Adding any `dioxus` or platform import to `kiss-tictactoe-engine/Cargo.toml`:** Hard gate — breaks ENGI-03 and INFRA-02. The purity check `cargo tree -p kiss-tictactoe-engine | grep dioxus` is a Phase 2 success criterion.
- **Flat index (0–8) for board positions:** Locked decision D-02 mandates `Position { row, col }`.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Property-based AI correctness testing | Custom board generator / manual test matrix | `proptest` with `Strategy` for `Board` | Proptest shrinks counterexamples automatically; covers all reachable board states cheaply |
| Parameterized win/draw test cases | Repetitive `#[test]` copy-paste | `rstest` `#[rstest]` with `#[case]` | Eliminates boilerplate; failure messages name the failing case |
| CSS grid rendering | Manual border CSS on individual cells | CSS Grid gap + background color technique | Single approach, no per-cell border math, handles gap as "grid line" cleanly |
| Signal reactivity system | Custom `Arc<Mutex<T>>` + manual re-render | Dioxus `use_signal` | Signals are the standard Dioxus 0.7 primitive; custom solutions break the render lifecycle |

**Key insight:** In Rust desktop game UIs with Dioxus, the reactive primitive (`Signal`) does the hard lifting — fighting it with manual state management creates hard-to-debug render ordering issues.

---

## Common Pitfalls

### Pitfall 1: Runtime Borrow Panic (Signal Read-Write Overlap)

**What goes wrong:** Code reads from `game` signal and then tries to write to it in the same scope. Panics at runtime: "already borrowed."

**Why it happens:** `Signal<T>` uses `RefCell`-like runtime borrow checking. `.read()` returns a `Ref` guard; holding it while calling `.write()` violates the borrow rule.

**How to avoid:** Clone the values you need from `.read()` before calling `.write()`, or scope the read in a block so the guard drops first.

```rust
// WRONG — panics
let status = game.read().status();  // guard still held
game.write().make_move(pos);        // panic: already borrowed

// CORRECT — clone out, then write
let status = game.read().status();  // guard drops at end of statement (Copy type)
// For non-Copy: { let s = game.read().something().clone(); /* use s */ }
game.write().make_move(pos);        // fine
```

**Warning signs:** "panicked at 'already borrowed'" in the console when clicking a cell.

---

### Pitfall 2: Engine Purity Violation

**What goes wrong:** Accidentally adding a Dioxus type or transitive dependency to the engine crate. `cargo tree -p kiss-tictactoe-engine | grep dioxus` returns output.

**Why it happens:** Adding a helper crate to engine `[dependencies]` that itself depends on Dioxus (e.g., a serialization crate with Dioxus feature flags enabled).

**How to avoid:** Engine crate `[dependencies]` section must remain empty. All dev-dependencies (`proptest`, `rstest`) are `[dev-dependencies]` only. Run the purity gate check at the end of engine implementation.

**Warning signs:** `cargo tree -p kiss-tictactoe-engine | grep dioxus` returns any output.

---

### Pitfall 3: Cell Interaction After Game Over

**What goes wrong:** Player can still click cells after a win/draw because the overlay only covers visually, but pointer events still reach the underlying cells.

**Why it happens:** CSS `position: absolute` overlay intercepts clicks on the overlay area, but if cells have their own `onclick` and the overlay is smaller or has gaps, clicks leak through.

**How to avoid:** Apply `pointer-events: none` via the `.cell--frozen` class to ALL cells when `status != InProgress`, regardless of the overlay. Belt-and-suspenders: both the CSS class and the `interactive: false` prop guard in Rust.

**Warning signs:** `make_move` called after game over returns `MoveError::GameAlreadyOver` (which is a `MoveError` variant to include).

---

### Pitfall 4: Minimax Returns Wrong Winner for Depth-Scored Boards

**What goes wrong:** AI doesn't prefer faster wins — it finds any winning path rather than the shortest one, leading to sub-optimal-looking play (e.g., engine misses a 1-move win and plays arbitrarily).

**Why it happens:** Flat scoring (`+1`/`-1`/`0`) doesn't penalize depth; all wins look equal.

**How to avoid:** Use depth-adjusted scores: `10 - depth` for a maximizer win, `depth - 10` for a minimizer win. This makes earlier wins score higher.

**Warning signs:** Engine has a winning move available but plays elsewhere; `cargo test` passes for "always optimal" but humans notice the engine "wastes moves."

---

### Pitfall 5: CSS Grid Cell Size vs. 320px Minimum Viewport

**What goes wrong:** At 320px viewport, fixed `--cell-size: 112px` produces a board of 344px (3 × 112 + 2 × 4), wider than the 288px available (320 - 32 padding). Cells overflow or scroll.

**Why it happens:** `--cell-size` is fixed in `:root`; without the responsive media query the grid overflows.

**How to avoid:** The responsive media query (Pattern 6 above) scales `--cell-size` dynamically below 360px. This is already specified in the approved UI-SPEC. Ensure the media query is the first thing verified in layout testing.

**Warning signs:** Horizontal scrollbar appears at 320px; cells clip at screen edge.

---

## Code Examples

### Win Detection (8 Lines)

```rust
// Source: [ASSUMED] — standard 3×3 tic-tac-toe win detection

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

pub fn detect_winner(board: &Board) -> Option<GameStatus> {
    for line in &WINNING_LINES {
        let [a, b, c] = *line;
        if let (Some(p1), Some(p2), Some(p3)) =
            (board[a.0][a.1], board[b.0][b.1], board[c.0][c.1])
        {
            if p1 == p2 && p2 == p3 {
                return Some(GameStatus::Win(p1));
            }
        }
    }
    if board.iter().all(|row| row.iter().all(|cell| cell.is_some())) {
        return Some(GameStatus::Draw);
    }
    None
}
```

### GameOverOverlay Component

```rust
// Source: CONTEXT.md D-05, D-06, D-07 + Dioxus 0.7 EventHandler pattern

#[component]
fn GameOverOverlay(status: GameStatus, on_play_again: EventHandler<()>) -> Element {
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

### proptest for AI Correctness (ENGI-04)

```rust
// Source: [ASSUMED] — proptest 1.12.0 standard API

use proptest::prelude::*;

proptest! {
    #[test]
    fn engine_never_makes_illegal_move(board in valid_board_strategy()) {
        let game = Game::from_board(board).unwrap();
        if game.status() == GameStatus::InProgress {
            let result = game.engine_best_move();
            prop_assert!(result.is_some());
            let pos = result.unwrap();
            prop_assert!(game.board()[pos.row as usize][pos.col as usize].is_none());
        }
    }

    #[test]
    fn engine_never_loses(board in reachable_board_strategy()) {
        // Play out from any reachable state; engine should always achieve >= draw
        // ...
    }
}
```

### rstest for Known Board States (ENGI-02)

```rust
// Source: [ASSUMED] — rstest 0.27.0 standard API

use rstest::rstest;

#[rstest]
#[case("X wins row 0", [[Some(X),Some(X),Some(X)],[None,None,None],[None,None,None]], GameStatus::Win(Player::X))]
#[case("O wins col 1", [[None,Some(O),None],[None,Some(O),None],[None,Some(O),None]], GameStatus::Win(Player::O))]
#[case("draw", [[Some(X),Some(O),Some(X)],[Some(X),Some(X),Some(O)],[Some(O),Some(X),Some(O)]], GameStatus::Draw)]
#[case("in progress", [[None,None,None],[None,None,None],[None,None,None]], GameStatus::InProgress)]
fn win_detection(#[case] _name: &str, #[case] board: Board, #[case] expected: GameStatus) {
    assert_eq!(detect_status(&board), expected);
}
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| `dioxus-desktop` as separate crate | `dioxus` with `features = ["desktop"]` | Dioxus 0.4+ | Desktop feature on main crate is the canonical approach; separate crate is redundant |
| Manual CSS borders on each cell | CSS Grid gap + container background color | CSS Grid era | Cleaner, fewer CSS rules, no per-cell border math |
| Pass `(row, col): (u8, u8)` for positions | `Position { row: u8, col: u8 }` struct | Rust idiom | Self-documenting, pattern-matchable, extendable |
| `Rc<RefCell<GameState>>` passed as prop | `Signal<T>` (Copy + reactive) | Dioxus 0.4+ | Signals are the Dioxus 0.7 standard; no clone overhead, automatic re-render |

**Deprecated/outdated:**
- `dioxus-desktop` standalone crate: redundant with `dioxus` + `features = ["desktop"]`; CLAUDE.md explicitly rejects it
- `quickcheck`: superseded by `proptest` for structured inputs; proptest is actively maintained at 1.12.0

---

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | Alpha-beta pruning implementation (Pattern 5) | Code Examples | If pruning logic is incorrect, engine may not find optimal move; property tests catch this |
| A2 | proptest `Strategy` for `Board` type works via standard derive | Code Examples | `Board = [[Option<Player>; 3]; 3]` — may require custom `Arbitrary` impl; low risk, well-known pattern |
| A3 | `Game::from_board()` constructor (used in proptest example) | Code Examples | Not in locked API; Claude can define this as a test-only constructor; internal to engine tests |
| A4 | rstest `#[case]` syntax handles array literals for Board | Code Examples | rstest 0.27.0 should handle this; verify at implementation time |
| A5 | proptest download count (~26.7M) and age (~8 yrs) | Package Legitimacy | Third-party tracker data; not from crates.io directly — legitimacy indicator only |
| A6 | rstest download count (~10.9M) and age (~5 yrs) | Package Legitimacy | Third-party tracker data; not from crates.io directly — legitimacy indicator only |

**All non-assumed claims:** Dioxus 0.7 API patterns (signals, EventHandler, conditional RSX, context) are VERIFIED via official Dioxus 0.7 documentation. Package versions (proptest 1.12.0, rstest 0.27.0) are VERIFIED via `cargo search`. Environment (rustc 1.93.1, dx 0.7.10) is VERIFIED via local tools.

---

## Open Questions

1. **`MoveError::GameAlreadyOver` variant**
   - What we know: D-03 specifies `CellOccupied` and `InvalidPosition`. The UI prevents invalid moves, but a third variant for "game already over" adds a clean defensive guard.
   - What's unclear: CONTEXT.md doesn't enumerate all `MoveError` variants, only two.
   - Recommendation: Add `GameAlreadyOver` as a third `MoveError` variant — it costs nothing and handles the case where a user bypasses UI guards.

2. **`Board` type: array vs. flat array**
   - What we know: D-01 mentions `Board` as a type; CONTEXT.md doesn't specify the internal representation.
   - What's unclear: `[[Option<Player>; 3]; 3]` (2D) vs. `[Option<Player>; 9]` (1D) are both common.
   - Recommendation: Use `[[Option<Player>; 3]; 3]` — indexes naturally with `board[row][col]`, matches `Position { row, col }` semantics from D-02.

3. **`key` attribute on iterated Cells**
   - What we know: Dioxus 0.7 uses `key` for list reconciliation, similar to React.
   - What's unclear: Exact `key` syntax in 0.7 RSX not verified against official docs in this session.
   - Recommendation: Use `key: "{row}-{col}"` string format; verify compiles without warning during implementation.

---

## Environment Availability

| Dependency | Required By | Available | Version | Fallback |
|------------|------------|-----------|---------|----------|
| `rustc` / `cargo` | Engine + UI compilation | ✓ | 1.93.1 | — |
| `dx` (dioxus-cli) | UI hot-reload, build, bundle | ✓ | 0.7.10 | `cargo run -p kiss-tictactoe-ui` for non-hot-reload |
| `cargo test` | Engine unit/property tests | ✓ | 1.93.1 | — |

**Missing dependencies with no fallback:** none

**Missing dependencies with fallback:** none — all required tools are present.

---

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in `#[test]` + `proptest` 1.12.0 + `rstest` 0.27.0 |
| Config file | none — standard `cargo test` discovery |
| Quick run command | `cargo test -p kiss-tictactoe-engine` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ENGI-01 | Game state tracks board, player, history | unit | `cargo test -p kiss-tictactoe-engine game::tests` | ❌ Wave 0 |
| ENGI-02 | Win detection for all 8 lines + draw | unit (rstest parameterized) | `cargo test -p kiss-tictactoe-engine detection::tests` | ❌ Wave 0 |
| ENGI-04 | Minimax plays optimally (never loses) | property (proptest) | `cargo test -p kiss-tictactoe-engine ai::tests` | ❌ Wave 0 |
| ENGI-03 | Engine purity (no Dioxus dep) | smoke (cargo tree) | `cargo tree -p kiss-tictactoe-engine \| grep dioxus` returns empty | ❌ Wave 0 |
| UI-01 | Occupied cells non-interactive | manual smoke | `dx serve --platform desktop` + click test | — manual |
| UI-06 | Layout at 320px | manual smoke | Resize webview window to 320px | — manual |

### Sampling Rate

- **Per task commit:** `cargo test -p kiss-tictactoe-engine`
- **Per wave merge:** `cargo test --workspace`
- **Phase gate:** `cargo test --workspace` + purity check + manual smoke test before `/gsd:verify-work`

### Wave 0 Gaps

- [ ] `kiss-tictactoe-engine/src/types.rs` — type definitions (not tests, but prerequisite for test files)
- [ ] `kiss-tictactoe-engine/src/detection.rs` — win detection + inline `#[cfg(test)]` module with rstest cases
- [ ] `kiss-tictactoe-engine/src/ai.rs` — minimax + inline `#[cfg(test)]` module with proptest suite
- [ ] `kiss-tictactoe-engine/src/game.rs` — Game struct + inline `#[cfg(test)]` module

---

## Security Domain

### Applicable ASVS Categories (ASVS L1, desktop game context)

| ASVS Category | Applies | Standard Control |
|---------------|---------|-----------------|
| V2 Authentication | No | No users, no accounts; single-player local game |
| V3 Session Management | No | No sessions; game state lives in process memory only |
| V4 Access Control | No | No permissions model; no multi-user scenario |
| V5 Input Validation | Yes (minimal) | Engine validates all moves via `make_move` returning `Result<MoveResult, MoveError>`; UI prevents invalid clicks via `interactive: bool` |
| V6 Cryptography | No | No secrets, no tokens, no encrypted data |

### Known Threat Patterns for This Stack

| Pattern | STRIDE | Standard Mitigation |
|---------|--------|---------------------|
| Invalid board position (row/col out of bounds) | Tampering | `MoveError::InvalidPosition` returned from engine; UI enforces `interactive: bool` |
| Move on occupied cell | Tampering | `MoveError::CellOccupied` returned; cell has `pointer-events: none` via CSS |
| Move after game over | Tampering | `MoveError::GameAlreadyOver` (recommended variant); overlay covers board |

**Threat model summary:** This is a local desktop game with no network, no persistence, no authentication, and no sensitive data. The threat surface is effectively zero for external attackers. Input validation concerns are limited to ensuring the engine API returns errors rather than panicking on invalid inputs — this is handled by the `Result<MoveResult, MoveError>` API design locked in D-03.

---

## Sources

### Primary (HIGH confidence)

- Dioxus 0.7 official docs (signals) — https://dioxuslabs.com/learn/0.7/essentials/basics/signals/llms.txt — signals API, .read()/.write()/.set(), borrow rules
- Dioxus 0.7 official docs (event handlers) — https://dioxuslabs.com/learn/0.7/essentials/basics/event_handlers/llms.txt — EventHandler<T>, .call(), Callback<I,O>
- Dioxus 0.7 official docs (context) — https://dioxuslabs.com/learn/0.7/essentials/basics/context/llms.txt — provide_context, use_context, GlobalSignal
- Dioxus 0.7 official docs (attributes/conditional classes) — https://dioxuslabs.com/learn/0.7/essentials/ui/attributes/llms.txt — multiple class: attributes, conditional if
- Dioxus 0.7 official docs (conditional rendering) — https://dioxuslabs.com/learn/0.7/essentials/ui/conditional/llms.txt — if/else in rsx! body
- Dioxus 0.7 official docs (state hoisting) — https://dioxuslabs.com/learn/0.7/essentials/basics/hoisting — ReadSignal props, one-way data flow
- `cargo search proptest` — version 1.12.0 confirmed on crates.io
- `cargo search rstest` — version 0.27.0 confirmed on crates.io
- `dx --version` — dioxus 0.7.10 confirmed installed
- `rustc --version` — 1.93.1 confirmed installed
- 02-CONTEXT.md — locked decisions D-01 through D-11
- 02-UI-SPEC.md — approved design contract (CSS tokens, layout, component inventory, interaction contract)
- CLAUDE.md — tech stack, pinned versions, workspace structure, constraints

### Secondary (MEDIUM confidence)

- Dioxus 0.7 docs WebSearch results — https://dioxuslabs.com/learn/0.7/essentials/state — confirmed signal patterns, reactive props
- docs.rs/dioxus-hooks 0.7.10 — https://docs.rs/crate/dioxus-hooks/0.7.10/source/docs/moving_state_around.md — bundled app state pattern with use_context_provider

### Tertiary (LOW confidence / ASSUMED)

- Minimax alpha-beta implementation (Pattern 5) — standard CS algorithm, training knowledge, not verified against a specific authoritative source for this exact Rust code shape
- proptest Board strategy pattern (proptest test example) — standard proptest usage, training knowledge

---

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH — versions verified via cargo search and local install
- Architecture: HIGH — locked by CONTEXT.md decisions, Dioxus patterns from official docs
- Pitfalls: HIGH for Dioxus signal pitfalls (official docs), MEDIUM for minimax scoring (standard algorithm, not Dioxus-specific)
- Engine API shape: HIGH — locked by D-01 through D-04
- CSS patterns: HIGH — locked by approved UI-SPEC

**Research date:** 2026-10-14
**Valid until:** 2026-11-14 (Dioxus 0.7.x stable; 30-day window reasonable for stable library)
