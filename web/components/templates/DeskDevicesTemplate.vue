<script setup lang="ts">
import type { DeskDevice, DeskProfile } from '~/types/desk'

defineProps<{
  profile: DeskProfile
}>()

const emit = defineEmits<{
  connect: [device: DeskDevice]
}>()
</script>

<template>
  <DevicesDirectory :profile="profile" @connect="emit('connect', $event)" />
</template>
