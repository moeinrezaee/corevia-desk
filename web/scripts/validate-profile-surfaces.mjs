import { readFileSync } from 'node:fs'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const root = fileURLToPath(new URL('..', import.meta.url))
const read = (relativePath) => readFileSync(join(root, relativePath), 'utf8')

const customerRequestPanel = read('components/organisms/CustomerRequestPanel.vue')
if (/fileTransfer|selected\.files|use-file-transfer/iu.test(customerRequestPanel)) {
  throw new Error('Customer Support request UI exposes file transfer.')
}

const profile = read('composables/useDeskProfile.ts')
if (!profile.includes("if (configuredProfile === 'customer-support') return 'customer-support'")) {
  throw new Error('Customer Support profile can be escalated from the URL.')
}

const sidebar = read('components/organisms/DeskSidebar.vue')
if (!sidebar.includes("if (props.profile === 'customer-support') return common")) {
  throw new Error('Customer Support navigation is not restricted to common pages.')
}

const settingsPanel = read('components/organisms/SettingsPanel.vue')
for (const setting of ['clipboardEnabled', 'filesEnabled', 'audioEnabled', 'terminalEnabled', 'tunnelEnabled']) {
  if (!settingsPanel.includes(`${setting}.value = isFullDesk.value &&`)) {
    throw new Error(`Customer Support must not report ${setting} as enabled.`)
  }
}
if (!settingsPanel.includes("saveSetting('image_quality', value)")) {
  throw new Error('Display quality controls are not connected to the native settings action.')
}
if (settingsPanel.includes('@click="selectedMode =')) {
  throw new Error('Installed profile must not be switchable from a display-only mode selector.')
}

const runtime = read('composables/useDeskRuntime.ts')
if (!runtime.includes('const { locale, setLocale, setTheme } = useDeskAppearance()')
  || !runtime.includes('const savedTheme = runtime.value.settings.values.theme')
  || !runtime.includes('const savedLocale = runtime.value.settings.values.lang')) {
  throw new Error('Persisted theme and language must be applied from the shared runtime on every entry route.')
}

const permissionsDialog = read('components/organisms/PermissionsDialog.vue')
if (!permissionsDialog.includes("t('permissionRuntimeCheckNote')")
  || permissionsDialog.includes("t('granted')")
  || permissionsDialog.includes("t('needsAttention')")
  || permissionsDialog.includes("emit('action'")) {
  throw new Error('Permission guidance must not report unverified status or expose unwired actions.')
}

const fullDeskPage = read('pages/full-desk.vue')
const customerPage = read('pages/customer-support.vue')
if (!fullDeskPage.includes('DeskDashboardTemplate') || !customerPage.includes('DeskDashboardTemplate')) {
  throw new Error('Both product profile entrypoints must compose the dashboard template.')
}

const quickConnectForm = read('components/molecules/QuickConnectForm.vue')
const sessionsOverview = read('components/organisms/SessionsOverview.vue')
const nuxtStyles = read('assets/css/corevia-nuxt.css')
if (!sessionsOverview.includes('<QuickConnectForm compact @connect="showConnect = false" />')
  || !quickConnectForm.includes('v-if="!props.compact" class="connection-card-copy"')
  || !nuxtStyles.includes('.corevia-connection-card.is-dialog')) {
  throw new Error('The Sessions connection dialog must use the compact single-column Peer ID form.')
}

console.log('Profile surfaces PASS: Customer Support stays inbound-only and Full Desk keeps its separate surface.')
