<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref } from 'vue'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { copyText } from '~/composables/useDeskClipboard'
import { useCoreviaBridge } from '~/composables/useCoreviaBridge'
import { useDeskBridgeFeedback } from '~/composables/useDeskBridgeFeedback'
import { useDeskRuntime } from '~/composables/useDeskRuntime'

const emit = defineEmits<{
  copied: []
}>()

const { t } = useDeskCopy()
const { identity, refresh, source, status } = useDeskRuntime()
const { request } = useCoreviaBridge()
const { notifyFailure, notifyUnexpectedFailure } = useDeskBridgeFeedback()
const id = computed(() => identity.value?.localId ?? '')
const secretAvailable = computed(() => identity.value?.temporaryPasswordAvailable ?? false)
const routeConfigured = computed(() => source.value === 'rust' && Boolean(id.value) && status.value?.networkConfigured === true)
const identityStatus = computed(() => {
  if (source.value === 'unavailable') return t('nativeUnavailableShort')
  if (source.value === 'error') return t('runtimeReadErrorShort')
  if (!id.value) return t('identityUnavailable')
  if (!status.value?.networkConfigured) return t('networkNotConfigured')
  return t('idRelayConfigured')
})
const showPassword = ref(false)
const temporaryPassword = ref('')
const loadingPassword = ref(false)

onMounted(() => refresh())

const copy = async (value: string) => {
  if (!value) return
  if (await copyText(value)) emit('copied')
}

const togglePassword = async () => {
  if (!secretAvailable.value || loadingPassword.value) return
  if (showPassword.value) {
    showPassword.value = false
    temporaryPassword.value = ''
    return
  }
  if (!showPassword.value && !temporaryPassword.value) {
    loadingPassword.value = true
    const response = await request('read-temporary-password')
    const value = response.status === 'accepted' ? response.data?.temporaryPassword : undefined
    if (typeof value !== 'string' || !value) {
      loadingPassword.value = false
      if (response.status === 'accepted') {
        notifyUnexpectedFailure()
      } else {
        notifyFailure(response)
      }
      return
    }
    temporaryPassword.value = value
    loadingPassword.value = false
  }
  showPassword.value = true
}

onBeforeUnmount(() => {
  temporaryPassword.value = ''
})
</script>

<template>
  <article class="local-identity-card corevia-identity-card" aria-label="This device identity">
    <div class="identity-card-heading">
      <span class="identity-icon">
        <IconGlyph name="monitor" />
      </span>
      <span>
        <strong>{{ t('thisDevice') }}</strong>
        <small><StatusDot :class="{ 'is-muted': !routeConfigured }" />{{ identityStatus }}</small>
      </span>
    </div>

    <div class="identity-field">
      <span>{{ t('yourId') }}</span>
      <strong class="code-value" dir="ltr">{{ id || '—' }}</strong>
      <button
        class="corevia-copy-button"
        type="button"
        :aria-label="t('yourId')"
        :disabled="!id"
        :title="t('yourId')"
        @click="copy(id)"
      >
        <IconGlyph name="copy" />
      </button>
    </div>

    <div class="identity-field">
      <span>{{ t('oneTimePassword') }}</span>
      <strong class="code-value" dir="ltr">{{ showPassword ? temporaryPassword : secretAvailable ? '••••••••' : '—' }}</strong>
      <button
        class="corevia-copy-button"
        type="button"
        :aria-label="showPassword ? t('hideSecret') : t('showSecret')"
        :title="showPassword ? t('hideSecret') : t('showSecret')"
        :aria-pressed="showPassword"
        :disabled="!secretAvailable || loadingPassword"
        @click="togglePassword"
      >
        <IconGlyph :name="showPassword ? 'eyeSlash' : 'eye'" />
      </button>
    </div>
  </article>
</template>
