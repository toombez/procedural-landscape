// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    compatibilityDate: '2024-11-01',
    devtools: { enabled: true },

    modules: [
        'vue3-pixi-nuxt',
        '@tresjs/nuxt',
    ],

    tres: {
        glsl: true,
    },

    nitro: {
        experimental: {
            wasm: true,
        }
    }
})
