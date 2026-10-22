<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useDeskCopy } from '~/composables/useDeskCopy'
import { useCoreviaBridge } from '~/composables/useCoreviaBridge'
import { useDeskBridgeFeedback } from '~/composables/useDeskBridgeFeedback'
import { useDeskRuntime } from '~/composables/useDeskRuntime'
import type { DeskDevice } from '~/types/desk'

defineProps<{
  profile: 'customer-support' | 'full-desk'
}>()

interface DeviceFixture extends DeskDevice {
  favorite: boolean
}

const emit = defineEmits<{
  connect: [device: DeskDevice]
}>()

const { t } = useDeskCopy()
const { request } = useCoreviaBridge()
const { isBridgeActionSuccessful, notifyFailure } = useDeskBridgeFeedback()
const { devices, refresh } = useDeskRuntime()
const search = ref('')
const activeFilter = ref<'all' | 'online' | 'favorite'>('all')
const showAddDevice = ref(false)
const newDeviceName = ref('')
const newDeviceAddress = ref('')
const newDevicePassword = ref('')
const newDeviceTag = ref('')
const formError = ref('')

onMounted(() => refresh())

const closeAddDevice = () => {
  showAddDevice.value = false
  newDeviceName.value = ''
  newDeviceAddress.value = ''
  newDevicePassword.value = ''
  newDeviceTag.value = ''
  formError.value = ''
}

const localizedDevices = computed<DeviceFixture[]>(() => devices.value.map((device) => ({
  ...device,
  favorite: device.favorite ?? false
})))

const filteredDevices = computed(() => {
  const needle = search.value.trim().toLowerCase()
  return localizedDevices.value.filter((device) => {
    const matchesFilter = activeFilter.value === 'all'
      || (activeFilter.value === 'online' && device.status === 'online')
      || (activeFilter.value === 'favorite' && device.favorite)
    const matchesSearch = !needle || `${device.name} ${device.address} ${device.tag}`.toLowerCase().includes(needle)
    return matchesFilter && matchesSearch
  })
})

const toggleFavorite = async (device: DeskDevice) => {
  const item = localizedDevices.value.find((candidate) => candidate.id === device.id)
  if (!item) return
  const response = await request(
    'manage-peers',
    { peerId: item.id, action: item.favorite ? 'unfavorite' : 'favorite' },
    'nuxt-device-favorite-approval'
  )
  if (isBridgeActionSuccessful(response)) {
    await refresh(true)
  } else {
    notifyFailure(response)
  }
}

const connectDevice = async (device: DeskDevice) => {
  const response = await request(
    'start-connection',
    { remoteId: device.address },
    'nuxt-device-connect-approval'
  )
  if (isBridgeActionSuccessful(response)) {
    emit('connect', device)
  } else {
    notifyFailure(response)
  }
}

const addDevice = async () => {
  const name = newDeviceName.value.trim()
  const address = newDeviceAddress.value.trim()
  const tag = newDeviceTag.value.trim()
  if (!name || !address) {
    formError.value = t('deviceFieldsRequired')
    return
  }
  formError.value = ''

  const response = await request(
    'manage-peers',
    {
      peerId: address,
      action: 'save',
      name,
      platform: '',
      tag,
      password: newDevicePassword.value
    },
    'nuxt-device-save-approval'
  )
  if (!isBridgeActionSuccessful(response)) {
    notifyFailure(response)
    return
  }
  await refresh(true)
  closeAddDevice()
}
</script>

