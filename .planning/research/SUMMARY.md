# Project Research Summary

**Project:** kiss-tictactoe
**Domain:** Rust desktop game — platform-agnostic engine library + Dioxus 0.7 UI
**Researched:** 2026-10-14
**Confidence:** HIGH (stack and architecture verified against official Dioxus 0.7 docs; features and pitfalls at MEDIUM from community and adjacent-domain sources)

## Executive Summary

kiss-tictactoe is a KISS-branded desktop tic-tac-toe game built in Rust, demonstrating idiomatic library/UI separation. The project has two hard constraints that drive all architectural decisions: (1) the game engine must be a standalone pure-Rust library crate with zero UI-framework dependency, and (2) the UI uses Dioxus 0.7.x with the webview desktop renderer — not the experimental Blitz/Vello GPU renderer. The combination of these constraints creates a workspace with two members: `kiss-tictactoe-engine` (lib crate, zero Dioxus dep) and `kiss-tictactoe-ui` (bin crate, depends on engine and Dioxus). This separation is both the architectural showcase and a practical mobile-readiness strategy — enabling a future feature-flag swap from `dioxus/desktop` to `dioxus/mobile` without touching engine code.

The recommended approach is to build and fully test the engine in isolation before touching the UI at all. The engine's minimax AI, training mode (epsilon-greedy weak variant), win/draw detection, and trash-talk message bank are all pure Rust logic that can be verified with `cargo test` without running a display. Only after the engine API is stable should Dioxus component work begin. State management uses Dioxus 0.7's built-in signals — `use_context_provider` at the App root holding a struct of signals, consumed via `use_context` in child components — no external state crate is needed. The trash-talk speech bubble is a must-have, not polish: it is the primary differentiator that gives the engine personality.

The critical risks are version drift and signal misuse. Dioxus 0.7 broke backward compatibility with pre-0.6 and pre-0.7 patterns (`use_state(cx, ...)` is gone; `use_signal(|| ...)` is the replacement). Any tutorial or LLM suggestion must be verified against 0.7 docs before use. The second risk is Dioxus dependency bleeding into the engine crate — one accidental `use dioxus::...` in the engine destroys the architecture's value proposition and its testability. A `cargo tree -p kiss-tictactoe-engine | grep dioxus` CI check enforces the boundary automatically.

## Key Findings

### Recommended Stack

Dioxus 0.7.10 with the `desktop` feature is the correct choice — it uses the stable Wry/Tao webview renderer matching the project's explicit constraint. The matching `dioxus-cli` (dx) 0.7.10 handles hot-reload development and production bundling. The `manganis` crate (also 0.7.10, from the Dioxus monorepo) provides the `asset!()` macro required for CSS and images to survive `dx build` bundling correctly on all platforms. No external state management crate is needed — Dioxus 0.7 ships signals-based reactivity as a first-class built-in. Documentation uses `mdbook` 0.5.4 at the workspace root (covering both a user guide and engine architecture narrative), with `cargo doc` for API reference.

For testing, `proptest` 1.12.0 (property-based testing for board invariants and AI legality) and `rstest` 0.27.0 (parameterized test cases for known board states) are the only dev-dependencies needed in the engine crate. Standard `#[test]` covers the rest.

**Core technologies:**
- `dioxus` 0.7.10 (desktop feature): primary UI framework — only stable webview desktop renderer matching project constraints
- `dioxus-cli` 0.7.10: build tool and hot-reload — must match dioxus crate version exactly
- `manganis` 0.7.10: compile-time asset bundling — required for CSS to survive `dx build` correctly
- `mdbook` 0.5.4: user guide and architecture docs — standard Rust prose documentation tool
- `proptest` 1.12.0 + `rstest` 0.27.0: engine dev-dependencies — property testing and parameterized cases
- Rust 2024 edition: current standard for greenfield Rust; MSRV follows Dioxus 0.7.x

### Expected Features

