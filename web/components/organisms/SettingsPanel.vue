<script setup lang="ts">
import { computed, onMounted, ref, watch } from 'vue'
import { useDeskAppearance } from '~/composables/useDeskAppearance'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useDeskBridgeFeedback } from '~/composables/useDeskBridgeFeedback'
import { useCoreviaBridge } from '~/composables/useCoreviaBridge'
import { useDeskRuntime } from '~/composables/useDeskRuntime'
import type { DeskProfile, DeskTheme } from '~/types/desk'

const props = defineProps<{
  profile: DeskProfile
}>()

type SettingsSection = 'general' | 'display' | 'security' | 'network' | 'advanced' | 'about'
type ImageQuality = 'balanced' | 'best' | 'low'

const { t } = useDeskCopy()
const { theme, locale, setTheme, setLocale } = useDeskAppearance()
const { isBridgeActionSuccessful, notifyFailure } = useDeskBridgeFeedback()
const { request } = useCoreviaBridge()
const { refresh, settings, status } = useDeskRuntime()
const activeSection = ref<SettingsSection>('general')
const showPermissions = ref(false)
const imageQuality = ref<ImageQuality>('balanced')

const tabsEnabled = ref(true)
const recordingEnabled = ref(false)
const viewOnlyEnabled = ref(false)
const monitorsEnabled = ref(true)
const remoteFocusEnabled = ref(false)
const clipboardEnabled = ref(true)
const clipboardDisabled = ref(false)
const keyboardEnabled = ref(true)
const filesEnabled = ref(true)
const audioEnabled = ref(true)
const terminalEnabled = ref(false)
const tunnelEnabled = ref(false)
const codecEnabled = ref(true)
const softwareRenderingEnabled = ref(false)
const websocketEnabled = ref(false)
const alwaysOnTopEnabled = ref(false)
const idServer = ref('')
const relayServer = ref('')
const apiServer = ref('')
const serverKey = ref('')
const networkUnlocked = ref(false)

const isFullDesk = computed(() => props.profile === 'full-desk')

const localSettingKeys = new Set([
  'theme',
  'lang',
  'enable-confirm-closing-tabs',
  'enable-open-new-connections-in-tabs'
])

const readSetting = (...keys: string[]) => {
  const values = settings.value?.values ?? {}
  for (const key of keys) {
    if (key in values) return values[key]
  }
  return undefined
}

const readBoolean = (fallback: boolean, ...keys: string[]) => {
  const value = readSetting(...keys)
  if (value === undefined) return fallback
  return value !== 'N' && value !== 'false' && value !== '0'
}

const applyRuntimeSettings = () => {
  tabsEnabled.value = readBoolean(true, 'enable-open-new-connections-in-tabs')
  recordingEnabled.value = readBoolean(false, 'allow-auto-record-incoming')
  viewOnlyEnabled.value = readBoolean(false, 'view_only', 'view-only')
  monitorsEnabled.value = readBoolean(true, 'show_monitors_toolbar', 'show-monitors-toolbar')
  remoteFocusEnabled.value = readBoolean(false, 'follow_remote_window', 'follow-remote-window')
  clipboardDisabled.value = readBoolean(false, 'disable_clipboard', 'disable-clipboard')
  clipboardEnabled.value = isFullDesk.value && readBoolean(true, 'enable-clipboard', 'enable_clipboard')
  keyboardEnabled.value = readBoolean(true, 'enable-keyboard', 'enable_keyboard')
  filesEnabled.value = isFullDesk.value && readBoolean(true, 'enable-file-transfer', 'enable_file_transfer')
  audioEnabled.value = isFullDesk.value && readBoolean(true, 'enable-audio', 'enable_audio')
  terminalEnabled.value = isFullDesk.value && readBoolean(false, 'enable-terminal', 'enable_terminal')
  tunnelEnabled.value = isFullDesk.value && readBoolean(false, 'enable-tunnel', 'enable_tunnel')
  codecEnabled.value = readBoolean(true, 'enable-hwcodec', 'enable_hwcodec')
  softwareRenderingEnabled.value = readBoolean(false, 'allow-always-software-render')
  websocketEnabled.value = readBoolean(false, 'allow-websocket')
  alwaysOnTopEnabled.value = readBoolean(false, 'main-window-always-on-top')

  const savedImageQuality = readSetting('image_quality')
  imageQuality.value = savedImageQuality === 'best' || savedImageQuality === 'low'
    ? savedImageQuality
    : 'balanced'

  const savedTheme = readSetting('theme')
  if (savedTheme === 'dark' || savedTheme === 'light') setTheme(savedTheme)
  const savedLocale = readSetting('lang')
  if (savedLocale === 'fa' || savedLocale === 'en') setLocale(savedLocale)

  const network = settings.value?.network
  if (network) {
    idServer.value = network.idServer
    relayServer.value = network.relayServer
    apiServer.value = network.apiServer
  }
}

