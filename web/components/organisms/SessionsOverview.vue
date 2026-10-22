<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useDeskAppearance } from '~/composables/useDeskAppearance'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useDeskBridgeFeedback } from '~/composables/useDeskBridgeFeedback'
import { useCoreviaBridge } from '~/composables/useCoreviaBridge'
import { useDeskRuntime } from '~/composables/useDeskRuntime'
import { latestSessionPerDevice, sessionDeviceKey } from '~/utils/session-history.mjs'
import type { DeskSession } from '~/types/desk'
import labDark from '~/assets/media/desktop-lab.svg'
import labLight from '~/assets/media/desktop-lab-light.svg'
import studioDark from '~/assets/media/desktop-studio.svg'
import studioLight from '~/assets/media/desktop-studio-light.svg'
import homeDark from '~/assets/media/desktop-home.svg'
import homeLight from '~/assets/media/desktop-home-light.svg'
import serverDark from '~/assets/media/desktop-server.svg'
import serverLight from '~/assets/media/desktop-server-light.svg'

const { theme } = useDeskAppearance()
const { t } = useDeskCopy()
const { isBridgeActionSuccessful, notifyFailure } = useDeskBridgeFeedback()
const { hasLocalBridge, request } = useCoreviaBridge()
const { activeSessions, history, refresh } = useDeskRuntime()
const activeSessionId = ref('')
const closedTabs = ref<string[]>([])
const toolboxOpen = ref(true)
const showConnect = ref(false)
const toolState = ref<Record<string, boolean>>({})
const isSessionFullscreen = ref(false)
const selectedDisplay = ref(0)
const qualityIndex = ref(0)
const scrollIndex = ref(0)
const liveFrameUrl = ref('')
const liveFrameSequence = ref(0)
const liveFrameWidth = ref(0)
const liveFrameHeight = ref(0)
const terminalPanelOpen = ref(false)
const terminalStatus = ref('closed')
const terminalOutput = ref('')
const terminalError = ref('')
const terminalInput = ref('')

let refreshTimer: ReturnType<typeof setInterval> | undefined
let frameTimer: ReturnType<typeof setInterval> | undefined
let terminalTimer: ReturnType<typeof setInterval> | undefined
let lastPointerSentAt = 0
let lastInputFailureNotifiedAt = 0

onMounted(() => {
  void refresh()
  refreshTimer = setInterval(() => {
    void refresh(true)
  }, 1500)
  void refreshLiveFrame()
  frameTimer = setInterval(() => {
    void refreshLiveFrame()
  }, 200)
  terminalTimer = setInterval(() => {
    void refreshTerminal()
  }, 1000)
})

onBeforeUnmount(() => {
  if (refreshTimer) clearInterval(refreshTimer)
  if (frameTimer) clearInterval(frameTimer)
  if (terminalTimer) clearInterval(terminalTimer)
  if (import.meta.client) document.body.classList.remove('session-is-fullscreen')
})

const sessions = computed(() => activeSessions.value.filter((session) => !closedTabs.value.includes(session.id)))
const activeSession = computed(() => sessions.value.find((session) => session.id === activeSessionId.value) ?? sessions.value[0])

const sessionPresentation = (session: DeskSession) => {
  const platform = session.platform.toLowerCase()
  if (platform.includes('mac')) {
    return {
      ...session,
      visualClass: 'os-visual-macos',
      osIcon: 'fa-brands fa-apple',
      avatarClass: 'device-avatar-purple',
      avatarIcon: 'fa-solid fa-laptop',
      darkImage: studioDark,
      lightImage: studioLight
    }
  }
  if (platform.includes('linux')) {
    return {
      ...session,
      visualClass: 'os-visual-linux',
      osIcon: 'fa-brands fa-linux',
      avatarClass: 'device-avatar-cyan',
      avatarIcon: 'fa-solid fa-desktop',
      darkImage: platform.includes('server') ? serverDark : homeDark,
      lightImage: platform.includes('server') ? serverLight : homeLight
    }
  }
  return {
    ...session,
    visualClass: 'os-visual-windows',
    osIcon: 'fa-brands fa-windows',
    avatarClass: 'device-avatar-green',
    avatarIcon: 'fa-solid fa-server',
    darkImage: labDark,
    lightImage: labLight
  }
}

