[فارسی](./roadmap.fa.md)

# Corevia Desk Roadmap

## Current Delivery Lock — 2026-10-08

The exact current Customer Support bundle was rechecked after its own permission request and restart: macOS shows its screen/audio entries enabled, but the current UI still reads capture=false. The root cause is unconfirmed. The owner-approved cross-platform proxy rule applies, so this Mac-specific result does not stop testing; shared image/connection behavior must still pass on an isolated Windows desktop. Read-only WinRM access to the Windows worker now succeeds, but its Customer Support session is disconnected and the RDP certificate is not trusted; there is still no active isolated desktop for visual acceptance. Windows C: has about 15 GiB free and no Personal volume, so Docker work must not be placed there.

Latest native data readback: Devices still has one row each for the Mac and Windows peers, with their last successful-connection times unchanged. Sessions reads Active=0 / two recent; the latest Mac-peer attempt is Failed, not a successful connection. Customer Support has no inbound request. See the [P1 UI/data evidence](../evidence/corevia-desk-p1-ui-data.md).

First priority is stable, release-ready **Full Desk for macOS and Windows** and
**Customer Support for macOS and Windows**. Do not call any edition ready until
install/launch, every screen and interactive control, each control's real product
action, data/session behavior, loading/empty/error states, RTL/LTR, and responsive
layouts are tested and evidenced on the current source. Full Desk connectivity
is proven by peer ID through ID/Relay; IP, RDP, or same-network access is not a
substitute.
On every supported desktop platform, a repeated launcher click must activate
or restore that profile's existing window without creating a second main
process; the two profiles may coexist. Customer Support is fixed at a mobile
size, while Full Desk remains wide and resizable.

Product-owner decision: a Mac signing certificate is not a condition for continuing. Shared behavior blocked only by Mac permission may be accepted from equivalent Windows/other-OS evidence, clearly labeled as a proxy; Mac capabilities that do not need signing must still be tested directly on Mac.

Latest live test: Current Full Desk connected to the older Customer Support bundle on the same Mac by Peer ID and `/r`; encrypted Relay and a real frame remained stable for 40+ seconds, then the screen-only session closed. This proves the route and that permission-authorized bundle, not the current Customer Support release: the fresh bundle still reads capture as `false`. macOS reopened the older path after Quit & Reopen; manually starting the fresh bundle did not fix it. Both bundles are ad-hoc with different CDHashes and no valid signing identity is installed; this is a plausible lead, not a proven root cause, and no TCC reset/bypass occurred. Full Desk briefly showed stale waiting status until “Open session” refreshed it to Online/Relay/Encrypted. `/` and Personal each have about 40 GiB free; `.work` is about 15 GiB and Docker was untouched. Current-bundle image, independent Windows, input, and all live tools remain open gates.

Latest continuation — 2026-10-08: On the older permitted bundle, keyboard/mouse permission was accepted for one test session, but no input was sent. The destination displayed personal browser content, so testing stopped immediately; input was disabled and the session ended. No Toolbox/MCP operation was run. The current Customer Support bundle still reports screen capture unavailable and Full Desk reports Active=0. Devices currently has one row for the repeatedly used Mac peer (last-connected time Oct 7, 2:07 PM) and a separate older Windows device; Sessions has the ended peer and a distinct synthetic failed-ID attempt. Windows TCP 3389/5985 responded, 5986/22 timed out; the self-signed RDP certificate was not trusted. Official Standards Validate/Plan passed without connecting, but the draft Bridge executor hardcodes HTTP Basic and disabled certificate validation, so it was not run. At that earlier network-only check no remote command was sent; a later read-only WinRM inventory succeeded without any remote write. `/` and Personal each have about 37 GiB free at that earlier readback; Docker remains untouched. Details: [Windows safe-access evidence](../evidence/corevia-desk-windows-safe-access-20261008.md) and [latest WinRM evidence](../evidence/corevia-desk-windows-winrm-readback-20261008.md).
Latest continuation — 2026-10-08: On the older permitted bundle, keyboard/mouse permission was accepted for one test session, but no input was sent. The destination displayed personal browser content, so testing stopped immediately; input was disabled and the session ended. No Toolbox/MCP operation was run. The exact current Customer Support bundle was added/enabled in macOS settings; after its own in-app request, a graceful restart, and confirmation that the current executable was running, capture still reported unavailable. Full Desk reports Active=0. Devices has one row for the repeatedly used Mac peer (last-connected time Oct 7, 2:07 PM) and a separate older Windows device; Sessions has the ended peer and a distinct synthetic failed-ID attempt. Windows TCP 3389/5985 responded, 5986/22 timed out; the self-signed RDP certificate was not trusted. The available Bridge executor hardcodes HTTP Basic and disabled certificate validation and is only a network check, not an interactive app-test route, so it was not run. No remote command or host change occurred. To reduce local disk use, the regenerable Corevia Desk Cargo target cache was removed through `cargo clean` (5.5 GiB); the current app bundles remain running and Docker was untouched. `/` and Personal each now have about 37 GiB free. Details: [Windows safe-access evidence](../evidence/corevia-desk-windows-safe-access-20261008.md).
Automated rechecks in the latest continuation: Web 15/15, MCP 27/27, Release 10/10, and Atomic/Profile lints passed; native input and live Toolbox remain open.
Fresh disk readback: after the 5.5-GiB Cargo cleanup, the old Codex plugin-catalog cache (~77 MiB) was also removed. `/` and Personal now each have about 38 GiB free; Docker, active bundles, and source were preserved.
The official Standards Documentation Audit also passed Validate and Plan on snapshot `f663ba2`: 53 files, zero findings; temporary audit links were removed.
Latest refs (2026-10-08): Standards `f663ba2`, Bridge `7bb1645`, Knowledge `develop` `e605a19` / `main` `539b3e9`. Bridge Source Audit is green only in the isolated snapshot after the official generated-catalog update; that one-file change is not published upstream. Desk's three requested remote branch refs are already equal at `248afe1b`; its local working tree remains dirty and unpushed.

Continuation — 2026-10-08: Production Nuxt generation passed for both profiles, along with Web 15/15, MCP 27/27, Release 10/10, Atomic/Profile/Scope lints, and `git diff --check`. The official Standards docs audit passed Validate/Plan across 53 files with zero findings. A Peer-ID `/r` request reached current Customer Support; it was declined and no screen or input data was transmitted, leaving Active=0. Repeat only after action-time confirmation, then use a safe isolated Windows desktop for the connection/tool matrix. Current free space is about 42 GiB.

In this continuation, a draft Standards Apply truncated the two spec files; acceptance content was reconstructed and passed YAML parsing and the official validator. Official Plan still proposes both updates, so Apply was not repeated and the Pattern must be fixed. Product acceptance gates remain open.

The pending live-test request was declined when no action-time confirmation arrived; no screen or input data was transmitted and Sessions read Active=0 afterward. Both local volumes have about 42 GiB free.

