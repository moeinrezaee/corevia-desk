<script setup lang="ts">
import { computed } from 'vue'
import { useDeskCopy } from '~/composables/useDeskCopy'
import type { DeskProfile } from '~/types/desk'

const props = defineProps<{
  profile: DeskProfile
}>()

const { t } = useDeskCopy()

const capabilities = computed(() => props.profile === 'customer-support'
  ? [
      { label: t('screenShare'), icon: 'screen', allowed: true },
      { label: t('keyboardMouse'), icon: 'input', allowed: true },
      { label: t('fileTransfer'), icon: 'files', allowed: false }
    ]
  : [
      { label: t('screenShare'), icon: 'screen', allowed: true },
      { label: t('keyboardMouse'), icon: 'input', allowed: true },
      { label: t('fileTransfer'), icon: 'files', allowed: true },
      { label: t('terminal'), icon: 'terminal', allowed: true }
    ])

const allowedCapabilityCount = computed(() => capabilities.value.filter((capability) => capability.allowed).length)
const capabilityProgress = computed(() => `${Math.round((allowedCapabilityCount.value / capabilities.value.length) * 100)}%`)
</script>

<template>
  <aside class="inspector-column" aria-label="Corevia Desk status">
    <div class="inspector-intro">
      <span class="eyebrow eyebrow-muted">COREVIA DESK / STATUS</span>
      <h2>{{ profile === 'customer-support' ? t('customerSupport') : t('fullDesk') }}</h2>
      <p>{{ profile === 'customer-support' ? t('receiveOnly') : t('serverConnected') }}</p>
    </div>

    <article class="inspector-card glass-card inspector-card-primary corevia-inspector-card">
      <div class="inspector-card-top">
        <span class="inspector-card-heading">
          <span class="inspector-card-icon"><IconGlyph :name="profile === 'customer-support' ? 'shield' : 'activity'" /></span>
          <span>
            <strong>{{ t('security') }}</strong>
            <small>{{ profile === 'customer-support' ? t('customerSecurityNote') : t('connectionHint') }}</small>
          </span>
        </span>
        <BaseBadge small>{{ profile === 'customer-support' ? 'ACL' : 'OK' }}</BaseBadge>
      </div>

      <div class="inspector-progress">
        <span :style="{ width: capabilityProgress }" />
      </div>
      <div class="inspector-progress-meta">
        <span>{{ t('security') }}</span>
        <strong>{{ allowedCapabilityCount }} / {{ capabilities.length }}</strong>
      </div>

      <div class="corevia-capability-list">
        <span
          v-for="capability in capabilities"
          :key="capability.label"
          class="corevia-capability-chip"
          :class="capability.allowed ? 'is-allowed' : 'is-locked'"
        >
          <IconGlyph :name="capability.icon" />
          {{ capability.label }}
        </span>
      </div>

      <p class="inspector-footnote">
        <IconGlyph name="lock" />
        {{ profile === 'customer-support' ? t('customerLocked') : t('connectionHint') }}
      </p>
    </article>
  </aside>
</template>
