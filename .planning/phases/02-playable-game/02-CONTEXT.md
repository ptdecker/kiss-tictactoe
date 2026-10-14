# Phase 2: Playable Game - Context

**Gathered:** 2026-10-14
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 2 delivers a fully playable tic-tac-toe game: the engine core (game state, win/draw detection, minimax AI) wired to a Dioxus desktop UI where a human can click board cells, watch the engine respond, and see the outcome declared. A game-over overlay communicates the result with a minimal restart button.

**In scope:** ENGI-01 (game state), ENGI-02 (win/draw detection), ENGI-03 (clean public API), ENGI-04 (minimax AI), UI-01 (clickable board), UI-06 (responsive layout, 320px+).

**Out of scope (Phase 3):** Trash talk message bank (ENGI-05, ENGI-06, ENGI-07), speech bubble (UI-02), session scoreboard (UI-03), formal New Game button (UI-04), Reset Scores button (UI-05).

</domain>

<decisions>
## Implementation Decisions

### Engine API Structure
- **D-01:** The engine exposes a `Game` struct with methods — `Game::new()` creates a game; `game.make_move(pos)`, `game.status()`, `game.board()` are instance methods. No pure-function alternative. Types mentioned in REQUIREMENTS.md (Phase 4 docs success criteria): `Game`, `Board`, `GameStatus`, `Player`, `MoveResult`.
- **D-02:** Board positions use a `Position { row: u8, col: u8 }` struct — named fields, self-documenting, idiomatic Rust. Not a flat 0–8 index or a raw tuple.
- **D-03:** `make_move` returns `Result<MoveResult, MoveError>`. `MoveError` covers `CellOccupied` and `InvalidPosition`. The UI will prevent invalid moves via non-interactive cells, but the API stays honest.
- **D-04:** The engine moves automatically inside `make_move` — one call places the player's mark and the engine responds. The Dioxus UI calls `game.make_move(pos)` and re-renders; no separate `engine_move()` call needed.

### Game-Over Display (Phase 2)
- **D-05:** Game outcome is shown as a board overlay/banner. The board goes non-interactive and an overlay appears on top declaring the result.
- **D-06:** Outcome text is user-centric: "You win!" / "Engine wins!" / "Draw!" — speaks directly to the player. Phase 3 replaces this with the engine's trash talk speech bubble.
- **D-07:** The Phase 2 overlay includes a minimal "Play again" button that resets to a new game. This keeps Phase 2 self-contained and testable without requiring Phase 3's formal New Game control. Phase 3 will replace or augment this with the proper UI-04 New Game button.

### Responsive Layout
- **D-08:** Fixed max-size board (e.g., ~360px), centered using flexbox. Below the max-size it fills the viewport width. Simple and predictable; Phase 3 additions (scoreboard, speech bubble) stack vertically in the same column.
- **D-09:** Single centered column layout: title → board → (Phase 3 will insert bubble + scoreboard + controls). No sidebar, no two-column split. Extends naturally to Phase 3's components.
- **D-10:** CSS custom properties (`--cell-size`, `--board-max`, `--color-x`, `--color-o`, `--color-bg`, etc.) defined on `:root` for core tokens. No magic numbers. Phase 3 adds to the token set.
- **D-11:** Board cells use a clean minimal style: borders define the grid, no background fill on cells. X and O marks displayed in the cell center. Classic tic-tac-toe look with hover states on empty cells.

### Claude's Discretion
- Player assignment: user is always X (goes first), engine is always O. SETUP-01 (who goes first) is a v2 requirement — Phase 2 hardcodes this convention. Claude picks the convention without re-asking (X = human player is the classic default).
- Board dimensions: 3×3 CSS Grid for the cells, not flexbox rows. Standard approach for a fixed-aspect board.
- Move history tracking: whether `Game` stores move history internally (for future use) or only current board state — Claude decides based on what's needed by ENGI-01.
- Minimax implementation details: alpha-beta pruning is acceptable; full traversal is fine at 3×3 scale. Claude picks the cleaner implementation.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Requirements and Scope
- `.planning/ROADMAP.md` §Phase 2 — Goal, success criteria, requirements (ENGI-01–04, UI-01, UI-06), branch name (`phase/2-playable-game`)
- `.planning/REQUIREMENTS.md` §v1 Requirements → Engine — Detailed specs for ENGI-01 through ENGI-04
- `.planning/REQUIREMENTS.md` §v1 Requirements → UI — Detailed specs for UI-01 and UI-06
- `.planning/PROJECT.md` §Constraints — Engine purity rule, Dioxus version pin, plain CSS, webview renderer

### Prior Phase Context
- `.planning/phases/01-foundation/01-CONTEXT.md` — Phase 1 decisions: `dx new` scaffold, `asset!()` wiring, Dioxus.toml structure, placeholder app entrypoint to replace

### Technology Stack and Versions
- `CLAUDE.md` §Technology Stack — Exact pinned versions: `dioxus = "0.7.10"`, `dioxus-cli = "0.7.10"`, `manganis = "0.7.10"`. State management table (`use_signal`, `Signal<T>`, `use_context`). Cargo workspace structure.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `kiss-tictactoe-ui/src/main.rs` — Existing `App` component with `asset!()` and `document::Stylesheet` wired. Replace the placeholder `div` body with real game components.
- `kiss-tictactoe-ui/assets/main.css` — Existing CSS asset loaded via `manganis`. Add CSS custom properties and board styles here.
- `kiss-tictactoe-ui/Dioxus.toml` — Existing Dioxus project config with asset pipeline ready. No changes needed.

### Established Patterns
- Dioxus 0.7.x `#[component]` + `rsx!` macro: established in Phase 1. Phase 2 follows the same component pattern.
- `asset!("/assets/main.css")` via `manganis`: already wired. Add all CSS to `assets/main.css`.
- `dioxus::launch(App)` entry point: intact. Phase 2 replaces the `App` body, not the entry point.

### Integration Points
- `kiss-tictactoe-engine` is a workspace sibling with zero dependencies. Phase 2 adds it as a dependency to `kiss-tictactoe-ui/Cargo.toml`.
- Engine purity check (`cargo tree -p kiss-tictactoe-engine | grep dioxus`) must return empty after integration — ROADMAP.md Phase 2 success criterion #4 and a hard gate.

</code_context>

<specifics>
## Specific Ideas

- The minimal "Play again" button in the Phase 2 overlay is intentionally temporary — it resets game state but does not update any scoreboard (scoreboard is Phase 3). Phase 3's formal New Game button replaces or supersedes it.
- The game-over overlay sits on top of the frozen board. The "Play again" button dismisses the overlay and resets `Game::new()` via the Dioxus signal.
- X marks should be visually distinct from O marks (different colors using CSS variables `--color-x` and `--color-o`). Claude picks accessible default values.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 2-Playable Game*
*Context gathered: 2026-10-14*
