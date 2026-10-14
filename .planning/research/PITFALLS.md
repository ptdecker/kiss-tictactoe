# Domain Pitfalls

**Domain:** Rust desktop app — Dioxus 0.7.x UI + platform-agnostic engine library
**Researched:** 2026-10-14
**Confidence:** MEDIUM — Dioxus 0.7.x API findings verified against official docs; minimax and workspace pitfalls corroborated by multiple sources; some RustRover claims are from release notes only

---

## Critical Pitfalls

### Pitfall 1: Treating Old Tutorial Code as Dioxus 0.7

**What goes wrong:** Pre-0.6 tutorials use the `cx` closure argument and `cx.use_hook()` / `use_state(cx, || ...)` syntax. Pre-0.7 tutorials may use `dioxus-lib` or a removed prelude item. Copying this code produces compile errors that are hard to diagnose without knowing a version change is the cause.

**Why it happens:** The Dioxus changelog is dramatic — 0.5 to 0.6 rewrote the component model, and 0.7 trimmed the prelude and removed `dioxus-lib`. Web tutorials, blog posts, and LLM training data lag behind by months to years.

**Warning signs:**
- `use_state(cx, ...)` instead of `use_signal(|| ...)`
- Component defined as `static Foo: Component = |cx| { ... }` instead of `#[component] fn Foo()`
- `use dioxus_lib::prelude::*` in any example you're copying
- `use_drop`, `Runtime`, `queue_effect`, or `provide_root_context` imported from prelude (removed in 0.7)

**Consequences:** Compile errors, sometimes cryptic because the removed identifiers are gone rather than deprecated with a message.

**Prevention:** Pin Dioxus in Cargo.toml to an exact 0.7.x version. Verify every API against the 0.7 docs at https://dioxuslabs.com/learn/0.7/. Do not copy code without cross-checking the version it was written for.

**Detection:** `cargo check` immediately after any copy-paste from a tutorial. Watch for the identifiers listed above.

**Phase to address:** Phase 1 (project setup). Pin versions in `Cargo.toml` on day one; add a comment citing the doc URL.

---

### Pitfall 2: Form Submission Default Inverted in 0.7

**What goes wrong:** In Dioxus 0.6, forms were blocked by default — an empty `onsubmit: |e| {}` handler prevented the default browser submission. In 0.7, this default was flipped: forms now submit by default, and you must call `e.prevent_default()` to stop them. 0.6 patterns silently do the wrong thing at runtime.

**Warning signs:** A button that was supposed to trigger in-app logic instead causes the app to refresh or submit to a URL.

**Consequences:** Silent runtime misbehavior, not a compile error. Hard to notice if you're not testing form submit paths.

**Prevention:** Any `onsubmit` handler must include `e.prevent_default()` when the intent is to keep navigation in-app.

**Detection:** End-to-end test that clicks submit buttons and verifies no page reload occurs.

**Phase to address:** Whichever phase adds the game control buttons (New Game, Mode Toggle). Call `prevent_default()` proactively.

---

### Pitfall 3: Signal Guard Lifetime Errors

**What goes wrong:** Holding a `.write()` guard (returned by `Signal::write()`) across an `await` point causes a runtime panic or borrow error. Holding a `.read()` guard and also calling `.write()` in the same scope causes a compile error or runtime panic depending on the access path used.

**Warning signs:**
- `panicked at 'already borrowed'` in the console
- Clippy warning about holding a `RefCell` borrow across an `await`
- Code that reads a signal value and then writes to it in the same closure

**Consequences:** Panics at runtime, potentially mid-game. The game becomes unresponsive.

**Prevention:**
- Clone the value out of the read guard before you need to write: `let val = *state.read(); state.set(val + 1);`
- Drop guards explicitly with `drop(guard)` before awaiting.
- Prefer `.set()` over `.write()` for simple replacements.

**Detection:** Clippy lint `await_holding_refcell_ref`. Run `cargo clippy` in CI.

**Phase to address:** Phase where Dioxus state is first introduced (game board interactivity). Also applies to any async event handler.

---

### Pitfall 4: Dioxus Dependency Leaking into Engine Crate

