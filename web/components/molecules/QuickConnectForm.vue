<script setup lang="ts">
import { ref } from 'vue'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useCoreviaBridge } from '~/composables/useCoreviaBridge'
import { useDeskBridgeFeedback } from '~/composables/useDeskBridgeFeedback'
import { useDeskRuntime } from '~/composables/useDeskRuntime'

const props = defineProps<{
  compact?: boolean
}>()

const emit = defineEmits<{
  connect: [remoteId: string]
}>()

const { t } = useDeskCopy()
const { request } = useCoreviaBridge()
const { isBridgeActionSuccessful, notifyFailure } = useDeskBridgeFeedback()
const { refresh } = useDeskRuntime()
const remoteIdInput = ref<HTMLInputElement | null>(null)
const remoteId = ref('')
const formError = ref('')

const focusInput = () => {
  const input = remoteIdInput.value
  if (!input) return
  input.scrollIntoView({ behavior: 'smooth', block: 'center' })
  input.focus({ preventScroll: true })
}

defineExpose({ focusInput })

const submit = async () => {
  const value = remoteId.value.trim()
  if (!value) {
    formError.value = t('peerIdRequired')
    return
  }
  formError.value = ''

  const response = await request(
    'start-connection',
    { remoteId: value },
    'nuxt-home-quick-connect-approval'
  )
  // A connection is announced only after the Rust bridge accepts it. Browser
  // previews without the native host must not present a fake live connection.
  if (isBridgeActionSuccessful(response)) {
    await refresh(true)
    emit('connect', value)
  } else {
    notifyFailure(response)
  }
}
</script>

<template>
  <section id="connectCard" class="connection-card glass-card corevia-connection-card" :class="{ 'is-dialog': props.compact }">
    <div v-if="!props.compact" class="connection-card-copy">
      <span class="eyebrow eyebrow-muted">{{ t('connectToDesktop') }}</span>
      <h2>{{ t('connectTitle') }}</h2>
      <p>{{ t('connectDescription') }}</p>
    </div>

    <form class="connect-form corevia-connect-form" @submit.prevent="submit">
      <label class="sr-only" for="remote-id">{{ t('remoteIdPlaceholder') }}</label>
      <input
        ref="remoteIdInput"
        id="remote-id"
        v-model="remoteId"
        class="text-input connect-input"
        type="text"
        inputmode="numeric"
        autocomplete="off"
        :placeholder="t('remoteIdPlaceholder')"
        :aria-invalid="formError ? 'true' : undefined"
        :aria-describedby="formError ? 'remote-id-error' : undefined"
        @input="formError = ''"
      />
      <BaseButton variant="primary" type="submit">
        <span>{{ t('connect') }}</span>
        <IconGlyph name="arrow" />
      </BaseButton>
    </form>

    <p v-if="formError" id="remote-id-error" class="corevia-form-error" role="alert">{{ formError }}</p>

    <div class="connection-hint corevia-connection-hint">
      <IconGlyph name="shield" />
      <span>{{ t('connectionHint') }}</span>
    </div>
  </section>
</template>
