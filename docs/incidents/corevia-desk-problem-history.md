[فارسی](./corevia-desk-problem-history.fa.md)

# Corevia Desk Problem History

Last Updated: 2026-09-10
Status: Active historical record
Owner: Platform Engineering

This is a compact, source-linked history of failures that previously caused drift or false completion. A future agent must read it before repeating a diagnosis.

## Purpose

Prevent resolved investigations from being repeated and keep every open problem tied to a concrete acceptance gate.

## Scope

This record covers the active Corevia Desk Rust/WebView product, its ID/Relay smoke path, MCP boundary, native packaging, and governed release operations.

## Prerequisites

Read the current status, acceptance matrix, and handoff before investigating a new connection or release issue.

## Configuration

Server configuration is runtime-only. This history contains names of settings and ports, never their secret values.

## Run / Usage

Use the tables to classify a report as resolved, open, non-blocking, or a regression. Add a new row only after recording evidence.

## Validation / Verification

Cross-check each status against the current evidence and acceptance matrix. A plan or source contract alone cannot close an incident.

## Troubleshooting

Start with product-process state on both endpoints, then peer ID/ID/Relay, then the native session/frame/input path. Do not substitute IP, RDP, or a browser fixture.

## Resolved problems

| Windows runner 403 | The Windows HTTPS source had not been reconciled into the active Traefik production route | Reconciled the official Traefik/input sources; runner API returned 200 through both paths and `gitlab-runner verify` returned `rc=0` | Resolved; runner is online |

| Problem | Cause | Resolution | Status |
| --- | --- | --- | --- |
| macOS binary crashed because `libsciter.dylib` was missing | The inherited Sciter launcher was still being used | Active launcher moved to Rust `tao-wry` WebView and packaged Nuxt assets | Resolved; Sciter remains out of active scope |
| Flutter/Dart was treated as a desktop dependency | Old RustDesk source layout was mistaken for the selected Corevia architecture | Active path and CI scope were locked to Rust + Nuxt/WebView; Flutter/Dart are preservation-only | Resolved by scope contract |
| Windows request stayed pending before the native manager delivered it | Native delivery path waited on an obsolete pre-login state | Rust/native session delivery and the Nuxt approval action were fixed and installed on the tested Windows Full Desk | Resolved in the tested build |
| First video frame could close a pending session | The first-frame callback was interpreted as `close_success()` | First-frame handling now marks the connection active and records `first-frame-ready` | Resolved |
| Repeated same-ID requests created duplicate native sessions | Registry lookup happened after a new connection could be inserted | Existing pending/active connection is reused before a second one is created | Resolved and live-verified |
| RDP capture was unstable | The Windows process was running inside an RDP session | WinRM moved the session to Windows Console; RDP was not used as the product connection path | Resolved for the smoke environment |
| A local Customer Support macOS bundle carried restricted metadata but a Full Desk native binary | Full Desk and Customer Support builds shared a Cargo target root and a stale native binary could be copied into the new profile bundle | Release packaging now isolates Cargo targets by profile/platform/architecture, rejects skip-build packaging, and the Customer Support bundle was rebuilt and native-read back against its expected profile | Resolved in the local release path; install and live GUI remain open |
| Two concurrent image readbacks in Bridge wrote one shared report | The report run ID contained only the current second | The run ID now includes the process PID, and parallel Node/Debian readbacks return independent correct reports | Resolved; parallel test and evidence recorded |

## Open or non-blocking problems

