[فارسی](./acceptance.fa.md)

# Corevia Desk Acceptance Matrix

Last Updated: 2026-10-08
Status: Active
Owner: Platform Engineering

## Purpose

Prevent a visual mock, a handshake, or a local plan from being mistaken for a working product.

## Gate order

The exact current Customer Support bundle remains at `screen capture=false` after its own request and restart, although macOS shows the corresponding screen/audio entries enabled. This Mac-specific result stays open, but is not a stop condition: shared capture behavior may be accepted through the owner-approved, clearly labeled Windows/other-OS proxy. The proxy must still pass on an isolated desktop over Peer ID/ID-Relay.

Latest readback, 2026-10-08: the older, permission-authorized Customer Support bundle again showed a real frame over encrypted Relay. Keyboard/mouse permission was briefly accepted for one session, but no remote input was sent; personal browser content was visible, so testing stopped immediately and the session closed. No live Toolbox/MCP operation ran. This is same-Mac evidence from an older bundle, not acceptance of current Customer Support, whose capture preflight remains `false`. Current UI reads Active=0. P0/P1/P2 and independent Windows remain open; live tools require an isolated test desktop.

Automated rechecks on 2026-10-08: Web 15/15, MCP 27/27, Release 10/10, and Atomic/Profile lints passed. They do not substitute for the open native acceptance gates.

Earlier readback, 2026-10-06 (historical): Screen Capture was re-registered for the exact bundle at that time; Full Desk received a real Peer-ID/ID-Relay frame and restored it after reconnect. MCP also read back and safely closed an active encrypted Relay session. This was same-Mac evidence and does not replace current-bundle or independent-host acceptance.

Product-owner decision, 2026-10-07: the missing Mac signing certificate is not a reason to stop. Shared behavior that cannot be exercised on macOS solely because of its signing/permission gate may be accepted using an equivalent test on Windows or another supported OS; label this evidence explicitly as a cross-platform proxy, not as direct macOS permission proof. Mac capabilities that do not need signing must still work and be tested on the Mac itself.

Windows route review, 2026-10-08: official Standards Validate/Plan passed locally without connecting. The matching Bridge pattern is draft and its executor uses HTTP Basic with certificate validation disabled, so it was not executed. A later read-only WinRM inventory succeeded with NTLM message encryption, but the Customer Support GUI session is disconnected. The self-signed RDP certificate shown by Windows App exactly matches the listener thumbprint read through WinRM; “Always trust” remained off and the warning was canceled before password entry. The active blocker is an isolated interactive desktop and explicit one-time approval to continue past that self-signed certificate, not WinRM connectivity or Mac signing. See [safe-route evidence](../evidence/corevia-desk-windows-safe-access-20261008.md) and [WinRM readback](../evidence/corevia-desk-windows-winrm-readback-20261008.md).

