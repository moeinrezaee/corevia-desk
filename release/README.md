# Corevia Desk release tools

The release path creates a native Rust plus static Nuxt bundle for each profile,
platform, and architecture. On macOS it wraps the executable and the view in a
real `Corevia Desk.app`; it then creates a versioned archive, a manifest, and an
independent checksum. Signing is optional and is not required by CI. The Nexus
publisher performs a preflight check and refuses to overwrite an existing
versioned path.

Required CI secrets are supplied by the CI secret store only:

- `NEXUS_URL`, `NEXUS_REPOSITORY`, `NEXUS_USER`, and `NEXUS_PASSWORD` for
  publication.

No secret belongs in source, manifests, logs, or the repository metadata. A
real cross-platform release needs native Windows, macOS, and Linux runners.
The local macOS arm64 path is buildable; Windows and optional Linux builds are
validated only when their native runners are available.

The Windows runner is a persistent build machine. Its one-time setup must
provide Rust/MSVC, Node.js, vcpkg with the `x64-windows-static` multimedia
libraries, and Windows SDK headers. The source-controlled
`scripts/ci/verify-windows-native-environment.ps1` gate checks those tools and
exports `VCPKG_ROOT`, `VCPKG_INSTALLED_ROOT`, and the bindgen include paths on
every Windows job. A fresh or rebuilt runner must be prepared again; an
ordinary Windows target machine only receives the packaged application.

For a local macOS build, set `COREVIA_SKIP_NPM_INSTALL=1` only when the existing
`web/node_modules` tree is trusted and already installed. CI never sets this
switch and always performs a clean install.

For a non-release validation build that must preserve an existing bundle, set
`COREVIA_RELEASE_OUTPUT_DIR` to a dedicated path beneath repository `.work`
(for example `.work/native-smoke`). The build rejects paths outside `.work`;
without this option, the normal `release-output` location is unchanged.

The macOS bundle is signed after its `Info.plist` is written, using the bundle
identifier as its code identity so privacy permissions attach to the correct
profile. Local builds default to an ad-hoc signature; macOS may ask for screen
permission again after rebuilding. Set `COREVIA_MACOS_SIGNING_IDENTITY` to a
stable signing certificate to preserve identity across builds. Public macOS
distribution still requires a Developer ID certificate and notarization.

The release pipeline also creates a Full Desk Linux Docker image on a Linux
runner with Docker available. It publishes the image archive and metadata as
the Linux build artifact and publishes the image to the private Nexus Docker
registry; Customer Support is intentionally excluded from this container output.
At runtime, set `COREVIA_ID_SERVER`, `COREVIA_RELAY_SERVER`,
optional `COREVIA_API_SERVER`, `COREVIA_SERVER_KEY`, and `COREVIA_DESK_PORT`.
The last value controls the direct-access port and must be mapped with Docker's
`-p` option. No network secret is stored in the image or repository.
