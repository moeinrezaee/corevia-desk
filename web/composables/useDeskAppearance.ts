import type { DeskLocale, DeskTheme } from '~/types/desk'

export const useDeskAppearance = () => {
  const theme = useState<DeskTheme>('corevia-desk-theme', () => 'dark')
  const locale = useState<DeskLocale>('corevia-desk-locale', () => 'fa')

  const toggleTheme = () => {
    theme.value = theme.value === 'dark' ? 'light' : 'dark'
  }

  const toggleLocale = () => {
    locale.value = locale.value === 'fa' ? 'en' : 'fa'
  }

  const setTheme = (nextTheme: DeskTheme) => {
    theme.value = nextTheme
  }

  const setLocale = (nextLocale: DeskLocale) => {
    locale.value = nextLocale
  }

  return {
    theme,
    locale,
    toggleTheme,
    toggleLocale,
    setTheme,
    setLocale
  }
}
