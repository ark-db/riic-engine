<script lang="ts">
	import { tooltip } from '$lib/tooltip';
	export let desc: string;
	export let onClick: () => void;

	let refreshState: Record<string, never> = {};

	function handleClick() {
		// This hack forces elements to be recreated via the {#key} block, since {} !== {}
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
		border: 0;
		padding: 0;
		background: none;
	}
</style>
