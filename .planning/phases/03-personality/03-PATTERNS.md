# Phase 3: Personality - Pattern Map

**Mapped:** 2026-10-14
**Files analyzed:** 17 (7 new, 10 modified)
**Analogs found:** 17 / 17 (all in-repo; the codebase is small, so analogs are mostly role-match)

## File Classification

| New/Modified File | Role | Data Flow | Closest Analog | Match Quality |
|-------------------|------|-----------|----------------|---------------|
| `kiss-tictactoe-engine/src/events.rs` (new) | utility (classifier + threat detection) | transform | `kiss-tictactoe-engine/src/detection.rs` | exact (same line-table scan, same test style) |
| `kiss-tictactoe-engine/src/commentary.rs` (new) | service (PRNG + const banks) | transform | `kiss-tictactoe-engine/src/ai.rs` (pure module, `#[cfg(test)]` rstest/proptest) | role-match |
| `kiss-tictactoe-engine/src/ai.rs` (mod: add `move_quality`; fix TD-03/04) | service | transform | itself (`minimax`, `best_engine_move`) | exact |
| `kiss-tictactoe-engine/src/game.rs` (mod: `Turn`, `Game::play`; TD-02) | model | request-response | itself (`make_move`) | exact |
| `kiss-tictactoe-engine/src/types.rs` (mod: `TalkEvent`/`Turn` if placed here) | model | n/a | itself (enum + doc style) | exact |
| `kiss-tictactoe-engine/src/lib.rs` (mod: mods and re-exports) | config | n/a | itself | exact |
| `kiss-tictactoe-engine/tests/gameplay.rs` (mod: extend) | test | request-response | itself | exact |
| `kiss-tictactoe-ui/src/state.rs` (new: `Tally`, `BubbleState`) | model | CRUD (in-memory) | `components/game_over_overlay.rs` `outcome_text` (pure fn + in-file tests) | role-match |
| `kiss-tictactoe-ui/src/components/speech_bubble.rs` (new) | component | event-driven (signal read) | `components/game_over_overlay.rs` | role-match |
| `kiss-tictactoe-ui/src/components/scoreboard.rs` (new) | component | event-driven (signal read) | `components/game_over_overlay.rs` | role-match |
| `kiss-tictactoe-ui/src/components/controls.rs` (new) | component | event-driven (callbacks) | `components/game_over_overlay.rs` (button + `EventHandler<()>`) | exact |
| `kiss-tictactoe-ui/src/components/board.rs` (mod: TD-01, TD-06, use `play`, forward signals) | component | request-response | itself | exact |
| `kiss-tictactoe-ui/src/components/mod.rs` (mod) | config | n/a | itself | exact |
| `kiss-tictactoe-ui/src/main.rs` (mod: signals, layout, `start_new_game`, `clock_seed`) | component (root) | event-driven | itself (`use_signal(Game::new)`) | exact |
| `kiss-tictactoe-ui/src/components/game_over_overlay.rs` (mod: unchanged unless handler wiring) | component | event-driven | itself | exact |
| `kiss-tictactoe-ui/assets/main.css` (mod: TD-05 + new tokens/rules) | config | n/a | itself | exact |

Note: the UI crate is a binary (`main.rs`, no `lib.rs`), so `state.rs` needs `mod state;` in `main.rs`, mirroring `mod components;` at line 1.

## Pattern Assignments

### `kiss-tictactoe-engine/src/events.rs` (utility, transform)

**Analog:** `kiss-tictactoe-engine/src/detection.rs`

**Module header and imports** (detection.rs lines 1-3):
```rust
//! Win and draw detection for the 3×3 board.

use crate::types::{Board, GameStatus};
```

**Line table to reuse** (detection.rs lines 5-18). It is a private `const WINNING_LINES: [[(usize, usize); 3]; 8]`. Change it to `pub(crate) const` and import in `events.rs` via `use crate::detection::WINNING_LINES;`. Do not duplicate it.

