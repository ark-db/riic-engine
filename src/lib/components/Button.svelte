<script lang="ts">
	// tippy.js tooltips are buggy when used on interactive elements such as <button>.
	// This custom button component resolves some of the issues by refreshing the element.

	import { tooltip } from '$lib/tooltip';

	export let desc: string;
	export let onClick: () => void;

	// Reassigning `refreshState` forces element refresh via the {#key} block, since {} !== {}
	let refreshState: Record<string, never> = {};

	// Force element refresh whenever `desc` changes
	$: desc, (refreshState = {});

	function handleClick() {
		refreshState = {};
		onClick();
	}
</script>

{#key refreshState}
	<button use:tooltip={desc} on:click|trusted={handleClick}>
		<slot />
	</button>
{/key}

<style>
	button {
		margin: 0;
		border: none;
		padding: 0;
		background: none;
	}
</style>
