<script setup lang="ts">
import type { DeskProfile } from '~/types/desk'

defineProps<{
  profile: DeskProfile
}>()
</script>

<template>
  <SettingsPanel :profile="profile" />
</template>
