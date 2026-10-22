import assert from 'node:assert/strict'
import test from 'node:test'
import { createRustBridgeAdapter } from '../src/bridge-adapter.mjs'
import { createMcpServer } from '../src/server.mjs'
import { authorizeToolCall, safeDeniedMessage } from '../src/policy.mjs'
import { findTool, getToolCatalog, toolsForProfile } from '../src/catalog.mjs'

const initializeInteractiveClient = async (server, capabilities = { elicitation: { form: {} } }, protocolVersion = '2025-11-25') => {
  const result = await server.handleMessage({
    jsonrpc: '2.0',
    id: 'initialize',
    method: 'initialize',
    params: { protocolVersion, capabilities, clientInfo: { name: 'test-client', version: '1.0.0' } }
  })
  assert.equal(result.result.protocolVersion, protocolVersion)
  await server.handleMessage({ jsonrpc: '2.0', method: 'notifications/initialized' })
}

const autoApprove = (server, action = 'accept', approved = true, requests = []) => {
  server.setTransportWriter((message) => {
    if (message.method !== 'elicitation/create') return
    requests.push(message)
    queueMicrotask(() => {
      void server.handleMessage({
        jsonrpc: '2.0',
        id: message.id,
        result: action === 'accept' ? { action, content: { approved } } : { action }
      })
    })
  })
}

test('customer support exposes only its small tool set', () => {
  const names = toolsForProfile('customer-support').map((tool) => tool.name)
  assert.deepEqual(names, [
    'corevia.get_status',
    'corevia.read_settings',
    'corevia.read_identity',
    'corevia.read_devices',
    'corevia.read_sessions',
    'corevia.write_customer_setting',
    'corevia.approve_connection',
    'corevia.close_session',
    'corevia.change_connection_permission'
  ])
  assert.equal(names.includes('corevia.start_connection'), false)
  assert.equal(names.includes('corevia.use_terminal'), false)
})

test('unknown tools and arbitrary system commands are denied', () => {
  assert.equal(authorizeToolCall('full-desk', 'corevia.run_shell', {}).allowed, false)
  assert.equal(authorizeToolCall('full-desk', 'corevia.system_command', {}).allowed, false)
  assert.match(safeDeniedMessage('tool-not-available'), /not available/i)
})

test('sensitive full desk tools require per-call interactive approval, not caller text', () => {
  assert.equal(authorizeToolCall('full-desk', 'corevia.start_connection', { remoteId: '123' }).code, 'explicit-approval-required')
  assert.equal(authorizeToolCall('full-desk', 'corevia.start_connection', {
    remoteId: '123',
    approvalReference: 'made-up-approval'
  }).code, 'unknown-argument')
  assert.equal(authorizeToolCall('full-desk', 'corevia.start_connection', { remoteId: '123' }).code, 'explicit-approval-required')
  assert.equal(authorizeToolCall('full-desk', 'corevia.start_connection', { remoteId: '123' }, { approvalGranted: true }).allowed, true)
  assert.match(safeDeniedMessage('mcp-user-approval-unavailable'), /not run/i)
  const sendChatArgs = {
    connectionId: 'connection-1',
    message: 'hello from Corevia Desk'
  }
  assert.equal(authorizeToolCall('full-desk', 'corevia.send_chat', sendChatArgs).code, 'explicit-approval-required')
  assert.equal(authorizeToolCall('full-desk', 'corevia.send_chat', sendChatArgs, { approvalGranted: true }).allowed, true)
})

test('server forwards only an allowlisted safe read to the injected Rust adapter', async () => {
  const calls = []
  const server = createMcpServer({
    profile: 'full-desk',
    bridge: createRustBridgeAdapter({
      dispatch(request) {
        calls.push(request)
        return {
          requestId: request.requestId,
          operation: request.operation,
          surface: 'mcp',
          profile: 'full-desk',
          status: 'accepted',
          data: { authorized: true }
        }
      }
    })
  })

  const result = await server.handleMessage({
    jsonrpc: '2.0',
    id: 1,
    method: 'tools/call',
    params: {
      name: 'corevia.read_sessions',
      arguments: {}
    }
  })

  assert.equal(result.result.isError, false)
  assert.equal(calls.length, 1)
  assert.equal(calls[0].operation, 'read-sessions')
  assert.deepEqual(calls[0].payload, {})
  assert.equal(calls[0].approvalReference, undefined)
})

