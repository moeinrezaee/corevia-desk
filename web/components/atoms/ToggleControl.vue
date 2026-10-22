<script setup lang="ts">
const props = withDefaults(defineProps<{
  modelValue?: boolean
  disabled?: boolean
}>(), {
  modelValue: false,
  disabled: false
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
}>()

const toggle = () => {
  if (!props.disabled) emit('update:modelValue', !props.modelValue)
}
</script>

<template>
  <button
    class="corevia-toggle"
    :class="{ 'is-on': modelValue }"
    :aria-checked="modelValue"
    :disabled="disabled"
    role="switch"
    type="button"
    @click="toggle"
  >
    <span class="sr-only">{{ modelValue ? 'On' : 'Off' }}</span>
  </button>
</template>
