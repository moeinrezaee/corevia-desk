import { createHmac, randomBytes, randomUUID } from 'node:crypto'
import { createInterface } from 'node:readline'
import { pathToFileURL } from 'node:url'
import { createConfiguredRustBridge } from './bridge-adapter.mjs'
import { findTool, normalizeProfile, toolsForProfile } from './catalog.mjs'
import { authorizeToolCall, safeDeniedMessage } from './policy.mjs'

const JSON_RPC_VERSION = '2.0'
const LATEST_PROTOCOL_VERSION = '2025-11-25'
const SUPPORTED_PROTOCOL_VERSIONS = new Set([
  LATEST_PROTOCOL_VERSION,
  '2025-06-18',
  '2025-03-26',
  '2024-11-05'
])
const APPROVAL_TIMEOUT_MS = 120_000
const MCP_APPROVAL_KEY_ENV = 'COREVIA_DESK_MCP_APPROVAL_KEY'
let requestSequence = 0

const response = (id, result) => ({ jsonrpc: JSON_RPC_VERSION, id, result })
const error = (id, code, message) => ({
  jsonrpc: JSON_RPC_VERSION,
  id,
  error: { code, message }
})

const contentResult = (value, isError = false) => ({
  isError,
  content: [{ type: 'text', text: JSON.stringify(value) }]
})

const nextRequestId = () => {
  requestSequence += 1
  return `mcp-${requestSequence.toString(36)}`
}

const canonicalize = (value) => {
  if (Array.isArray(value)) return value.map(canonicalize)
  if (value && typeof value === 'object') {
    return Object.fromEntries(
      Object.keys(value).sort().map((key) => [key, canonicalize(value[key])])
    )
  }
  return value
}

const canonicalJson = (value) => JSON.stringify(canonicalize(value))

const approvalProof = (key, reference, operation, payload) => createHmac('sha256', key)
  .update(reference)
  .update('\0')
  .update(operation)
  .update('\0')
  .update(canonicalJson(payload))
  .digest('hex')

const safePromptValue = (key, value, args) => {
  if (/password|secret|token/i.test(key)) return '[hidden]'
  if (key === 'value' && /password|secret|token|api.?key|^option$/i.test(String(args.key ?? ''))) {
    return '[hidden]'
  }
  const text = typeof value === 'string' ? value : JSON.stringify(value)
  return String(text).replace(/[\u0000-\u001f\u007f]/gu, ' ').slice(0, 180)
}

const approvalPrompt = (tool, args) => {
  const fieldNames = {
    remoteId: 'Peer ID',
    connectionId: 'Session',
    permission: 'Permission',
    enabled: 'Enable',
    key: 'Setting',
    value: 'Value',
    localPort: 'Local port',
    remotePort: 'Remote port',
    action: 'Requested change',
    peerId: 'Peer ID',
    password: 'Peer password',
    message: 'Message'
  }
  const details = Object.entries(args)
    .map(([key, value]) => `• ${fieldNames[key] ?? key}: ${safePromptValue(key, value, args)}`)
  return [
    'Corevia Desk requests your approval for one action. Review the action and details before accepting; you can decline or cancel.',
    `Tool: ${tool.name}`,
    `Action: ${tool.description}`,
    ...(details.length ? ['Details:', ...details] : [])
  ].join('\n')
}

const supportsFormElicitation = (capabilities) => {
  const elicitation = capabilities?.elicitation
  if (!elicitation || typeof elicitation !== 'object' || Array.isArray(elicitation)) return false
  if (Object.keys(elicitation).length === 0) return true
  return elicitation.form !== null
    && typeof elicitation.form === 'object'
    && !Array.isArray(elicitation.form)
}

