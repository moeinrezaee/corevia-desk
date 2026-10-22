import type { DeskProfile } from '~/types/desk'

export const normalizeDeskProfile = (value: unknown): DeskProfile => {
  return value === 'customer-support' ? 'customer-support' : 'full-desk'
}

export const useDeskProfile = () => {
  const config = useRuntimeConfig()
  const route = useRoute()
  const configuredProfile = normalizeDeskProfile(config.public.deskProfile)
  const profile = computed<DeskProfile>(() => {
    // A customer-support build stays customer-only even if a full-desk URL is opened.
    if (configuredProfile === 'customer-support') return 'customer-support'

    const requestedProfile = Array.isArray(route.query.profile)
      ? route.query.profile[0]
      : route.query.profile

    if (requestedProfile === 'customer-support' || route.path === '/customer-support') {
      return 'customer-support'
    }

    if (route.path === '/full-desk') return 'full-desk'
    return configuredProfile
  })
  const isCustomerSupport = computed(() => profile.value === 'customer-support')

  return {
    profile,
    isCustomerSupport,
    isFullDesk: computed(() => !isCustomerSupport.value)
  }
}