test('natively authorized actions that lack an executor are reported as tool failures', async () => {
  const server = createMcpServer({
    profile: 'full-desk',
    bridge: {
      invoke: async (request) => ({
        requestId: request.requestId,
        operation: request.operation,
        surface: 'mcp',
        profile: 'full-desk',
        status: 'accepted',
        data: {
          authorized: true,
          executed: false,
          next: 'native-executor',
          reason: 'platform-executor-required'
        }
      })
    }
  })
  autoApprove(server)
  await initializeInteractiveClient(server)

  const unavailableTools = [
    { name: 'corevia.use_camera', arguments: { connectionId: 'synthetic-session' } },
    { name: 'corevia.use_printer', arguments: { connectionId: 'synthetic-session' } },
    { name: 'corevia.manage_service', arguments: { action: 'status' } }
  ]
  for (const [index, tool] of unavailableTools.entries()) {
    const result = await server.handleMessage({
      jsonrpc: '2.0',
      id: `unavailable-tool-${index}`,
      method: 'tools/call',
      params: tool
    })

    assert.equal(result.result.isError, true, tool.name)
    const payload = JSON.parse(result.result.content[0].text)
    assert.equal(payload.status, 'accepted', tool.name)
    assert.equal(payload.data.authorized, true, tool.name)
    assert.equal(payload.data.executed, false, tool.name)
    assert.equal(payload.data.reason, 'platform-executor-required', tool.name)
  }
})

test('sensitive MCP calls are denied when the client cannot present interactive approval', async () => {
  const calls = []
  const server = createMcpServer({
    profile: 'full-desk',
    bridge: { invoke: async (request) => calls.push(request) }
  })

  const result = await server.handleMessage({
    jsonrpc: '2.0',
    id: 3,
    method: 'tools/call',
    params: {
      name: 'corevia.start_connection',
      arguments: { remoteId: 'synthetic-peer-id' }
    }
  })

  assert.equal(result.result.isError, true)
  assert.match(result.result.content[0].text, /mcp-user-approval-unavailable/)
  assert.equal(calls.length, 0)
})

test('an elicitation-capable client must explicitly accept before native dispatch', async () => {
  const calls = []
  const requests = []
  const server = createMcpServer({
    profile: 'full-desk',
    bridge: { invoke: async (request) => { calls.push(request); return { status: 'accepted' } } }
  })
  autoApprove(server, 'accept', true, requests)
  await initializeInteractiveClient(server)
  const result = await server.handleMessage({
    jsonrpc: '2.0',
    id: 5,
    method: 'tools/call',
    params: { name: 'corevia.start_connection', arguments: { remoteId: 'real-peer-id' } }
  })

  assert.equal(result.result.isError, false)
  assert.equal(requests.length, 1)
  assert.equal(requests[0].method, 'elicitation/create')
  assert.match(requests[0].params.message, /real-peer-id/)
  assert.equal(requests[0].params.requestedSchema.properties.approved.type, 'boolean')
  assert.equal(calls.length, 1)
  assert.equal(calls[0].approvalReference.length, 36)
  assert.match(calls[0].approvalProof, /^[a-f0-9]{64}$/)
})

test('sending chat requires per-call approval and shows the sanitized message in the prompt', async () => {
  const calls = []
  const prompts = []
  const server = createMcpServer({
    profile: 'full-desk',
    bridge: {
      invoke: async (request) => {
        calls.push(request)
        return { status: 'accepted', data: { executed: true } }
      }
    }
  })
  autoApprove(server, 'accept', true, prompts)
  await initializeInteractiveClient(server)

  const message = 'synthetic test message — no real chat sent'
  const result = await server.handleMessage({
    jsonrpc: '2.0',
    id: 'approved-chat-message',
    method: 'tools/call',
    params: {
      name: 'corevia.send_chat',
      arguments: { connectionId: '1', message }
    }
  })

  assert.equal(result.result.isError, false)
  assert.equal(prompts.length, 1)
  assert.match(prompts[0].params.message, new RegExp(`Message: ${message}`))
  assert.equal(calls.length, 1)
  assert.match(calls[0].approvalProof, /^[a-f0-9]{64}$/)
})

