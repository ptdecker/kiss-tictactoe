# Roadmap: kiss-tictactoe

## Overview

kiss-tictactoe is built in four vertical phases. Phase 1 establishes the Cargo workspace, toolchain, and GitHub infrastructure. Phase 2 delivers a fully playable game — engine core wired to a Dioxus UI — so a human can sit down and play a complete match. Phase 3 adds engine personality (trash talk message bank, speech bubble, scoreboard, game controls) turning a bare game into the demonstration-worthy experience described in the core value. Phase 4 finalizes user and technical documentation. Each phase branch is merged to main only when the phase is in a working, verifiable state.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: Foundation** - Cargo workspace, .gitignore, toolchain validation, GitHub repo and open-source artifacts
- [ ] **Phase 2: Playable Game** - Engine core (minimax, win/draw detection, clean API) + Dioxus UI wired together; a human can play a complete game
- [ ] **Phase 3: Personality** - Trash talk message bank, speech bubble, session scoreboard, New Game and Reset Scores controls
- [ ] **Phase 4: Documentation** - mdBook user guide, mdBook engine technical manual, cargo doc

## Phase Details

### Phase 1: Foundation
**Goal**: The project has a working Cargo workspace, enforced engine/UI boundary, and a public GitHub repo with all open-source collaboration artifacts.
**Mode:** mvp
**Depends on**: Nothing (first phase)
**Branch**: `phase/1-foundation`
**Requirements**: INFRA-01, INFRA-02, INFRA-03, INFRA-04, INFRA-05
**Success Criteria** (what must be TRUE):
  1. `cargo build --workspace` and `dx --version` succeed from a fresh clone with no manual steps beyond `cargo install dioxus-cli`
  2. `cargo tree -p kiss-tictactoe-engine | grep dioxus` returns no output (engine purity confirmed at workspace creation)
  3. The GitHub repository is public and accessible at `github.com/ptdecker/kiss-tictactoe` with LICENSE (MIT), CONTRIBUTING.md, CODE_OF_CONDUCT.md, bug-report and feature-request issue templates, and a PR template all present
  4. `.gitignore` covers `target/`, `.idea/`, and `*.iml`; none of these paths appear in `git status` output on a fresh build
**Plans**: TBD

### Phase 2: Playable Game
**Goal**: A human can sit down and play a complete tic-tac-toe game — clicking cells, watching the engine respond, and seeing a win, loss, or draw declared — all within a responsive Dioxus desktop UI.
**Mode:** mvp
**Depends on**: Phase 1
**Branch**: `phase/2-playable-game`
**Requirements**: ENGI-01, ENGI-02, ENGI-03, ENGI-04, UI-01, UI-06
**Success Criteria** (what must be TRUE):
  1. A human can play a full game from start to game-over (win, loss, or draw) by clicking board cells; the engine moves immediately after each player move
  2. The engine never loses — `cargo test` AI correctness suite verifies minimax plays optimally for all reachable board states
  3. Win and draw states are detected correctly and the board becomes non-interactive immediately after game-over
  4. `cargo tree -p kiss-tictactoe-engine | grep dioxus` returns nothing after UI integration, confirming the engine purity boundary survives wiring
  5. The UI is usable at phone-width viewport (minimum 320px) using plain CSS; no layout breaks at narrow widths
**Plans**: TBD
**UI hint**: yes

### Phase 3: Personality
**Goal**: The engine has a voice — context-aware trash talk appears in a speech bubble after game events, a session scoreboard tracks W/L/D across games, and the player has explicit New Game and Reset Scores controls.
**Mode:** mvp
**Depends on**: Phase 2
**Branch**: `phase/3-personality`
**Requirements**: ENGI-05, ENGI-06, ENGI-07, UI-02, UI-03, UI-04, UI-05
**Success Criteria** (what must be TRUE):
  1. After every significant game event (game start, engine wins, user wins, draw), a context-appropriate message appears in the engine speech bubble drawn randomly from a bank of 4-8 strings per category
  2. Tactical events (engine about to win, user about to win) also trigger distinct speech-bubble messages from their own per-category message banks
  3. The session scoreboard shows cumulative wins, losses, and draws that persist across games within the session
  4. The "New Game" button resets the board while the scoreboard remains unchanged; the "Reset Scores" button clears the scoreboard independently without affecting a game in progress
**Plans**: TBD
**UI hint**: yes

### Phase 4: Documentation
**Goal**: The project ships complete user-facing and developer-facing documentation: an mdBook user guide explaining how to play and an mdBook engine technical manual covering the public API, data types, and event system.
**Mode:** mvp
**Depends on**: Phase 3
**Branch**: `phase/4-documentation`
**Requirements**: INFRA-06, INFRA-07
**Success Criteria** (what must be TRUE):
  1. `mdbook build docs/user-guide` and `mdbook build docs/engine` both succeed without errors or warnings
  2. The user guide covers how to play, describes the game interface, and explains each control (New Game, Reset Scores)
  3. The engine technical manual documents the public API types (`Game`, `Board`, `GameStatus`, `Player`, `MoveResult`), the event system (all trash-talk trigger categories), and extension points for adding interfaces
  4. `cargo doc --workspace --no-deps` completes without warnings; all public items in `kiss-tictactoe-engine` have doc comments
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. Foundation | 0/TBD | Not started | - |
| 2. Playable Game | 0/TBD | Not started | - |
| 3. Personality | 0/TBD | Not started | - |
| 4. Documentation | 0/TBD | Not started | - |