| Gate | Requirement | Status | Evidence / next action |
| --- | --- | --- | --- |
| P0 | Full Desk macOS/Windows, peer ID via ID/Relay, live frame, input control, close and reconnect | Open/partial; encrypted Relay/image passed on the older authorized bundle, not current Customer Support. The exact current bundle still reads capture=false after restart. The owner-approved Windows proxy has not passed. The input toggle was accepted once but no key/mouse action was sent. WinRM read-only access works, but Customer Support is in a disconnected session and RDP trust is unresolved; no RDP credentials were submitted. Independent Windows image, input, stability/reconnect remain unpassed | Establish a trusted, isolated interactive Windows desktop without bypassing certificate checks; test end-to-end with independent Peer IDs. Continue Mac capabilities that do not require signing. IP or same-network access is not a substitute |
| P1 | Every screen/control in all four variants, real actions, data, sessions, persistence, error/empty/loading states, responsive UI and RTL/LTR | Open; Devices has one Mac row (`528700924`, last success Oct 7 2:07 PM) and one Windows row (`298636616`, Sep 7 10:08 PM). Sessions reads Active=0 / two recent; the latest Mac-peer attempt is Failed (Oct 8 11:44 AM), separate from the synthetic `abc-invalid` failure. No duplicate or last-success timestamp update occurred. Live tools, restart persistence, and Windows variants remain open | Recheck the session-state transition on an independent host; verify persistence and complete the full UI/data matrix on all four variants, especially both Windows editions |
| P1.5 | Install and run Full Desk and Customer Support on macOS/Windows with separate identity/ACL, side-by-side support, and one GUI instance per profile | Open; forced relaunch of each current Mac bundle left one main process and the window remained readable; current focus restoration is unconfirmed | Customer Support Windows still lacks reliable side-by-side install and visual verification; Windows and Full Desk Linux single-instance behavior also need native tests. Build and exercise every supported combination from current source |
| P2 | Every MCP tool with shared ACL, permissions, approval, audit, and safe denial | Open/partial; current Mac bundles returned catalogs 25/9 and five safe reads per profile were accepted through the live MCP-to-Rust bridge; MCP tests pass 27/27. A live `start_connection`/`close_session` was previously read back over encrypted Relay. No tool ran in the latest permission-boundary test; Camera/Printer/Service and individual live-tool execution remain open | Complete the 25/9 tool matrix on an isolated test desktop with allow/deny/cancel and safe readback; mutations only on test data and with per-call approval |
| P3 | Success/Warning/Danger/Info notifications and controlled Secret disclosure | Open | Use controlled test data for positive Secret readback and never record the Secret; visually verify notification states in all four variants |
| P4 | Protected CI, immutable publication, and Nexus readback for all four macOS/Windows artifacts | Open; Desk remote branch refs already match at `248afe1b`, but current-commit pipeline Values/executor are missing. Bridge catalog audit passes only in an isolated snapshot after an unpublished generated-hash refresh | Resolve/publish the governed Bridge catalog update and Standards/Bridge current-pipeline capability; manual pipeline or publish is forbidden |
| P5 | Full Desk Linux and Linux Docker for the server's actual architecture | Lower priority than macOS/Windows delivery | Start only after all desktop gates pass |
| P6 | Possible VS Code extension | Deferred | Outside the delivery gate |

### Required live-session Toolbox gate

In an approved active Full Desk session, connect every displayed Toolbox item to
its real action and test each separately: monitor selection, video refresh/
resolution, image quality, scroll style, remote cursor, view-only, keyboard and
mouse, Clipboard, file transfer, audio, terminal, fullscreen, elevation request,
and end session; also verify toolbar expand/collapse. Record success, denied or
unsupported feedback, and native state readback for every action. UI state must
not change optimistically before native acceptance. Use isolated test data and
safe read-only terminal checks. Customer Support may expose only tools allowed
by its inbound-only profile. This P1 gate stays open until the live test passes.

## Single-instance launcher gate

For every supported native desktop combination, repeated launcher clicks must
bring the existing main window forward; reopening after minimize must restore
that same window, with no more than one main process per profile. Full Desk and
Customer Support must still be able to run side by side. Customer Support uses
a fixed mobile-sized desktop window; Full Desk keeps its wide, resizable window.

| Combination | Status |
| --- | --- |
| Customer Support / macOS | Partial; forced current-bundle relaunch kept one process/window; current focus restoration not confirmed |
| Full Desk / macOS | Partial; forced current-bundle relaunch kept one process/window; current focus restoration not confirmed |
| Customer Support / Windows | Open; native test required |
| Full Desk / Windows | Open; native test required |
| Full Desk / Linux | Open; native test required, lower build priority |
| Linux Docker / CLI / service | No GUI window; outside the desktop-launcher gate |

See the [single-instance evidence matrix](../evidence/corevia-desk-single-instance.md).

## P0 regression procedure

1. Launch Full Desk on macOS and Windows; verify both are the actual Corevia Desk product, not RDP or a browser-only page.
2. Confirm both product processes are running.
3. Enter the peer ID and connect through the configured ID/Relay route. Do not use IP as the test identity.
4. Approve the incoming request through the product/approved MCP flow.
5. Verify active state, continuous remote frame, mouse, keyboard, close, and reconnect.
6. Record the destination session readback and sanitized logs. Never record server keys or passwords.

## P1 UI matrix

Prior evidence records the UI, data, responsive, RTL/LTR, settings, Devices,
Sessions, Empty State, persistence, two-way Clipboard, and two simultaneous
sessions. Treat it as historical evidence; current-source P1 remains open until
every screen and interactive control is traced to and exercised through a real
product action in all four Full Desk/Customer Support × macOS/Windows variants.

The latest no-native-host preview covered the empty Sessions state, new
connection dialog, localized blank peer ID, and rejected test-only ID; no fake
session was created. The inert “more tools” and “clear history” controls were
removed. Native success/rejection behavior of session-tab close still requires
a real active session and remains open.

