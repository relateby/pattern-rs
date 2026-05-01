import { defineConfig } from 'vitepress'

const base = process.env.GITHUB_ACTIONS === 'true' ? '/pattern-rs/' : '/'

export default defineConfig({
  title: 'pattern-rs',
  base,
  description: 'Pattern<V> — a decorated sequence for Rust, Python, and TypeScript',

  // Reference sub-sites are generated static dirs, not VitePress pages.
  // contributor/** docs are for internal use only and not published.
  ignoreDeadLinks: [/^\/reference\//, /^http:\/\/localhost/],

  srcExclude: [
    'public/**',
    'contributor/**',
  ],

  themeConfig: {
    nav: [
      { text: 'Guides', link: '/guides/' },
      { text: 'Explanations', link: '/explanations/' },
      { text: 'Reference', link: '/reference/' },
    ],

    sidebar: {
      '/guides/': [
        {
          text: 'Guides',
          items: [
            { text: 'How do I create an atomic pattern?', link: '/guides/create-atomic-pattern' },
            { text: 'How do I create a pattern with elements?', link: '/guides/create-pattern-with-elements' },
            { text: 'How do I give a pattern a value?', link: '/guides/give-pattern-a-value' },
            { text: 'How do I parse Gram notation?', link: '/guides/parse-gram-notation' },
            { text: 'How do I serialize to Gram notation?', link: '/guides/serialize-gram-notation' },
            { text: 'How do I traverse pattern elements?', link: '/guides/traverse-pattern-elements' },
            { text: 'How do I map over pattern values?', link: '/guides/map-pattern-values' },
            { text: 'How do I build a graph from patterns?', link: '/guides/build-graph-from-patterns' },
            { text: 'How do I merge two patterns?', link: '/guides/merge-two-patterns' },
          ],
        },
      ],
      '/explanations/': [
        {
          text: 'Explanations',
          items: [
            { text: 'What is a Pattern?', link: '/explanations/what-is-pattern' },
            { text: 'What is a decorated sequence?', link: '/explanations/what-is-decorated-sequence' },
            { text: 'Why is Pattern not a tree?', link: '/explanations/why-pattern-not-tree' },
            { text: 'What is a Subject?', link: '/explanations/what-is-subject' },
            { text: 'What is Gram notation?', link: '/explanations/what-is-gram-notation' },
            { text: 'How does Gram notation relate to Pattern?', link: '/explanations/gram-notation-and-pattern' },
            { text: 'Atomic vs elements pattern', link: '/explanations/atomic-vs-elements-pattern' },
            { text: 'How do the three language bindings relate?', link: '/explanations/three-language-bindings' },
            { text: 'When should I use Pattern?', link: '/explanations/when-to-use-pattern' },
            { text: 'What does the V in Pattern<V> mean?', link: '/explanations/what-is-v-in-pattern' },
          ],
        },
      ],
    },

    search: {
      provider: 'local',
    },
  },

  markdown: {
    lineNumbers: true,
  },
})
