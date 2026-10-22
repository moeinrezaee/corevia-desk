<script setup lang="ts">
withDefaults(defineProps<{
  as?: string
}>(), {
  as: 'article'
})
</script>

<template>
  <component :is="as" class="glass-card">
    <slot />
  </component>
</template>
