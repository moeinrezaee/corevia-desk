/**
 * Copy text in both a normal browser and the native WebView.  The native
 * scheme does not always expose navigator.clipboard, so keep a small DOM
 * fallback for the ID and other non-sensitive values shown by the UI.
 */
export const copyText = async (value: string): Promise<boolean> => {
  if (!import.meta.client || !value) return false

  try {
    if (navigator.clipboard?.writeText) {
      await navigator.clipboard.writeText(value)
      return true
    }
  } catch {
    // Continue with the WebView-compatible fallback below.
  }

  const textarea = document.createElement('textarea')
  textarea.value = value
  textarea.setAttribute('readonly', '')
  textarea.setAttribute('aria-hidden', 'true')
  textarea.style.position = 'fixed'
  textarea.style.top = '0'
  textarea.style.left = '-9999px'
  textarea.style.opacity = '0'
  document.body.appendChild(textarea)
  textarea.focus()
  textarea.select()
  textarea.setSelectionRange(0, textarea.value.length)

  let copied = false
  try {
    copied = document.execCommand('copy')
  } catch {
    copied = false
  } finally {
    textarea.remove()
  }
  return copied
}