The feature landscape has a clear two-tier structure. The table-stakes tier is straightforward — standard tic-tac-toe UI conventions that any player will expect. The differentiator tier (trash talk, training mode) is what makes this a demonstration project rather than a tutorial exercise. The trash talk speech bubble is explicitly classified as a must-have, not deferred polish.

**Must have (table stakes):**
- 3x3 clickable board with turn enforcement and occupied-cell rejection
- Win detection with winning-line cell highlight (shape/border, not color alone)
- Draw detection and game-over lockout (board becomes non-interactive immediately)
- Whose-turn indicator (implicit: engine moves instantly, so indicator means "your turn")
- New Game button (clears board only; separate from score reset)
- Session scoreboard (W/L/D counters; separate Reset Scores action)
- Engine speech bubble with trash talk at minimum for game-start, win, loss, and draw events

**Should have (differentiators):**
- Context-aware trash talk covering all 10+ trigger categories (engine about to win, user about to win, hint used, grudging respect, etc.)
- Training mode toggle (Normal = optimal minimax, Training = epsilon-greedy weak engine)
- On-demand hint system in training mode only — highlights one recommended cell per user request; never auto-shown
- Mode-aware trash talk tone (engine acknowledges the handicap in training mode while still taunting)

**Defer (v2+):**
- Animated winning-line draw (static highlight sufficient for v1)
- Mid-game move-quality trash talk (start/end events are enough for v1)
- Reduced-motion CSS (add when animation is added)
- Keyboard grid navigation with arrow keys (Tab is sufficient for v1)

**Explicitly out of scope:**
- Multiplayer, persistent cross-session stats, board variants, sound effects, animated engine thinking delay, settings menus, save/resume game

### Architecture Approach

The architecture is a two-crate Cargo workspace where the engine crate is a pure Rust library (zero Dioxus dependency, testable with `cargo test` alone) and the UI crate is a Dioxus binary that wraps the engine in signals. All game logic — minimax AI, win/draw detection, training mode, trash-talk message selection, hint computation — lives exclusively in the engine. The UI never implements game rules; it calls engine methods and stores returned values in Dioxus signals. App-wide state (game, score, mode, last-move result for trash talk) is provided via `use_context_provider` at the App root; child components consume it via `use_context` without prop drilling. The workspace uses a `crates/` subdirectory for both members, and mdBook lives at workspace root in `docs/` spanning both crates.

**Major components:**
1. `kiss-tictactoe-engine` (lib crate) — all game state, minimax + training AI, win/draw detection, trash-talk message bank, hint computation; zero platform dependency
2. `kiss-tictactoe-ui` (bin crate) — Dioxus app shell; owns `AppState` context with signals wrapping engine types
3. `App` root component — provides `AppState` context (game signal, score signal, mode signal, last-move-result signal)
4. `Board` + `Cell` components — render the 3x3 grid; dispatch click events via closures capturing the game signal
5. `SpeechBubble` + `Scoreboard` + `ModeToggle` components — read-only or read-write consumers of context signals

### Critical Pitfalls

1. **Outdated Dioxus tutorial patterns** — `use_state(cx, ...)`, `cx.use_hook()`, `static Foo: Component`, or `use dioxus_lib` indicate pre-0.7 code; copy-pasting causes cryptic compile failures. Verify every API against https://dioxuslabs.com/learn/0.7/ before use; pin exact version in Cargo.toml.

2. **Dioxus dependency leaking into engine crate** — any `use dioxus::...` in the engine destroys testability and the architecture showcase. Enforce via CI: `cargo tree -p kiss-tictactoe-engine | grep dioxus` must produce zero output.

3. **Signal guard lifetime errors** — holding a `.write()` guard across an `await` point or holding `.read()` while also calling `.write()` in the same scope causes runtime panics. Prefer `.set()` over `.write()` for simple replacements; clone out of read guards before writing.

