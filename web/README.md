# Corevia Desk Web View Layer

This directory is the Nuxt view layer for Corevia Desk. Its visual source is
the raw prototype in `../corevia-desk-ui`; the prototype remains separate and
is not the runtime application.

## Composition rule

Every page composes a Template. Templates compose Organisms, Organisms compose
Molecules, and Molecules compose Atoms:

```text
Atoms → Molecules → Organisms → Templates → Layouts → Pages
```

Pages must not copy primitive controls or reach directly into a lower layer.
The `lint:atomic` script checks this boundary without requiring a package
install.

## Profiles

- `full-desk` is the default build and exposes the complete approved desktop
  workspace.
- `customer-support` exposes customer-side settings plus inbound connection
  review and permission selection. Outbound connection, device book, and
  session navigation are not exposed.

Use `COREVIA_DESK_PROFILE=customer-support` when building the restricted
profile. Without a native desktop host, the browser view stays empty instead
of inventing identity, device, or session data. A desktop host injects
`window.__COREVIA_DESK_BRIDGE__`; that binding calls the Rust ACL contract in
`../src/corevia_bridge.rs`.

## Run

```sh
npm install
npm run lint:atomic
npm run dev
```

The web layer is inside `corevia-desk` and is the active desktop UI. It is
hosted by the native Rust WebView shell while Rust remains the source of truth
for system access. The active build does not install or run Flutter/Dart.

The complete bridge contract is documented in
`../docs/contracts/corevia-desk-bridge.md`. The Nuxt bridge is independent of
all preserved legacy UI sources.
