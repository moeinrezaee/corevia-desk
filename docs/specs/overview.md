[فارسی](./overview.fa.md)

# Corevia Desk Product Overview

Last Updated: 2026-09-05
Status: Active
Owner: Platform Engineering

## Purpose

Corevia Desk is the Corevia-branded remote desktop product built around one Rust Core, a Nuxt view layer hosted in a native WebView, and a governed MCP control surface.

## Scope

The repository contains the Rust remote-desktop runtime, the active Nuxt UI, the shared Rust ACL and bridge, two release profiles, MCP policy/catalog code, and release packaging.

The raw visual source remains in the sibling `corevia-desk-ui` repository. The runtime view is implemented in this repository under `web/`. `corevia-web` is not the UI source for this product.

## Product profiles

| Profile | Allowed product surface | Distribution |
| --- | --- | --- |
| Full Desk | All approved remote-desktop capabilities, subject to approval and ACL | Independent Nexus artifacts for macOS, Windows, Linux, and Linux Docker output |
| Customer Support | Customer settings, incoming connection approval, and connection permissions only | Separate macOS and Windows artifacts; optional customer installer only |

Both profiles use the same Rust Core and ACL. A profile difference is not implemented only by hiding a button; it must be enforced by Rust policy, UI surface, MCP catalog, and release metadata.

## Active runtime path

```text
Rust Core -> Rust ACL -> native WebView host -> Nuxt Atomic Design UI
                         \-> governed MCP adapter
```

RustDesk ID/Relay is the product connection path. The endpoints do not need to share a network. IP-based or same-network testing is not acceptance evidence.

## Explicitly outside active scope

- Flutter and Dart are not used by the active desktop UI, CI, installer, or release path.
- Sciter is preserved only as inherited source history and rollback reference.
- Mobile is deferred and does not block desktop delivery.
- A future VS Code Docker extension is a very-low-priority follow-up, not a release gate.

## State and data expectations

Devices, sessions, recent connections, settings, concurrent sessions, clipboard, Toolbox, responsive layout, and exact RTL/LTR mirroring are product requirements. A fixture, static card, or successful page render is not evidence that the feature is real. The acceptance matrix records each item separately.

## Related documents

- [Contracts](./contracts.md)
- [Acceptance](./acceptance.md)
- [Architecture decision](../decisions/corevia-desk-architecture.md)
- [Roadmap](../roadmaps/roadmap.md)
- [Current status](../evidence/current-status.md)

## Change Log

- 2026-09-05: Consolidated the product boundary, profile model, active runtime, and deferred scope.

## Ownership

Corevia Desk owns product behavior and release definitions. Corevia Standards owns shared governance. Corevia Bridge owns operational lifecycle only.
