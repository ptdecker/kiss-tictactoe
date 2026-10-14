---
phase: 02-playable-game
plan: 02
subsystem: testing
tags: [rust, proptest, rstest, minimax, property-based-testing, parameterized-tests]

requires:
  - phase: 02-playable-game
    plan: 01
    provides: Engine public API (Game, Position, Player, Board, GameStatus, detect_status, best_engine_move)

provides:
  - proptest 1.12.0 and rstest 0.27.0 as dev-dependencies in kiss-tictactoe-engine
  - 21-test rstest detection suite: all 8 winning lines × 2 players + 5 named cases
  - Exhaustive engine_never_loses proof across all X move sequences (9×7×5×3=945 paths)
  - perfect_play_draws: machine-verified ENGI-04 (Draw when both sides play optimally)
  - Tactical minimax tests: takes win, blocks threat, corner→center, center→corner
  - proptest random_play_preserves_invariants: history/count/status invariants
  - proptest out_of_range_positions_rejected: all u8 pairs where row>2 or col>2

affects: [02-03-game-over-overlay, 03-personality, 04-documentation]

tech-stack:
  added:
    - proptest 1.12.0 (dev-dependency, engine crate only)
    - rstest 0.27.0 (dev-dependency, engine crate only)
  patterns:
    - "Blocking human checkpoint for crate legitimacy before adding dev-dependencies"
    - "board_with_line helper using module-private WINNING_LINES const from child test module"
    - "rstest #[values] for combinatorial parameterization (line × player = 16 cases)"
    - "Exhaustive recursive explore() counts terminal games and draws for prove-no-loss"
    - "x_best_outcome() test-local minimax for perfect-play-draws proof"
    - "proptest prop_assume! for open-ended strategy filtering (out-of-range positions)"

key-files:
  modified:
    - kiss-tictactoe-engine/Cargo.toml
    - Cargo.lock
    - kiss-tictactoe-engine/src/detection.rs
    - kiss-tictactoe-engine/src/ai.rs
    - kiss-tictactoe-engine/src/game.rs

key-decisions:
  - "board_with_line accesses module-private WINNING_LINES via child test module — no need to make the const pub"
  - "x_best_outcome uses make_move recursively (not raw board minimax) — consistent with the engine's single-call API contract"
  - "prop_assume!(row > 2 || col > 2) preferred over constrained strategy — rejection rate <0.1%, cleaner"
  - "Retained existing basic ai.rs tests alongside new ones — additional coverage at zero cost"

patterns-established:
  - "Pattern: Exhaustive game-tree enumeration with recursive helper + stats tuple for no-loss proofs"
  - "Pattern: Test-local minimax (x_best_outcome) wrapping make_move for perfect-play-draws proof"

requirements-completed: [ENGI-01, ENGI-02, ENGI-04]

duration: 15min
completed: 2026-10-14
---

# Phase 02 Plan 02: Engine Correctness Proof — Property and Parameterized Tests Summary

**52-test suite machine-verifies the minimax engine never loses across all reachable game states, draws against perfect play, and preserves state invariants under random and out-of-range inputs**

## Performance

- **Duration:** ~15 min
- **Started:** 2026-10-14T20:57:00Z
- **Completed:** 2026-10-14T21:12:31Z
- **Tasks:** 3 (plus package legitimacy checkpoint approved by human)
- **Files modified:** 5

## Accomplishments

- ROADMAP Phase 2 success criterion 2 satisfied: AI correctness suite verifies minimax plays optimally for all reachable board states (engine_never_loses_exhaustive covers all 9×7×5×3=945 X move sequences)
- All 8 win lines × 2 players proven via rstest #[values] (16 combinations + 5 named detection cases = 21 detection tests)
- Engine purity gate holds: `cargo tree -p kiss-tictactoe-engine | grep dioxus` prints nothing after adding dev-dependencies

## Task Commits

1. **Task 1: Add dev-dependencies** — `fd66771` (feat)
2. **Task 2: Parameterized detection suite** — `b8c1efe` (test)
3. **Task 3: AI correctness proof + game.rs invariants** — `5ce0678` (test)

## Files Created/Modified

- `kiss-tictactoe-engine/Cargo.toml` — Added [dev-dependencies] with proptest = "1.12.0" and rstest = "0.27.0"; [dependencies] remains empty (ENGI-03)
- `Cargo.lock` — Updated with resolved proptest/rstest transitive deps
- `kiss-tictactoe-engine/src/detection.rs` — Expanded test module: board_with_line helper, each_line_wins_for_each_player (16 combinations via #[values]), detection_cases (5 #[case] entries)
- `kiss-tictactoe-engine/src/ai.rs` — Expanded test module: board_from helper, engine_never_loses_exhaustive, perfect_play_draws, 4 tactical rstest cases, 2 proptest! blocks, retained 4 basic tests
- `kiss-tictactoe-engine/src/game.rs` — Added history_records_human_then_engine_moves, current_player_is_x_between_turns, default_matches_new

## Decisions Made

- `board_with_line` uses the module-private `WINNING_LINES` const directly — child test modules can access private items without `pub`, keeping the const internal
- `x_best_outcome` is implemented as a test-local recursive function that calls `make_move` (not raw board minimax) — this means perfect play is tested through the same API path as production code
- `prop_assume!(row > 2 || col > 2)` avoids a constrained range strategy; rejection rate is under 0.1% (only 9 out of 65536 u8 pairs fail the assume)

## Deviations from Plan

None. Plan executed exactly as written. All acceptance criteria passed on first run.

## Issues Encountered

None beyond normal compilation iteration.

## Known Stubs

None. This plan adds tests only — no stubs introduced.

## Threat Flags

All threat mitigations from the plan's threat model were verified:
- T-02-SC: Blocking human checkpoint approved (proptest 1.12.0 and rstest 0.27.0 verified on crates.io before install)
- T-02-01: `out_of_range_positions_rejected` proptest confirms Err(InvalidPosition) for any row>2 or col>2
- T-02-02: Exhaustive test only plays empty cells and asserts Ok; invariant test verifies history/mark count consistency
- T-02-03: Exhaustive test asserts status never Win(X); random_play_preserves_invariants stops at terminal status
- T-02-05: `cargo tree -p kiss-tictactoe-engine | grep dioxus` prints nothing

No new threat surface introduced.

## Self-Check: PASSED

Files exist:
- kiss-tictactoe-engine/Cargo.toml: FOUND
- kiss-tictactoe-engine/src/detection.rs: FOUND
- kiss-tictactoe-engine/src/ai.rs: FOUND
- kiss-tictactoe-engine/src/game.rs: FOUND

Commits found:
- fd66771 (feat dev-deps): FOUND
- b8c1efe (test detection): FOUND
- 5ce0678 (test AI proof): FOUND