test('declined, cancelled, timed-out, and false approval responses never reach native dispatch', async (t) => {
  for (const [action, approved, expectedCode] of [
    ['decline', false, 'mcp-user-approval-declined'],
    ['cancel', false, 'mcp-user-approval-cancelled'],
    ['timeout', false, 'mcp-user-approval-timeout'],
    ['accept', false, 'mcp-user-approval-declined']
  ]) {
    await t.test(`${action}:${approved}`, async () => {
      const calls = []
      const server = createMcpServer({ profile: 'full-desk', bridge: { invoke: async (request) => calls.push(request) } })
      autoApprove(server, action, approved)
      await initializeInteractiveClient(server)
      const result = await server.handleMessage({
        jsonrpc: '2.0',
        id: 6,
        method: 'tools/call',
        params: { name: 'corevia.start_connection', arguments: { remoteId: 'real-peer-id' } }
      })
      assert.equal(result.result.isError, true)
      assert.match(result.result.content[0].text, new RegExp(expectedCode))
      assert.equal(calls.length, 0)
    })
  }
})

test('older clients and clients without form elicitation stay fail-closed', async (t) => {
  for (const [version, capabilities] of [
    ['2024-11-05', {}],
    ['2025-11-25', {}],
    ['2025-11-25', { elicitation: { url: {} } }],
    ['2025-11-25', { elicitation: { form: false } }],
    ['2025-11-25', { elicitation: { form: null } }]
  ]) {
    await t.test(`${version}:${JSON.stringify(capabilities)}`, async () => {
      const calls = []
      const server = createMcpServer({ profile: 'full-desk', bridge: { invoke: async (request) => calls.push(request) } })
      autoApprove(server)
      await initializeInteractiveClient(server, capabilities, version)
      const result = await server.handleMessage({
        jsonrpc: '2.0',
        id: 7,
        method: 'tools/call',
        params: { name: 'corevia.start_connection', arguments: { remoteId: 'real-peer-id' } }
      })
      assert.equal(result.result.isError, true)
      assert.match(result.result.content[0].text, /mcp-user-approval-unavailable/)
      assert.equal(calls.length, 0)
    })
  }
})

test('customer support cannot forward outbound connection calls', async () => {
  const calls = []
  const server = createMcpServer({
    profile: 'customer-support',
    bridge: { invoke: async (request) => calls.push(request) }
  })
  const result = await server.handleMessage({
    jsonrpc: '2.0',
    id: 2,
    method: 'tools/call',
    params: {
      name: 'corevia.start_connection',
      arguments: { remoteId: '123', approvalReference: 'customer-connect-1' }
    }
  })

  assert.equal(result.result.isError, true)
  assert.equal(calls.length, 0)
})

test('customer support permission tools stay narrow and require approval for allowed permissions', () => {
  for (const permission of ['file', 'clipboard', 'audio']) {
  assert.equal(authorizeToolCall('customer-support', 'corevia.change_connection_permission', {
    connectionId: 'connection-1',
    permission,
    enabled: true
  }).code, 'customer-permission-only')
  }
  assert.equal(authorizeToolCall('customer-support', 'corevia.change_connection_permission', {
    connectionId: 'connection-1',
    permission: 'keyboard',
    enabled: true
  }).code, 'explicit-approval-required')
  assert.equal(authorizeToolCall('customer-support', 'corevia.change_connection_permission', {
    connectionId: '1',
    permission: 'keyboard',
    enabled: true
  }, { approvalGranted: true }).allowed, true)
})

test('full desk exposes every bounded native capability and never exposes shell access', () => {
  const names = toolsForProfile('full-desk').map((tool) => tool.name)
  for (const name of [
    'corevia.get_status',
    'corevia.read_settings',
    'corevia.read_identity',
    'corevia.read_devices',
    'corevia.read_sessions',
    'corevia.write_customer_setting',
    'corevia.write_global_setting',
    'corevia.approve_connection',
    'corevia.close_session',
    'corevia.change_connection_permission',
    'corevia.start_connection',
    'corevia.input_control',
    'corevia.use_file_transfer',
    'corevia.use_camera',
    'corevia.use_port_forward',
    'corevia.use_printer',
    'corevia.record_session',
    'corevia.use_terminal',
    'corevia.restart_remote_device',
    'corevia.elevate_session',
    'corevia.toggle_privacy_mode',
    'corevia.manage_service',
    'corevia.manage_peers',
    'corevia.send_chat',
    'corevia.use_voice_call'
  ]) assert.equal(names.includes(name), true, name)
  assert.equal(names.some((name) => name.includes('system') || name.includes('shell')), false)
})

