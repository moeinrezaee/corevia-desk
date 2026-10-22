import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import test from 'node:test'

const deviceCard = await readFile(new URL('../components/molecules/DeviceCard.vue', import.meta.url), 'utf8')

test('offline devices use a readable status label instead of a close-like cross', () => {
  assert.match(deviceCard, /<StatusLabel :online="device\.status === 'online'">/)
  assert.match(deviceCard, /t\('offline'\)/)
  assert.doesNotMatch(deviceCard, /os-visual-status|fa-xmark/)
})
