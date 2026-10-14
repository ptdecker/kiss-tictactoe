# Phase 2: Playable Game - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-10-14
**Phase:** 02-playable-game
**Areas discussed:** Engine API shape, Game-over display, Responsive layout approach

---

## Engine API shape

### Q1: Top-level engine structure

| Option | Description | Selected |
|--------|-------------|----------|
| `Game` struct with methods | `Game::new()`, `game.make_move(pos)`, etc. Idiomatic Rust OOP-style, easy to doc | ✓ |
| Pure functions + opaque state | `new_game() -> GameState`, functional-style. Clunkier with Dioxus signals. | |
| You decide | Claude picks based on Dioxus 0.7 signals fit | |

**User's choice:** `Game` struct with methods

---

### Q2: Board position representation

| Option | Description | Selected |
|--------|-------------|----------|
| `Position` struct with row/col fields | `Position { row: u8, col: u8 }` — explicit, self-documenting, idiomatic | ✓ |
| 0–8 flat index (`usize`) | Simplest; `game.make_move(4)` for center | |
| `(row, col)` tuple | Concise but weakly typed | |

**User's choice:** `Position { row: u8, col: u8 }` struct

---

### Q3: Error handling for invalid moves

| Option | Description | Selected |
|--------|-------------|----------|
| `Result<MoveResult, MoveError>` | Explicit: `Err(MoveError::CellOccupied)`. API stays honest even if UI prevents invalid moves. | ✓ |
| `Option<MoveResult>` — None on invalid | Simpler; less information at error site | |
| `MoveResult::Invalid` variant | No `Result` wrapper; enum covers all cases | |

**User's choice:** `Result<MoveResult, MoveError>`

---

### Q4: Engine move timing

| Option | Description | Selected |
|--------|-------------|----------|
| Engine moves automatically inside `make_move` | One call: player moves + engine responds. Simplest UI code. | ✓ |
| Two separate calls: `make_move` + `engine_move` | More control; more UI orchestration required | |
| You decide | Claude minimizes UI complexity | |

**User's choice:** Engine moves automatically inside `make_move`

---

## Game-over display

### Q1: How to communicate outcome

| Option | Description | Selected |
|--------|-------------|----------|
| Status text line above/below board | Dedicated text label; simple and Phase 3 replaces it | |
| Board overlay / banner | Semi-transparent overlay on frozen board; visually prominent | ✓ |
| Nothing — board just freezes | Minimal; user might think app crashed | |

**User's choice:** Board overlay / banner

---

### Q2: New game access in Phase 2

| Option | Description | Selected |
|--------|-------------|----------|
| Banner with a minimal restart button | Overlay + "Play again" button; app stays testable in Phase 2 | ✓ |
| Banner only — no restart | Strict phase boundary; limits manual testing | |
| You decide | Claude picks for testability | |

**User's choice:** Banner with a minimal "Play again" button

---

### Q3: Outcome text phrasing

| Option | Description | Selected |
|--------|-------------|----------|
| User-centric: "You win!" / "Engine wins!" / "Draw!" | Speaks to the player; Phase 3 replaces with trash talk | ✓ |
| Mark-centric: "X wins!" / "O wins!" / "Draw!" | Neutral; game-standard | |
| You decide | Claude picks based on core value (personal, not abstract) | |

**User's choice:** User-centric phrasing ("You win!" / "Engine wins!" / "Draw!")

---

## Responsive layout approach

### Q1: Board size strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Fixed max-size, centered with flexbox | ~360px max-width, fills below. Simple; Phase 3 stacks vertically. | ✓ |
| Fluid vmin/vw-based cells | Board fills viewport proportionally. More adaptive but harder to compose with Phase 3. | |
| You decide | Claude picks for Phase 3 extensibility | |

**User's choice:** Fixed max-size board, centered with flexbox

---

### Q2: Page layout structure

| Option | Description | Selected |
|--------|-------------|----------|
| Single centered column — stacked vertically | Title → Board → (Phase 3 adds bubble + scoreboard + controls). Works at all widths. | ✓ |
| Two-column on wide screens | Board + sidebar. Phase 2 board would be lonely with empty sidebar. | |
| You decide | Claude picks for natural Phase 3 extension | |

**User's choice:** Single centered column, stacked vertically

---

### Q3: CSS custom properties

| Option | Description | Selected |
|--------|-------------|----------|
| Yes — CSS variables for core tokens | `--cell-size`, `--board-max`, `--color-x`, `--color-o`, etc. on `:root`. Theming-ready. | ✓ |
| No — direct values for now | Simpler short-term; likely inconsistencies in Phase 3 | |
| You decide | Claude picks per KISS principle | |

**User's choice:** CSS variables for core tokens

---

### Q4: Cell visual style

| Option | Description | Selected |
|--------|-------------|----------|
| Clean minimal — bordered grid, no fill | Borders only define cells; classic look; easy hover states | ✓ |
| Filled cells with contrast marks | Light background fill; modern card feel | |
| You decide | Claude picks cleaner option | |

**User's choice:** Clean minimal — bordered grid, no fill

---

## Claude's Discretion

- **Player assignment:** User is always X (goes first), engine is always O. Hardcoded in Phase 2; SETUP-01 (who goes first) is a v2 requirement.
- **Board grid implementation:** CSS Grid for the 3×3 cell layout.
- **Move history:** Whether `Game` stores move history — Claude decides based on ENGI-01 scope.
- **Minimax details:** Alpha-beta pruning acceptable; Claude picks the cleaner implementation.

## Deferred Ideas

None — discussion stayed within phase scope.