**What goes wrong:** The engine crate (`kiss-tictactoe-engine`) accidentally gains a Dioxus or platform-specific dependency — even transitively — breaking the promise that it can be used from a CLI, a different UI framework, or a future mobile build.

**Warning signs:**
- `use dioxus::...` appears anywhere in `kiss-tictactoe-engine/src/`
- A Dioxus type appears in any public engine API (e.g., `Signal<Board>` as a return type)
- `Cargo.toml` for the engine lists `dioxus` or `dioxus-desktop`

**Consequences:** The engine can no longer compile without the Dioxus feature tree. Testing the engine standalone requires a GUI renderer. The whole "architecture showcase" value proposition is compromised.

**Prevention:**
- The engine's `Cargo.toml` must list zero Dioxus dependencies. Enforce this in CI with `cargo tree -p kiss-tictactoe-engine | grep dioxus` — the command should produce no output.
- The engine's public API must use only stdlib types (`Vec`, `Option`, enums, structs).
- Write engine unit tests that run with `cargo test -p kiss-tictactoe-engine` in isolation.

**Detection:** `cargo tree -p kiss-tictactoe-engine --edges all | grep dioxus` in CI.

**Phase to address:** Phase 1 (workspace setup) — establish the rule before any code is written. Phase 2 (engine implementation) — enforce via CI job.

---

## Moderate Pitfalls

### Pitfall 5: Minimax Score Perspective Bug

**What goes wrong:** Classic minimax uses +1 for the first player winning and -1 for the second player winning, from a fixed global perspective. If you implement negamax (which is cleaner for alternating players) but forget to negate leaf scores by depth parity, the AI plays as if it is trying to lose.

**Specific failure modes:**
- AI prefers losing moves over winning ones.
- AI never blocks an opponent win because it scores it positively.
- Training mode (intentionally weak) is accidentally correct because the sign is flipped.

**Warning signs:** AI consistently makes obviously bad moves; unit tests show it choosing losing positions.

**Prevention:**
- Choose one scoring convention and document it: either "score is always from X's perspective" or "score is from the perspective of the player to move." Pick the latter (negamax-style) and negate consistently.
- Terminal states: check win/loss before checking for draws. A board that is full and has a winner is a win, not a draw.
- Write unit tests for these exact positions before writing the search:
  - AI to move, wins in one → score is maximum
  - AI to move, opponent has won already → score is minimum
  - Board full, no winner → draw score (0)

**Detection:** A unit test suite with at least the three positions above. Also test that the AI never loses when given optimal play against itself.

**Phase to address:** Phase 2 (engine implementation). Do not integrate into UI until these tests pass.

---

### Pitfall 6: Minimax Terminal State Detection Order

**What goes wrong:** Checking for a draw (board full) before checking for a win means a winning position on the last move is scored as a draw. The game ends correctly visually but the AI's search tree has the wrong value for that node.

**Prevention:** Always check win conditions before checking draw conditions. Order matters.

**Detection:** Unit test: fill a board where the last cell completes a win. Verify the returned result is a win, not a draw.

**Phase to address:** Phase 2 (engine implementation).

---

### Pitfall 7: Cargo Workspace Root as a Package (Mixed Root Anti-Pattern)

**What goes wrong:** Placing a `[package]` section in the workspace root `Cargo.toml` alongside `[workspace]` means the root is both a workspace manifest and a package. This creates confusion about what `cargo build` at the root builds, causes IDE indexing problems, and makes it harder to add future members cleanly.

**Warning signs:** Workspace `Cargo.toml` contains both `[workspace]` and `[package]`.

**Prevention:** Keep the workspace root `Cargo.toml` as a pure workspace manifest (`[workspace]` only, no `[package]`). Put both crates as members under their own directories.

**Detection:** Inspect `Cargo.toml` at workspace root — it should have no `[package]` section.

**Phase to address:** Phase 1 (workspace setup). Get this right before creating any crates.

---

### Pitfall 8: Missing `--workspace` Flag for Multi-Crate Commands

**What goes wrong:** Running `cargo test`, `cargo clippy`, `cargo fmt`, or `cargo check` without `--workspace` only operates on the crate in the current directory. In CI, this silently misses the other crate.

