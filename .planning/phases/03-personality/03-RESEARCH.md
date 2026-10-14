# Phase 3: Personality - Research

**Researched:** 2026-10-14
**Domain:** Pure-Rust engine event/commentary system + Dioxus 0.7.10 signal-driven UI (speech bubble, scoreboard, controls)
**Confidence:** HIGH (codebase read directly; no new external dependencies; Dioxus patterns follow existing Phase 2 code)

<user_constraints>
## User Constraints (from CONTEXT.md)

### Locked Decisions
- D-01: "User wins" event stays in engine with its own 4-8 line bank; documented as unreachable vs perfect minimax until v2 TRAIN-01. No requirements edit.
- D-02: Good/bad move judge the USER's move via minimax score comparison (user's move vs best available). Good = preserves best achievable outcome (draw). Bad = drops outcome so engine can force a win. Reuse `ai.rs` minimax; no second heuristic evaluator.
- D-03: Exactly one message per turn (user move + engine reply). Fixed priority: game-over, then tactical (engine about to win before user about to win), then move quality (bad before good), then nothing. Event selection deterministic; only line-within-category random.
- D-04: "User about to win" evaluated right after user's move, before engine reply. "Engine about to win" evaluated on board after engine reply. One `make_move` call must compute both (e.g. return event alongside `MoveResult`) without exposing a half-turn board.
- D-05: Game-start event fires on app launch and every New Game. User always X, moves first.
- D-06: Message bank and line selection live in the engine crate. Engine stays free of Dioxus/platform deps.
- D-07: Tiny std-only xorshift-style PRNG, seed passed by caller (UI seeds from system clock; tests fixed seed). Do NOT add `rand`.
- D-08: Never repeat the last line used within a category (remember last index per category).
- D-09: Tone sharp and sarcastic, clean language, nothing aimed at the player as a person. Claude drafts 4-8 lines per category.
- D-10: Bubble between title and board, fixed reserved height. Scoreboard and controls below board.
- D-11: Bubble fades ~5s with CSS-only animation (no Rust timer). New message restarts animation. Game-over message does not fade. Reserved space kept.
- D-12: Game-over overlay stays (result text + "Play again"); taunt goes in bubble.
- D-13: "Play again" and "New Game" do the same thing (fresh `Game`, scoreboard untouched, game-start greeting).
- D-14: UI owns session tally as a Dioxus signal. Engine has no `Scoreboard` type. UI increments exactly once when a game transitions to finished. Resets on restart.
- D-15: New Game mid-game discards unfinished game, no score change.
- D-16: Reset Scores immediate, no confirmation, does not touch game in progress, quiet secondary button.
- D-17: One compact row Wins / Draws / Losses; New Game + Reset Scores row beneath. New CSS values as `:root` custom properties; responsive to 320px.
- D-18: Fix TD-01 to TD-06 in this phase. TD-07/TD-08 deferred.

### Claude's Discretion
- Event enum names and returned event shape (must satisfy D-03/D-04, UI-agnostic).
- Bubble visual styling using new `:root` tokens; fade duration tunable.
- The actual trash-talk strings (D-09 tone, 4-8 per category).
- Whether D-08 last-index state lives in `Game` or a separate commentary struct, provided `Game` keeps a clean public API.

### Deferred Ideas (OUT OF SCOPE)
- Winning-line highlight (POLSH-01); difficulty/training/hints (SETUP-02, TRAIN-*); Dependabot TD-07 (glib) / TD-08 (rand); documentation (Phase 4); persistence across restarts; keyboard nav (POLSH-02).
</user_constraints>

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|------------------|
| ENGI-05 | Lifecycle events: game start, engine wins, user wins, draw | `TalkEvent` enum variants + `Commentator::line`; game-start produced by UI calling `line(GameStart)` on new game |
| ENGI-06 | Tactical events: engine about to win, user about to win, good move, bad move | Threat detector (two-in-line + empty third) on two board snapshots; move-quality via minimax signum comparison |
| ENGI-07 | 4-8 strings per category, random selection | `const` banks per event; xorshift64 with seed; no-repeat index trick |
| UI-02 | Speech bubble after each game event | `SpeechBubble` component + `BubbleState` signal written from click handler |
| UI-03 | Session scoreboard W/L/D | `Scoreboard` component + `Tally` signal |
| UI-04 | New game, score persists | Shared `start_new_game` handler; replaces `Game`, leaves `Tally` |
| UI-05 | Reset scores independent | `Controls` `on_reset_scores` sets `Tally::default()` only |
</phase_requirements>

