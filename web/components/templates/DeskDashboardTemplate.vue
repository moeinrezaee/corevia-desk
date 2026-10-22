<script setup lang="ts">
import { ref } from 'vue'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useDeskNotifications, type DeskNotificationTone } from '~/composables/useDeskNotifications'
import type { DeskProfile } from '~/types/desk'

const props = defineProps<{
  profile: DeskProfile
}>()

const { t } = useDeskCopy()
const { push } = useDeskNotifications()
const quickConnectForm = ref<{ focusInput: () => void } | null>(null)
const showPermissions = ref(false)

const focusQuickConnect = () => quickConnectForm.value?.focusInput()

const announce = (message: string, tone: DeskNotificationTone = 'info') => {
  push(message, tone)
}

const openPermissions = () => {
  showPermissions.value = true
}
</script>

<template>
  <section id="homeView" class="view-panel corevia-home-page corevia-page-stack">
    <DashboardHero
      :profile="props.profile"
      @quick-connect="focusQuickConnect"
      @copied="announce(t('copied'), 'success')"
    />

    <QuickConnectForm
      v-if="props.profile === 'full-desk'"
      ref="quickConnectForm"
      @connect="announce(`${t('connectionRequested')} · ${$event}`, 'warning')"
    />

    <FullDeskOverview
      v-if="props.profile === 'full-desk'"
      @connect="announce(`${t('connectionRequested')} · ${$event.name}`, 'warning')"
      @permissions="openPermissions"
    />

    <CustomerSupportOverview
      v-else
      @approved="announce(t('requestAllowed'), 'success')"
      @denied="announce(t('requestDenied'), 'danger')"
    />

    <PermissionsDialog :open="showPermissions" @close="showPermissions = false" />

  </section>
</template>
