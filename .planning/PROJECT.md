# kiss-tictactoe

## What This Is

A Rust tic-tac-toe demonstration project showcasing clean library/UI separation. The game engine lives in a platform-agnostic library crate (`kiss-tictactoe-engine`) with a clean API; a Dioxus 0.7.x desktop app (`kiss-tictactoe-ui`) provides the interface. Always user vs. engine — no multiplayer, no networking.

## Core Value

A fully playable, good-looking tic-tac-toe game where the engine trash talks you and can teach you how to play — demonstrating idiomatic Rust architecture to a friend.

## Requirements

### Validated

(None yet — ship to validate)

### Active

**Engine Library (`kiss-tictactoe-engine`)**
- [ ] Game state management (board, current player, win/draw detection)
- [ ] Clean public API usable from any UI or interface
- [ ] Optimal AI engine (minimax — always draws with perfect play)
- [ ] Training mode engine (intentionally suboptimal, allows user to win)
- [ ] Training mode hints (surfaces good moves to the user)
- [ ] Trash talk system (context-aware messages: winning, losing, draw imminent)

**UI (`kiss-tictactoe-ui`, Dioxus 0.7.x)**
- [ ] 3×3 game board, clickable cells
- [ ] Engine speech bubble displaying trash talk
- [ ] Session scoreboard (wins / losses / draws)
- [ ] Mode toggle: Normal vs Training
- [ ] Desktop-first, mobile-ready responsive layout (plain CSS)
- [ ] New game / reset action

**Project Infrastructure**
- [ ] Rust workspace with two crates (engine + ui)
- [ ] `docs/` with mdBook user guide (how to play) and engine technical manual (API reference)
- [ ] Standard GitHub open-source collaboration artifacts (LICENSE, CONTRIBUTING.md, CODE_OF_CONDUCT.md, issue templates, PR template)
- [ ] Public GitHub repo synced from local
- [ ] `.gitignore` covering Rust build artifacts, JetBrains RustRover IDE files

### Out of Scope

- Multiplayer / second human player — always user vs engine
- Networking — local game only
- Persistent cross-session statistics — scoreboard resets on app restart
- Mobile builds in v1 — architecture is mobile-ready but only desktop ships first
- Multiple board sizes or game variants — 3×3 classic only

## Context

- Demonstration project for a friend; clarity and clean architecture matter as much as functionality
- "KISS" branding (Keep It Simple, Stupid) — project lives under `ptdecker`'s GitHub account
- Engine must have zero Dioxus dependency so it can be reused with other interfaces (CLI, web, mobile)
- Dioxus 0.7.x APIs changed significantly from 0.5/0.6 — all code must be verified against 0.7 docs, not training data
- Developer uses JetBrains RustRover for local development

## Constraints

- **Tech Stack**: Rust + Dioxus 0.7.x — no exceptions; webview renderer (not Blitz/Native)
- **UI Styling**: Plain CSS only — no Tailwind, no external CSS frameworks
- **Platform**: Desktop (macOS/Windows/Linux) primary; mobile-ready architecture required
- **Engine purity**: `kiss-tictactoe-engine` must have no Dioxus or platform dependency
- **Dioxus version**: Pin exact Dioxus and dioxus-cli versions in Cargo.toml; verify all APIs against 0.7 docs

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust workspace (two crates) | Engine reusability; clean separation of logic and UI | — Pending |
| Dioxus 0.7.x with default webview | User directive; no experimental renderers | — Pending |
| Plain CSS (no Tailwind) | User preference; reduces build tooling complexity | — Pending |
| Speech bubble for trash talk | User preference; visually clear engine personality | — Pending |
| Training mode = weaker play + hints | User preference; teaches while still engaging | — Pending |
| mdBook for documentation | Standard Rust ecosystem doc tooling; supports user guide + API manual | — Pending |
| Public GitHub repo | Open-source demonstration; solicits collaboration | — Pending |

## Evolution

This document evolves at phase transitions and milestone boundaries.

**After each phase transition** (via `/gsd-transition`):
1. Requirements invalidated? → Move to Out of Scope with reason
2. Requirements validated? → Move to Validated with phase reference
3. New requirements emerged? → Add to Active
4. Decisions to log? → Add to Key Decisions
5. "What This Is" still accurate? → Update if drifted

**After each milestone** (via `/gsd:complete-milestone`):
1. Full review of all sections
2. Core Value check — still the right priority?
3. Audit Out of Scope — reasons still valid?
4. Update Context with current state

---
*Last updated: 2026-10-14 after initialization*