## Summary

Phase 3 needs no new crates. The engine grows two small modules (event detection + commentary bank/PRNG) that reuse the private `minimax` in `ai.rs`; the UI grows three presentational components and two signals (`BubbleState`, `Tally`) plus a `Commentator` held in a signal. The hard design question is D-04: the "user about to win" check must see the board after X's move but before O replies, inside one `make_move` call. Solve it in the engine by adding a new method that returns `(MoveResult, Option<TalkEvent>)` while leaving `make_move` untouched (all existing tests and doctests keep compiling).

The line-selection state (PRNG + last index per category) goes in a separate `Commentator` struct, not in `Game`. This keeps `Game` `Clone + PartialEq + Eq` (tests compare `Game` values) and keeps events deterministic/pure while randomness is isolated and seedable.

**Primary recommendation:** Add `Game::play(pos) -> Result<Turn, MoveError>` where `Turn { result: MoveResult, event: Option<TalkEvent> }`; make `make_move` a thin wrapper returning `turn.result`. Add `Commentator::new(seed)` with `line(&mut self, TalkEvent) -> &'static str`. UI does the tally increment and bubble update in a single pure helper called from the click handler.

## Architectural Responsibility Map

| Capability | Primary Tier | Secondary Tier | Rationale |
|------------|-------------|----------------|-----------|
| Event classification (threats, move quality, game over, priority) | Engine crate | — | Needs board snapshots + minimax; D-02/D-03/D-04 |
| Message bank + random line selection | Engine crate | — | D-06 |
| PRNG seed source (system clock) | UI | — | Engine has zero platform deps (D-07) |
| Bubble text/serial/is_final state | UI (App signal) | — | Presentational concern |
| Fade animation | Browser/webview CSS | — | D-11 CSS only |
| Session tally | UI (App signal) | — | D-14 |

## Standard Stack

No new dependencies. Use what exists: `dioxus` 0.7.10 (desktop), `manganis` asset, engine dev-deps `proptest` 1.12.0 and `rstest` 0.27.0 (already in `kiss-tictactoe-engine/Cargo.toml`) [VERIFIED: codebase read]. `std::time::SystemTime` in the UI for the seed [CITED: doc.rust-lang.org/std/time/struct.SystemTime.html]. Do not add `rand` (D-07).

### Package Legitimacy Audit
No external packages are installed in this phase. Slopcheck not applicable. Packages removed: none. Flagged: none.

## Architecture Patterns

### Data flow

```
click cell -> Board handler
   -> game.write().play(pos)  --Err--> debug_assert / ignore (TD-01)
        inside engine: place X -> detect_status
           -> snapshot A (after X): user_threat? move_quality (uses board BEFORE X)
           -> if not over: engine reply -> snapshot B: engine_threat?
           -> classify by D-03 priority -> Option<TalkEvent>
   -> (Turn{result,event})
   -> UI helper apply_turn(): 
        if result==GameOver(s): tally.record(s)  (exactly once, here)
        if event.is_some(): line = commentator.write().line(ev); bubble = {msg, serial+1, is_final: game over}
   -> SpeechBubble / Scoreboard / Overlay re-render

New Game / Play again -> game.set(Game::new()); bubble = greeting line(GameStart), serial+1, is_final=false; tally untouched
Reset Scores -> tally.set(Tally::default()); nothing else
```

### Recommended structure
```
kiss-tictactoe-engine/src/
  events.rs      # TalkEvent enum, threat detection, classify() (pure, deterministic)
  commentary.rs  # Commentator { rng state, last: [Option<usize>; N] }, banks (const &[&str])
  ai.rs          # add pub(crate) fn move_quality(before:&Board, pos) -> MoveQuality
  game.rs        # add Turn + Game::play; make_move delegates
kiss-tictactoe-ui/src/
  components/speech_bubble.rs, scoreboard.rs, controls.rs
  state.rs       # BubbleState, Tally (+ unit-testable record()/apply_turn)
```
Export from `lib.rs`: `TalkEvent`, `Turn`, `Commentator`. Everything has `#![warn(missing_docs)]`, so doc every public item.