**Core scan pattern to copy** (detection.rs lines 28-37), adapted for "two marks of `p` plus one `None`":
```rust
for line in &WINNING_LINES {
    let [a, b, c] = *line;
    match (board[a.0][a.1], board[b.0][b.1], board[c.0][c.1]) {
        (Some(p1), Some(p2), Some(p3)) if p1 == p2 && p2 == p3 => { return GameStatus::Win(p1); }
        _ => {}
    }
}
```
Threat version: count cells equal to `Some(p)` and cells equal to `None` per line; threat iff `marks == 2 && empties == 1`.

**Test style** (detection.rs lines 47-120): `#[cfg(test)] mod tests`, `use rstest::rstest;`, `use crate::types::Player::{O, X};`, `#[rstest] #[case(board, expected)]` with a boards-as-literal layout, and `#[values(...)]` for line x player cross-products. Use the same shape for threat detection and `classify` priority cases. `board_from(xs, os)` helper exists in `ai.rs` tests lines 113-122; copy it locally (it is test-private).

**Classification** itself: RESEARCH.md Pattern 1 `classify(over, user_threat, engine_threat, q) -> Option<TalkEvent>` (the priority is game over, engine threat, user threat, bad, good). No codebase analog; use the RESEARCH snippet.

**Docs:** crate has `#![warn(missing_docs)]` (lib.rs line 17); every `pub` item needs `///`.

---

### `kiss-tictactoe-engine/src/ai.rs` (service, transform) - add `move_quality`, fix TD-03/04

**Analog:** itself.

**Existing private helpers to reuse** (ai.rs lines 12-22 `empty_cells`, lines 28-73 `minimax`). `minimax(board, depth, is_maximizing, alpha, beta)`; X maximizes, O minimizes; terminal scores `10 - depth` / `depth - 10` / `0`.

**Root-call pattern to copy** (ai.rs `best_engine_move`, lines 85-96): clone board, place mark, call `minimax(&b, 0, <next-mover>, i32::MIN, i32::MAX)`:
```rust
let mut b = *board;
b[pos.row as usize][pos.col as usize] = Some(Player::O);
let score = minimax(&b, 0, true, i32::MIN, i32::MAX);
```
For `move_quality` mirror this with `Some(Player::X)` and `is_maximizing = false` (O moves next), then compare `signum()` of chosen vs best (RESEARCH Pattern 2). `pub(crate) enum MoveQuality { Good, Bad }` defined next to it; `best_engine_move` is `pub(crate)` at line 79 as the visibility precedent.

**TD-03 / TD-04 fix sites** (ai.rs tests): lines 196, 219, 230 use `let _ = ...make_move(...)`. Replace with the pattern already used at line 138:
```rust
let result = g.make_move(Position { row, col });
assert!(result.is_ok(), "make_move returned Err at ({row},{col})");
```
or `.expect("legal move")`. In `perfect_play_draws` (lines 213-237) the loop can spin forever if `make_move` errors; the `.expect` on line 230 ends the test instead.

---

### `kiss-tictactoe-engine/src/game.rs` (model, request-response) - add `Turn` and `Game::play`; TD-02

**Analog:** itself, `make_move` (game.rs lines 62-113).

**Guard pattern to preserve verbatim** (lines 63-76): `GameAlreadyOver`, `InvalidPosition`, `CellOccupied` guards, returning `Err` without mutating state.

**Imports to extend** (lines 3-5):
```rust
use crate::ai::best_engine_move;
use crate::detection::detect_status;
use crate::types::{Board, GameStatus, MoveError, MoveResult, Player, Position};
```

**Plan:** move the body of `make_move` into `pub fn play(&mut self, pos) -> Result<Turn, MoveError>`; insert (a) `move_quality(&self.board, pos)` BEFORE line 79 placement, (b) user-threat check after line 81 when status is InProgress, (c) engine-threat check after line 95 when InProgress. Then `make_move` becomes `self.play(pos).map(|t| t.result)` so the 20+ existing tests and doctests (lib.rs line 14, game.rs line 24) keep compiling. The existing early return at lines 84-86 (human move ended game) must still go through `classify` so a `UserWins` event is produced.

**Struct derive precedent** (line 28): `#[derive(Debug, Clone, PartialEq, Eq)]`. Keep `Game` free of RNG state (Commentator is separate). `Turn` should derive `Debug, Clone, Copy, PartialEq, Eq` like `MoveResult` (types.rs line 44).

