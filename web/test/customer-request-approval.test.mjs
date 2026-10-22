import assert from 'node:assert/strict'
import test from 'node:test'
import { approveCustomerRequest, createCustomerRequestSelection } from '../utils/customer-request-approval.mjs'

const completed = { status: 'accepted', data: { executed: true } }

const requestHarness = (responses = []) => {
  const calls = []
  return {
    calls,
    request: async (...args) => {
      calls.push(args)
      return responses.shift() ?? completed
    }
  }
}

test('each incoming request starts with screen sharing on and keyboard input off', () => {
  const previous = createCustomerRequestSelection()
  previous.input = true

  const next = createCustomerRequestSelection()

  assert.deepEqual(next, { screen: true, input: false })
  assert.notStrictEqual(next, previous)
})

test('screen sharing is required and a declined screen choice sends no native calls', async () => {
  const harness = requestHarness()
  const result = await approveCustomerRequest({
    connectionId: 'incoming-1', screen: false, input: true, request: harness.request
  })

  assert.deepEqual(result, { status: 'screen-required' })
  assert.deepEqual(harness.calls, [])
})

test('screen-only approval revokes keyboard input before authorizing the session', async () => {
  const harness = requestHarness()
  const result = await approveCustomerRequest({
    connectionId: 'incoming-1', screen: true, input: false, request: harness.request
  })

  assert.deepEqual(result, { status: 'approved', permissions: ['screen'] })
  assert.deepEqual(harness.calls.map(([operation, payload]) => [operation, payload]), [
    ['change-connection-permission', { connectionId: 'incoming-1', permission: 'keyboard', enabled: false }],
    ['approve-connection', { connectionId: 'incoming-1' }]
  ])
})

test('keyboard input is enabled only after explicit selection and session approval', async () => {
  const harness = requestHarness()
  const result = await approveCustomerRequest({
    connectionId: 'incoming-1', screen: true, input: true, request: harness.request
  })

  assert.deepEqual(result, { status: 'approved', permissions: ['screen', 'input'] })
  assert.deepEqual(harness.calls.map(([operation, payload]) => [operation, payload]), [
    ['change-connection-permission', { connectionId: 'incoming-1', permission: 'keyboard', enabled: false }],
    ['approve-connection', { connectionId: 'incoming-1' }],
    ['change-connection-permission', { connectionId: 'incoming-1', permission: 'keyboard', enabled: true }]
  ])
})

test('failed keyboard revocation fails closed and never authorizes the connection', async () => {
  const harness = requestHarness([{ status: 'accepted', data: { executed: false } }])
  const result = await approveCustomerRequest({
    connectionId: 'incoming-1', screen: true, input: false, request: harness.request
  })

  assert.equal(result.status, 'permission-failed')
  assert.equal(harness.calls.length, 1)
})

test('failed session approval never re-enables keyboard input', async () => {
  const failedApproval = { status: 'accepted', data: { executed: false } }
  const harness = requestHarness([completed, failedApproval])
  const result = await approveCustomerRequest({
    connectionId: 'incoming-1', screen: true, input: true, request: harness.request
  })

  assert.equal(result.status, 'approval-failed')
  assert.equal(harness.calls.length, 2)
  assert.equal(harness.calls[1][0], 'approve-connection')
})

test('failed keyboard enablement leaves an approved screen-only session', async () => {
  const failedEnable = { status: 'accepted', data: { executed: false } }
  const harness = requestHarness([completed, completed, failedEnable])
  const result = await approveCustomerRequest({
    connectionId: 'incoming-1', screen: true, input: true, request: harness.request
  })

  assert.deepEqual(result, {
    status: 'approved-screen-only',
    response: failedEnable,
    permissions: ['screen']
  })
})
