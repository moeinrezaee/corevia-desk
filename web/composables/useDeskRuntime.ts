import { computed } from 'vue'
import { useCoreviaBridge } from '~/composables/useCoreviaBridge'
import { useDeskAppearance } from '~/composables/useDeskAppearance'
import { useDeskCopy } from '~/composables/useDeskCopy'
import type {
  CoreviaBridgeResponse
} from '~/types/corevia-bridge'
import type {
  DeskDevice,
  DeskIdentity,
  DeskRuntimeStatus,
  DeskSession
} from '~/types/desk'

interface DeskRuntimeSettings {
  values: Record<string, string>
  network: {
    idServer: string
    relayServer: string
    apiServer: string
    keyConfigured: boolean
  }
}

interface DeskRuntimeState {
  initialized: boolean
  loading: boolean
  source: 'rust' | 'unavailable' | 'error'
  error: string | null
  status: DeskRuntimeStatus | null
  identity: DeskIdentity | null
  devices: DeskDevice[]
  sessions: DeskSession[]
  history: DeskSession[]
  settings: DeskRuntimeSettings | null
}

const emptyState = (): DeskRuntimeState => ({
  initialized: false,
  loading: false,
  source: 'unavailable',
  error: null,
  status: null,
  identity: null,
  devices: [],
  sessions: [],
  history: [],
  settings: null
})

const asRecord = (value: unknown): Record<string, unknown> => {
  return value !== null && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {}
}

const asString = (value: unknown, fallback = '') => {
  return typeof value === 'string' ? value : fallback
}

const asBoolean = (value: unknown, fallback = false) => {
  if (typeof value === 'boolean') return value
  if (typeof value === 'string') return value !== 'N' && value !== 'false' && value !== '0'
  return fallback
}

const asNumber = (value: unknown, fallback = 0) => {
  return typeof value === 'number' && Number.isFinite(value) ? value : fallback
}

const mediaForPlatform = (platform: string): DeskDevice['media'] => {
  const value = platform.toLowerCase()
  if (value.includes('linux')) return 'home'
  if (value.includes('mac') || value.includes('darwin')) return 'studio'
  if (value.includes('server')) return 'server'
  return 'lab'
}