**TD-02:** delete the duplicate test at game.rs lines 162-165 (`default_matches_new`), keeping `default_equals_new` (line 158).

---

### `kiss-tictactoe-engine/src/types.rs` / `lib.rs`

**Analog:** types.rs `MoveResult` / `GameStatus` (lines 32-50) for enum doc style; every variant carries a `///` doc.

**`TalkEvent` enum:** `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`, 8 variants, each documented. Add a `COUNT` const and use `ev as usize` for the `last` array index. Include the short comment on `UserWins` explaining it is unreachable until TRAIN-01 (CONTEXT specifics).

**lib.rs registration** (lines 19-26):
```rust
mod ai;
mod detection;
mod game;
mod types;

pub use detection::detect_status;
pub use game::Game;
pub use types::{Board, GameStatus, MoveError, MoveResult, Player, Position};
```
Add `mod commentary; mod events;` and `pub use commentary::Commentator; pub use game::{Game, Turn}; pub use types::TalkEvent` (or wherever `TalkEvent` lands). Update the crate-level doc (lines 5-7) to mention `play` and `Commentator`.

---

### `kiss-tictactoe-engine/src/commentary.rs` (service, transform)

**Analog:** `ai.rs` for module shape (module doc `//!`, private helpers, `pub` surface, `#[cfg(test)]` with `proptest` + `rstest`). Test imports precedent (ai.rs lines 103-108):
```rust
use super::*;
use proptest::prelude::*;
use rstest::rstest;
```
**Core:** RESEARCH.md Pattern 4 (`Commentator { state: u64, last: [Option<usize>; N] }`, xorshift 13/7/17, seed 0 remap, skip-over-last trick). Banks as `const` `&[&str]` slices per event; draft strings in RESEARCH "Draft Message Banks".

**Tests to include:** every bank has 4..=8 entries, each string <= 56 chars and non-empty, total 32..=64; same seed gives the same sequence; never repeats consecutively (proptest over `any::<u64>()` seeds); seed 0 yields a non-constant output. No analog for proptest-with-seed beyond the dev-dependency in `kiss-tictactoe-engine/Cargo.toml`. Dependencies must remain empty (`[dependencies]` is blank).

---

### `kiss-tictactoe-engine/tests/gameplay.rs` (test, request-response)

**Analog:** itself. Imports (line 1): `use kiss_tictactoe_engine::{Game, GameStatus, MoveError, MoveResult, Player, Position};`. Helper `first_empty(&Game) -> Option<Position>` (lines 3-13) drives full games. Add tests: play to completion with `play`, assert at most one event per turn, `make_move` result equals `play().result`, and a crafted D-04 scenario (user two-in-a-row seen before the reply). Assertion style: `assert_eq!(result, Ok(MoveResult::Continuing))` (line 30).

---

### `kiss-tictactoe-ui/src/state.rs` (model, CRUD in-memory)

**Analog:** `components/game_over_overlay.rs` lines 4-16 (pure function over engine types, unit-tested in the same file without a Dioxus runtime) and its test module (lines 40-64).

```rust
pub fn outcome_text(status: GameStatus) -> Option<&'static str> {
    match status {
        GameStatus::Win(Player::X) => Some("You win!"),
        ...
        GameStatus::InProgress => None,
    }
}
```
`Tally::record(&mut self, GameStatus)` should use the same exhaustive match (RESEARCH Pattern 6). `BubbleState { message: Option<&'static str>, serial: u32, is_final: bool }` with `greeting(line, serial)` and an `after_turn` constructor. Derive `Clone, Copy, PartialEq, Debug, Default` so it works as a `Signal` value. Tests: `#[cfg(test)] mod tests { use super::*; ... #[test] fn ... }` in the same file, same naming style (`outcome_text_engine_win`).

Register with `mod state;` in `main.rs`.

---

### `kiss-tictactoe-ui/src/components/speech_bubble.rs`, `scoreboard.rs` (component, signal read)

**Analog:** `components/game_over_overlay.rs` (lines 23-38) for the `#[component]` + `rsx!` + class naming, and `cell.rs` lines 12-29 for conditional classes.

