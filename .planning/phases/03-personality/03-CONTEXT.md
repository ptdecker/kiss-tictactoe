# Phase 3: Personality - Context

**Gathered:** 2026-10-14
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 3 gives the engine a voice. The engine emits trash-talk events (lifecycle and tactical) and owns a message bank; the Dioxus UI shows the current line in a speech bubble, tracks a session scoreboard (wins / draws / losses), and provides New Game and Reset Scores controls.

**In scope:** ENGI-05 (lifecycle events), ENGI-06 (tactical events), ENGI-07 (4-8 strings per category, random selection), UI-02 (speech bubble), UI-03 (session scoreboard), UI-04 (New Game, score persists), UI-05 (Reset Scores, independent of game), plus technical-debt items TD-01 to TD-06 (see decisions).

**Out of scope:** difficulty levels, training mode, hints, who-goes-first (all v2), persistence across app restarts, winning-line highlight (POLSH-01), keyboard nav (POLSH-02), Dependabot items TD-07/TD-08 (upstream, deferred to a Dioxus bump), documentation (Phase 4).

</domain>

<decisions>
## Implementation Decisions

### Event model and triggers
- **D-01:** The "user wins" event stays in the engine with its own 4-8 line bank, but is documented as unreachable against the perfect-minimax engine until v2 training mode (TRAIN-01) makes it possible. ENGI-05/ENGI-07 stay satisfied as written; no requirements edit.
- **D-02:** "Engine assesses a good move" and "bad move" (ENGI-06) judge the **user's** move (the engine never plays badly). Judging uses **minimax score comparison**: score the user's chosen move against the best available move. Good = preserves the best achievable outcome (a draw). Bad = drops the outcome so the engine can now force a win. Reuse the existing `ai.rs` minimax; do not introduce a second heuristic evaluator.
- **D-03:** Exactly **one message per turn** (a turn = the user's move plus the engine's reply), chosen by fixed priority: game-over events first, then tactical threats (engine about to win before user about to win), then move quality (bad before good), then nothing. Selection of *which* event is deterministic; only the *line within* the category is random.
- **D-04:** "User about to win" (user has two in a row with the third cell open) is evaluated **immediately after the user's move, before the engine replies**, even though the engine will then block it. "Engine about to win" is evaluated on the board **after the engine's reply**. Because the two checks look at different board states within one `make_move` call, the planner must design the engine API so a single call can compute both (e.g. `make_move` returns the event alongside `MoveResult`) without exposing a half-turn board to the UI.
- **D-05:** The game-start event fires when a new game begins (app launch and every New Game). The user is always X and moves first (carried over from Phase 2 discretion; SETUP-01 is v2), so the engine never opens a game.

### Message bank and randomness
- **D-06:** The message bank and line selection live in the **engine crate**. Any future UI gets the trash talk for free; Phase 4's engine manual documents the event system there. The engine stays free of Dioxus and platform dependencies.
- **D-07:** Randomness uses a **tiny std-only PRNG (xorshift-style, about 10 lines) with the seed passed in by the caller**. The UI seeds it from the system clock; tests seed it with a fixed value so line selection is deterministic and testable. The engine keeps zero dependencies. Do **not** add the `rand` crate.
- **D-08:** **Never repeat the last line used within a category** (remember the last index per category and re-pick on a match). Falls out naturally from a 4-8 line bank. Not a shuffled deck, not pure random.
- **D-09:** Tone is **sharp and sarcastic**: dry, cutting wit with more bite than cheerful teasing, but clean language and nothing aimed at the player as a person (repo is public). Claude drafts the 4-8 lines per category in this voice; the user can edit the bank afterward.

### Speech bubble and game-over flow
- **D-10:** The speech bubble sits **between the title and the board** in the single centered column (extends D-09 from Phase 2). It reserves a fixed height so the board never jumps when text changes or fades. Scoreboard and controls sit **below** the board.
- **D-11:** Bubble messages **fade out after a few seconds (about 5s)** using a **CSS-only animation** (no Rust timer or async state). A new message restarts the animation. The **game-over message does not fade**, so the final taunt stays visible. The bubble keeps its reserved space while faded.
- **D-12:** The Phase 2 **game-over overlay stays** and keeps its clear result text ("You win!" / "Engine wins!" / "Draw!") and its "Play again" button. The trash talk for the result goes in the **bubble**, not the overlay. This supersedes the "Phase 3 replaces this" note in Phase 2 D-06/D-07 only for the taunt; the explicit result and Play again remain.
- **D-13:** "Play again" on the overlay and the always-visible "New Game" button do the same thing (fresh `Game`, scoreboard untouched, game-start greeting). "New Game" is available mid-game; "Play again" appears only at game over.

### Scoreboard and controls
- **D-14:** The **UI owns the session tally** as a Dioxus signal. The engine does not get a `Scoreboard` type; it only reports each game's outcome via `GameStatus`. The UI increments exactly once when a game transitions to a finished status. Resets on app restart (cross-session stats are out of scope).
- **D-15:** Pressing New Game mid-game **discards the unfinished game with no score change** (matches roadmap criterion 4: scoreboard unchanged by New Game). It does not count as a loss.
- **D-16:** **Reset Scores** is immediate (no confirmation), zeros the tally, and does not touch a game in progress. It is styled as a quiet secondary button to avoid accidental clicks.
- **D-17:** Layout is **one compact row with player-perspective labels: Wins / Draws / Losses** (three counts side by side, fits 320px), with the **New Game** and **Reset Scores** buttons in a row beneath it. New CSS values must be added as custom properties on `:root`, consistent with Phase 2 D-10; stay responsive down to 320px (UI-06).

