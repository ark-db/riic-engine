import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter(),
		alias: {
			'@types': 'src/types.ts',
			'@stores': 'src/stores.ts',
			'@tooltip': 'src/tooltip.ts',
			'@main': 'src/lib/components/main'
		},
		csp: {
			mode: 'hash',
			directives: {
				'default-src': ['none'],
				'img-src': ['self'],
				'script-src': ['self'],
				'style-src': ['self', 'unsafe-inline']
			}
		}
	}
};

export default config;
