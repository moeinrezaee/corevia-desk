# Corevia Desk MCP adapter

This is a sibling adapter, not a second Rust core. It exposes a small MCP tool
catalog and forwards only named operations to the local Rust bridge.

For a packaged desktop process, set `COREVIA_DESK_RUST_BRIDGE` to the Corevia
Desk executable. The adapter starts it with `--corevia-mcp-bridge` and exchanges
newline-delimited JSON over local stdio. A native host may also inject
`createRustBridgeAdapter({ dispatch })`, where `dispatch` is the bounded Rust
binding for `src/corevia_bridge.rs`. Without either local bridge, the safe
result is `local-agent-required`.

The adapter does not start a shell, open a public socket, proxy an arbitrary
URL, access a repository, or forward SQL. The Rust ACL remains the final
authority for every request.

## Profiles

```sh
COREVIA_DESK_PROFILE=customer-support npm test
COREVIA_DESK_PROFILE=full-desk npm test
COREVIA_DESK_PROFILE=full-desk npm start
```

Customer Support receives status, settings, incoming approval, and connection
permission tools only. Full Desk receives those plus outbound and session tools.
Sensitive tools do not accept a caller-supplied approval reference. After
protocol negotiation, the adapter asks the MCP host for a per-call form
elicitation only when that host advertises the capability; rejection,
cancellation, timeout, an older protocol, or a host without the capability fails
closed. The accepted response becomes an internal, operation-and-payload-bound
one-time proof verified by Rust. A real TypeScript MCP SDK client over stdio has
now verified the packaged native helpers: Full Desk listed 25 tools and passed
five safe reads; Customer Support listed 9 tools and passed four safe reads.
This is read-only client compatibility, not a real elicitation-capable host.
Sensitive-call approval/decline/cancel/timeout and some Toolbox executors remain
unverified. Safe reads continue to follow the Rust ACL.

If Rust authorizes a request but the native result has `executed: false`, the
adapter reports an MCP tool error; authorization alone is not presented as a
successful action. This does not supply missing executors such as Camera,
Printer, or Service—it only prevents a misleading success result.
The `send_chat` tool also obtains separate per-message approval and previews
the sanitized message before dispatch.
Port forwarding accepts an optional `remoteHost`; when omitted it keeps the
`127.0.0.1` loopback default. Host input is bounded and validated before native
dispatch. Per-call approval still applies, and this contract check is not a
live-tunnel test.
The `use_voice_call` tool also accepts an optional Boolean `enabled` field for
explicit start/stop requests. The native bridge rejects non-Boolean values;
this argument-contract test does not prove live voice calling.