### Technical debt folded into this phase
- **D-18:** Fix **TD-01 to TD-06** from ROADMAP.md "Technical Debt" while editing the same files: TD-01 (click handler discards `make_move` Result in `board.rs`), TD-02 (duplicate `default_*` tests in `game.rs`), TD-03 and TD-04 (test helpers in `ai.rs` discard `make_move` results and can recurse or loop forever instead of failing), TD-05 (duplicate `body` CSS block), TD-06 (`board_cells` visibility). TD-07/TD-08 (Dependabot: glib, rand) remain deferred to a Dioxus bump.

### Claude's Discretion
- Exact event enum names and the shape of the returned event (as long as it satisfies D-03/D-04 and stays UI-agnostic).
- Bubble visual styling (tail shape, colors) using new `:root` tokens; the ~5s fade duration may be tuned.
- The actual trash-talk strings, within D-09's tone and the 4-8 per category count.
- Whether history/last-index state for D-08 lives in `Game` or in a separate commentary struct, provided `Game` keeps a clean public API.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project requirements and scope
- `.planning/ROADMAP.md` §Phase 3 — goal, success criteria 1-4, requirements ENGI-05/06/07 and UI-02/03/04/05
- `.planning/ROADMAP.md` §Technical Debt — TD-01 to TD-08 (TD-01 to TD-06 folded into this phase)
- `.planning/REQUIREMENTS.md` §v1 Requirements → Engine, UI — exact wording of ENGI-05 to ENGI-07 and UI-02 to UI-05; §v2 for deferred training/difficulty items
- `.planning/PROJECT.md` §Constraints — engine purity, plain CSS only, Dioxus version pin

### Prior phase context
- `.planning/phases/02-playable-game/02-CONTEXT.md` — D-01 to D-11 (Game API shape, Position struct, auto engine reply, overlay, single-column layout, CSS tokens)
- `.planning/phases/02-playable-game/02-UI-SPEC.md` — design contract tokens and layout the new components must extend
- `.planning/phases/02-playable-game/02-REVIEW.md` — source of TD-01 to TD-06 findings

### Technology stack
- `CLAUDE.md` §Technology Stack — pinned Dioxus 0.7.10, state-management primitives (`use_signal`, `use_context`, `use_memo`), engine dependency rules

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- `kiss-tictactoe-engine/src/ai.rs` — minimax with alpha-beta and depth-adjusted scoring; reuse its scoring to judge the user's move (D-02).
- `kiss-tictactoe-engine/src/game.rs` — `Game` with `make_move`, `status`, `board`, `history`, `current_player`; `make_move` already runs the engine reply (Phase 2 D-04).
- `kiss-tictactoe-engine/src/types.rs` — `Position`, `Player`, `GameStatus`, `MoveResult`, `MoveError`; the new event type belongs alongside these.
- `kiss-tictactoe-ui/src/components/` — `board.rs`, `cell.rs`, `game_over_overlay.rs`; `main.rs` holds `use_signal(Game::new)`.
- `kiss-tictactoe-ui/assets/main.css` — `:root` tokens (colors, spacing, typography, `--radius-pill`, overlay tokens) and `.game-container` column layout.

### Established Patterns
- Engine is dependency-free and unit-tested via `rstest`/`proptest` dev-dependencies; new engine logic follows the same test style, and seeded determinism (D-07) is what makes message selection testable.
- UI components take `Signal<Game>` and clone values out before any `write()` to avoid borrow conflicts (Pitfall 1 in Phase 2 research).
- All styling is plain CSS in `assets/main.css` via `manganis`; no magic numbers outside `:root`.

### Integration Points
- `Board` click handler currently ignores the `make_move` result (TD-01); it is where the returned event will be captured and pushed into the bubble state.
- `main.rs` `App` is where the bubble message signal, scoreboard signal, and controls row will be added around `Board`.
- Engine purity gate (`cargo tree -p kiss-tictactoe-engine | grep dioxus` returns nothing) must still hold after adding the event/bank code.

</code_context>

<specifics>
## Specific Ideas

- Sample voice for the bank (tone reference only, not final copy): "Bold choice. Wrong, but bold." / "Take your time. It won't help."
- The engine will have a sarcastic line ready when the user bails with New Game mid-game (flavor only; if it adds an event category beyond ENGI-05/06, treat it as optional).
- The unreachable "user wins" bank is a forward hook for TRAIN-01; leave a short comment in the code explaining why it exists.

</specifics>

<deferred>
## Deferred Ideas

- Winning three cells highlighted after a win (POLSH-01, already v2).
- Engine difficulty levels and training mode with hints (SETUP-02, TRAIN-01 to TRAIN-04, v2).
- Dependabot items TD-07 (glib) and TD-08 (rand): fix by bumping Dioxus when patched releases land.

</deferred>

---

*Phase: 3-Personality*
*Context gathered: 2026-10-14*
