[فارسی](./corevia-desk-release-lifecycle.fa.md)

# Corevia Desk Release Lifecycle Contract

## Scope

This contract defines how Corevia Bridge may operate Corevia Desk desktop
artifacts after CI has produced them. It does not own Rust behavior, Nuxt UI,
MCP permissions, or product capability decisions.

The active desktop runtime is Rust plus Nuxt inside a native WebView. Flutter
and Dart are outside the active build, test, installer, and release path.

## Immutable artifact identity

Every release is addressed by:

```text
corevia-desk/<profile>/<version>/<platform>/<architecture>/
```

The allowed profiles are `customer-support` and `full-desk`. The allowed
platforms are `linux`, `macos`, and `windows`; macOS supports `x86_64` and
`arm64`, while the first Linux and Windows matrix is `x86_64`.

Each address must contain the archive and its manifest. A signature file may be
added for a signed release, but is not required. The manifest must agree with
the archive checksum and identify `rust`, `nuxt`, and `rust-webview`. A
versioned Nexus path is immutable and must never be replaced.

## Bridge sequence

The future product-specific Bridge executor must perform this sequence through
the existing Corevia Standard and `connectcore` boundaries:

1. Resolve the requested profile, version, platform, and architecture from
   source-controlled values.
2. Read the exact Nexus manifest and any optional signature before changing a target.
3. Verify the checksum, and verify the signature when one is present; then
   acquire the immutable artifact.
4. Install or update only the declared target and record the previous verified
   version as the rollback reference.
5. Run the declared local health checks and write a sanitized result.
6. On a failed acceptance check, restore the previous verified artifact and
   repeat the health checks.

No direct SSH, manual server edit, arbitrary download, or UI mutation is a
fallback for a missing executor. The correct result is `extension-needed`.

## Current boundary

Corevia Bridge already has generic governed artifact and Nexus operations. A
Corevia Desk-specific installer/health/rollback executor is not yet present in
the catalog, so CD-10 is contract-ready but its remote mutation remains
`extension-needed` until that executor is registered and verified.