### Pattern 1: Event classification (D-03/D-04)
```rust
// Priority: game over > engine-about-to-win > user-about-to-win > bad > good > none
fn classify(over: GameStatus, user_threat: bool, engine_threat: bool, q: MoveQuality)
    -> Option<TalkEvent> {
    match over {
        GameStatus::Win(Player::O) => return Some(TalkEvent::EngineWins),
        GameStatus::Win(Player::X) => return Some(TalkEvent::UserWins), // unreachable vs perfect engine; TRAIN-01 hook
        GameStatus::Draw => return Some(TalkEvent::Draw),
        GameStatus::InProgress => {}
    }
    if engine_threat { Some(TalkEvent::EngineAboutToWin) }
    else if user_threat { Some(TalkEvent::UserAboutToWin) }
    else if q == MoveQuality::Bad { Some(TalkEvent::BadMove) }
    else if q == MoveQuality::Good { Some(TalkEvent::GoodMove) }
    else { None }
}
```
Note `GoodMove` would fire on almost every non-threat turn vs perfect play (nearly all moves preserve a draw). That satisfies the requirement; if too chatty the planner may choose to emit `GoodMove` only when the move was the unique/near-unique best, but D-02 defines good as "preserves best outcome" so keep the simple rule. [ASSUMED: acceptable chattiness; user can tune]

### Pattern 2: Move quality via existing minimax (D-02)
`minimax(board, depth, is_maximizing, alpha, beta)` is private in `ai.rs` with X maximizer. For a pre-move board and chosen X move:
```rust
pub(crate) fn move_quality(before: &Board, pos: Position) -> MoveQuality {
    let value_of = |p: Position| { let mut b = *before; b[p.row as usize][p.col as usize] = Some(Player::X);
        minimax(&b, 0, false, i32::MIN, i32::MAX) };   // O to move next
    let best = empty_cells(before).into_iter().map(value_of).max().unwrap();
    let chosen = value_of(pos);
    if chosen.signum() < best.signum() { MoveQuality::Bad } else { MoveQuality::Good }
}
```
Compare `signum()` (outcome class), not raw scores, because scores are depth-adjusted. If X's move ends the game, `minimax` returns the terminal score immediately (fine). Compute quality BEFORE mutating the board. Full-window root call gives an exact value (alpha-beta only prunes inside). 9! tree is trivial in cost. [VERIFIED: ai.rs read]

### Pattern 3: Threat detection
A player "is about to win" if some line (3 rows, 3 cols, 2 diagonals) has exactly two of their marks and one `None`. Reuse the line table from `detection.rs` (read it; expose a `pub(crate)` const if not already). Evaluate user threat on the board right after X placed (and only if game still in progress), engine threat on the board after O replied (and only if in progress). Because a user threat after X is immediately blocked by the engine, do not suppress it for that reason (D-04).

### Pattern 4: Commentator (D-07/D-08)
```rust
pub struct Commentator { state: u64, last: [Option<usize>; TalkEvent::COUNT] }
impl Commentator {
    pub fn new(seed: u64) -> Self { Self { state: if seed == 0 { 0x9E37_79B9_7F4A_7C15 } else { seed }, last: [None; _] } }
    fn next(&mut self) -> u64 { let mut x = self.state; x ^= x << 13; x ^= x >> 7; x ^= x << 17; self.state = x; x }
    pub fn line(&mut self, ev: TalkEvent) -> &'static str {
        let bank = bank(ev); let n = bank.len();
        let mut i = (self.next() % (n as u64 - 1)) as usize;   // pick among n-1 non-last slots
        if let Some(l) = self.last[ev as usize] { if i >= l { i += 1; } } else { i = (self.next() % n as u64) as usize; }
        self.last[ev as usize] = Some(i); bank[i]
    }
}
```
The skip-over-last trick avoids a re-pick loop. Seed 0 is the xorshift fixed point, so remap it (pitfall). Banks must have >= 2 lines (they have 4-8). Derive `Clone, Debug`. [ASSUMED: xorshift64 13/7/17 shifts are the standard Marsaglia triple; unit-test that output is non-constant.]