export const useDeskRuntime = () => {
  const runtime = useState<DeskRuntimeState>('corevia-desk-runtime', emptyState)
  const { locale, setLocale, setTheme } = useDeskAppearance()
  const { t } = useDeskCopy()
  const { hasLocalBridge, request } = useCoreviaBridge()
  let sessionsRefreshInFlight = false

  const formatLastSeen = (value: unknown) => {
    const timestamp = asNumber(value)
    if (!timestamp) return t('notAvailable')
    return new Intl.DateTimeFormat(locale.value === 'fa' ? 'fa-IR' : 'en-US', {
      dateStyle: 'medium',
      timeStyle: 'short'
    }).format(new Date(timestamp * 1000))
  }

  const mapDevice = (value: unknown): DeskDevice & { favorite: boolean } => {
    const item = asRecord(value)
    const platform = asString(item.platform, t('notAvailable'))
    const address = asString(item.address, asString(item.id, ''))
    return {
      id: asString(item.id, address),
      name: asString(item.name, address || t('notAvailable')),
      platform,
      address,
      tag: asString(item.tag),
      media: mediaForPlatform(platform),
      status: item.status === 'online' ? 'online' : 'offline',
      latency: typeof item.latency === 'number' ? `${item.latency} ms` : asString(item.latency, '—'),
      lastSeen: formatLastSeen(item.lastSeenAt),
      lastSeenAt: asNumber(item.lastSeenAt),
      favorite: asBoolean(item.favorite)
    }
  }

  const mapSession = (value: unknown): DeskSession => {
    const item = asRecord(value)
    const state = ['pending', 'active', 'closed', 'failed'].includes(asString(item.state))
      ? asString(item.state) as DeskSession['state']
      : 'failed'
    const permissions = asRecord(item.permissions)
    const features = asRecord(item.features)
    const endedAt = asNumber(item.endedAt)
    return {
      id: asString(item.sessionId, asString(item.id)),
      peerId: asString(item.remoteId, asString(item.address)),
      name: asString(item.name, asString(item.remoteId, t('notAvailable'))),
      platform: asString(item.platform, t('notAvailable')),
      address: asString(item.address, asString(item.remoteId)),
      state,
      authorized: asBoolean(item.authorized),
      disconnected: asBoolean(item.disconnected),
      incoming: typeof item.incoming === 'boolean' ? item.incoming : true,
      permissions: Object.fromEntries(Object.entries(permissions).map(([key, value]) => [key, asBoolean(value)])),
      features: Object.fromEntries(Object.entries(features).map(([key, value]) => [
        key,
        typeof value === 'string' || typeof value === 'number' ? value : asBoolean(value)
      ])),
      displayCount: asNumber(item.display_count),
      currentDisplay: asNumber(item.current_display),
      direct: typeof item.direct === 'boolean' ? item.direct : undefined,
      secured: typeof item.secured === 'boolean' ? item.secured : undefined,
      error: typeof item.error === 'string' ? item.error : undefined,
      endedAt: endedAt || undefined,
      when: endedAt ? formatLastSeen(endedAt) : undefined
    }
  }

  const read = async (operation: Parameters<typeof request>[0]): Promise<Record<string, unknown> | null> => {
    const response: CoreviaBridgeResponse = await request(operation)
    if (response.status !== 'accepted') return null
    return asRecord(response.data)
  }

  const refresh = async (force = false) => {
    if (!import.meta.client || runtime.value.loading || (runtime.value.initialized && !force)) return
    runtime.value.initialized = true
    runtime.value.loading = true
    runtime.value.error = null

    if (!hasLocalBridge.value) {
      runtime.value.source = 'unavailable'
      runtime.value.error = t('runtimeUnavailable')
      runtime.value.loading = false
      runtime.value.status = null
      runtime.value.identity = null
      runtime.value.devices = []
      runtime.value.sessions = []
      runtime.value.history = []
      runtime.value.settings = null
      return
    }

    const [status, identity, devices, sessions, settings] = await Promise.all([
      read('get-status'),
      read('read-identity'),
      read('read-devices'),
      read('read-sessions'),
      read('read-settings')
    ])

    const failed = [status, identity, devices, sessions, settings].some((value) => value === null)
    runtime.value.source = failed ? 'error' : 'rust'
    if (failed) runtime.value.error = t('runtimeReadError')

    if (status) {
      runtime.value.status = {
        product: asString(status.product),
        version: asString(status.version),
        profile: asString(status.profile) === 'customer-support' ? 'customer-support' : 'full-desk',
        platform: asString(status.platform),
        localId: asString(status.localId),
        networkConfigured: asBoolean(status.networkConfigured),
        idServer: asString(status.idServer),
        relayServer: asString(status.relayServer),
        apiServer: asString(status.apiServer),
        keyConfigured: asBoolean(status.keyConfigured),
        activeSessionCount: asNumber(status.activeSessionCount),
        pendingSessionCount: asNumber(status.pendingSessionCount),
        savedDeviceCount: asNumber(status.savedDeviceCount)
      }
    }
    if (identity) {
      runtime.value.identity = {
        localId: asString(identity.localId),
        passwordConfigured: asBoolean(identity.passwordConfigured),
        temporaryPasswordAvailable: asBoolean(identity.temporaryPasswordAvailable)
      }
    }
    if (devices) {
      const values = Array.isArray(devices.devices) ? devices.devices : []
      runtime.value.devices = values.map(mapDevice)
    }
    if (sessions) {
      const active = Array.isArray(sessions.active) ? sessions.active : []
      const historyValues = Array.isArray(sessions.history) ? sessions.history : []
      runtime.value.sessions = active.map(mapSession)
      runtime.value.history = historyValues.map(mapSession)
    }
    if (settings) {
      const values = asRecord(settings.values)
      const network = asRecord(settings.network)
      runtime.value.settings = {
        values: Object.fromEntries(Object.entries(values).map(([key, value]) => [key, asString(value)])),
        network: {
          idServer: asString(network.idServer),
          relayServer: asString(network.relayServer),
          apiServer: asString(network.apiServer),
          keyConfigured: asBoolean(network.keyConfigured)
        }
      }

      // Apply persisted appearance for every entry route, not only Settings.
      // Otherwise Home first renders the defaults and stays out of sync until
      // the user opens Settings.
      const savedTheme = runtime.value.settings.values.theme
      if (savedTheme === 'dark' || savedTheme === 'light') setTheme(savedTheme)
      const savedLocale = runtime.value.settings.values.lang
      if (savedLocale === 'fa' || savedLocale === 'en') setLocale(savedLocale)
    }
    runtime.value.loading = false
  }

  const refreshSessions = async () => {
    if (
      !import.meta.client ||
      !hasLocalBridge.value ||
      runtime.value.loading ||
      sessionsRefreshInFlight
    ) return

    sessionsRefreshInFlight = true
    try {
      const snapshot = await read('read-sessions')
      if (!snapshot) return

      const active = Array.isArray(snapshot.active) ? snapshot.active : []
      const historyValues = Array.isArray(snapshot.history) ? snapshot.history : []
      runtime.value.sessions = active.map(mapSession)
      runtime.value.history = historyValues.map(mapSession)
    } finally {
      sessionsRefreshInFlight = false
    }
  }

  // Pending sessions are still live connection attempts and must remain
  // visible until Rust reports a terminal state. Failed/closed sessions are
  // presented through the history list.
  const activeSessions = computed(() => runtime.value.sessions.filter((session) => session.state === 'active' || session.state === 'pending'))
  const pendingSessions = computed(() => runtime.value.sessions.filter((session) => session.state === 'pending'))

  return {
    activeSessions,
    devices: computed(() => runtime.value.devices),
    error: computed(() => runtime.value.error),
    hasLocalBridge,
    history: computed(() => runtime.value.history),
    identity: computed(() => runtime.value.identity),
    loading: computed(() => runtime.value.loading),
    pendingSessions,
    refresh,
    refreshSessions,
    sessions: computed(() => runtime.value.sessions),
    settings: computed(() => runtime.value.settings),
    source: computed(() => runtime.value.source),
    status: computed(() => runtime.value.status)
  }
}
