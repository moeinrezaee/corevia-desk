[فارسی](./INDEX.fa.md)

# Corevia Desk Documentation Index

This is the map for the Corevia Desk source, its current truth, and the exact continuation point. Read the current status and handoff before changing code.

## Start here

1. [Current status](./evidence/current-status.md) — what is actually verified now.
2. [Next-context handoff](./handoff/next-context.md) — the copy/paste contract for a fresh AI context.
3. [Roadmap](./roadmaps/roadmap.md) — phase order, ownership, acceptance, and open work.
4. [Acceptance gates](./specs/acceptance.md) — the definition of success for each priority.
5. [Problem history](./incidents/corevia-desk-problem-history.md) — resolved failures and open risks.

## Architecture and contracts

- [Architecture decision](./decisions/corevia-desk-architecture.md)
- [Product overview](./specs/overview.md)
- [Behavior and ownership contracts](./specs/contracts.md)
- [Nuxt/Rust bridge contract](./contracts/corevia-desk-bridge.md)
- [Machine-readable bridge schema](./contracts/corevia-desk-bridge.schema.json)
- [Release lifecycle contract](./contracts/corevia-desk-release-lifecycle.md)

## Specifications and verification

- [OpenSpec](./specs/openspec.yaml)
- [TestSpec](./specs/testspec.yaml)
- [Specification overview](./specs/overview.md)
- [Acceptance matrix](./specs/acceptance.md)
- [Specification changelog](./specs/changelog.md)
- [Phase evidence](./evidence/corevia-desk-phase-1-2.md), [CD-3](./evidence/corevia-desk-phase-3.md), [CD-4](./evidence/corevia-desk-phase-4.md), [CD-5](./evidence/corevia-desk-phase-5.md), [CD-6](./evidence/corevia-desk-phase-6.md), [CD-7](./evidence/corevia-desk-phase-7.md), [CD-8 to CD-12](./evidence/corevia-desk-phase-8-12.md), [P1 UI and data matrix](./evidence/corevia-desk-p1-ui-data.md)

## Operations and release

- [Corevia Standard and Bridge operating path](./operations/corevia-standard-bridge.md)
- [Release tools](../release/README.md)
- [WebView/Nuxt layer](../web/README.md)
- [MCP layer](../mcp/README.md)
- [Documentation standard](./standards/DOCUMENTATION_STANDARD.md)
- Source values: `.corevia/operations/`
- Bridge values: sibling `corevia-bridge/values/` files named `corevia-desk-*`

## Non-negotiable boundaries

- The active product is Rust Core plus Nuxt in a native WebView.
- Full Desk and Customer Support are separate profiles with separate release identities.
- Product connectivity is verified by RustDesk peer ID through ID/Relay; IP or same-network testing is not a substitute.
- MCP uses the same Rust ACL and is not a replacement for the product UI or a generic system-command proxy.
- Flutter, Dart, and Sciter are outside the active scope.
- Mobile is deferred and cannot delay the desktop goal.

## Ownership

Corevia Desk owns product behavior, Rust ACL, Nuxt, MCP adapter, and release definitions. Corevia Standards owns shared governance. Corevia Bridge owns approved operational deployment, health, update, rollback, and Nexus lifecycle only.