### Pattern 5: Bubble restart and a11y (Dioxus 0.7)
Keep `role="status" aria-live="polite"` on a STABLE outer wrapper, and put the animated element inside with `key: "{serial}"` so the animation restarts. If the live region itself is remounted, screen readers may not announce the change. [ASSUMED: general ARIA behavior; low cost to follow]
```rust
rsx! {
  div { class: "speech-bubble-region", role: "status", aria_live: "polite",
    div { key: "{serial}", class: "speech-bubble", class: if is_final { "speech-bubble--final" } else { "speech-bubble--fading" },
      span { class: "speech-bubble-text", {message.unwrap_or_default()} } } } }
```
Apply the reserved `min-height`, border, tail on `.speech-bubble` per UI-SPEC. Put `-webkit-line-clamp: 2; display: -webkit-box; -webkit-box-orient: vertical; overflow: hidden` on the inner `.speech-bubble-text` span; the UI-SPEC puts `display:flex` on `.speech-bubble` and line-clamp cannot share `display` with flex, so clamp the inner span. The region wrapper holds the reserved height if the bubble is empty (`visibility: hidden` state). Verify attribute naming (`aria_live`) against dioxus 0.7 rsx; if the compiler rejects it use `"aria-live": "polite"`.

### Pattern 6: Single-point state transition (UI)
Put game-over tally and bubble logic in a pure function so it's unit-testable without a Dioxus runtime:
```rust
pub struct Tally { pub wins: u32, pub draws: u32, pub losses: u32 }
impl Tally { pub fn record(&mut self, s: GameStatus) { match s {
  GameStatus::Win(Player::X) => self.wins += 1, GameStatus::Win(Player::O) => self.losses += 1,
  GameStatus::Draw => self.draws += 1, GameStatus::InProgress => {} } } }
```
Call `record` only from the click handler when `play` returns `MoveResult::GameOver(s)`. Do NOT increment from a `use_effect`/render-time check on `status` (re-renders or effect re-runs would double count; `play` can never return GameOver twice because the guard returns `GameAlreadyOver`).

### Anti-Patterns
- Returning a half-turn board to the UI to detect user threats: compute inside the engine.
- Putting RNG/last-index inside `Game`: breaks `PartialEq`-based tests and API cleanliness.
- A Rust timer/`spawn` for fading (D-11 says CSS only).
- Holding `game.read()` while calling `game.write()` (Pitfall 1 from Phase 2): clone out first; the `Commentator` and `bubble`/`tally` signals should likewise be written in separate statements.

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Move evaluation | New heuristic evaluator | Existing `minimax` in `ai.rs` | D-02 |
| Randomness | `rand` crate | ~10-line xorshift64 | D-07 |
| Fade timing | Timer/async state | CSS `@keyframes` + keyed remount | D-11 |
| Win/draw detection | New checks | `detect_status` | Already tested |

## Common Pitfalls

1. **xorshift seed 0 locks at 0** (every pick returns index 0). Remap 0 to a nonzero constant; test it.
2. **`make_move` return type change breaks 20+ tests/doctests** (`assert_eq!(result, MoveResult::Continuing)`). Add `play`, keep `make_move`.
3. **Quality computed on wrong board.** Must use the board BEFORE X's move; compute prior to placing.
4. **Comparing raw minimax scores.** Depth-adjusted; use `signum()`.
5. **Double tally increment** (see Pattern 6).
6. **Animation does not restart** if the same element persists and only text changes. Use `key` on serial. A game-over message after a faded one is a new serial too.
7. **Faded bubble hides the "no event" turn.** Per UI-SPEC, no event leaves bubble untouched; fine, but don't reset serial.
8. **`line-clamp` + flex conflict** (see Pattern 5).
9. **`@keyframes` cannot use `var()` in percentages**; only duration token (`--bubble-fade-duration`) may be a var.
10. **Strings over 56 chars** clamp/truncate at 320px. Add a unit test asserting every bank string is <= 56 chars and each bank has 4..=8 entries (ENGI-07) and total >= 32.
11. **Missing docs:** `#![warn(missing_docs)]` is on; document new public items or CI-noise results.
12. **Engine purity:** after changes run `cargo tree -p kiss-tictactoe-engine | grep dioxus` (must be empty) and confirm `[dependencies]` still empty.
13. **TD-05 CSS:** merge duplicate `body` blocks (lines 5 and 49) before adding new tokens. **TD-06:** `board_cells` -> `pub(crate)` (tests are in-module so visibility change is safe).
14. **TD-03/04:** replace `let _ = g.make_move(..)` in `ai.rs` test helpers with `.expect("legal move")` / `assert!(.is_ok())` so failure ends the test rather than recursing/looping.
15. **TD-02:** delete the duplicate `default_matches_new` test in `game.rs`.
16. **TD-01:** the handler should `debug_assert!`/match the `Result`; use the new `play` result for the event anyway.

