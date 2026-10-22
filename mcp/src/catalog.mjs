const scalar = {
  type: ['string', 'number', 'boolean', 'null']
}

const noArguments = {
  type: 'object',
  properties: {},
  additionalProperties: false
}

const tools = [
  {
    name: 'corevia.get_status',
    description: 'Read the safe local Corevia Desk status.',
    operation: 'get-status',
    capability: 'read-settings',
    profiles: ['customer-support', 'full-desk'],
    requiresApproval: false,
    inputSchema: noArguments
  },
  {
    name: 'corevia.read_settings',
    description: 'Read settings that the active product profile allows.',
    operation: 'read-settings',
    capability: 'read-settings',
    profiles: ['customer-support', 'full-desk'],
    requiresApproval: false,
    inputSchema: noArguments
  },
  {
    name: 'corevia.read_identity',
    description: 'Read this installation\'s local identity without exposing secrets.',
    operation: 'read-identity',
    capability: 'read-settings',
    profiles: ['customer-support', 'full-desk'],
    requiresApproval: false,
    inputSchema: noArguments
  },
  {
    name: 'corevia.read_devices',
    description: 'Read the locally saved devices and their live availability.',
    operation: 'read-devices',
    capability: 'read-settings',
    profiles: ['customer-support', 'full-desk'],
    requiresApproval: false,
    inputSchema: noArguments
  },
  {
    name: 'corevia.read_sessions',
    description: 'Read current local connection requests and sessions.',
    operation: 'read-sessions',
    capability: 'read-settings',
    profiles: ['customer-support', 'full-desk'],
    requiresApproval: false,
    inputSchema: noArguments
  },
  {
    name: 'corevia.write_customer_setting',
    description: 'Change one customer-side setting from the allowlist.',
    operation: 'write-customer-setting',
    capability: 'write-customer-setting',
    profiles: ['customer-support', 'full-desk'],
    requiresApproval: false,
    inputSchema: {
      type: 'object',
      properties: {
        key: { type: 'string' },
        value: scalar
      },
      required: ['key', 'value'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.write_global_setting',
    description: 'Change one Full Desk setting after the MCP client obtains your explicit approval.',
    operation: 'write-global-setting',
    capability: 'write-global-setting',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        key: { type: 'string' },
        value: scalar
      },
      required: ['key', 'value'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.approve_connection',
    description: 'Approve an incoming connection after the MCP client obtains your explicit approval.',
    operation: 'approve-connection',
    capability: 'approve-connection',
    profiles: ['customer-support', 'full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' }
      },
      required: ['connectionId'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.close_session',
    description: 'Close an incoming or active session after the MCP client obtains your explicit approval.',
    operation: 'close-session',
    capability: 'close-session',
    profiles: ['customer-support', 'full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' }
      },
      required: ['connectionId'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.change_connection_permission',
    description: 'Change one allowlisted permission on an active connection.',
    operation: 'change-connection-permission',
    capability: 'change-connection-permission',
    profiles: ['customer-support', 'full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' },
        permission: { type: 'string' },
        enabled: { type: 'boolean' }
      },
      required: ['connectionId', 'permission', 'enabled'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.start_connection',
    description: 'Start an outbound desktop connection only after the MCP client obtains your explicit approval.',
    operation: 'start-connection',
    capability: 'start-connection',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        remoteId: { type: 'string' }
      },
      required: ['remoteId'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.input_control',
    description: 'Request approved keyboard and pointer control for a session.',
    operation: 'input-control',
    capability: 'input-control',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' },
        enabled: { type: 'boolean' }
      },
      required: ['connectionId'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.use_file_transfer',
    description: 'Request the approved file-transfer capability for a session.',
    operation: 'use-file-transfer',
    capability: 'use-file-transfer',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' }
      },
      required: ['connectionId'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.use_camera',
    description: 'Request the approved camera capability for a session.',
    operation: 'use-camera',
    capability: 'use-camera',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' }
      },
      required: ['connectionId'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.use_port_forward',
    description: 'Request an approved port-forwarding capability for a session.',
    operation: 'use-port-forward',
    capability: 'use-port-forward',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' },
        localPort: { type: 'number' },
        remotePort: { type: 'number' },
        remoteHost: { type: 'string', maxLength: 255 }
      },
      required: ['connectionId', 'localPort', 'remotePort'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.use_printer',
    description: 'Request the approved printer capability for a session.',
    operation: 'use-printer',
    capability: 'use-printer',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' }
      },
      required: ['connectionId'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.record_session',
    description: 'Request an approved session-recording change.',
    operation: 'record-session',
    capability: 'record-session',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' },
        enabled: { type: 'boolean' }
      },
      required: ['connectionId', 'enabled'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.use_terminal',
    description: 'Request the approved terminal capability for a session.',
    operation: 'use-terminal',
    capability: 'use-terminal',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' }
      },
      required: ['connectionId'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.restart_remote_device',
    description: 'Request an approved restart of the remote device.',
    operation: 'restart-remote-device',
    capability: 'restart-remote-device',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' }
      },
      required: ['connectionId'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.elevate_session',
    description: 'Request approved elevation for an existing session.',
    operation: 'elevate-session',
    capability: 'elevate-session',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' }
      },
      required: ['connectionId'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.toggle_privacy_mode',
    description: 'Request an approved privacy-mode change for a session.',
    operation: 'toggle-privacy-mode',
    capability: 'toggle-privacy-mode',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' },
        enabled: { type: 'boolean' }
      },
      required: ['connectionId', 'enabled'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.manage_service',
    description: 'Request an approved local service-management action.',
    operation: 'manage-service',
    capability: 'manage-service',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        action: { type: 'string' }
      },
      required: ['action'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.manage_peers',
    description: 'Request an approved address-book or peer-management action.',
    operation: 'manage-peers',
    capability: 'manage-peers',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        peerId: { type: 'string' },
        action: { type: 'string' },
        name: { type: 'string' },
        platform: { type: 'string' },
        tag: { type: 'string' },
        password: { type: 'string' }
      },
      required: ['peerId', 'action'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.send_chat',
    description: 'Send a message through an existing approved session.',
    operation: 'send-chat',
    capability: 'send-chat',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' },
        message: { type: 'string' }
      },
      required: ['connectionId', 'message'],
      additionalProperties: false
    }
  },
  {
    name: 'corevia.use_voice_call',
    description: 'Request the approved voice-call capability for a session.',
    operation: 'use-voice-call',
    capability: 'use-voice-call',
    profiles: ['full-desk'],
    requiresApproval: true,
    inputSchema: {
      type: 'object',
      properties: {
        connectionId: { type: 'string' },
        enabled: { type: 'boolean' }
      },
      required: ['connectionId'],
      additionalProperties: false
    }
  }
]

