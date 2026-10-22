<script setup lang="ts">
defineProps<{
  label: string
  value: string
  icon: string
  tone?: 'core' | 'cyan' | 'purple'
}>()
</script>

<template>
  <article class="stat-card glass-card corevia-stat-card">
    <span class="stat-icon" :class="`stat-icon-${tone ?? 'core'}`">
      <IconGlyph :name="icon" />
    </span>
    <span class="stat-copy">
      <small>{{ label }}</small>
      <strong>{{ value }}</strong>
    </span>
  </article>
</template>