The latest production preview also read back Customer Support checked/disabled
permission switches, Devices blank validation and rejected-save/no-ghost-record,
empty filters/search, blank Sessions connection/no fake history, Network
unlock/relock without save, rejected setting rollback, and Persian notification
after a rejected locale write. These are browser-only checks without a native
host; they do not satisfy native action, install, persistence, or four-variant
acceptance.

The latest-source browser pass covered Full Desk Home, Devices, Sessions, Settings,
Customer Support, and Customer Support Settings at 390×844, 768×1024, and
1280×720 with no horizontal overflow. The Quick Connect shortcut now focuses its
Peer ID field; an empty connect stayed validation-only, and incomplete device save
created no record. This remains browser-only; native focus and every real action
must still be verified.

All six Full Desk settings categories were inspected. Test writes for Light theme
and Best image quality were rejected without the native host and retained the
previous Dark/Balanced values. Sidebar collapse/expand worked with both mouse and
keyboard and restored its accessible labels. These remain preview-only results.
With no local identity, the ID-copy control also reads as disabled; successful
copy of an actual identity remains open for native acceptance.

Fresh native readback on 2026-10-05 covered Online/Favorites filters, no-result
search, restoring All, blank Peer ID validation, and every Settings category;
records and persistent settings were untouched. Home initially rendered the
Persian/dark defaults, while Settings read saved English/light values. The source
now applies saved appearance from the shared runtime; profile/atomic lint and the
production web build pass, but a fresh native bundle is still required to accept
the fix.

In the native Full Desk Mac bundle `1.5.36-dialog-compact-test11`, visual and
accessibility readback confirmed one title and a single-column Peer ID form in
the Sessions dialog; Home retained its own copy and the Quick Connect shortcut
focused Peer ID. Blank Peer ID showed validation only and created no session or
network request. This does not replace the four-variant matrix, successful
connection, persistence after restart, or Windows UI acceptance.
When the runtime or network route is unavailable, status must remain accurate
and neutral rather than implying readiness or an ID/Relay connection; the fresh
preview read back this unavailable state.
In Customer Support with no incoming request, the status dot must be neutral and
Review permissions/Reject must be disabled in accessibility and visually; the
real request flow remains open.

Persian/RTL ↔ English/LTR switching and restoration were checked at desktop width
in preview; mobile/tablet widths and all four native products remain acceptance
gates.

The current source now separates the address book from successful-connection
recency: a newly saved peer remains in Devices but does not appear in Home
recents until a successful connection; reading pending or disconnected sessions
must not advance that timestamp. Session history must carry the native end time
and sort newest-first. In the UI, group terminal events by normalized Peer ID
and show one row with the latest state and native end time per device; `/r` and
case differences must not create extra devices. If Peer ID is absent, fall back
to address/name; keep unidentified session IDs separate. This is a display
projection and must not delete native events. The state must be a noninteractive
labeled chip, not an X/cross that resembles a close action. Verify populated
cards and restart behavior natively across all four variants before closing P1.

Test at mobile, tablet, and desktop widths in both RTL and LTR:

- devices, sessions, settings, recent connections;
- a new saved peer stays out of recent connections until success; a successful
  ID/Relay connection persists its recent timestamp across restart, and a
  disconnected session cannot refresh it;
- session-history end times and newest-first order agree with native session
  readback;
- save, restart, reload, and verify data is not lost or duplicated;
- concurrent sessions and correct session ownership;
- clipboard copy/paste using normal keyboard shortcuts;
- Toolbox actions and their disabled/unsupported states;
- empty-state icons centered inside their square boxes;
- exact visual spacing, weight, alignment, mirroring, and preserved icons;
- no accidental underlines or cross-page cards/features;
- responsive layout without changing the supplied design.
- For every button, field, menu, and control, record the real action, success,
  failure/permission-denied, and applicable loading/empty state; inert or visual-
  only controls fail.
- Customer Support must keep the same controls within its restricted capability
  boundary; Full Desk-only actions must not be reachable from its UI or MCP.

## P2 MCP matrix

- Same Rust ACL and redacted audit path as the product.
- Full Desk: list, approve, connect, inspect, control, permission changes, close, and supported Toolbox actions, each with its approval rule.
- Customer Support: only customer settings, incoming approval, and connection permissions; outbound and unrelated-system operations must be denied.
- Unknown tools, malformed arguments, missing approvals, system commands, and arbitrary proxy attempts must fail closed.
- A caller-provided approval reference is never human-consent evidence. Sensitive
  MCP calls require negotiated host form elicitation and a one-time,
  operation/payload-bound proof verified by Rust. Unsupported clients fail
  closed; the actual elapsed timeout is now verified with a local stdio host, but
  verify interactive accept/decline/cancel and all remaining paths before closing P2.

