<script lang="ts">
	import { dev } from '$app/environment';
	import Error from '$lib/components/Error.svelte';
	import '../focus.css';
	import '../input.css';
	import '../misc.css';
	import '../tooltip.css';

	// Disable copying/saving links and images contained within the app
	function handleMousedown(event: MouseEvent) {
		const { target } = event;

		if (target && (target instanceof HTMLAnchorElement || target instanceof HTMLImageElement)) {
			event.preventDefault();
		}
	}

	// Disable context menus in release builds
	function handleContextOpen(event: MouseEvent) {
		if (!dev) event.preventDefault();
	}
</script>

<svelte:body on:mousedown={handleMousedown} on:contextmenu={handleContextOpen} />

<slot />

<Error />

<style>
	:root {
		--light: #e6e6e7;
		--light-strong: #d3d3d6;
		--gray-mild: #a9a5a5;
		--gray: #9a9696;
		--dark-mild: #444550;
		--darkish: #343538;
		--dark: #2d2e31;
		--dark-strong: #1f2023;
		--blue-mild: #4a82ea;
		--blue: #1d67ee;
		--blue-strong: #135ee5;
		--salmon: #ed6e69;
		--salmon-strong: #fd4842;

		font-family:
			-apple-system,
			BlinkMacSystemFont,
			avenir next,
			avenir,
			segoe ui,
			helvetica neue,
			helvetica,
			Cantarell,
			Ubuntu,
			roboto,
			noto,
			arial,
			sans-serif;
		user-select: none;
		-webkit-user-select: none;
		background-color: var(--dark);
	}
</style>
