import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit()],
	clearScreen: false,
	json: {
		stringify: true
	},
	build: {
		// The Windows webview (WebView2) is supposed to update itself automatically, so it's safe to target the latest 3 versions (i.e. Blink 108).
		// The macOS webview (WebKit) only updates with new OS versions, so the build target is more conservative; the earliest Safari version for macOS Big Sur is 14 (i.e. WebKit 610.2.11).
		// The Linux webview (WebKitGTK) is based on WebKit, so the target is the same as macOS.
		target: process.env.TAURI_PLATFORM == 'windows' ? 'edge108' : ['es2021', 'safari14'],
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
