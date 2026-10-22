import assert from 'node:assert/strict'
import test from 'node:test'
import { isBridgeActionSuccessful } from '../utils/bridge-response.mjs'

test('accepted bridge action without an explicit execution failure remains compatible', () => {
  assert.equal(isBridgeActionSuccessful({ status: 'accepted' }), true)
  assert.equal(isBridgeActionSuccessful({ status: 'accepted', data: { executed: true } }), true)
})

test('accepted authorization with executed false is not reported as an action success', () => {
  assert.equal(isBridgeActionSuccessful({ status: 'accepted', data: { executed: false } }), false)
})

test('denied, unsupported, and error responses are not action successes', () => {
  for (const status of ['denied', 'unsupported', 'error']) {
    assert.equal(isBridgeActionSuccessful({ status }), false)
  }
})
