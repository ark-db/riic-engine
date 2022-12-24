<script lang="ts">
	import { tooltip } from '$lib/tooltip';
	export let desc: string;
	export let onClick: () => void;

	// Note: reassigning refreshState forces content refresh via the {#key} block, since {} !== {}
	let refreshState: Record<string, never> = {};

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
