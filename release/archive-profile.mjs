import { existsSync, mkdirSync } from 'node:fs'
import { basename, dirname, join, resolve } from 'node:path'
import { spawnSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'

const required = (name) => {
  const value = process.env[name]
  if (!value) throw new Error(`${name} is required.`)
  return value
}

const profile = required('COREVIA_DESK_PROFILE')
const platform = required('COREVIA_DESK_PLATFORM')
const arch = required('COREVIA_DESK_ARCH')
const version = process.env.COREVIA_DESK_VERSION
  || (process.env.CI_COMMIT_TAG || '').replace(/^v/u, '')
  || '0.1.0-dev'
const root = fileURLToPath(new URL('..', import.meta.url))
const outputDir = join(root, 'release-output', profile, platform, arch)
const bundleDir = join(outputDir, 'bundle')
const archiveName = `corevia-desk-${profile}-${version}-${platform}-${arch}.tar.gz`
const archivePath = join(outputDir, archiveName)

if (!/^[A-Za-z0-9][A-Za-z0-9._-]*$/u.test(version)) {
  throw new Error('COREVIA_DESK_VERSION must contain only letters, numbers, dots, underscores, and hyphens.')
}

if (!existsSync(bundleDir)) throw new Error(`Bundle does not exist: ${bundleDir}`)
mkdirSync(outputDir, { recursive: true })

const tar = process.platform === 'win32' ? 'tar.exe' : 'tar'
const result = spawnSync(tar, ['-czf', archivePath, '-C', dirname(bundleDir), basename(bundleDir)], {
  cwd: root,
  stdio: 'inherit'
})
if (result.error) throw result.error
if (result.status !== 0) throw new Error(`Could not create ${archiveName}.`)

console.log(`Corevia Desk archive prepared: ${archivePath}`)
