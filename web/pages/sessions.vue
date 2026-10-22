<script setup lang="ts">
import { useDeskProfile } from '~/composables/useDeskProfile'

const { profile } = useDeskProfile()

if (profile.value === 'customer-support') {
  await navigateTo('/customer-support')
}
</script>

<template>
  <DeskWorkspaceTemplate :profile="profile" active-page="sessions">
    <DeskSessionsTemplate :profile="profile" />
  </DeskWorkspaceTemplate>
</template>
