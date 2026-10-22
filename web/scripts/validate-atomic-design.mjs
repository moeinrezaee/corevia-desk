import { existsSync, readFileSync, readdirSync } from 'node:fs'
import { join, relative } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = fileURLToPath(new URL('..', import.meta.url))

const requiredDirectories = [
  'components/atoms',
  'components/molecules',
  'components/organisms',
  'components/templates',
  'layouts',
  'pages',
  'composables',
  'types'
]

const requiredFiles = [
  'package.json',
  'nuxt.config.ts',
  'app.vue',
  'layouts/default.vue',
  'components/templates/DeskWorkspaceTemplate.vue',
  'components/templates/DeskDashboardTemplate.vue',
  'components/templates/DeskSettingsTemplate.vue',
  'pages/index.vue',
  'pages/customer-support.vue',
  'pages/full-desk.vue',
  'assets/css/corevia-desk.css',
  'assets/css/corevia-nuxt.css',
  'README.md'
]

const failures = []

for (const directory of requiredDirectories) {
  if (!existsSync(join(root, directory))) failures.push(`Missing directory: ${directory}`)
}

for (const file of requiredFiles) {
  if (!existsSync(join(root, file))) failures.push(`Missing file: ${file}`)
}

const pageFiles = readdirSync(join(root, 'pages')).filter((file) => file.endsWith('.vue'))
for (const file of pageFiles) {
  const path = join(root, 'pages', file)
  const source = readFileSync(path, 'utf8')
  if (!source.includes('DeskWorkspaceTemplate')) {
    failures.push(`Page does not compose DeskWorkspaceTemplate: pages/${file}`)
  }
  if (!source.match(/Desk(?:Dashboard|Devices|Sessions|Settings)Template/)) {
    failures.push(`Page does not compose a content Template: pages/${file}`)
  }
  if (source.match(/components\/(?:atoms|molecules|organisms)\//)) {
    failures.push(`Page reaches below Template level: pages/${file}`)
  }
}

const sourceReadme = readFileSync(join(root, 'README.md'), 'utf8')
if (!sourceReadme.includes('../corevia-desk-ui')) {
  failures.push('README.md must identify corevia-desk-ui as the visual source')
}

const nuxtConfig = readFileSync(join(root, 'nuxt.config.ts'), 'utf8')
if (!nuxtConfig.includes("pathPrefix: false")) {
  failures.push('Nuxt component discovery must keep Atomic component names readable')
}

if (failures.length) {
  console.error(failures.map((failure) => `FAIL: ${failure}`).join('\n'))
  process.exitCode = 1
} else {
  console.log(`Atomic Design structure PASS (${pageFiles.length} pages, ${relative(process.cwd(), root)})`)
}
