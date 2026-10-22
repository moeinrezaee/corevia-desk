import assert from 'node:assert/strict'
import test from 'node:test'
import { latestSessionPerDevice, sessionDeviceKey } from '../utils/session-history.mjs'

test('shows one latest history entry per Peer ID in newest-first order', () => {
  const sessions = [
    { id: 'older-peer-a', peerId: 'peer-a', name: 'Work Mac', platform: 'Mac OS', state: 'failed', endedAt: 100 },
    { id: 'peer-b', peerId: 'peer-b', name: 'Build PC', platform: 'Windows', state: 'closed', endedAt: 150 },
    { id: 'newer-peer-a', peerId: 'peer-a', name: 'Work Mac', platform: 'Mac OS', state: 'closed', endedAt: 200 }
  ]

  const latest = latestSessionPerDevice(sessions)

  assert.deepEqual(latest.map(({ id }) => id), ['newer-peer-a', 'peer-b'])
  assert.deepEqual(latest.map(({ endedAt }) => endedAt), [200, 150])
  assert.deepEqual(latest.map(({ state }) => state), ['closed', 'closed'])
  assert.equal(sessions.length, 3)
})

test('treats Relay suffix and case differences as the same Peer ID', () => {
  assert.equal(sessionDeviceKey({ peerId: 'Peer-A/r' }), sessionDeviceKey({ peerId: 'peer-a' }))
})

test('uses address or device name only when Peer ID is missing', () => {
  assert.equal(
    sessionDeviceKey({ address: 'device-address' }),
    sessionDeviceKey({ address: 'DEVICE-ADDRESS' })
  )
  assert.equal(
    sessionDeviceKey({ name: 'Work Mac', platform: 'Mac OS' }),
    sessionDeviceKey({ name: 'work mac', platform: 'mac os' })
  )
})

test('keeps unidentified sessions separate when they have distinct session IDs', () => {
  const latest = latestSessionPerDevice([
    { id: 'session-a', name: 'Not available', endedAt: 100 },
    { id: 'session-b', name: 'Not available', endedAt: 90 }
  ])

  assert.deepEqual(latest.map(({ id }) => id), ['session-a', 'session-b'])
})
