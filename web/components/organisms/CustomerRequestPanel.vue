<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useCoreviaBridge } from '~/composables/useCoreviaBridge'
import { useDeskBridgeFeedback } from '~/composables/useDeskBridgeFeedback'
import { useDeskRuntime } from '~/composables/useDeskRuntime'
import { approveCustomerRequest, createCustomerRequestSelection } from '~/utils/customer-request-approval.mjs'

const emit = defineEmits<{
  approved: [permissions: string[]]
  denied: []
}>()

const { t } = useDeskCopy()
const { request } = useCoreviaBridge()
const { isBridgeActionSuccessful, notifyFailure, notifyUnexpectedFailure } = useDeskBridgeFeedback()
const { hasLocalBridge, pendingSessions, refresh, refreshSessions } = useDeskRuntime()
const isOpen = ref(false)
const selected = ref(createCustomerRequestSelection())
const approvalError = ref('')
const screenCaptureGranted = ref<boolean | null>(null)
const screenCaptureRequesting = ref(false)
const screenCaptureRequestSent = ref(false)
let requestRefreshTimer: ReturnType<typeof setInterval> | undefined
let requestPanelUnmounted = false

const pendingRequest = computed(() => pendingSessions.value.find((session) => session.incoming))
const requesterName = computed(() => pendingRequest.value?.name || t('notAvailable'))
const requesterAddress = computed(() => pendingRequest.value?.address || pendingRequest.value?.id || t('notAvailable'))

const resetSelection = () => {
  selected.value = createCustomerRequestSelection()
  approvalError.value = ''
}

watch(() => pendingRequest.value?.id, resetSelection)

const reviewRequest = () => {
  resetSelection()
  isOpen.value = true
}

const refreshWhenVisible = () => {
  if (document.visibilityState === 'visible') void refreshSessions()
}

onMounted(async () => {
  await refresh()
  if (requestPanelUnmounted || !hasLocalBridge.value) return

  const captureStatus = await request('read-screen-capture-status')
  if (captureStatus.status === 'accepted') {
    const granted = captureStatus.data?.granted
    screenCaptureGranted.value = typeof granted === 'boolean' ? granted : null
  }

  document.addEventListener('visibilitychange', refreshWhenVisible)
  requestRefreshTimer = setInterval(refreshWhenVisible, 1000)
})

onBeforeUnmount(() => {
  requestPanelUnmounted = true
  document.removeEventListener('visibilitychange', refreshWhenVisible)
  if (requestRefreshTimer) clearInterval(requestRefreshTimer)
})

const close = () => {
  isOpen.value = false
  resetSelection()
}

const requestScreenCaptureAccess = async () => {
  if (screenCaptureRequesting.value || screenCaptureRequestSent.value || !hasLocalBridge.value) return

  screenCaptureRequesting.value = true
  const response = await request(
    'request-screen-capture-access',
    {},
    'nuxt-customer-screen-capture-request'
  )
  screenCaptureRequesting.value = false

  if (!isBridgeActionSuccessful(response) || response.data?.supported !== true) {
    notifyFailure(response)
    return
  }

  screenCaptureRequestSent.value = true
}

const approve = async () => {
  const requestItem = pendingRequest.value
  if (!requestItem) return
  approvalError.value = ''

  const result = await approveCustomerRequest({
    connectionId: requestItem.id,
    screen: selected.value.screen,
    input: selected.value.input,
    request
  })
  if (result.status === 'screen-required') {
    approvalError.value = t('screenShareRequired')
    return
  }
  if (result.status === 'permission-failed' || result.status === 'approval-failed') {
    if (isBridgeActionSuccessful(result.response)) notifyUnexpectedFailure()
    else notifyFailure(result.response)
    return
  }

  if (result.status === 'approved-screen-only') {
    if (isBridgeActionSuccessful(result.response)) notifyUnexpectedFailure()
    else notifyFailure(result.response)
  }

  await refresh(true)
  emit('approved', result.permissions)
  close()
}

const deny = async () => {
  const requestItem = pendingRequest.value
  if (!requestItem) return
  const response = await request(
    'close-session',
    { connectionId: requestItem.id },
    'nuxt-customer-deny-approval'
  )
  if (!isBridgeActionSuccessful(response)) {
    notifyFailure(response)
    return
  }
  await refresh(true)
  emit('denied')
  close()
}
</script>

