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
const outputDir = process.env.COREVIA_RELEASE_OUTPUT_DIR
  ? resolve(process.env.COREVIA_RELEASE_OUTPUT_DIR)
  : join(root, 'release-output', 'full-desk', 'linux', 'x86_64')
const metadataPath = join(outputDir, 'docker-image.json')
const publishPath = join(outputDir, 'docker-publish.json')
if (!existsSync(metadataPath) || !existsSync(publishPath)) throw new Error('Docker publish metadata is incomplete.')
if (!/^[a-z0-9]+(?:[._/-][a-z0-9]+)*$/u.test(repository)) throw new Error('COREVIA_DOCKER_REPOSITORY is invalid.')
if (!/^[A-Za-z0-9][A-Za-z0-9._-]*$/u.test(version)) throw new Error('COREVIA_DESK_VERSION is invalid.')
if (/^(?:docker\.io|index\.docker\.io|ghcr\.io)(?::\d+)?$/iu.test(registry)) {
  throw new Error('Public registries are not allowed for the Corevia Desk image.')
}

const metadata = JSON.parse(readFileSync(metadataPath, 'utf8'))
const published = JSON.parse(readFileSync(publishPath, 'utf8'))
const targetImage = `${registry}/${repository}:${version}`
if (metadata.product !== 'corevia-desk'
  || metadata.profile !== 'full-desk'
  || metadata.platform !== 'linux'
  || metadata.architecture !== 'x86_64') {
  throw new Error('Docker metadata does not describe the Full Desk Linux image.')
}
if (published.image !== targetImage || published.repository !== repository || published.version !== version) {
  throw new Error('Docker publish metadata does not match the declared target.')
}
const publishedDigest = typeof published.digest === 'string' ? published.digest.split('@').pop() : ''
if (!/^sha256:[0-9a-f]{64}$/u.test(publishedDigest)) throw new Error('Docker publish digest is invalid.')

const registryBase = /^https?:\/\//u.test(registry) ? registry : `http://${registry}`
const repositoryPath = repository.split('/').map((part) => encodeURIComponent(part)).join('/')
const manifestUrl = `${registryBase}/v2/${repositoryPath}/manifests/${encodeURIComponent(version)}`
const authDirectory = mkdtempSync(join(root, '.work-docker-verify-'))
const netrcPath = join(authDirectory, 'netrc')
const headersPath = join(authDirectory, 'headers')
const bodyPath = join(authDirectory, 'manifest.json')
const tokenPath = join(authDirectory, 'token.json')
const registryHost = new URL(registryBase).hostname
const writeNetrc = (hosts) => {
  const entries = [...new Set(hosts.filter(Boolean))]
    .map((host) => `machine ${host}\nlogin ${username}\npassword ${password}`)
    .join('\n')
  writeFileSync(netrcPath, `${entries}\n`, { mode: 0o600 })
}
writeNetrc([registryHost])
chmodSync(netrcPath, 0o600)

const parseBearerChallenge = (headers) => {
  const challenge = headers.match(/^WWW-Authenticate:\s*Bearer\s+(.+)$/imu)?.[1]
  if (!challenge) return null
  const parameters = {}
  const matcher = /([A-Za-z][A-Za-z0-9_-]*)="([^"]*)"|([A-Za-z][A-Za-z0-9_-]*)=([^,\s]+)/gu
  let match
  while ((match = matcher.exec(challenge)) !== null) {
    parameters[match[1] || match[3]] = match[2] || match[4]
  }
  if (!parameters.realm) return null
  return parameters
}

const manifestRequest = (authorization) => {
  const args = [
    '--silent', '--show-error', '--connect-timeout', '10', '--max-time', '30',
    '--netrc-file', netrcPath,
    '--dump-header', headersPath,
    '--output', bodyPath,
    '--write-out', '%{http_code}',
    '--header', 'Accept: application/vnd.docker.distribution.manifest.v2+json, application/vnd.oci.image.manifest.v1+json, application/vnd.docker.distribution.manifest.list.v2+json, application/vnd.oci.image.index.v1+json'
  ]
  if (authorization) args.push('--header', `Authorization: ${authorization}`)
  args.push(manifestUrl)
  return spawnSync('curl', args, { encoding: 'utf8' })
}

try {
  let response = manifestRequest()
  if (response.error) throw response.error
  let status = Number.parseInt((response.stdout || '').trim(), 10)
  if (status === 401) {
    const challenge = parseBearerChallenge(readFileSync(headersPath, 'utf8'))
    if (!challenge) throw new Error('Nexus Docker registry returned 401 without a Bearer challenge.')
    const tokenUrl = new URL(challenge.realm)
    writeNetrc([registryHost, tokenUrl.hostname])
    tokenUrl.searchParams.set('service', challenge.service || new URL(registryBase).host)
    tokenUrl.searchParams.set('scope', challenge.scope || `repository:${repository}:pull`)
    const tokenResponse = spawnSync('curl', [
      '--silent', '--show-error', '--connect-timeout', '10', '--max-time', '30',
      '--netrc-file', netrcPath,
      '--output', tokenPath,
      '--write-out', '%{http_code}',
      tokenUrl.toString()
    ], { encoding: 'utf8' })
    if (tokenResponse.error) throw tokenResponse.error
    const tokenStatus = Number.parseInt((tokenResponse.stdout || '').trim(), 10)
    if (tokenStatus < 200 || tokenStatus >= 300) {
      throw new Error(`Nexus Docker token request failed with HTTP ${tokenStatus}.`)
    }
    const tokenPayload = JSON.parse(readFileSync(tokenPath, 'utf8'))
    const bearerToken = tokenPayload.token || tokenPayload.access_token
    if (typeof bearerToken !== 'string' || !bearerToken) {
      throw new Error('Nexus Docker token response did not contain a token.')
    }
    response = manifestRequest(`Bearer ${bearerToken}`)
    if (response.error) throw response.error
    status = Number.parseInt((response.stdout || '').trim(), 10)
  }
  if (status < 200 || status >= 300) throw new Error(`Nexus Docker manifest readback failed with HTTP ${status}.`)
  const headers = readFileSync(headersPath, 'utf8')
  const remoteDigest = headers.match(/^Docker-Content-Digest:\s*(sha256:[0-9a-f]{64})\s*$/imu)?.[1]
  if (remoteDigest !== publishedDigest) throw new Error('Nexus Docker manifest digest does not match the publish readback.')
  const body = JSON.parse(readFileSync(bodyPath, 'utf8'))
  if (!body || typeof body !== 'object') throw new Error('Nexus Docker manifest body is invalid.')
  writeFileSync(join(outputDir, 'docker-verify.json'), `${JSON.stringify({
    product: 'corevia-desk',
    profile: 'full-desk',
    platform: 'linux',
    architecture: 'x86_64',
    version,
    image: targetImage,
    digest: remoteDigest,
    registry,
    repository,
    endpoint: manifestUrl,
    status,
    readback: 'nexus-registry-manifest',
    secrets: 'runtime-only'
  }, null, 2)}\n`)
  console.log(`Verified Corevia Desk Docker image in Nexus: ${targetImage}@${remoteDigest}`)
} finally {
  rmSync(authDirectory, { recursive: true, force: true })
}
