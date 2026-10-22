<script setup lang="ts">
import { onMounted } from 'vue'
import { useDeskAppearance } from '~/composables/useDeskAppearance'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useDeskRuntime } from '~/composables/useDeskRuntime'
import type { DeskProfile } from '~/types/desk'

const props = defineProps<{
  profile: DeskProfile
  homePath?: string
}>()

const emit = defineEmits<{
  toggleTheme: []
  toggleLocale: []
}>()

const { theme, locale } = useDeskAppearance()
const { t } = useDeskCopy()
const { status, refresh } = useDeskRuntime()

onMounted(() => refresh())
</script>

<template>
  <header class="topbar glass-surface" aria-label="Corevia Desk toolbar">
    <NuxtLink
      class="brand-lockup"
      :to="props.homePath ?? '/'"
      :external="props.profile === 'customer-support'"
      :aria-label="t('desk')"
    >
      <BrandMark />
      <span class="brand-copy">
        <span class="brand-name">Corevia</span>
        <span class="brand-product">Desk</span>
      </span>
    </NuxtLink>

    <div class="topbar-center">
      <BaseBadge :tone="status?.networkConfigured ? 'success' : 'neutral'">
        {{ status?.networkConfigured ? t('networkReady') : t('networkNotConfigured') }}
      </BaseBadge>
      <span class="mode-indicator" :class="{ 'is-customer': profile === 'customer-support' }">
        <IconGlyph name="connection" />
        <span>{{ profile === 'customer-support' ? t('customerSupport') : t('fullDesk') }}</span>
      </span>
    </div>

    <div class="topbar-actions">
      <button
        class="icon-button corevia-topbar-button"
        type="button"
        :aria-label="theme === 'dark' ? t('light') : t('dark')"
        @click="emit('toggleTheme')"
      >
        <IconGlyph :name="theme === 'dark' ? 'moon' : 'sun'" />
      </button>
      <button
        class="language-button"
        type="button"
        :aria-label="t('language')"
        @click="emit('toggleLocale')"
      >
        <span class="language-code">{{ locale === 'fa' ? 'EN' : 'FA' }}</span>
        <IconGlyph name="language" />
      </button>
    </div>
  </header>
</template>
