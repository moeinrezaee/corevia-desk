[فارسی](./corevia-standard-bridge.fa.md)

# Corevia Standard and Corevia Bridge Operating Path

Last Updated: 2026-10-08
Status: Active
Owner: Platform Engineering

## Purpose

Document how Corevia Desk uses the shared Standards rules and the Bridge execution boundary without turning product work into undocumented manual operations.

## Scope

This file covers repository governance, documentation/spec validation, GitLab branch operations, Nexus artifact/image operations, and the boundary between product code and operational infrastructure.

## Prerequisites

- The local `corevia-standards` and `corevia-bridge` sources are available beside this repository.
- The requested operation has a source-controlled values file.
- The required executor and runtime secret references are available; raw credentials are never copied into values.

## Configuration

Repository intent lives under `.corevia/operations/`. Product behavior remains in Corevia Desk; GitLab, Nexus, and host lifecycle state remains behind the declared Bridge values and executor.

## Latest source readback

After the 2026-10-08 fetch, Desk's remote `codex/desk-phases-8-12`, `develop`,
and `main` all resolve to `248afe1b69a73efe171c12d2714cd42a942fb04f`; each
fast-forward comparison is 0/0, so no remote branch update was needed. The Desk
worktree and `libs/hbb_common` remain dirty; those uncommitted changes are not
included in the three remote refs and were not committed or pushed.

Latest clean snapshots are Standards `origin/develop` `f663ba2`, Bridge
`origin/develop` `7bb1645`, and Knowledge `origin/develop` `e605a19`; the
Knowledge `origin/main` snapshot remains `539b3e9`. Bridge Source Audit on
`7bb1645` initially found a single generated-hash drift for
`kernel/usecases/service_repo.sh` (no missing or extra catalog entries). After
reviewing that upstream change, the Standards `bridge-operation.govern`
Validate/Plan passed; the official `connectcore source catalog --write` updated
only that component hash in the isolated Bridge snapshot. A rerun of Source
Audit and the Standards `bridge-source.catalog` Validate/Plan passed with the
same 180 playbooks, 196 roles, 168 use cases, 184 commands, 182 outbound
adapters, 8 Terraform modules, and 32 legacy candidates. This one-file catalog
diff remains local and unpublished; the remote Bridge branch was not changed.

Primary Bridge, Standards, and Knowledge checkouts were not switched, reset, or
stashed. Use the `.work/*-latest` snapshots for reviewed operations. Preserve
all dirty Desk files and the submodule.

## Run / Usage

Start with the Pattern Detection Gate, run `validate` and `plan`, then use the concrete Bridge executor only when the operation is approved. Finish with readback and sanitized evidence.

## Pattern Detection Gate

Before a repeatable operation, classify it as `reuse`, `extend`, `migrate-on-touch`, `create-new`, or `not-repeatable`.

Current declared operations:

| Work | Pattern | Values | Owner |
| --- | --- | --- | --- |
| Roadmap | `roadmap.create-or-update` | `.corevia/operations/roadmap.values.yaml` | Standards local executor |
| Phase evidence | `roadmap-phase.execute` | `.corevia/operations/roadmap-phase.values.yaml` | Standards/Bridge governance |
| Repository specs | `repo-specs.create-or-update` | `.corevia/operations/repo-specs.values.yaml` | Standards local executor |
| Documentation audit | `documentation.audit` | `.corevia/operations/documentation-audit.values.yaml` | Standards local executor |
| GitLab branches | `gitlab-branch.reconcile` | `.corevia/operations/gitlab-branch-reconcile.values.yaml` | Corevia Bridge |
| Branch synchronization | `gitlab-branch.sync` | `.corevia/operations/gitlab-branch-sync.values.yaml` | Corevia Bridge |
| Protected branches | `gitlab-branch-protection.reconcile` | `.corevia/operations/gitlab-branch-protection.values.yaml` | Corevia Bridge |
| Nexus image publication | `container-image.publish` | sibling `corevia-bridge/values/platform/corevia-desk-gitlab-runner-image.yaml` | Corevia Bridge |
| Nexus image readback | `container-image.nexus-readback` | sibling `corevia-bridge/values/platform/corevia-desk-gitlab-runner-image-readback.yaml` | Corevia Bridge |

### Repository spec warning

The current `repo-specs.create-or-update` Pattern is draft and regenerates the
OpenSpec and TestSpec files from Values; it can discard detailed existing
acceptance criteria. Validate and Plan are read-only, but one Apply on this
repository's primary files truncated those criteria. The files have been
reconstructed from the authoritative acceptance documents and validated. Do
not Apply again until the Pattern preserves unknown content and the complete
diff has been reviewed.

The official Plan still proposes updating both files; that is a dry-run only,
and no further Apply was run.

## Standard commands

