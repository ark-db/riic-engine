<script>
	import { error } from '$lib/stores';
	import Error from '$lib/components/Error.svelte';
</script>

<slot />

{#if $error}
	<Error msg={$error} />
{/if}

<style>
	:root {
		--light: #e6e6e7;
		--light-strong: #d3d3d6;
		--gray: #9a9696;
		--dark-mild: #444550;
		--dark: #2d2e31;
		--dark-strong: #1f2023;
		--blue: #1d67ee;
		--blue-strong: #135ee5;
		--salmon: #ed6e69;
		--salmon-strong: #fd4842;

		font-family: -apple-system, BlinkMacSystemFont, avenir next, avenir, segoe ui, helvetica neue,
			helvetica, Cantarell, Ubuntu, roboto, noto, arial, sans-serif;
		background-color: var(--dark);
		scrollbar-width: none;
	}
	:root::-webkit-scrollbar {
		display: none;
	}
</style>
