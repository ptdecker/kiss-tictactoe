---
phase: 03-personality
plan: 03
subsystem: engine
tags: [minimax, threat-detection, trash-talk, events]
requires: [03-01]
provides:
  - has_threat, classify (D-03 priority), MoveQuality/move_quality (minimax-judged user moves)
  - Game::play emits tactical TalkEvents
affects: [03-04]
tech-stack:
  added: []
  patterns: [reuse minimax for move judging via signum outcome class, intermediate checks as locals inside play]
key-files:
  created: []
  modified:
    - kiss-tictactoe-engine/src/detection.rs
    - kiss-tictactoe-engine/src/events.rs
    - kiss-tictactoe-engine/src/ai.rs
    - kiss-tictactoe-engine/src/game.rs
    - kiss-tictactoe-engine/tests/gameplay.rs
key-decisions:
  - "Move quality compares outcome class via signum, not depth-adjusted scores"
  - "User threat evaluated after X is placed and before the engine replies (D-04)"
requirements-completed: [ENGI-06, ENGI-07]
duration: 10min
completed: 2026-10-14
---

# Phase 3 Plan 03: Tactical Trash Talk Summary

Game::play now emits exactly one event per turn by fixed priority (game over, engine threat, user threat, bad move, good move), with threats found by a shared-line-table scan and user moves judged by the engine's own minimax.

## Tasks
1. Threat detection, minimax move quality, TD-03/TD-04: 1097063
2. classify wired into Game::play, tactical end-to-end tests: 8a7d116

## Verification
`cargo test --workspace` passes (86 engine unit, 16 + 4 integration/doc tests); `cargo doc` zero warnings; no dioxus in engine dependency tree; no `let _ = ` left in ai.rs.

## Deviations from Plan
None - plan executed exactly as written. Engine replies matched the interface notes, so no test sequences were substituted. Task 1's commit included `classify` (in events.rs) since it shares the file with `has_threat`; game.rs wiring is in Task 2's commit.

## Known Stubs
None.

## Self-Check: PASSED
