import assert from 'node:assert/strict'
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { createServer } from 'node:http'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { spawn } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import test from 'node:test'

test('Docker Nexus verification follows a Bearer challenge and keeps the tag immutable', async () => {
  const directory = mkdtempSync(join(tmpdir(), 'corevia-desk-docker-verify-'))
  const outputDir = join(directory, 'release-output', 'full-desk', 'linux', 'x86_64')
  mkdirSync(outputDir, { recursive: true })
  const digest = `sha256:${'a'.repeat(64)}`
  let registry = ''
  const repository = 'corevia-desk/full-desk-linux'
  const version = '1.5.32'
  const targetImage = () => `${registry}/${repository}:${version}`
  const server = createServer((request, response) => {
    if (request.url?.startsWith('/service/token')) {
      const expected = `Basic ${Buffer.from('test-user:test-secret').toString('base64')}`
      if (request.headers.authorization !== expected) {
        response.writeHead(401).end()
        return
      }
      response.writeHead(200, { 'content-type': 'application/json' })
        .end(JSON.stringify({ token: 'test-token' }))
      return
    }

    if (request.url?.startsWith('/v2/')) {
      if (request.headers.authorization === 'Bearer test-token') {
        response.writeHead(200, {
          'content-type': 'application/vnd.oci.image.manifest.v1+json',
          'Docker-Content-Digest': digest
        }).end(JSON.stringify({ schemaVersion: 2, config: { digest } }))
        return
      }
      response.writeHead(401, {
        'WWW-Authenticate': `Bearer realm="http://${registry}/service/token",service="nexus"`
      }).end(JSON.stringify({ errors: [{ code: 'UNAUTHORIZED' }] }))
      return
    }

    response.writeHead(404).end()
  })

  await new Promise((resolve, reject) => {
    server.once('error', reject)
    server.listen(0, '127.0.0.1', resolve)
  })
  const address = server.address()
  assert.ok(address && typeof address === 'object')
  registry = `127.0.0.1:${address.port}`
  const target = targetImage()
  writeFileSync(join(outputDir, 'docker-image.json'), JSON.stringify({
    product: 'corevia-desk',
    profile: 'full-desk',
    platform: 'linux',
    architecture: 'x86_64',
    version,
    image: 'corevia-desk/full-desk-linux:1.5.32',
    baseImage: 'registry.example.test/debian:trixie-slim'
  }))
  writeFileSync(join(outputDir, 'docker-publish.json'), JSON.stringify({
    product: 'corevia-desk',
    profile: 'full-desk',
    platform: 'linux',
    architecture: 'x86_64',
    version,
    image: target,
    repository,
    digest: `${target}@${digest}`
  }))

  try {
    const root = fileURLToPath(new URL('../..', import.meta.url))
    const result = await new Promise((resolve, reject) => {
      const child = spawn(process.execPath, [join(root, 'release/verify-docker-image.mjs')], {
        cwd: root,
        env: {
        ...process.env,
        NO_PROXY: `127.0.0.1,localhost${process.env.NO_PROXY ? `,${process.env.NO_PROXY}` : ''}`,
        no_proxy: `127.0.0.1,localhost${process.env.no_proxy ? `,${process.env.no_proxy}` : ''}`,
        COREVIA_DOCKER_REGISTRY: registry,
        COREVIA_DOCKER_REPOSITORY: repository,
        COREVIA_RELEASE_OUTPUT_DIR: outputDir,
        CI_COMMIT_TAG: 'v1.5.32',
        NEXUS_USERNAME: 'test-user',
        NEXUS_PASSWORD: 'test-secret'
        }
      })
      let stdout = ''
      let stderr = ''
      child.stdout.setEncoding('utf8').on('data', (chunk) => { stdout += chunk })
      child.stderr.setEncoding('utf8').on('data', (chunk) => { stderr += chunk })
      child.once('error', reject)
      child.once('close', (status, signal) => resolve({ status, signal, stdout, stderr }))
    })
    assert.equal(result.status, 0, result.stderr)
    assert.match(result.stdout, /Verified Corevia Desk Docker image in Nexus/u)
    assert.doesNotMatch(result.stdout, /test-secret/u)
    assert.deepEqual(JSON.parse(readFileSync(join(outputDir, 'docker-verify.json'), 'utf8')), {
      product: 'corevia-desk',
      profile: 'full-desk',
      platform: 'linux',
      architecture: 'x86_64',
      version,
      image: target,
      digest,
      registry,
      repository,
      endpoint: `http://${registry}/v2/corevia-desk/full-desk-linux/manifests/${version}`,
      status: 200,
      readback: 'nexus-registry-manifest',
      secrets: 'runtime-only'
    })
  } finally {
    await new Promise((resolve) => server.close(resolve))
    rmSync(directory, { recursive: true, force: true })
  }
})