const validArguments = {
  'corevia.get_status': {},
  'corevia.read_settings': {},
  'corevia.read_identity': {},
  'corevia.read_devices': {},
  'corevia.read_sessions': {},
  'corevia.write_customer_setting': { key: 'theme', value: 'dark' },
  'corevia.write_global_setting': { key: 'theme', value: 'dark' },
  'corevia.approve_connection': { connectionId: '1' },
  'corevia.close_session': { connectionId: '1' },
  'corevia.change_connection_permission': {
    connectionId: '1',
    permission: 'keyboard',
    enabled: true
  },
  'corevia.start_connection': { remoteId: '123' },
  'corevia.input_control': {
    connectionId: '1',
    enabled: false
  },
  'corevia.use_file_transfer': { connectionId: '1' },
  'corevia.use_camera': { connectionId: '1' },
  'corevia.use_port_forward': {
    connectionId: '1',
    localPort: 22000,
    remotePort: 22,
    remoteHost: 'localhost'
  },
  'corevia.use_printer': { connectionId: '1' },
  'corevia.record_session': { connectionId: '1', enabled: true },
  'corevia.use_terminal': { connectionId: '1' },
  'corevia.restart_remote_device': { connectionId: '1' },
  'corevia.elevate_session': { connectionId: '1' },
  'corevia.toggle_privacy_mode': { connectionId: '1', enabled: true },
  'corevia.manage_service': { action: 'status' },
  'corevia.manage_peers': { peerId: '123', action: 'favorite' },
  'corevia.send_chat': { connectionId: '1', message: 'matrix message' },
  'corevia.use_voice_call': { connectionId: '1', enabled: false }
}

const needsUserApproval = (profile, tool) => findTool(profile, tool.name)?.requiresApproval === true

test('every published tool has a valid profile-specific policy path', () => {
  const catalog = getToolCatalog()
  assert.equal(new Set(catalog.map((tool) => tool.name)).size, catalog.length)

  for (const profile of ['customer-support', 'full-desk']) {
    const exposed = toolsForProfile(profile)
    assert.ok(exposed.length > 0)
    for (const tool of exposed) {
      assert.ok(validArguments[tool.name], tool.name)
      const decision = authorizeToolCall(profile, tool.name, validArguments[tool.name])
      if (needsUserApproval(profile, tool)) {
        assert.equal(decision.code, 'explicit-approval-required', `${profile}:${tool.name}`)
      } else {
        assert.equal(decision.allowed, true, `${profile}:${tool.name}`)
      }
      assert.equal(Object.hasOwn(tool.inputSchema.properties ?? {}, 'approvalReference'), false, `${tool.name} must not accept caller-supplied approval`)
      assert.equal('operation' in tool, false, `${tool.name} leaks operation metadata`)
      assert.equal('capability' in tool, false, `${tool.name} leaks capability metadata`)
      assert.equal('profiles' in tool, false, `${tool.name} leaks profile metadata`)
    }
  }

  for (const tool of toolsForProfile('full-desk')) {
    if (!toolsForProfile('customer-support').some((candidate) => candidate.name === tool.name)) {
      assert.equal(authorizeToolCall('customer-support', tool.name, validArguments[tool.name]).code, 'tool-not-available')
    }
  }
})

