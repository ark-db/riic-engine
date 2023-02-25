import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit()],
	json: {
		stringify: true
	},
	build: {
		assetsInlineLimit: 0
	},
	server: {
		port: 5173,
		strictPort: true
	}
};

export default config;
