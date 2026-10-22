const runtimeEnvironment = (globalThis as typeof globalThis & {
  process?: { env?: Record<string, string | undefined> }
}).process?.env

const buildProfile = runtimeEnvironment?.COREVIA_DESK_PROFILE === 'customer-support'
  ? 'customer-support'
  : 'full-desk'

export default defineNuxtConfig({
  ssr: false,
  devtools: { enabled: false },
  components: [{ path: '~/components', pathPrefix: false }],
  css: [
    '~/assets/vendor/fontawesome/css/all.min.css',
    '~/assets/css/corevia-desk.css',
    '~/assets/css/corevia-nuxt.css'
  ],
  runtimeConfig: {
    public: {
      deskProfile: buildProfile
    }
  },
  app: {
    head: {
      htmlAttrs: {
        lang: 'fa',
        dir: 'rtl'
      },
      meta: [
        { name: 'color-scheme', content: 'dark light' },
        {
          name: 'description',
          content: 'Corevia Desk — رابط دسکتاپ Corevia با هسته‌ی Rust.'
        }
      ],
      title: 'Corevia Desk'
    }
  },
  typescript: {
    strict: true,
    typeCheck: false
  }
})