The browser-preview responsive matrix passed 60/60 checks on 2026-10-07 across
six routes, five viewport sizes, and RTL/LTR. This closes only the preview
layout sample. The current Customer Support dark/RTL preview was also visually
rechecked at 1280×720 and appears consistent with the established style, but
the browser has no native bridge. Native UI and every control across all four
variants remain open.
The fresh Mac bundle reached encrypted, relayed Connected by Peer ID `/r`, but
Customer Support still reports screen capture unavailable and no real frame
arrived. An in-app permission request and quit/reopen of that exact bundle did
not change the result. The earlier image-producing and fresh bundles have
different ad-hoc designated code hashes, a plausible but unproven permission
identity mismatch; the exact cause is unproven. Stable signing and direct Mac
permission remain unverified, but per the owner decision this does not stop work:
validate shared behavior on Windows as an explicitly labeled proxy. The session
ended at Active=0. Devices showed one row with a 9:09 timestamp and Sessions one
latest-state card for that peer. Direct Mac UI checks that need no signing passed
for language/theme switching and restoration, Full Desk navigation, and five
display preferences. An empty ID was rejected; a synthetic custom ID failed and
left only a Sessions-history row, with no new device or Home recent. Independent
Windows connectivity and live tool tests remain open.

The MCP result path now reports an error when authorization succeeds but the
native action returns `executed: false`; sending chat also requires per-call
approval and previews the message. Synthetic approval tests pass and the MCP
suite passes 27/27. The optional port-forward destination now matches the MCP
schema and Rust allowlist, with malformed hosts rejected. The optional Boolean
enable/disable field for voice calls is also exposed and validated by Rust.
Focused Rust tests pass for both profiles; neither a live tunnel nor a live
voice call has been tested. Missing Camera, Printer, or Service executors and
the live Toolbox matrix remain open.
The UI also shows failure feedback and does not advance local state ahead of
native readback; Web tests pass 15/15 and the production build passes. The live
tool matrix remains open.

Live MCP readback on the current `1.5.36-local.20261007` Mac bundles returned
catalogs of 25/9 tools. Five safe reads per profile reached the native Rust
bridge and returned `accepted`; Full Desk had two saved devices and both
profiles had zero active sessions. No identity values were printed. This is
read integration evidence only: mutations, interactive approval, and live
session tools remain open.

Rust unit tests pass 131/131 for Full Desk and 131/131 for Customer Support on
macOS arm64 with the repository's pinned vcpkg. The 2.2 GiB temporary target
was isolated on Personal and cleaned after testing. This does not close live
cross-host input, Windows UI, or every Toolbox action.

On 2026-10-07, both Mac bundles were rebuilt as `1.5.36-local.20261007`, passed
release smoke, and were opened in the native UI with one main process per
profile. Customer Support has the distinct title and fixed mobile-sized window.
Two same-named macOS permission entries are on, but the fresh bundle still reads
screen capture as false; the exact cause is unproven. This Mac has no valid code
signing identity and the bundles are ad-hoc. Their designated code hashes differ,
which may explain the permission mismatch but is not proven. Signed release, current-build image,
independent-host input, and full native UI acceptance remain open.

Latest readback, 2026-10-06: Full Desk Mac showed a stable self-test image and
reconnect; MCP created and closed an encrypted ID/Relay session, but independent
two-host operation remains open. Customer Support Windows built and passed smoke
from the same source, and NTLM-encrypted WinRM worked. A real Mac `/r` request
reached Relay but did not become encrypted or reach Customer Support; the Windows
RDP session was disconnected and had no display. Full Desk was already running on
that host with the same Peer ID, so the target is not isolated. The RDP certificate
warning was cancelled, not bypassed. See the [Windows evidence](../evidence/corevia-desk-windows-customer-support-20261006.md).
The Windows build is unsigned and has no installer. The three Desk refs were
previously read back at `248afe1b69a73efe171c12d2714cd42a942fb04f`; local changes
remain uncommitted/unpublished.

| Gate | Verified state on 2026-10-06 | Next action |
| --- | --- | --- |
| Desk refs | `248afe1b69a73efe171c12d2714cd42a942fb04f` on the work branch, `develop`, and `main`; tag `v1.5.36` is the same commit | Do not resync unless fresh readback shows drift |
| Bridge Source Audit | `ok: true` on clean latest `develop` snapshot `5952da0`; catalog Write not needed | Preserve the official source path before external operations |
| Standards / Knowledge | Clean Standards snapshot `bdfe0b9` on `origin/develop`; Knowledge default `main` `539b3e9` and newer `develop` `e6a2a01` are isolated; dirty primary checkouts preserved | Continue using fresh isolated snapshots |
| Connection, image, and input; then MCP/tools | Older permission-authorized Customer Support displayed a real frame over encrypted Relay for 40+ seconds, screen-only. Current Customer Support still reads capture false. Full Desk briefly showed stale waiting status until “Open session” refreshed it. Independent Windows, input, and live Toolbox remain open | On an independent Windows desktop, verify Peer-ID/Relay image and stability, then input, reconnect, and every tool/MCP operation with native readback. Do not treat the older bundle as current release proof |
| Mac Screen Capture | Fresh bundle still reads false after manual launch. macOS initially reopened the older bundle on Quit & Reopen. Both ad-hoc designated requirements bind different CDHashes; no valid signing identity is installed. Plausible mismatch, not proven | Do not bypass/reset TCC. Use an official stable signing/registration route if available; otherwise label Windows evidence as the approved cross-platform proxy and continue direct tests for Mac capabilities not gated by signing |
| Customer Support Windows | Earlier build `1.5.36-test.248afe1b6`, smoke, single-instance, and safe MCP reads PASS; no Authenticode signature or installer. Windows live UI remains unverified. The matching official Bridge verifier is draft and hardcodes HTTP Basic with certificate validation disabled; it was not run. See [safe-access evidence](../evidence/corevia-desk-windows-safe-access-20261008.md) | Provide a governed secure access route, then test the current build on an isolated active desktop: real `/r` image, stability/reconnect, input, and each tool |
| CI/Nexus | Latest Pattern references a missing Bridge Values file and `connectcore` lacks `gitlab-pipeline-current` | Resolve through official Standards → Bridge → `connectcore`; no manual trigger/publish meanwhile |
| Linux/Docker | Lower priority than the four desktop variants. Docker's main disk image is physically on Personal through the existing symlink (42 GiB free); images, containers, build cache, and container logs were preserved. About 11 MiB of separate Docker Desktop diagnostic logs remain on Data. Docker Desktop's image list/API is unresponsive despite the “Engine running” indicator. | After engine health returns, set/verify the Personal disk-image location through Docker Settings → Resources → Advanced, then test Linux/Docker. Do not manually move the disk image or restart/purge while the engine is unresponsive. |
| VS Code | Future scope, not a delivery gate | No current work |

### Session-history continuation — 2026-10-06

