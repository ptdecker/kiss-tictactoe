# Walking Skeleton: Phase 1 — Foundation

**Produced by:** Phase 1 planning
**Used by:** All subsequent phases build on this verified foundation

## What the Skeleton Proves

After Phase 1 completes, the following is verifiably true from a fresh clone:

1. A two-crate Cargo workspace compiles from the repo root: `kiss-tictactoe-engine` (library) and `kiss-tictactoe-ui` (Dioxus 0.7.10 desktop binary)
2. The engine crate has zero Dioxus dependency — the purity boundary is enforced at workspace creation, before any game logic exists
3. A Dioxus desktop app can be built and run, displaying a placeholder UI
4. All open-source collaboration artifacts exist on a public GitHub repository at github.com/ptdecker/kiss-tictactoe

## Architectural Decisions (Locked for All Subsequent Phases)

| Concern | Decision | Rationale |
|---------|-----------|-----------|
| Language | Rust (edition 2024) | Project constraint; Rust 1.85+ required |
| UI Framework | Dioxus 0.7.10, `features = ["desktop"]` (webview/Wry) | Project constraint; webview renderer, not Blitz/native |
| CSS approach | Plain CSS via `manganis::asset!()` macro, `kiss-tictactoe-ui/assets/main.css` | Project constraint: no Tailwind, no CSS frameworks |
| Asset bundling | `manganis` 0.7.10 (Dioxus monorepo) | Required for CSS to survive `dx build` bundling on all platforms |
| Workspace layout | Cargo workspace at repo root; two members: `kiss-tictactoe-engine/`, `kiss-tictactoe-ui/` | Engine reusability; clean engine/UI separation |
| Engine purity | `kiss-tictactoe-engine` has NO dioxus or platform dependency | INFRA-02; enables CLI/web/mobile reuse without recompilation |
| State management | Dioxus 0.7 built-in signals: `use_signal`, `Signal<T>`, `use_context`/`provide_context`, `use_memo` | Signals built into Dioxus 0.7 — no external store library needed |
| Build tool | `dioxus-cli` (`dx`) 0.7.10 | Matches dioxus crate version; provides hot-reload and bundling |
| Platform target | Desktop (macOS primary); mobile-ready architecture | Phase 1 desktop only; swapping `"desktop"` for `"mobile"` is the only change needed later |
| Repository | `github.com/ptdecker/kiss-tictactoe` (public, MIT) | Open-source demonstration project |
| Phase branching | Each phase on its own branch; merge to main only when phase passes all success criteria | D-08; keeps main always in a working state |

## Directory Layout

```
kiss-tictactoe/                    <- Cargo workspace root, git repo root
├── Cargo.toml                     <- [workspace] members = ["kiss-tictactoe-engine", "kiss-tictactoe-ui"]
├── Cargo.lock                     <- Committed (workspace contains a binary crate)
├── .gitignore                     <- /target/, .idea/, *.iml
├── LICENSE                        <- MIT, copyright P. Todd Decker 2026
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md             <- Contributor Covenant v2.1
├── .github/
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.md
│   │   └── feature_request.md
│   └── PULL_REQUEST_TEMPLATE.md
├── kiss-tictactoe-engine/
│   ├── Cargo.toml                 <- lib crate, edition 2024, NO dioxus dep
│   └── src/
│       └── lib.rs                 <- empty stub; Phase 2 defines the API
└── kiss-tictactoe-ui/
    ├── Cargo.toml                 <- dioxus = "0.7.10" + desktop feature; manganis = "0.7.10"
    ├── Dioxus.toml                <- dx project config (produced by dx new, kept intact)
    ├── src/
    │   └── main.rs                <- placeholder: rsx! { div { "kiss-tictactoe — coming soon" } }
    └── assets/
        └── main.css               <- minimal CSS reset; Phase 2 expands this
```

## The One Real Interaction

Running `cargo build --workspace` from the repo root compiles both crates end-to-end. The Dioxus UI binary is produced; the engine library is compiled. No manual steps beyond `cargo install dioxus-cli --version 0.7.10`.

Bonus verification: `cd kiss-tictactoe-ui && dx serve` opens a desktop webview window showing "kiss-tictactoe — coming soon".

## Verification Commands

```bash
# From workspace root after fresh clone + cargo install dioxus-cli --version 0.7.10

# 1. Workspace compiles end-to-end
cargo build --workspace

# 2. dx CLI present and correct version
dx --version    # expect: dx 0.7.x

# 3. Engine purity gate (MUST return empty — zero lines)
cargo tree -p kiss-tictactoe-engine | grep dioxus

# 4. Gitignore working (MUST show no target/ or .idea/ in git status output)
git status
```

## Invariants for Subsequent Phases

These constraints MUST be maintained by every subsequent phase and verified before merge to main:

1. `cargo tree -p kiss-tictactoe-engine | grep dioxus` MUST remain empty after any phase
2. All CSS lives in `kiss-tictactoe-ui/assets/` — no inline styles unless truly dynamic
3. `dioxus` version stays at `0.7.10` — no upgrades without explicit user decision
4. `manganis::asset!()` is the only CSS/asset bundling mechanism
5. Engine types contain no `#[cfg(target_os = ...)]` hacks — pure Rust logic only
6. `Cargo.lock` stays committed — the workspace contains a binary
