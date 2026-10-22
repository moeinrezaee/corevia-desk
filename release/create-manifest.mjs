import { createHash } from 'node:crypto'
import { readdirSync, readFileSync, statSync, writeFileSync } from 'node:fs'
import { basename, dirname, join, resolve } from 'node:path'
import { assertReleaseTarget, releaseMetadata } from './profile-contract.mjs'
import { fileURLToPath } from 'node:url'

const argument = (name) => {
  const index = process.argv.indexOf(name)
  return index >= 0 ? process.argv[index + 1] : undefined
}

const profile = argument('--profile') || process.env.COREVIA_DESK_PROFILE
const platform = argument('--platform') || process.env.COREVIA_DESK_PLATFORM
const arch = argument('--arch') || process.env.COREVIA_DESK_ARCH
const version = argument('--version')
  || process.env.COREVIA_DESK_VERSION
  || (process.env.CI_COMMIT_TAG || '').replace(/^v/u, '')
  || '0.1.0-dev'
const explicitArtifact = argument('--artifact') || process.env.COREVIA_DESK_ARTIFACT
const outputDir = join(fileURLToPath(new URL('..', import.meta.url)), 'release-output', profile || '', platform || '', arch || '')
const discoveredArtifact = explicitArtifact
  ? undefined
  : readdirSync(outputDir, { withFileTypes: true }).find((entry) => entry.isFile() && entry.name.endsWith('.tar.gz'))
const artifactInput = explicitArtifact || (discoveredArtifact && join(outputDir, discoveredArtifact.name))
if (!artifactInput) throw new Error('--artifact is required when no archive exists in the profile output directory.')
const artifactPath = resolve(artifactInput)
const outputPath = resolve(argument('--output') || `${artifactPath}.manifest.json`)

assertReleaseTarget({ profile, platform, architecture: arch })
if (!/^[A-Za-z0-9][A-Za-z0-9._-]*$/u.test(version)) throw new Error('Invalid release version.')

const bytes = readFileSync(artifactPath)
const manifest = {
  schemaVersion: 1,
  product: 'corevia-desk',
  profile,
  platform,
  architecture: arch,
  version,
  artifact: {
    name: basename(artifactPath),
    size: statSync(artifactPath).size,
    sha256: createHash('sha256').update(bytes).digest('hex')
  },
  source: {
    commit: process.env.CI_COMMIT_SHA || 'local',
    pipeline: process.env.CI_PIPELINE_ID || 'local'
  },
  build: releaseMetadata({ profile, platform, architecture: arch, version })
}

writeFileSync(outputPath, `${JSON.stringify(manifest, null, 2)}\n`)
console.log(`Release manifest prepared: ${join(dirname(outputPath), basename(outputPath))}`)