- The screenshot's duplicate cards and close-like red cross are fixed in source: terminal rows group by normalized Peer ID, preserve native event records, and show only the latest state/end time. A warning icon plus short noninteractive “Failed” chip remains fully readable; recent-connection count deduplicates identities.
- In fresh Full Desk macOS/arm64 bundle `1.5.36-history-badge-20261006`, two consecutive live ID/Relay `/r` requests to same-Mac Customer Support were rejected. Native events remain, while Full Desk rendered one card, advanced its displayed time on attempt two, kept count 1, showed zero active sessions, and visually displayed the full label. Customer Support returned to no incoming request; no access was granted.
- History logic tests passed 4/4, profile/Atomic lints, strict release smoke, and active-scope validation passed. This does not close P1: successful-connection persistence, the full control matrix, and the other three variants remain open. Official Docs/Repo Specs checks must be rerun after this update; no generic Apply was run.
- The exact 2.1 GiB temporary Cargo target was cleaned and the 24 MB staging bundle was kept; `/` and Personal each have about 62 GiB free. Docker images/cache are untouched. CI/Nexus remains blocked on the missing official exact-commit pipeline executor; no commit or push was made.

### Latest execution and disk readback — 2026-10-05

- CI/Nexus: official Source Catalog, capability, and governance Validate/Plan pass on Bridge `5952da0` and Standards `bdfe0b9`; decision is Extend. The existing command only retries the latest failed pipeline and is insufficient for the exact current commit/artifact readback. The product Pattern remains `draft`; its Values file and `connectcore platform gitlab-pipeline-current` operation are missing. No pipeline/publication ran; Desk edits remain uncommitted and absent from Remote.
- Current-source browser QA measured six Full Desk/Customer Support routes at five widths (320–1440) in RTL and LTR: 60/60 had no horizontal overflow or browser-console error. Blank connection/device forms, Devices filters/search, and Cancel created no data. Real settings could not be tested without the native host. Nuxt dev printed transient `#app-manifest` HMR errors although routes rendered. Disk has 67 GiB free; Docker images/cache remain intact.
- Latest run: both macOS/arm64 `1.5.36-mcp-elicitation-test8` bundles passed strict smoke; Rust passed 125/125 per profile, MCP Node 22/22, and release tests 10/10. MCP host elicitation plus Rust proof verification passed through a synthetic stdio harness for both profiles; a real host has not been verified.
- Active `test6` UI was read-only inspected before staging. Two post-fix requests were sent and closed safely; only the first appeared after returning to Home. The second did not establish whether Customer Support stayed visible, so the live auto-refresh check remains open. Native Customer Support `test10` passed strict smoke and selected visual/accessibility checks. Full Desk `test11` fixed and natively verified the duplicated/narrow Sessions connection dialog; blank Peer ID created no session and no network request was made. A real TypeScript MCP SDK client over stdio read both native helpers: catalogs 25/9 and five/four safe reads passed; elicitation and sensitive calls remain open. Before another live check transmits a Peer ID to ID/Relay, confirm that specific action. The official Standards Documentation Audit Validate/Plan passed across 49 files with zero findings. After removing this build's temporary 2.1 GiB Cargo target, storage readback showed about 53 GiB free on `/` and Personal; the 24 MB app bundle was kept and Docker images/cache were untouched.
- Current-source browser QA covered Full Desk Home, Devices, Sessions, Settings, Customer Support, and Customer Support Settings at 390×844, 768×1024, and 1280×720; no route overflowed horizontally. The Home Quick Connect button now focuses the Peer ID field. Blank connection and incomplete device save remained validation-only with no records. Profile/atomic lints and the production web build pass. Native test8 UI, LTR readback, and every real action remain open.

- Historical `test7` snapshot before elicitation: those bundles and suites passed at the time, while sensitive calls were denied because host proof was unavailable. The newer status is above. Do not repeat the earlier synthetic outbound probe with fabricated IDs.
- Fresh `test5` readback: both macOS/arm64 bundles (`1.5.36-single-instance-test5`) passed strict release smoke and reported the same version in About; Rust passed 120/120 for each profile. Native UI verified the startup appearance and permission-guidance fixes; Customer Support's English preference survived restart and was restored to Persian. Finder reopening retained one process per profile. Blank Peer ID validation, device search/filters, Sessions empty state, selected Settings, and accurate permission guidance passed. Real connectivity and the complete matrix remain open. Current disk readback: macOS Data about 3.2–3.4 GiB free; Personal about 61–62 GiB free.
- A follow-up current-bundle Peer ID `/r` request from Full Desk showed `Relayed via ID/Relay` but did not appear in local Customer Support; it remained pending until ended and was recorded as failed. Native MCP `read-sessions` was Accepted and empty for both profiles afterward. Relay selection passed; delivery, approval, live image, and P0 did not.
- Full Desk and Customer Support for Mac were built as `1.5.36-single-instance-test4` and passed strict version-matching release smoke; the About page now reports the same version as the bundle. After minimize and reopening from Finder, both restored the same window; double-clicking each bundle again did not create a second process.
- A limited native check opened Full Desk Home, Sessions, Devices, and Settings: the helper identifies Peer ID through ID/Relay as primary, `/r` to force Relay, and IP as secondary. Blank connection forms created no session, empty filters/search behaved correctly, and canceling device creation preserved data. Customer Support showed the empty incoming-request state and restricted profile permissions. This is not the complete UI matrix.
- The earlier `test4` readback found the startup appearance mismatch (default Persian/dark versus saved English/light); the shared-runtime fix is verified natively in the newer `test5` readback above.
- The full single-instance module and its tests pass the Windows/MSVC cross-target check; this does not replace a native Windows build/runtime test. Full Windows, native Linux, full UI/data, real inbound request, current-source MCP, and CI/Nexus remain open gates.
- Latest disk readback: macOS Data 1.6 GiB free / 99% used; Personal has 66 GiB free / 38% used. Cargo removed 9.8 GiB of generated output, Go about 330 MiB, and npm about 50 MiB. NuGet caches were cleared but partially repopulated alongside active .NET tooling, so cleanup was not repeated. The web build stayed on Personal. Test bundles were preserved. Docker, Codex, and active Chrome/VS Code caches remain untouched. Avoid large builds on Data.

Latest source fetch/readback: a subsequent fetch across Desk, Bridge, Standards, and Knowledge found no ref changes. Desk's remote refs align at the release SHA; its active worktree remains dirty and uncommitted, including the pre-existing dirty submodule; no switch, stage, commit, or push was made. Bridge is on latest `develop`; the clean Standards snapshot is on latest `develop` while its conflicted primary checkout was preserved. Knowledge's clean snapshot equals latest `origin/main`; its dirty primary checkout was preserved. The 49-file Documentation Audit and repeated official Bridge Source Audit both pass. `corevia-desk-ui` is not a Git repository.

