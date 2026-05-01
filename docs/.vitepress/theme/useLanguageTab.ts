const STORAGE_KEY = 'relateby-lang-tab'

export function applyLanguageTab(): void {
  if (typeof window === 'undefined') return
  const saved = localStorage.getItem(STORAGE_KEY)
  if (!saved) return
  document.querySelectorAll<HTMLButtonElement>('.vp-code-group .tabs button').forEach((btn) => {
    if (btn.textContent?.trim() === saved) {
      btn.click()
    }
  })
}

export function registerLanguageTabListeners(): void {
  if (typeof window === 'undefined') return
  document.addEventListener('click', (event) => {
    const target = event.target as HTMLElement
    if (target.matches('.vp-code-group .tabs button')) {
      const label = target.textContent?.trim()
      if (label) {
        localStorage.setItem(STORAGE_KEY, label)
      }
    }
  })
}
