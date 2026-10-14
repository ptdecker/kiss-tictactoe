# Phase 1: Foundation - Context

**Gathered:** 2026-10-14
**Status:** Ready for planning

<domain>
## Phase Boundary

Phase 1 delivers the project skeleton: a Cargo workspace with two crates (`kiss-tictactoe-engine` as a library, `kiss-tictactoe-ui` as a Dioxus 0.7.10 desktop binary), a proper `.gitignore`, toolchain validation confirming `cargo build --workspace` and `dx --version` succeed, and a public GitHub repository with all required open-source collaboration artifacts (LICENSE, CONTRIBUTING.md, CODE_OF_CONDUCT.md, issue templates, PR template).

No game logic. No UI components. No documentation. Just the foundation.

</domain>

<decisions>
## Implementation Decisions

### Dioxus UI Crate Scaffolding
- **D-01:** Use `dx new` to scaffold `kiss-tictactoe-ui/` as a proper Dioxus project — not a manual stub. The scaffold produces `Dioxus.toml`, `assets/` directory, and working build/hot-reload configuration that Phase 2 depends on.
- **D-02:** Run `dx new` targeting `kiss-tictactoe-ui/` as a subdirectory of the workspace root. The workspace `Cargo.toml` wraps both crates. Do not scaffold first and integrate manually — scaffold directly into the workspace.
- **D-03:** Replace the generated hello-world body with a minimal placeholder — `rsx! { div { "kiss-tictactoe — coming soon" } }` — immediately after scaffolding. This keeps the entrypoint clean and avoids throwaway code polluting Phase 2.
- **D-04:** Keep the `assets/` directory and `Dioxus.toml` asset config intact (Phase 2 needs this plumbing). Replace any generated CSS with an empty or minimal reset file — do not delete asset structure entirely.

### GitHub Repository and Open-Source Artifacts
- **D-05:** Create the GitHub repository via `gh repo create` in the plan (CLI-automated). No manual setup required.
- **D-06:** Community files (CONTRIBUTING.md, CODE_OF_CONDUCT.md, issue templates, PR template) use standard templates — Contributor Covenant CoC, standard GitHub templates. This is a demonstration project; generic quality is appropriate.
- **D-07:** LICENSE: MIT.

### Phase Branch Setup
- **D-08:** Phase 1 work happens on `phase/1-foundation` from the first commit. No work lands on `main` before the phase branch is created. Merge to `main` only when all Phase 1 success criteria pass.

### Claude's Discretion
- Engine crate scaffold: minimal lib crate (Cargo.toml + `src/lib.rs` with empty public API stub). No special scaffolding needed — the engine is a standard Rust library.
- .gitignore content: standard Rust entries (`target/`) plus JetBrains RustRover entries (`.idea/`, `*.iml`). Use a well-known .gitignore source (e.g., gitignore.io) as the base.
- Engine purity verification command (`cargo tree -p kiss-tictactoe-engine | grep dioxus`): run as a plan step immediately after workspace creation, before any other work. It must return empty output.

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Project Requirements and Scope
- `.planning/ROADMAP.md` §Phase 1 — Goal, success criteria, requirements (INFRA-01 through INFRA-05), branch name (`phase/1-foundation`)
- `.planning/REQUIREMENTS.md` §v1 Requirements → Infrastructure — Detailed specs for INFRA-01 through INFRA-05
- `.planning/PROJECT.md` §Constraints — Engine purity rule, Dioxus version pin, plain CSS, webview renderer

### Technology Stack and Versions
- `CLAUDE.md` §Technology Stack — Exact pinned versions: `dioxus = "0.7.10"`, `dioxus-cli = "0.7.10"`, `manganis = "0.7.10"`. Install command for `dx` CLI. Workspace Cargo.toml structure. `Dioxus.toml` config reference.

</canonical_refs>

<code_context>
## Existing Code Insights

### Reusable Assets
- None — greenfield project. No existing components, hooks, or utilities.

### Established Patterns
- None yet — this phase establishes the patterns that all subsequent phases follow.

### Integration Points
- The workspace `Cargo.toml` created in this phase is the integration point for all future crates.
- `Dioxus.toml` and `assets/` structure created here must be compatible with Phase 2's CSS and asset additions.

</code_context>

<specifics>
## Specific Ideas

- The placeholder app entrypoint should use: `rsx! { div { "kiss-tictactoe — coming soon" } }` — minimal, intentional, easy to remove in Phase 2.
- Engine crate starts with an empty `src/lib.rs` (just a `pub fn placeholder() {}` or completely empty is fine — Phase 2 defines the API).
- The engine purity check (`cargo tree -p kiss-tictactoe-engine | grep dioxus`) is a hard gate and should be a numbered plan step, not an afterthought.

</specifics>

<deferred>
## Deferred Ideas

None — discussion stayed within phase scope.

</deferred>

---

*Phase: 1-Foundation*
*Context gathered: 2026-10-14*
