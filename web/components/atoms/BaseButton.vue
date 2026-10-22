<script setup lang="ts">
import { computed } from 'vue'

type ButtonVariant = 'primary' | 'secondary' | 'text'
type ButtonSize = 'normal' | 'small'

const props = withDefaults(defineProps<{
  variant?: ButtonVariant
  size?: ButtonSize
  type?: 'button' | 'submit' | 'reset'
  disabled?: boolean
}>(), {
  variant: 'secondary',
  size: 'normal',
  type: 'button',
  disabled: false
})

const classes = computed(() => [
  'button',
  `button-${props.variant}`,
  props.size === 'small' ? 'button-small' : undefined
])
</script>

<template>
  <button
    v-bind="$attrs"
    :class="classes"
    :disabled="disabled"
    :type="type"
  >
    <slot />
  </button>
</template>
