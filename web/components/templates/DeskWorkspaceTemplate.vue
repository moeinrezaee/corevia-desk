<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { useDeskAppearance } from '~/composables/useDeskAppearance'
import { useDeskCopy } from '~/composables/useDeskCopy'
import type { DeskPage, DeskProfile } from '~/types/desk'

const props = defineProps<{
  profile: DeskProfile
  activePage: DeskPage
}>()

const { theme, locale, toggleTheme, toggleLocale } = useDeskAppearance()
const { t } = useDeskCopy()
const railCollapsed = ref(false)
const homePath = computed(() => props.profile === 'customer-support' ? '/customer-support' : '/full-desk')
const settingsPath = computed(() => props.profile === 'customer-support' ? '/settings?profile=customer-support' : '/settings')

const syncRootAttributes = () => {
  if (!import.meta.client) return
  const root = document.documentElement
  root.lang = locale.value
  root.dir = locale.value === 'fa' ? 'rtl' : 'ltr'
  root.dataset.theme = theme.value
  root.dataset.language = locale.value
  root.dataset.mode = props.profile === 'customer-support' ? 'customer' : 'operator'
}

watch([theme, locale, () => props.profile], syncRootAttributes, { immediate: true })
</script>

<template>
  <div class="desk-app corevia-nuxt-app">
    <AmbientBackground />
    <a class="skip-link" href="#mainContent">{{ t('skipToContent') }}</a>

    <div class="app-shell" :class="{ 'rail-collapsed': railCollapsed }">
      <DeskTopbar
        :profile="profile"
        :home-path="homePath"
        @toggle-theme="toggleTheme"
        @toggle-locale="toggleLocale"
      />

      <div class="workspace-grid">
        <DeskSidebar
          :profile="profile"
          :active-page="activePage"
          :collapsed="railCollapsed"
          @toggle-rail="railCollapsed = !railCollapsed"
        />

        <main id="mainContent" class="main-column" tabindex="-1">
          <slot />
        </main>
      </div>

      <nav class="mobile-nav glass-surface" aria-label="Mobile navigation">
        <NuxtLink class="mobile-nav-item" :class="{ 'is-active': activePage === 'home' }" :to="homePath" :external="profile === 'customer-support'">
          <IconGlyph name="home" />
          <span>{{ t('home') }}</span>
        </NuxtLink>
        <NuxtLink v-if="profile === 'full-desk'" class="mobile-nav-item" :class="{ 'is-active': activePage === 'devices' }" to="/devices">
          <IconGlyph name="devices" />
          <span>{{ t('devices') }}</span>
        </NuxtLink>
        <NuxtLink v-if="profile === 'full-desk'" class="mobile-nav-item" :class="{ 'is-active': activePage === 'sessions' }" to="/sessions">
          <IconGlyph name="sessions" />
          <span>{{ t('sessions') }}</span>
        </NuxtLink>
        <NuxtLink class="mobile-nav-item" :class="{ 'is-active': activePage === 'settings' }" :to="settingsPath" :external="profile === 'customer-support'">
          <IconGlyph name="settings" />
          <span>{{ t('settings') }}</span>
        </NuxtLink>
      </nav>
    </div>

    <DeskNotificationRegion />
  </div>
</template>
