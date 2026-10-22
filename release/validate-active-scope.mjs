import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = fileURLToPath(new URL('..', import.meta.url))
const activeFiles = [
  '.gitlab-ci.yml',
  'release/build-profile.mjs',
  'release/archive-profile.mjs',
  'release/create-manifest.mjs',
  'release/validate-manifest.mjs',
  'web/nuxt.config.ts',
  'web/composables/useCoreviaBridge.ts'
]

const forbidden = /\b(?:flutter|dart)\b/iu
const findings = activeFiles.flatMap((relativePath) => {
  const content = readFileSync(join(root, relativePath), 'utf8')
  return forbidden.test(content) ? [relativePath] : []
})

if (findings.length > 0) {
  throw new Error(`Active Rust + Nuxt/WebView scope mentions Flutter/Dart: ${findings.join(', ')}`)
}

console.log('Active UI scope PASS: Rust + Nuxt/WebView only; Flutter/Dart are not part of the release path.')