<template>
  <aside v-if="screenCaptureGranted === false" class="customer-capture-notice glass-card" role="status" aria-live="polite">
    <span class="customer-capture-notice-icon" aria-hidden="true"><i class="fa-solid fa-display" /></span>
    <span>
      <strong>{{ t('screenCapturePermissionTitle') }}</strong>
      <small>{{ t('screenCapturePermissionHelp') }}</small>
      <div class="customer-capture-notice-actions">
        <BaseButton
          size="small"
          variant="secondary"
          :disabled="screenCaptureRequesting || screenCaptureRequestSent"
          @click="requestScreenCaptureAccess"
        >
          <span>{{ t('requestScreenCaptureAccess') }}</span>
        </BaseButton>
        <small v-if="screenCaptureRequestSent">{{ t('screenCaptureRestartHelp') }}</small>
      </div>
    </span>
  </aside>

  <section class="customer-incoming-card glass-card corevia-incoming-card">
    <div class="corevia-incoming-copy">
      <span class="eyebrow eyebrow-muted">{{ t('incomingRequest') }}</span>
      <h2>{{ t('incomingTitle') }}</h2>
      <p>{{ t('incomingDescription') }}</p>
      <div class="corevia-requester">
        <span class="corevia-requester-avatar"><IconGlyph name="support" /></span>
        <span class="corevia-requester-copy">
          <strong>{{ requesterName }}</strong>
          <small dir="ltr">{{ requesterAddress }}</small>
        </span>
      </div>
    </div>

    <div class="corevia-incoming-actions">
      <span class="corevia-request-status" :class="{ 'is-muted': !pendingRequest }">
        <StatusDot :class="{ 'is-muted': !pendingRequest }" />{{ pendingRequest ? t('now') : t('notAvailable') }}
      </span>
      <div v-if="pendingRequest" class="corevia-access-list">
        <PermissionChip :label="t('screenShare')" icon="screen" />
        <PermissionChip :label="t('keyboardMouse')" icon="input" />
      </div>
      <p v-else class="corevia-request-empty">{{ t('noIncomingRequests') }}</p>
      <BaseButton variant="primary" :disabled="!pendingRequest" @click="reviewRequest">
        <IconGlyph name="shield" />
        <span>{{ t('reviewPermissions') }}</span>
      </BaseButton>
      <button class="text-button corevia-deny-button" type="button" :disabled="!pendingRequest" @click="deny">
        {{ t('deny') }}
      </button>
    </div>
  </section>

  <Teleport to="body">
    <div v-if="isOpen && pendingRequest" class="corevia-dialog-backdrop" @click.self="close">
      <section class="corevia-dialog glass-surface" role="dialog" aria-modal="true" aria-labelledby="request-dialog-title">
        <div class="corevia-dialog-heading">
          <div>
            <span class="eyebrow"><span class="eyebrow-dot" />COREVIA DESK / {{ t('incomingRequest') }}</span>
            <h2 id="request-dialog-title">{{ t('allow') }}؟</h2>
            <p>{{ t('incomingDescription') }}</p>
          </div>
          <button class="icon-button corevia-dialog-close" type="button" :aria-label="t('deny')" @click="close">
            <IconGlyph name="close" />
          </button>
        </div>

        <div class="corevia-requester">
          <span class="corevia-requester-avatar"><IconGlyph name="support" /></span>
          <span class="corevia-requester-copy">
            <strong>{{ requesterName }}</strong>
            <small dir="ltr">{{ requesterAddress }}</small>
          </span>
        </div>

        <div class="corevia-permission-options">
          <label class="corevia-permission-option" :class="{ 'is-checked': selected.screen }">
            <input v-model="selected.screen" type="checkbox" @change="approvalError = ''" />
            <span class="corevia-permission-mark"><IconGlyph name="check" /></span>
            <span><strong>{{ t('screenShare') }}</strong><small>{{ t('incomingDescription') }}</small></span>
          </label>
          <label class="corevia-permission-option" :class="{ 'is-checked': selected.input }">
            <input v-model="selected.input" type="checkbox" @change="approvalError = ''" />
            <span class="corevia-permission-mark"><IconGlyph name="check" /></span>
            <span><strong>{{ t('keyboardMouse') }}</strong><small>{{ t('keyboardMouseDescription') }}</small></span>
          </label>
        </div>

        <p v-if="approvalError" class="corevia-form-error" role="alert" aria-live="assertive">
          {{ approvalError }}
        </p>

        <p class="corevia-lock-callout">
          <IconGlyph name="lock" />
          <span>{{ t('permissionsNote') }}</span>
        </p>

        <div class="corevia-dialog-actions">
          <BaseButton variant="secondary" @click="deny">
            <IconGlyph name="close" />
            <span>{{ t('deny') }}</span>
          </BaseButton>
          <BaseButton variant="primary" @click="approve">
            <IconGlyph name="check" />
            <span>{{ t('allow') }}</span>
          </BaseButton>
        </div>
      </section>
    </div>
  </Teleport>
</template>
