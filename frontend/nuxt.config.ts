// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  css: [
    "~/assets/css/tailwind.scss",
    "~/assets/css/globals.scss",
    "@fortawesome/fontawesome-svg-core/styles.css",
  ],
  ssr: false,
  postcss: {
    plugins: {
      tailwindcss: {},
      autoprefixer: {},
    },
  },
})