Fresh disk readback on 2026-10-04: the macOS Data volume has about 1.5 GiB free and is 99% used; Personal has about 56 GiB free. Cargo registry and Git caches were moved, not deleted, to Personal and their original paths were preserved. Future build output and temp files must also use that volume. The 2.1 GiB Chrome code-sign clone is in use by a live process and was left untouched; the VS Code update cache, Docker images/cache, source, and product data also remain untouched. See the [single-instance evidence matrix](../evidence/corevia-desk-single-instance.md).

## Endpoint / Definition of Done

- One existing Rust Core remains the source of truth for system access and remote-desktop behavior.
- A mandatory Rust ACL and shared command boundary serve the active Nuxt/WebView UI and MCP.
- The Nuxt Atomic Design view layer is implemented inside `corevia-desk` from the raw `corevia-desk-ui` source.
- Customer Support and Full Desk are separate release profiles and separate Nexus artifacts per environment.
- Corevia Installer is optional and may expose only Customer Support.
- Corevia Bridge owns only approved operational deployment, health, update, rollback, and runtime configuration.
- Mobile remains deferred until the desktop-first path is verified.

## Historical release snapshot — 2026-09-18 (not current status)

The detailed release and phase notes below retain their original dates and are
historical evidence, not a fresh verification of the current source. For current
gates, use [current status](../evidence/current-status.md) and the acceptance
matrix.

### Latest governed continuation readback

- Commit `939f46c0594cf7a251acae34ca862f1d96dfb338` and immutable tag `v1.5.32` are published; official Bridge sync fast-forwarded `codex/desk-phases-8-12` -> `develop` -> `main` in order to the same SHA.
- Pipeline `3536` completed all builds and package publication; only Docker manifest verification job `24630` failed with Nexus HTTP 401 after the corrected native Runner completed Windows Full Desk job `24627` successfully.
- Runner evidence confirms system Git long-path support, Rust/Cargo, API reachability, a Running service, and accepted job heartbeats. The old Windows backlog was cleared only through exact graceful official cancellations.
- P4 remains open until a fresh immutable tag passes Docker verification and the exact `linux/amd64` product-image/package Nexus readbacks are recorded. The actual `1.5.32` image already passed official Bridge readback; P1.5 Windows side-by-side/visual UI and P3 positive Secret disclosure remain open.

### Source and branch release state

- The current release commit is on `codex/desk-phases-8-12`, and official sync readback is green at `939f46c0594cf7a251acae34ca862f1d96dfb338`.
- `codex/desk-phases-8-12` was fast-forwarded to `develop`, then `develop` was fast-forwarded to `main`; all three refs read back to the same SHA.
- The official `gitlab-branch.publish` Pattern and Bridge executor now enforce a clean worktree, fast-forward-only ancestry, and exact remote commit readback. The Service Repo push plan still targets only `main`, so it remains unused to preserve the required promotion order.
- `.corevia/operations/gitlab-branch-sync.values.yaml` declares the permitted order: working branch to `develop`, then `develop` to `main`, both `fast-forward-only`.
- The official Bridge source audit is green after the permitted catalog regeneration, and protected-branch readback succeeded; no force-push or manual mutation was used.
- The 2026-09-08 readback briefly found catalog drift after a Bridge source change; the permitted catalog write was run and the immediate follow-up audit returned `ok: true`.
- The 2026-09-09 official readback confirmed that Source Audit remained green; the image source work then resolved and published both CI base images.
- On 2026-09-10, official Runner readback confirmed `linux/amd64`; the immutable Node/Debian CI images were published and read back. Pipeline `3536` then consumed them successfully and produced the Linux and Docker outputs; its remaining active work is Windows Full Desk followed by publication/readback.

- CD-1 Corevia Standards Bootstrap: Complete.
- CD-2 Architecture Boundary and Decision: Complete.
- CD-3 Shared Rust ACL and Command Boundary: Complete.
- CD-4 Nuxt Atomic Design View Layer: Complete.
- CD-5 Nuxt-to-Rust bridge: Implemented; the native binding is delivered in CD-5A.
- CD-5A Native WebView host and runtime wiring: Implemented locally for macOS arm64 and exercised in the real product smoke; native runner/release checks remain.
- CD-6 MCP control layer: Complete locally; the 25/9 catalogs and native readback for allowed mutations, approval, audit, missing-session non-execution, and profile-sensitive/secret/system denials pass for both profiles; external publication remains in CD-7/P4.
- CD-7 CI/CD and Nexus release path: Pipeline `3536` passed contracts, both macOS artifacts, Linux, Docker packaging, both Windows builds, and package publication on the corrected native Windows runner; only Docker manifest verification failed with HTTP 401. The actual `1.5.32` product image passed official Bridge readback as immutable `linux/amd64`; the Bearer-challenge verifier fix requires a fresh immutable tag and complete readback.
- The current Docker package job pushes directly to the declared Nexus registry and emits only small metadata; the exact product-image readback is repeated after the publish stage. No unverified image or platform substitution is allowed.
- CD-8 Customer Support product: UI, ACL, MCP, macOS, Windows archive/manifest/smoke, process launch, profile namespaces, concurrent approved Active readback, and the governed Windows runner are checked; the current pipeline also produced its protected Windows artifact. Side-by-side install and visual Windows UI remain open.
- CD-9 Full Desk product: Implemented locally; macOS/Linux/Docker/Windows artifacts are green in `3536`, Docker publication succeeded, and only the verifier's registry-auth method failed; fresh protected CI/readback remain external.
- CD-10 Corevia Bridge operations: Contract recorded; the branch-publish executor is now implemented and plan-verified, while product-specific install/health/update/rollback remains `extension-needed`.
- CD-11 Mobile follow-up: Deferred by design.
- CD-12 Desktop WebView release verification: Open for final delivery; the real
  macOS↔Windows ID/Relay frame/control/close/reconnect smoke passed locally,
  the duplicate-session guard is verified on the installed macOS binary, and
  two independent `/r` destinations (Windows and Customer Support) reached
  approved simultaneous Active with `direct:false` and `secured:true` before
  clean close. Side-by-side Customer Support Windows install and visual UI,
  live P3 GUI, and protected Nexus image/package readback remain.
- The local P1 evidence now covers responsive layout, RTL/LTR, Devices, Sessions,
  Settings, Empty State, persistence, Customer Support UI, and route metadata.
  A fresh Peer ID/ID-Relay run reached Online with a real Windows frame, destination
  approval, and correct Remote Cursor/View Only toggles; real Clipboard transfer is
  passed, and two independent native `/r` requests reached approved simultaneous
  Active with `direct:false`/`secured:true` before clean close.
- The MCP policy matrix, local-only Secret eye, and notification region are
  implemented and build-checked; internal browser smoke for both profiles is
  green across three viewport sizes, RTL/LTR, and console health, while the
  no-Bridge preview produced no fake session or notification. Native GUI and
  external release gates remain.
