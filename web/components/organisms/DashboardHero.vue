<script setup lang="ts">
import { useDeskCopy } from '~/composables/useDeskCopy'
import type { DeskProfile } from '~/types/desk'

const props = defineProps<{
  profile: DeskProfile
}>()

const emit = defineEmits<{
  quickConnect: []
  copied: []
}>()

const { t } = useDeskCopy()
</script>

<template>
  <section class="page-heading corevia-page-heading">
    <div class="page-heading-copy">
      <span class="eyebrow">
        <span class="eyebrow-dot" />
        COREVIA DESK / {{ props.profile === 'customer-support' ? t('customerSupport') : t('home') }}
      </span>
      <h1>{{ props.profile === 'customer-support' ? t('customerWelcome') : t('welcome') }}</h1>
      <p>{{ props.profile === 'customer-support' ? t('customerWelcomeDescription') : t('welcomeDescription') }}</p>

      <div v-if="props.profile === 'customer-support'" class="corevia-mode-note">
        <IconGlyph name="shield" />
        <span>{{ t('receiveOnly') }}</span>
      </div>
      <div v-else class="heading-actions">
        <BaseButton variant="primary" @click="emit('quickConnect')">
          <IconGlyph name="plug" />
          <span>{{ t('quickConnect') }}</span>
        </BaseButton>
        <NuxtLink class="button button-secondary" to="/devices">
          <IconGlyph name="devices" />
          <span>{{ t('browseDevices') }}</span>
        </NuxtLink>
      </div>
    </div>

    <ConnectionIdCard @copied="emit('copied')" />
  </section>
</template>