| Problem | Impact | Current treatment |
| --- | --- | --- |
| Two simultaneous remote sessions with two independent destinations are not fully evidenced | Previously blocked the final P1 gate | Two independent `/r` requests by Peer ID to Windows and Customer Support were approved and read back Active with `direct:false` and `secured:true` for over 20 seconds, then closed cleanly. A same-ID request still correctly reuses the existing session; visual selector readback and side-by-side Windows install remain P1.5 observation/delivery gates |
| Full Customer Support native install/connect/UI matrix is not fully evidenced | Blocks P1.5 | Native builds for both targets, the Windows artifact, process launch, and headless ACL/MCP are recorded; side-by-side install, visual UI, and product readback remain |
| MCP needed native coverage beyond five safe reads for both macOS profiles | Previously blocked P2 | 25/9 catalogs, allowed mutations, approval, audit readback, missing-session non-execution, and profile-sensitive/secret/system denials now pass locally; P2 is closed locally |
| The Windows destination disk filled during the install attempt | Blocked operational test execution | Exact prior build/cache directories were removed, freeing about 4 GB; the installed Full Desk product was untouched |
| The Customer Support Windows archive could not be transferred reliably through the available WinRM stream | Blocks P1.5 installation/readback | Truncated files and exact temporary tasks were removed; no incomplete install was claimed, and the governed artifact/executor path is still required |
| A valid CI run for the current commit is not yet read back | The CI base images are now in Nexus from the official resolver/Bridge path with immutable digests for `linux/amd64`, but the available run at old commit `2fc0e57f` pulled Node successfully and then failed in `npm ci` on a lockfile mismatch | Official source resolver, publish, and exact Nexus readback are recorded; the old run was not retried and no unrelated image was substituted | P4 remains open; a current-commit run, product artifacts, and governed publication/readback are required |
| Governed working-branch publication was unavailable | Prevented current-commit CI from seeing the local verified work | New `gitlab-branch.publish` Pattern and Bridge executor now check clean worktree, fast-forward-only ancestry, and exact remote commit readback; Plan passed, Apply is pending the clean local commit | Apply the branch publish, then use the existing fast-forward-only `codex → develop → main` sync and run current CI |
| Audit/API side port 21114 timed out during earlier checks | Does not block ID/Relay connection/control but remains a separate service issue | Track separately; do not widen the product connection criterion |
| Protected CI/Nexus publication and readback have not been claimed locally | Blocks release completion, not local P0 | Run only through the declared native runners and Bridge publish/readback patterns |
| Product-specific Bridge installer/health/update/rollback executor is missing | Blocks governed remote install/update | Keep `extension-needed`; do not use manual remote mutation |

## Investigation rules

- Never substitute IP connectivity, same-network placement, RDP, or a browser fixture for a RustDesk peer-ID ID/Relay test.
- Check that the product is running on both source and destination before diagnosing the server.
- Use WinRM for Windows operational checks when available; RDP is not the product test path.
- Treat MCP as an optional governed companion after the product path works.
- Keep secrets out of evidence; refer to runtime configuration by name only.

## Evidence

- [Current status](../evidence/current-status.md)
- [CD-8 to CD-12 evidence](../evidence/corevia-desk-phase-8-12.md)
- [Acceptance matrix](../specs/acceptance.md)

## Change Log

- 2026-09-17: Resolved the Windows runner 403 through the official Traefik reconcile and HTTPS/API readback; no manual runner or WireGuard mutation was used. CI, artifact, and visual Windows UI gates remain separate.

- 2026-09-10: The official Source resolver/publish/readback completed for both CI base images with immutable `linux/amd64` digests and exact Nexus destination readback. The available pipeline then pulled Node successfully but failed at old commit `2fc0e57f` on a lockfile mismatch; the P4 blocker moved from image supply to a current-commit run and artifact readback.

- 2026-09-09: Official readback was repeated; Source Audit remained green, but the CI base images are still absent from Nexus and the pipeline stops before tests. No retry or manual mutation was performed.
- 2026-09-08: The fresh Source Audit detected drift after a Bridge source change; the permitted catalog write and immediate follow-up audit returned green. CI image readback still showed missing `node:22-bookworm` and `debian:trixie-slim`, and the pipeline remained stopped before tests.
- 2026-09-05: Consolidated the missing Sciter dependency, Flutter scope, native delivery, first-frame, duplicate-session, RDP, and release-gate history.
- 2026-09-06: Recorded the native Customer Support Windows build, process launch, and headless ACL/MCP evidence; the missing governed installer executor and unreliable UI Automation keep P1.5 open.
- 2026-09-07: Recorded and fixed the stale-profile macOS packaging mismatch with isolated native targets and two-profile safe native readback; the governed installer and live GUI gates remain open.
- 2026-09-07: Live Clipboard content passed in both directions; an intermediate same-ID request reused the existing connection and temporarily left the independent concurrent-session gate open, which the later run closed the same day.
- 2026-09-07: Closed the concurrent P1 session gate with two independent approved `/r` Peer-ID destinations (Windows and Customer Support); both stayed Active, relayed, and secured for over 20 seconds before clean close. Side-by-side Windows install/UI, positive Secret, Linux/Docker, and CI/Nexus remain open.
- 2026-09-07: The official pipeline stopped in `verify:contracts` before tests because `node:22-bookworm` was absent; official Nexus readback also confirmed the missing `node:22-bookworm` tag and `debian:trixie-slim` repository. No unrelated internal image was substituted and no retry/manual mutation was performed.
- 2026-09-07: Completed native MCP mutation/approval/audit/denial readback locally for both profiles and closed P2 locally. Freed the exact Windows build/cache paths after disk exhaustion; governed Customer Support artifact transfer remains open.

## Ownership

Platform Engineering.
