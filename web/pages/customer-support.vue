<script setup lang="ts">
const { profile } = useDeskProfile()
</script>

<template>
  <DeskWorkspaceTemplate :profile="profile" active-page="home">
    <DeskDashboardTemplate :profile="profile" />
  </DeskWorkspaceTemplate>
</template>
