import { defineConfig } from 'vite';
import svelte from '@sveltejs/vite-plugin-svelte';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [svelte()],
  server: {
    hmr: { host: 'localhost' },
  },
  optimizeDeps: {
    exclude: ['svelte-routing'],
  },
});
