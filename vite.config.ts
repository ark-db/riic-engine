import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit()],
	json: {
		stringify: true
	},
	// By default, Vite inlines assets smaller than 4 KiB as base64 (see https://vitejs.dev/config/build-options.html#build-assetsinlinelimit)
	// These assets are blocked by the CSP in production builds, so inlining is explicitly disabled
	build: {
		assetsInlineLimit: 0
	},
	server: {
		port: 5173,
		strictPort: true
	}
};

export default config;
