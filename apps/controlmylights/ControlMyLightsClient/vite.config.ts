import babel from '@rolldown/plugin-babel'
import { viteImage } from '@son426/vite-image'
import tailwindcss from '@tailwindcss/vite'
import react, { reactCompilerPreset } from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    babel({ presets: [reactCompilerPreset()] }),
    tailwindcss(),
    viteImage({
      autoApply: {
        extensions: ['.jpg'],
      },
    }),
  ],
})