## Draft Message Banks (tone: D-09; each <= 56 chars; planner/executor may refine)

- **GameStart:** "Ready to lose? I'll go easy. Not really." / "Pick a square. Any square. It won't matter." / "Another challenger. How refreshing. And brief." / "Your move, champ. Try to look like you have a plan."
- **EngineWins:** "Another one for the win column. Mine." / "Did you want a rematch, or just a nap?" / "Predictable. Still enjoyable." / "That went about as expected."
- **UserWins** (unreachable until TRAIN-01): "Impossible. I demand a recount." / "Fine. You win. Enjoy it." / "A glitch. That was clearly a glitch." / "Well played. Do not get used to it."
- **Draw:** "A draw. The coward's victory." / "Nobody wins. Nobody learns." / "Stalemate. Try harder next time." / "Even. Barely. Not that I was trying."
- **EngineAboutToWin:** "Two in a row. Your move to lose." / "I smell victory. It smells like me." / "Block this. Or don't. I'm easy." / "Tick tock. The third square is mine."
- **UserAboutToWin:** "Cute. Two in a row. Watch me ruin it." / "Oh, a plan. How adorable. Blocked." / "You almost had something. Almost." / "Nice try. That square is mine now."
- **GoodMove:** "Fine. That was not terrible." / "Careful. You might get the hang of this." / "A sensible move. I'm shocked." / "Hmm. Not a mistake. Yet."
- **BadMove:** "Bold choice. Wrong, but bold." / "Was that on purpose? Please say no." / "I'll take it. Thanks for the gift." / "Oof. Even I felt that one."

Optional extra category (CONTEXT specifics): `NewGameAbandoned` for mid-game New Game. Treat as optional; if added, document and give it 4-8 lines. Recommendation: skip to keep scope at 8 categories (32-64 strings). [ASSUMED]

## Code Examples

CSS fade (literals in keyframes; UI-SPEC Section "Fade behavior"): see `03-UI-SPEC.md`. Add tokens `--focus-ring-width`, `--focus-ring-offset` as well.

Seeding in the UI:
```rust
fn clock_seed() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64).unwrap_or(0x5EED)   // 0 handled inside Commentator::new
}
// in App: let commentator = use_signal(|| Commentator::new(clock_seed()));
```
`use_signal` initializer runs once per component mount, so the seed is stable for the session. [VERIFIED: standard Dioxus signals behavior, matches existing `use_signal(Game::new)`]

Shared new-game handler: define `let start_new_game = move |_| { game.set(Game::new()); let line = commentator.write().line(TalkEvent::GameStart); bubble.set(BubbleState::greeting(line, bubble.read().serial + 1)); };` and pass to both `Controls` and `Board`'s overlay (Board needs `on_play_again` prop forwarded from App, or App passes `game`, `bubble`, `tally`, `commentator` signals to Board). Initial greeting on launch: initialize `BubbleState` with a line drawn from the commentator (use a single `use_signal` closure or `use_hook`), so the bubble is populated from first paint.

## State of the Art

| Old | Current | Impact |
|-----|---------|--------|
| `rand` crate for tiny choice | std-only xorshift (D-07) | zero engine deps |

## Assumptions Log

| # | Claim | Section | Risk if Wrong |
|---|-------|---------|---------------|
| A1 | GoodMove firing on most non-threat turns is acceptable | Pattern 1 | Bubble feels chatty; tune rule |
| A2 | xorshift64 13/7/17 triple is correct | Pattern 4 | Weak randomness; test non-constant |
| A3 | Live region should be stable wrapper | Pattern 5 | Screen-reader announcement quirks |
| A4 | Skip optional NewGameAbandoned category | Banks | Minor flavor loss |
| A5 | `aria_live` attribute spelling in rsx | Pattern 5 | Compile error; fall back to string-literal attr |

## Open Questions

1. **Should `GoodMove` be suppressed on the first move?** Everything preserves a draw on move 1, so it always fires. Recommendation: allow it; D-03 permits.
2. **Where does `Board` get `bubble`/`tally`/`commentator`?** Recommend App passes the signals as props (or a small `AppState` context via `provide_context`). Planner's choice; props are simplest and consistent with `Signal<Game>` now.

