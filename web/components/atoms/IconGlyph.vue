<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  name: string
}>()

const iconClasses: Record<string, string> = {
  home: 'fa-solid fa-house',
  sessions: 'fa-solid fa-display',
  devices: 'fa-solid fa-address-book',
  settings: 'fa-solid fa-sliders',
  network: 'fa-solid fa-network-wired',
  connection: 'fa-solid fa-arrow-right-arrow-left',
  plug: 'fa-solid fa-plug',
  shield: 'fa-solid fa-shield-halved',
  screen: 'fa-solid fa-display',
  input: 'fa-solid fa-keyboard',
  files: 'fa-regular fa-folder-open',
  copy: 'fa-regular fa-copy',
  eye: 'fa-regular fa-eye',
  eyeSlash: 'fa-regular fa-eye-slash',
  server: 'fa-solid fa-server',
  clock: 'fa-regular fa-clock',
  activity: 'fa-solid fa-gauge-high',
  arrow: 'fa-solid fa-arrow-left-long',
  plus: 'fa-solid fa-plus',
  close: 'fa-solid fa-xmark',
  check: 'fa-solid fa-check',
  lock: 'fa-solid fa-lock',
  language: 'fa-solid fa-language',
  moon: 'fa-solid fa-moon',
  sun: 'fa-solid fa-sun',
  menu: 'fa-solid fa-bars',
  more: 'fa-solid fa-ellipsis',
  terminal: 'fa-solid fa-terminal',
  audio: 'fa-solid fa-volume-high',
  clipboard: 'fa-regular fa-clipboard',
  monitor: 'fa-solid fa-computer',
  support: 'fa-solid fa-headset',
  info: 'fa-solid fa-circle-info',
  windows: 'fa-brands fa-windows',
  apple: 'fa-brands fa-apple',
  linux: 'fa-brands fa-linux',
  laptop: 'fa-solid fa-laptop',
  desktop: 'fa-solid fa-desktop',
  database: 'fa-solid fa-database',
  star: 'fa-solid fa-star',
  starOutline: 'fa-regular fa-star',
  collapse: 'fa-solid fa-angles-left',
  collapseRight: 'fa-solid fa-angles-right',
  expand: 'fa-solid fa-expand',
  chevronLeft: 'fa-solid fa-chevron-left',
  chevronRight: 'fa-solid fa-chevron-right',
  resolution: 'fa-solid fa-up-right-and-down-left-from-center',
  cursor: 'fa-solid fa-arrow-pointer',
  search: 'fa-solid fa-magnifying-glass',
  folder: 'fa-regular fa-folder-open',
  power: 'fa-solid fa-power-off',
  rotate: 'fa-solid fa-rotate',
  external: 'fa-solid fa-arrow-up-right-from-square',
  unlock: 'fa-solid fa-unlock',
  exclamation: 'fa-solid fa-exclamation',
  question: 'fa-solid fa-circle-question',
  half: 'fa-solid fa-circle-half-stroke',
  arrowUp: 'fa-solid fa-arrow-up',
  zoom: 'fa-solid fa-up-right-and-down-left-from-center',
  arrowRight: 'fa-solid fa-arrow-right-long'
}

const classes = computed(() => iconClasses[props.name] ?? 'fa-solid fa-circle')
</script>

<template>
  <i :class="[classes, 'corevia-icon-glyph']" aria-hidden="true" />
</template>
