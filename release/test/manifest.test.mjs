import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join, resolve } from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
import test from 'node:test'

test('release metadata contains profile, platform, architecture, version, and checksum', () => {
  const directory = mkdtempSync(join(tmpdir(), 'corevia-desk-release-'))
  try {
    const artifact = join(directory, 'corevia-desk-full-desk-1.0.0-linux-x86_64.tar.gz')
    const content = Buffer.from('corevia desk artifact')
    writeFileSync(artifact, content)
    const digest = createHash('sha256').update(content).digest('hex')
    const manifest = {
      schemaVersion: 1,
      product: 'corevia-desk',
      profile: 'full-desk',
      platform: 'linux',
      architecture: 'x86_64',
      version: '1.0.0',
      artifact: {
        name: 'corevia-desk-full-desk-1.0.0-linux-x86_64.tar.gz',
        size: content.length,
        sha256: digest,
        signature: 'corevia-desk-full-desk-1.0.0-linux-x86_64.tar.gz.sig'
      },
      build: {
        product: 'corevia-desk',
        profile: 'full-desk',
        platform: 'linux',
        architecture: 'x86_64',
        version: '1.0.0',
        core: 'rust',
        ui: 'nuxt',
        host: 'rust-webview'
      }
    }
    const manifestPath = join(directory, 'artifact.manifest.json')
    writeFileSync(manifestPath, `${JSON.stringify(manifest)}\n`)
    const loaded = JSON.parse(readFileSync(manifestPath, 'utf8'))
    assert.equal(loaded.product, 'corevia-desk')
    assert.equal(loaded.profile, 'full-desk')
    assert.equal(loaded.platform, 'linux')
    assert.equal(loaded.architecture, 'x86_64')
    assert.equal(loaded.version, '1.0.0')
    assert.equal(loaded.artifact.sha256, digest)
  } finally {
    rmSync(directory, { recursive: true, force: true })
  }
})

test('manifest and validation CLIs work without a signing secret', () => {
  const directory = mkdtempSync(join(tmpdir(), 'corevia-desk-release-cli-'))
  const root = fileURLToPath(new URL('../..', import.meta.url))
  try {
    const artifact = join(directory, 'corevia-desk-customer-support-1.0.0-linux-x86_64.tar.gz')
    const manifest = `${artifact}.manifest.json`
    writeFileSync(artifact, 'contract-test-artifact')
    const environment = {
      ...process.env,
      COREVIA_DESK_PROFILE: 'customer-support',
      COREVIA_DESK_PLATFORM: 'linux',
      COREVIA_DESK_ARCH: 'x86_64',
      COREVIA_DESK_VERSION: '1.0.0'
    }
    const create = spawnSync(process.execPath, [
      join(root, 'release/create-manifest.mjs'),
      '--artifact', artifact
    ], { cwd: root, env: environment, encoding: 'utf8' })
    assert.equal(create.status, 0, create.stderr)
    const validate = spawnSync(process.execPath, [
      join(root, 'release/validate-manifest.mjs'),
      '--manifest', manifest
    ], { cwd: root, env: environment, encoding: 'utf8' })
    assert.equal(validate.status, 0, validate.stderr)
    const loaded = JSON.parse(readFileSync(manifest, 'utf8'))
    assert.equal(loaded.artifact.name, 'corevia-desk-customer-support-1.0.0-linux-x86_64.tar.gz')
    assert.equal(loaded.artifact.signature, undefined)
    assert.equal(loaded.build.host, 'rust-webview')
  } finally {
    rmSync(directory, { recursive: true, force: true })
  }
})