const visibleTabs = computed(() => sessions.value.map(sessionPresentation))
const activePresentation = computed(() => activeSession.value ? sessionPresentation(activeSession.value) : undefined)
const activeImage = computed(() => {
  const presentation = activePresentation.value
  return presentation
    ? (theme.value === 'dark' ? presentation.darkImage : presentation.lightImage)
    : ''
})
const historyPresentations = computed(() => latestSessionPerDevice(history.value).map(sessionPresentation))
const recentConnectionsCount = computed(() => new Set(
  [...sessions.value, ...history.value].map(sessionDeviceKey)
).size)

const historyStateLabel = (session: DeskSession) => {
  if (session.state === 'failed') return t('connectionFailedShort')
  return t('ended')
}

const sessionTransportKey = (session: DeskSession) => {
  if (session.direct === true) return 'direct'
  if (session.direct === false) return 'relayed'
  return 'negotiating'
}

const sessionTransportLabel = (session: DeskSession) => {
  if (session.direct === true) return t('directConnection')
  if (session.direct === false) return t('relayedConnection')
  return t('transportNegotiating')
}

const sessionSecurityLabel = (session: DeskSession) => {
  if (session.secured === true) return t('encrypted')
  if (session.secured === false) return t('notEncrypted')
  return t('securityNegotiating')
}

const sessionSecurityKey = (session: DeskSession) => {
  if (session.secured === true) return 'secured'
  if (session.secured === false) return 'unsecured'
  return 'negotiating'
}

const refreshLiveFrame = async () => {
  const session = activeSession.value
  if (!hasLocalBridge.value || !session || session.state !== 'active') {
    liveFrameUrl.value = ''
    liveFrameSequence.value = 0
    liveFrameWidth.value = 0
    liveFrameHeight.value = 0
    return
  }

  const response = await request('read-session-frame', { connectionId: session.id })
  const data = response.data ?? {}
  const available = response.status === 'accepted' && data.available === true
  const frameUrl = typeof data.frameUrl === 'string' ? data.frameUrl : ''
  const sequence = typeof data.sequence === 'number' ? data.sequence : 0
  if (!available || !frameUrl) {
    return
  }
  if (sequence !== liveFrameSequence.value) {
    liveFrameSequence.value = sequence
    liveFrameWidth.value = typeof data.width === 'number' ? data.width : 0
    liveFrameHeight.value = typeof data.height === 'number' ? data.height : 0
    liveFrameUrl.value = frameUrl
  }
}

