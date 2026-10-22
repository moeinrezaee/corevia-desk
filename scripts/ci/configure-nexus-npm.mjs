import { writeFileSync } from 'node:fs'
import { resolve } from 'node:path'

const registry = process.env.NPM_CONFIG_REGISTRY
const username = process.env.NEXUS_USERNAME
const password = process.env.NEXUS_PASSWORD

if (!registry || !username || !password) {
  throw new Error('NPM_CONFIG_REGISTRY, NEXUS_USERNAME, and NEXUS_PASSWORD are required.')
}

const registryUrl = new URL(registry)
const registryPath = registryUrl.pathname.endsWith('/') ? registryUrl.pathname : `${registryUrl.pathname}/`
const authKey = `//${registryUrl.host}${registryPath}`
const userConfig = resolve(process.env.NPM_CONFIG_USERCONFIG || '.npmrc.ci')
const auth = Buffer.from(`${username}:${password}`, 'utf8').toString('base64')

writeFileSync(
  userConfig,
  [`registry=${registryUrl.href}`, `${authKey}:_auth=${auth}`, `${authKey}:always-auth=true`, ''].join('\n'),
  { mode: 0o600 },
)

console.log(`Configured authenticated npm access for ${registryUrl.host}.`)
