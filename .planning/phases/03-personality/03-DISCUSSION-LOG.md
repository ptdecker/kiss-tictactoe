# Phase 3: Personality - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-10-14
**Phase:** 3-Personality
**Areas discussed:** What triggers a message, Message bank & randomness, Speech bubble & game-over flow, Scoreboard & controls, Technical debt scope

---

## What triggers a message

**"User wins" is unreachable vs perfect minimax**

| Option | Description | Selected |
|--------|-------------|----------|
| Keep in engine, mark unreachable | Define the event and bank for v2 training mode; note it never fires today | ✓ |
| Drop it for now | Ship only reachable events; edit ENGI-05 | |
| Reframe as 'you survived' on draw | Grudging-respect line on a draw | |

**User's choice:** Keep it in the engine, mark unreachable.

**How a user move is judged good/bad**

| Option | Description | Selected |
|--------|-------------|----------|
| Minimax score comparison | Compare chosen move to best available | ✓ |
| Simple heuristics | Center/block/fork rules | |
| You decide | Claude picks | |

**User's choice:** Minimax score comparison.

**Which message when several events are true**

| Option | Description | Selected |
|--------|-------------|----------|
| One message, fixed priority | Game-over > tactical > move quality | ✓ |
| One message, random among true events | Variety, less predictable | |
| Two messages per turn | Chattier, bigger bubble | |

**User's choice:** One message, fixed priority.

**When "user about to win" fires**

| Option | Description | Selected |
|--------|-------------|----------|
| After engine reply, only if threat still stands | Rare; matches the visible board | |
| Right after the user's move, before the engine replies | Fires often; "nice try" | ✓ |

**User's choice:** Right after the user's move, before the engine replies.
**Notes:** Implies the engine API must compute events across both halves of a turn within one `make_move` call (D-04).

---

## Message bank & randomness

**Where the bank lives**

| Option | Description | Selected |
|--------|-------------|----------|
| In the engine crate | Reusable by any UI; documented in engine manual | ✓ |
| Engine emits IDs only; UI owns text | Smaller engine, duplicated text per UI | |

**User's choice:** In the engine crate.

**Randomness source**

| Option | Description | Selected |
|--------|-------------|----------|
| Tiny std-only PRNG, seed passed in | Zero deps, deterministic tests | ✓ |
| Add the `rand` crate | Standard but first runtime dependency; rand has open alert | |
| Caller supplies random index | Pure engine, pushes logic to every UI | |

**User's choice:** Tiny std-only PRNG, seed passed in.

**Repeat avoidance**

| Option | Description | Selected |
|--------|-------------|----------|
| Never repeat last line in a category | Cheap, small bank feels bigger | ✓ |
| Cycle through a shuffled deck | Even coverage, more state | |
| Pure random | Repeats accepted | |

**User's choice:** Never repeat the last line in a category.

**Tone**

| Option | Description | Selected |
|--------|-------------|----------|
| Playful smug, family-friendly | Cocky, gentle ribbing | |
| Sharp and sarcastic | Drier, meaner wit, clean language | ✓ |
| Mix by event | Tone varies with outcome | |

**User's choice:** Sharp and sarcastic.

---

## Speech bubble & game-over flow

**Bubble placement**

| Option | Description | Selected |
|--------|-------------|----------|
| Between title and board | Fixed height, no board jump | ✓ |
| Below the board | Caption style; risk of off-screen on 320px | |

**User's choice:** Between title and board.

**Persistence**

| Option | Description | Selected |
|--------|-------------|----------|
| Stays until next message | No timers or animation | |
| Fades out after a few seconds | Livelier, adds timer/animation state | ✓ |

**User's choice:** Fades out after a few seconds.
**Notes:** Chosen over the recommended option; mechanism settled in the next question.

**How the fade works**

| Option | Description | Selected |
|--------|-------------|----------|
| CSS-only animation, final result stays | No Rust timer; game-over message persists | ✓ |
| CSS-only animation, everything fades | Final taunt can vanish | |
| Rust timer controls visibility | Full control, async code for a cosmetic effect | |

**User's choice:** CSS-only animation, final result stays.

**Phase 2 overlay**

| Option | Description | Selected |
|--------|-------------|----------|
| Keep overlay for the result, taunt in bubble | Result text and Play again stay | ✓ |
| Remove the overlay entirely | Result only in bubble and scoreboard | |
| Overlay shows the taunt instead of result | Loses explicit result text | |

**User's choice:** Keep overlay for the result, trash talk goes in the bubble.

---

## Scoreboard & controls

**Tally ownership**

| Option | Description | Selected |
|--------|-------------|----------|
| UI owns it as a Dioxus signal | Engine stays single-game | ✓ |
| Engine provides a Scoreboard type | Reusable, more engine surface | |

**User's choice:** UI owns it as a Dioxus signal.

**New Game mid-game**

| Option | Description | Selected |
|--------|-------------|----------|
| Game discarded, score unchanged | Matches roadmap criterion 4 | ✓ |
| Count it as a loss | Discourages quitting, contradicts criterion | |

**User's choice:** Nothing, the game is just discarded.

**Reset Scores behavior**

| Option | Description | Selected |
|--------|-------------|----------|
| Immediate reset, no confirmation | Low stakes, quiet secondary button | ✓ |
| Two-step confirm | Safer, adds timer state | |

**User's choice:** Immediate reset, no confirmation.

**Layout**

| Option | Description | Selected |
|--------|-------------|----------|
| One compact row (Wins/Draws/Losses) then controls | Fits 320px | ✓ |
| Stacked list with a header | Roomier, taller | |

**User's choice:** One compact row, controls beneath.

---

## Technical debt scope

| Option | Description | Selected |
|--------|-------------|----------|
| Fold in TD-01 to TD-06 | Files already being edited | ✓ |
| Only TD-03 and TD-04 | Just the hang risks | |
| Leave all for later | Phase strictly about personality | |

**User's choice:** Fold in TD-01 to TD-06.

---

## Claude's Discretion

- Event enum names and returned event shape (within D-03/D-04, UI-agnostic).
- Bubble visual styling and exact fade timing.
- Drafting of the actual trash-talk strings.
- Where last-index state for repeat avoidance lives.

## Deferred Ideas

- POLSH-01 winning-line highlight (v2).
- Difficulty levels and training mode with hints (v2).
- TD-07 / TD-08 Dependabot items (fix by bumping Dioxus).
