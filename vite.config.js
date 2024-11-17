import { svelte } from '@sveltejs/vite-plugin-svelte';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [svelte()],
	root: 'src/frontend',
	resolve: {
		alias: {
			$lib: '/src/frontend/src/lib'
		}
	},
	server: {
		proxy: {
			'/api': {
				target: 'http://localhost:8001',
				changeOrigin: true
			}
		}
	}
});
