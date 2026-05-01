import DefaultTheme from 'vitepress/theme'
import type { Theme } from 'vitepress'
import { applyLanguageTab, registerLanguageTabListeners } from './useLanguageTab'

const theme: Theme = {
  extends: DefaultTheme,
  enhanceApp({ router }) {
    if (typeof window === 'undefined') return
    registerLanguageTabListeners()
    router.onAfterRouteChange = () => {
      setTimeout(applyLanguageTab, 50)
    }
  },
}

export default theme
