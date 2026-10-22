<script setup lang="ts">
import { computed } from 'vue'
import { useDeskAppearance } from '~/composables/useDeskAppearance'
import { useDeskCopy } from '~/composables/useDeskCopy'
import type { DeskDevice } from '~/types/desk'
import labDark from '~/assets/media/desktop-lab.svg'
import labLight from '~/assets/media/desktop-lab-light.svg'
import homeDark from '~/assets/media/desktop-home.svg'
import homeLight from '~/assets/media/desktop-home-light.svg'
import serverDark from '~/assets/media/desktop-server.svg'
import serverLight from '~/assets/media/desktop-server-light.svg'
import studioDark from '~/assets/media/desktop-studio.svg'
import studioLight from '~/assets/media/desktop-studio-light.svg'

const props = withDefaults(defineProps<{
  device: DeskDevice
  featured?: boolean
  showActionIcon?: boolean
  showFavorite?: boolean
  favorite?: boolean
  showLatency?: boolean
  context?: string
}>(), {
  featured: false,
  showActionIcon: false,
  showFavorite: false,
  favorite: false,
  showLatency: true,
  context: undefined
})

const emit = defineEmits<{
  connect: [device: DeskDevice]
  toggleFavorite: [device: DeskDevice]
}>()

const { theme } = useDeskAppearance()
const { t } = useDeskCopy()

const media = {
  lab: { dark: labDark, light: labLight },
  home: { dark: homeDark, light: homeLight },
  server: { dark: serverDark, light: serverLight },
  studio: { dark: studioDark, light: studioLight }
}

const image = computed(() => media[props.device.media][theme.value])

const platformMeta = computed(() => {
  const platform = props.device.platform.toLowerCase()

  if (platform.includes('mac')) {
    return {
      visualClass: 'os-visual-macos',
      brandClass: 'fa-brands fa-apple',
      avatarClass: 'device-avatar-purple',
      avatarIcon: 'fa-solid fa-laptop'
    }
  }

  if (platform.includes('linux')) {
    return {
      visualClass: 'os-visual-linux',
      brandClass: 'fa-brands fa-linux',
      avatarClass: 'device-avatar-cyan',
      avatarIcon: 'fa-solid fa-desktop'
    }
  }

  return {
    visualClass: 'os-visual-windows',
    brandClass: 'fa-brands fa-windows',
    avatarClass: 'device-avatar-green',
    avatarIcon: 'fa-solid fa-server'
  }
})

const actionIcon = computed(() => props.featured ? 'external' : (props.showActionIcon ? 'arrow' : undefined))
</script>

<template>
  <article
    class="device-card device-surface-card glass-card"
    :class="{ 'resume-card': featured }"
    data-visual-mode="os"
    :data-device-status="device.status"
    :data-device-name="device.name"
    :data-device-favorite="favorite"
  >
    <div class="device-card-media os-visual" :class="platformMeta.visualClass" aria-hidden="true">
      <img class="os-visual-fallback-image" :src="image" alt="" />
      <span class="os-visual-glow" />
      <span class="os-visual-icon"><i :class="platformMeta.brandClass" /></span>
      <span class="os-visual-label">{{ device.platform }}</span>
    </div>

    <div class="device-card-top">
      <span class="device-avatar" :class="platformMeta.avatarClass">
        <i :class="platformMeta.avatarIcon" />
      </span>
      <div class="device-card-copy">
        <span v-if="featured" class="device-card-kicker">{{ t('resumeTitle') }}</span>
        <h2 class="device-name" dir="ltr">{{ device.name }}</h2>
        <p>{{ device.platform }} · <span dir="ltr">{{ device.address }}</span></p>
      </div>
      <span v-if="showLatency || context" class="device-card-context" :dir="showLatency ? 'ltr' : undefined">
        <i v-if="showLatency" class="fa-solid fa-gauge-high" />{{ context ?? device.latency }}
      </span>
      <button v-if="showFavorite" class="card-icon-button" :class="{ 'is-favorite': favorite }" type="button" :aria-label="t('favorite')" :title="t('favorite')" @click="emit('toggleFavorite', device)">
        <i :class="favorite ? 'fa-solid fa-star' : 'fa-regular fa-star'" aria-hidden="true" />
      </button>
    </div>

    <div class="device-card-tags">
      <span class="tag-chip">{{ device.tag }}</span>
      <StatusLabel :online="device.status === 'online'">
        {{ device.status === 'online' ? t('online') : t('offline') }}
      </StatusLabel>
    </div>

    <div class="device-card-footer">
      <span>
        <template v-if="featured">
          <i class="fa-solid fa-lock" />{{ t('encrypted') }}<span aria-hidden="true"> · </span>
        </template>
        {{ device.lastSeen }}
      </span>
      <BaseButton variant="secondary" size="small" @click="emit('connect', device)">
        <template v-if="featured">
          <IconGlyph v-if="actionIcon" :name="actionIcon" />
          <span>{{ t('resumeSession') }}</span>
        </template>
        <template v-else>
          <span>{{ t('connect') }}</span>
          <IconGlyph v-if="actionIcon" :name="actionIcon" />
        </template>
      </BaseButton>
    </div>
  </article>
</template>
