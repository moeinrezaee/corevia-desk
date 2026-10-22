<script setup lang="ts">
import { useDeskProfile } from '~/composables/useDeskProfile'

const { profile } = useDeskProfile()
</script>

<template>
  <DeskWorkspaceTemplate :profile="profile" active-page="settings">
    <DeskSettingsTemplate :profile="profile" />
  </DeskWorkspaceTemplate>
</template>
