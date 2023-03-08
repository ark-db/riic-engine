<script lang="ts">
	import Error from '$lib/components/Error.svelte';
	import { error } from '$lib/stores';
	import '../focus.css';
	import '../input.css';

	// Disable copying/saving links and images contained within the app
	function handleMousedown(event: MouseEvent) {
		if (
			event.target &&
			(event.target instanceof HTMLAnchorElement || event.target instanceof HTMLImageElement)
		) {
			event.preventDefault();
		}
	}
</script>

<svelte:body on:mousedown={handleMousedown} />

<slot />

{#if $error}
	<Error msg={$error} />
{/if}

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

		font-family: -apple-system, BlinkMacSystemFont, avenir next, avenir, segoe ui, helvetica neue,
			helvetica, Cantarell, Ubuntu, roboto, noto, arial, sans-serif;
		background-color: var(--dark);
		scrollbar-width: none;
		user-select: none;
		-webkit-user-select: none;
	}
	:root::-webkit-scrollbar {
		display: none;
	}
	:global(button:hover) {
		cursor: pointer;
	}
	:global(a) {
		text-decoration: none;
	}
</style>