**Component and conditional class pattern** (cell.rs lines 12-17):
```rust
rsx! {
    div {
        class: "cell",
        class: if content == Some(Player::X) { "cell--x" },
        class: if !interactive { "cell--frozen" },
```
Use for `speech-bubble--fading` / `speech-bubble--final` and for a hidden state. Keyed remount for animation restart: `key: "{serial}"` (the same `key: "{pos.row}-{pos.col}"` precedent is in board.rs line 43). Props: plain values (`message: Option<&'static str>`, `serial: u32`, `is_final: bool`) or `Signal<..>`; `Board` takes `Signal<Game>` (board.rs line 32), so either is consistent. Doc comments `///` on each component like game_over_overlay.rs lines 18-22. Stable `role="status"` wrapper per RESEARCH Pattern 5; confirm `aria_live` spelling on first compile.

Scoreboard: three `span`s Wins / Draws / Losses from a `Tally` prop; extract a pure label/format helper if you want testable logic, per `outcome_text`.

---

### `kiss-tictactoe-ui/src/components/controls.rs` (component, event-driven)

**Analog:** `components/game_over_overlay.rs` lines 24-34 (exact: button + `EventHandler<()>` prop):
```rust
pub fn GameOverOverlay(status: GameStatus, on_play_again: EventHandler<()>) -> Element {
    ...
    button {
        class: "play-again-btn",
        onclick: move |_| on_play_again.call(()),
        "Play again"
    }
```
`Controls(on_new_game: EventHandler<()>, on_reset_scores: EventHandler<()>)` with two buttons. Reset Scores uses a quieter secondary class (D-16). CSS analog for the button base: `.play-again-btn` (main.css lines 175-190).

---

### `kiss-tictactoe-ui/src/components/board.rs` (component, request-response) - TD-01, TD-06, event capture

**Analog:** itself.

**Borrow-safety pattern to preserve** (lines 33-36):
```rust
// Clone values out before any write to avoid borrow conflicts (Pitfall 1)
let board = *game.read().board();
let status = game.read().status();
```

**Handler to rewrite** (lines 47-49), currently `let _ = game.write().make_move(p);` (TD-01). Replace with a match on `game.write().play(p)` as a separate statement bound to a local before touching `tally`, `bubble`, or `commentator` signals (each write in its own statement, per RESEARCH anti-patterns):
```rust
on_click: move |p: Position| {
    let outcome = game.write().play(p);   // guard dropped at end of statement
    match outcome {
        Ok(turn) => { /* tally.write().record(..) if GameOver; bubble update from turn.event */ }
        Err(e) => debug_assert!(false, "unexpected move error: {e}"),
    }
},
```
`MoveError` implements `Display` (types.rs lines 63-71).

**Overlay wiring** (lines 53-58): `on_play_again: move |_| game.set(Game::new())` must become the shared `start_new_game` handler (D-13). Easiest: add `on_new_game: EventHandler<()>` prop to `Board`, passed from `App`, plus `bubble`, `tally`, `commentator` signals (RESEARCH Open Question 2).

**TD-06:** line 12 `pub fn board_cells` becomes `pub(crate) fn board_cells`. Tests (lines 63-161) are in-module, so unaffected. The tests' `let _ = game.make_move(...)` at lines 86 and 137 can stay (test code) or get `.expect`.

---

### `kiss-tictactoe-ui/src/main.rs` (root component, event-driven)

**Analog:** itself (lines 1-23).
```rust
mod components;

use components::board::Board;
use dioxus::prelude::*;
use kiss_tictactoe_engine::Game;

const MAIN_CSS: Asset = asset!("/assets/main.css");

#[component]
fn App() -> Element {
    let game = use_signal(Game::new);
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        div { class: "game-container",
            h1 { class: "app-title", "Tic-Tac-Toe" }
            Board { game }
        }
    }
}
```
Add `mod state;`, `use_signal(|| Commentator::new(clock_seed()))`, `use_signal(Tally::default)`, and a `BubbleState` signal initialized with a GameStart line (use one closure that draws from the commentator, RESEARCH "Shared new-game handler"). Layout order inside `.game-container`: `h1` -> `SpeechBubble` -> `Board` -> `Scoreboard` -> `Controls` (D-10, D-17). `clock_seed` snippet is in RESEARCH "Seeding in the UI". `use_signal(Game::new)` shows the initializer-function idiom.

