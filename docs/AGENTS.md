# Corevia Desk Documentation Guide

This file is the operating guide for the documentation under `docs/`.

## Source of truth

- `docs/specs/openspec.yaml` defines the selected architecture and ownership.
- `docs/specs/testspec.yaml` defines the verification gates.
- `docs/roadmaps/roadmap.md` and `roadmap.fa.md` define phase order and status.
- `docs/evidence/` records only verified results.
- `docs/handoff/next-context.*` is the continuation contract for a fresh AI context.

## Required documentation behavior

- Keep English and Persian documents paired and link to the reciprocal language at the top.
- Keep the current status, roadmap, specs, evidence, incident history, and handoff aligned in the same change when the accepted state changes.
- Keep the locked delivery objective visible in the status, roadmap, acceptance
  matrix, and handoff: stable Full Desk and Customer Support releases for both
  macOS and Windows, followed by Linux/Docker at lower priority; a VS Code
  extension is future scope only.
- The desktop acceptance matrix covers all four profile/platform combinations.
  For every screen and interactive control, record the underlying action and
  its success/error/empty/loading/disabled/permission-denied behavior; do not
  count a visual-only smoke as functional evidence.
- Rejected native-bridge mutations must produce localized user feedback and
  must not create optimistic records, sessions, or settings state. Record
  browser-preview evidence separately from native-product evidence.
- Home recents represent successful connections, not newly saved address-book
  entries or pending/closed sessions. Persist the successful connection time,
  never advance it while reading disconnected records, and show session history
  newest-first with the native end time when available.
- Session history is a display-only projection grouped by normalized Peer ID:
  show one row with the latest terminal state and native end time per device;
  fall back to address/name only when Peer ID is missing, and keep unidentified
  session IDs separate. Preserve native event records. Use a noninteractive,
  labeled status chip, not an X/cross that resembles a close action.
- Custom-hidden switches must show checked, disabled, and keyboard-focus states
  consistently with their accessibility values in both profiles. If a locale
  change is rejected, its notification must use the locale retained after
  rollback.
- Do not leave enabled controls without an action. A session-tab close must call
  the native close operation and preserve the confirmed session on rejection;
  unsupported actions such as history clearing must not be shown as clickable.
- Keep agent instructions accurate about the active Rust + Nuxt native-WebView
  architecture. Flutter, Dart, and Sciter are excluded from active work.
- Never write passwords, private keys, server keys, tokens, cookies, or connection strings into source, documentation, values, logs, or evidence.
- Do not mark a phase complete because a contract, branch, or CI file exists. Completion requires the acceptance check and evidence.
- Record blockers as a concrete missing capability or verification result, not as a vague status.
- Before build, image, or test work, check free disk space. Do not delete Docker
  images/cache, GitLab/Nexus data, source files, or user data unless a current
  approved cleanup source explicitly covers the exact target. Prefer bounded,
  regenerable artifacts and document any cleanup.
- Preserve the active-scope rule: Rust Core plus Nuxt inside the native WebView. Flutter, Dart, and Sciter are preserved history only and must not re-enter active build, test, installer, or release work.

## Required sections for technical and operational documents

Unless a document is an index, spec data file, evidence record, or handoff, include:

- Purpose
- Scope
- Prerequisites
- Configuration
- Run / Usage
- Validation / Verification
- Troubleshooting
- Change Log
- Ownership

## Validation

When primary Standards/Knowledge checkouts are dirty, run the Documentation
Audit from `/Volumes/Personal/Projects/corevia/.work` against the recorded
latest Standards snapshot. Create the exact repo links only if absent and
remove only links created for that run:

```sh
ln -s ../corevia-desk corevia-desk
ln -s ../corevia-bridge corevia-bridge
./corevia-standards-latest/bin/corevia-run validate --pattern documentation.audit --values corevia-desk/.corevia/operations/documentation-audit.values.yaml
./corevia-standards-latest/bin/corevia-run plan --pattern documentation.audit --values corevia-desk/.corevia/operations/documentation-audit.values.yaml
git -C ../corevia-desk diff --check
unlink corevia-desk
unlink corevia-bridge
```

If a primary Standards or Knowledge checkout has local changes, do not switch,
reset, or stash it to run newer tooling; use an isolated latest-ref snapshot and
record its exact revision. If the repository-wide validator fails on an
unrelated Standard example, report that separately from the Desk documentation
audit and do not claim a full repository pass.

For a state-changing GitLab, Nexus, or host operation, fetch and inspect the
latest Corevia Standards and Corevia Bridge refs, run the official Source Audit,
then use the Standards Pattern Detection/capability decision and declared
Bridge `connectcore` executor. If that chain reports a missing executor, stop
that external operation and record `extension-needed`; direct APIs, manual UI
changes, SSH mutation, and force-push are not fallbacks.

## Ownership

Platform Engineering owns this documentation set. Corevia Standards owns reusable documentation rules; Corevia Bridge owns governed operational execution, not product behavior.
