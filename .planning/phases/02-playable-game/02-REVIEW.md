---
phase: 02-playable-game
reviewed: 2026-10-14T00:00:00Z
depth: standard
files_reviewed: 14
files_reviewed_list:
  - kiss-tictactoe-engine/Cargo.toml
  - kiss-tictactoe-engine/src/ai.rs
  - kiss-tictactoe-engine/src/detection.rs
  - kiss-tictactoe-engine/src/game.rs
  - kiss-tictactoe-engine/src/lib.rs
  - kiss-tictactoe-engine/src/types.rs
  - kiss-tictactoe-engine/tests/gameplay.rs
  - kiss-tictactoe-ui/Cargo.toml
  - kiss-tictactoe-ui/assets/main.css
  - kiss-tictactoe-ui/src/components/board.rs
  - kiss-tictactoe-ui/src/components/cell.rs
  - kiss-tictactoe-ui/src/components/game_over_overlay.rs
  - kiss-tictactoe-ui/src/components/mod.rs
  - kiss-tictactoe-ui/src/main.rs
findings:
  critical: 0
  warning: 4
  info: 2
  total: 6
status: issues_found
---

# Phase 02: Code Review Report

**Reviewed:** 2026-10-14
**Depth:** standard
**Files Reviewed:** 14
**Status:** issues_found

## Summary

Reviewed the complete engine (`kiss-tictactoe-engine`) and UI (`kiss-tictactoe-ui`) implementation
for the playable game phase, including AI, detection, game state, all UI components, and CSS.

The engine core is sound. Minimax with alpha-beta pruning is correctly implemented: O is the
minimizer, depth-adjusted scoring prefers faster wins (`depth - 10` grows toward zero as depth
increases, so an immediate O win at depth 0 scores -10 while one at depth 2 scores -8, correctly
prioritizing the sooner win). Win/draw detection is complete and correct across all 8 lines. The
`Game` struct correctly guards every invalid-move case atomically before mutating state.

No critical issues were found. Four warnings exist: one in production UI code (silently swallowed
engine error) and three in test helpers (silent discards that degrade failure diagnostics, including
two paths that would hang or stack-overflow instead of producing a clean test failure).

---

## Warnings

### WR-01: Silent discard of `make_move` result in click handler

**File:** `kiss-tictactoe-ui/src/components/board.rs:48`
**Issue:** The click callback writes `let _ = game.write().make_move(p);`, discarding the
`Result<MoveResult, MoveError>` without inspection. The UI does guard against invalid state (cells
are frozen via `interactive = false` and `pointer-events: none` before they become non-interactive),
but a future change — for example, relaxing the `interactive` gate or wiring the overlay button
incorrectly — could let a `GameAlreadyOver` or `CellOccupied` error slip through silently. Silent
swallowing also makes runtime debugging harder when something does go wrong.
**Fix:**
```rust
onclick: move |p: Position| {
    if interactive {
        if let Err(e) = game.write().make_move(p) {
            // Should not occur; log so it is visible during development.
            web_sys::console::error_1(&format!("make_move error: {e}").into());
            // Or in desktop: eprintln!("make_move error: {e}");
        }
    }
},
```
At minimum, add a `debug_assert!(result.is_ok())` so development builds catch unexpected failures.

---

### WR-02: Duplicate test in `game.rs`

**File:** `kiss-tictactoe-engine/src/game.rs:158` and `kiss-tictactoe-engine/src/game.rs:163`
**Issue:** `default_equals_new` (line 158) and `default_matches_new` (line 163) are byte-for-byte
identical. One of them is dead test coverage — it adds CI time but proves nothing additional. More
importantly, duplicate names with slightly different suffixes suggest the second test was meant to
verify something different and that check was never actually written.
**Fix:** Remove one of the two tests. If a second test for `Default` was intended to cover a
different angle (e.g., that `Default::default()` produces the same `status` as `new()` rather than
identity equality), write that distinct assertion instead.

---

### WR-03: Silent discard in `x_best_outcome` test helper enables infinite recursion

**File:** `kiss-tictactoe-engine/src/ai.rs:197`
**Issue:**
```rust
let mut g = game.clone();
let _ = g.make_move(Position { row, col });  // result silently discarded
let score = x_best_outcome(&g);
```
If `make_move` returns `Err` for any reason, `g` is left in its original state (the engine contract
guarantees no mutation on error). `x_best_outcome` then recurses on the unchanged game. The same
position is tried again on the next invocation, `make_move` fails again, and the process repeats
— producing an unbounded stack growth and stack overflow rather than a clean test failure. In
practice, valid positions on an in-progress game should never fail, but the intent of the test
suite is to _verify_ that invariant, so using a result-discarding helper to do so is circular. If
the engine had a regression that caused spurious `Err` returns, this test would crash the process
instead of printing a useful failure message.
**Fix:**
```rust
let mut g = game.clone();
let result = g.make_move(Position { row, col });
assert!(result.is_ok(), "make_move unexpectedly failed at ({row},{col}): {result:?}");
let score = x_best_outcome(&g);
```

---

### WR-04: Silent discard in `perfect_play_draws` causes infinite loop on test failure

**File:** `kiss-tictactoe-engine/src/ai.rs:230`
**Issue:**
```rust
Some(pos) => {
    let _ = game.make_move(pos);  // result silently discarded
}
```
Inside a `while game.status() == GameStatus::InProgress { ... }` loop. If `make_move` fails, the
game state is unchanged, `status` remains `InProgress`, `best_move` is found again on the next
iteration, `make_move` fails again, and the loop spins forever. A CI runner would time out rather
than producing a test failure with a useful message.
**Fix:**
```rust
Some(pos) => {
    game.make_move(pos)
        .expect("perfect-play move should never fail");
}
```

---

## Info

### IN-01: `body` rule declared twice in CSS

**File:** `kiss-tictactoe-ui/assets/main.css:5` and `kiss-tictactoe-ui/assets/main.css:49`
**Issue:** `body { margin: 0; }` appears first as a reset (line 5–7), then a full `body { ... }`
rule appears at line 49–58. CSS applies both additively; `margin: 0` is not overridden by the
second rule since the second rule does not set `margin`. This works today, but splitting a single
element's rules across two blocks separated by ~45 lines increases the risk of accidentally
duplicating or losing a property during future edits.
**Fix:** Merge `margin: 0` into the full `body` block at line 49, removing the standalone block at
line 5–7. The reset intent is clear from the comment at the top of the file.

---

### IN-02: `board_cells` exported as `pub` but only consumed internally

**File:** `kiss-tictactoe-ui/src/components/board.rs:12`
**Issue:** `pub fn board_cells(...)` is only called from within `board.rs` itself (both the
`Board` component and the `#[cfg(test)]` child module). `components/mod.rs` re-exports `pub mod
board`, making `board_cells` reachable from `main.rs` and any future sibling component. The
function is a layout-computation helper that is an implementation detail of `Board`; leaking it
into the crate's public surface invites unintended external callers.
**Fix:** Change to `pub(super)` to restrict visibility to the `components` module, or to
`pub(crate)` if cross-component reuse is anticipated. The `mod tests` child module can see
private items directly, so `pub(crate)` suffices.

---

_Reviewed: 2026-10-14_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
