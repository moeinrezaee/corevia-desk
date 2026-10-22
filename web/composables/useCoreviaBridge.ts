import { computed, ref } from 'vue'
import { useDeskProfile } from '~/composables/useDeskProfile'
import type {
  CoreviaBridgeOperation,
  CoreviaBridgeRequest,
  CoreviaBridgeResponse
} from '~/types/corevia-bridge'

let requestSequence = 0

const nextRequestId = () => {
  requestSequence += 1
  return `nuxt-${Date.now().toString(36)}-${requestSequence.toString(36)}`
}

export const useCoreviaBridge = () => {
  const { profile } = useDeskProfile()
  const source = ref<'rust' | 'unavailable' | 'error'>('unavailable')
  const lastError = ref<string | null>(null)

  const hasLocalBridge = computed(() => {
    return import.meta.client && typeof window.__COREVIA_DESK_BRIDGE__?.request === 'function'
  })

  const request = async (
    operation: CoreviaBridgeOperation,
    payload: Record<string, unknown> = {},
    approvalReference?: string
  ): Promise<CoreviaBridgeResponse> => {
    const bridgeRequest: CoreviaBridgeRequest = {
      requestId: nextRequestId(),
      operation,
      payload,
      ...(approvalReference ? { approvalReference } : {})
    }

    const localBridge = import.meta.client ? window.__COREVIA_DESK_BRIDGE__ : undefined
    if (localBridge) {
      try {
        const response = await localBridge.request(bridgeRequest)
        source.value = 'rust'
        lastError.value = response.error?.message ?? null
        return response
      } catch {
        source.value = 'error'
        lastError.value = 'The local Corevia Desk bridge is unavailable.'
        return {
          requestId: bridgeRequest.requestId,
          operation,
          surface: 'nuxt',
          profile: profile.value,
          status: 'error',
          error: {
            code: 'bridge-unavailable',
            message: lastError.value
          }
        }
      }
    }

    // A browser has no native identity, device list, or session state. Keep
    // those surfaces empty until the Rust WebView bridge is present.
    source.value = 'unavailable'
    lastError.value = 'A local Corevia Desk agent is required for native actions.'
    return {
      requestId: bridgeRequest.requestId,
      operation,
      surface: 'nuxt',
      profile: profile.value,
      status: 'unsupported',
      error: {
        code: 'local-agent-required',
        message: lastError.value
      }
    }
  }

  return {
    hasLocalBridge,
    lastError,
    request,
    source
  }
}
