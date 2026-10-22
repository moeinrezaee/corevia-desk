[فارسی](./DOCUMENTATION_STANDARD.fa.md)

# Corevia Desk Documentation Standard

Last Updated: 2026-09-05
Owner: Platform Engineering
Status: Active
Version: 1.0.0

## Purpose

Keep the Corevia Desk documentation usable by a person, an AI agent, and the Corevia validators without relying on chat history.

## Scope

This standard covers Markdown, OpenSpec, TestSpec, roadmap, evidence, incident, release, and handoff files in this repository.

## Prerequisites

- Read `docs/INDEX.md`.
- Read `docs/evidence/current-status.md` and `docs/handoff/next-context.md` before continuation.
- Resolve the applicable Corevia Standards Pattern before any repeatable external operation.

## Configuration

- Pair English and Persian documents as `<name>.md` and `<name>.fa.md`.
- Put runtime secrets in the approved secret store or CI secret variables only.
- Keep source-controlled values declarative and free of secret bytes.
- Use the status words `complete`, `in_progress`, `pending`, `blocked`, and `deferred` consistently.

## Run / Usage

Use the index to locate the source of truth. Update the relevant spec and evidence at the same time as a behavior or verification change. Use the handoff document when starting from an empty context.

## Validation / Verification

- English/Persian pair and reciprocal-link check.
- Required-section and local-link check.
- No-secret scan.
- Corevia Standards repository validator.
- `git diff --check`.
- Evidence must identify the command, target, result, and date without exposing credentials.

## Troubleshooting

- If two documents disagree, prefer verified evidence over a plan and correct both language files.
- If a result is only a contract or local plan, mark it open; do not call it delivered.
- If a Bridge executor is missing, record `extension-needed`; do not replace it with manual remote mutation.
- If a native runner is unavailable, keep the target open and continue safe local work.

## Change Log

- 2026-09-05: Added the Corevia Desk documentation map, acceptance model, incident history, and empty-context handoff contract.

## Ownership

Platform Engineering.
