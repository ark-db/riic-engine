import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit()],
	clearScreen: false,
	json: {
		stringify: true
	},
	build: {
		// The Windows webview (WebView2) automatically updates by default, so the target depends on the last 3 Edge versions (i.e. Edge 116+).
		// The macOS webview (WebKit) only updates with new OS versions, so the target depends on the last 3 OS versions (i.e. Safari 15+).
		// The Linux webview (WebKitGTK) is based on WebKit, so the target is the same as macOS.
		target: process.env.TAURI_PLATFORM == 'windows' ? 'edge116' : ['es2021', 'safari15'],
		// By default, Vite inlines assets smaller than 4 KiB as base64 (see https://vitejs.dev/config/build-options.html#build-assetsinlinelimit).
		// These assets are blocked by the CSP in production builds, so inlining is explicitly disabled.
		assetsInlineLimit: 0
	},
	server: {
		port: 5173,
		strictPort: true
	}
};

export default config;
