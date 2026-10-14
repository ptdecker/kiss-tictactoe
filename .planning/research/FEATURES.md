# Feature Landscape

**Domain:** Rust tic-tac-toe demonstration — user vs. engine, desktop-first, KISS-branded
**Researched:** 2026-10-14
**Confidence:** MEDIUM (table stakes from game conventions; trash talk and training mode from adjacent sources)

---

## Table Stakes

Features users expect. Missing = product feels incomplete or broken.

| Feature | Why Expected | Complexity | Notes |
|---------|--------------|------------|-------|
| 3x3 clickable board | Core game surface | Low | Occupied cells must be rejected silently (no error message needed) |
| Turn enforcement | Players can't skip or double-move | Low | Engine moves immediately after user; no delay needed unless animated |
| Win detection + winning-line highlight | Shows which three cells won; users look for it | Low | Highlight the actual three cells, not just a banner |
| Draw detection | Board full with no winner | Low | Fire after last cell filled; no special case needed |
| Game-over lockout | No moves accepted after win/draw | Low | Board becomes non-interactive until New Game |
| Whose-turn indicator | Users need to know it's their turn | Low | Can be implicit (engine moves instantly, so indicator means "your turn") |
| New Game button | Start fresh without restarting app | Low | Clears board only; separate from score reset |
| Session scoreboard | Wins / losses / draws this session | Low | Three counters; reset separately from board |

**Notes on table stakes:**
- "New Game" and "Reset Scores" are separate actions by convention. Keep them distinct in the UI.
- The winning-line highlight should use shape or position in addition to color so it works for colorblind users.
- Game-over lockout must be immediate — accepting a move after the game ends is a correctness bug users will notice.

---

## Differentiators

Features that set this demonstration apart. Not expected, but valued.

| Feature | Value Proposition | Complexity | Dependencies |
|---------|-------------------|------------|--------------|
| Speech-bubble trash talk | Gives the engine personality; makes the game memorable | Medium | Requires trash talk message bank and game-event triggers |
| Context-aware trash talk | Messages respond to game state, not random | Medium | Requires game-event classification (see categories below) |
| Training mode | Teaches users who don't know optimal play | Medium | Requires weak-AI variant + hint system |
| On-demand hints | Teaches without removing player agency | Low | Requires training mode toggle; hint is opt-in per turn |
| Mode toggle (Normal vs Training) | User controls difficulty explicitly | Low | Drives which engine variant runs |
| Engine "grudging respect" responses | Engine reacts differently when user plays well | Low | Requires move-quality or outcome detection |
| Clean library/UI separation | Architecture demonstrates idiomatic Rust | Medium | Engine as pure library crate, zero Dioxus dependency |

---

## Anti-Features

Features to deliberately NOT build. Each costs complexity with little return for KISS scope.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Persistent cross-session stats | Requires storage, migration, reset UX; not part of v1 scope | Session-only scoreboard, resets on app restart |
| Multiplayer | Contradicts "always vs engine" premise; networking complexity | Keep as explicit out-of-scope |
| Board variants (4x4, 3D) | Complicates engine, UX, win detection; KISS means classic only | 3x3 only; note as potential future work |
| Sound effects / music | Increases asset management; not mentioned in requirements; KISS | Silent game |
| Always-on hints | Removes player agency; research shows players abandon intended moves when hints are shown every turn | Opt-in button per turn |
| Tutorial overlays / onboarding flow | Tic-tac-toe rules are universally known; training mode serves the teaching function | Training mode + in-game hints |
| Settings / preferences menus | Unnecessary complexity; mode toggle covers the only meaningful setting | Single mode toggle (Normal / Training) |
| Animated engine "thinking" delay | Adds fake latency; minimax on 3x3 is instantaneous | Engine moves immediately after user |
| User accounts or profiles | Authentication complexity; this is a local demo | Anonymous always |
| Save / resume game | Mid-game state not valuable for a 9-move game | New Game only |
| Score confirmation dialog ("Are you sure you want to reset?") | Friction over a low-stakes action | Direct reset action; scores recover in ~1 min of play |

---

## Training Mode UX

Research from chess and puzzle game hint systems (adjacent domain — no tic-tac-toe specific sources found):

### Design Principles

