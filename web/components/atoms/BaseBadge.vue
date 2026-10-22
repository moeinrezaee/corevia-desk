<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  tone?: 'success' | 'neutral' | 'purple'
  small?: boolean
  dot?: boolean
}>(), {
  tone: 'success',
  small: false,
  dot: true
})

const classes = computed(() => [
  'status-pill',
  `status-pill-${props.tone}`,
  props.small ? 'status-pill-small' : undefined
])
</script>

<template>
  <span :class="classes">
    <StatusDot v-if="dot" />
    <slot />
  </span>
</template>
