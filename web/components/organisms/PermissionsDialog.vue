<script setup lang="ts">
import { useDeskCopy } from '~/composables/useDeskCopy'

defineProps<{
  open: boolean
}>()

const { t } = useDeskCopy()

const emit = defineEmits<{
  close: []
}>()
</script>

<template>
  <Teleport to="body">
    <div v-if="open" class="modal-overlay permissions-overlay" @keydown.esc="emit('close')">
      <div class="modal-backdrop" aria-hidden="true" @click="emit('close')" />
      <section class="modal-dialog permissions-dialog glass-surface" role="dialog" aria-modal="true" :aria-label="t('permissionsTitle')">
        <button class="icon-button permission-dialog-close" type="button" :aria-label="t('closeDialog')" :title="t('closeDialog')" @click="emit('close')">
          <i class="fa-solid fa-xmark" aria-hidden="true" />
        </button>

        <div class="permission-page-heading">
          <button class="back-button" type="button" @click="emit('close')">
            <i class="fa-solid fa-arrow-right-long" aria-hidden="true" />
            <span>{{ t('back') }}</span>
          </button>
          <div class="screen-header">
            <div>
              <span class="eyebrow"><span class="eyebrow-dot" />COREVIA DESK / {{ t('permissions') }}</span>
              <h1>{{ t('permissionsTitle') }}</h1>
              <p>{{ t('permissionsDescription') }}</p>
            </div>
          </div>
        </div>

        <div class="permission-layout">
          <article class="glass-card permission-card">
            <div class="permission-card-heading">
              <div><span class="eyebrow eyebrow-muted">{{ t('requiredPermissions') }}</span><h2>{{ t('permissionCheckTitle') }}</h2></div>
            </div>
            <p>{{ t('permissionRuntimeCheckNote') }}</p>
            <div class="permission-list">
              <div class="permission-item">
                <span class="permission-state"><i class="fa-solid fa-keyboard" aria-hidden="true" /></span>
                <span><strong>{{ t('accessibilityPermission') }}</strong><small>{{ t('accessibilityPermissionHelp') }}</small></span>
              </div>
              <div class="permission-item">
                <span class="permission-state"><i class="fa-solid fa-display" aria-hidden="true" /></span>
                <span><strong>{{ t('screenRecordingPermission') }}</strong><small>{{ t('screenRecordingPermissionHelp') }}</small></span>
              </div>
              <div class="permission-item">
                <span class="permission-state"><i class="fa-solid fa-computer-mouse" aria-hidden="true" /></span>
                <span><strong>{{ t('inputMonitoringPermission') }}</strong><small>{{ t('inputMonitoringPermissionHelp') }}</small></span>
              </div>
            </div>
            <div class="permission-actions">
              <button class="button button-primary" type="button" @click="emit('close')">{{ t('permissionGuidanceDone') }}</button>
            </div>
          </article>

          <aside class="permission-note glass-card">
            <span class="permission-note-icon"><i class="fa-solid fa-lock" aria-hidden="true" /></span>
            <h2>{{ t('whyPermissions') }}</h2>
            <p>{{ t('whyPermissionsDescription') }}</p>
          </aside>
        </div>
      </section>
    </div>
  </Teleport>
</template>