const refreshTerminal = async () => {
  const session = activeSession.value
  if (!terminalPanelOpen.value || !hasLocalBridge.value || !session || session.state !== 'active') return

  const response = await request('read-terminal', { connectionId: session.id })
  if (response.status !== 'accepted') return

  const data = response.data ?? {}
  terminalStatus.value = typeof data.status === 'string' ? data.status : 'unavailable'
  terminalOutput.value = typeof data.output === 'string'
    ? data.output
      .replace(/\u001b\[[0-?]*[ -/]*[@-~]/g, '')
      .replace(/[\u0000-\u0008\u000b\u000c\u000e-\u001f\u007f]/g, '')
    : ''
  terminalError.value = typeof data.error === 'string' ? data.error : ''
  if (terminalStatus.value === 'closed') terminalPanelOpen.value = false
}

const terminalStatusLabel = () => {
  if (terminalStatus.value === 'opening') return t('terminalOpening')
  if (terminalStatus.value === 'open') return t('terminalOpened')
  if (terminalStatus.value === 'closing') return t('terminalClosing')
  if (terminalStatus.value === 'error') return t('terminalFailed')
  if (terminalStatus.value === 'unavailable') return t('terminalUnavailable')
  return t('terminalClosed')
}

const toggleTerminal = async () => {
  const session = activeSession.value
  if (!session || session.state !== 'active') return

  const opening = !['opening', 'open', 'closing'].includes(terminalStatus.value)
  const response = await request(
    opening ? 'use-terminal' : 'close-terminal',
    { connectionId: session.id },
    opening ? 'nuxt-terminal-open-approval' : 'nuxt-terminal-close-approval'
  )
  if (response.status !== 'accepted' || response.data?.executed !== true) {
    notifyFailure(response)
    return
  }

  terminalPanelOpen.value = true
  terminalStatus.value = opening ? 'opening' : 'closing'
  terminalError.value = ''
  if (opening) void refreshTerminal()
}

const closeTerminalPanel = async () => {
  if (['opening', 'open'].includes(terminalStatus.value)) {
    await toggleTerminal()
    return
  }
  terminalPanelOpen.value = false
}

const sendTerminalInput = async () => {
  const session = activeSession.value
  const input = terminalInput.value
  if (!session || session.state !== 'active' || terminalStatus.value !== 'open' || !input.trim()) return

  const response = await request(
    'send-terminal-input',
    { connectionId: session.id, input: `${input}\n` },
    'nuxt-terminal-input-approval'
  )
  if (response.status === 'accepted' && response.data?.executed === true) {
    terminalInput.value = ''
  } else {
    notifyFailure(response)
  }
}

watch(() => activeSession.value?.id, () => {
  selectedDisplay.value = activeSession.value?.currentDisplay ?? 0
  liveFrameUrl.value = ''
  liveFrameSequence.value = 0
  liveFrameWidth.value = 0
  liveFrameHeight.value = 0
  terminalPanelOpen.value = false
  terminalStatus.value = 'closed'
  terminalOutput.value = ''
  terminalError.value = ''
  void refreshLiveFrame()
})

watch(isSessionFullscreen, (enabled) => {
  if (import.meta.client) document.body.classList.toggle('session-is-fullscreen', enabled)
})

const remotePoint = (event: PointerEvent) => {
  const image = event.currentTarget as HTMLImageElement
  const rect = image.getBoundingClientRect()
  const width = liveFrameWidth.value || image.naturalWidth
  const height = liveFrameHeight.value || image.naturalHeight
  if (!width || !height || !rect.width || !rect.height) return null

  const scale = Math.min(rect.width / width, rect.height / height)
  const renderedWidth = width * scale
  const renderedHeight = height * scale
  const offsetX = (rect.width - renderedWidth) / 2
  const offsetY = (rect.height - renderedHeight) / 2
  const x = Math.round(Math.min(width - 1, Math.max(0, (event.clientX - rect.left - offsetX) / scale)))
  const y = Math.round(Math.min(height - 1, Math.max(0, (event.clientY - rect.top - offsetY) / scale)))
  return { x, y }
}

const reportInputFailure = (response: Awaited<ReturnType<typeof request>>) => {
  if (isBridgeActionSuccessful(response)) return
  const now = Date.now()
  if (now - lastInputFailureNotifiedAt < 2000) return
  lastInputFailureNotifiedAt = now
  notifyFailure(response)
}

const sendMouse = (event: PointerEvent, type: 'move' | 'down' | 'up') => {
  const session = activeSession.value
  if (!session || session.state !== 'active' || toolState.value.view) return
  if (type === 'move' && event.timeStamp - lastPointerSentAt < 30) return
  const point = remotePoint(event)
  if (!point) return

  lastPointerSentAt = event.timeStamp
  const button = type === 'move' ? 0 : event.button === 2 ? 2 : event.button === 1 ? 4 : 1
  const eventType = type === 'down' ? 1 : type === 'up' ? 2 : 0
  void request(
    'send-mouse',
    {
      connectionId: session.id,
      mask: (button << 3) | eventType,
      x: point.x,
      y: point.y,
      alt: event.altKey,
      ctrl: event.ctrlKey,
      shift: event.shiftKey,
      command: event.metaKey
    },
    'nuxt-session-input-approval'
  ).then(reportInputFailure)
  if (type !== 'move') event.preventDefault()
}

const remoteKeyName = (key: string) => {
  const aliases: Record<string, string> = {
    Enter: 'VK_ENTER',
    Escape: 'VK_ESCAPE',
    Backspace: 'VK_BACK',
    Tab: 'VK_TAB',
    ' ': 'VK_SPACE',
    ArrowLeft: 'VK_LEFT',
    ArrowUp: 'VK_UP',
    ArrowRight: 'VK_RIGHT',
    ArrowDown: 'VK_DOWN',
    Delete: 'VK_DELETE',
    Insert: 'VK_INSERT',
    Home: 'VK_HOME',
    End: 'VK_END',
    PageUp: 'VK_PRIOR',
    PageDown: 'VK_NEXT',
    Shift: 'VK_SHIFT',
    Control: 'VK_CONTROL',
    Alt: 'VK_MENU',
    Meta: 'Meta',
    CapsLock: 'VK_CAPITAL',
    F1: 'VK_F1',
    F2: 'VK_F2',
    F3: 'VK_F3',
    F4: 'VK_F4',
    F5: 'VK_F5',
    F6: 'VK_F6',
    F7: 'VK_F7',
    F8: 'VK_F8',
    F9: 'VK_F9',
    F10: 'VK_F10',
    F11: 'VK_F11',
    F12: 'VK_F12'
  }
  return aliases[key] ?? key
}

const sendKey = (event: KeyboardEvent, down: boolean) => {
  const session = activeSession.value
  if (!session || session.state !== 'active' || toolState.value.view) return
  const name = remoteKeyName(event.key)
  if (!name || name === 'Dead' || name === 'Unidentified') return
  void request(
    'send-key',
    {
      connectionId: session.id,
      name,
      down,
      press: false,
      alt: event.altKey,
      ctrl: event.ctrlKey,
      shift: event.shiftKey,
      command: event.metaKey
    },
    'nuxt-session-input-approval'
  ).then(reportInputFailure)
  event.preventDefault()
}

const qualityChoices = ['balanced', 'best', 'low'] as const
const qualityDetailKeys = ['balanced', 'bestQuality', 'speed'] as const
const scrollChoices = ['auto', 'reverse', 'edge'] as const
const scrollDetailKeys = ['scrollAuto', 'scrollReverse', 'scrollEdge'] as const

const useDisplayTool = async (id: string) => {
  const session = activeSession.value
  if (!session || session.state !== 'active') return

  if (id === 'monitor') {
    const count = Math.max(1, session.displayCount ?? 1)
    const nextDisplay = (selectedDisplay.value + 1) % count
    const response = await request(
      'switch-display',
      { connectionId: session.id, display: nextDisplay },
      'nuxt-session-display-approval'
    )
    if (isBridgeActionSuccessful(response)) {
      selectedDisplay.value = nextDisplay
      await refresh(true)
    } else {
      notifyFailure(response)
    }
    return
  }

  if (id === 'resolution') {
    const response = await request('refresh-video', { connectionId: session.id }, 'nuxt-session-resolution-approval')
    if (!isBridgeActionSuccessful(response)) notifyFailure(response)
    return
  }

  if (id === 'quality') {
    const nextQuality = (qualityIndex.value + 1) % qualityChoices.length
    const response = await request(
      'set-session-quality',
      { connectionId: session.id, quality: qualityChoices[nextQuality] },
      'nuxt-session-quality-approval'
    )
    if (isBridgeActionSuccessful(response)) {
      qualityIndex.value = nextQuality
    } else {
      notifyFailure(response)
    }
    return
  }

  if (id === 'scroll') {
    const nextScroll = (scrollIndex.value + 1) % scrollChoices.length
    const response = await request(
      'set-session-scroll',
      { connectionId: session.id, style: scrollChoices[nextScroll] },
      'nuxt-session-scroll-approval'
    )
    if (isBridgeActionSuccessful(response)) {
      scrollIndex.value = nextScroll
    } else {
      notifyFailure(response)
    }
  }
}

const displayToolDetail = (id: string) => {
  if (id === 'monitor') return `monitor ${selectedDisplay.value + 1}`
  if (id === 'quality') return qualityDetailKeys[qualityIndex.value]
  if (id === 'scroll') return scrollDetailKeys[scrollIndex.value]
  return 'adaptive'
}

watch(activeSession, (session) => {
  const permissions = session?.permissions ?? {}
  toolState.value = {
    cursor: Boolean(permissions.show_remote_cursor),
    view: Boolean(permissions.view_only),
    keyboard: Boolean(permissions.keyboard),
    clipboard: Boolean(permissions.clipboard),
    files: Boolean(permissions.file),
    audio: Boolean(permissions.audio),
    terminal: Boolean(session?.features.terminal)
  }
}, { immediate: true })

const displayTools = [
  { id: 'monitor', icon: 'fa-solid fa-display', label: 'selectMonitor' },
  { id: 'resolution', icon: 'fa-solid fa-up-right-and-down-left-from-center', label: 'resolution' },
  { id: 'quality', icon: 'fa-solid fa-gauge-high', label: 'imageQuality' },
  { id: 'scroll', icon: 'fa-solid fa-arrows-up-down', label: 'scrollStyle' }
]

const sessionTools = [
  { id: 'cursor', icon: 'fa-solid fa-arrow-pointer', label: 'showRemoteCursor' },
  { id: 'view', icon: 'fa-solid fa-eye', label: 'viewOnly' },
  { id: 'keyboard', icon: 'fa-solid fa-keyboard', label: 'keyboardMouse' },
  { id: 'clipboard', icon: 'fa-regular fa-clipboard', label: 'clipboard' },
  { id: 'files', icon: 'fa-regular fa-folder-open', label: 'fileTransfer' },
  { id: 'audio', icon: 'fa-solid fa-volume-high', label: 'audio' },
  { id: 'terminal', icon: 'fa-solid fa-terminal', label: 'terminal' }
]

const setActiveSession = (id: string) => {
  activeSessionId.value = id
}

const closeTab = async (id: string) => {
  const response = await request(
    'close-session',
    { connectionId: id },
    'nuxt-session-close-approval'
  )
  if (!isBridgeActionSuccessful(response)) {
    notifyFailure(response)
    return
  }

  closedTabs.value = [...closedTabs.value, id]
  if (activeSessionId.value === id) {
    activeSessionId.value = sessions.value.find((session) => session.id !== id)?.id ?? ''
  }
  await refresh(true)
}

const toggleTool = async (id: string) => {
  if (!activeSession.value) return
  const permissionMap: Record<string, string> = {
    cursor: 'show_remote_cursor',
    keyboard: 'keyboard',
    clipboard: 'clipboard',
    audio: 'audio',
    files: 'file',
    view: 'view_only'
  }
  if (id === 'terminal') {
    await toggleTerminal()
    return
  }
  const permission = permissionMap[id]
  if (!permission) return
  const enabled = !toolState.value[id]
  const response = await request(
    'change-connection-permission',
    { connectionId: activeSession.value.id, permission, enabled },
    'nuxt-session-permission-approval'
  )
  if (response.status === 'accepted' && response.data?.executed === true) {
    await refresh(true)
  } else {
    notifyFailure(response)
  }
}

const endSession = async () => {
  if (!activeSession.value) return
  const response = await request(
    'close-session',
    { connectionId: activeSession.value.id },
    'nuxt-session-close-approval'
  )
  if (isBridgeActionSuccessful(response)) {
    await refresh(true)
  } else {
    notifyFailure(response)
  }
}

const approveSession = async () => {
  const session = activeSession.value
  if (!session || session.state !== 'pending' || !session.incoming) return
  const response = await request(
    'approve-connection',
    { connectionId: session.id },
    'nuxt-incoming-connection-approval'
  )
  if (isBridgeActionSuccessful(response)) {
    await refresh(true)
  } else {
    notifyFailure(response)
  }
}

const requestElevation = async () => {
  if (!activeSession.value) return
  const response = await request(
    'elevate-session',
    { connectionId: activeSession.value.id },
    'nuxt-session-elevation-approval'
  )
  if (!isBridgeActionSuccessful(response)) notifyFailure(response)
}

const openSession = () => {
  toolboxOpen.value = true
  isSessionFullscreen.value = false
}
</script>

<template>
  <section class="view-panel corevia-sessions-page corevia-page-stack">
    <div class="screen-header">
      <div>
        <span class="eyebrow"><span class="eyebrow-dot" />COREVIA DESK / {{ t('sessions') }}</span>
        <h1>{{ t('sessionsTitle') }}</h1>
        <p>{{ t('sessionsDescription') }}</p>
      </div>
      <button class="button button-primary" type="button" @click="showConnect = true">
        <i class="fa-solid fa-plug" aria-hidden="true" />
        <span>{{ t('newConnection') }}</span>
      </button>
    </div>

    <div class="sessions-layout session-workspace">
      <article class="glass-card active-session-card" :class="{ 'is-session-fullscreen': isSessionFullscreen }">
        <div class="card-heading">
          <div>
            <span class="eyebrow eyebrow-muted">{{ t('activeNow') }}</span>
            <h2>{{ t('activeSessionTitle') }}</h2>
          </div>
          <span v-if="activePresentation" class="status-pill status-pill-small"><span class="status-dot" /><span>{{ activePresentation.state === 'pending' ? (activePresentation.incoming ? t('incomingRequest') : t('waitingForRemoteApproval')) : t('online') }}</span></span>
          <span v-else class="status-pill status-pill-neutral status-pill-small">{{ t('noActiveSessions') }}</span>
        </div>

        <template v-if="activePresentation">
        <div class="session-tab-strip" role="tablist" :aria-label="t('connectionTabs')">
          <div
            v-for="session in visibleTabs"
            :key="session.id"
            class="session-tab"
            :class="{ 'is-active': activeSession?.id === session.id }"
            role="tab"
            tabindex="0"
            :aria-selected="activeSession?.id === session.id"
            @click="setActiveSession(session.id)"
            @keydown.enter="setActiveSession(session.id)"
          >
            <span class="session-tab-os" :class="`session-tab-os-${session.visualClass.includes('macos') ? 'macos' : session.visualClass.includes('linux') ? 'linux' : 'windows'}`">
              <i :class="session.osIcon" aria-hidden="true" />
            </span>
            <span class="session-tab-copy">
              <strong class="device-name" dir="ltr">{{ session.name }}</strong>
              <small dir="ltr">{{ session.platform }} · {{ session.address }}</small>
              <small class="session-tab-route" :data-connection-transport="sessionTransportKey(session)">{{ sessionTransportLabel(session) }}</small>
            </span>
            <span class="session-tab-live" aria-hidden="true"><span class="status-dot" /></span>
            <button
              class="session-tab-close"
              type="button"
              :aria-label="`${t('closeSession')} ${session.name}`"
              :title="`${t('closeSession')} ${session.name}`"
              @click.stop="closeTab(session.id)"
            >
              <i class="fa-solid fa-xmark" aria-hidden="true" />
            </button>
          </div>
          <button class="session-tab-add" type="button" :aria-label="t('openConnectionTab')" :title="t('openConnectionTab')" @click="showConnect = true">
            <i class="fa-solid fa-plus" aria-hidden="true" />
            <span class="sr-only">{{ t('openConnectionTab') }}</span>
          </button>
        </div>

        <div class="active-session-preview session-stage os-visual" :class="[activePresentation.visualClass, { 'has-live-frame': Boolean(liveFrameUrl) }]" data-visual-mode="os">
          <img class="active-session-preview-image os-visual-fallback-image" :class="{ 'is-live': Boolean(liveFrameUrl) }" :src="liveFrameUrl || activeImage" alt="" tabindex="0" @error="liveFrameUrl = ''" @pointermove="sendMouse($event, 'move')" @pointerdown="sendMouse($event, 'down')" @pointerup="sendMouse($event, 'up')" @contextmenu.prevent @keydown="sendKey($event, true)" @keyup="sendKey($event, false)" />
          <span class="os-visual-glow" aria-hidden="true" />
          <span class="os-visual-icon" aria-hidden="true"><i :class="activePresentation.osIcon" /></span>
          <span class="os-visual-label">{{ activePresentation.platform }}</span>
          <span class="active-session-preview-sheen" aria-hidden="true" />

          <div class="session-toolbox" :class="{ 'is-open': toolboxOpen, 'is-collapsed': !toolboxOpen }">
            <button
              class="toolbox-toggle toolbox-toggle-edge"
              type="button"
              :aria-expanded="toolboxOpen"
              :aria-label="toolboxOpen ? t('collapseToolbar') : t('expandToolbar')"
              :title="toolboxOpen ? t('collapseToolbar') : t('expandToolbar')"
              @click="toolboxOpen = !toolboxOpen"
            >
              <i :class="toolboxOpen ? 'fa-solid fa-chevron-left' : 'fa-solid fa-chevron-right'" aria-hidden="true" />
            </button>
            <button
              class="toolbox-toggle toolbox-toggle-rail"
              type="button"
              :aria-expanded="toolboxOpen"
              :aria-label="toolboxOpen ? t('collapseToolbar') : t('expandToolbar')"
              :title="toolboxOpen ? t('collapseToolbar') : t('expandToolbar')"
              @click="toolboxOpen = !toolboxOpen"
            >
              <i :class="toolboxOpen ? 'fa-solid fa-chevron-right' : 'fa-solid fa-chevron-left'" aria-hidden="true" />
            </button>

            <div class="toolbox-content">
              <div class="toolbox-heading">
                <span class="toolbox-heading-icon"><i class="fa-solid fa-sliders" aria-hidden="true" /></span>
                <span class="toolbox-heading-copy"><strong>{{ t('connectionTools') }}</strong><small>{{ t('remoteControl') }}</small></span>
              </div>

              <div class="toolbox-group">
                <span class="toolbox-group-label">{{ t('displaySettings') }}</span>
                <button v-for="tool in displayTools" :key="tool.id" class="toolbox-row" type="button" @click="useDisplayTool(tool.id)">
                  <i :class="tool.icon" aria-hidden="true" />
                  <span><strong>{{ t(tool.label) }}</strong><small>{{ tool.id === 'monitor' ? displayToolDetail(tool.id) : t(displayToolDetail(tool.id)) }}</small></span>
                  <i class="fa-solid fa-chevron-left toolbox-row-arrow" aria-hidden="true" />
                </button>
                <button class="toolbox-row toolbox-toggle-row" type="button" :aria-pressed="toolState.cursor" @click="toggleTool('cursor')">
                  <i class="fa-solid fa-arrow-pointer" aria-hidden="true" /><span><strong>{{ t('showRemoteCursor') }}</strong></span><span class="tool-status-dot" :class="{ 'is-enabled': toolState.cursor }" />
                </button>
                <button class="toolbox-row toolbox-toggle-row" type="button" :aria-pressed="toolState.view" @click="toggleTool('view')">
                  <i class="fa-solid fa-eye" aria-hidden="true" /><span><strong>{{ t('viewOnly') }}</strong></span><span class="tool-status-dot" :class="{ 'is-enabled': toolState.view }" />
                </button>
              </div>

              <div class="toolbox-group">
                <span class="toolbox-group-label">{{ t('controlActions') }}</span>
                <button v-for="tool in sessionTools.slice(2)" :key="tool.id" class="toolbox-row toolbox-toggle-row" type="button" :disabled="tool.id === 'terminal' && activeSession.features.terminalSupported !== true" :aria-pressed="tool.id === 'terminal' ? ['opening', 'open', 'closing'].includes(terminalStatus) : toolState[tool.id]" :title="tool.id === 'terminal' && activeSession.features.terminalSupported !== true ? t('terminalUnavailable') : undefined" @click="toggleTool(tool.id)">
                  <i :class="tool.icon" aria-hidden="true" /><span><strong>{{ t(tool.label) }}</strong><small v-if="tool.id === 'terminal' && activeSession.features.terminalSupported !== true">{{ t('terminalUnavailable') }}</small></span><span class="tool-status-dot" :class="{ 'is-enabled': tool.id === 'terminal' ? ['opening', 'open', 'closing'].includes(terminalStatus) : toolState[tool.id] }" />
                </button>
              </div>

              <div class="toolbox-footer">
                <button class="toolbox-footer-action" type="button" @click="isSessionFullscreen = !isSessionFullscreen"><i class="fa-solid fa-expand" aria-hidden="true" /><span>{{ t('fullscreen') }}</span></button>
                <button class="toolbox-footer-action" type="button" @click="requestElevation"><i class="fa-solid fa-arrow-up" aria-hidden="true" /><span>{{ t('requestElevation') }}</span></button>
                <button class="toolbox-footer-action toolbox-footer-danger" type="button" @click="endSession"><i class="fa-solid fa-power-off" aria-hidden="true" /><span>{{ t('endSession') }}</span></button>
              </div>
            </div>
          </div>
        </div>

        <section v-if="terminalPanelOpen" class="terminal-tool-panel" :aria-label="t('terminal')">
          <header class="terminal-tool-heading">
            <strong>{{ t('terminal') }}</strong>
            <span>{{ terminalStatusLabel() }}</span>
            <button type="button" :aria-label="t('terminalClose')" @click="closeTerminalPanel">
              <i class="fa-solid fa-xmark" aria-hidden="true" />
            </button>
          </header>
          <pre class="terminal-tool-output" aria-live="polite">{{ terminalOutput || terminalError || t('terminalNoOutput') }}</pre>
          <form class="terminal-tool-input" @submit.prevent="sendTerminalInput">
            <input v-model="terminalInput" type="text" :placeholder="t('terminalInputPlaceholder')" :disabled="terminalStatus !== 'open'" autocomplete="off" spellcheck="false" />
            <button type="submit" :disabled="terminalStatus !== 'open' || !terminalInput.trim()">{{ t('terminalSend') }}</button>
          </form>
        </section>

        <div class="device-row">
          <span class="device-avatar" :class="activePresentation.avatarClass"><i :class="activePresentation.avatarIcon" aria-hidden="true" /></span>
          <span class="device-row-copy"><strong class="device-name" dir="ltr">{{ activePresentation.name }}</strong><small dir="ltr">{{ activePresentation.platform }} · {{ activePresentation.address }}</small></span>
          <span
            class="device-meta"
            :data-connection-transport="sessionTransportKey(activeSession)"
            :data-connection-security="sessionSecurityKey(activeSession)"
          >
            <i :class="activeSession.direct === false ? 'fa-solid fa-route' : activeSession.direct === true ? 'fa-solid fa-arrows-left-right' : 'fa-solid fa-arrows-rotate'" aria-hidden="true" />
            <span>{{ sessionTransportLabel(activeSession) }}</span>
            <span aria-hidden="true">·</span>
            <i :class="activeSession.secured === false ? 'fa-solid fa-triangle-exclamation' : 'fa-solid fa-lock'" aria-hidden="true" />
            <span>{{ sessionSecurityLabel(activeSession) }}</span>
          </span>
        </div>
        <div class="active-session-footer">
          <span><i :class="activeSession.state === 'active' ? 'fa-solid fa-circle-check' : 'fa-regular fa-clock'" aria-hidden="true" /><span>{{ activeSession.state === 'pending' ? (activeSession.incoming ? t('incomingRequest') : t('waitingForRemoteApproval')) : t('sessionConnected') }}</span></span>
          <div class="active-session-footer-actions">
            <button v-if="activeSession.state === 'pending' && activeSession.incoming" class="button button-primary button-small" type="button" @click="approveSession">
              <i class="fa-solid fa-check" aria-hidden="true" />
              <span>{{ t('allow') }}</span>
            </button>
            <button class="button button-secondary button-small" type="button" @click="openSession">{{ t('openSession') }}</button>
          </div>
        </div>
        </template>
        <div v-else class="corevia-empty-state corevia-session-empty-state">
          <span><i class="fa-solid fa-link-slash corevia-icon-glyph" aria-hidden="true" /><strong>{{ t('noActiveSessions') }}</strong></span>
        </div>
      </article>

      <article class="glass-card session-report-strip" aria-labelledby="sessionReportsTitle">
        <div class="session-report-heading">
          <span class="session-report-icon"><i class="fa-solid fa-chart-simple" aria-hidden="true" /></span>
          <span><strong id="sessionReportsTitle">{{ t('sessionReports') }}</strong><small>{{ t('reportWindow') }}</small></span>
        </div>
        <div class="session-report-metrics">
          <div class="session-report-metric"><span>{{ t('openSessions') }}</span><strong dir="ltr">{{ sessions.length }}</strong></div>
          <div class="session-report-metric"><span>{{ t('recentConnections') }}</span><strong dir="ltr">{{ recentConnectionsCount }}</strong></div>
          <div class="session-report-metric"><span>{{ t('averageLatency') }}</span><strong dir="ltr">{{ t('notAvailable') }}</strong></div>
        </div>
      </article>

      <article class="glass-card history-card">
        <div class="card-heading">
          <div><span class="eyebrow eyebrow-muted">{{ t('history') }}</span><h2>{{ t('sessionHistory') }}</h2></div>
        </div>
        <div class="session-history-list">
          <article v-for="item in historyPresentations" :key="item.id" class="device-card history-item history-device-card device-surface-card" data-visual-mode="os" data-device-status="offline">
            <div class="device-card-media os-visual os-visual-offline" :class="item.visualClass" aria-hidden="true">
              <img class="os-visual-fallback-image" :src="theme === 'dark' ? item.darkImage : item.lightImage" alt="" />
              <span class="os-visual-glow" /><span class="os-visual-icon"><i :class="item.osIcon" /></span><span class="os-visual-label">{{ item.platform }}</span>
            </div>
            <div class="device-card-top">
              <span class="device-avatar" :class="item.avatarClass"><i :class="item.avatarIcon" /></span>
              <div class="device-card-copy"><h2 class="device-name" dir="ltr">{{ item.name }}</h2><p>{{ item.platform }}</p></div>
              <span class="device-card-context session-state" :class="item.state === 'failed' ? 'state-danger' : 'state-neutral'">
                <i :class="item.state === 'failed' ? 'fa-solid fa-triangle-exclamation' : 'fa-solid fa-clock-rotate-left'" aria-hidden="true" />
                <span>{{ historyStateLabel(item) }}</span>
              </span>
            </div>
            <div class="device-card-tags"><span class="tag-chip">{{ item.platform }}</span></div>
            <div class="device-card-footer">
              <span><i class="fa-regular fa-clock" aria-hidden="true" />{{ item.when || t('notAvailable') }}</span>
              <small v-if="item.error" class="session-error" :title="item.error">{{ t('connectionErrorRecorded') }}</small>
            </div>
          </article>
          <div v-if="!historyPresentations.length" class="corevia-empty-state corevia-history-empty-state">
            <span><i class="fa-regular fa-clock corevia-icon-glyph" aria-hidden="true" /><strong>{{ t('noSessionHistory') }}</strong></span>
          </div>
        </div>
      </article>
    </div>

    <Teleport to="body">
      <div v-if="showConnect" class="corevia-dialog-backdrop" @click.self="showConnect = false">
        <section class="corevia-dialog glass-surface" role="dialog" aria-modal="true" :aria-label="t('newConnection')">
          <div class="corevia-dialog-heading">
            <div><span class="eyebrow"><span class="eyebrow-dot" />COREVIA DESK / {{ t('newConnection') }}</span><h2>{{ t('connectTitle') }}</h2><p>{{ t('connectDescription') }}</p></div>
            <button class="icon-button corevia-dialog-close" type="button" :aria-label="t('closeSession')" @click="showConnect = false"><i class="fa-solid fa-xmark" aria-hidden="true" /></button>
          </div>
          <QuickConnectForm compact @connect="showConnect = false" />
        </section>
      </div>
    </Teleport>
  </section>
</template>