test('every published tool traverses the MCP server dispatch boundary with per-call approval', async () => {
  for (const profile of ['customer-support', 'full-desk']) {
    const calls = []
    const approvalRequests = []
    const server = createMcpServer({
      profile,
      bridge: createRustBridgeAdapter({
        dispatch(request) {
          calls.push(request)
          return {
            requestId: request.requestId,
            operation: request.operation,
            surface: 'mcp',
            profile,
            status: 'accepted',
            data: { executed: true }
          }
        }
      })
    })
    autoApprove(server, 'accept', true, approvalRequests)
    await initializeInteractiveClient(server)
    const listed = await server.handleMessage({
      jsonrpc: '2.0',
      id: `${profile}-list`,
      method: 'tools/list'
    })
    assert.deepEqual(
      listed.result.tools.map((tool) => tool.name),
      toolsForProfile(profile).map((tool) => tool.name)
    )

    let expectedBridgeCalls = 0
    for (const tool of toolsForProfile(profile)) {
      const args = structuredClone(validArguments[tool.name])
      const result = await server.handleMessage({
        jsonrpc: '2.0',
        id: `${profile}-${tool.name}`,
        method: 'tools/call',
        params: { name: tool.name, arguments: args }
      })
      expectedBridgeCalls += 1
      assert.equal(result.result.isError, false, `${profile}:${tool.name}`)
      assert.equal(calls.at(-1).surface, 'mcp', `${profile}:${tool.name}`)
      assert.deepEqual(calls.at(-1).payload, args, `${profile}:${tool.name}`)
      if (needsUserApproval(profile, tool)) {
        assert.equal(typeof calls.at(-1).approvalReference, 'string', `${profile}:${tool.name}`)
        assert.match(calls.at(-1).approvalProof, /^[a-f0-9]{64}$/, `${profile}:${tool.name}`)
      } else {
        assert.equal(calls.at(-1).approvalReference, undefined, `${profile}:${tool.name}`)
        assert.equal(calls.at(-1).approvalProof, undefined, `${profile}:${tool.name}`)
      }
    }

    assert.equal(calls.length, expectedBridgeCalls, profile)
    assert.equal(approvalRequests.length, toolsForProfile(profile).filter((tool) => needsUserApproval(profile, tool)).length, profile)
  }
})

test('the matrix fails closed for malformed arguments and missing approvals', () => {
  for (const tool of toolsForProfile('full-desk')) {
    const args = validArguments[tool.name]
    assert.ok(args, tool.name)
    assert.equal(authorizeToolCall('full-desk', tool.name, { ...args, unexpected: true }).allowed, false, tool.name)
    if (needsUserApproval('full-desk', tool)) {
      assert.equal(authorizeToolCall('full-desk', tool.name, args).allowed, false, tool.name)
      assert.equal(authorizeToolCall('full-desk', tool.name, args, { approvalGranted: true }).allowed, true, tool.name)
    }
  }
})

test('port forwarding accepts an optional host and rejects invalid host values', () => {
  const tool = 'corevia.use_port_forward'
  const base = { connectionId: '1', localPort: 22000, remotePort: 22 }
  assert.equal(authorizeToolCall('full-desk', tool, { ...base, remoteHost: 'localhost' }, { approvalGranted: true }).allowed, true)
  assert.equal(authorizeToolCall('full-desk', tool, base, { approvalGranted: true }).allowed, true)
  assert.equal(authorizeToolCall('full-desk', tool, { ...base, remoteHost: '' }, { approvalGranted: true }).code, 'invalid-argument-value')
  assert.equal(authorizeToolCall('full-desk', tool, { ...base, remoteHost: 'host name' }, { approvalGranted: true }).code, 'invalid-argument-value')
  assert.equal(authorizeToolCall('full-desk', tool, { ...base, remoteHost: 'x'.repeat(256) }, { approvalGranted: true }).code, 'invalid-argument-value')
  assert.equal(authorizeToolCall('full-desk', tool, { ...base, remoteHost: 22 }, { approvalGranted: true }).code, 'invalid-argument-type')
})

test('voice-call MCP tool exposes approved enable and disable operations', () => {
  const tool = 'corevia.use_voice_call'
  const base = { connectionId: '1' }
  assert.equal(authorizeToolCall('full-desk', tool, base, { approvalGranted: true }).allowed, true)
  assert.equal(authorizeToolCall('full-desk', tool, { ...base, enabled: true }, { approvalGranted: true }).allowed, true)
  assert.equal(authorizeToolCall('full-desk', tool, { ...base, enabled: false }, { approvalGranted: true }).allowed, true)
  assert.equal(authorizeToolCall('full-desk', tool, { ...base, enabled: 'false' }, { approvalGranted: true }).code, 'invalid-argument-type')
})