const settingOperation = (key: string) => {
  if (localSettingKeys.has(key)) return 'write-customer-setting' as const
  return isFullDesk.value ? 'write-global-setting' as const : 'write-customer-setting' as const
}

const saveSetting = async (
  key: string,
  value: string | boolean | number,
  revertOnFailure = true,
  beforeFailureFeedback?: () => void
) => {
  const operation = settingOperation(key)
  const response = await request(
    operation,
    { key, value },
    operation === 'write-global-setting' ? 'nuxt-settings-approval' : undefined
  )
  if (isBridgeActionSuccessful(response)) {
    await refresh(true)
    return true
  } else {
    if (revertOnFailure) applyRuntimeSettings()
    beforeFailureFeedback?.()
    notifyFailure(response)
    return false
  }
}

const saveBoolean = async (key: string, value: boolean) => {
  await saveSetting(key, value)
}

const saveNetwork = async () => {
  if (!isFullDesk.value) return
  const values: Array<[string, string]> = [
    ['custom-rendezvous-server', idServer.value.trim()],
    ['relay-server', relayServer.value.trim()],
    ['api-server', apiServer.value.trim()]
  ]
  if (serverKey.value.trim()) values.push(['key', serverKey.value.trim()])
  for (const [key, value] of values) {
    if (!await saveSetting(key, value, false)) return
  }
  serverKey.value = ''
  networkUnlocked.value = false
}

onMounted(() => refresh())
watch(settings, applyRuntimeSettings, { immediate: true })

const sections = computed<Array<{ id: SettingsSection; label: string; icon: string }>>(() => {
  const common = [
    { id: 'general' as const, label: 'general', icon: 'fa-solid fa-circle-info' },
    { id: 'security' as const, label: 'security', icon: 'fa-solid fa-shield-halved' },
    { id: 'about' as const, label: 'about', icon: 'fa-solid fa-circle-question' }
  ]

  if (!isFullDesk.value) return common

  return [
    common[0],
    { id: 'display', label: 'display', icon: 'fa-solid fa-display' },
    common[1],
    { id: 'network', label: 'network', icon: 'fa-solid fa-network-wired' },
    { id: 'advanced', label: 'advanced', icon: 'fa-solid fa-sliders' },
    common[2]
  ]
})

const chooseTheme = async (value: DeskTheme) => {
  const previous = theme.value
  setTheme(value)
  if (!await saveSetting('theme', value)) setTheme(previous)
}

const chooseLocale = async (value: 'fa' | 'en') => {
  const previous = locale.value
  setLocale(value)
  await saveSetting('lang', value, true, () => setLocale(previous))
}

const chooseImageQuality = async (value: ImageQuality) => {
  const previous = imageQuality.value
  imageQuality.value = value
  if (!await saveSetting('image_quality', value)) imageQuality.value = previous
}
</script>

