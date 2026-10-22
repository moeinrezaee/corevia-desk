import type { CoreviaBridgeResponse } from '~/types/corevia-bridge'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useDeskNotifications } from '~/composables/useDeskNotifications'
import { isBridgeActionSuccessful } from '~/utils/bridge-response.mjs'

export const useDeskBridgeFeedback = () => {
  const { t } = useDeskCopy()
  const { push } = useDeskNotifications()

  const notifyFailure = (response: CoreviaBridgeResponse) => {
    if (isBridgeActionSuccessful(response)) return
    const message = response.error?.code === 'local-agent-required'
      ? t('runtimeUnavailable')
      : t('requestFailed')
    push(message, 'danger')
  }

  const notifyUnexpectedFailure = () => push(t('requestFailed'), 'danger')

  return { isBridgeActionSuccessful, notifyFailure, notifyUnexpectedFailure }
}
