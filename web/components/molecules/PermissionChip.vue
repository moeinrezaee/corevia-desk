<script setup lang="ts">
defineProps<{
  label: string
  icon: string
}>()
</script>

<template>
  <span class="access-chip corevia-access-chip">
    <IconGlyph :name="icon" />
    <span>{{ label }}</span>
  </span>
</template>