**Warning signs:** Tests pass in CI but a broken crate was never compiled.

**Prevention:**
- In CI: always pass `--workspace` to all cargo commands.
- Locally: prefer running from workspace root with `--workspace`, or alias it.

**Detection:** Run `cargo test --workspace` in CI. Add `cargo clippy --workspace -- -D warnings`.

**Phase to address:** Phase 1 (CI setup). Bake `--workspace` into CI scripts from day one.

---

### Pitfall 9: mdBook Chapter Not in SUMMARY.md

**What goes wrong:** Adding a new `.md` file to `src/` without adding it to `SUMMARY.md` means the page is unreachable from the book's table of contents. Link checkers will also report broken links if any other page references it by path.

**Warning signs:** A chapter file exists but the book renders without it. CI link checks fail on the new file.

**Prevention:** Every new chapter file must be added to `SUMMARY.md` in the same commit. Treat `SUMMARY.md` as the authoritative table of contents — if it's not there, it doesn't exist.

**Detection:** `mdbook build` succeeds but the chapter is absent from the rendered output. Add a CI step: `mdbook build && mdbook test`.

**Phase to address:** Whichever phase writes the mdBook. Keep `SUMMARY.md` in sync as the book grows.

---

### Pitfall 10: mdBook Code Blocks Not Tagged for Doc Tests

**What goes wrong:** Code blocks in mdBook chapters are only compiled and tested by `mdbook test` if the fence tag is `rust` (or `rs`). Blocks with no tag, or with `text`, `bash`, `toml`, etc., are skipped silently. A Rust example that stops compiling goes undetected.

**Warning signs:** Rust code blocks in chapters use no language tag, or use a non-`rust` tag.

**Prevention:**
- Tag all Rust code blocks with `rust`.
- Tag non-Rust blocks explicitly (`bash`, `toml`, `text`) so it is clear which are intentionally skipped.
- Run `mdbook test` in CI.

**Detection:** `mdbook test` fails when a `rust`-tagged block stops compiling.

**Phase to address:** Documentation phase. Set up `mdbook test` in CI when the book is first created.

---

### Pitfall 11: Dioxus External Stylesheet via `link` Instead of `document::Stylesheet`

**What goes wrong:** Using a raw `link { rel: "stylesheet", href: "..." }` element works in the webview but is not pre-loadable during SSR and bypasses Dioxus's asset pipeline. Using `document::Stylesheet { href: asset!("assets/main.css") }` is the correct 0.7 pattern.

**Warning signs:** Stylesheet is loaded with a raw `link` element; `asset!()` macro is not used for CSS files.

**Consequences:** Styles may flash on load, or may not be correctly bundled by `dx build` for distribution.

**Prevention:** Use `document::Stylesheet { href: asset!("assets/main.css") }` in the app root component. Use `asset!()` for all static assets (CSS, images).

**Detection:** Review the root component for raw `link` elements during code review.

**Phase to address:** Phase where UI shell is first built (initial Dioxus setup).

---

### Pitfall 12: CSS Class Name Typos Are Invisible at Compile Time

**What goes wrong:** CSS class names in Dioxus RSX are strings. A typo like `"gme-board"` instead of `"game-board"` compiles and runs without error — the styling simply does not apply.

**Warning signs:** A styled component appears unstyled; inspecting the DOM shows the wrong class name.

**Prevention:**
- Keep class names short and consistent. Define them as constants if reused.
- Do a final pass during styling review: compare every class name in `.rs` files against the CSS file.
- Consider a CSS module approach for scoped names if the stylesheet grows large.

**Detection:** Visual inspection during development. Browser devtools class name mismatch.

**Phase to address:** UI styling phase. Establish class naming conventions at the start.

---

### Pitfall 13: `.idea/` Files Causing Merge Conflicts or Leaking Local State

**What goes wrong:** Committing the wrong subset of `.idea/` files causes merge conflicts on session state files (`workspace.xml`, `tasks.xml`) and can expose machine-specific paths or credentials in run configurations.

**Warning signs:** `workspace.xml` or `shelf/` directories appear in `git diff` after routine IDE use.