export const PROFILE_IDS = Object.freeze(['customer-support', 'full-desk'])

const CUSTOMER_SUPPORT_CONNECTION_PERMISSIONS = Object.freeze([
  'keyboard',
  'block_input',
  'view_only'
])

export const normalizeProfile = (value) => {
  if (value === 'customer-support' || value === 'full-desk') return value
  return null
}

export const isConnectionPermissionAllowed = (profile, permission) => {
  const normalized = normalizeProfile(profile)
  if (!normalized) return false
  if (normalized === 'full-desk') return [
    'keyboard',
    'clipboard',
    'audio',
    'file',
    'restart',
    'recording',
    'block_input',
    'privacy_mode'
  ].includes(permission)
  return CUSTOMER_SUPPORT_CONNECTION_PERMISSIONS.includes(permission)
}

export const toolsForProfile = (profile) => {
  const normalized = normalizeProfile(profile)
  if (!normalized) return []
  return tools
    .filter((tool) => tool.profiles.includes(normalized))
    .map(({ profiles, operation, capability, requiresApproval, ...publicTool }) => ({
      ...publicTool,
      description: requiresApproval
        ? `${publicTool.description} Corevia Desk requests separate interactive approval from the MCP client for every call; it fails closed when form elicitation is not supported.`
        : publicTool.description
    }))
}

export const findTool = (profile, name) => {
  const normalized = normalizeProfile(profile)
  if (!normalized) return undefined
  return tools.find((tool) => tool.name === name && tool.profiles.includes(normalized))
}

export const getToolCatalog = () => tools.map((tool) => ({ ...tool }))