---

### `kiss-tictactoe-ui/assets/main.css` (config)

**Analog:** itself.

**TD-05:** duplicate `body` blocks at lines 5-7 (`margin: 0`) and 49-58. Merge `margin: 0;` into the second block and delete lines 5-7.

**Token convention** (lines 13-44): new values go inside `:root` grouped by comment (`/* Color */`, `/* Spacing */`, etc.). Add bubble colors, `--bubble-fade-duration`, `--focus-ring-width`, `--focus-ring-offset` there. Values in `@keyframes` percentages must be literals (RESEARCH pitfall 9).

**Section header convention** (e.g. lines 72-75):
```css
/* ============================================================
   App title
   ============================================================ */
.app-title { ... }
```
Use for `.speech-bubble*`, `.scoreboard`, `.controls`. BEM-ish modifiers use `--` (`.cell--x`, `.cell--frozen`), so use `.speech-bubble--fading` / `--final`.

**Responsive precedent** (lines 137-144): `@media (max-width: 360px)` overriding `:root` tokens; keep 320px working for the new row via flex-wrap and tokens.

**Button base** to copy for `.controls` buttons: `.play-again-btn` (lines 175-186).

---

## Shared Patterns

### Pure-function core, thin framework shell
**Source:** `components/game_over_overlay.rs` lines 4-16 (`outcome_text`) and `board.rs` lines 12-25 (`board_cells`).
**Apply to:** `state.rs`, `scoreboard.rs`, `speech_bubble.rs`. Put logic in a pure `pub fn` (testable with plain `#[test]`), let the `#[component]` only render.

### Clone out of signals before writing (Pitfall 1)
**Source:** `board.rs` lines 33-36.
**Apply to:** every handler touching `game`, `tally`, `bubble`, `commentator`. One write per statement; never hold a `read()` guard across a `write()`.

### Engine API doc discipline
**Source:** `lib.rs` line 17 `#![warn(missing_docs)]`; `types.rs` per-variant `///`.
**Apply to:** all new `pub` engine items (`TalkEvent`, `Turn`, `Commentator`, their methods and fields).

### Test style
**Source:** `detection.rs` lines 47-120 (rstest `#[case]`/`#[values]`), `ai.rs` lines 103-122 (proptest + rstest imports, `board_from` helper), `game_over_overlay.rs` lines 40-64 (plain `#[test]` per case).
**Apply to:** `events.rs`, `commentary.rs`, `state.rs`.

### Engine purity gate
**Source:** `kiss-tictactoe-engine/Cargo.toml` (`[dependencies]` empty; dev-deps `proptest = "1.12.0"`, `rstest = "0.27.0"`).
**Apply to:** all engine work. Check `cargo tree -p kiss-tictactoe-engine | grep dioxus` stays empty. No `rand`.

### Error handling
**Source:** `types.rs` `MoveError` (lines 53-73), guards at `game.rs` lines 63-76.
**Apply to:** `Game::play` (same guards, same error type, no new error variants needed); UI handler matches on the `Result` instead of discarding it.

## No Analog Found

| File / Concern | Role | Data Flow | Reason |
|----------------|------|-----------|--------|
| xorshift PRNG inside `commentary.rs` | utility | transform | No randomness exists in the repo; use RESEARCH Pattern 4 |
| CSS `@keyframes` fade + keyed remount | styling | animation | No animations in `main.css`; use 03-UI-SPEC.md "Fade behavior" |
| `SystemTime` seed (`clock_seed`) | utility | n/a | First platform-clock use; RESEARCH snippet |
| `aria-live` region | component | n/a | No a11y attributes yet; RESEARCH Pattern 5, verify attribute spelling on compile |

## Metadata

**Analog search scope:** whole repo (`git ls-files`, 14 source/asset files in the two crates)
**Files scanned:** 12 read in full or in relevant parts (lib.rs, types.rs, detection.rs, game.rs, ai.rs, gameplay.rs, main.rs, 3 components, mod.rs, main.css)
**Pattern extraction date:** 2026-10-14
