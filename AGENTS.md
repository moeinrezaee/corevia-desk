# RustDesk Guide

## Corevia Desk delivery lock (authoritative)

- Highest priority is a stable, release-ready Full Desk and Customer Support
  edition for both macOS and Windows. Do not declare delivery complete until all
  four profile/platform combinations pass the acceptance matrix.
- Test every visible UI screen and interactive control against its real product
  action, including loading, empty, success, and failure states. A visual smoke
  alone is not functional acceptance.
- Never silently drop a rejected native-bridge action or show optimistic success;
  provide localized feedback, preserve the last confirmed state, and test the
  failure path without fabricating product data.
- Home recents must represent successful connections, not newly saved address-
  book entries or pending/closed sessions. Persist the successful connection
  time, do not advance it while reading disconnected records, and present
  session history newest-first with its native end time when available.
- In session history, group terminal events by normalized Peer ID and display
  only the latest state/end time per device; use address/name only if Peer ID
  is absent, and keep unidentified session IDs separate. This is a display
  projection: do not delete the native event records. Use a clear, non-clickable
  text status, never an X/cross that can look like a close action.
- For custom-hidden switches, verify that checked, disabled, and keyboard-focus
  appearances agree with their accessibility state in Full Desk and Customer
  Support; a correct accessibility value with a visually false switch is a UI
  failure. When a language change is rejected, render its error in the locale
  that remains active after rollback.
- Do not ship an enabled button or menu without a real action. Remove unsupported
  controls or present them as clearly informational; closing a session tab must
  request native `close-session`, and a rejected close must leave the session
  visible and report the failure. Do not present history clearing until the
  product has a real supported operation for it.
- Validate real peer-ID connections through ID/Relay first. IP, RDP, or being on
  the same network is not a substitute.
- The active interface is Nuxt hosted by the native Rust WebView. Flutter,
  Dart, and Sciter are outside active build, test, installer, and release scope.
- After the four macOS/Windows desktop deliverables pass, complete Linux and
  Docker at lower priority. A VS Code extension is future scope only.
- Read `docs/evidence/current-status.md`, `docs/handoff/next-context.md`, the
  acceptance matrix, and the roadmap before resuming. Preserve dirty files and
  submodules; never reset, stash, or switch away from a dirty primary checkout.
- Check available disk before builds or image work. Clean only safe,
  regenerable targets through the declared owner workflow; do not remove
  user/source data, GitLab/Nexus data, or Docker images/cache without an
  explicit approved cleanup source.
- Any GitLab, Nexus, or host mutation must use the latest Corevia Standards
  decision and the latest Corevia Bridge `connectcore` executor. If a capability
  is missing, record the exact gap and resolve it through the official pattern;
  do not use a manual bypass.

## Project Layout

### Directory Structure
* `src/` Rust app
* `src/server/` audio / clipboard / input / video / network
* `src/platform/` platform-specific code
* `src/ui/` preserved legacy UI sources (not the active UI)
* `flutter/` historical UI source (outside active scope)
* `web/` active Nuxt UI, packaged inside the native WebView
* `libs/hbb_common/` config / proto / shared utils
* `libs/scrap/` screen capture
* `libs/enigo/` input control
* `libs/clipboard/` clipboard
* `libs/hbb_common/src/config.rs` all options

### Key Components
- **Remote Desktop Protocol**: Custom protocol implemented in `src/rendezvous_mediator.rs` for communicating with rustdesk-server
- **Screen Capture**: Platform-specific screen capture in `libs/scrap/`
- **Input Handling**: Cross-platform input simulation in `libs/enigo/`
- **Audio/Video Services**: Real-time audio/video streaming in `src/server/`
- **File Transfer**: Secure file transfer implementation in `libs/hbb_common/`

### UI Architecture
- **Active UI**: Nuxt in `web/`, loaded and governed by the native Rust WebView.
- **Preserved UI**: Sciter/Flutter sources remain historical references only;
  do not add them to active build, test, installer, or release paths.

## Rust Rules

* Avoid `unwrap()` / `expect()` in production code.
* Exceptions:

  * tests;
  * lock acquisition where failure means poisoning, not normal control flow.
