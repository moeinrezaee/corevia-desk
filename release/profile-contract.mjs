export const RELEASE_PROFILES = Object.freeze({
  'customer-support': Object.freeze({
    id: 'customer-support',
    capabilities: Object.freeze([
      'read-settings',
      'read-identity',
      'read-local-secret',
      'read-devices',
      'read-sessions',
      'write-customer-setting',
      'approve-connection',
      'close-session',
      'change-connection-permission'
    ]),
    forbiddenCapabilities: Object.freeze([
      'start-connection',
      'input-control',
      'use-file-transfer',
      'use-camera',
      'use-terminal',
      'use-port-forward',
      'use-printer',
      'record-session',
      'restart-remote-device',
      'elevate-session',
      'toggle-privacy-mode',
      'manage-service',
      'manage-peers',
      'send-chat',
      'use-voice-call',
      'write-global-setting',
      'system-command'
    ]),
    installer: 'customer-support-only'
  }),
  'full-desk': Object.freeze({
    id: 'full-desk',
    capabilities: Object.freeze([
      'read-settings',
      'read-identity',
      'read-local-secret',
      'read-devices',
      'read-sessions',
      'write-customer-setting',
      'write-global-setting',
      'start-connection',
      'approve-connection',
      'change-connection-permission',
      'input-control',
      'use-camera',
      'use-file-transfer',
      'use-terminal',
      'use-port-forward',
      'use-printer',
      'record-session',
      'restart-remote-device',
      'elevate-session',
      'toggle-privacy-mode',
      'manage-service',
      'manage-peers',
      'send-chat',
      'use-voice-call'
    ]),
    forbiddenCapabilities: Object.freeze(['system-command']),
    installer: 'independent-nexus-install'
  })
})

export const RELEASE_PLATFORMS = Object.freeze({
  linux: Object.freeze(['x86_64']),
  macos: Object.freeze(['x86_64', 'arm64']),
  windows: Object.freeze(['x86_64'])
})

export const UI_RUNTIME = Object.freeze({
  core: 'rust',
  ui: 'nuxt',
  host: 'rust-webview',
  bridge: 'window.__COREVIA_DESK_BRIDGE__',
  mobile: 'deferred'
})

export const isReleaseProfile = (value) => {
  return typeof value === 'string' && Object.hasOwn(RELEASE_PROFILES, value)
}

export const isReleasePlatform = (value) => {
  return typeof value === 'string' && Object.hasOwn(RELEASE_PLATFORMS, value)
}

export const isReleaseArchitecture = (platform, architecture) => {
  return isReleasePlatform(platform) && RELEASE_PLATFORMS[platform].includes(architecture)
}

export const profileContract = (profile) => {
  if (!isReleaseProfile(profile)) return undefined
  return RELEASE_PROFILES[profile]
}

export const macBundleIdentity = (profile) => {
  if (!isReleaseProfile(profile)) return undefined
  if (profile === 'customer-support') {
    return Object.freeze({
      bundleIdentifier: 'com.corevia.desk.customer-support',
      displayName: 'Corevia Desk - CustomerSupport'
    })
  }
  return Object.freeze({
    bundleIdentifier: 'com.corevia.desk',
    displayName: 'Corevia Desk'
  })
}

export const assertReleaseTarget = ({ profile, platform, architecture }) => {
  if (!isReleaseProfile(profile)) throw new Error('Release profile must be customer-support or full-desk.')
  if (!isReleasePlatform(platform)) throw new Error('Release platform must be linux, macos, or windows.')
  if (!isReleaseArchitecture(platform, architecture)) {
    throw new Error(`Architecture ${architecture} is not enabled for ${platform}.`)
  }
  return RELEASE_PROFILES[profile]
}

export const releaseMetadata = ({ profile, platform, architecture, version }) => {
  const contract = assertReleaseTarget({ profile, platform, architecture })
  return {
    product: 'corevia-desk',
    profile: contract.id,
    platform,
    architecture,
    version,
    ...UI_RUNTIME,
    capabilities: [...contract.capabilities],
    forbiddenCapabilities: [...contract.forbiddenCapabilities],
    installer: contract.installer
  }
}
