import { chmodSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { dirname, join, resolve } from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const manifestPath = resolve(process.argv[2] || '')
if (!manifestPath || manifestPath === resolve('/')) throw new Error('A manifest path is required.')
const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'))
const nexusUrl = process.env.COREVIA_NEXUS_RAW_URL || process.env.NEXUS_URL
const repository = process.env.NEXUS_REPOSITORY
// Keep the raw-release publisher aligned with the canonical GitLab secret
// name used by the Docker publisher and the release-job guards.  Accept the
// historical NEXUS_USER alias only as a compatibility fallback.
const username = process.env.NEXUS_USERNAME || process.env.NEXUS_USER
const password = process.env.NEXUS_PASSWORD
if (!nexusUrl || !repository || !username || !password) {
  throw new Error('NEXUS_URL, NEXUS_REPOSITORY, NEXUS_USERNAME, and NEXUS_PASSWORD are required.')
}
if (manifest.product !== 'corevia-desk'
  || !['customer-support', 'full-desk'].includes(manifest.profile)
  || !['windows', 'macos', 'linux'].includes(manifest.platform)
  || !['x86_64', 'arm64'].includes(manifest.architecture)
  || !/^[A-Za-z0-9][A-Za-z0-9._-]*$/u.test(manifest.version)) {
  throw new Error('Manifest identity is invalid.')
}

const base = `${nexusUrl.replace(/\/$/u, '')}/repository/${encodeURIComponent(repository)}/corevia-desk/${manifest.profile}/${manifest.version}/${manifest.platform}/${manifest.architecture}`
const directory = dirname(manifestPath)
const nexusHost = new URL(nexusUrl).host
const files = [
  manifest.artifact.name,
  `${manifest.artifact.name}.manifest.json`
]
if (typeof manifest.artifact.signature === 'string' && manifest.artifact.signature) {
  files.splice(1, 0, manifest.artifact.signature)
}

if (files.some((file) => typeof file !== 'string' || file !== file.split(/[\\/]/u).pop())) {
  throw new Error('Manifest file names must stay inside the artifact directory.')
}

const run = (args, allowFailure = false) => {
  const result = spawnSync('curl', args, { stdio: 'inherit' })
  if (result.error) throw result.error
  if (result.status !== 0 && !allowFailure) throw new Error(`Nexus transfer failed with exit code ${result.status}.`)
  return result.status
}

let authDirectory
let netrcPath
try {
  authDirectory = mkdtempSync(join(fileURLToPath(new URL('..', import.meta.url)), '.work-nexus-auth-'))
  netrcPath = join(authDirectory, 'netrc')
  writeFileSync(netrcPath, `machine ${nexusHost}\nlogin ${username}\npassword ${password}\n`, { mode: 0o600 })
  chmodSync(netrcPath, 0o600)

  for (const file of files) {
    const path = resolve(directory, file)
    const url = `${base}/${encodeURIComponent(file)}`
    // A versioned path is immutable. Existing content is an error, never an overwrite.
    const probe = spawnSync('curl', [
      '--silent', '--show-error', '--output', '/dev/null', '--write-out', '%{http_code}',
      '--head', '--netrc-file', netrcPath, url
    ], { encoding: 'utf8' })
    if (probe.error) throw probe.error
    const code = Number.parseInt((probe.stdout || '').trim(), 10)
    if (code >= 200 && code < 400) throw new Error(`Nexus path already exists and is immutable: ${url}`)
    if (code !== 404) {
      const detail = (probe.stderr || '').trim()
      const observed = Number.isNaN(code) ? 'unavailable' : code
      throw new Error(`Nexus preflight failed for ${url} (HTTP ${observed})${detail ? `: ${detail}` : ''}.`)
    }

    run([
      '--fail', '--silent', '--show-error', '--retry', '3',
      '--netrc-file', netrcPath,
      '--upload-file', path,
      url
    ])
  }
} finally {
  if (authDirectory) rmSync(authDirectory, { recursive: true, force: true })
}

console.log(`Published Corevia Desk ${manifest.profile}/${manifest.platform}/${manifest.architecture}/${manifest.version} to Nexus.`)