Prior local evidence records 10 Node tests and all 34 published members passing
from `tools/list` through `tools/call`. Keep that as history; rerun every tool,
allowed mutation, approval, audit, and denial path for both profiles on the
current source before closing P2.

## P4 release matrix

| Profile | macOS | Windows | Linux | Linux Docker |
| --- | --- | --- | --- | --- |
| Customer Support | required | required | out of requested matrix | out of requested matrix |
| Full Desk | required | required | required, lower priority | required, lower priority |

Only macOS and Windows for both profiles belong to the first delivery gate;
Linux and Docker move to lower-priority P5.

Each result must include profile, platform, architecture, version, checksum, launch result, and Nexus readback. A signature may be present but is not required.

## Non-completion rules

- A successful Login/Handshake alone is not P0.
- A browser fixture is not a real feature.
- MCP evidence cannot replace product UI evidence.
- A CI file, branch, or plan is not a published artifact.
- Do not close CD-12 while P1, P2, P4, or their evidence remain open.
- Do not close desktop delivery until P1, P1.5, P2, P4, and current evidence for
  all four variants pass.
- Browser-only smoke, MCP, IP/RDP, old commits, and network co-location are not
  substitutes for real product acceptance on the current source.
- Never trigger a pipeline, tag, or Nexus publish outside Corevia Standards →
  Corevia Bridge → `connectcore`; report a missing executor as a blocker.

## Change Log

- 2026-10-05: Found and fixed an inert Full Desk Quick Connect shortcut; it now focuses the Peer ID field. Six routes passed browser overflow checks at three widths; empty connect/device validation created no records. Profile/atomic lint and production web build pass. Fresh native readback and the complete four-variant matrix remain required for P1. Earlier targeted cleanup moved 1.4 GiB of staged VS Code update data to Personal.
- 2026-10-05 continuation: Fixed and natively verified the Full Desk Mac Sessions connection dialog in `test11`; strict smoke, Release (10/10), MCP (22/22), and Rust (125/125 per profile) passed. Blank Peer ID created no session. The full P1 matrix remains open.
- 2026-10-04: Locked the first delivery to Full Desk and Customer Support on macOS and Windows; all UI controls, current-source MCP, and CI/Nexus remain open pending fresh evidence. Linux/Docker is lower priority and VS Code is future scope.

- 2026-09-06: Recorded the real P1 matrix, MCP matrix, Notification/Secret-eye implementation, and native Customer Support Windows evidence; concurrent-session, install/UI, and native/CI gates remain open.
- 2026-09-07: Rejected stale-profile macOS Customer Support packaging, isolated native release targets, rebuilt the correct profile, and recorded safe native readback for both macOS profiles; P1.5 install/UI and external readback remain open.
- 2026-09-07: Connected the real MCP Server to both macOS native profiles and passed the 25/9 catalog listing plus complete mutation/approval/audit/deny readback for each profile; P2 is complete locally.
- 2026-09-07: Read back real Clipboard content in both directions; the macOS-to-Windows marker `COREVIA_P1_CLIP_UI_20260907_G` passed, the profile-native namespace fix produced an independent second Peer ID, and destination approval plus simultaneous Active sessions remain open in P1.
- 2026-09-07: Closed the P1 concurrent-session gate with two independent approved Peer-ID `/r` destinations (Windows and Customer Support); both were simultaneously Active, relayed, secured for over 20 seconds, and cleanly closed.
- 2026-09-07: Completed native MCP mutation/approval/audit/deny readback for both profiles and locally closed P2; external artifact publication remains governed under P4.
- 2026-09-10: Resolved, published, and read back the CI base images through the official Standards → Bridge → `connectcore` path for `linux/amd64`; the governed `gitlab-branch.publish` executor was added and plan-verified, while the available pipeline pulled Node successfully but failed at old commit `2fc0e57f` on a lockfile mismatch, so clean apply, the current-commit run, and product artifacts remain open.
- 2026-09-05: Marked P0 complete from the real macOS-to-Windows ID/Relay smoke and split the remaining work into explicit gates.

## Ownership

Platform Engineering, with product verification owned by Corevia Desk and operational publication owned by Corevia Bridge.