**Specific files to ignore (personal, machine-specific):**
- `workspace.xml`
- `tasks.xml`
- `shelf/`
- `usage.statistics.xml`
- `dataSources.local.xml`
- `*.iml` (contains machine-specific module paths in some configurations)

**Specific files that are safe to commit (shared project settings):**
- `misc.xml` (SDK version)
- `modules.xml` (module list)
- `vcs.xml` (VCS root)
- `codeStyles/` (formatting rules)
- `inspectionProfiles/` (Clippy / inspection settings)

**Prevention:** Use the JetBrains `.gitignore` template from `github/gitignore` as the base. Verify which files are committed before the first push.

**Detection:** `git status` after routine IDE use — only committed files should appear as modified.

**Phase to address:** Phase 1 (project setup). Get `.gitignore` right before the first commit.

---

## Minor Pitfalls

### Pitfall 14: Cargo.lock Committed for Library Crates

**What goes wrong:** The Rust convention is to commit `Cargo.lock` for binary crates (reproducible builds) and omit it for library crates (so consumers can resolve their own compatible versions). Committing `Cargo.lock` for `kiss-tictactoe-engine` is incorrect if it is ever published, and adds noise to diffs.

**Prevention:** `.gitignore` the engine crate's `Cargo.lock` if one exists. Commit the workspace-level `Cargo.lock` (which covers the binary crate). For a workspace, the single `Cargo.lock` at the root is the one to commit.

**Detection:** Check that only one `Cargo.lock` exists at the workspace root, not inside individual crate directories.

**Phase to address:** Phase 1 (project setup). Note: workspaces produce a single `Cargo.lock` at the root — this is the correct behavior and it should be committed.

---

### Pitfall 15: GitHub Collaboration Artifacts Forgotten or Incomplete

**What goes wrong:** Contributors arrive and find no CONTRIBUTING.md, no issue templates, and no PR template. They don't know the contribution process, preferred branch names, or coding standards. First-time contributors open issues with insufficient information.

**Commonly forgotten items:**
- `.github/ISSUE_TEMPLATE/bug_report.md` — structured bug reports
- `.github/ISSUE_TEMPLATE/feature_request.md` — structured feature requests
- `.github/PULL_REQUEST_TEMPLATE.md` — PR checklist
- `CONTRIBUTING.md` — how to set up the project, branch naming, coding standards, how to run tests
- `CODE_OF_CONDUCT.md` — sets expectations for community behavior
- `CHANGELOG.md` — release history (even v0.1.0 should have an entry)
- License headers in source files (the `LICENSE` file at the root is required but not sufficient for all open-source contexts)

**Prevention:** Create all of these as a batch in the infrastructure phase. The GitHub UI prompts for `CODE_OF_CONDUCT.md` and `LICENSE` when creating a repo — don't skip them.

**Detection:** GitHub repo Community Standards checklist (visible under Insights > Community Standards).

**Phase to address:** Infrastructure phase (whichever phase sets up the GitHub repo).

---

### Pitfall 16: RustRover Proc-Macro Server Failures

**What goes wrong:** RustRover uses rust-analyzer internally. If `rust-src` is not installed for the active toolchain, the proc-macro server fails to spawn. Dioxus uses proc macros (`#[component]`, `rsx!`) heavily. When the proc-macro server fails, RustRover shows false errors throughout the UI crate.

**Warning signs:**
- Red errors in `rsx!` blocks that `cargo check` does not reproduce
- "Failed to spawn proc-macro server" in RustRover's Rust Analyzer logs
- `#[component]` attribute shows unresolved

**Prevention:**
- Run `rustup component add rust-src` for the project's active toolchain.
- Verify with `rustup component list | grep rust-src`.
- In RustRover, check Settings > Rust > Toolchain (verify the correct toolchain is selected and proc-macro server is enabled).

**Detection:** `cargo check --workspace` succeeds but RustRover shows errors → proc-macro server issue.

**Phase to address:** Phase 1 (project setup). Document this in CONTRIBUTING.md as a prerequisite.

---

### Pitfall 17: Duplicate Dependencies from Version Mismatches Between Crates