- Customer Support Windows was built natively with the C/Windows SDK runner;
  archive, manifest, official smoke, and headless native MCP ACL checks pass.
  A fresh incoming request was approved through the product and remote readback
  returned an active session.
  A scheduled task launched the exact portable bundle with `--no-server`; its
  process was observed in Session 2 for 8 seconds. UI Automation did not return
  reliable output, so side-by-side install and visual Windows UI remain open.
  Windows release path handling was hardened with `fileURLToPath` and `npm.cmd`
  shell execution.
- A packaging audit rejected an earlier macOS Customer Support bundle whose
  metadata was restricted but whose native binary was Full Desk. Release builds
  now use profile/platform/architecture-isolated Cargo targets and reject
  `COREVIA_SKIP_CARGO_BUILD`; the Customer Support macOS bundle was rebuilt with
  the project vcpkg, and native safe-readback matched the expected profile for
  both macOS bundles. The two profiles now also have distinct macOS application
  identifiers, display names, config/IPC namespaces, and Peer IDs; two independent
  `/r` destinations reached approved simultaneous Active with secure Relay metadata
  before clean close. Install, visual GUI, and external product readback remain open.
- Devices and connection-dialog copy now makes destination Peer ID primary and IP
  secondary; the final Persian/English browser smoke read this back.
- The connection dialog now accepts RustDesk's explicit `/r` suffix as an optional Relay selector; the Bridge normalizes it before validation and preserves Peer ID as the primary identity. Unit coverage and refreshed macOS profile bundles pass; a native `/r` run was approved and read back Active with `direct:false`/`secured:true`, while visual selector readback remains open.
- Live Clipboard content transfer passed in both directions; marker
  `COREVIA_P1_CLIP_UI_20260907_G` was read on Windows after macOS→Windows transfer.
  A same-ID second request reused the active connection as designed. Two independent
  `/r` destinations were also approved and read back simultaneously Active, relayed,
  and secured for over 20 seconds before clean close; the remaining gates are P1.5,
  P3, P4, and Linux/Docker.
- Complete native MCP mutation/approval/audit/deny readback passed locally for both
  macOS profiles (25 Full Desk tools and 9 Customer Support tools); P2 is locally
  complete, while P4 publication remains governed and external.
- Exact Windows build/cache directories were cleaned after disk exhaustion. The
  Customer Support archive could not be transferred reliably through the available
  WinRM stream, so no incomplete install was claimed.

## Pattern Detection Gate

- `roadmap.create-or-update`: this roadmap and its bilingual pair.
- `repo-specs.create-or-update`: OpenSpec and TestSpec baseline.
- `mcp-service.create-or-migrate`: no existing MCP source was found; future work routes to `mcp-source.create-or-update`.
- `corevia-bridge`: operational work must pass source catalog, capability resolution, and operation governance before any deployment change.

## Ownership

| Area | Owner | Boundary |
| --- | --- | --- |
| Shared contracts and validators | `corevia-standards` | Generic rules, schemas, evidence, and safety gates |
| Rust Core, ACL, product profiles, Nuxt, and MCP | `corevia-desk` | Product behavior and approved capability access |
| Raw visual template | `corevia-desk-ui` | Design source only; not the runtime UI |
| Artifact build and publication | `corevia-desk` CI/CD | Build, test, optionally sign, and publish immutable packages |
| Nexus and server operations | `corevia-bridge` | Operational lifecycle only; no product business logic |
| Optional customer installer | `corevia-bootstrap-installer` | Customer Support profile only |

## Phases

### CD-1 — Corevia Standards Bootstrap

Status: Complete.

Outcome:

- Corevia Desk is registered as a governed Corevia repository with a pinned local Standards source.

Deliverables:

- `.corevia/repo.yaml`, `.corevia/follows.yaml`, `.corevia/skills.yaml`, and `.corevia/validators.yaml`.
- Bilingual OpenSpec, TestSpec, and roadmap.
- Source-controlled values for repository specs and roadmap governance.

Acceptance:

- Repository metadata and required documentation are present.
- The Standards validator and no-secret check pass.

Verification:

- `ruby ../corevia-standards/validators/corevia-standards/validate.rb --target . --check-repo`
- `git diff --check`

Evidence:

- Repository metadata, validator output, and `docs/evidence/corevia-desk-phase-1-2.md`.

### CD-2 — Architecture Boundary and Decision

Status: Complete.

Outcome:

- The ownership and dependency direction for Rust, Nuxt/WebView, MCP, Nexus, Installer, and Bridge are explicit.

Deliverables:

- Architecture Decision Record in `docs/decisions/`.
- Rust Core, mandatory ACL, Nuxt View Layer, MCP Layer, release profiles, and operational boundaries are recorded in OpenSpec and this roadmap; inherited native UI sources are preservation-only.

Acceptance:

- `corevia-desk-ui` is the raw design source and `corevia-desk/web` is the Nuxt runtime layer.
- `corevia-web` is outside the Corevia Desk UI path.
- Desktop is first priority; mobile is deferred.
- Full Desk and Customer Support have separate release and distribution boundaries.

Verification:

- Review the decision record, OpenSpec, TestSpec, and roadmap as one dependency map.
- Run the repository Standards validator and `git diff --check`.

Evidence:

- `docs/decisions/corevia-desk-architecture.md`, its Persian pair, and `docs/evidence/corevia-desk-phase-1-2.md`.

### CD-3 — Shared Rust ACL and Command Boundary

Status: Complete.

Outcome:

- Rust now owns one typed permission boundary used by the active Nuxt/WebView and MCP adapters; preserved native paths remain guarded for rollback safety.

Deliverables:

- `src/corevia_acl.rs` with Customer Support and Full Desk profiles, named capabilities, safe denial reasons, approval references, and redacted audit records.
- `corevia-customer-support` Cargo feature; the default build remains Full Desk.
- Covered settings, connection-manager approval/permission/chat, and preserved native session operations pass through the Rust boundary before executing.
- Direct system-command capability is denied for every profile.

Acceptance:

- The active Nuxt/WebView path and preserved native UI paths cannot reach covered sensitive actions without the Rust ACL.
- Customer Support exposes only customer settings, incoming connection approval, and connection permissions at this boundary.
- Sensitive actions require a valid action reference and create a redacted, monotonic audit record.
- Nuxt and MCP have named surfaces in the same Rust boundary; their native host executors remain separately controlled.

Verification:

- `git diff --check` — PASS.
- Corevia Standards repository validator — PASS.
- Static source review of every covered preserved native entrypoint — PASS.
- Rust tests are defined in TestSpec but could not run on this host because the Rust toolchain (`cargo`/`rustc`) is not installed.

Evidence:

- `docs/evidence/corevia-desk-phase-3.md` and `docs/evidence/corevia-desk-phase-3.fa.md`.

### CD-4 — Nuxt Atomic Design View Layer

Status: Complete.

Outcome:

- The approved `corevia-desk-ui` template becomes a real Nuxt component system inside `corevia-desk/web`.

Deliverables:

