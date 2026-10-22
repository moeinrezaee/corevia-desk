[فارسی](./corevia-desk-bridge.fa.md)

# Corevia Desk bridge contract

The Nuxt view and local MCP adapter use one small, named request envelope.
The Rust ACL remains the only authority for the decision. A request is never
interpreted as a shell command, URL, SQL statement, repository operation, or
arbitrary Rust call.

## Runtime shape

- Nuxt sends `{ requestId, operation, payload, approvalReference? }`.
- Rust resolves `operation` against the explicit catalog, validates the small
  fields that affect permissions, and calls `src/corevia_acl.rs`.
- The response returns `accepted`, `denied`, `unsupported`, or `error`.
- `accepted` means that the ACL authorized the next native executor step; it
  does not claim that the operating-system action has already finished.
- Only a monotonic audit id and safe error codes cross the boundary. Payloads
  and approval contents are not logged or echoed in errors.

The complete machine-readable contract is in
`docs/contracts/corevia-desk-bridge.schema.json`.

## Browser and desktop limits

A normal browser tab cannot safely perform screen capture, input control,
service management, or a remote session by itself. The Nuxt page therefore
stays empty for native-only data and reports that a local agent is required
until it is hosted inside a desktop WebView with the injected Corevia Desk
bridge.

The local agent must expose only this contract and must call the Rust ACL. It
must not expose a public control port. The active desktop UI is the Nuxt path
inside a native Rust WebView. Preserved legacy UI sources are not part of the
active build or release path, and the new contract does not import or depend on
them.
