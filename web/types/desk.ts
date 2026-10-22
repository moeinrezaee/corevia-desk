export type DeskProfile = 'customer-support' | 'full-desk'
export type DeskTheme = 'dark' | 'light'
export type DeskLocale = 'fa' | 'en'
export type DeskPage = 'home' | 'devices' | 'sessions' | 'settings'

export interface DeskDevice {
  id: string
  name: string
  platform: string
  address: string
  tag: string
  media: 'lab' | 'home' | 'server' | 'studio'
  status: 'online' | 'offline'
  latency: string
  lastSeen: string
  lastSeenAt?: number
  favorite?: boolean
}

export interface DeskIdentity {
  localId: string
  passwordConfigured: boolean
  temporaryPasswordAvailable: boolean
}

export interface DeskRuntimeStatus {
  product: string
  version: string
  profile: DeskProfile
  platform: string
  localId: string
  networkConfigured: boolean
  idServer: string
  relayServer: string
  apiServer: string
  keyConfigured: boolean
  activeSessionCount: number
  pendingSessionCount: number
  savedDeviceCount: number
}

export interface DeskSession {
  id: string
  peerId?: string
  name: string
  platform: string
  address: string
  state: 'pending' | 'active' | 'closed' | 'failed'
  authorized: boolean
  disconnected: boolean
  incoming: boolean
  permissions: Record<string, boolean>
  features: Record<string, boolean | number | string>
  displayCount?: number
  currentDisplay?: number
  direct?: boolean
  secured?: boolean
  error?: string
  endedAt?: number
  when?: string
}

export interface DeskNavItem {
  page: DeskPage
  labelKey: string
  icon: string
  to: string
  operatorOnly?: boolean
}