<template>
  <section class="view-panel corevia-settings-page corevia-page-stack">
    <div class="screen-header settings-page-heading">
      <div>
        <span class="eyebrow"><span class="eyebrow-dot" />COREVIA DESK / {{ t('settings') }}</span>
        <h1>{{ t('settingsTitle') }}</h1>
        <p>{{ t('settingsDescription') }}</p>
      </div>
      <span class="settings-badge"><i class="fa-solid fa-sliders" aria-hidden="true" /><span>{{ t('localSettings') }}</span></span>
    </div>

    <div class="settings-layout">
      <nav class="settings-nav glass-card" aria-label="Settings categories">
        <button
          v-for="section in sections"
          :key="section.id"
          class="settings-nav-item"
          :class="{ 'is-active': activeSection === section.id }"
          type="button"
          @click="activeSection = section.id"
        >
          <i :class="section.icon" aria-hidden="true" />
          <span>{{ t(section.label) }}</span>
        </button>
      </nav>

      <div class="settings-content">
        <article v-if="activeSection === 'general'" class="settings-panel glass-card">
          <div class="settings-panel-heading">
            <span class="settings-panel-icon settings-panel-icon-purple"><i class="fa-solid fa-circle-info" aria-hidden="true" /></span>
            <div><h2>{{ t('generalTitle') }}</h2><p>{{ t('generalDescription') }}</p></div>
          </div>

          <div v-if="isFullDesk" class="setting-block mode-setting-block">
            <div class="setting-label"><strong>{{ t('clientMode') }}</strong><small>{{ t('clientModeHelp') }}</small></div>
            <div class="mode-choice-grid" role="group" :aria-label="t('clientMode')">
              <div class="mode-choice is-readonly" :class="{ 'is-active': isFullDesk }" :aria-current="isFullDesk ? 'true' : undefined">
                <span class="mode-choice-icon mode-choice-icon-core"><i class="fa-solid fa-arrows-to-circle" aria-hidden="true" /></span>
                <span><strong>{{ t('fullModeTitle') }}</strong><small>{{ t('fullModeDescription') }}</small></span>
                <i class="fa-solid fa-check mode-choice-check" aria-hidden="true" />
              </div>
              <div class="mode-choice is-readonly">
                <span class="mode-choice-icon mode-choice-icon-purple"><i class="fa-solid fa-shield-halved" aria-hidden="true" /></span>
                <span><strong>{{ t('customerModeTitle') }}</strong><small>{{ t('customerModeDescription') }}</small></span>
                <i class="fa-solid fa-check mode-choice-check" aria-hidden="true" />
              </div>
            </div>
          </div>

          <div v-else class="setting-block customer-mode-lock">
            <div class="setting-label"><strong>{{ t('customerModeLocked') }}</strong><small>{{ t('customerModeLockedHelp') }}</small></div>
            <span class="mode-lock-badge"><i class="fa-solid fa-lock" aria-hidden="true" /><span>{{ t('installerManaged') }}</span></span>
          </div>

          <div class="setting-block">
            <div class="setting-label"><strong>{{ t('theme') }}</strong><small>{{ t('themeHelp') }}</small></div>
            <div class="segmented-control" role="group" :aria-label="t('theme')">
              <button type="button" :class="{ 'is-active': theme === 'dark' }" @click="chooseTheme('dark')"><i class="fa-solid fa-moon" aria-hidden="true" /><span>{{ t('dark') }}</span></button>
              <button type="button" :class="{ 'is-active': theme === 'light' }" @click="chooseTheme('light')"><i class="fa-solid fa-sun" aria-hidden="true" /><span>{{ t('light') }}</span></button>
              <button type="button" disabled><i class="fa-solid fa-circle-half-stroke" aria-hidden="true" /><span>{{ t('systemTheme') }}</span></button>
            </div>
          </div>

          <div class="setting-block">
            <div class="setting-label"><strong>{{ t('language') }}</strong><small>{{ t('languageHelp') }}</small></div>
            <div class="language-options language-options-compact">
              <button class="language-option" :class="{ 'is-active': locale === 'fa' }" type="button" @click="chooseLocale('fa')">
                <span class="language-flag">فا</span><span><strong>{{ t('persianLanguage') }}</strong><small>{{ t('persianLanguageMeta') }}</small></span><i class="fa-solid fa-check" aria-hidden="true" />
              </button>
              <button class="language-option" :class="{ 'is-active': locale === 'en' }" type="button" @click="chooseLocale('en')">
                <span class="language-flag language-flag-en">EN</span><span><strong>{{ t('englishLanguage') }}</strong><small>{{ t('englishLanguageMeta') }}</small></span><i class="fa-solid fa-check" aria-hidden="true" />
              </button>
            </div>
          </div>

          <div class="setting-block"><div class="setting-label"><strong>{{ t('openConnectionsInTabs') }}</strong><small>{{ t('openConnectionsInTabsHelp') }}</small></div><label class="toggle-row"><input v-model="tabsEnabled" type="checkbox" @change="saveBoolean('enable-open-new-connections-in-tabs', tabsEnabled)" /><span class="toggle-control" aria-hidden="true" /></label></div>
          <div v-if="isFullDesk" class="setting-block"><div class="setting-label"><strong>{{ t('autoRecordIncoming') }}</strong><small>{{ t('autoRecordIncomingHelp') }}</small></div><label class="toggle-row"><input v-model="recordingEnabled" type="checkbox" @change="saveBoolean('allow-auto-record-incoming', recordingEnabled)" /><span class="toggle-control" aria-hidden="true" /></label></div>
        </article>

        <article v-else-if="activeSection === 'display' && isFullDesk" class="settings-panel glass-card">
          <div class="settings-panel-heading"><span class="settings-panel-icon settings-panel-icon-cyan"><i class="fa-solid fa-display" aria-hidden="true" /></span><div><h2>{{ t('displayTitle') }}</h2><p>{{ t('displayDescription') }}</p></div></div>
          <div class="setting-block"><div class="setting-label"><strong>{{ t('imageQuality') }}</strong><small>{{ t('imageQualityHelp') }}</small></div><div class="segmented-control" role="group" :aria-label="t('imageQuality')"><button :class="{ 'is-active': imageQuality === 'balanced' }" type="button" :aria-pressed="imageQuality === 'balanced'" @click="chooseImageQuality('balanced')">{{ t('balanced') }}</button><button :class="{ 'is-active': imageQuality === 'best' }" type="button" :aria-pressed="imageQuality === 'best'" @click="chooseImageQuality('best')">{{ t('bestQuality') }}</button><button :class="{ 'is-active': imageQuality === 'low' }" type="button" :aria-pressed="imageQuality === 'low'" @click="chooseImageQuality('low')">{{ t('speed') }}</button></div></div>
          <div class="setting-block setting-block-list">
            <label class="toggle-row setting-toggle-full"><span class="toggle-copy"><span class="toggle-title">{{ t('viewOnly') }}</span><small>{{ t('viewOnlyHelp') }}</small></span><input v-model="viewOnlyEnabled" type="checkbox" @change="saveBoolean('view_only', viewOnlyEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="toggle-row setting-toggle-full"><span class="toggle-copy"><span class="toggle-title">{{ t('showMonitorsToolbar') }}</span><small>{{ t('showMonitorsToolbarHelp') }}</small></span><input v-model="monitorsEnabled" type="checkbox" @change="saveBoolean('show_monitors_toolbar', monitorsEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="toggle-row setting-toggle-full"><span class="toggle-copy"><span class="toggle-title">{{ t('followRemoteFocus') }}</span><small>{{ t('followRemoteFocusHelp') }}</small></span><input v-model="remoteFocusEnabled" type="checkbox" @change="saveBoolean('follow_remote_window', remoteFocusEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="toggle-row setting-toggle-full"><span class="toggle-copy"><span class="toggle-title">{{ t('disableClipboard') }}</span><small>{{ t('disableClipboardHelp') }}</small></span><input v-model="clipboardDisabled" type="checkbox" @change="saveBoolean('disable_clipboard', clipboardDisabled)" /><span class="toggle-control" aria-hidden="true" /></label>
          </div>
        </article>

        <article v-else-if="activeSection === 'security'" class="settings-panel glass-card">
          <div class="settings-panel-heading"><span class="settings-panel-icon settings-panel-icon-pink"><i class="fa-solid fa-shield-halved" aria-hidden="true" /></span><div><h2>{{ t('securityTitle') }}</h2><p>{{ t('securityDescription') }}</p></div></div>
          <div v-if="!isFullDesk" class="customer-security-callout"><i class="fa-solid fa-arrow-down-to-bracket" aria-hidden="true" /><span>{{ t('customerSecurityNote') }}</span></div>
          <div class="security-settings-list">
            <label class="security-setting-row"><span><strong>{{ t('keyboardMouse') }}</strong><small>{{ t('keyboardMouseHelp') }}</small></span><input v-model="keyboardEnabled" type="checkbox" @change="saveBoolean('enable-keyboard', keyboardEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="security-setting-row"><span><strong>{{ t('clipboard') }}</strong><small>{{ t('clipboardHelp') }}</small></span><input v-model="clipboardEnabled" :disabled="!isFullDesk" type="checkbox" @change="saveBoolean('enable-clipboard', clipboardEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="security-setting-row"><span><strong>{{ t('fileTransfer') }}</strong><small>{{ t('fileTransferHelp') }}</small></span><input v-model="filesEnabled" :disabled="!isFullDesk" type="checkbox" @change="saveBoolean('enable-file-transfer', filesEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="security-setting-row"><span><strong>{{ t('audio') }}</strong><small>{{ t('audioHelp') }}</small></span><input v-model="audioEnabled" :disabled="!isFullDesk" type="checkbox" @change="saveBoolean('enable-audio', audioEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="security-setting-row"><span><strong>{{ t('terminal') }}</strong><small>{{ t('terminalHelp') }}</small></span><input v-model="terminalEnabled" :disabled="!isFullDesk" type="checkbox" @change="saveBoolean('enable-terminal', terminalEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="security-setting-row"><span><strong>{{ t('tcpTunneling') }}</strong><small>{{ t('tcpTunnelingHelp') }}</small></span><input v-model="tunnelEnabled" :disabled="!isFullDesk" type="checkbox" @change="saveBoolean('enable-tunnel', tunnelEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
          </div>
          <div v-if="isFullDesk" class="settings-action-row"><button class="button button-secondary" type="button" @click="showPermissions = true"><i class="fa-solid fa-lock-open" aria-hidden="true" /><span>{{ t('openPermissionRequest') }}</span></button><span class="settings-action-note">{{ t('permissionSettingsNote') }}</span></div>
        </article>

        <article v-else-if="activeSection === 'network' && isFullDesk" class="settings-panel glass-card">
          <div class="settings-panel-heading"><span class="settings-panel-icon settings-panel-icon-core"><i class="fa-solid fa-network-wired" aria-hidden="true" /></span><div><h2>{{ t('networkTitle') }}</h2><p>{{ t('networkDescription') }}</p></div></div>
          <div class="server-config-grid"><label class="field-label"><span>{{ t('idServer') }}</span><input v-model="idServer" class="text-input" :disabled="!networkUnlocked" dir="ltr" /></label><label class="field-label"><span>{{ t('relayServer') }}</span><input v-model="relayServer" class="text-input" :disabled="!networkUnlocked" dir="ltr" /></label><label class="field-label"><span>{{ t('apiServer') }}</span><input v-model="apiServer" class="text-input" :disabled="!networkUnlocked" dir="ltr" /></label><label class="field-label"><span>{{ t('publicKey') }}</span><input v-model="serverKey" class="text-input" :disabled="!networkUnlocked" type="password" dir="ltr" :placeholder="settings?.network.keyConfigured ? '••••••••••••••••' : t('notAvailable')" /></label></div>
          <div class="settings-action-row"><button class="button button-secondary" type="button" @click="networkUnlocked = !networkUnlocked"><i class="fa-solid" :class="networkUnlocked ? 'fa-lock' : 'fa-unlock'" aria-hidden="true" /><span>{{ t('unlockNetwork') }}</span></button><button v-if="networkUnlocked" class="button button-primary" type="button" @click="saveNetwork"><i class="fa-solid fa-floppy-disk" aria-hidden="true" /><span>{{ t('saveDevice') }}</span></button><span class="settings-action-note">{{ t('networkKeyNote') }}</span></div>
          <div class="setting-block"><div class="setting-label"><strong>{{ t('proxy') }}</strong><small>{{ t('proxyHelp') }}</small></div><span class="settings-action-note">{{ t('proxyUnavailable') }}</span></div>
        </article>

        <article v-else-if="activeSection === 'advanced' && isFullDesk" class="settings-panel glass-card">
          <div class="settings-panel-heading"><span class="settings-panel-icon settings-panel-icon-orange"><i class="fa-solid fa-sliders" aria-hidden="true" /></span><div><h2>{{ t('advancedTitle') }}</h2><p>{{ t('advancedDescription') }}</p></div></div>
          <div class="setting-block setting-block-list">
            <label class="toggle-row setting-toggle-full"><span class="toggle-copy"><span class="toggle-title">{{ t('hardwareCodec') }}</span><small>{{ t('hardwareCodecHelp') }}</small></span><input v-model="codecEnabled" type="checkbox" @change="saveBoolean('enable-hwcodec', codecEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="toggle-row setting-toggle-full"><span class="toggle-copy"><span class="toggle-title">{{ t('softwareRendering') }}</span><small>{{ t('softwareRenderingHelp') }}</small></span><input v-model="softwareRenderingEnabled" type="checkbox" @change="saveBoolean('allow-always-software-render', softwareRenderingEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="toggle-row setting-toggle-full"><span class="toggle-copy"><span class="toggle-title">{{ t('websocket') }}</span><small>{{ t('websocketHelp') }}</small></span><input v-model="websocketEnabled" type="checkbox" @change="saveBoolean('allow-websocket', websocketEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
            <label class="toggle-row setting-toggle-full"><span class="toggle-copy"><span class="toggle-title">{{ t('alwaysOnTop') }}</span><small>{{ t('alwaysOnTopHelp') }}</small></span><input v-model="alwaysOnTopEnabled" type="checkbox" @change="saveBoolean('main-window-always-on-top', alwaysOnTopEnabled)" /><span class="toggle-control" aria-hidden="true" /></label>
          </div>
        </article>

        <article v-else class="settings-panel glass-card">
          <div class="settings-panel-heading"><span class="settings-panel-icon settings-panel-icon-neutral"><i class="fa-solid fa-circle-question" aria-hidden="true" /></span><div><h2>{{ t('aboutTitle') }}</h2><p>{{ t('aboutDescription') }}</p></div></div>
          <div class="about-card"><div><span>{{ t('version') }}</span><strong dir="ltr">{{ status?.version || t('notAvailable') }}</strong></div><div><span>{{ t('engine') }}</span><strong>Corevia Desk client</strong></div><div><span>{{ t('support') }}</span><strong>{{ t('documentation') }}</strong></div></div>
          <div class="info-callout"><i class="fa-solid fa-circle-info" aria-hidden="true" /><span>{{ t('aboutNote') }}</span></div>
        </article>
      </div>
    </div>
  </section>

  <PermissionsDialog :open="showPermissions" @close="showPermissions = false" />
</template>
