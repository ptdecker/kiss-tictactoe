---
phase: 03-personality
reviewed: 2026-10-14T00:00:00Z
depth: standard
files_reviewed: 16
files_reviewed_list:
  - kiss-tictactoe-engine/src/ai.rs
  - kiss-tictactoe-engine/src/commentary.rs
  - kiss-tictactoe-engine/src/detection.rs
  - kiss-tictactoe-engine/src/events.rs
  - kiss-tictactoe-engine/src/game.rs
  - kiss-tictactoe-engine/src/lib.rs
  - kiss-tictactoe-engine/src/types.rs
  - kiss-tictactoe-engine/tests/gameplay.rs
  - kiss-tictactoe-ui/assets/main.css
  - kiss-tictactoe-ui/src/components/board.rs
  - kiss-tictactoe-ui/src/components/controls.rs
  - kiss-tictactoe-ui/src/components/mod.rs
  - kiss-tictactoe-ui/src/components/scoreboard.rs
  - kiss-tictactoe-ui/src/components/speech_bubble.rs
  - kiss-tictactoe-ui/src/main.rs
  - kiss-tictactoe-ui/src/state.rs
findings:
  critical: 0
  warning: 3
  info: 5
  total: 8
status: issues_found
---

# Phase 3: Code Review Report

**Reviewed:** 2026-10-14
**Depth:** standard
**Files Reviewed:** 16
**Status:** issues_found

## Summary

The engine and UI are largely sound. The minimax, the event classification and the non-repeating selector are correct. The no-repeat index remap `i >= prev => i+1` is right, and `n-1` can't be 0 because every bank has at least 4 lines. No security issues were found. The findings below are one behavioral flaw in move judging, two robustness concerns, and some maintainability items.

## Warnings

### WR-01: `move_quality` calls every move by a doomed player "Good"

**File:** `kiss-tictactoe-engine/src/ai.rs:116-133`
**Issue:** The comparison uses `signum()` only. When X is already in a forced-loss position, every move scores -1, so `chosen.signum() == best.signum()` and the result is `Good`. A move that hands the engine an immediate win (for example ignoring an open O threat) is therefore praised with "Fine. That was not terrible." or "A sensible move." This contradicts the intent of BadMove. The case is reachable: once X blunders, every later move is judged Good.
**Fix:** When both classes are losses, compare depth-adjusted scores, or at minimum treat "chosen loses and a better-scoring move delays the loss" as Bad:
```rust
if chosen.signum() < best.signum() || (best < 0 && chosen < best) {
    MoveQuality::Bad
} else {
    MoveQuality::Good
}
```
Add a test with X facing an unblocked O threat, where the non-blocking move must be Bad.

### WR-02: `debug_assert!` on a user-driven path can panic in debug builds

**File:** `kiss-tictactoe-ui/src/components/board.rs:54`
**Issue:** The click handler asserts that `play` returns `Ok`. The UI only freezes cells after re-render, so a fast second click on the same cell, or a click after game over but before re-render, can produce `CellOccupied` or `GameAlreadyOver`. That aborts the app under `dx serve` and in debug builds. In release the error is silently dropped. Both outcomes are poor handling of an expected condition.
**Fix:** Remove the assert and ignore the expected errors explicitly:
```rust
if let Ok(turn) = game.write().play(p) { on_turn.call(turn); }
```

### WR-03: `play` flags a user threat the engine may not actually need to block, and the signal is computed before the engine reply but is not tied to it

**File:** `kiss-tictactoe-engine/src/game.rs:133, 160-162`
**Issue:** `user_threat` is computed from X's board before the engine replies. If the engine instead wins outright, status is `Win` and the lifecycle event wins, so that case is fine. The classification does not verify that the threat is still open after the reply. In practice the perfect engine always blocks, but the code relies on that unstated invariant. If the engine ever becomes imperfect (TRAIN-01, which the comments already anticipate), `UserAboutToWin` would be reported with the wrong semantics ("Watch me ruin it" when the user's threat was left open).
**Fix:** Compute `user_threat` as "X had a threat before the reply and it is now blocked", or add a doc comment pinning the invariant. This is low risk today.

## Info

### IN-01: Misleading and duplicated comments

**File:** `kiss-tictactoe-engine/src/types.rs:66-69`, `kiss-tictactoe-engine/src/ai.rs:212`
**Issue:** The `UserWins` variant has a rustdoc paragraph and a near-identical `//` comment saying the same thing. The exhaustive test comment says "Enumerates up to 9x7x5x3 = 945 human move sequences", which is not the number of branches actually visited.
**Fix:** Delete the redundant `//` block and correct or drop the 945 figure.

### IN-02: `TalkEvent::ALL` and `COUNT` are hand-synced with the enum

**File:** `kiss-tictactoe-engine/src/types.rs:85-96`, `kiss-tictactoe-engine/src/commentary.rs:125`
**Issue:** `COUNT = 8`, the `[TalkEvent; 8]` length and `event as usize` indexing all depend on declaration order. Adding a variant without updating them yields a panic or a missing category.
**Fix:** Define `ALL` as `[TalkEvent; Self::COUNT]` and add a test asserting `ALL[i] as usize == i`.

### IN-03: Weak seed mixing for xorshift

**File:** `kiss-tictactoe-engine/src/commentary.rs:104-118`, `kiss-tictactoe-ui/src/main.rs:31-36`
**Issue:** The raw nanosecond clock is used as the xorshift state. For small or similar seeds the first outputs are poorly mixed, so the first greeting is only weakly random.
**Fix:** Run the seed through a splitmix64 step in `Commentator::new`, or discard a few initial outputs.

### IN-04: Unused-assignment test and needless clone

**File:** `kiss-tictactoe-ui/src/state.rs:172-176`, `kiss-tictactoe-ui/src/components/speech_bubble.rs:10`
**Issue:** The `reset_is_default` test assigns `t = Tally { .. }` and then immediately overwrites it, which triggers the `unused_assignments` warning. It doesn't exercise the real reset path. `message.clone().unwrap_or_default()` clones the `String` unnecessarily.
**Fix:** Use `message.as_deref().unwrap_or("")` in the component. Replace the test with something meaningful or remove it.

### IN-05: Accessibility and layout gaps in CSS

**File:** `kiss-tictactoe-ui/assets/main.css:194-209`, `kiss-tictactoe-ui/assets/main.css:124-154`, `kiss-tictactoe-ui/src/main.rs:21`
**Issue:** The Play again button has no `:focus-visible` style, unlike `.btn`. Cells use `pointer-events: none` and carry no keyboard affordance. The minimum window height of 560px is less than the column's natural height of about 685px, so the controls can scroll out of view at minimum size. The bubble fades to opacity 0 after 5s, so sighted users lose the final context, though final messages are exempt.
**Fix:** Reuse the `.btn:focus-visible` rule for `.play-again-btn`, and raise the minimum height or allow the column to scroll cleanly.

---

_Reviewed: 2026-10-14_
_Reviewer: Claude (gsd-code-reviewer)_
_Depth: standard_
