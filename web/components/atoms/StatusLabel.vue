<script setup lang="ts">
withDefaults(defineProps<{
  online?: boolean
}>(), {
  online: true
})
</script>

<template>
  <span :class="online ? 'device-online' : 'device-offline'">
    <StatusDot v-if="online" />
    <span v-else class="status-dot status-dot-offline" aria-hidden="true" />
    <slot />
  </span>
</template>
