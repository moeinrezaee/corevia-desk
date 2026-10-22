<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useDeskAppearance } from '~/composables/useDeskAppearance'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useDeskRuntime } from '~/composables/useDeskRuntime'
import type { DeskPage, DeskProfile, DeskNavItem } from '~/types/desk'

const props = defineProps<{
  profile: DeskProfile
  activePage: DeskPage
  collapsed?: boolean
}>()

const emit = defineEmits<{
  toggleRail: []
}>()

const { t } = useDeskCopy()
const { locale } = useDeskAppearance()
const { activeSessions, status, refresh } = useDeskRuntime()

onMounted(() => refresh())

const activeSessionCount = computed(() => status.value?.activeSessionCount ?? activeSessions.value.length)
const networkLabel = computed(() => status.value?.networkConfigured ? t('networkReady') : t('networkNotConfigured'))

const homePath = computed(() => props.profile === 'customer-support' ? '/customer-support' : '/full-desk')
const settingsPath = computed(() => props.profile === 'customer-support' ? '/settings?profile=customer-support' : '/settings')

const collapseIcon = computed(() => {
  const isRtl = locale.value === 'fa'
  return props.collapsed ? (isRtl ? 'collapse' : 'collapseRight') : (isRtl ? 'collapseRight' : 'collapse')
})

const items = computed<DeskNavItem[]>(() => {
  const common: DeskNavItem[] = [
    { page: 'home', labelKey: 'home', icon: 'home', to: homePath.value },
    { page: 'settings', labelKey: 'settings', icon: 'settings', to: settingsPath.value }
  ]

  if (props.profile === 'customer-support') return common

  return [
    common[0],
    { page: 'sessions', labelKey: 'sessions', icon: 'sessions', to: '/sessions' },
    { page: 'devices', labelKey: 'devices', icon: 'devices', to: '/devices' },
    common[1]
  ]
})
</script>

<template>
  <aside class="side-rail glass-surface" aria-label="Primary navigation">
    <div class="rail-heading">
      <span class="rail-heading-label">{{ t('desk') }}</span>
      <button
        class="icon-button icon-button-small"
        type="button"
        :aria-label="collapsed ? t('expandNavigation') : t('collapseNavigation')"
        :title="collapsed ? t('expandNavigation') : t('collapseNavigation')"
        @click="emit('toggleRail')"
      >
        <IconGlyph :name="collapseIcon" />
      </button>
    </div>

    <nav class="primary-nav" aria-label="Desk sections">
      <p class="nav-section-label">{{ t('main') }}</p>
      <NuxtLink
        v-for="(item, index) in items"
        :key="item.page"
        class="nav-item"
        :class="{ 'is-active': activePage === item.page }"
        :to="item.to"
        :external="props.profile === 'customer-support'"
      >
        <span class="nav-icon" :class="`nav-icon-${item.page === 'home' ? 'core' : item.page === 'devices' ? 'purple' : item.page === 'sessions' ? 'cyan' : 'neutral'}`">
          <IconGlyph :name="item.icon" />
        </span>
        <span class="nav-label">{{ t(item.labelKey) }}</span>
        <span v-if="item.page === 'sessions' && activeSessionCount" class="nav-count">{{ activeSessionCount }}</span>
        <span v-else-if="index === 0" class="nav-shortcut">1</span>
      </NuxtLink>
    </nav>

    <div class="rail-footer">
      <div class="network-card">
        <span class="network-card-icon"><IconGlyph name="network" /></span>
        <span class="network-card-copy">
          <strong>{{ t('network') }}</strong>
          <small>{{ networkLabel }}</small>
        </span>
        <StatusDot :class="{ 'is-muted': !status?.networkConfigured }" />
      </div>
    </div>
  </aside>
</template>