**Opt-in, per-turn hints.** A chess community thread found that showing hints automatically on every move leads players to abandon their own intended moves. The hint button should be visible only in training mode and should fire on demand, not automatically. Source: [Chess Up community](https://community.playchessup.com/t/hint-system-isnt-good-a-quick-win-feature-request-w-ai-prompt/4366)

**Specific, not abstract.** A University of Illinois study found that abstract hints are perceived as worse than no hint. For tic-tac-toe, a highlighted cell is the right level of specificity — "consider this cell" rather than "look for a fork." Source: [Washington/CUHK research on hint timing](https://faculty.washington.edu/weicaics/paper/papers/HaoHZC2022.pdf)

**Highlight best moves, not just warn about bad ones.** Nudging toward good play is more instructive than penalizing bad play after the fact.

### Recommended Training Mode Implementation

| Element | Behavior |
|---------|----------|
| Mode toggle | Switches between Normal engine (optimal minimax) and Training engine (deliberately weaker) |
| Weak engine | Depth-limited minimax OR epsilon-greedy (N% random move probability); either approach allows user to win |
| Hint button | Appears only in training mode; always visible but disabled when it's not the user's turn |
| Hint visual | Highlights one recommended cell (the best move for the user) with a distinct but subtle marker |
| Hint cost | No cost — this is a teaching tool, not a competitive hint system |
| Engine trash talk in training | Engine should acknowledge it's "going easy" while still taunting; see trash talk categories |

### Training Mode — What NOT to Do

- Don't auto-highlight hints before the user asks.
- Don't show the entire game tree or evaluation scores.
- Don't penalize hint usage with score penalties.
- Don't use a different board or visual theme — training mode is the same game with coaching overlay.

---

## Trash Talk System — Message Categories

Based on research of chess personality systems (Chess.com, Chessiverse, hobby projects) and the Pine Script chess banter library's event-driven architecture, which fires messages at 8 distinct occasions. Applied to tic-tac-toe:

### Recommended Event Triggers

| Trigger Category | When | Example Tone |
|-----------------|------|-------------|
| Game start | New game begins, before first move | Boastful, confident — "Prepare to be humiliated." |
| Engine about to win | Engine has two in a row with open third | Anticipatory, taunting — "Don't bother looking for a way out." |
| Engine wins | Game over, engine won | Victory lap, contemptuous — "Was that your best?" |
| User about to win (normal mode) | User has two in a row with open third | Surprised, alarmed — "Wait, how did you—" |
| User wins (normal mode) | User won (should be nearly impossible vs optimal) | Disbelief, making excuses — "I let you win." |
| User wins (training mode) | User won in training mode | Grudging acknowledgment with asterisk — "Not bad. For someone I was babying." |
| Draw imminent | Board nearly full, heading to draw | Bored/dismissive — "Fine. A draw. Whatever." |
| Draw result | Game ends in draw | Mutual resignation — "You're not bad. For someone who can't win." |
| User makes a good move | User plays toward a fork or blocks well | Grudging respect — "Hm. Lucky." |
| User makes a bad move | User misses an obvious block or opportunity | Mockery — "Oh no. Did you mean to do that?" |
| Training mode hint used | User requests a hint | Condescending sympathy — "Since you clearly need help…" |
| Engine resigns (never) | N/A — optimal minimax never resigns | N/A |

### Implementation Notes

- **Fire rate:** Don't comment on every move. Prioritize game-state transitions (game start, about to win, game over) over mid-game commentary. Mid-game comments are a bonus, not required.
- **Deterministic selection:** The Pine Script chess library uses seeded arithmetic for determinism so the same game produces the same taunts. This is optional — random selection from a category pool is simpler and acceptable.
- **Static bank preferred over LLM:** For a KISS demonstration, a curated message bank is more predictable, faster, and works offline. No LLM dependency needed. Aim for 4-8 messages per category for variety.
- **Training mode tone shift:** Engine should maintain its personality in training mode but acknowledge the handicap. It still trash-talks — it just trash-talks differently.
- **Speech bubble location:** A fixed position near the engine's "side" of the board; the bubble updates in place, not stacked. This keeps the layout stable.

---

## Feature Dependencies

```
Win detection
  → winning-line cell highlight
  → game-over lockout
  → scoreboard update
  → trash talk: engine wins / user wins trigger

Draw detection
  → game-over lockout
  → scoreboard update
  → trash talk: draw result trigger

Training mode toggle
  → weak engine variant (replaces optimal minimax)
  → hint button appears
  → trash talk: training-mode variants activate

Hint button (training mode only)
  → highlight recommended cell
  → trash talk: hint-used trigger

Game state events
  → trash talk system (all triggers above)
  → speech bubble display
```

---

## Accessibility Considerations

Tic-tac-toe has no timing pressure, which removes one of the hardest accessibility categories. What remains:

| Concern | Requirement | Implementation Approach |
|---------|-------------|------------------------|
| Keyboard navigation | All 9 cells + all buttons reachable via keyboard | Arrow keys within board grid; Tab between board, hint button, mode toggle, new game, reset scores |
| Focus visibility | Focus ring always visible | Never set `outline: none` without replacement; use CSS custom focus styles |
| No keyboard traps | Player can always Tab out of any element | Standard HTML interactive elements avoid this by default |
| Screen reader cell labels | Each cell announces its position and content | `aria-label="Row 1, Column 2: X"` or equivalent; update dynamically |
| Game status announcements | Win, draw, whose turn — announced without user action | `aria-live="polite"` region for status updates |
| Color independence | Winning line highlight doesn't rely on color alone | Add a distinct border, icon, or animation in addition to color change |
| Reduced motion | Win animations don't cause discomfort | `@media (prefers-reduced-motion: reduce)` — skip or simplify any CSS animations |
| Single-character shortcuts | If arrow-key shortcuts are added, make them work only when board has focus | Standard HTML focus scoping handles this |

Sources: [WCAG keyboard criteria via Stark](https://getstark.co/wcag-explained/operable/keyboard-accessible), [Accessibility Maze pattern via A11y Paradise](https://a11y-paradise.onrender.com/reviews/69b6e736f6271573a809254c), [Puzzle Workout arrow-key pattern](https://puzzle-workout-rnesd.kinsta.page/accessibility/)

---

## MVP Feature Priority

### Must-Have (ship without these = broken)

1. 3x3 board with move enforcement and win/draw detection
2. Winning-line highlight and game-over lockout
3. New Game button
4. Session scoreboard (W/L/D)
5. Engine speech bubble with trash talk (game start, win, loss, draw at minimum)

### Should-Have (core differentiators, expected from the project brief)

6. Training mode toggle with weak engine
7. On-demand hint system (training mode only)
8. Full trash talk event coverage (all 10 trigger categories)
9. Mode-aware engine messages (different tone in training mode)

### Can-Defer (polish, post-MVP)

- Animated winning-line draw (static highlight is sufficient for v1)
- Mid-game move-quality trash talk (start/end events are enough)
- Reduced-motion CSS (add when animation is added)
- Keyboard grid navigation with arrow keys (Tab is sufficient for v1; arrow keys are polish)

### Explicitly Out of Scope

- Multiplayer, networking, persistent storage, sound, multiple board sizes — all documented as out of scope in PROJECT.md

---

## Gaps and Open Questions

- **Weak engine strategy:** Depth-limited minimax vs. epsilon-greedy (random move N% of the time) — both are valid; epsilon-greedy is simpler to tune interactively. Recommend epsilon-greedy starting at 30% random moves for training mode.
- **Trash talk authorship:** The message bank needs to be written. The categories above define the structure; the actual strings need creative writing time. Aim for 4-8 messages per category.
- **Hint visual style:** Highlighted cell (border glow? dot? overlay?) — needs design decision before UI phase.
- **Engine move "who goes first":** Convention varies. Recommend user always goes first (X) as the default; this simplifies the initial UX.

---

## Sources

- [Chess Up community — hint system design](https://community.playchessup.com/t/hint-system-isnt-good-a-quick-win-feature-request-w-ai-prompt/4366)
- [CUHK research on hint timing](https://faculty.washington.edu/weicaics/paper/papers/HaoHZC2022.pdf)
- [GDC — Board Game Design Day UX](https://www.gdcvault.com/play/1025684/Board-Game-Design-Day-Designing)
- [Pine Script chess banter library — event-driven design](https://www.tradingview.com/script/ay5yUe8j-Public-Library-ChessTalk/)
- [Chessiverse — chess bot personality categories](https://chessiverse.com/compare/best-chess-bot-personalities)
- [Trash-talking chess AI with Kiro](https://dev.to/omu/trash-talking-chess-ai-with-kiro-4fl1)
- [arXiv 2603.05352 — static personality + dynamic psyche split](https://arxiv.org/abs/2603.05352)
- [Stark — WCAG keyboard accessibility](https://getstark.co/wcag-explained/operable/keyboard-accessible)
- [A11y Paradise — Accessibility Maze screen reader patterns](https://a11y-paradise.onrender.com/reviews/69b6e736f6271573a809254c)
- [Puzzle Workout — arrow-key and focus patterns](https://puzzle-workout-rnesd.kinsta.page/accessibility/)
- [Acagamic — consistent feedback patterns](https://acagamic.com/newsletter/2022/03/22/how-to-provide-consistent-and-clear-feedback)
- [Boston University CS640 — minimax depth experiments](https://cs-people.bu.edu/kaiwkh/cs640/640_P3/P3.html)
- [FreeCodeCamp — minimax for tic-tac-toe](https://www.freecodecamp.org/news/minimax-algorithm-guide-how-to-create-an-unbeatable-ai/)