Run the Documentation Audit with the latest isolated Standards snapshot. Its
values resolve the Desk repository from the `.work` workspace, so run from
`/Volumes/Personal/Projects/corevia/.work` and use the temporary repository
links only when absent; remove only those exact links after validation:

```sh
ln -s ../corevia-desk corevia-desk
ln -s corevia-bridge-latest corevia-bridge
./corevia-standards-latest/bin/corevia-run validate --pattern documentation.audit --values corevia-desk/.corevia/operations/documentation-audit.values.yaml
./corevia-standards-latest/bin/corevia-run plan --pattern documentation.audit --values corevia-desk/.corevia/operations/documentation-audit.values.yaml
unlink corevia-desk
unlink corevia-bridge
```

The plan/evidence output is retained under the declared `.work` location. No raw credential belongs in a values file or report.

## Bridge execution chain

```text
Corevia Standards -> source catalog -> capability resolution -> concrete Pattern
-> bridge-operation.govern -> connectcore -> Terraform/Ansible/provider -> readback/evidence
```

For GitLab or Nexus state changes, the local source values are the request. Corevia Bridge is the executor. Direct SSH, manual GitLab UI changes, ad-hoc WinRM mutation, or a one-off upload are not valid substitutes when an executor is missing. Corevia Desk branch synchronization is declared as `codex/desk-phases-8-12 -> develop`, followed by `develop -> main`, both with `fast-forward-only` policy.

## Current Bridge gap

Generic GitLab branch and container-image capabilities are declared. A Corevia Desk-specific installer/health/update/rollback executor is still `extension-needed`; the product release contract records the required future sequence. This does not block the already verified local P0 session, but it keeps operational delivery open.

The latest isolated Bridge snapshot now audits clean after the reviewed
generated-catalog refresh; the remote Bridge branch still has not received that
one-file update. In the latest Standards snapshot, the product pipeline Pattern still references the absent
`corevia-bridge/values/platform/corevia-desk-gitlab-pipeline-current.yaml`, and
the latest `connectcore` catalog does not declare `gitlab-pipeline-current`.
Therefore CI/Publish was not run manually. Nexus Runner-image readback is green,
but product-artifact publication and its product-specific executor are not
complete. WinRM or SSH readback is not a substitute for the governed executor.

The Bridge values file referenced by Desk's `gitlab-branch.sync` request is also
absent. This did not require a remote write because Desk's fetched source,
`develop`, and `main` already point to the same commit. No branch-sync mutation
was run.

The fresh official Source Catalog, Resolve/Extend, and Operation Governance
Validate/Plan checks passed; the decision is Extend. Existing `platform:gitlab-pipeline`
only retries the latest failed pipeline and cannot satisfy exact-current-commit
creation/reuse plus complete job/artifact readback. The product Pattern remains
`draft`; no pipeline or publication is allowed until its executor, Values, and
pre-run approval gate are complete.

## Secret handling

Only references such as `env://NEXUS_PASSWORD` may appear in source values. Actual credentials are supplied by the runtime secret store or CI secret variables and are never copied into chat, docs, logs, manifests, or commits.

## Validation / Verification

- Repository Standards validator.
- Documentation audit and local-link/pair checks.
- `git diff --check`.
- Pattern-specific validate/plan before any Bridge apply.
- Post-apply readback and sanitized evidence.

## Troubleshooting

- If the Pattern is missing or the executor does not support the requested state, stop at `extension-needed`.
- If a protected branch rejects a direct push, use the declared review/merge path; never force-push.
- If Nexus readback differs from the source digest, fail closed and do not overwrite the versioned path.

## Change Log

- 2026-10-04: Recorded fetched source revisions and safe snapshot policy for dirty Standards/Knowledge checkouts; confirmed the Bridge Source Audit and documented the exact missing current-pipeline Values/operation blocker.
- 2026-09-06: Resolved Catalog Drift through the official path and recorded the green Source Audit; the product executor, artifact publication, and CI/Nexus readback gaps remain.
- 2026-09-05: Added the Corevia Desk-specific Standard/Bridge operation map, official branch-sync Values, and declared executor gap.
- 2026-10-08: Recorded the lossy Apply risk in `repo-specs.create-or-update`; OpenSpec and TestSpec were reconstructed from the acceptance source. Official Plan still proposes both updates; do not re-Apply until the Pattern is fixed and its complete diff is reviewed.
- 2026-10-08: Recorded the lossy Apply risk in `repo-specs.create-or-update`; OpenSpec and TestSpec were reconstructed from the acceptance source and validated. Do not re-Apply until the Pattern is fixed and its complete diff is reviewed.

## Ownership

Corevia Standards owns governance; Corevia Bridge owns operational mutation and readback; Corevia Desk owns product source and release intent.
