// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2025-05-15',
  devtools: { enabled: false },
  ssr: true,
  modules: ['@nuxt/eslint', '@nuxt/icon', '@nuxt/image', '@nuxtjs/tailwindcss'],
  css: ['~/assets/css/main.css'],
  devServer: {
    port: 32030,
    host: 'localhost',
  },
  icon: {
    mode: 'local',
    collections: ['heroicons'],
    fallbackToApi: false,
    clientBundle: {
      scan: true,
      sizeLimitKb: 256,
    },
  },
  vite: {
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    server: {
      strictPort: true,
      watch: {
        ignored: ['**/src-tauri/**'],
      },
    },
  },
});