## Environment Availability

Rust toolchain, cargo, and `dx` are already in use for Phases 1-2 (per STATE/CLAUDE.md). No new external tools. Note from memory: non-interactive `dx` callers need `RUSTC_WRAPPER=""` (sccache issue). Tests use plain `cargo test`.

## Validation Architecture

### Test Framework
| Property | Value |
|----------|-------|
| Framework | built-in `#[test]` + `rstest` 0.27.0 + `proptest` 1.12.0 (engine); `#[test]` in UI crate |
| Config file | none (Cargo defaults) |
| Quick run command | `cargo test -p kiss-tictactoe-engine` |
| Full suite command | `cargo test --workspace` |

### Phase Requirements to Test Map
| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| ENGI-05 | classify returns EngineWins/Draw/UserWins on terminal statuses; GameStart line exists | unit (rstest) | `cargo test -p kiss-tictactoe-engine events` | Wave 0 |
| ENGI-06 | threat detection both boards; D-04 user-threat seen pre-reply; priority order D-03; move quality good/bad on crafted boards | unit (rstest) | `cargo test -p kiss-tictactoe-engine events` | Wave 0 |
| ENGI-07 | every bank 4..=8 lines, <=56 chars, no empties, total 32..=64; seeded determinism; never repeats consecutively (proptest over seeds/many draws); seed 0 works | unit + proptest | `cargo test -p kiss-tictactoe-engine commentary` | Wave 0 |
| ENGI-06/D-04 | `play` through full games: at most one event per turn; `make_move` still equals `play().result` | integration (tests/gameplay.rs) | `cargo test -p kiss-tictactoe-engine --test gameplay` | extend |
| UI-03 | `Tally::record` maps statuses; no change for InProgress | unit | `cargo test -p kiss-tictactoe-ui state` | Wave 0 |
| UI-04/05 | new-game keeps tally; reset zeroes tally only (pure helper functions) | unit | `cargo test -p kiss-tictactoe-ui state` | Wave 0 |
| UI-02 | bubble state serial increments, is_final set on game over | unit | `cargo test -p kiss-tictactoe-ui state` | Wave 0 |
| Purity | no dioxus in engine | smoke | `! cargo tree -p kiss-tictactoe-engine | grep -q dioxus` | exists |
| Visual (fade, layout 320px, tail, focus ring) | manual-only | manual | `dx serve` in kiss-tictactoe-ui (use `RUSTC_WRAPPER=""` if non-interactive) | — |

### Sampling Rate
- Per task commit: `cargo test -p kiss-tictactoe-engine` (or `-ui` for UI tasks)
- Per wave merge: `cargo test --workspace`
- Phase gate: full suite green + purity check + manual visual check

### Wave 0 Gaps
- [ ] `kiss-tictactoe-engine/src/events.rs` tests, `commentary.rs` tests
- [ ] `kiss-tictactoe-ui/src/state.rs` tests (pure `Tally`/`BubbleState`)
- No new framework install.

## Security Domain

Local single-user desktop game; no network, auth, persistence, or user text input. ASVS V2/V3/V4/V6 do not apply. V5: inputs are click positions already validated by `make_move` (`InvalidPosition`/`CellOccupied`). Rendered bubble text is engine-owned `&'static str` rendered through rsx (auto-escaped). No new threat patterns introduced; no `unsafe`.

## Sources

### Primary (HIGH)
- Codebase: `kiss-tictactoe-engine/src/{ai,game,types,lib}.rs`, `kiss-tictactoe-ui/src/{main.rs,components/*}`, `assets/main.css`
- `.planning/phases/03-personality/03-CONTEXT.md`, `03-UI-SPEC.md`, `.planning/REQUIREMENTS.md`, `.planning/ROADMAP.md` (Technical Debt)

### Secondary / Tertiary
- Training knowledge on xorshift constants and ARIA live regions (tagged [ASSUMED]); no Context7 lookups performed because no new library APIs are introduced beyond patterns already used in Phase 2.

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH (nothing new)
- Architecture: HIGH (derived from read code and locked decisions)
- Pitfalls: MEDIUM-HIGH (Dioxus key-remount and aria attribute spelling should be confirmed on first compile)

**Research date:** 2026-10-14
**Valid until:** 30 days
