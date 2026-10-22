[فارسی](./contracts.fa.md)

# Corevia Desk Contracts

Last Updated: 2026-10-05
Status: Active
Owner: Platform Engineering

## Purpose

Define the boundaries that prevent UI, MCP, packaging, and operations from drifting away from the Rust product core.

## Rust Core and ACL

- `src/corevia_acl.rs` is the single authorization boundary for Nuxt, MCP, and covered preserved native entrypoints.
- Full Desk uses the default profile; Customer Support uses `corevia-customer-support`.
- Sensitive actions require an approval reference and produce redacted, monotonic audit data.
- `system-command` is denied for every profile.
- UI code cannot call low-level Rust, shell, URL, SQL, repository, or arbitrary command surfaces.

## Nuxt/WebView contract

- The active launcher is `src/main.rs` -> `src/webview.rs`.
- The host loads packaged Nuxt output through the private `corevia://` protocol.
- The named `window.__COREVIA_DESK_BRIDGE__` surface routes to Rust ACL dispatch.
- The browser-only fallback may show fixtures and `local-agent-required`; it must not claim native capability.
- An `accepted` response alone does not prove execution. If `data.executed` is `false`, the UI must show understandable failure feedback and must not optimistically change state/data; MCP must return a tool error for the same result.
- Atomic Design direction is `Atoms -> Molecules -> Organisms -> Templates -> Layouts/Pages`.

## Product connection contract

- A baseline connection starts from the product on both endpoints.
- The remote target is selected by RustDesk peer ID.
- ID Server, Relay Server, and server key come from runtime configuration; secrets are never copied into docs or logs.
- Acceptance requires approval, active state, live frame, mouse and keyboard control, close, and reconnect.
- MCP can approve or observe only through its governed surface; it cannot replace the product session path.

## MCP contract

- MCP transport is stdio and exposes only `initialize`, `tools/list`, and `tools/call`.
- Full Desk and Customer Support have separate effective catalogs.
- MCP uses the same Rust ACL and approval rules. Sensitive tools do not accept a
  caller-supplied approval reference. If the client advertises form elicitation,
  the adapter sends `elicitation/create` for each sensitive call; rejection,
  cancellation, timeout, an older protocol, or a missing capability fails
  closed.
- An accepted response becomes an internal, one-time HMAC proof bound to a
  random reference, operation, and payload; Rust verifies it. This proof is not part of
  the public tool arguments, and arbitrary caller text cannot replace it. Final
  acceptance still requires a real elicitation-capable MCP client and the full
  executor matrix.
- MCP is not a generic Rust, shell, system-command, SQL, URL, or repository proxy.
- ACL authorization is not successful execution. If the native bridge returns
  `data.executed: false`, the MCP call must be surfaced as a tool error, not an
  apparent success.
- Full Desk chat sending requires separate per-message approval and previews
  the sanitized message in the approval request.

## Release contract

- Artifact identity is `corevia-desk/<profile>/<version>/<platform>/<architecture>/`.
- Profiles are `customer-support` and `full-desk`.
- Customer Support targets macOS and Windows.
- Full Desk targets macOS, Windows, Linux, and a Linux Docker image.
- Archives, manifests, and checksums are required; signatures are optional and not required by CI.
- Versioned Nexus paths are immutable and must not be overwritten.

## Corevia Bridge boundary

Bridge owns source-driven installation, health, update, rollback, runtime configuration, and Nexus operations. It does not own Rust behavior, UI rules, MCP authorization, or product business logic. If the product-specific executor is missing, the status is `extension-needed`; manual remote mutation is not a valid substitute.

## Change Log

- 2026-10-05: Added host-mediated MCP form elicitation and an internal one-time proof; real-client compatibility and some executors remain open.
- 2026-09-05: Consolidated Rust ACL, WebView, ID/Relay, MCP, release, and Bridge boundaries.

## Ownership

Corevia Desk owns these product contracts. Shared schemas and governance come from Corevia Standards; operational execution comes from Corevia Bridge.
