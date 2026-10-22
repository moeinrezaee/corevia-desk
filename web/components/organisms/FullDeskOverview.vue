<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useCoreviaBridge } from '~/composables/useCoreviaBridge'
import { useDeskBridgeFeedback } from '~/composables/useDeskBridgeFeedback'
import { useDeskRuntime } from '~/composables/useDeskRuntime'
import type { DeskDevice } from '~/types/desk'

const emit = defineEmits<{
  connect: [device: DeskDevice]
  permissions: []
}>()

const { t } = useDeskCopy()
const { request } = useCoreviaBridge()
const { isBridgeActionSuccessful, notifyFailure } = useDeskBridgeFeedback()
const { devices, refresh } = useDeskRuntime()
const recentDevices = computed(() => devices.value
  .filter((device) => (device.lastSeenAt ?? 0) > 0)
  .slice(0, 5))
const otherRecentDevices = computed(() => recentDevices.value.slice(1))

onMounted(() => refresh())

const connectDevice = async (device: DeskDevice) => {
  const response = await request(
    'start-connection',
    { remoteId: device.address },
    'nuxt-home-connect-approval'
  )
  if (isBridgeActionSuccessful(response)) {
    emit('connect', device)
  } else {
    notifyFailure(response)
  }
}
</script>

<template>
  <div class="corevia-page-stack">
    <div v-if="recentDevices.length" class="home-summary-grid">
      <DeviceCard :device="recentDevices[0]" featured @connect="connectDevice" />
        <article class="glass-card compact-actions-card">
        <div class="card-heading">
          <div>
            <span class="eyebrow eyebrow-muted">{{ t('quickActions') }}</span>
            <h2>{{ t('startHere') }}</h2>
          </div>
        </div>
        <div class="compact-action-list">
          <button class="compact-action" type="button" @click="connectDevice(recentDevices[0])">
            <span class="quick-action-icon quick-action-icon-core"><IconGlyph name="plug" /></span>
            <span><strong>{{ t('newConnection') }}</strong><small>{{ t('newConnectionHelp') }}</small></span>
            <IconGlyph name="arrow" />
          </button>
          <NuxtLink class="compact-action" to="/devices">
            <span class="quick-action-icon quick-action-icon-purple"><IconGlyph name="devices" /></span>
            <span><strong>{{ t('openAddressBook') }}</strong><small>{{ t('openAddressBookHelp') }}</small></span>
            <IconGlyph name="arrow" />
          </NuxtLink>
          <button class="compact-action" type="button" @click="emit('permissions')">
            <span class="quick-action-icon quick-action-icon-cyan"><IconGlyph name="shield" /></span>
            <span><strong>{{ t('checkPermissions') }}</strong><small>{{ t('checkPermissionsHelp') }}</small></span>
            <IconGlyph name="arrow" />
          </button>
        </div>
      </article>
    </div>
    <div v-else class="corevia-empty-state">
      <span><i class="fa-solid fa-address-book corevia-icon-glyph" aria-hidden="true" /><strong>{{ devices.length ? t('noRecentConnections') : t('noDevices') }}</strong></span>
      <NuxtLink v-if="devices.length" class="text-button" to="/devices">{{ t('openAddressBook') }}</NuxtLink>
    </div>

    <div v-if="otherRecentDevices.length" class="section-heading corevia-section-heading">
      <div>
        <span class="eyebrow eyebrow-muted">{{ t('recentDesktops') }}</span>
        <h2>{{ t('recentDesktopsTitle') }}</h2>
      </div>
      <NuxtLink class="text-button" to="/devices">
        <span>{{ t('viewAll') }}</span>
        <IconGlyph name="arrow" />
      </NuxtLink>
    </div>

    <div v-if="otherRecentDevices.length" class="recent-desktops-grid corevia-recent-grid">
      <DeviceCard
        v-for="device in otherRecentDevices"
        :key="device.id"
        :device="device"
        show-action-icon
        :show-latency="device.latency !== '—'"
        :context="device.lastSeen"
        @connect="connectDevice"
      />
    </div>
  </div>
</template>
