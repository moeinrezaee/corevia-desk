export type CoreviaBridgeSurface = 'nuxt' | 'mcp' | 'internal'

export type CoreviaBridgeOperation =
  | 'get-status'
  | 'read-screen-capture-status'
  | 'request-screen-capture-access'
  | 'read-settings'
  | 'read-identity'
  | 'read-temporary-password'
  | 'read-devices'
  | 'read-sessions'
  | 'read-session-frame'
  | 'read-terminal'
  | 'write-customer-setting'
  | 'write-global-setting'
  | 'start-connection'
  | 'approve-connection'
  | 'close-session'
  | 'change-connection-permission'
  | 'input-control'
  | 'send-mouse'
  | 'send-key'
  | 'switch-display'
  | 'refresh-video'
  | 'set-session-quality'
  | 'set-session-scroll'
  | 'use-camera'
  | 'use-file-transfer'
  | 'use-terminal'
  | 'close-terminal'
  | 'send-terminal-input'
  | 'use-port-forward'
  | 'use-printer'
  | 'record-session'
  | 'restart-remote-device'
  | 'elevate-session'
  | 'toggle-privacy-mode'
  | 'manage-service'
  | 'manage-peers'
  | 'send-chat'
  | 'use-voice-call'
  | 'system-command'

export type CoreviaBridgeStatus = 'accepted' | 'denied' | 'unsupported' | 'error'

export type CoreviaBridgeSessionState = 'pending' | 'active' | 'closed' | 'failed'

export type CoreviaBridgeEventKind =
  | 'request-accepted'
  | 'request-denied'
  | 'session-opened'
  | 'session-closed'
  | 'permission-changed'

export interface CoreviaBridgeSession {
  sessionId: string
  remoteId: string
  state: CoreviaBridgeSessionState
}

export interface CoreviaBridgeEvent {
  eventId: string
  kind: CoreviaBridgeEventKind
  sessionId?: string
  data: Record<string, unknown>
}

export interface CoreviaBridgeRequest {
  requestId: string
  operation: CoreviaBridgeOperation
  payload?: Record<string, unknown>
  approvalReference?: string
}

export interface CoreviaBridgeError {
  code: string
  message: string
}

export interface CoreviaBridgeResponse {
  requestId: string
  operation: string
  surface: CoreviaBridgeSurface
  profile: 'customer-support' | 'full-desk'
  status: CoreviaBridgeStatus
  auditId?: number
  data?: Record<string, unknown>
  events?: CoreviaBridgeEvent[]
  error?: CoreviaBridgeError
}

export interface CoreviaDeskBridge {
  request: (request: CoreviaBridgeRequest) => Promise<CoreviaBridgeResponse>
  subscribe?: (listener: (event: CoreviaBridgeEvent) => void) => () => void
}

declare global {
  interface Window {
    __COREVIA_DESK_BRIDGE__?: CoreviaDeskBridge
  }
}

export {}