export const createMcpServer = ({
  profile = process.env.COREVIA_DESK_PROFILE || 'full-desk',
  bridge,
  approvalKey: configuredApprovalKey
} = {}) => {
  const normalizedProfile = normalizeProfile(profile)
  if (!normalizedProfile) {
    throw new Error('COREVIA_DESK_PROFILE must be customer-support or full-desk.')
  }

  const approvalKey = configuredApprovalKey ? Buffer.from(configuredApprovalKey) : randomBytes(32)
  if (approvalKey.length !== 32) throw new Error('The MCP approval key must be exactly 32 bytes.')
  const rustBridge = bridge ?? createConfiguredRustBridge({
    env: { [MCP_APPROVAL_KEY_ENV]: approvalKey.toString('hex') }
  })
  rustBridge.setApprovalKey?.(approvalKey)
  const pendingClientRequests = new Map()
  let writeJsonRpc
  let initialized = false
  let negotiatedProtocolVersion
  let elicitationFormSupported = false
  let approvalInFlight = false

  const sendClientRequest = (method, params) => {
    if (typeof writeJsonRpc !== 'function') return Promise.resolve(null)
    const id = `corevia-${nextRequestId()}`
    return new Promise((resolve) => {
      const timer = setTimeout(() => {
        pendingClientRequests.delete(id)
        resolve({ action: 'timeout' })
      }, APPROVAL_TIMEOUT_MS)
      pendingClientRequests.set(id, { resolve, timer })
      try {
        writeJsonRpc({ jsonrpc: JSON_RPC_VERSION, id, method, params })
      } catch {
        clearTimeout(timer)
        pendingClientRequests.delete(id)
        resolve(null)
      }
    })
  }

  const requestUserApproval = async (tool, args) => {
    if (!initialized || negotiatedProtocolVersion !== LATEST_PROTOCOL_VERSION || !elicitationFormSupported) {
      return { approved: false, code: 'mcp-user-approval-unavailable' }
    }
    if (approvalInFlight) {
      return { approved: false, code: 'mcp-user-approval-unavailable' }
    }

    approvalInFlight = true
    try {
      const result = await sendClientRequest('elicitation/create', {
        mode: 'form',
        message: approvalPrompt(tool, args),
        requestedSchema: {
          type: 'object',
          properties: {
            approved: {
              type: 'boolean',
              title: 'Approve this action?',
              description: 'Choose true only if you want Corevia Desk to run this exact action.'
            }
          },
          required: ['approved']
        }
      })

      if (!result || result.action === 'timeout') {
        return { approved: false, code: 'mcp-user-approval-timeout' }
      }
      if (result.action === 'decline') {
        return { approved: false, code: 'mcp-user-approval-declined' }
      }
      if (result.action === 'cancel') {
        return { approved: false, code: 'mcp-user-approval-cancelled' }
      }
      if (result.action !== 'accept' || result.content?.approved !== true) {
        return { approved: false, code: result.action === 'accept' ? 'mcp-user-approval-declined' : 'mcp-user-approval-invalid' }
      }
      return { approved: true }
    } finally {
      approvalInFlight = false
    }
  }

  const resolveClientResponse = (message) => {
    if (!Object.prototype.hasOwnProperty.call(message, 'id')) return false
    const pending = pendingClientRequests.get(message.id)
    if (!pending) return false
    pendingClientRequests.delete(message.id)
    clearTimeout(pending.timer)
    pending.resolve(message.error ? null : message.result)
    return true
  }

  const handleMessage = async (message) => {
    const id = Object.prototype.hasOwnProperty.call(message ?? {}, 'id') ? message.id : null
    if (!message || message.jsonrpc !== JSON_RPC_VERSION) {
      return error(id, -32600, 'Invalid JSON-RPC request.')
    }

    if (typeof message.method !== 'string') {
      if (resolveClientResponse(message)) return null
      return error(id, -32600, 'Invalid JSON-RPC request.')
    }

    if (message.method === 'notifications/initialized') {
      initialized = true
      return null
    }

    if (message.method === 'initialize') {
      const params = message.params ?? {}
      const requestedVersion = params.protocolVersion
      negotiatedProtocolVersion = SUPPORTED_PROTOCOL_VERSIONS.has(requestedVersion)
        ? requestedVersion
        : LATEST_PROTOCOL_VERSION
      elicitationFormSupported = negotiatedProtocolVersion === LATEST_PROTOCOL_VERSION
        && supportsFormElicitation(params.capabilities)
      initialized = false
      return response(id, {
        protocolVersion: negotiatedProtocolVersion,
        capabilities: { tools: {} },
        serverInfo: {
          name: 'corevia-desk',
          version: '0.1.0'
        }
      })
    }

    if (message.method === 'ping') return response(id, {})

    if (message.method === 'tools/list') {
      return response(id, { tools: toolsForProfile(normalizedProfile) })
    }

    if (message.method === 'tools/call') {
      const params = message.params
      if (!params || typeof params.name !== 'string') {
        return error(id, -32602, 'A tool name is required.')
      }

      const args = params.arguments ?? {}
      let decision = authorizeToolCall(normalizedProfile, params.name, args)
      let approvalReference
      let approvalProofValue
      if (!decision.allowed && decision.code === 'explicit-approval-required') {
        const tool = findTool(normalizedProfile, params.name)
        const consent = tool?.requiresApproval
          ? await requestUserApproval(tool, args)
          : { approved: false, code: 'mcp-user-approval-unavailable' }
        if (!consent.approved) {
          return response(id, contentResult({
            status: 'denied',
            error: { code: consent.code, message: safeDeniedMessage(consent.code) }
          }, true))
        }

        decision = authorizeToolCall(normalizedProfile, params.name, args, { approvalGranted: true })
        if (!decision.allowed) {
          return response(id, contentResult({
            status: 'denied',
            error: { code: decision.code, message: safeDeniedMessage(decision.code) }
          }, true))
        }
        approvalReference = randomUUID()
        approvalProofValue = approvalProof(approvalKey, approvalReference, decision.operation, args)
      }

      if (!decision.allowed) {
        return response(id, contentResult({
          status: 'denied',
          error: { code: decision.code, message: safeDeniedMessage(decision.code) }
        }, true))
      }

      try {
        const bridgeResponse = await rustBridge.invoke({
          requestId: nextRequestId(),
          operation: decision.operation,
          payload: args,
          approvalReference,
          approvalProof: approvalProofValue,
          profile: normalizedProfile,
          surface: 'mcp'
        })
        if (!bridgeResponse || typeof bridgeResponse !== 'object') {
          return response(id, contentResult({
            status: 'error',
            error: {
              code: 'invalid-bridge-response',
              message: 'The local Corevia Desk bridge returned an invalid response.'
            }
          }, true))
        }
        const executionFailed = bridgeResponse?.data?.executed === false
        return response(id, contentResult(
          bridgeResponse,
          bridgeResponse?.status === 'denied' || bridgeResponse?.status === 'error' || executionFailed
        ))
      } catch {
        return response(id, contentResult({
          status: 'error',
          error: {
            code: 'bridge-unavailable',
            message: 'The local Corevia Desk bridge could not handle the request.'
          }
        }, true))
      }
    }

    return error(id, -32601, 'This Corevia Desk method is not available.')
  }

  const run = () => {
    const input = createInterface({ input: process.stdin, crlfDelay: Infinity })
    writeJsonRpc = (message) => process.stdout.write(`${JSON.stringify(message)}\n`)
    input.on('line', (line) => {
      if (!line.trim()) return
      let message
      try {
        message = JSON.parse(line)
      } catch {
        writeJsonRpc(error(null, -32700, 'Invalid JSON.'))
        return
      }
      void handleMessage(message)
        .then((result) => { if (result) writeJsonRpc(result) })
        .catch(() => { if (message.id !== undefined) writeJsonRpc(error(message.id, -32603, 'The MCP request could not be completed.')) })
    })
    input.on('close', () => {
      for (const [id, pending] of pendingClientRequests) {
        clearTimeout(pending.timer)
        pending.resolve({ action: 'cancel' })
        pendingClientRequests.delete(id)
      }
      rustBridge.close?.()
      writeJsonRpc = undefined
    })
  }

  return {
    handleMessage,
    run,
    profile: normalizedProfile,
    setTransportWriter(writer) {
      writeJsonRpc = writer
    }
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  createMcpServer().run()
}
