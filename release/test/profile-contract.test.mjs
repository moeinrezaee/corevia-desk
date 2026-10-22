import assert from 'node:assert/strict'
import test from 'node:test'
import { join, resolve, sep } from 'node:path'
import {
  RELEASE_PLATFORMS,
  RELEASE_PROFILES,
  UI_RUNTIME,
  assertReleaseTarget,
  isReleaseArchitecture,
  macBundleIdentity
} from '../profile-contract.mjs'
import { releaseOutputDir, releaseTargetRoot } from '../target-path.mjs'

const workspaceRoot = resolve(sep, 'workspace', 'corevia-desk')

test('both desktop profiles have separate, explicit capability contracts', () => {
  assert.deepEqual(Object.keys(RELEASE_PROFILES).sort(), ['customer-support', 'full-desk'])
  assert.ok(RELEASE_PROFILES['customer-support'].capabilities.includes('approve-connection'))
  assert.ok(RELEASE_PROFILES['customer-support'].forbiddenCapabilities.includes('use-file-transfer'))
  assert.ok(RELEASE_PROFILES['customer-support'].forbiddenCapabilities.includes('start-connection'))
  assert.ok(RELEASE_PROFILES['full-desk'].capabilities.includes('use-file-transfer'))
  assert.ok(RELEASE_PROFILES['full-desk'].forbiddenCapabilities.includes('system-command'))
})

test('desktop target matrix is explicit and keeps macOS arm64 available', () => {
  assert.deepEqual(Object.keys(RELEASE_PLATFORMS).sort(), ['linux', 'macos', 'windows'])
  assert.equal(isReleaseArchitecture('macos', 'arm64'), true)
  assert.equal(isReleaseArchitecture('windows', 'arm64'), false)
  assert.throws(() => assertReleaseTarget({
    profile: 'customer-support',
    platform: 'windows',
    architecture: 'arm64'
  }))
})

test('the active runtime is Rust plus Nuxt inside a native WebView', () => {
  assert.deepEqual(UI_RUNTIME, {
    core: 'rust',
    ui: 'nuxt',
    host: 'rust-webview',
    bridge: 'window.__COREVIA_DESK_BRIDGE__',
    mobile: 'deferred'
  })
})

test('native release target roots are isolated by profile and target', () => {
  const common = { root: workspaceRoot, platform: 'macos', architecture: 'arm64' }
  const fullDesk = releaseTargetRoot({ ...common, profile: 'full-desk' })
  const customerSupport = releaseTargetRoot({ ...common, profile: 'customer-support' })
  assert.notEqual(fullDesk, customerSupport)
  assert.equal(
    customerSupport,
    join(workspaceRoot, 'target', 'corevia-profiles', 'macos', 'arm64', 'customer-support')
  )
})

test('macOS profiles are side-by-side safe', () => {
  assert.deepEqual(macBundleIdentity('full-desk'), {
    bundleIdentifier: 'com.corevia.desk',
    displayName: 'Corevia Desk'
  })
  assert.deepEqual(macBundleIdentity('customer-support'), {
    bundleIdentifier: 'com.corevia.desk.customer-support',
    displayName: 'Corevia Desk - CustomerSupport'
  })
  assert.notEqual(
    macBundleIdentity('full-desk').bundleIdentifier,
    macBundleIdentity('customer-support').bundleIdentifier
  )
})

test('an explicitly configured Cargo target root is honored', () => {
  assert.equal(
    releaseTargetRoot({
      root: workspaceRoot,
      profile: 'full-desk',
      platform: 'linux',
      architecture: 'x86_64',
      configuredTargetDir: './custom-target'
    }),
    resolve(workspaceRoot, 'custom-target')
  )
})

test('isolated release output is allowed only under repository .work', () => {
  const common = {
    root: workspaceRoot,
    profile: 'full-desk',
    platform: 'macos',
    architecture: 'arm64'
  }

  assert.equal(
    releaseOutputDir({ ...common, configuredOutputDir: '.work/test-artifacts' }),
    join(workspaceRoot, '.work', 'test-artifacts', 'full-desk', 'macos', 'arm64')
  )
  assert.equal(
    releaseOutputDir(common),
    join(workspaceRoot, 'release-output', 'full-desk', 'macos', 'arm64')
  )
  assert.throws(
    () => releaseOutputDir({ ...common, configuredOutputDir: 'release-output' }),
    /dedicated subdirectory of the repository \.work/u
  )
  assert.throws(
    () => releaseOutputDir({ ...common, configuredOutputDir: '../outside' }),
    /dedicated subdirectory of the repository \.work/u
  )
})