- A Nuxt 3 application in `web/` with the raw Corevia Desk visual assets copied into the runtime layer.
- Atomic components under `components/atoms`, `components/molecules`, `components/organisms`, and `components/templates`.
- Shared workspace, dashboard, devices, sessions, and settings surfaces with Persian/English and dark/light presentation state.
- Explicit Full Desk and Customer Support page entrypoints. Customer Support does not expose outbound connection, device-book, or session navigation.
- `web/scripts/validate-atomic-design.mjs` and `web/README.md` documenting and checking the composition boundary.

Required structure:

```text
web/
  components/
    atoms/
    molecules/
    organisms/
    templates/
  layouts/
  pages/
```

Acceptance:

- Pages compose Templates, Templates compose Organisms, and lower layers are not duplicated at page level.
- The visual source remains `corevia-desk-ui`; the runtime UI lives only inside `corevia-desk/web`.
- The web UI uses fixture data only and does not bypass the Rust ACL or claim that the Nuxt-to-Rust bridge is complete.

Verification:

- `npm run lint:atomic` from `web/` — PASS.
- `npm run build` from `web/` with the default Full Desk profile — PASS.
- `COREVIA_DESK_PROFILE=customer-support npm run build` — PASS.
- Local browser smoke check for Full Desk, Customer Support, settings, devices, sessions, language direction, and permission dialog — PASS.
- `git diff --check` — PASS.

### CD-5 — Nuxt to Rust Bridge

Status: Implemented — local contract, browser adapter, and native host binding are in place.

Outcome:

- Nuxt uses approved Rust capabilities through a separate bridge; no legacy UI bridge is part of the active path.

Deliverables:

- `src/corevia_bridge.rs` resolves a fixed operation catalog and passes every
  decision through `src/corevia_acl.rs`.
- `docs/contracts/corevia-desk-bridge.schema.json` defines request, response,
  session, event, and safe-error shapes.
- `web/composables/useCoreviaBridge.ts` uses an injected local WebView/agent
  bridge and keeps the raw UI usable with a non-native fixture fallback.

Acceptance:

- Nuxt requests are accepted only after the shared Rust ACL decision; the
  bridge has no generic Rust, shell, URL, SQL, or repository passthrough.
- Browser and local-agent limits are explicit, and the adapter does not import
  or depend on a legacy UI bridge.

Verification:

- Nuxt Atomic lint and Full Desk production build — PASS.
- Rust bridge source is wired as a public module and includes ACL boundary
  tests; native host and ACL tests pass on macOS arm64.
- Local browser continues to render the unchanged raw design with fixture data.

### CD-5A — Native WebView Host and Runtime Wiring

Status: Implemented locally for macOS arm64 — native runner smoke checks and protected release execution remain.

Outcome:

- A concrete native Rust WebView host launches the Nuxt view inside the desktop
  application and connects it to the existing Rust bridge.

Deliverables:

- A checked-in native host entrypoint and its selected WebView dependency.
- A reliable packaged-asset loading path for `web/.output` in development and
  release builds.
- Injection of `window.__COREVIA_DESK_BRIDGE__` through the native host, with
  every request continuing through the Rust ACL.
- The inherited Sciter/Flutter paths remain preservation-only and are not
  selected by the active desktop launcher or release package.

Acceptance:

- Launching the desktop binary opens the Nuxt UI rather than Sciter or Flutter.
- A bridge request from the running Nuxt page reaches Rust and returns the
  expected accepted/denied result without a public control endpoint.
- Full Desk and Customer Support each launch with their correct restricted or
  full surface.
- The release bundle contains everything needed by the selected host and does
  not merely place a standalone web directory beside an unrelated binary.

Verification:

- Build and launch both profiles on the first target platform, prioritising
  macOS as the initial native proof. The local macOS arm64 build now produces
  a real `Corevia Desk.app` for each profile.
- Smoke-test window launch, packaged asset loading, bridge calls, profile
  separation, close/reopen, and the required desktop permissions. The binary
  launch, visual window inspection, live remote frame, and Rust bridge tests
  pass locally on the unlocked macOS session.
- Repeat the same smoke checks on Windows and Linux native runners before CD-12
  is marked complete.

### CD-6 — MCP Control Layer

Status: Implemented — bounded stdio adapter ready; native binding pending.

Outcome:

- MCP controls approved Rust capabilities through named, permissioned tools.

Deliverables:

- `mcp/src/catalog.mjs` contains separate Customer Support and Full Desk tool
  catalogs.
- `mcp/src/policy.mjs` validates arguments and approval references before the
  injected Rust dispatch function can be called.
- `mcp/src/server.mjs` implements only `initialize`, `tools/list`, and
  `tools/call`; the default bridge reports `local-agent-required`.

Acceptance:

- No unrestricted Rust, system command, endpoint, repository, or SQL proxy exists.
- Customer Support and Full Desk have separate effective tool policies.

Verification:

- `npm --prefix mcp test` — PASS (5 tests).
- Tests cover profile separation, approval requirements, unknown tools, and
  the absence of arbitrary system-command forwarding.

### CD-7 — CI/CD and Nexus Release Pipeline

Status: Implemented locally — macOS arm64 runnable bundles are produced for both profiles, and the two immutable CI base images are published/read back for `linux/amd64`; protected CI for the current commit and product artifact publication remain, while signing is optional.

Outcome:

- GitLab CI builds versioned desktop packages and the Full Desk Linux server
  image, then publishes them to Nexus.

Deliverables:

- `.gitlab-ci.yml` builds the exact requested matrix: Customer Support on
  Windows and macOS; Full Desk on Windows, macOS, Linux, and as a Linux Docker
  image.
- `release/build-profile.mjs` builds Rust plus the static Nuxt output for one
  matrix entry and creates a macOS `.app`; `archive-profile.mjs` creates the
  transport package.
- `release/create-manifest.mjs` and `validate-manifest.mjs` provide immutable
  identity, checksum, and verification gates; signing remains optional and is
  not required by CI.
- `release/publish-nexus.mjs` refuses to overwrite an existing versioned path.
- Official Pipeline readback through Corevia Bridge reached `verify:contracts`
  with `node:22-bookworm` pulled successfully, then stopped at old commit
  `2fc0e57f` because `package.json` and `package-lock.json` were out of sync
  during `npm ci`. The two immutable `linux/amd64` base images are now
  published and read back in Nexus; no unverified internal image was substituted.

Acceptance:

- Customer Support has separate Windows and macOS artifacts. Full Desk has
  separate Windows, macOS, and Linux artifacts plus its Linux Docker image.
- Every artifact has platform, architecture, version, checksum, and profile identity.

Verification:

- `npm --prefix release test` — PASS.
- All release scripts pass Node syntax checks and use Nexus secrets from CI only;
  a signing secret is read only if signing is explicitly enabled. No secret is
  stored in this repository.
- Local macOS arm64 native builds and manifest validation pass for both
  profiles. The Windows release-path fix is covered by the release tests, and
  the official base-image resolver/publish/readback passes for `linux/amd64`.
  A new protected CI run on the current commit and product Nexus publication
  are not claimed here; they remain external CI gates.

