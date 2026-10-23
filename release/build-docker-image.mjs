import { chmodSync, cpSync, copyFileSync, existsSync, mkdirSync, rmSync, writeFileSync } from 'node:fs'
import { join } from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import { assertReleaseTarget, releaseMetadata } from './profile-contract.mjs'

const root = fileURLToPath(new URL('..', import.meta.url))
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
if (profile !== 'full-desk' || platform !== 'linux' || architecture !== 'x86_64') {
  throw new Error('The Docker image is only produced for the Linux x86_64 Full Desk target.')
}

const run = (command, args) => {
  const result = spawnSync(command, args, { cwd: root, stdio: 'inherit' })
  if (result.error) throw result.error
  if (result.status !== 0) throw new Error(`${command} failed with exit code ${result.status}.`)
}

const outputDir = join(root, 'release-output', profile, platform, architecture)
const bundleDir = join(outputDir, 'bundle')
const contextDir = join(outputDir, 'docker-context')
const dockerDir = join(root, 'docker', 'full-desk')
const binaryPath = join(bundleDir, 'rustdesk')
const webPath = join(bundleDir, 'web', 'public')
const baseImage = process.env.COREVIA_DOCKER_BASE_IMAGE
const requiredRegistry = process.env.COREVIA_DOCKER_BASE_REGISTRY

if (!baseImage) {
  throw new Error('COREVIA_DOCKER_BASE_IMAGE is required; the Corevia pipeline provides it.')
}

if (requiredRegistry && !baseImage.startsWith(`${requiredRegistry}/`)) {
  throw new Error('COREVIA_DOCKER_BASE_IMAGE must resolve through the registry named by COREVIA_DOCKER_BASE_REGISTRY.')
}

if (!existsSync(binaryPath) || !existsSync(webPath)) {
  throw new Error('The Linux Full Desk bundle is incomplete.')
}
rmSync(contextDir, { recursive: true, force: true })
mkdirSync(join(contextDir, 'web'), { recursive: true })
copyFileSync(binaryPath, join(contextDir, 'CoreviaDesk'))
chmodSync(join(contextDir, 'CoreviaDesk'), 0o755)
cpSync(webPath, join(contextDir, 'web', 'public'), { recursive: true })
copyFileSync(join(dockerDir, 'Dockerfile'), join(contextDir, 'Dockerfile'))
copyFileSync(join(dockerDir, 'entrypoint.sh'), join(contextDir, 'entrypoint.sh'))
chmodSync(join(contextDir, 'entrypoint.sh'), 0o755)

const image = process.env.COREVIA_DOCKER_IMAGE || `corevia-desk/full-desk-linux:${version}`
run('docker', [
  'build',
  '--tag', image,
  '--build-arg', `COREVIA_DOCKER_BASE_IMAGE=${baseImage}`,
  '--build-arg', `COREVIA_DESK_PORT=${process.env.COREVIA_DESK_PORT || '21116'}`,
  contextDir
])

const exportArchive = process.env.COREVIA_DOCKER_EXPORT_ARCHIVE !== 'false'
let compressedArchivePath
if (exportArchive) {
  const archivePath = join(outputDir, `corevia-desk-full-desk-linux-${version}.docker.tar`)
  run('docker', ['save', '--output', archivePath, image])
  run('gzip', ['--best', '--no-name', archivePath])
  compressedArchivePath = `${archivePath}.gz`
}
writeFileSync(join(outputDir, 'docker-image.json'), `${JSON.stringify({
  ...releaseMetadata({ profile, platform, architecture, version }),
  image,
  baseImage,
  archive: compressedArchivePath ? compressedArchivePath.split('/').pop() : null,
  publishMode: exportArchive ? 'archive' : 'local-registry-push',
  networkEnvironment: [
    'COREVIA_ID_SERVER',
    'COREVIA_RELAY_SERVER',
    'COREVIA_API_SERVER',
    'COREVIA_SERVER_KEY',
    'COREVIA_DESK_PORT'
  ]
}, null, 2)}\n`)
console.log(`Corevia Desk Full Desk Docker image prepared: ${image}`)
