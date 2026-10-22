<script setup lang="ts">
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useDeskNotifications } from '~/composables/useDeskNotifications'

const { t } = useDeskCopy()
const { notifications, dismiss } = useDeskNotifications()

const icons: Record<string, string> = {
  success: 'check',
  warning: 'exclamation',
  danger: 'exclamation',
  info: 'info'
}

const toneLabel = (tone: string) => {
  if (tone === 'success') return t('success')
  if (tone === 'warning') return t('warning')
  if (tone === 'danger') return t('danger')
  return t('info')
}
</script>

<template>
  <div class="corevia-notification-region" aria-live="polite" aria-atomic="false">
    <TransitionGroup name="corevia-notification" tag="div" class="corevia-notification-list">
      <article
        v-for="notice in notifications"
        :key="notice.id"
        class="corevia-notification"
        :class="`is-${notice.tone}`"
        :data-notification-tone="notice.tone"
        :role="notice.tone === 'danger' ? 'alert' : 'status'"
      >
        <span class="corevia-notification-icon" :aria-label="toneLabel(notice.tone)">
          <IconGlyph :name="icons[notice.tone]" />
        </span>
        <span class="corevia-notification-message">{{ notice.message }}</span>
        <button
          class="corevia-notification-dismiss"
          type="button"
          :aria-label="t('dismissNotification')"
          :title="t('dismissNotification')"
          @click="dismiss(notice.id)"
        >
          <IconGlyph name="close" />
        </button>
      </article>
    </TransitionGroup>
  </div>
</template>
