[فارسی](./corevia-desk-architecture.fa.md)

# Corevia Desk Architecture Decision

## Status

Accepted for the roadmap baseline on 2026-09-02.

## Problem

Corevia Desk already contains a Rust core and inherited native UI sources. The new product needs a web-based UI, MCP control, two permission profiles, and cross-platform desktop packages without duplicating the Rust behavior. The active product path must not depend on Flutter or Dart.

## Options

### Option A — Keep the inherited native UI as the active UI

Lowest short-term integration risk because the existing native UI sources already exist. The HTML/Tailwind design would need to be rebuilt in a different UI stack, and the web UI would not share the design source directly.

### Option B — Nuxt view layer inside Corevia Desk with a Rust WebView shell

The Nuxt UI can be built from the approved `corevia-desk-ui` template and packaged for desktop while Rust remains the system and remote-desktop core. It requires a new Nuxt-to-Rust bridge, but it keeps the UI and Rust responsibilities clear and supports a later mobile path.

### Option C — Electron-based desktop shell

It reuses web UI well for Windows, macOS, and Linux, but adds a separate desktop runtime and does not solve the mobile target. It also creates an extra boundary between the existing Rust core and the application shell.

## Decision

Select Option B.

- Rust remains the existing source of truth for system access and remote-desktop behavior.
- A mandatory Rust ACL and shared command boundary will serve the active Nuxt/WebView UI and MCP.
- The raw visual source remains `corevia-desk-ui`.
- The Nuxt implementation will live inside `corevia-desk/web` and follow Atoms, Molecules, Organisms, Templates, Layouts, and Pages.
- The MCP adapter will live inside `corevia-desk/mcp` and will call the shared Rust boundary, never an inherited UI bridge or low-level Rust functions directly.
- The active desktop UI is Nuxt inside a native Rust WebView. Inherited Flutter/Dart and Sciter sources remain only as preserved history/rollback references; they are excluded from the active build, CI, installer, and release path. Mobile work is deferred.
- The native host is implemented in `src/webview.rs` with `tao` and `wry`; it
  serves packaged Nuxt assets through the private `corevia://` protocol and
  routes the named browser bridge through the Rust ACL.
- CI/CD owned by Corevia Desk will build two release profiles per environment and publish them to Nexus.
- Corevia Installer may consume only the Customer Support profile if later extended.
- Corevia Bridge owns only approved operational deployment, health, update, rollback, and runtime configuration work.

## Consequences

- Phase 3 must establish the Rust ACL before Nuxt or MCP capabilities are exposed.
- Phase 4 must convert the approved visual source into a real Nuxt component system rather than a second unrelated UI.
- Inherited UI sources are preserved during migration, which keeps source history and rollback evidence possible; they are not a reason to install or run Flutter/Dart.
- Desktop delivery is the first cross-platform target; mobile does not block the roadmap.