* Otherwise prefer `Result` + `?` or explicit handling.
* Do not ignore errors silently.
* Avoid unnecessary `.clone()`.
* Prefer borrowing when practical.
* Do not add dependencies unless needed.
* Keep code simple and idiomatic.

## Tokio Rules

* Assume a Tokio runtime already exists.
* Never create nested runtimes.
* Never call `Runtime::block_on()` inside Tokio / async code.
* Do not hide runtime creation inside helpers or libraries.
* Do not hold locks across `.await`.
* Prefer `.await`, `tokio::spawn`, channels.
* Use `spawn_blocking` or dedicated threads for blocking work.
* Do not use `std::thread::sleep()` in async code.

## Editing Hygiene

* Change only what is required.
* Prefer the smallest valid diff.
* Do not refactor unrelated code.
* Do not make formatting-only changes.
* Keep naming/style consistent with nearby code.

### Comments

* Avoid comments unless they explain a non-obvious reason, constraint, or workaround.
* Never restate what the code does; prefer clearer code instead.
* If the code is self-explanatory, add no comment.

### Be minimally invasive

* Prefer purely additive changes: layer new (`#[cfg]`-gated) blocks or new functions around existing code instead of restructuring it. The ideal diff for a fix adds lines and modifies/deletes none.
* Do not extract or reshape existing code just to enable your new code; look for a mechanism that leaves existing lines untouched (e.g. hide/show an existing object instead of refactoring its construction into a helper for rebuilding).
* Accept a little duplication over a restructure. A new function that repeats a few lines of an existing one is a better diff than reshaping the original so both can share it.
* Put new logic in self-contained functions in the module it belongs to (platform-specific logic in `src/platform/`, with `use` inside the function body to avoid churning shared import blocks). Call sites in shared files (`src/tray.rs`, `src/core_main.rs`, `src/server/connection.rs`, …) should be thin one-line hooks.

## Reviewing a PR

* Review only what the diff introduces. Verify ownership with `gh pr diff` before reporting a finding — if the offending lines are untouched context, it is a pre-existing problem, not this PR's.
* List pre-existing problems in a separate section at the end, or leave out the ones that are not fatal. Never mix them into the findings the author has to fix.
* Before re-reviewing, read the author's reply comments. Do not re-raise items they declined on scope grounds.
* State a finding's consequence exactly: distinguish "the value is lost" from "the shortcut is inert but the value still saves".

## Localization (`src/lang/*.rs`)

Each file is a `HashMap<key, translation>`. Layout:

* `template.rs` is the master list of every key. **Never edit it** as part of translation work.
* `en.rs` holds only the keys whose English display text differs from the key itself.
* Every other file (`de.rs`, `fr.rs`, …) carries the full key set; an untranslated entry has an empty value: `("key", "")`.

### Finding the English source for a key

When filling an empty entry, determine the source English text with this rule:

* If `key` exists in `en.rs` **with a non-empty value**, that value is the source text (look it up in `en.rs`).
* Otherwise the **key string itself is the source text** (the key is already plain English).

Then translate that source into the file's target language (infer the language from the file's existing non-empty entries / filename).

### Translation hygiene

* Only fill empty values. Never change keys, and never touch existing non-empty translations.
* Preserve placeholders (`{}`) and escape sequences (`\n`, `\"`) exactly as in the source.
* Do not translate brand or technical tokens: `RustDesk`, `Socks5`, `TLS`, `UAC`, `Wayland`, `X11`, `TCP`, `UDP`, `2FA`, `RDP`, `D3D`, etc.
* Copy URL values (e.g. `doc_*` keys) verbatim from `en.rs`.

### Adding new keys (feature work)

* New English-text keys use sentence case, not Title Case: `Use ID whitelisting`, **not** `Use ID Whitelisting`. Acronyms (ID, IP, 2FA…) stay uppercase. Legacy Title-Case keys (e.g. `Use IP Whitelisting`) stay as-is — do not rename them.
* Since the key itself is the English display text, a sentence-case key usually needs **no** `en.rs` entry; add one only when the display text must differ from the key (e.g. `*_tip` keys).
* Append each new key to `template.rs` (with `""`) and to every `src/lang/*.rs` file (translated, or `""` if unsure), at the end of the list.