### CD-8 — Customer Support Product

Status: Implemented locally — profile, UI, ACL, MCP, and release contract verified; protected Nexus publication remains external.

Outcome:

- A restricted customer package exposes only customer settings, connection approval, and connection permissions.

Acceptance:

- No unrelated-system connection, unrestricted file transfer, terminal, or arbitrary system command is exposed.

Verification:

- Customer Support UI surface gate — PASS.
- MCP profile tests — PASS; outbound connection and extra connection permissions are rejected.
- Rust ACL contains a separate Customer Support capability contract; targeted
  Bridge and ACL tests pass 11/9 for both profiles. A fresh incoming request was
  approved through the product and remote readback returned an active session.
  Native Customer Support
  Windows Cargo build, archive, manifest, and executable smoke pass on the
  C/Windows SDK runner, and headless native MCP status/identity/settings/
  sessions plus Secret/SystemCommand denies pass. Its portable process launch
  was observed in Session 2; UI Automation did not return reliable output, so
  side-by-side install and visual Windows UI remain open.

### CD-9 — Full Desk Product

Status: Implemented locally — Full Desk profile, full MCP catalog, and independent release identity verified; protected Nexus publication remains external.

Outcome:

- A separate Full Desk package exposes all approved RustDesk capabilities and the full MCP catalog.

Acceptance:

- Full Desk has an independent Nexus artifact and release path.
- High-risk actions retain approval and audit controls.

Verification:

- Full Desk capability and approval contract tests — PASS.
- Release profile contract keeps Full Desk separate from Customer Support.

### CD-10 — Corevia Bridge Operations

Status: Contract ready — source-driven lifecycle is defined; product-specific executor extension remains `extension-needed`.

Outcome:

- Server-side installation, health, update, rollback, and runtime configuration are source-driven and evidenced.

Acceptance:

- Bridge does not own Rust, Nuxt, MCP, or product authorization rules.
- Missing operational executor support is reported as `extension-needed`; manual mutation is not a fallback.

Verification:

- Bridge lifecycle contract records Nexus identity, health, update, rollback, and runtime readbacks without credentials.
- No remote mutation is claimed without a matching Corevia Bridge executor.

### CD-11 — Mobile Follow-up

Status: Deferred.

Outcome:

- Android and iOS packaging are reconsidered after desktop, browser, ACL, MCP, and Nexus paths are stable.

Acceptance:

- Mobile does not delay desktop or browser delivery.

### Phase CD-12 - Desktop WebView Release Verification

Status: Open for final delivery — the native host, macOS arm64 bundle, unlocked
GUI smoke, real ID/Relay session, Full Desk Windows checks, and Customer Support
Windows artifact/ACL checks are verified locally. A fresh ID/Relay session reached
Online with a real Windows frame, destination approval, and correct Remote
Cursor/View Only toggles; Clipboard transfer and independent `/r` requests to
Windows and Customer Support also reached simultaneous approved Active with
`direct:false`/`secured:true`. On Mac, three consecutive post-fix requests appeared
in visible Customer Support without route navigation in 1.644 s, 1.425 s, and
1.668 s, and were rejected; one earlier pre-restart ~6 s no-show remains
inconclusive because visibility was not proved. These checks do not close P0 or
the full P1 UI/data matrix. Customer Support Windows visual/side-by-side install,
approved active image/input/close/reconnect coverage, positive Secret disclosure,
live P3 GUI, protected CI/Nexus readback, and Linux verification remain.

Outcome:

- The active desktop path is Rust plus Nuxt inside a native WebView, with two separate release profiles.

Deliverables:

- Stable, separately identified Full Desk and Customer Support packages for macOS and Windows, backed by the verified acceptance matrix.
- Evidence for native UI/data behavior, MCP policy, single-instance launch, and governed CI/Nexus publication.

Tasks:

- [x] Verify post-fix Customer Support incoming-request refresh on a live ID/Relay run: after restarting and raising Customer Support on Mac, three consecutive requests appeared without route navigation in 1.644 s, 1.425 s, and 1.668 s; all were rejected with no access granted. Preserve the inconclusive pre-restart ~6 s no-show in the evidence.
- [ ] Complete the native UI, data, persistence, and error/empty/loading matrix for both profiles on macOS and Windows, including Customer Support visual and side-by-side installation.
- [ ] Complete Windows repeated-launch, restore, and side-by-side single-instance checks for both profiles.
- [ ] Finish the current-source MCP approval, denial, audit, elicitation, and executor matrix for both profiles. A local stdio host's real 120.042-second no-response timeout now fails closed without native dispatch; interactive SDK decisions, audit, sensitive native operations, and some executors remain open.
- [ ] Verify notification states and controlled positive Secret disclosure across the four desktop variants without recording secret contents.
- [ ] Resolve the exact-current-commit CI/Nexus executor gap through Corevia Standards, Corevia Bridge, and `connectcore`; publish and read back immutable artifacts only through the declared executor.
- [ ] After the four desktop variants pass, complete the Full Desk Linux and Docker build, UI/runtime, architecture, and image readback checks.

Acceptance:

- Linux, macOS, and Windows targets are represented in CI; macOS arm64 is explicitly supported by the release contract.
- Each target launches the real Nuxt-in-WebView desktop application, not only a
  Rust binary plus an adjacent web directory.
- A manifest requires matching profile, platform, architecture, version, runtime,
  and checksum metadata; a signature is optional when a signed release is used.
- Flutter and Dart are not installed, invoked, or required by active build/test jobs.

Verification:

- `npm --prefix release test` — PASS (8 tests).
- `npm --prefix release run validate-scope` — PASS.
- Full Desk and Customer Support Nuxt production builds — PASS.
- Native Rust build, packaged Nuxt assets, and native bridge unit smoke tests — PASS on macOS arm64.
- Visual GUI smoke test — PASS on the unlocked macOS session with a real Windows
  frame and accessibility text visible inside the Nuxt UI.
- Native Windows Full Desk smoke/build — PASS on the supplied host through WinRM.
  Customer Support Windows native Cargo build, archive, manifest, and smoke also
  pass on the C/Windows SDK runner; its portable process launch was observed in
  Session 2, while visual UI readback and side-by-side install remain open.
  Linux smoke/build remains pending on a native runner.
- Targeted `cargo test --locked --release --lib corevia_bridge` and `corevia_acl` — PASS (11 Bridge tests and 9 ACL tests); both groups also pass with the Customer Support feature.
- Nuxt build/generate, macOS arm64 smoke, and manifest checks — PASS for both profiles.
- Customer Support Windows manifest and native executable smoke — PASS on the
  Windows runner; PE smoke on macOS fails with `ENOEXEC` as expected and is not
  counted as Windows UI evidence.
- Native headless Customer Support MCP readback — PASS for the five safe reads,
  missing-session approval without execution, and deny rules for temporary password
  and system command.
- Repeated same-ID requests reuse the existing pending/active native session and
  do not create a duplicate remote session.
