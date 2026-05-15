import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      'pixel-math-wasm': path.resolve(__dirname, '../pkg/pixel_math_wasm.js'),
    },
  },
  optimizeDeps: {
    exclude: ['pixel-math-wasm'],
  },
  server: {
    fs: {
      allow: ['..'],
    },
  },
})