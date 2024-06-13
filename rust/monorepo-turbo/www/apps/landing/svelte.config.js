import adapter from '@sveltejs/adapter-auto';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	preprocess: vitePreprocess(),

	kit: {
		files: {
			lib: './lib',
			routes: './routes'
		},
		adapter: adapter()
	}
};

export default config;