- Native `/r` Relay selection was exercised against independent Windows and
  Customer Support destinations; both reached approved simultaneous Active with
  `direct:false` and `secured:true`, stayed stable for over 20 seconds, and closed
  cleanly, closing the concurrent two-destination P1 gate.

Evidence:

- `docs/evidence/current-status.md`
- `docs/evidence/corevia-desk-p1-ui-data.md`
- `docs/evidence/corevia-desk-single-instance.md`
- Sanitized native, MCP, CI, and Nexus reports under the declared `.work` evidence roots.

## Validation

- Corevia Standards repository validator.
- `roadmap.create-or-update` values validation and plan.
- `repo-specs.create-or-update` values validation and plan.
- `git diff --check`.
- Before later implementation phases: the matching MCP contract and Bridge source/capability/governance checks.
- Active Rust + Nuxt/WebView scope and profile-surface gates.

## Update Policy

Allowed:

- Refine tasks, acceptance, and evidence after verified findings.
- Add implementation detail without changing ownership or the selected architecture.

Requires a new decision:

- Change the Rust Core, ACL, Nuxt, MCP, Nexus, Installer, or Bridge ownership boundary.
- Move mobile ahead of desktop-first delivery.
- Add a mandatory central Gateway or public control endpoint.

## Changelog

- 2026-10-05: After fixing startup appearance sync, UI review found fabricated OS-permission states and unhandled settings/recheck actions. The dialog now provides truthful guidance; profile/atomic lint and the production web build pass, while fresh native-bundle readback of both fixes remains open. A recoverable 1.4 GiB VS Code update cache was moved to Personal, raising macOS Data free space from 1.8 to 3.7 GiB; active Chrome and system install assets were preserved.
- 2026-10-05 continuation: Fixed and verified the Full Desk macOS Sessions connection dialog in native `test11`; strict smoke passed. Release (10/10), MCP (22/22), and Rust (125/125 per profile) passed. Removed this build's exact 2.1 GiB temporary Cargo target while keeping the 24 MB app bundle; storage then showed about 53 GiB free on `/` and Personal. Docker images/cache were untouched; release acceptance remains open.
- 2026-10-04: Locked the four macOS/Windows desktop variants, verified current Desk refs and the official Bridge Source Audit, marked older P1/MCP results as historical evidence, recorded the current CI capability blocker, and moved Linux/Docker to lower priority.

- 2026-09-17: Reconciled the official Traefik source for the Windows runner; HTTPS 302, runner API 200 through both paths, a Running service, and `gitlab-runner verify` `rc=0` were read back. The macOS vcpkg environment and Linux `libopus-dev` prerequisite were verified; current-commit publication and a new CI run remain open.
- 2026-09-17 continuation: Official `v1.5.21` pipeline `3485` passed contracts and both macOS builds, while both Windows jobs failed before build during long-path checkout cleanup. System Git `core.longpaths=true` was then applied and read back through the official Runner source/executor; the next commit/tag and full CI/Docker/Nexus matrix remain open.

- 2026-09-10: Through Corevia Standards → Corevia Bridge → `connectcore`, resolved Docker Hub `node:22-bookworm` and `debian:trixie-slim` with immutable index/platform digests, published both to Nexus for `linux/amd64`, and read back exact destination digests. The governed `gitlab-branch.publish` executor was then added and plan-verified; its clean apply, current-commit run, and product artifact readback remain open after the old-commit lockfile failure.

- 2026-09-02: Added the Corevia Standards bootstrap and accepted the Rust Core, ACL, Nuxt, MCP, Nexus, Installer, Bridge, and desktop-first architecture baseline.
- 2026-09-02: Completed CD-3 with the shared Rust ACL, Customer Support profile gate, Full Desk default profile, approval references, redacted audit, and Flutter/Sciter enforcement.
- 2026-09-02: Completed CD-4 with the Nuxt Atomic Design view layer inside `corevia-desk/web`, sourced from `corevia-desk-ui`, including both product profile entrypoints and successful builds.
- 2026-09-02: Implemented CD-5 through CD-7 with the Rust bridge contract, bounded MCP adapter, profile-separated release matrix, checksum gates, and Nexus publisher.
- 2026-09-03: Rebased the active desktop decision on Rust plus Nuxt/WebView only; preserved inherited native UI sources are not part of active build, CI, installer, or release work.
- 2026-09-03: Implemented the Customer Support restriction gate, explicit two-profile release contract, platform matrix checks, and local CD-8 through CD-12 verification gates.
- 2026-09-03: Added and locally built the CD-5A Rust WebView host, static Nuxt packaging, macOS `.app` bundle, and two-profile native bridge checks; CD-12 remains open until GUI and native runner checks pass.
- 2026-09-07: Separated the macOS application identifiers and display names for Full Desk and Customer Support, added release-contract coverage, rebuilt both profiles, and read back both `Info.plist` files; live Full Desk theme, direction, permissions, Info, and copy-success checks passed, while Customer Support visual launch and positive Secret disclosure remain open.
- 2026-09-05: Passed the primary Full Desk Mac↔Windows ID/Relay smoke test with a live Windows frame, real mouse control, product close, MCP state readback, reconnect, and more than 20 seconds of stable Console capture; the P1/P2 feature matrix and protected CI/Nexus publication stay open.
- 2026-09-05: Reinstalled the Release macOS binary after adding the native same-ID duplicate-session guard; final smoke showed a live Windows frame, remote Start-menu control, one active MCP session, and 7 passing targeted Rust ACL/MCP tests.
- 2026-09-06: Resolved the official Bridge source-catalog drift through the permitted catalog write, fast-forwarded `codex/desk-phases-8-12` -> `develop` -> `main`, and recorded the P1 UI/data evidence; live Clipboard and simultaneous-active-session gates remain open.
- 2026-09-07: Exercised the native `/r` Relay selector end to end; the Windows destination approved it and read back an Active session with `direct:false` and `secured:true`. The independent Customer Support endpoint stayed offline in the concurrent attempt, so P1 remains open; temporary bridge/process/task/transfer artifacts were cleaned.
- 2026-09-07: Closed the core P1 gate with two independent `/r` sessions to Windows and Customer Support; both destinations were approved, simultaneously Active, relayed, secured, stable for over 20 seconds, and then cleanly closed. Controlled headless server support was used only for the locked macOS acceptance runner; side-by-side Customer Support UI, positive Secret, Linux/Docker, and CI/Nexus remain open.

## Metadata

- Status: Active
- Owner: Platform Engineering
- Standard: `corevia-standards@0.1.0`
- Current open phase: rebuild/read back the native macOS bundle containing the
  startup-appearance and permission-guidance fixes, then finish UI/data and
  Customer Support Windows visual installation; revalidate current-source MCP,
  positive Secret, and P4 CI/Nexus; Linux/Docker remains lower priority. CD-12 remains open
  until native runner, Nexus readback, optional signing checks, and any
  product-specific Bridge executor extension are resolved
