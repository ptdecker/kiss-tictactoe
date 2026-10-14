---
phase: 03-personality
plan: 01
subsystem: engine
tags: [rust, commentary, xorshift, trash-talk]
requires: []
provides:
  - TalkEvent (8 categories), Commentator (seeded, non-repeating), Turn, Game::play
affects: [03-02, 03-03, 03-04]
tech-stack:
  added: []
  patterns: [std-only xorshift64 PRNG, make_move as thin wrapper over play]
key-files:
  created:
    - kiss-tictactoe-engine/src/commentary.rs
    - kiss-tictactoe-engine/src/events.rs
  modified:
    - kiss-tictactoe-engine/src/types.rs
    - kiss-tictactoe-engine/src/game.rs
    - kiss-tictactoe-engine/src/lib.rs
    - kiss-tictactoe-engine/tests/gameplay.rs
key-decisions:
  - "Skip-over-last index selection guarantees no consecutive repeats without re-pick loops"
requirements-completed: [ENGI-05, ENGI-07]
duration: 15min
completed: 2026-10-14
---

# Phase 3 Plan 01: Engine Voice Summary

Engine crate gains `TalkEvent`, a seeded std-only xorshift64 `Commentator` with 8 banks (4-5 lines each, at most 56 chars), and `Game::play` returning `Turn { result, event }` with lifecycle events; `make_move` now delegates to `play`.

## Tasks
1. RED end-to-end tests: fc6eebf
2. TalkEvent, Commentator, banks: 12471ae
3. Game::play, Turn, lifecycle_event, TD-02 (duplicate test removed): 102a8bb

## Verification
`cargo test -p kiss-tictactoe-engine` green (57 unit, 11 integration, 4 doctests); `cargo doc` zero warnings; engine has zero normal dependencies.

## Deviations from Plan
None - plan executed exactly as written.

## Known Stubs
None. (`UserWins` is intentionally unreachable until TRAIN-01.)

## Self-Check: PASSED
