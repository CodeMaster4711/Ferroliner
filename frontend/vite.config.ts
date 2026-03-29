import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { enhancedImages } from '@sveltejs/enhanced-img';

export default defineConfig({
  plugins: [enhancedImages(), tailwindcss(), sveltekit()],
  server: {
    port: 1420,
    strictPort: true,
    host: '0.0.0.0',
    hmr: {
      protocol: 'ws',
      port: 1420
    },
    proxy: {
      '/api': {
        target: process.env.BACKEND_URL ?? 'http://localhost:8000',
        changeOrigin: true,
      }
    }
  },
  preview: {
    port: 1420
  }
});
