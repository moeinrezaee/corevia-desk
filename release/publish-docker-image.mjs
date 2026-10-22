import { chmodSync, existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { join, resolve } from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const required = (name) => {
  const value = process.env[name]
  if (!value) throw new Error(`${name} is required.`)
  return value
}

const registry = required('COREVIA_DOCKER_REGISTRY').replace(/\/$/u, '')
const repository = required('COREVIA_DOCKER_REPOSITORY')
const username = required('NEXUS_USERNAME')
const password = required('NEXUS_PASSWORD')
const version = process.env.COREVIA_DESK_VERSION
  || (process.env.CI_COMMIT_TAG || '').replace(/^v/u, '')
  || '0.1.0-dev'
const root = fileURLToPath(new URL('..', import.meta.url))
const outputDir = join(root, 'release-output', 'full-desk', 'linux', 'x86_64')
const metadataPath = join(outputDir, 'docker-image.json')
if (!existsSync(metadataPath)) throw new Error(`Docker image metadata is missing: ${metadataPath}`)
if (!/^[a-z0-9]+(?:[._/-][a-z0-9]+)*$/u.test(repository)) throw new Error('COREVIA_DOCKER_REPOSITORY is invalid.')
if (!/^[A-Za-z0-9][A-Za-z0-9._-]*$/u.test(version)) throw new Error('COREVIA_DESK_VERSION is invalid.')
if (/^(?:docker\.io|index\.docker\.io|ghcr\.io)(?::\d+)?$/iu.test(registry)) {
  throw new Error('Public registries are not allowed for the Corevia Desk image.')
}

const metadata = JSON.parse(readFileSync(metadataPath, 'utf8'))
if (metadata.product !== 'corevia-desk'
  || metadata.profile !== 'full-desk'
  || metadata.platform !== 'linux'
  || metadata.architecture !== 'x86_64') {
  throw new Error('Docker metadata does not describe the Full Desk Linux image.')
}
if (typeof metadata.baseImage !== 'string'
  || !/^nexus\.getcorevia\.ir(?::\d+)?\//u.test(metadata.baseImage)) {
  throw new Error('Docker metadata must identify an internal Nexus base image.')
}
const archivePath = join(outputDir, metadata.archive)
if (!existsSync(archivePath)) throw new Error(`Docker image archive is missing: ${archivePath}`)

const targetImage = `${registry}/${repository}:${version}`
const dockerConfigDir = mkdtempSync(join(root, '.work-docker-config-'))
chmodSync(dockerConfigDir, 0o700)

const run = (args, options = {}) => {
  const result = spawnSync('docker', args, {
    cwd: root,
    env: { ...process.env, DOCKER_CONFIG: dockerConfigDir },
    stdio: options.input === undefined ? 'inherit' : ['pipe', 'inherit', 'inherit'],
    input: options.input
  })
  if (result.error) throw result.error
  if (result.status !== 0) throw new Error(`docker ${args[0]} failed with exit code ${result.status}.`)
  return result
}

try {
  run(['load', '--input', archivePath])
  run(['login', registry, '--username', username, '--password-stdin'], { input: `${password}\n` })
  run(['tag', metadata.image, targetImage])
  run(['push', targetImage])
  const digest = spawnSync('docker', ['image', 'inspect', '--format', '{{index .RepoDigests 0}}', targetImage], {
    cwd: root,
    env: { ...process.env, DOCKER_CONFIG: dockerConfigDir },
    encoding: 'utf8'
  })
  if (digest.error) throw digest.error
  if (digest.status !== 0 || !(digest.stdout || '').trim()) throw new Error('Docker image digest readback failed.')
  writeFileSync(join(outputDir, 'docker-publish.json'), `${JSON.stringify({
    product: 'corevia-desk',
    profile: 'full-desk',
    platform: 'linux',
    architecture: 'x86_64',
    version,
    image: targetImage,
    digest: digest.stdout.trim(),
    registry,
    repository,
    source: 'release-output/full-desk/linux/x86_64/docker-image.json',
    secrets: 'runtime-only'
  }, null, 2)}\n`)
  console.log(`Published Corevia Desk Docker image: ${targetImage}`)
} finally {
  const logout = spawnSync('docker', ['logout', registry], {
    cwd: root,
    env: { ...process.env, DOCKER_CONFIG: dockerConfigDir },
    stdio: 'ignore'
  })
  if (logout.error || logout.status !== 0) {
    // Logout is cleanup only; the temporary config directory is removed below.
  }
  rmSync(dockerConfigDir, { recursive: true, force: true })
}
