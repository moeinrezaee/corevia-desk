<script setup lang="ts">
const emit = defineEmits<{
  approved: [permissions: string[]]
  denied: []
}>()
</script>

<template>
  <div class="corevia-page-stack">
    <CustomerRequestPanel
      @approved="emit('approved', $event)"
      @denied="emit('denied')"
    />
  </div>
</template>