4. **Minimax score perspective bug** — using an inconsistent scoring convention (global vs. player-to-move perspective) causes the AI to prefer losing moves. Write unit tests for "AI wins in one," "opponent already won," and "draw" terminal states before integrating the engine into the UI.

5. **Workspace root as a package** — placing `[package]` in the workspace root `Cargo.toml` alongside `[workspace]` creates IDE indexing problems and cluttered builds. The root manifest must be workspace-only.

## Implications for Roadmap

Based on research, the fundamental dependency chain is: workspace/CI foundation → engine correctness → UI shell → game interactivity → personality features → polish and docs. The engine must be correct and fully tested before the UI is built, because the UI has no game logic to fall back on.

### Phase 1: Foundation
**Rationale:** All subsequent work depends on correct workspace structure, CI enforcement of the engine/UI boundary, and proper toolchain setup. Getting these wrong early creates compounding problems — this is the cheapest phase to get right.
**Delivers:** Cargo workspace (two-crate, workspace-only root manifest), `.gitignore` (Rust + JetBrains template), CI pipeline (`--workspace` flags on all cargo commands, `cargo tree | grep dioxus` boundary check), `rustup component add rust-src`, GitHub repo artifacts (CONTRIBUTING.md, issue templates, PR template, CODE_OF_CONDUCT.md, LICENSE)
**Addresses:** Infrastructure required for all features
**Avoids:** Pitfalls 7 (mixed workspace root), 8 (missing --workspace), 13 (.idea/ leaks), 15 (missing GitHub artifacts), 16 (proc-macro server), 17 (duplicate deps)

### Phase 2: Engine Core
**Rationale:** Engine is the dependency for the entire application. No UI work should begin until the engine API is stable and all game-logic tests pass. This phase is pure Rust with no Dioxus involvement — it can run and be tested in any environment.
**Delivers:** `Game` struct with `Board`, `GameStatus`, `Player`, `MoveResult` types; `make_player_move` and `make_engine_move` methods; minimax AI (optimal); win detection (all 8 lines); draw detection (board full, checked after win); game-over lockout semantics in state; session score tracking; proptest + rstest test suite covering board invariants, AI legality, all win/draw positions
**Implements:** All table-stakes game logic; complete engine public API (plain stdlib types only, zero Dioxus)
**Avoids:** Pitfalls 4 (Dioxus dep in engine), 5 (minimax score perspective), 6 (terminal state detection order)
**Research flag:** Standard minimax patterns are well-documented; no research phase needed. The score-perspective unit tests are the gating criterion.

### Phase 3: Engine Personality
**Rationale:** Trash talk, training mode, and hints are engine concerns (pure Rust logic), not UI concerns. Building them before the UI means the engine API (specifically `MoveResult`) is complete when UI work starts — no mid-UI refactors to add return fields.
**Delivers:** Training mode epsilon-greedy weak engine (30% random move probability as starting point); `Mode` enum (Normal/Training); hint computation (`Option<usize>` cell index in training mode); trash-talk message bank (4-8 messages per category across 10+ trigger categories); `MoveResult` carries both trash-talk string and optional hint
**Implements:** All "should-have" engine features; completes the engine public API
**Avoids:** Pitfall 4 (engine must remain Dioxus-free through this phase)
**Research flag:** Message bank requires creative writing time — 40-80 strings across 10+ categories. The categories and example tones are defined in FEATURES.md; authorship is the work. Epsilon-greedy tuning (30% is an estimate) may need playtest adjustment.

