import { existsSync, readdirSync } from 'node:fs'
import { dirname, join, resolve } from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const argument = (name) => {
  const index = process.argv.indexOf(name)
  return index >= 0 ? process.argv[index + 1] : undefined
}

const profile = process.env.COREVIA_DESK_PROFILE
const platform = process.env.COREVIA_DESK_PLATFORM
const arch = process.env.COREVIA_DESK_ARCH
const outputDir = join(fileURLToPath(new URL('..', import.meta.url)), 'release-output', profile || '', platform || '', arch || '')
const explicitArtifact = argument('--artifact') || process.env.COREVIA_DESK_ARTIFACT
const discoveredArtifact = explicitArtifact
  ? undefined
  : readdirSync(outputDir, { withFileTypes: true }).find((entry) => entry.isFile() && entry.name.endsWith('.tar.gz'))
const artifactInput = explicitArtifact || (discoveredArtifact && join(outputDir, discoveredArtifact.name))
if (!artifactInput) throw new Error('--artifact is required when no archive exists in the profile output directory.')
const artifactPath = resolve(artifactInput)
if (!existsSync(artifactPath)) {
  throw new Error('--artifact must point to an existing file.')
}
const key = process.env.COSIGN_KEY
const publicKey = process.env.COSIGN_PUBLIC_KEY
if (!key || !publicKey) {
  throw new Error('COSIGN_KEY and COSIGN_PUBLIC_KEY are required; unsigned release artifacts are not allowed.')
}

const signaturePath = `${artifactPath}.sig`
const run = (args) => {
  const result = spawnSync('cosign', args, { stdio: 'inherit' })
  if (result.error) throw result.error
  if (result.status !== 0) throw new Error(`cosign failed with exit code ${result.status}.`)
}

run(['sign-blob', '--yes', '--key', key, '--output-signature', signaturePath, artifactPath])
run(['verify-blob', '--key', publicKey, '--signature', signaturePath, artifactPath])
console.log(`Artifact signature verified: ${dirname(signaturePath)}`)
