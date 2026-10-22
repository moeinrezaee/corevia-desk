import { findTool, isConnectionPermissionAllowed, normalizeProfile } from './catalog.mjs'

const MAX_STRING_LENGTH = 512

const isPlainObject = (value) => {
  return value !== null && typeof value === 'object' && !Array.isArray(value)
}

const matchesType = (value, expected) => {
  const types = Array.isArray(expected) ? expected : [expected]
  return types.some((type) => {
    if (type === 'null') return value === null
    if (type === 'string') return typeof value === 'string'
    if (type === 'number') return typeof value === 'number' && Number.isFinite(value)
    if (type === 'boolean') return typeof value === 'boolean'
    return false
  })
}

const validString = (value) => {
  return typeof value === 'string'
    && value.length > 0
    && value.length <= MAX_STRING_LENGTH
    && !/[\u0000-\u001f\u007f\s]/u.test(value)
}

const validArgumentString = (key, value) => {
  if (key === 'message' || key === 'value') {
    return typeof value === 'string'
      && value.length > 0
      && value.length <= MAX_STRING_LENGTH
      && !/[\u0000-\u001f\u007f]/u.test(value)
  }
  return validString(value)
}

const validateArguments = (tool, args) => {
  if (!isPlainObject(args)) return 'invalid-arguments'

  const schema = tool.inputSchema
  const allowedKeys = new Set(Object.keys(schema.properties ?? {}))
  if (Object.keys(args).some((key) => !allowedKeys.has(key))) return 'unknown-argument'

  for (const required of schema.required ?? []) {
    if (!(required in args)) return 'missing-argument'
  }

  for (const [key, value] of Object.entries(args)) {
    const definition = schema.properties[key]
    if (!matchesType(value, definition.type)) return 'invalid-argument-type'
    if (definition.type === 'string' && !validArgumentString(key, value)) return 'invalid-argument-value'
    if (definition.maxLength && value.length > definition.maxLength) return 'invalid-argument-value'
    if (key.endsWith('Port') && (!Number.isInteger(value) || value < 1 || value > 65535)) {
      return 'invalid-argument-value'
    }
  }

  return null
}

export const authorizeToolCall = (profile, name, args = {}, { approvalGranted = false } = {}) => {
  const normalized = normalizeProfile(profile)
  if (!normalized) {
    return { allowed: false, code: 'invalid-profile' }
  }

  const tool = findTool(normalized, name)
  if (!tool) {
    return { allowed: false, code: 'tool-not-available' }
  }

  const argumentError = validateArguments(tool, args)
  if (argumentError) {
    return { allowed: false, code: argumentError }
  }

  if (tool.operation === 'change-connection-permission'
    && !isConnectionPermissionAllowed(normalized, args.permission)) {
    return { allowed: false, code: 'customer-permission-only' }
  }

  if (tool.requiresApproval && !approvalGranted) {
    return { allowed: false, code: 'explicit-approval-required' }
  }

  return {
    allowed: true,
    capability: tool.capability,
    operation: tool.operation,
    tool
  }
}

export const safeDeniedMessage = (code) => {
  const messages = {
    'invalid-profile': 'The active Corevia Desk profile is invalid.',
    'tool-not-available': 'This tool is not available in the active product profile.',
    'invalid-arguments': 'Tool arguments must be a JSON object.',
    'unknown-argument': 'The tool received an argument outside its allowlist.',
    'missing-argument': 'A required tool argument is missing.',
    'invalid-argument-type': 'A tool argument has an invalid type.',
    'invalid-argument-value': 'A tool argument has an invalid value.',
    'explicit-approval-required': 'This action requires a separate approval from the MCP client.',
    'mcp-user-approval-unavailable': 'This MCP client does not support interactive approval, so the action was not run.',
    'mcp-user-approval-declined': 'The requested action was declined in the MCP client.',
    'mcp-user-approval-cancelled': 'The approval request was cancelled, so the action was not run.',
    'mcp-user-approval-timeout': 'The approval request timed out, so the action was not run.',
    'mcp-user-approval-invalid': 'The MCP client returned an invalid approval response, so the action was not run.',
    'customer-permission-only': 'This connection permission is not available in the Customer Support profile.'
  }
  return messages[code] ?? 'The Corevia Desk policy denied this request.'
}
