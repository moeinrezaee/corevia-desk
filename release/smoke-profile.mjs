import { existsSync, readFileSync } from 'node:fs'
import { join } from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { assertReleaseTarget, macBundleIdentity } from './profile-contract.mjs'
import { releaseOutputDir } from './target-path.mjs'

const required = (name) => {
  const value = process.env[name]
  if (!value) throw new Error(`${name} is required.`)
  return value
}

const profile = required('COREVIA_DESK_PROFILE')
const platform = required('COREVIA_DESK_PLATFORM')
const architecture = required('COREVIA_DESK_ARCH')
const version = process.env.COREVIA_DESK_VERSION
  || (process.env.CI_COMMIT_TAG || '').replace(/^v/u, '')
  || '0.1.0-dev'
assertReleaseTarget({ profile, platform, architecture })

const root = fileURLToPath(new URL('..', import.meta.url))
const outputDir = releaseOutputDir({
  root,
  profile,
  platform,
  architecture,
  configuredOutputDir: process.env.COREVIA_RELEASE_OUTPUT_DIR
})
const bundleDir = join(outputDir, 'bundle')
const checks = platform === 'macos'
  ? [
      join(bundleDir, 'Corevia Desk.app', 'Contents', 'MacOS', 'CoreviaDesk'),
      join(bundleDir, 'Corevia Desk.app', 'Contents', 'Resources', 'CoreviaDesk.icns'),
      join(bundleDir, 'Corevia Desk.app', 'Contents', 'Resources', 'web', 'public', 'index.html'),
      join(bundleDir, 'Corevia Desk.app', 'Contents', 'Info.plist')
    ]
  : [
      join(bundleDir, platform === 'windows' ? 'Corevia Desk.exe' : 'rustdesk'),
      join(bundleDir, 'web', 'public', 'index.html'),
      join(bundleDir, 'profile.json')
    ]

for (const path of checks) {
  if (!existsSync(path)) throw new Error(`Release smoke check failed; required file is missing: ${path}`)
}

if (platform === 'macos') {
  const appBundle = join(bundleDir, 'Corevia Desk.app')
  const infoPlist = join(appBundle, 'Contents', 'Info.plist')
  const expectedIdentity = macBundleIdentity(profile)
  for (const [key, expected] of Object.entries({
    CFBundleIdentifier: expectedIdentity.bundleIdentifier,
    CFBundleDisplayName: expectedIdentity.displayName,
    CFBundleName: expectedIdentity.displayName
  })) {
    const actual = spawnSync('/usr/libexec/PlistBuddy', ['-c', `Print :${key}`, infoPlist], {
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe']
    })
    if (actual.error) throw actual.error
    if (actual.status !== 0 || actual.stdout.trim() !== expected) {
      throw new Error(`Release smoke check failed; ${key} does not match the ${profile} macOS bundle identity.`)
    }
  }

  const signatureCheck = spawnSync('codesign', ['--verify', '--deep', '--strict', appBundle], {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe']
  })
  if (signatureCheck.error) throw signatureCheck.error
  if (signatureCheck.status !== 0) {
    throw new Error(`Release smoke check failed; the macOS app signature is invalid: ${(signatureCheck.stderr || '').trim()}`)
  }

  const signatureDetails = spawnSync('codesign', ['-dv', '--verbose=2', appBundle], {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe']
  })
  if (signatureDetails.error) throw signatureDetails.error
  const signatureOutput = `${signatureDetails.stdout || ''}\n${signatureDetails.stderr || ''}`
  if (signatureDetails.status !== 0 || !signatureOutput.includes(`Identifier=${expectedIdentity.bundleIdentifier}`)) {
    throw new Error(`Release smoke check failed; the macOS code identity does not match ${expectedIdentity.bundleIdentifier}.`)
  }
}

const binary = platform === 'macos'
  ? join(bundleDir, 'Corevia Desk.app', 'Contents', 'MacOS', 'CoreviaDesk')
  : join(bundleDir, platform === 'windows' ? 'Corevia Desk.exe' : 'rustdesk')
const result = spawnSync(binary, ['--version'], {
  cwd: bundleDir,
  encoding: 'utf8',
  stdio: ['ignore', 'pipe', 'pipe']
})
if (result.error) throw result.error
if (result.status !== 0) {
  throw new Error(`Release smoke check failed; ${platform} executable exited with ${result.status}: ${(result.stderr || '').trim()}`)
}
const binaryVersion = (result.stdout || '').trim()
if (binaryVersion !== version) {
  throw new Error(`Release smoke check failed; binary version ${binaryVersion || '(empty)'} does not match bundle version ${version}.`)
}

const metadataPath = platform === 'macos'
  ? join(bundleDir, 'Corevia Desk.app', 'Contents', 'Resources', 'profile.json')
  : join(bundleDir, 'profile.json')
const metadata = JSON.parse(readFileSync(metadataPath, 'utf8'))
if (metadata.product !== 'corevia-desk'
  || metadata.profile !== profile
  || metadata.platform !== platform
  || metadata.architecture !== architecture
  || metadata.version !== version
  || metadata.core !== 'rust'
  || metadata.ui !== 'nuxt'
  || metadata.host !== 'rust-webview') {
  throw new Error('Release smoke check failed; bundle metadata is not the active Rust/Nuxt/WebView contract.')
}

console.log(`Release smoke PASS: ${profile}/${platform}/${architecture}/${version}`)
