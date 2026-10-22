<script setup lang="ts">
withDefaults(defineProps<{
  label: string
  description: string
  modelValue?: boolean
  control?: 'toggle' | 'text'
  value?: string
  disabled?: boolean
}>(), {
  modelValue: false,
  control: 'toggle',
  value: '',
  disabled: false
})

const emit = defineEmits<{
  'update:modelValue': [value: boolean]
  'update:value': [value: string]
}>()

const updateValue = (event: Event) => {
  emit('update:value', (event.target as HTMLInputElement).value)
}
</script>

<template>
  <div class="corevia-setting-row">
    <span>
      <strong>{{ label }}</strong>
      <small>{{ description }}</small>
    </span>

    <ToggleControl
      v-if="control === 'toggle'"
      :model-value="modelValue"
      :disabled="disabled"
      @update:model-value="emit('update:modelValue', $event)"
    />
    <input
      v-else
      class="text-input"
      :value="value"
      :disabled="disabled"
      @input="updateValue"
    />
  </div>
</template>