### Phase 4: UI Shell
**Rationale:** First Dioxus code. Establishing correct patterns (use_signal, use_context_provider, document::Stylesheet with asset!()) in the shell prevents the hardest-to-fix bugs from propagating through all components. The shell delivers a running desktop app with no game logic yet — skeleton only.
**Delivers:** `kiss-tictactoe-ui` binary crate with Dioxus 0.7.10; `AppState` context struct with signals (game, score, mode, last_move_result); `App` root component with `use_context_provider`; component module structure (board, cell, speech_bubble, scoreboard, mode_toggle placeholders); `document::Stylesheet { href: asset!("assets/main.css") }` in root; `dx serve --platform desktop` working with hot reload
**Uses:** Dioxus 0.7.10 (desktop feature via `[features]` block, not hardcoded in dependency), manganis `asset!()` macro
**Avoids:** Pitfalls 1 (outdated patterns), 3 (signal guards), 11 (raw link for CSS), 18 (conditional hooks)
**Research flag:** No additional research needed — patterns are fully specified in ARCHITECTURE.md with verified Dioxus 0.7 source.

### Phase 5: Core Game UI
**Rationale:** Connect the engine to the UI shell. Delivers the fully playable game including the speech bubble (must-have, not polish). Engine already has the message bank from Phase 3.
**Delivers:** Board grid (CSS Grid, aspect-ratio:1 cells, vw/vh units for mobile-readiness); Cell click handling with signal mutation via `.set()`; winning-line highlight (border/shape in addition to color); game-over lockout (board non-interactive after win/draw); whose-turn indicator; New Game button; session scoreboard (W/L/D); SpeechBubble displaying trash talk from MoveResult; all must-have features complete
**Implements:** All table-stakes features; all must-have features from FEATURES.md
**Avoids:** Pitfalls 2 (form submit prevent_default), 3 (signal guard lifetimes), 12 (CSS class name typos)
**Research flag:** No additional research needed. CSS class name audit should be a code-review step (compare every class name in .rs files against main.css).

