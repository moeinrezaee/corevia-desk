import { mkdtempSync, readFileSync, rmSync } from 'node:fs'
import { tmpdir } from 'node:os'
import { join, resolve } from 'node:path'
import { spawnSync } from 'node:child_process'
import { createHash } from 'node:crypto'

const root = resolve(import.meta.dirname, '../..')
const lock = JSON.parse(readFileSync(join(root, 'web/package-lock.json'), 'utf8'))
const repair = JSON.parse(readFileSync(join(import.meta.dirname, 'nexus-npm-repair.json'), 'utf8'))
const registry = process.env.NPM_CONFIG_REGISTRY
const username = process.env.NEXUS_USERNAME
const password = process.env.NEXUS_PASSWORD

if (!registry || !process.env.NPM_CONFIG_USERCONFIG || !username || !password) {
  throw new Error('NPM_CONFIG_REGISTRY, NPM_CONFIG_USERCONFIG, and Nexus credentials are required.')
}

function npm(args, options = {}) {
  return spawnSync('npm', args, { encoding: 'utf8', stdio: options.capture ? 'pipe' : 'inherit', ...options })
}

const candidates = new Map()
for (const locked of Object.values(lock.packages || {})) {
  if (!locked?.version || !locked?.integrity || !locked?.resolved || locked.inBundle) continue
  const source = new URL(locked.resolved)
  if (source.hostname !== 'registry.npmjs.org') continue
  const pathParts = source.pathname.split('/-/')[0].split('/').filter(Boolean)
  const name = pathParts[0]?.startsWith('@') ? `${pathParts[0]}/${pathParts[1]}` : pathParts[0]
  if (!name) continue
  candidates.set(`${name}@${locked.version}`, { spec: `${name}@${locked.version}`, locked, source })
}

const auth = `Basic ${Buffer.from(`${username}:${password}`, 'utf8').toString('base64')}`
const registryBase = registry.endsWith('/') ? registry : `${registry}/`
const pending = [...candidates.values()]
const missing = []
const workers = Array.from({ length: Math.min(24, pending.length) }, async () => {
  while (pending.length) {
    const candidate = pending.pop()
    const target = new URL(candidate.source.pathname.replace(/^\//, ''), registryBase)
    const response = await fetch(target, {
      method: 'HEAD',
      headers: { authorization: auth },
      signal: AbortSignal.timeout(30_000),
    })
    if (response.status === 404) missing.push(candidate)
    else if (!response.ok) throw new Error(`Nexus readback failed for ${candidate.spec}: HTTP ${response.status}`)
  }
})
await Promise.all(workers)
console.log(`Nexus npm readback complete: ${candidates.size - missing.length} present, ${missing.length} missing.`)

for (const spec of repair.packages) {
  const candidate = candidates.get(spec)
  if (!candidate) throw new Error(`${spec} is not pinned with integrity in web/package-lock.json.`)
  const target = new URL(candidate.source.pathname.replace(/^\//, ''), registryBase)
  const response = await fetch(target, { headers: { authorization: auth }, signal: AbortSignal.timeout(60_000) })
  if (response.status === 404) {
    if (!missing.includes(candidate)) missing.push(candidate)
    continue
  }
  if (!response.ok) throw new Error(`Nexus integrity readback failed for ${spec}: HTTP ${response.status}`)
  const actual = `sha512-${createHash('sha512').update(Buffer.from(await response.arrayBuffer())).digest('base64')}`
  if (actual === candidate.locked.integrity) continue

  const unpublished = npm(['unpublish', spec, '--force', '--registry', registry])
  if (unpublished.status !== 0) throw new Error(`Nexus repair removal failed for ${spec}.`)
  if (!missing.includes(candidate)) missing.push(candidate)
  console.log(`Removed corrupt Nexus npm component: ${spec}`)
}

for (const { spec, locked } of missing.sort((a, b) => a.spec.localeCompare(b.spec))) {

  const workspace = mkdtempSync(join(tmpdir(), 'corevia-npm-seed-'))
  try {
    const packed = npm(
      ['pack', spec, '--registry', 'https://registry.npmjs.org/', '--json', '--pack-destination', workspace],
      { capture: true },
    )
    if (packed.status !== 0) throw new Error(`Unable to acquire ${spec} from the declared public source.`)
    const [artifact] = JSON.parse(packed.stdout)
    if (artifact.integrity !== locked.integrity) {
      throw new Error(`Integrity mismatch for ${spec}.`)
    }
    const published = npm([
      'publish',
      join(workspace, artifact.filename),
      '--registry',
      registry,
      '--provenance=false',
    ])
    if (published.status !== 0) throw new Error(`Unable to publish ${spec} to Nexus.`)
    console.log(`Nexus npm package synchronized: ${spec}`)
  } finally {
    rmSync(workspace, { recursive: true, force: true })
  }
}