**What goes wrong:** If `kiss-tictactoe-engine` and `kiss-tictactoe-ui` declare different versions of the same dependency (e.g., different versions of `serde`), Cargo compiles both versions, bloating the binary and potentially causing type mismatches at the API boundary if those types cross the crate boundary.

**Warning signs:** `cargo tree -d` reports duplicate entries.

**Prevention:** Declare shared dependency versions once in `[workspace.dependencies]` in the root `Cargo.toml`. Have each member crate inherit them with `dep = { workspace = true }`.

**Detection:** `cargo tree -d --workspace` — no duplicates should appear for crates used by both members.

**Phase to address:** Phase 1 (workspace setup). Set up `[workspace.dependencies]` from the beginning.

---

### Pitfall 18: `use_signal` Called Conditionally (Rules of Hooks Violation)

**What goes wrong:** Calling `use_signal` inside an `if` block, `match` arm, or loop body causes hooks to run in a different order on different renders, which panics at runtime with a message about hook count mismatch.

**Warning signs:** Panic at runtime when a condition that guards a hook changes value.

**Prevention:** All `use_signal` (and other hook) calls must be at the top level of the component function, unconditionally, before any early returns.

**Detection:** Runtime panic with a hook-related message. This is a programming error, not a logic bug — a code review step is the primary prevention.

**Phase to address:** Phase where Dioxus components are first written. Establish the pattern in the first component and document it.

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|---|---|---|
| Workspace setup | Mixed root package; missing `[workspace.dependencies]`; wrong `.gitignore` | Follow workspace-only root; define shared deps upfront |
| Engine implementation | Minimax score perspective; terminal state order; accidental Dioxus dep | Write tests before integration; `cargo tree` check in CI |
| First Dioxus component | Outdated tutorial patterns; `use_state` instead of `use_signal`; hooks in conditionals | Verify every API against 0.7 docs; no pre-0.6 examples |
| Game board interactivity | Signal guard lifetime; form submit default inverted | Use `.set()` over `.write()`; always call `prevent_default()` |
| UI styling | Class name typos; raw `link` for CSS; inline styles for repeated rules | Use `document::Stylesheet`; compare class names to CSS |
| mdBook setup | Chapter missing from `SUMMARY.md`; untagged code blocks | Run `mdbook build && mdbook test` in CI |
| GitHub repo creation | Missing issue templates, PR template, CONTRIBUTING.md | Use GitHub Community Standards checklist post-launch |
| RustRover first open | Proc-macro server failure; `.idea/` files committed | Install `rust-src` component; verify `.gitignore` before first commit |

---

## Sources

- Dioxus 0.7 migration guide: https://dioxuslabs.com/learn/0.7/migration/to_07/llms.txt
- Dioxus 0.7 styling docs: https://dioxuslabs.com/learn/0.7/essentials/ui/styling
- Dioxus 0.7 signals docs: https://dioxuslabs.com/learn/0.7/essentials/basics/signals
- dioxus-hooks 0.7.10 rules of hooks: https://docs.rs/crate/dioxus-hooks/0.7.10/source/docs/rules_of_hooks.md
- Microsoft Rust Guidelines (workspace structure): https://microsoft.github.io/rust-guidelines/guidelines/project/
- University of Alberta negamax tic-tac-toe: https://webdocs.cs.ualberta.ca/~hayward/396/jem/ttt.html
- mdBook link checker behavior (Rust Reference): https://rust.googlesource.com/rust-lang/reference/+/refs/heads/master/dev-guide/src/tests.md
- JetBrains RustRover git setup: https://www.jetbrains.com/help/rust/set-up-a-git-repository.html
- RustRover release notes (proc-macro issues): https://www.componentsource.com/product/rustrover/releases/2922596
- GitHub gitignore template (Rust): https://code.morphllm.com/github/gitignore/src/commit/6578d74599385ec36ef4a6d8ff98e8c19b4149b8/Rust.gitignore
- dioxus-class crate (string class name typo risk): https://docs.rs/dioxus-class
- Cargo workspace duplicate deps: https://meteoraweb.com/en/sviluppo-di-siti-web/cargo-and-cratesio-for-rust-dependency-and-workspace-management-that-handles-production
