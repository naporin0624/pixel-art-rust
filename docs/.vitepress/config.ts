import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Pixel Art Rust',
  description: 'Convert images to pixel art with advanced color quantization',
  base: '/pixel-art-rust/',
  
  themeConfig: {
    logo: '/logo.svg',
    
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/getting-started' },
      { text: 'API', link: '/api/cli' },
      { text: 'Algorithms', link: '/algorithms/overview' }
    ],
    
    sidebar: {
      '/guide/': [
        {
          text: 'Guide',
          items: [
            { text: 'Getting Started', link: '/guide/getting-started' },
            { text: 'Installation', link: '/guide/installation' },
            { text: 'Usage', link: '/guide/usage' },
            { text: 'Examples', link: '/guide/examples' }
          ]
        }
      ],
      '/api/': [
        {
          text: 'API Reference',
          items: [
            { text: 'CLI', link: '/api/cli' },
            { text: 'Core Library', link: '/api/core' },
            { text: 'Algorithms', link: '/api/algorithms' }
          ]
        }
      ],
      '/algorithms/': [
        {
          text: 'Algorithms',
          items: [
            { text: 'Overview', link: '/algorithms/overview' },
            { text: 'Average Color', link: '/algorithms/average-color' },
            { text: 'Median Cut', link: '/algorithms/median-cut' },
            { text: 'K-Means', link: '/algorithms/kmeans' },
            { text: 'Quadtree', link: '/algorithms/quadtree' }
          ]
        }
      ]
    },
    
    socialLinks: [
      { icon: 'github', link: 'https://github.com/naporin0624/pixel-art-rust' }
    ],
    
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present'
    },
    
    search: {
      provider: 'local'
    }
  }
})