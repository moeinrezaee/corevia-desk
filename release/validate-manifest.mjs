import { createHash } from 'node:crypto'
import { existsSync, readdirSync, readFileSync } from 'node:fs'
import { dirname, join, resolve } from 'node:path'
import { assertReleaseTarget, UI_RUNTIME } from './profile-contract.mjs'
import { fileURLToPath } from 'node:url'

const argument = (name) => {
  const index = process.argv.indexOf(name)
  return index >= 0 ? process.argv[index + 1] : undefined
}

const profile = process.env.COREVIA_DESK_PROFILE
const platform = process.env.COREVIA_DESK_PLATFORM
const arch = process.env.COREVIA_DESK_ARCH
const outputDir = join(fileURLToPath(new URL('..', import.meta.url)), 'release-output', profile || '', platform || '', arch || '')
const explicitManifest = argument('--manifest') || process.env.COREVIA_DESK_MANIFEST
const discoveredManifest = explicitManifest
  ? undefined
  : readdirSync(outputDir, { withFileTypes: true }).find((entry) => entry.isFile() && entry.name.endsWith('.manifest.json'))
const manifestInput = explicitManifest || (discoveredManifest && join(outputDir, discoveredManifest.name))
if (!manifestInput) throw new Error('--manifest is required when no manifest exists in the profile output directory.')
const manifestPath = resolve(manifestInput)
const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'))

for (const field of ['product', 'profile', 'platform', 'architecture', 'version']) {
  if (typeof manifest[field] !== 'string' || !manifest[field]) throw new Error(`Manifest field is missing: ${field}`)
}
if (manifest.product !== 'corevia-desk') throw new Error('Manifest belongs to another product.')
assertReleaseTarget({
  profile: manifest.profile,
  platform: manifest.platform,
  architecture: manifest.architecture
})
if (!/^[A-Za-z0-9][A-Za-z0-9._-]*$/u.test(manifest.version)) throw new Error('Manifest version is invalid.')
if (!manifest.artifact || typeof manifest.artifact.name !== 'string' || !manifest.artifact.sha256) {
  throw new Error('Manifest artifact metadata is incomplete.')
}
if (!manifest.build || manifest.build.core !== UI_RUNTIME.core || manifest.build.ui !== UI_RUNTIME.ui || manifest.build.host !== UI_RUNTIME.host) {
  throw new Error('Manifest runtime must identify Rust, Nuxt, and the native WebView host.')
}

const artifactPath = resolve(dirname(manifestPath), manifest.artifact.name)
if (!existsSync(artifactPath)) throw new Error(`Artifact is missing: ${artifactPath}`)
const digest = createHash('sha256').update(readFileSync(artifactPath)).digest('hex')
if (digest !== manifest.artifact.sha256) throw new Error('Artifact checksum does not match the manifest.')

if (process.argv.includes('--require-signature')) {
  if (typeof manifest.artifact.signature !== 'string' || !manifest.artifact.signature) {
    throw new Error('Artifact signature is required by this validation mode.')
  }
  const signaturePath = resolve(dirname(manifestPath), manifest.artifact.signature)
  if (!existsSync(signaturePath)) throw new Error(`Artifact signature is missing: ${signaturePath}`)
}

console.log(`Release manifest PASS: ${manifest.profile}/${manifest.platform}/${manifest.architecture}/${manifest.version}`)