### Phase 6: Training Mode UI and Polish
**Rationale:** Training mode and the hint system are the "should-have" differentiators. They can only be built after Phase 5 proves the core game loop is correct.
**Delivers:** ModeToggle component (Normal/Training); training mode activates weak engine variant; hint button (visible only in training mode, disabled when not user's turn); hint cell highlight visual; full trash talk coverage (all 10+ trigger categories); mode-aware engine messages; keyboard Tab navigation across all interactive elements; aria-live regions for game status; aria-label on cells
**Implements:** All "should-have" features from FEATURES.md; WCAG keyboard accessibility requirements
**Avoids:** Pitfall 18 (use_signal must remain unconditional even with mode-conditional rendering)
**Research flag:** Hint visual style (border glow vs. dot vs. background tint) is an unresolved design decision — make the call at the start of this phase. Epsilon-greedy percentage may need tuning based on playtest.

### Phase 7: Documentation
**Rationale:** Documentation can only be finalized once the feature set is stable.
**Delivers:** mdBook at workspace root (`docs/`) with user-guide (how-to-play, training-mode chapters) and engine-architecture sections; all Rust code blocks tagged `rust`; `mdbook test` in CI; `cargo doc --workspace`; `@media (prefers-reduced-motion)` if CSS animation was added; GitHub Community Standards checklist verified complete
**Avoids:** Pitfalls 9 (chapter not in SUMMARY.md), 10 (untagged code blocks), 15 (GitHub artifacts incomplete)
**Research flag:** Standard patterns; no research phase needed.

### Phase Ordering Rationale

- Engine before UI is a hard dependency: the UI has no fallback game logic. Testing the engine in isolation (Phases 2-3) means the UI (Phases 4-6) can trust the API it calls.
- Personality in the engine (Phase 3) before UI shell (Phase 4) ensures `MoveResult` is complete when the UI first imports it — no mid-UI API changes.
- Phase 4 (UI Shell) deliberately delivers zero game logic — just Dioxus patterns proven correct. This is cheaper than fixing anti-patterns discovered six components deep.
- Training mode UI (Phase 6) after core game UI (Phase 5) prevents mode-toggle complexity from complicating the core game loop.
- Documentation last (Phase 7) because it documents the final feature set, not a moving target.

### Research Flags

Phases likely needing deeper research during planning:
- **Phase 3:** Trash talk message bank authorship — categories and triggers are defined in FEATURES.md, but 40-80 actual strings need to be written (creative work, not technical research).
- **Phase 6:** Hint visual style design decision is unresolved. Epsilon-greedy tuning (30% starting point) needs playtest validation.

Phases with standard patterns (skip research phase):
- **Phase 1:** Workspace, CI, and toolchain setup fully specified in STACK.md and PITFALLS.md.
- **Phase 2:** Minimax for 3x3 tic-tac-toe is extremely well-documented; unit test suite is the gating criterion.
- **Phase 4:** Dioxus 0.7 component patterns fully specified in ARCHITECTURE.md with verified source references.
- **Phase 5:** Core game UI patterns are standard; no novel integration challenges.
- **Phase 7:** mdBook and accessibility patterns are standard documentation work.

## Confidence Assessment

| Area | Confidence | Notes |
|------|------------|-------|
| Stack | HIGH | All versions verified against crates.io as of October 2026; Dioxus patterns verified against official 0.7 docs via Context7 |
| Features | MEDIUM | Table stakes from game conventions (HIGH); trash talk and training mode from chess/puzzle adjacent-domain sources (MEDIUM) — no tic-tac-toe specific UX studies found |
| Architecture | HIGH | Primary sources are Dioxus 0.7 official documentation; mdBook workspace-root placement is ecosystem convention (MEDIUM for that specific sub-point) |
| Pitfalls | MEDIUM | Dioxus 0.7 API pitfalls verified against official migration guide; RustRover proc-macro claims from release notes only (LOW); minimax pitfalls from multiple corroborating sources |

**Overall confidence:** MEDIUM-HIGH — the technical stack and architecture are on solid ground; feature priorities and UX decisions have more uncertainty due to sparse tic-tac-toe specific research.

### Gaps to Address

- **Weak engine tuning:** Epsilon-greedy at 30% random moves is an estimate. The right percentage is the one where a novice can win in training mode without it feeling trivially easy. Requires playtest in Phase 6; document the constant as configurable in the engine.
- **Hint visual style:** Three options (border glow, dot overlay, background tint) are viable — none is definitively better. Decide at the start of Phase 6; implement as a single CSS class so it can be changed without touching Rust.
- **Trash talk authorship:** 40-80 strings needed across 10+ categories. Categories and example tones are defined in FEATURES.md. Authorship is creative work, not technical research — allocate time in Phase 3.
- **Who goes first:** FEATURES.md recommends user (X) always goes first as the default. Confirm this is the desired UX before wiring the engine-move trigger in Phase 5.

## Sources

### Primary (HIGH confidence)
- Dioxus 0.7 official docs (via Context7) — component model, signals, platform features, launch API
- crates.io: dioxus 0.7.10, dioxus-cli 0.7.10, manganis 0.7.10, proptest 1.12.0, rstest 0.27.0, mdbook 0.5.4 — version verification
- Dioxus 0.7 migration guide (https://dioxuslabs.com/learn/0.7/migration/to_07/llms.txt) — breaking changes and pitfalls
- Rust 1.85.0 release notes — Rust 2024 edition stabilization

### Secondary (MEDIUM confidence)
- Chess Up community hint-system thread — opt-in per-turn hint design rationale
- University of Washington/CUHK research on hint timing — abstract vs. specific hint effectiveness
- Pine Script chess banter library — event-driven trash talk architecture
- Chessiverse, Chess.com bot personality research — trash talk tone categories
- FreeCodeCamp minimax guide — terminal state detection order
- University of Alberta negamax tic-tac-toe — score perspective convention
- WCAG keyboard criteria (Stark) and Puzzle Workout — accessibility patterns

### Tertiary (LOW confidence)
- JetBrains RustRover release notes — proc-macro server failure mode (single source)
- arXiv 2603.05352 — static personality + dynamic psyche split for AI characters

---
*Research completed: 2026-10-14*
*Ready for roadmap: yes*