<template>
  <section class="view-panel corevia-devices-page corevia-page-stack">
    <div class="screen-header">
      <div>
        <span class="eyebrow"><span class="eyebrow-dot" />COREVIA DESK / {{ t('devices') }}</span>
        <h1>{{ t('devicesTitle') }}</h1>
        <p>{{ t('devicesDescription') }}</p>
      </div>
      <button class="button button-primary" type="button" @click="showAddDevice = true">
        <i class="fa-solid fa-plus" aria-hidden="true" />
        <span>{{ t('addDesktop') }}</span>
      </button>
    </div>

    <div class="device-toolbar glass-card">
      <label class="toolbar-search">
        <i class="fa-solid fa-magnifying-glass" aria-hidden="true" />
        <span class="sr-only">{{ t('searchDevices') }}</span>
        <input v-model="search" type="search" :placeholder="t('searchDevicesPlaceholder')" />
      </label>
      <div class="filter-pills" role="group" :aria-label="t('deviceFilters')">
        <button class="filter-pill" :class="{ 'is-active': activeFilter === 'all' }" type="button" @click="activeFilter = 'all'">{{ t('allDevices') }}</button>
        <button class="filter-pill" :class="{ 'is-active': activeFilter === 'online' }" type="button" @click="activeFilter = 'online'">{{ t('onlineOnly') }}</button>
        <button class="filter-pill" :class="{ 'is-active': activeFilter === 'favorite' }" type="button" @click="activeFilter = 'favorite'">{{ t('favoritesOnly') }}</button>
      </div>
    </div>

    <div v-if="filteredDevices.length" class="device-grid">
      <DeviceCard
        v-for="device in filteredDevices"
        :key="device.id"
        :device="device"
        :favorite="device.favorite"
        show-favorite
        :show-latency="false"
        @connect="connectDevice"
        @toggle-favorite="toggleFavorite"
      />
      <button class="add-device-card glass-card" type="button" @click="showAddDevice = true">
        <span class="add-device-icon"><i class="fa-solid fa-plus" aria-hidden="true" /></span>
        <strong>{{ t('addDesktopCard') }}</strong>
        <small>{{ t('addDesktopCardHelp') }}</small>
      </button>
    </div>
    <div v-else class="corevia-empty-state">
      <span><i class="fa-solid fa-address-book corevia-icon-glyph" aria-hidden="true" /><strong>{{ t('noDevices') }}</strong></span>
    </div>

    <Teleport to="body">
      <div v-if="showAddDevice" class="modal-overlay" @click.self="closeAddDevice">
        <section class="modal-dialog glass-surface" role="dialog" aria-modal="true" :aria-label="t('addDesktop')">
          <div class="modal-heading">
            <span class="modal-heading-icon"><i class="fa-solid fa-computer" aria-hidden="true" /></span>
            <div><h2>{{ t('addDesktop') }}</h2><p>{{ t('addDesktopDescription') }}</p></div>
            <button class="icon-button" type="button" :aria-label="t('closeDialog')" @click="closeAddDevice"><i class="fa-solid fa-xmark" aria-hidden="true" /></button>
          </div>
          <form class="add-device-form" @submit.prevent="addDevice">
            <label class="field-label"><span>{{ t('deviceName') }}</span><input v-model="newDeviceName" class="text-input" :aria-invalid="formError && !newDeviceName.trim() ? 'true' : undefined" :placeholder="t('deviceNamePlaceholder')" @input="formError = ''" /></label>
            <label class="field-label"><span>{{ t('deviceId') }}</span><input v-model="newDeviceAddress" class="text-input" dir="ltr" :aria-invalid="formError && !newDeviceAddress.trim() ? 'true' : undefined" :placeholder="t('deviceIdPlaceholder')" @input="formError = ''" /></label>
            <label class="field-label"><span>{{ t('devicePassword') }}</span><input v-model="newDevicePassword" class="text-input" type="password" dir="ltr" :placeholder="t('devicePasswordPlaceholder')" /></label>
            <label class="field-label"><span>{{ t('deviceTag') }}</span><input v-model="newDeviceTag" class="text-input" :placeholder="t('deviceTagPlaceholder')" /></label>
            <p v-if="formError" class="corevia-form-error" role="alert">{{ formError }}</p>
            <div class="modal-actions"><button class="button button-secondary" type="button" @click="closeAddDevice">{{ t('cancel') }}</button><button class="button button-primary" type="submit"><i class="fa-solid fa-plus" aria-hidden="true" /><span>{{ t('saveDevice') }}</span></button></div>
          </form>
        </section>
      </div>
    </Teleport>
  </section>
</template>
